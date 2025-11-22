pub fn sub_8260F9D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260F9D8 size=112
    let mut pc: u32 = 0x8260F9D8;
    'dispatch: loop {
        match pc {
            0x8260F9D8 => {
    //   block [0x8260F9D8..0x8260FA48)
	// 8260F9D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260F9DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260F9E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260F9E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260F9E8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260F9EC: 38AA8BF4  addi r5, r10, -0x740c
	ctx.r[5].s64 = ctx.r[10].s64 + -29708;
	// 8260F9F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260F9F4: 390B2570  addi r8, r11, 0x2570
	ctx.r[8].s64 = ctx.r[11].s64 + 9584;
	// 8260F9F8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8260F9FC: 388A5480  addi r4, r10, 0x5480
	ctx.r[4].s64 = ctx.r[10].s64 + 21632;
	// 8260FA00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260FA04: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260FA08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260FA0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260FA10: 386A8C54  addi r3, r10, -0x73ac
	ctx.r[3].s64 = ctx.r[10].s64 + -29612;
	// 8260FA14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260FA18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260FA1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260FA20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260FA24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260FA28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260FA2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260FA30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260FA34: 4BE573ED  bl 0x82466e20
	ctx.lr = 0x8260FA38;
	sub_82466E20(ctx, base);
	// 8260FA38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260FA3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260FA40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260FA44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260FA48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260FA48 size=112
    let mut pc: u32 = 0x8260FA48;
    'dispatch: loop {
        match pc {
            0x8260FA48 => {
    //   block [0x8260FA48..0x8260FAB8)
	// 8260FA48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260FA4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260FA50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260FA54: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260FA58: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260FA5C: 38AA8C24  addi r5, r10, -0x73dc
	ctx.r[5].s64 = ctx.r[10].s64 + -29660;
	// 8260FA60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260FA64: 390B25D0  addi r8, r11, 0x25d0
	ctx.r[8].s64 = ctx.r[11].s64 + 9680;
	// 8260FA68: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8260FA6C: 388A54A4  addi r4, r10, 0x54a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21668;
	// 8260FA70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260FA74: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260FA78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260FA7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260FA80: 386A8C84  addi r3, r10, -0x737c
	ctx.r[3].s64 = ctx.r[10].s64 + -29564;
	// 8260FA84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260FA88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260FA8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260FA90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260FA94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260FA98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260FA9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260FAA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260FAA4: 4BE5737D  bl 0x82466e20
	ctx.lr = 0x8260FAA8;
	sub_82466E20(ctx, base);
	// 8260FAA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260FAAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260FAB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260FAB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260FAB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260FAB8 size=100
    let mut pc: u32 = 0x8260FAB8;
    'dispatch: loop {
        match pc {
            0x8260FAB8 => {
    //   block [0x8260FAB8..0x8260FB1C)
	// 8260FAB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260FABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260FAC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260FAC4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260FAC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260FACC: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 8260FAD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260FAD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260FAD8: 388A54C8  addi r4, r10, 0x54c8
	ctx.r[4].s64 = ctx.r[10].s64 + 21704;
	// 8260FADC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260FAE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260FAE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260FAE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260FAEC: 386A8CB4  addi r3, r10, -0x734c
	ctx.r[3].s64 = ctx.r[10].s64 + -29516;
	// 8260FAF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260FAF4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260FAF8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260FAFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260FB00: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260FB04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260FB08: 4BE57319  bl 0x82466e20
	ctx.lr = 0x8260FB0C;
	sub_82466E20(ctx, base);
	// 8260FB0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260FB10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260FB14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260FB18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260FB20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260FB20 size=112
    let mut pc: u32 = 0x8260FB20;
    'dispatch: loop {
        match pc {
            0x8260FB20 => {
    //   block [0x8260FB20..0x8260FB90)
	// 8260FB20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260FB24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260FB28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260FB2C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260FB30: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260FB34: 38AA8CB4  addi r5, r10, -0x734c
	ctx.r[5].s64 = ctx.r[10].s64 + -29516;
	// 8260FB38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260FB3C: 390B2630  addi r8, r11, 0x2630
	ctx.r[8].s64 = ctx.r[11].s64 + 9776;
	// 8260FB40: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 8260FB44: 388A54D8  addi r4, r10, 0x54d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21720;
	// 8260FB48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260FB4C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260FB50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260FB54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260FB58: 386A8CE4  addi r3, r10, -0x731c
	ctx.r[3].s64 = ctx.r[10].s64 + -29468;
	// 8260FB5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260FB60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260FB64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260FB68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260FB6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260FB70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260FB74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260FB78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260FB7C: 4BE572A5  bl 0x82466e20
	ctx.lr = 0x8260FB80;
	sub_82466E20(ctx, base);
	// 8260FB80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260FB84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260FB88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260FB8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260FB90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260FB90 size=108
    let mut pc: u32 = 0x8260FB90;
    'dispatch: loop {
        match pc {
            0x8260FB90 => {
    //   block [0x8260FB90..0x8260FBFC)
	// 8260FB90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260FB94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260FB98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260FB9C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260FBA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260FBA4: 38EB2720  addi r7, r11, 0x2720
	ctx.r[7].s64 = ctx.r[11].s64 + 10016;
	// 8260FBA8: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 8260FBAC: 388A54F0  addi r4, r10, 0x54f0
	ctx.r[4].s64 = ctx.r[10].s64 + 21744;
	// 8260FBB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260FBB4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260FBB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260FBBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260FBC0: 386A8D14  addi r3, r10, -0x72ec
	ctx.r[3].s64 = ctx.r[10].s64 + -29420;
	// 8260FBC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260FBC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260FBCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260FBD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260FBD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260FBD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260FBDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260FBE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260FBE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260FBE8: 4BE57239  bl 0x82466e20
	ctx.lr = 0x8260FBEC;
	sub_82466E20(ctx, base);
	// 8260FBEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260FBF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260FBF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260FBF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260FC00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260FC00 size=108
    let mut pc: u32 = 0x8260FC00;
    'dispatch: loop {
        match pc {
            0x8260FC00 => {
    //   block [0x8260FC00..0x8260FC6C)
	// 8260FC00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260FC04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260FC08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260FC0C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260FC10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260FC14: 38EB2810  addi r7, r11, 0x2810
	ctx.r[7].s64 = ctx.r[11].s64 + 10256;
	// 8260FC18: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8260FC1C: 388A551C  addi r4, r10, 0x551c
	ctx.r[4].s64 = ctx.r[10].s64 + 21788;
	// 8260FC20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260FC24: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260FC28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260FC2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260FC30: 386A8D44  addi r3, r10, -0x72bc
	ctx.r[3].s64 = ctx.r[10].s64 + -29372;
	// 8260FC34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260FC38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260FC3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260FC40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260FC44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260FC48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260FC4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260FC50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260FC54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260FC58: 4BE571C9  bl 0x82466e20
	ctx.lr = 0x8260FC5C;
	sub_82466E20(ctx, base);
	// 8260FC5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260FC60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260FC64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260FC68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260FC70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260FC70 size=108
    let mut pc: u32 = 0x8260FC70;
    'dispatch: loop {
        match pc {
            0x8260FC70 => {
    //   block [0x8260FC70..0x8260FCDC)
	// 8260FC70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260FC74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260FC78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260FC7C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260FC80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260FC84: 38EB2858  addi r7, r11, 0x2858
	ctx.r[7].s64 = ctx.r[11].s64 + 10328;
	// 8260FC88: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 8260FC8C: 388A553C  addi r4, r10, 0x553c
	ctx.r[4].s64 = ctx.r[10].s64 + 21820;
	// 8260FC90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260FC94: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260FC98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260FC9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260FCA0: 386A8D74  addi r3, r10, -0x728c
	ctx.r[3].s64 = ctx.r[10].s64 + -29324;
	// 8260FCA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260FCA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260FCAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260FCB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260FCB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260FCB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260FCBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260FCC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260FCC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260FCC8: 4BE57159  bl 0x82466e20
	ctx.lr = 0x8260FCCC;
	sub_82466E20(ctx, base);
	// 8260FCCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260FCD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260FCD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260FCD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260FCE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260FCE0 size=108
    let mut pc: u32 = 0x8260FCE0;
    'dispatch: loop {
        match pc {
            0x8260FCE0 => {
    //   block [0x8260FCE0..0x8260FD4C)
	// 8260FCE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260FCE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260FCE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260FCEC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260FCF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260FCF4: 38EB2930  addi r7, r11, 0x2930
	ctx.r[7].s64 = ctx.r[11].s64 + 10544;
	// 8260FCF8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8260FCFC: 388A5560  addi r4, r10, 0x5560
	ctx.r[4].s64 = ctx.r[10].s64 + 21856;
	// 8260FD00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260FD04: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260FD08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260FD0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260FD10: 386A8DA4  addi r3, r10, -0x725c
	ctx.r[3].s64 = ctx.r[10].s64 + -29276;
	// 8260FD14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260FD18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260FD1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260FD20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260FD24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260FD28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260FD2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260FD30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260FD34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260FD38: 4BE570E9  bl 0x82466e20
	ctx.lr = 0x8260FD3C;
	sub_82466E20(ctx, base);
	// 8260FD3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260FD40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260FD44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260FD48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260FD50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260FD50 size=100
    let mut pc: u32 = 0x8260FD50;
    'dispatch: loop {
        match pc {
            0x8260FD50 => {
    //   block [0x8260FD50..0x8260FDB4)
	// 8260FD50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260FD54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260FD58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260FD5C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260FD60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260FD64: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 8260FD68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260FD6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260FD70: 388A5578  addi r4, r10, 0x5578
	ctx.r[4].s64 = ctx.r[10].s64 + 21880;
	// 8260FD74: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260FD78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260FD7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260FD80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260FD84: 386A8DD4  addi r3, r10, -0x722c
	ctx.r[3].s64 = ctx.r[10].s64 + -29228;
	// 8260FD88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260FD8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260FD90: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260FD94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260FD98: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260FD9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260FDA0: 4BE57081  bl 0x82466e20
	ctx.lr = 0x8260FDA4;
	sub_82466E20(ctx, base);
	// 8260FDA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260FDA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260FDAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260FDB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260FDB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260FDB8 size=112
    let mut pc: u32 = 0x8260FDB8;
    'dispatch: loop {
        match pc {
            0x8260FDB8 => {
    //   block [0x8260FDB8..0x8260FE28)
	// 8260FDB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260FDBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260FDC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260FDC4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260FDC8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260FDCC: 38AA8DD4  addi r5, r10, -0x722c
	ctx.r[5].s64 = ctx.r[10].s64 + -29228;
	// 8260FDD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260FDD4: 390B2948  addi r8, r11, 0x2948
	ctx.r[8].s64 = ctx.r[11].s64 + 10568;
	// 8260FDD8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8260FDDC: 388A558C  addi r4, r10, 0x558c
	ctx.r[4].s64 = ctx.r[10].s64 + 21900;
	// 8260FDE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260FDE4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260FDE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260FDEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260FDF0: 386A8E04  addi r3, r10, -0x71fc
	ctx.r[3].s64 = ctx.r[10].s64 + -29180;
	// 8260FDF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260FDF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260FDFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260FE00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260FE04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260FE08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260FE0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260FE10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260FE14: 4BE5700D  bl 0x82466e20
	ctx.lr = 0x8260FE18;
	sub_82466E20(ctx, base);
	// 8260FE18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260FE1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260FE20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260FE24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260FE28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260FE28 size=108
    let mut pc: u32 = 0x8260FE28;
    'dispatch: loop {
        match pc {
            0x8260FE28 => {
    //   block [0x8260FE28..0x8260FE94)
	// 8260FE28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260FE2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260FE30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260FE34: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260FE38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260FE3C: 38EB2990  addi r7, r11, 0x2990
	ctx.r[7].s64 = ctx.r[11].s64 + 10640;
	// 8260FE40: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8260FE44: 388A55A8  addi r4, r10, 0x55a8
	ctx.r[4].s64 = ctx.r[10].s64 + 21928;
	// 8260FE48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260FE4C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260FE50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260FE54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260FE58: 386A8E34  addi r3, r10, -0x71cc
	ctx.r[3].s64 = ctx.r[10].s64 + -29132;
	// 8260FE5C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260FE60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260FE64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260FE68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260FE6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260FE70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260FE74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260FE78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260FE7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260FE80: 4BE56FA1  bl 0x82466e20
	ctx.lr = 0x8260FE84;
	sub_82466E20(ctx, base);
	// 8260FE84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260FE88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260FE8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260FE90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260FE98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260FE98 size=112
    let mut pc: u32 = 0x8260FE98;
    'dispatch: loop {
        match pc {
            0x8260FE98 => {
    //   block [0x8260FE98..0x8260FF08)
	// 8260FE98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260FE9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260FEA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260FEA4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260FEA8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260FEAC: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 8260FEB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260FEB4: 390B29D8  addi r8, r11, 0x29d8
	ctx.r[8].s64 = ctx.r[11].s64 + 10712;
	// 8260FEB8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260FEBC: 388A55D8  addi r4, r10, 0x55d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21976;
	// 8260FEC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260FEC4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260FEC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260FECC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260FED0: 386A8E64  addi r3, r10, -0x719c
	ctx.r[3].s64 = ctx.r[10].s64 + -29084;
	// 8260FED4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260FED8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260FEDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260FEE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260FEE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260FEE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260FEEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260FEF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260FEF4: 4BE56F2D  bl 0x82466e20
	ctx.lr = 0x8260FEF8;
	sub_82466E20(ctx, base);
	// 8260FEF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260FEFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260FF00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260FF04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260FF08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260FF08 size=108
    let mut pc: u32 = 0x8260FF08;
    'dispatch: loop {
        match pc {
            0x8260FF08 => {
    //   block [0x8260FF08..0x8260FF74)
	// 8260FF08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260FF0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260FF10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260FF14: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260FF18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260FF1C: 38EB29F0  addi r7, r11, 0x29f0
	ctx.r[7].s64 = ctx.r[11].s64 + 10736;
	// 8260FF20: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8260FF24: 388A55EC  addi r4, r10, 0x55ec
	ctx.r[4].s64 = ctx.r[10].s64 + 21996;
	// 8260FF28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260FF2C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260FF30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260FF34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260FF38: 386A8E94  addi r3, r10, -0x716c
	ctx.r[3].s64 = ctx.r[10].s64 + -29036;
	// 8260FF3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260FF40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260FF44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260FF48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260FF4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260FF50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260FF54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260FF58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260FF5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260FF60: 4BE56EC1  bl 0x82466e20
	ctx.lr = 0x8260FF64;
	sub_82466E20(ctx, base);
	// 8260FF64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260FF68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260FF6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260FF70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260FF78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260FF78 size=112
    let mut pc: u32 = 0x8260FF78;
    'dispatch: loop {
        match pc {
            0x8260FF78 => {
    //   block [0x8260FF78..0x8260FFE8)
	// 8260FF78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260FF7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260FF80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260FF84: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260FF88: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260FF8C: 38AA8E64  addi r5, r10, -0x719c
	ctx.r[5].s64 = ctx.r[10].s64 + -29084;
	// 8260FF90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260FF94: 390B2A38  addi r8, r11, 0x2a38
	ctx.r[8].s64 = ctx.r[11].s64 + 10808;
	// 8260FF98: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260FF9C: 388A5628  addi r4, r10, 0x5628
	ctx.r[4].s64 = ctx.r[10].s64 + 22056;
	// 8260FFA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260FFA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260FFA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260FFAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260FFB0: 386A8EC4  addi r3, r10, -0x713c
	ctx.r[3].s64 = ctx.r[10].s64 + -28988;
	// 8260FFB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260FFB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260FFBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260FFC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260FFC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260FFC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260FFCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260FFD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260FFD4: 4BE56E4D  bl 0x82466e20
	ctx.lr = 0x8260FFD8;
	sub_82466E20(ctx, base);
	// 8260FFD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260FFDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260FFE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260FFE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260FFE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260FFE8 size=100
    let mut pc: u32 = 0x8260FFE8;
    'dispatch: loop {
        match pc {
            0x8260FFE8 => {
    //   block [0x8260FFE8..0x8261004C)
	// 8260FFE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260FFEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260FFF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260FFF4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260FFF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260FFFC: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 82610000: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82610004: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82610008: 388A5644  addi r4, r10, 0x5644
	ctx.r[4].s64 = ctx.r[10].s64 + 22084;
	// 8261000C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610010: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82610014: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82610018: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261001C: 386A8EF4  addi r3, r10, -0x710c
	ctx.r[3].s64 = ctx.r[10].s64 + -28940;
	// 82610020: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82610024: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82610028: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261002C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82610030: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82610034: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82610038: 4BE56DE9  bl 0x82466e20
	ctx.lr = 0x8261003C;
	sub_82466E20(ctx, base);
	// 8261003C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82610040: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82610044: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82610048: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82610050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82610050 size=112
    let mut pc: u32 = 0x82610050;
    'dispatch: loop {
        match pc {
            0x82610050 => {
    //   block [0x82610050..0x826100C0)
	// 82610050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82610054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82610058: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261005C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610060: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82610064: 38AA8EF4  addi r5, r10, -0x710c
	ctx.r[5].s64 = ctx.r[10].s64 + -28940;
	// 82610068: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261006C: 390B2A50  addi r8, r11, 0x2a50
	ctx.r[8].s64 = ctx.r[11].s64 + 10832;
	// 82610070: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82610074: 388A565C  addi r4, r10, 0x565c
	ctx.r[4].s64 = ctx.r[10].s64 + 22108;
	// 82610078: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261007C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610080: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82610084: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82610088: 386A8F24  addi r3, r10, -0x70dc
	ctx.r[3].s64 = ctx.r[10].s64 + -28892;
	// 8261008C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82610090: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82610094: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82610098: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261009C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826100A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826100A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826100A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826100AC: 4BE56D75  bl 0x82466e20
	ctx.lr = 0x826100B0;
	sub_82466E20(ctx, base);
	// 826100B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826100B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826100B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826100BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826100C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826100C0 size=108
    let mut pc: u32 = 0x826100C0;
    'dispatch: loop {
        match pc {
            0x826100C0 => {
    //   block [0x826100C0..0x8261012C)
	// 826100C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826100C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826100C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826100CC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826100D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826100D4: 38EB2AF8  addi r7, r11, 0x2af8
	ctx.r[7].s64 = ctx.r[11].s64 + 11000;
	// 826100D8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826100DC: 388A567C  addi r4, r10, 0x567c
	ctx.r[4].s64 = ctx.r[10].s64 + 22140;
	// 826100E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826100E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826100E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826100EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826100F0: 386A8F54  addi r3, r10, -0x70ac
	ctx.r[3].s64 = ctx.r[10].s64 + -28844;
	// 826100F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826100F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826100FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82610100: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82610104: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82610108: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261010C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82610110: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82610114: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82610118: 4BE56D09  bl 0x82466e20
	ctx.lr = 0x8261011C;
	sub_82466E20(ctx, base);
	// 8261011C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82610120: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82610124: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82610128: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82610130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82610130 size=112
    let mut pc: u32 = 0x82610130;
    'dispatch: loop {
        match pc {
            0x82610130 => {
    //   block [0x82610130..0x826101A0)
	// 82610130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82610134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82610138: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261013C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82610140: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82610144: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 82610148: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261014C: 390B2B28  addi r8, r11, 0x2b28
	ctx.r[8].s64 = ctx.r[11].s64 + 11048;
	// 82610150: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82610154: 388A568C  addi r4, r10, 0x568c
	ctx.r[4].s64 = ctx.r[10].s64 + 22156;
	// 82610158: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261015C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610160: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82610164: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82610168: 386A8F84  addi r3, r10, -0x707c
	ctx.r[3].s64 = ctx.r[10].s64 + -28796;
	// 8261016C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82610170: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82610174: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82610178: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261017C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82610180: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82610184: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82610188: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261018C: 4BE56C95  bl 0x82466e20
	ctx.lr = 0x82610190;
	sub_82466E20(ctx, base);
	// 82610190: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82610194: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82610198: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261019C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826101A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826101A0 size=112
    let mut pc: u32 = 0x826101A0;
    'dispatch: loop {
        match pc {
            0x826101A0 => {
    //   block [0x826101A0..0x82610210)
	// 826101A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826101A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826101A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826101AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826101B0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826101B4: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 826101B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826101BC: 390B2B70  addi r8, r11, 0x2b70
	ctx.r[8].s64 = ctx.r[11].s64 + 11120;
	// 826101C0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826101C4: 388A56A0  addi r4, r10, 0x56a0
	ctx.r[4].s64 = ctx.r[10].s64 + 22176;
	// 826101C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826101CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826101D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826101D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826101D8: 386A8FB4  addi r3, r10, -0x704c
	ctx.r[3].s64 = ctx.r[10].s64 + -28748;
	// 826101DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826101E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826101E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826101E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826101EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826101F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826101F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826101F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826101FC: 4BE56C25  bl 0x82466e20
	ctx.lr = 0x82610200;
	sub_82466E20(ctx, base);
	// 82610200: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82610204: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82610208: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261020C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82610210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82610210 size=100
    let mut pc: u32 = 0x82610210;
    'dispatch: loop {
        match pc {
            0x82610210 => {
    //   block [0x82610210..0x82610274)
	// 82610210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82610214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82610218: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261021C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82610220: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82610224: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 82610228: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261022C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82610230: 388A56B0  addi r4, r10, 0x56b0
	ctx.r[4].s64 = ctx.r[10].s64 + 22192;
	// 82610234: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610238: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261023C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82610240: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82610244: 386A8FE4  addi r3, r10, -0x701c
	ctx.r[3].s64 = ctx.r[10].s64 + -28700;
	// 82610248: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261024C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82610250: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82610254: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82610258: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261025C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82610260: 4BE56BC1  bl 0x82466e20
	ctx.lr = 0x82610264;
	sub_82466E20(ctx, base);
	// 82610264: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82610268: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261026C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82610270: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82610278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82610278 size=112
    let mut pc: u32 = 0x82610278;
    'dispatch: loop {
        match pc {
            0x82610278 => {
    //   block [0x82610278..0x826102E8)
	// 82610278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261027C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82610280: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82610284: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610288: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8261028C: 38AA8FE4  addi r5, r10, -0x701c
	ctx.r[5].s64 = ctx.r[10].s64 + -28700;
	// 82610290: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82610294: 390B2BB8  addi r8, r11, 0x2bb8
	ctx.r[8].s64 = ctx.r[11].s64 + 11192;
	// 82610298: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8261029C: 388A56C8  addi r4, r10, 0x56c8
	ctx.r[4].s64 = ctx.r[10].s64 + 22216;
	// 826102A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826102A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826102A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826102AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826102B0: 386A9014  addi r3, r10, -0x6fec
	ctx.r[3].s64 = ctx.r[10].s64 + -28652;
	// 826102B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826102B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826102BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826102C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826102C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826102C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826102CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826102D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826102D4: 4BE56B4D  bl 0x82466e20
	ctx.lr = 0x826102D8;
	sub_82466E20(ctx, base);
	// 826102D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826102DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826102E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826102E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826102E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826102E8 size=112
    let mut pc: u32 = 0x826102E8;
    'dispatch: loop {
        match pc {
            0x826102E8 => {
    //   block [0x826102E8..0x82610358)
	// 826102E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826102EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826102F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826102F4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826102F8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826102FC: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 82610300: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82610304: 390B2C00  addi r8, r11, 0x2c00
	ctx.r[8].s64 = ctx.r[11].s64 + 11264;
	// 82610308: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261030C: 388A56E8  addi r4, r10, 0x56e8
	ctx.r[4].s64 = ctx.r[10].s64 + 22248;
	// 82610310: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82610314: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610318: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261031C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82610320: 386A9044  addi r3, r10, -0x6fbc
	ctx.r[3].s64 = ctx.r[10].s64 + -28604;
	// 82610324: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82610328: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261032C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82610330: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82610334: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82610338: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261033C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82610340: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82610344: 4BE56ADD  bl 0x82466e20
	ctx.lr = 0x82610348;
	sub_82466E20(ctx, base);
	// 82610348: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261034C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82610350: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82610354: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82610358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82610358 size=112
    let mut pc: u32 = 0x82610358;
    'dispatch: loop {
        match pc {
            0x82610358 => {
    //   block [0x82610358..0x826103C8)
	// 82610358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261035C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82610360: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82610364: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82610368: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8261036C: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 82610370: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82610374: 390B2C18  addi r8, r11, 0x2c18
	ctx.r[8].s64 = ctx.r[11].s64 + 11288;
	// 82610378: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261037C: 388A5700  addi r4, r10, 0x5700
	ctx.r[4].s64 = ctx.r[10].s64 + 22272;
	// 82610380: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82610384: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610388: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261038C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82610390: 386A9074  addi r3, r10, -0x6f8c
	ctx.r[3].s64 = ctx.r[10].s64 + -28556;
	// 82610394: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82610398: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261039C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826103A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826103A4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826103A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826103AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826103B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826103B4: 4BE56A6D  bl 0x82466e20
	ctx.lr = 0x826103B8;
	sub_82466E20(ctx, base);
	// 826103B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826103BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826103C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826103C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826103C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826103C8 size=112
    let mut pc: u32 = 0x826103C8;
    'dispatch: loop {
        match pc {
            0x826103C8 => {
    //   block [0x826103C8..0x82610438)
	// 826103C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826103CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826103D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826103D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826103D8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826103DC: 38AA9044  addi r5, r10, -0x6fbc
	ctx.r[5].s64 = ctx.r[10].s64 + -28604;
	// 826103E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826103E4: 390B2C30  addi r8, r11, 0x2c30
	ctx.r[8].s64 = ctx.r[11].s64 + 11312;
	// 826103E8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826103EC: 388A571C  addi r4, r10, 0x571c
	ctx.r[4].s64 = ctx.r[10].s64 + 22300;
	// 826103F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826103F4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826103F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826103FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82610400: 386A90A4  addi r3, r10, -0x6f5c
	ctx.r[3].s64 = ctx.r[10].s64 + -28508;
	// 82610404: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82610408: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261040C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82610410: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82610414: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82610418: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261041C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82610420: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82610424: 4BE569FD  bl 0x82466e20
	ctx.lr = 0x82610428;
	sub_82466E20(ctx, base);
	// 82610428: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261042C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82610430: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82610434: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82610438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82610438 size=72
    let mut pc: u32 = 0x82610438;
    'dispatch: loop {
        match pc {
            0x82610438 => {
    //   block [0x82610438..0x82610480)
	// 82610438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261043C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82610440: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82610444: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82610448: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 8261044C: 38CBCF78  addi r6, r11, -0x3088
	ctx.r[6].s64 = ctx.r[11].s64 + -12424;
	// 82610450: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82610454: 388BE570  addi r4, r11, -0x1a90
	ctx.r[4].s64 = ctx.r[11].s64 + -6800;
	// 82610458: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 8261045C: 386B90D4  addi r3, r11, -0x6f2c
	ctx.r[3].s64 = ctx.r[11].s64 + -28460;
	// 82610460: 4BE6B629  bl 0x8247ba88
	ctx.lr = 0x82610464;
	sub_8247BA88(ctx, base);
	// 82610464: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 82610468: 386BCD20  addi r3, r11, -0x32e0
	ctx.r[3].s64 = ctx.r[11].s64 + -13024;
	// 8261046C: 4BF226CD  bl 0x82532b38
	ctx.lr = 0x82610470;
	sub_82532B38(ctx, base);
	// 82610470: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82610474: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82610478: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261047C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82610480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82610480 size=108
    let mut pc: u32 = 0x82610480;
    'dispatch: loop {
        match pc {
            0x82610480 => {
    //   block [0x82610480..0x826104EC)
	// 82610480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82610484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82610488: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261048C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82610490: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82610494: 38EB4550  addi r7, r11, 0x4550
	ctx.r[7].s64 = ctx.r[11].s64 + 17744;
	// 82610498: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8261049C: 388A2B24  addi r4, r10, 0x2b24
	ctx.r[4].s64 = ctx.r[10].s64 + 11044;
	// 826104A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826104A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826104A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826104AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826104B0: 386A90EC  addi r3, r10, -0x6f14
	ctx.r[3].s64 = ctx.r[10].s64 + -28436;
	// 826104B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826104B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826104BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826104C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826104C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826104C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826104CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826104D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826104D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826104D8: 4BE56949  bl 0x82466e20
	ctx.lr = 0x826104DC;
	sub_82466E20(ctx, base);
	// 826104DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826104E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826104E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826104E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826104F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826104F0 size=24
    let mut pc: u32 = 0x826104F0;
    'dispatch: loop {
        match pc {
            0x826104F0 => {
    //   block [0x826104F0..0x82610508)
	// 826104F0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826104F4: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 826104F8: 394ACB80  addi r10, r10, -0x3480
	ctx.r[10].s64 = ctx.r[10].s64 + -13440;
	// 826104FC: 816B45C8  lwz r11, 0x45c8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(17864 as u32) ) } as u64;
	// 82610500: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82610504: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82610508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82610508 size=112
    let mut pc: u32 = 0x82610508;
    'dispatch: loop {
        match pc {
            0x82610508 => {
    //   block [0x82610508..0x82610578)
	// 82610508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261050C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82610510: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82610514: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82610518: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261051C: 392AED9C  addi r9, r10, -0x1264
	ctx.r[9].s64 = ctx.r[10].s64 + -4708;
	// 82610520: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82610524: 390BCB80  addi r8, r11, -0x3480
	ctx.r[8].s64 = ctx.r[11].s64 + -13440;
	// 82610528: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8261052C: 388A2B3C  addi r4, r10, 0x2b3c
	ctx.r[4].s64 = ctx.r[10].s64 + 11068;
	// 82610530: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82610534: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610538: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261053C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82610540: 386A911C  addi r3, r10, -0x6ee4
	ctx.r[3].s64 = ctx.r[10].s64 + -28388;
	// 82610544: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82610548: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8261054C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82610550: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82610554: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82610558: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261055C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82610560: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82610564: 4BE568BD  bl 0x82466e20
	ctx.lr = 0x82610568;
	sub_82466E20(ctx, base);
	// 82610568: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261056C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82610570: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82610574: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82610578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82610578 size=108
    let mut pc: u32 = 0x82610578;
    'dispatch: loop {
        match pc {
            0x82610578 => {
    //   block [0x82610578..0x826105E4)
	// 82610578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261057C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82610580: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82610584: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82610588: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261058C: 38EB45CC  addi r7, r11, 0x45cc
	ctx.r[7].s64 = ctx.r[11].s64 + 17868;
	// 82610590: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82610594: 388A2B50  addi r4, r10, 0x2b50
	ctx.r[4].s64 = ctx.r[10].s64 + 11088;
	// 82610598: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261059C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826105A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826105A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826105A8: 386A914C  addi r3, r10, -0x6eb4
	ctx.r[3].s64 = ctx.r[10].s64 + -28340;
	// 826105AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826105B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826105B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826105B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826105BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826105C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826105C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826105C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826105CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826105D0: 4BE56851  bl 0x82466e20
	ctx.lr = 0x826105D4;
	sub_82466E20(ctx, base);
	// 826105D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826105D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826105DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826105E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826105E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826105E8 size=108
    let mut pc: u32 = 0x826105E8;
    'dispatch: loop {
        match pc {
            0x826105E8 => {
    //   block [0x826105E8..0x82610654)
	// 826105E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826105EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826105F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826105F4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826105F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826105FC: 38EB45FC  addi r7, r11, 0x45fc
	ctx.r[7].s64 = ctx.r[11].s64 + 17916;
	// 82610600: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82610604: 388A2B6C  addi r4, r10, 0x2b6c
	ctx.r[4].s64 = ctx.r[10].s64 + 11116;
	// 82610608: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261060C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610610: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82610614: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82610618: 386A917C  addi r3, r10, -0x6e84
	ctx.r[3].s64 = ctx.r[10].s64 + -28292;
	// 8261061C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82610620: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82610624: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82610628: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261062C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82610630: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82610634: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82610638: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261063C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82610640: 4BE567E1  bl 0x82466e20
	ctx.lr = 0x82610644;
	sub_82466E20(ctx, base);
	// 82610644: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82610648: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261064C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82610650: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82610658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82610658 size=24
    let mut pc: u32 = 0x82610658;
    'dispatch: loop {
        match pc {
            0x82610658 => {
    //   block [0x82610658..0x82610670)
	// 82610658: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8261065C: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82610660: 394ACBC8  addi r10, r10, -0x3438
	ctx.r[10].s64 = ctx.r[10].s64 + -13368;
	// 82610664: 816B462C  lwz r11, 0x462c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(17964 as u32) ) } as u64;
	// 82610668: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8261066C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82610670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82610670 size=116
    let mut pc: u32 = 0x82610670;
    'dispatch: loop {
        match pc {
            0x82610670 => {
    //   block [0x82610670..0x826106E4)
	// 82610670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82610674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82610678: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261067C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82610680: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82610684: 390BCBC8  addi r8, r11, -0x3438
	ctx.r[8].s64 = ctx.r[11].s64 + -13368;
	// 82610688: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261068C: 392AEDD0  addi r9, r10, -0x1230
	ctx.r[9].s64 = ctx.r[10].s64 + -4656;
	// 82610690: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610694: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82610698: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 8261069C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826106A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826106A4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826106A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826106AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826106B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826106B4: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 826106B8: 388A2BB4  addi r4, r10, 0x2bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 11188;
	// 826106BC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826106C0: 386B91AC  addi r3, r11, -0x6e54
	ctx.r[3].s64 = ctx.r[11].s64 + -28244;
	// 826106C4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826106C8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826106CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826106D0: 4BE56751  bl 0x82466e20
	ctx.lr = 0x826106D4;
	sub_82466E20(ctx, base);
	// 826106D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826106D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826106DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826106E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826106E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826106E8 size=108
    let mut pc: u32 = 0x826106E8;
    'dispatch: loop {
        match pc {
            0x826106E8 => {
    //   block [0x826106E8..0x82610754)
	// 826106E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826106EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826106F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826106F4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826106F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826106FC: 38EB4630  addi r7, r11, 0x4630
	ctx.r[7].s64 = ctx.r[11].s64 + 17968;
	// 82610700: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82610704: 388A2BC8  addi r4, r10, 0x2bc8
	ctx.r[4].s64 = ctx.r[10].s64 + 11208;
	// 82610708: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261070C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610710: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82610714: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82610718: 386A91DC  addi r3, r10, -0x6e24
	ctx.r[3].s64 = ctx.r[10].s64 + -28196;
	// 8261071C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82610720: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82610724: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82610728: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261072C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82610730: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82610734: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82610738: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261073C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82610740: 4BE566E1  bl 0x82466e20
	ctx.lr = 0x82610744;
	sub_82466E20(ctx, base);
	// 82610744: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82610748: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261074C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82610750: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82610758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82610758 size=112
    let mut pc: u32 = 0x82610758;
    'dispatch: loop {
        match pc {
            0x82610758 => {
    //   block [0x82610758..0x826107C8)
	// 82610758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261075C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82610760: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82610764: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610768: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8261076C: 38AA91AC  addi r5, r10, -0x6e54
	ctx.r[5].s64 = ctx.r[10].s64 + -28244;
	// 82610770: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82610774: 390B46C0  addi r8, r11, 0x46c0
	ctx.r[8].s64 = ctx.r[11].s64 + 18112;
	// 82610778: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 8261077C: 388A2C00  addi r4, r10, 0x2c00
	ctx.r[4].s64 = ctx.r[10].s64 + 11264;
	// 82610780: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82610784: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610788: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261078C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82610790: 386A920C  addi r3, r10, -0x6df4
	ctx.r[3].s64 = ctx.r[10].s64 + -28148;
	// 82610794: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82610798: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261079C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826107A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826107A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826107A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826107AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826107B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826107B4: 4BE5666D  bl 0x82466e20
	ctx.lr = 0x826107B8;
	sub_82466E20(ctx, base);
	// 826107B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826107BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826107C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826107C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826107C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826107C8 size=112
    let mut pc: u32 = 0x826107C8;
    'dispatch: loop {
        match pc {
            0x826107C8 => {
    //   block [0x826107C8..0x82610838)
	// 826107C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826107CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826107D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826107D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826107D8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826107DC: 38AA91AC  addi r5, r10, -0x6e54
	ctx.r[5].s64 = ctx.r[10].s64 + -28244;
	// 826107E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826107E4: 390B47E0  addi r8, r11, 0x47e0
	ctx.r[8].s64 = ctx.r[11].s64 + 18400;
	// 826107E8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826107EC: 388A2C24  addi r4, r10, 0x2c24
	ctx.r[4].s64 = ctx.r[10].s64 + 11300;
	// 826107F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826107F4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826107F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826107FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82610800: 386A923C  addi r3, r10, -0x6dc4
	ctx.r[3].s64 = ctx.r[10].s64 + -28100;
	// 82610804: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82610808: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261080C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82610810: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82610814: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82610818: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261081C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82610820: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82610824: 4BE565FD  bl 0x82466e20
	ctx.lr = 0x82610828;
	sub_82466E20(ctx, base);
	// 82610828: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261082C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82610830: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82610834: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82610838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82610838 size=108
    let mut pc: u32 = 0x82610838;
    'dispatch: loop {
        match pc {
            0x82610838 => {
    //   block [0x82610838..0x826108A4)
	// 82610838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261083C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82610840: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82610844: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82610848: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261084C: 38EB47F8  addi r7, r11, 0x47f8
	ctx.r[7].s64 = ctx.r[11].s64 + 18424;
	// 82610850: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82610854: 388A2C44  addi r4, r10, 0x2c44
	ctx.r[4].s64 = ctx.r[10].s64 + 11332;
	// 82610858: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261085C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610860: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82610864: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82610868: 386A926C  addi r3, r10, -0x6d94
	ctx.r[3].s64 = ctx.r[10].s64 + -28052;
	// 8261086C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82610870: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82610874: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82610878: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261087C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82610880: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82610884: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82610888: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261088C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82610890: 4BE56591  bl 0x82466e20
	ctx.lr = 0x82610894;
	sub_82466E20(ctx, base);
	// 82610894: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82610898: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261089C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826108A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826108A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826108A8 size=112
    let mut pc: u32 = 0x826108A8;
    'dispatch: loop {
        match pc {
            0x826108A8 => {
    //   block [0x826108A8..0x82610918)
	// 826108A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826108AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826108B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826108B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826108B8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826108BC: 38AA91AC  addi r5, r10, -0x6e54
	ctx.r[5].s64 = ctx.r[10].s64 + -28244;
	// 826108C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826108C4: 390B4888  addi r8, r11, 0x4888
	ctx.r[8].s64 = ctx.r[11].s64 + 18568;
	// 826108C8: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 826108CC: 388A2C74  addi r4, r10, 0x2c74
	ctx.r[4].s64 = ctx.r[10].s64 + 11380;
	// 826108D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826108D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826108D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826108DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826108E0: 386A929C  addi r3, r10, -0x6d64
	ctx.r[3].s64 = ctx.r[10].s64 + -28004;
	// 826108E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826108E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826108EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826108F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826108F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826108F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826108FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82610900: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82610904: 4BE5651D  bl 0x82466e20
	ctx.lr = 0x82610908;
	sub_82466E20(ctx, base);
	// 82610908: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261090C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82610910: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82610914: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82610918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82610918 size=108
    let mut pc: u32 = 0x82610918;
    'dispatch: loop {
        match pc {
            0x82610918 => {
    //   block [0x82610918..0x82610984)
	// 82610918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261091C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82610920: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82610924: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82610928: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261092C: 38EB4978  addi r7, r11, 0x4978
	ctx.r[7].s64 = ctx.r[11].s64 + 18808;
	// 82610930: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82610934: 388A2C90  addi r4, r10, 0x2c90
	ctx.r[4].s64 = ctx.r[10].s64 + 11408;
	// 82610938: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261093C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610940: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82610944: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82610948: 386A92CC  addi r3, r10, -0x6d34
	ctx.r[3].s64 = ctx.r[10].s64 + -27956;
	// 8261094C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82610950: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82610954: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82610958: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261095C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82610960: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82610964: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82610968: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261096C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82610970: 4BE564B1  bl 0x82466e20
	ctx.lr = 0x82610974;
	sub_82466E20(ctx, base);
	// 82610974: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82610978: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261097C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82610980: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82610988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82610988 size=108
    let mut pc: u32 = 0x82610988;
    'dispatch: loop {
        match pc {
            0x82610988 => {
    //   block [0x82610988..0x826109F4)
	// 82610988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261098C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82610990: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82610994: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82610998: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261099C: 38EB4990  addi r7, r11, 0x4990
	ctx.r[7].s64 = ctx.r[11].s64 + 18832;
	// 826109A0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826109A4: 388A2CA8  addi r4, r10, 0x2ca8
	ctx.r[4].s64 = ctx.r[10].s64 + 11432;
	// 826109A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826109AC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826109B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826109B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826109B8: 386A92FC  addi r3, r10, -0x6d04
	ctx.r[3].s64 = ctx.r[10].s64 + -27908;
	// 826109BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826109C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826109C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826109C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826109CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826109D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826109D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826109D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826109DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826109E0: 4BE56441  bl 0x82466e20
	ctx.lr = 0x826109E4;
	sub_82466E20(ctx, base);
	// 826109E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826109E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826109EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826109F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826109F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826109F8 size=116
    let mut pc: u32 = 0x826109F8;
    'dispatch: loop {
        match pc {
            0x826109F8 => {
    //   block [0x826109F8..0x82610A6C)
	// 826109F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826109FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82610A00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82610A04: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82610A08: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82610A0C: 390B49F4  addi r8, r11, 0x49f4
	ctx.r[8].s64 = ctx.r[11].s64 + 18932;
	// 82610A10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82610A14: 392AEDFC  addi r9, r10, -0x1204
	ctx.r[9].s64 = ctx.r[10].s64 + -4612;
	// 82610A18: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610A1C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82610A20: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 82610A24: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82610A28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82610A2C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82610A30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82610A34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82610A38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82610A3C: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82610A40: 388A2CB8  addi r4, r10, 0x2cb8
	ctx.r[4].s64 = ctx.r[10].s64 + 11448;
	// 82610A44: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82610A48: 386B932C  addi r3, r11, -0x6cd4
	ctx.r[3].s64 = ctx.r[11].s64 + -27860;
	// 82610A4C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82610A50: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82610A54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82610A58: 4BE563C9  bl 0x82466e20
	ctx.lr = 0x82610A5C;
	sub_82466E20(ctx, base);
	// 82610A5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82610A60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82610A64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82610A68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82610A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82610A70 size=108
    let mut pc: u32 = 0x82610A70;
    'dispatch: loop {
        match pc {
            0x82610A70 => {
    //   block [0x82610A70..0x82610ADC)
	// 82610A70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82610A74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82610A78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82610A7C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82610A80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82610A84: 38EB4A10  addi r7, r11, 0x4a10
	ctx.r[7].s64 = ctx.r[11].s64 + 18960;
	// 82610A88: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82610A8C: 388A2CCC  addi r4, r10, 0x2ccc
	ctx.r[4].s64 = ctx.r[10].s64 + 11468;
	// 82610A90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82610A94: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610A98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82610A9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82610AA0: 386A935C  addi r3, r10, -0x6ca4
	ctx.r[3].s64 = ctx.r[10].s64 + -27812;
	// 82610AA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82610AA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82610AAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82610AB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82610AB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82610AB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82610ABC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82610AC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82610AC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82610AC8: 4BE56359  bl 0x82466e20
	ctx.lr = 0x82610ACC;
	sub_82466E20(ctx, base);
	// 82610ACC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82610AD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82610AD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82610AD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82610AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82610AE0 size=108
    let mut pc: u32 = 0x82610AE0;
    'dispatch: loop {
        match pc {
            0x82610AE0 => {
    //   block [0x82610AE0..0x82610B4C)
	// 82610AE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82610AE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82610AE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82610AEC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82610AF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82610AF4: 38EB4A58  addi r7, r11, 0x4a58
	ctx.r[7].s64 = ctx.r[11].s64 + 19032;
	// 82610AF8: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82610AFC: 388A2CF0  addi r4, r10, 0x2cf0
	ctx.r[4].s64 = ctx.r[10].s64 + 11504;
	// 82610B00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82610B04: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610B08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82610B0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82610B10: 386A938C  addi r3, r10, -0x6c74
	ctx.r[3].s64 = ctx.r[10].s64 + -27764;
	// 82610B14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82610B18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82610B1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82610B20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82610B24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82610B28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82610B2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82610B30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82610B34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82610B38: 4BE562E9  bl 0x82466e20
	ctx.lr = 0x82610B3C;
	sub_82466E20(ctx, base);
	// 82610B3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82610B40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82610B44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82610B48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82610B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82610B50 size=108
    let mut pc: u32 = 0x82610B50;
    'dispatch: loop {
        match pc {
            0x82610B50 => {
    //   block [0x82610B50..0x82610BBC)
	// 82610B50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82610B54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82610B58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82610B5C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82610B60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82610B64: 38EB4AE8  addi r7, r11, 0x4ae8
	ctx.r[7].s64 = ctx.r[11].s64 + 19176;
	// 82610B68: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82610B6C: 388A2D14  addi r4, r10, 0x2d14
	ctx.r[4].s64 = ctx.r[10].s64 + 11540;
	// 82610B70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82610B74: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610B78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82610B7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82610B80: 386A93BC  addi r3, r10, -0x6c44
	ctx.r[3].s64 = ctx.r[10].s64 + -27716;
	// 82610B84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82610B88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82610B8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82610B90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82610B94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82610B98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82610B9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82610BA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82610BA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82610BA8: 4BE56279  bl 0x82466e20
	ctx.lr = 0x82610BAC;
	sub_82466E20(ctx, base);
	// 82610BAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82610BB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82610BB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82610BB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82610BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82610BC0 size=100
    let mut pc: u32 = 0x82610BC0;
    'dispatch: loop {
        match pc {
            0x82610BC0 => {
    //   block [0x82610BC0..0x82610C24)
	// 82610BC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82610BC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82610BC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82610BCC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610BD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82610BD4: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 82610BD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82610BDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82610BE0: 388A2D2C  addi r4, r10, 0x2d2c
	ctx.r[4].s64 = ctx.r[10].s64 + 11564;
	// 82610BE4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610BE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82610BEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82610BF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82610BF4: 386A93EC  addi r3, r10, -0x6c14
	ctx.r[3].s64 = ctx.r[10].s64 + -27668;
	// 82610BF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82610BFC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82610C00: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82610C04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82610C08: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82610C0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82610C10: 4BE56211  bl 0x82466e20
	ctx.lr = 0x82610C14;
	sub_82466E20(ctx, base);
	// 82610C14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82610C18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82610C1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82610C20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82610C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82610C28 size=112
    let mut pc: u32 = 0x82610C28;
    'dispatch: loop {
        match pc {
            0x82610C28 => {
    //   block [0x82610C28..0x82610C98)
	// 82610C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82610C2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82610C30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82610C34: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610C38: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82610C3C: 38AA93EC  addi r5, r10, -0x6c14
	ctx.r[5].s64 = ctx.r[10].s64 + -27668;
	// 82610C40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82610C44: 390B4B78  addi r8, r11, 0x4b78
	ctx.r[8].s64 = ctx.r[11].s64 + 19320;
	// 82610C48: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82610C4C: 388A2D48  addi r4, r10, 0x2d48
	ctx.r[4].s64 = ctx.r[10].s64 + 11592;
	// 82610C50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82610C54: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610C58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82610C5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82610C60: 386A941C  addi r3, r10, -0x6be4
	ctx.r[3].s64 = ctx.r[10].s64 + -27620;
	// 82610C64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82610C68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82610C6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82610C70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82610C74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82610C78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82610C7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82610C80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82610C84: 4BE5619D  bl 0x82466e20
	ctx.lr = 0x82610C88;
	sub_82466E20(ctx, base);
	// 82610C88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82610C8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82610C90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82610C94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82610C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82610C98 size=108
    let mut pc: u32 = 0x82610C98;
    'dispatch: loop {
        match pc {
            0x82610C98 => {
    //   block [0x82610C98..0x82610D04)
	// 82610C98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82610C9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82610CA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82610CA4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82610CA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82610CAC: 38EB4BD8  addi r7, r11, 0x4bd8
	ctx.r[7].s64 = ctx.r[11].s64 + 19416;
	// 82610CB0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82610CB4: 388A2D68  addi r4, r10, 0x2d68
	ctx.r[4].s64 = ctx.r[10].s64 + 11624;
	// 82610CB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82610CBC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610CC0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82610CC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82610CC8: 386A944C  addi r3, r10, -0x6bb4
	ctx.r[3].s64 = ctx.r[10].s64 + -27572;
	// 82610CCC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82610CD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82610CD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82610CD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82610CDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82610CE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82610CE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82610CE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82610CEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82610CF0: 4BE56131  bl 0x82466e20
	ctx.lr = 0x82610CF4;
	sub_82466E20(ctx, base);
	// 82610CF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82610CF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82610CFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82610D00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82610D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82610D08 size=108
    let mut pc: u32 = 0x82610D08;
    'dispatch: loop {
        match pc {
            0x82610D08 => {
    //   block [0x82610D08..0x82610D74)
	// 82610D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82610D0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82610D10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82610D14: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82610D18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82610D1C: 38EB4C08  addi r7, r11, 0x4c08
	ctx.r[7].s64 = ctx.r[11].s64 + 19464;
	// 82610D20: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82610D24: 388A2D70  addi r4, r10, 0x2d70
	ctx.r[4].s64 = ctx.r[10].s64 + 11632;
	// 82610D28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82610D2C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610D30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82610D34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82610D38: 386A947C  addi r3, r10, -0x6b84
	ctx.r[3].s64 = ctx.r[10].s64 + -27524;
	// 82610D3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82610D40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82610D44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82610D48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82610D4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82610D50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82610D54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82610D58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82610D5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82610D60: 4BE560C1  bl 0x82466e20
	ctx.lr = 0x82610D64;
	sub_82466E20(ctx, base);
	// 82610D64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82610D68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82610D6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82610D70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82610D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82610D78 size=108
    let mut pc: u32 = 0x82610D78;
    'dispatch: loop {
        match pc {
            0x82610D78 => {
    //   block [0x82610D78..0x82610DE4)
	// 82610D78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82610D7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82610D80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82610D84: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82610D88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82610D8C: 38EB4C68  addi r7, r11, 0x4c68
	ctx.r[7].s64 = ctx.r[11].s64 + 19560;
	// 82610D90: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82610D94: 388A2D84  addi r4, r10, 0x2d84
	ctx.r[4].s64 = ctx.r[10].s64 + 11652;
	// 82610D98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82610D9C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610DA0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82610DA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82610DA8: 386A94AC  addi r3, r10, -0x6b54
	ctx.r[3].s64 = ctx.r[10].s64 + -27476;
	// 82610DAC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82610DB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82610DB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82610DB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82610DBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82610DC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82610DC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82610DC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82610DCC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82610DD0: 4BE56051  bl 0x82466e20
	ctx.lr = 0x82610DD4;
	sub_82466E20(ctx, base);
	// 82610DD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82610DD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82610DDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82610DE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82610DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82610DE8 size=24
    let mut pc: u32 = 0x82610DE8;
    'dispatch: loop {
        match pc {
            0x82610DE8 => {
    //   block [0x82610DE8..0x82610E00)
	// 82610DE8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82610DEC: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82610DF0: 394ACC40  addi r10, r10, -0x33c0
	ctx.r[10].s64 = ctx.r[10].s64 + -13248;
	// 82610DF4: 816B4A0C  lwz r11, 0x4a0c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(18956 as u32) ) } as u64;
	// 82610DF8: 916A0128  stw r11, 0x128(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(296 as u32), ctx.r[11].u32 ) };
	// 82610DFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82610E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82610E00 size=116
    let mut pc: u32 = 0x82610E00;
    'dispatch: loop {
        match pc {
            0x82610E00 => {
    //   block [0x82610E00..0x82610E74)
	// 82610E00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82610E04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82610E08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82610E0C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82610E10: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82610E14: 390BCC40  addi r8, r11, -0x33c0
	ctx.r[8].s64 = ctx.r[11].s64 + -13248;
	// 82610E18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82610E1C: 392AEE30  addi r9, r10, -0x11d0
	ctx.r[9].s64 = ctx.r[10].s64 + -4560;
	// 82610E20: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610E24: 38E0000D  li r7, 0xd
	ctx.r[7].s64 = 13;
	// 82610E28: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 82610E2C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82610E30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82610E34: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82610E38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82610E3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82610E40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82610E44: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82610E48: 388A2D90  addi r4, r10, 0x2d90
	ctx.r[4].s64 = ctx.r[10].s64 + 11664;
	// 82610E4C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82610E50: 386B94DC  addi r3, r11, -0x6b24
	ctx.r[3].s64 = ctx.r[11].s64 + -27428;
	// 82610E54: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82610E58: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82610E5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82610E60: 4BE55FC1  bl 0x82466e20
	ctx.lr = 0x82610E64;
	sub_82466E20(ctx, base);
	// 82610E64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82610E68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82610E6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82610E70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82610E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82610E78 size=112
    let mut pc: u32 = 0x82610E78;
    'dispatch: loop {
        match pc {
            0x82610E78 => {
    //   block [0x82610E78..0x82610EE8)
	// 82610E78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82610E7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82610E80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82610E84: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610E88: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82610E8C: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 82610E90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82610E94: 390B4CC8  addi r8, r11, 0x4cc8
	ctx.r[8].s64 = ctx.r[11].s64 + 19656;
	// 82610E98: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82610E9C: 388A2DA4  addi r4, r10, 0x2da4
	ctx.r[4].s64 = ctx.r[10].s64 + 11684;
	// 82610EA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82610EA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610EA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82610EAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82610EB0: 386A950C  addi r3, r10, -0x6af4
	ctx.r[3].s64 = ctx.r[10].s64 + -27380;
	// 82610EB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82610EB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82610EBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82610EC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82610EC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82610EC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82610ECC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82610ED0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82610ED4: 4BE55F4D  bl 0x82466e20
	ctx.lr = 0x82610ED8;
	sub_82466E20(ctx, base);
	// 82610ED8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82610EDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82610EE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82610EE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82610EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82610EE8 size=112
    let mut pc: u32 = 0x82610EE8;
    'dispatch: loop {
        match pc {
            0x82610EE8 => {
    //   block [0x82610EE8..0x82610F58)
	// 82610EE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82610EEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82610EF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82610EF4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610EF8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82610EFC: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 82610F00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82610F04: 390B4D10  addi r8, r11, 0x4d10
	ctx.r[8].s64 = ctx.r[11].s64 + 19728;
	// 82610F08: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82610F0C: 388A2DBC  addi r4, r10, 0x2dbc
	ctx.r[4].s64 = ctx.r[10].s64 + 11708;
	// 82610F10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82610F14: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610F18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82610F1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82610F20: 386A953C  addi r3, r10, -0x6ac4
	ctx.r[3].s64 = ctx.r[10].s64 + -27332;
	// 82610F24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82610F28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82610F2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82610F30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82610F34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82610F38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82610F3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82610F40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82610F44: 4BE55EDD  bl 0x82466e20
	ctx.lr = 0x82610F48;
	sub_82466E20(ctx, base);
	// 82610F48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82610F4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82610F50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82610F54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82610F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82610F58 size=24
    let mut pc: u32 = 0x82610F58;
    'dispatch: loop {
        match pc {
            0x82610F58 => {
    //   block [0x82610F58..0x82610F70)
	// 82610F58: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82610F5C: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82610F60: 394ACD78  addi r10, r10, -0x3288
	ctx.r[10].s64 = ctx.r[10].s64 + -12936;
	// 82610F64: 816B4D58  lwz r11, 0x4d58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19800 as u32) ) } as u64;
	// 82610F68: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82610F6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82610F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82610F70 size=116
    let mut pc: u32 = 0x82610F70;
    'dispatch: loop {
        match pc {
            0x82610F70 => {
    //   block [0x82610F70..0x82610FE4)
	// 82610F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82610F74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82610F78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82610F7C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82610F80: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82610F84: 390BCD78  addi r8, r11, -0x3288
	ctx.r[8].s64 = ctx.r[11].s64 + -12936;
	// 82610F88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82610F8C: 392AEE5C  addi r9, r10, -0x11a4
	ctx.r[9].s64 = ctx.r[10].s64 + -4516;
	// 82610F90: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82610F94: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 82610F98: 38AA971C  addi r5, r10, -0x68e4
	ctx.r[5].s64 = ctx.r[10].s64 + -26852;
	// 82610F9C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82610FA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82610FA4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82610FA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82610FAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82610FB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82610FB4: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82610FB8: 388A2DCC  addi r4, r10, 0x2dcc
	ctx.r[4].s64 = ctx.r[10].s64 + 11724;
	// 82610FBC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82610FC0: 386B956C  addi r3, r11, -0x6a94
	ctx.r[3].s64 = ctx.r[11].s64 + -27284;
	// 82610FC4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82610FC8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82610FCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82610FD0: 4BE55E51  bl 0x82466e20
	ctx.lr = 0x82610FD4;
	sub_82466E20(ctx, base);
	// 82610FD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82610FD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82610FDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82610FE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82610FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82610FE8 size=108
    let mut pc: u32 = 0x82610FE8;
    'dispatch: loop {
        match pc {
            0x82610FE8 => {
    //   block [0x82610FE8..0x82611054)
	// 82610FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82610FEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82610FF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82610FF4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82610FF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82610FFC: 38EB4D60  addi r7, r11, 0x4d60
	ctx.r[7].s64 = ctx.r[11].s64 + 19808;
	// 82611000: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82611004: 388A2DD8  addi r4, r10, 0x2dd8
	ctx.r[4].s64 = ctx.r[10].s64 + 11736;
	// 82611008: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261100C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611010: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82611014: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82611018: 386A959C  addi r3, r10, -0x6a64
	ctx.r[3].s64 = ctx.r[10].s64 + -27236;
	// 8261101C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82611020: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82611024: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82611028: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261102C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82611030: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82611034: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82611038: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261103C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82611040: 4BE55DE1  bl 0x82466e20
	ctx.lr = 0x82611044;
	sub_82466E20(ctx, base);
	// 82611044: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82611048: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261104C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82611050: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82611058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82611058 size=108
    let mut pc: u32 = 0x82611058;
    'dispatch: loop {
        match pc {
            0x82611058 => {
    //   block [0x82611058..0x826110C4)
	// 82611058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261105C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82611060: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82611064: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82611068: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261106C: 38EB4DC0  addi r7, r11, 0x4dc0
	ctx.r[7].s64 = ctx.r[11].s64 + 19904;
	// 82611070: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 82611074: 388A2DF0  addi r4, r10, 0x2df0
	ctx.r[4].s64 = ctx.r[10].s64 + 11760;
	// 82611078: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261107C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611080: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82611084: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82611088: 386A95CC  addi r3, r10, -0x6a34
	ctx.r[3].s64 = ctx.r[10].s64 + -27188;
	// 8261108C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82611090: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82611094: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82611098: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261109C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826110A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826110A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826110A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826110AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826110B0: 4BE55D71  bl 0x82466e20
	ctx.lr = 0x826110B4;
	sub_82466E20(ctx, base);
	// 826110B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826110B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826110BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826110C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826110C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826110C8 size=112
    let mut pc: u32 = 0x826110C8;
    'dispatch: loop {
        match pc {
            0x826110C8 => {
    //   block [0x826110C8..0x82611138)
	// 826110C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826110CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826110D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826110D4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 826110D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826110DC: 392BEE90  addi r9, r11, -0x1170
	ctx.r[9].s64 = ctx.r[11].s64 + -4464;
	// 826110E0: 38C0000E  li r6, 0xe
	ctx.r[6].s64 = 14;
	// 826110E4: 38E90018  addi r7, r9, 0x18
	ctx.r[7].s64 = ctx.r[9].s64 + 24;
	// 826110E8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826110EC: 388A2E00  addi r4, r10, 0x2e00
	ctx.r[4].s64 = ctx.r[10].s64 + 11776;
	// 826110F0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826110F4: 396B4E68  addi r11, r11, 0x4e68
	ctx.r[11].s64 = ctx.r[11].s64 + 20072;
	// 826110F8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826110FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611100: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82611104: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82611108: 386A95FC  addi r3, r10, -0x6a04
	ctx.r[3].s64 = ctx.r[10].s64 + -27140;
	// 8261110C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82611110: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82611114: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82611118: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8261111C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82611120: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82611124: 4BE55CFD  bl 0x82466e20
	ctx.lr = 0x82611128;
	sub_82466E20(ctx, base);
	// 82611128: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261112C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82611130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82611134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82611138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82611138 size=112
    let mut pc: u32 = 0x82611138;
    'dispatch: loop {
        match pc {
            0x82611138 => {
    //   block [0x82611138..0x826111A8)
	// 82611138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261113C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82611140: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82611144: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611148: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8261114C: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 82611150: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82611154: 390B4FB8  addi r8, r11, 0x4fb8
	ctx.r[8].s64 = ctx.r[11].s64 + 20408;
	// 82611158: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8261115C: 388A2E18  addi r4, r10, 0x2e18
	ctx.r[4].s64 = ctx.r[10].s64 + 11800;
	// 82611160: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82611164: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611168: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261116C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82611170: 386A962C  addi r3, r10, -0x69d4
	ctx.r[3].s64 = ctx.r[10].s64 + -27092;
	// 82611174: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82611178: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261117C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82611180: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82611184: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82611188: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261118C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82611190: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82611194: 4BE55C8D  bl 0x82466e20
	ctx.lr = 0x82611198;
	sub_82466E20(ctx, base);
	// 82611198: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261119C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826111A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826111A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826111A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826111A8 size=112
    let mut pc: u32 = 0x826111A8;
    'dispatch: loop {
        match pc {
            0x826111A8 => {
    //   block [0x826111A8..0x82611218)
	// 826111A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826111AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826111B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826111B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826111B8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826111BC: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 826111C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826111C4: 390B5060  addi r8, r11, 0x5060
	ctx.r[8].s64 = ctx.r[11].s64 + 20576;
	// 826111C8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826111CC: 388A2E30  addi r4, r10, 0x2e30
	ctx.r[4].s64 = ctx.r[10].s64 + 11824;
	// 826111D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826111D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826111D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826111DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826111E0: 386A965C  addi r3, r10, -0x69a4
	ctx.r[3].s64 = ctx.r[10].s64 + -27044;
	// 826111E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826111E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826111EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826111F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826111F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826111F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826111FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82611200: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82611204: 4BE55C1D  bl 0x82466e20
	ctx.lr = 0x82611208;
	sub_82466E20(ctx, base);
	// 82611208: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261120C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82611210: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82611214: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82611218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82611218 size=112
    let mut pc: u32 = 0x82611218;
    'dispatch: loop {
        match pc {
            0x82611218 => {
    //   block [0x82611218..0x82611288)
	// 82611218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261121C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82611220: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82611224: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611228: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8261122C: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 82611230: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82611234: 390B50F0  addi r8, r11, 0x50f0
	ctx.r[8].s64 = ctx.r[11].s64 + 20720;
	// 82611238: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8261123C: 388A2E44  addi r4, r10, 0x2e44
	ctx.r[4].s64 = ctx.r[10].s64 + 11844;
	// 82611240: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82611244: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611248: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261124C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82611250: 386A968C  addi r3, r10, -0x6974
	ctx.r[3].s64 = ctx.r[10].s64 + -26996;
	// 82611254: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82611258: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261125C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82611260: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82611264: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82611268: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261126C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82611270: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82611274: 4BE55BAD  bl 0x82466e20
	ctx.lr = 0x82611278;
	sub_82466E20(ctx, base);
	// 82611278: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261127C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82611280: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82611284: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82611288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82611288 size=108
    let mut pc: u32 = 0x82611288;
    'dispatch: loop {
        match pc {
            0x82611288 => {
    //   block [0x82611288..0x826112F4)
	// 82611288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261128C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82611290: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82611294: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82611298: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261129C: 38EB5168  addi r7, r11, 0x5168
	ctx.r[7].s64 = ctx.r[11].s64 + 20840;
	// 826112A0: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826112A4: 388A2E58  addi r4, r10, 0x2e58
	ctx.r[4].s64 = ctx.r[10].s64 + 11864;
	// 826112A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826112AC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826112B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826112B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826112B8: 386A96BC  addi r3, r10, -0x6944
	ctx.r[3].s64 = ctx.r[10].s64 + -26948;
	// 826112BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826112C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826112C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826112C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826112CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826112D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826112D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826112D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826112DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826112E0: 4BE55B41  bl 0x82466e20
	ctx.lr = 0x826112E4;
	sub_82466E20(ctx, base);
	// 826112E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826112E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826112EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826112F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826112F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826112F8 size=112
    let mut pc: u32 = 0x826112F8;
    'dispatch: loop {
        match pc {
            0x826112F8 => {
    //   block [0x826112F8..0x82611368)
	// 826112F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826112FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82611300: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82611304: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82611308: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8261130C: 392AEEF0  addi r9, r10, -0x1110
	ctx.r[9].s64 = ctx.r[10].s64 + -4368;
	// 82611310: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82611314: 390B5214  addi r8, r11, 0x5214
	ctx.r[8].s64 = ctx.r[11].s64 + 21012;
	// 82611318: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 8261131C: 388A2E64  addi r4, r10, 0x2e64
	ctx.r[4].s64 = ctx.r[10].s64 + 11876;
	// 82611320: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82611324: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611328: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261132C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82611330: 386A96EC  addi r3, r10, -0x6914
	ctx.r[3].s64 = ctx.r[10].s64 + -26900;
	// 82611334: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82611338: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8261133C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82611340: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82611344: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82611348: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261134C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82611350: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82611354: 4BE55ACD  bl 0x82466e20
	ctx.lr = 0x82611358;
	sub_82466E20(ctx, base);
	// 82611358: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261135C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82611360: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82611364: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82611368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82611368 size=100
    let mut pc: u32 = 0x82611368;
    'dispatch: loop {
        match pc {
            0x82611368 => {
    //   block [0x82611368..0x826113CC)
	// 82611368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261136C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82611370: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82611374: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611378: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261137C: 38AA9ECC  addi r5, r10, -0x6134
	ctx.r[5].s64 = ctx.r[10].s64 + -24884;
	// 82611380: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82611384: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82611388: 388A2E70  addi r4, r10, 0x2e70
	ctx.r[4].s64 = ctx.r[10].s64 + 11888;
	// 8261138C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611390: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82611394: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82611398: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261139C: 386A971C  addi r3, r10, -0x68e4
	ctx.r[3].s64 = ctx.r[10].s64 + -26852;
	// 826113A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826113A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826113A8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826113AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826113B0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826113B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826113B8: 4BE55A69  bl 0x82466e20
	ctx.lr = 0x826113BC;
	sub_82466E20(ctx, base);
	// 826113BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826113C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826113C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826113C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826113D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826113D0 size=108
    let mut pc: u32 = 0x826113D0;
    'dispatch: loop {
        match pc {
            0x826113D0 => {
    //   block [0x826113D0..0x8261143C)
	// 826113D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826113D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826113D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826113DC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826113E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826113E4: 38EB5248  addi r7, r11, 0x5248
	ctx.r[7].s64 = ctx.r[11].s64 + 21064;
	// 826113E8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826113EC: 388A2E80  addi r4, r10, 0x2e80
	ctx.r[4].s64 = ctx.r[10].s64 + 11904;
	// 826113F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826113F4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826113F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826113FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82611400: 386A974C  addi r3, r10, -0x68b4
	ctx.r[3].s64 = ctx.r[10].s64 + -26804;
	// 82611404: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82611408: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261140C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82611410: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82611414: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82611418: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261141C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82611420: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82611424: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82611428: 4BE559F9  bl 0x82466e20
	ctx.lr = 0x8261142C;
	sub_82466E20(ctx, base);
	// 8261142C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82611430: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82611434: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82611438: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82611440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82611440 size=112
    let mut pc: u32 = 0x82611440;
    'dispatch: loop {
        match pc {
            0x82611440 => {
    //   block [0x82611440..0x826114B0)
	// 82611440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82611444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82611448: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261144C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82611450: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82611454: 392AEF50  addi r9, r10, -0x10b0
	ctx.r[9].s64 = ctx.r[10].s64 + -4272;
	// 82611458: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261145C: 390B5278  addi r8, r11, 0x5278
	ctx.r[8].s64 = ctx.r[11].s64 + 21112;
	// 82611460: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82611464: 388A2EA8  addi r4, r10, 0x2ea8
	ctx.r[4].s64 = ctx.r[10].s64 + 11944;
	// 82611468: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261146C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611470: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82611474: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82611478: 386A977C  addi r3, r10, -0x6884
	ctx.r[3].s64 = ctx.r[10].s64 + -26756;
	// 8261147C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82611480: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82611484: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82611488: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261148C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82611490: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82611494: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82611498: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261149C: 4BE55985  bl 0x82466e20
	ctx.lr = 0x826114A0;
	sub_82466E20(ctx, base);
	// 826114A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826114A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826114A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826114AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826114B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826114B0 size=112
    let mut pc: u32 = 0x826114B0;
    'dispatch: loop {
        match pc {
            0x826114B0 => {
    //   block [0x826114B0..0x82611520)
	// 826114B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826114B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826114B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826114BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826114C0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826114C4: 38AA971C  addi r5, r10, -0x68e4
	ctx.r[5].s64 = ctx.r[10].s64 + -26852;
	// 826114C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826114CC: 390B52F0  addi r8, r11, 0x52f0
	ctx.r[8].s64 = ctx.r[11].s64 + 21232;
	// 826114D0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826114D4: 388A2EBC  addi r4, r10, 0x2ebc
	ctx.r[4].s64 = ctx.r[10].s64 + 11964;
	// 826114D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826114DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826114E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826114E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826114E8: 386A97AC  addi r3, r10, -0x6854
	ctx.r[3].s64 = ctx.r[10].s64 + -26708;
	// 826114EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826114F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826114F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826114F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826114FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82611500: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82611504: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82611508: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261150C: 4BE55915  bl 0x82466e20
	ctx.lr = 0x82611510;
	sub_82466E20(ctx, base);
	// 82611510: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82611514: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82611518: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261151C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82611520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82611520 size=116
    let mut pc: u32 = 0x82611520;
    'dispatch: loop {
        match pc {
            0x82611520 => {
    //   block [0x82611520..0x82611594)
	// 82611520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82611524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82611528: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261152C: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82611530: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82611534: 390A5320  addi r8, r10, 0x5320
	ctx.r[8].s64 = ctx.r[10].s64 + 21280;
	// 82611538: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261153C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82611540: 38AA971C  addi r5, r10, -0x68e4
	ctx.r[5].s64 = ctx.r[10].s64 + -26852;
	// 82611544: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82611548: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8261154C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82611550: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82611554: 388A2EDC  addi r4, r10, 0x2edc
	ctx.r[4].s64 = ctx.r[10].s64 + 11996;
	// 82611558: 396BEF64  addi r11, r11, -0x109c
	ctx.r[11].s64 = ctx.r[11].s64 + -4252;
	// 8261155C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611560: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82611564: 386A97DC  addi r3, r10, -0x6824
	ctx.r[3].s64 = ctx.r[10].s64 + -26660;
	// 82611568: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8261156C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82611570: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82611574: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82611578: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261157C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82611580: 4BE558A1  bl 0x82466e20
	ctx.lr = 0x82611584;
	sub_82466E20(ctx, base);
	// 82611584: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82611588: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261158C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82611590: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82611598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82611598 size=100
    let mut pc: u32 = 0x82611598;
    'dispatch: loop {
        match pc {
            0x82611598 => {
    //   block [0x82611598..0x826115FC)
	// 82611598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261159C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826115A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826115A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826115A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826115AC: 38AA97DC  addi r5, r10, -0x6824
	ctx.r[5].s64 = ctx.r[10].s64 + -26660;
	// 826115B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826115B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826115B8: 388A2EF8  addi r4, r10, 0x2ef8
	ctx.r[4].s64 = ctx.r[10].s64 + 12024;
	// 826115BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826115C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826115C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826115C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826115CC: 386A980C  addi r3, r10, -0x67f4
	ctx.r[3].s64 = ctx.r[10].s64 + -26612;
	// 826115D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826115D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826115D8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826115DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826115E0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826115E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826115E8: 4BE55839  bl 0x82466e20
	ctx.lr = 0x826115EC;
	sub_82466E20(ctx, base);
	// 826115EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826115F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826115F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826115F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82611600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82611600 size=24
    let mut pc: u32 = 0x82611600;
    'dispatch: loop {
        match pc {
            0x82611600 => {
    //   block [0x82611600..0x82611618)
	// 82611600: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82611604: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82611608: 394ACE80  addi r10, r10, -0x3180
	ctx.r[10].s64 = ctx.r[10].s64 + -12672;
	// 8261160C: 816B53C8  lwz r11, 0x53c8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(21448 as u32) ) } as u64;
	// 82611610: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82611614: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82611618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82611618 size=116
    let mut pc: u32 = 0x82611618;
    'dispatch: loop {
        match pc {
            0x82611618 => {
    //   block [0x82611618..0x8261168C)
	// 82611618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261161C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82611620: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82611624: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82611628: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261162C: 392BEFA0  addi r9, r11, -0x1060
	ctx.r[9].s64 = ctx.r[11].s64 + -4192;
	// 82611630: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 82611634: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82611638: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 8261163C: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 82611640: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82611644: 388A2F1C  addi r4, r10, 0x2f1c
	ctx.r[4].s64 = ctx.r[10].s64 + 12060;
	// 82611648: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261164C: 396BCE80  addi r11, r11, -0x3180
	ctx.r[11].s64 = ctx.r[11].s64 + -12672;
	// 82611650: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82611654: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611658: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8261165C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82611660: 386A983C  addi r3, r10, -0x67c4
	ctx.r[3].s64 = ctx.r[10].s64 + -26564;
	// 82611664: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82611668: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8261166C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82611670: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82611674: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82611678: 4BE557A9  bl 0x82466e20
	ctx.lr = 0x8261167C;
	sub_82466E20(ctx, base);
	// 8261167C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82611680: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82611684: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82611688: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82611690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82611690 size=116
    let mut pc: u32 = 0x82611690;
    'dispatch: loop {
        match pc {
            0x82611690 => {
    //   block [0x82611690..0x82611704)
	// 82611690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82611694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82611698: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261169C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 826116A0: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826116A4: 392BEFF4  addi r9, r11, -0x100c
	ctx.r[9].s64 = ctx.r[11].s64 + -4108;
	// 826116A8: 38AA971C  addi r5, r10, -0x68e4
	ctx.r[5].s64 = ctx.r[10].s64 + -26852;
	// 826116AC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826116B0: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826116B4: 38C00009  li r6, 9
	ctx.r[6].s64 = 9;
	// 826116B8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826116BC: 388A2F38  addi r4, r10, 0x2f38
	ctx.r[4].s64 = ctx.r[10].s64 + 12088;
	// 826116C0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826116C4: 396B53D0  addi r11, r11, 0x53d0
	ctx.r[11].s64 = ctx.r[11].s64 + 21456;
	// 826116C8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826116CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826116D0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826116D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826116D8: 386A986C  addi r3, r10, -0x6794
	ctx.r[3].s64 = ctx.r[10].s64 + -26516;
	// 826116DC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826116E0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826116E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826116E8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826116EC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826116F0: 4BE55731  bl 0x82466e20
	ctx.lr = 0x826116F4;
	sub_82466E20(ctx, base);
	// 826116F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826116F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826116FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82611700: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82611708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82611708 size=108
    let mut pc: u32 = 0x82611708;
    'dispatch: loop {
        match pc {
            0x82611708 => {
    //   block [0x82611708..0x82611774)
	// 82611708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261170C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82611710: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82611714: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82611718: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261171C: 38EB54A8  addi r7, r11, 0x54a8
	ctx.r[7].s64 = ctx.r[11].s64 + 21672;
	// 82611720: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82611724: 388A2F4C  addi r4, r10, 0x2f4c
	ctx.r[4].s64 = ctx.r[10].s64 + 12108;
	// 82611728: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261172C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611730: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82611734: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82611738: 386A989C  addi r3, r10, -0x6764
	ctx.r[3].s64 = ctx.r[10].s64 + -26468;
	// 8261173C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82611740: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82611744: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82611748: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261174C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82611750: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82611754: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82611758: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261175C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82611760: 4BE556C1  bl 0x82466e20
	ctx.lr = 0x82611764;
	sub_82466E20(ctx, base);
	// 82611764: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82611768: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261176C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82611770: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82611778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82611778 size=24
    let mut pc: u32 = 0x82611778;
    'dispatch: loop {
        match pc {
            0x82611778 => {
    //   block [0x82611778..0x82611790)
	// 82611778: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8261177C: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82611780: 394ACF28  addi r10, r10, -0x30d8
	ctx.r[10].s64 = ctx.r[10].s64 + -12504;
	// 82611784: 816B5508  lwz r11, 0x5508(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(21768 as u32) ) } as u64;
	// 82611788: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8261178C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82611790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82611790 size=116
    let mut pc: u32 = 0x82611790;
    'dispatch: loop {
        match pc {
            0x82611790 => {
    //   block [0x82611790..0x82611804)
	// 82611790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82611794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82611798: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261179C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826117A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826117A4: 390BCF28  addi r8, r11, -0x30d8
	ctx.r[8].s64 = ctx.r[11].s64 + -12504;
	// 826117A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826117AC: 392AF068  addi r9, r10, -0xf98
	ctx.r[9].s64 = ctx.r[10].s64 + -3992;
	// 826117B0: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826117B4: 38E00011  li r7, 0x11
	ctx.r[7].s64 = 17;
	// 826117B8: 38AA971C  addi r5, r10, -0x68e4
	ctx.r[5].s64 = ctx.r[10].s64 + -26852;
	// 826117BC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826117C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826117C4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826117C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826117CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826117D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826117D4: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 826117D8: 388A3040  addi r4, r10, 0x3040
	ctx.r[4].s64 = ctx.r[10].s64 + 12352;
	// 826117DC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826117E0: 386B98CC  addi r3, r11, -0x6734
	ctx.r[3].s64 = ctx.r[11].s64 + -26420;
	// 826117E4: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826117E8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826117EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826117F0: 4BE55631  bl 0x82466e20
	ctx.lr = 0x826117F4;
	sub_82466E20(ctx, base);
	// 826117F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826117F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826117FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82611800: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82611808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82611808 size=112
    let mut pc: u32 = 0x82611808;
    'dispatch: loop {
        match pc {
            0x82611808 => {
    //   block [0x82611808..0x82611878)
	// 82611808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261180C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82611810: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82611814: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611818: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8261181C: 38AA971C  addi r5, r10, -0x68e4
	ctx.r[5].s64 = ctx.r[10].s64 + -26852;
	// 82611820: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82611824: 390B5510  addi r8, r11, 0x5510
	ctx.r[8].s64 = ctx.r[11].s64 + 21776;
	// 82611828: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8261182C: 388A3054  addi r4, r10, 0x3054
	ctx.r[4].s64 = ctx.r[10].s64 + 12372;
	// 82611830: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82611834: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611838: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261183C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82611840: 386A98FC  addi r3, r10, -0x6704
	ctx.r[3].s64 = ctx.r[10].s64 + -26372;
	// 82611844: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82611848: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261184C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82611850: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82611854: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82611858: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261185C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82611860: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82611864: 4BE555BD  bl 0x82466e20
	ctx.lr = 0x82611868;
	sub_82466E20(ctx, base);
	// 82611868: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261186C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82611870: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82611874: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82611878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82611878 size=24
    let mut pc: u32 = 0x82611878;
    'dispatch: loop {
        match pc {
            0x82611878 => {
    //   block [0x82611878..0x82611890)
	// 82611878: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8261187C: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82611880: 394AD0C0  addi r10, r10, -0x2f40
	ctx.r[10].s64 = ctx.r[10].s64 + -12096;
	// 82611884: 816B5540  lwz r11, 0x5540(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(21824 as u32) ) } as u64;
	// 82611888: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8261188C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82611890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82611890 size=116
    let mut pc: u32 = 0x82611890;
    'dispatch: loop {
        match pc {
            0x82611890 => {
    //   block [0x82611890..0x82611904)
	// 82611890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82611894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82611898: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261189C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826118A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826118A4: 390BD0C0  addi r8, r11, -0x2f40
	ctx.r[8].s64 = ctx.r[11].s64 + -12096;
	// 826118A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826118AC: 392AF0A0  addi r9, r10, -0xf60
	ctx.r[9].s64 = ctx.r[10].s64 + -3936;
	// 826118B0: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826118B4: 38E0000A  li r7, 0xa
	ctx.r[7].s64 = 10;
	// 826118B8: 38AA986C  addi r5, r10, -0x6794
	ctx.r[5].s64 = ctx.r[10].s64 + -26516;
	// 826118BC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826118C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826118C4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826118C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826118CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826118D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826118D4: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 826118D8: 388A30BC  addi r4, r10, 0x30bc
	ctx.r[4].s64 = ctx.r[10].s64 + 12476;
	// 826118DC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826118E0: 386B992C  addi r3, r11, -0x66d4
	ctx.r[3].s64 = ctx.r[11].s64 + -26324;
	// 826118E4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826118E8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826118EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826118F0: 4BE55531  bl 0x82466e20
	ctx.lr = 0x826118F4;
	sub_82466E20(ctx, base);
	// 826118F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826118F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826118FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82611900: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82611908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82611908 size=112
    let mut pc: u32 = 0x82611908;
    'dispatch: loop {
        match pc {
            0x82611908 => {
    //   block [0x82611908..0x82611978)
	// 82611908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261190C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82611910: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82611914: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611918: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8261191C: 38AA971C  addi r5, r10, -0x68e4
	ctx.r[5].s64 = ctx.r[10].s64 + -26852;
	// 82611920: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82611924: 390B5544  addi r8, r11, 0x5544
	ctx.r[8].s64 = ctx.r[11].s64 + 21828;
	// 82611928: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261192C: 388A30D8  addi r4, r10, 0x30d8
	ctx.r[4].s64 = ctx.r[10].s64 + 12504;
	// 82611930: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82611934: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611938: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261193C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82611940: 386A995C  addi r3, r10, -0x66a4
	ctx.r[3].s64 = ctx.r[10].s64 + -26276;
	// 82611944: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82611948: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261194C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82611950: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82611954: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82611958: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261195C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82611960: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82611964: 4BE554BD  bl 0x82466e20
	ctx.lr = 0x82611968;
	sub_82466E20(ctx, base);
	// 82611968: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261196C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82611970: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82611974: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82611978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82611978 size=100
    let mut pc: u32 = 0x82611978;
    'dispatch: loop {
        match pc {
            0x82611978 => {
    //   block [0x82611978..0x826119DC)
	// 82611978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261197C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82611980: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82611984: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611988: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261198C: 38AA9ECC  addi r5, r10, -0x6134
	ctx.r[5].s64 = ctx.r[10].s64 + -24884;
	// 82611990: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82611994: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82611998: 388A30F4  addi r4, r10, 0x30f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12532;
	// 8261199C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826119A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826119A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826119A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826119AC: 386A998C  addi r3, r10, -0x6674
	ctx.r[3].s64 = ctx.r[10].s64 + -26228;
	// 826119B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826119B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826119B8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826119BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826119C0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826119C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826119C8: 4BE55459  bl 0x82466e20
	ctx.lr = 0x826119CC;
	sub_82466E20(ctx, base);
	// 826119CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826119D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826119D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826119D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826119E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826119E0 size=108
    let mut pc: u32 = 0x826119E0;
    'dispatch: loop {
        match pc {
            0x826119E0 => {
    //   block [0x826119E0..0x82611A4C)
	// 826119E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826119E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826119E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826119EC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826119F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826119F4: 38EB5560  addi r7, r11, 0x5560
	ctx.r[7].s64 = ctx.r[11].s64 + 21856;
	// 826119F8: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826119FC: 388A3100  addi r4, r10, 0x3100
	ctx.r[4].s64 = ctx.r[10].s64 + 12544;
	// 82611A00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82611A04: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611A08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82611A0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82611A10: 386A99BC  addi r3, r10, -0x6644
	ctx.r[3].s64 = ctx.r[10].s64 + -26180;
	// 82611A14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82611A18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82611A1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82611A20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82611A24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82611A28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82611A2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82611A30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82611A34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82611A38: 4BE553E9  bl 0x82466e20
	ctx.lr = 0x82611A3C;
	sub_82466E20(ctx, base);
	// 82611A3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82611A40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82611A44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82611A48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82611A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82611A50 size=112
    let mut pc: u32 = 0x82611A50;
    'dispatch: loop {
        match pc {
            0x82611A50 => {
    //   block [0x82611A50..0x82611AC0)
	// 82611A50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82611A54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82611A58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82611A5C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611A60: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82611A64: 38AA998C  addi r5, r10, -0x6674
	ctx.r[5].s64 = ctx.r[10].s64 + -26228;
	// 82611A68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82611A6C: 390B5638  addi r8, r11, 0x5638
	ctx.r[8].s64 = ctx.r[11].s64 + 22072;
	// 82611A70: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82611A74: 388A312C  addi r4, r10, 0x312c
	ctx.r[4].s64 = ctx.r[10].s64 + 12588;
	// 82611A78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82611A7C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611A80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82611A84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82611A88: 386A99EC  addi r3, r10, -0x6614
	ctx.r[3].s64 = ctx.r[10].s64 + -26132;
	// 82611A8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82611A90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82611A94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82611A98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82611A9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82611AA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82611AA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82611AA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82611AAC: 4BE55375  bl 0x82466e20
	ctx.lr = 0x82611AB0;
	sub_82466E20(ctx, base);
	// 82611AB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82611AB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82611AB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82611ABC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82611AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82611AC0 size=108
    let mut pc: u32 = 0x82611AC0;
    'dispatch: loop {
        match pc {
            0x82611AC0 => {
    //   block [0x82611AC0..0x82611B2C)
	// 82611AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82611AC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82611AC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82611ACC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82611AD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82611AD4: 38EB5668  addi r7, r11, 0x5668
	ctx.r[7].s64 = ctx.r[11].s64 + 22120;
	// 82611AD8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82611ADC: 388A3144  addi r4, r10, 0x3144
	ctx.r[4].s64 = ctx.r[10].s64 + 12612;
	// 82611AE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82611AE4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611AE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82611AEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82611AF0: 386A9A1C  addi r3, r10, -0x65e4
	ctx.r[3].s64 = ctx.r[10].s64 + -26084;
	// 82611AF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82611AF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82611AFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82611B00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82611B04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82611B08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82611B0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82611B10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82611B14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82611B18: 4BE55309  bl 0x82466e20
	ctx.lr = 0x82611B1C;
	sub_82466E20(ctx, base);
	// 82611B1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82611B20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82611B24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82611B28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82611B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82611B30 size=112
    let mut pc: u32 = 0x82611B30;
    'dispatch: loop {
        match pc {
            0x82611B30 => {
    //   block [0x82611B30..0x82611BA0)
	// 82611B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82611B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82611B38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82611B3C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611B40: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82611B44: 38AA998C  addi r5, r10, -0x6674
	ctx.r[5].s64 = ctx.r[10].s64 + -26228;
	// 82611B48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82611B4C: 390B5698  addi r8, r11, 0x5698
	ctx.r[8].s64 = ctx.r[11].s64 + 22168;
	// 82611B50: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82611B54: 388A3164  addi r4, r10, 0x3164
	ctx.r[4].s64 = ctx.r[10].s64 + 12644;
	// 82611B58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82611B5C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611B60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82611B64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82611B68: 386A9A4C  addi r3, r10, -0x65b4
	ctx.r[3].s64 = ctx.r[10].s64 + -26036;
	// 82611B6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82611B70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82611B74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82611B78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82611B7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82611B80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82611B84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82611B88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82611B8C: 4BE55295  bl 0x82466e20
	ctx.lr = 0x82611B90;
	sub_82466E20(ctx, base);
	// 82611B90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82611B94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82611B98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82611B9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82611BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82611BA0 size=112
    let mut pc: u32 = 0x82611BA0;
    'dispatch: loop {
        match pc {
            0x82611BA0 => {
    //   block [0x82611BA0..0x82611C10)
	// 82611BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82611BA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82611BA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82611BAC: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82611BB0: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 82611BB4: 38EA56B0  addi r7, r10, 0x56b0
	ctx.r[7].s64 = ctx.r[10].s64 + 22192;
	// 82611BB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82611BBC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82611BC0: 388A317C  addi r4, r10, 0x317c
	ctx.r[4].s64 = ctx.r[10].s64 + 12668;
	// 82611BC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82611BC8: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82611BCC: 396BF0B4  addi r11, r11, -0xf4c
	ctx.r[11].s64 = ctx.r[11].s64 + -3916;
	// 82611BD0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82611BD4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611BD8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82611BDC: 386A9A7C  addi r3, r10, -0x6584
	ctx.r[3].s64 = ctx.r[10].s64 + -25988;
	// 82611BE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82611BE4: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82611BE8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82611BEC: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82611BF0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82611BF4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82611BF8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82611BFC: 4BE55225  bl 0x82466e20
	ctx.lr = 0x82611C00;
	sub_82466E20(ctx, base);
	// 82611C00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82611C04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82611C08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82611C0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82611C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82611C10 size=108
    let mut pc: u32 = 0x82611C10;
    'dispatch: loop {
        match pc {
            0x82611C10 => {
    //   block [0x82611C10..0x82611C7C)
	// 82611C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82611C14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82611C18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82611C1C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82611C20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82611C24: 38EB5788  addi r7, r11, 0x5788
	ctx.r[7].s64 = ctx.r[11].s64 + 22408;
	// 82611C28: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82611C2C: 388A318C  addi r4, r10, 0x318c
	ctx.r[4].s64 = ctx.r[10].s64 + 12684;
	// 82611C30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82611C34: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611C38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82611C3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82611C40: 386A9AAC  addi r3, r10, -0x6554
	ctx.r[3].s64 = ctx.r[10].s64 + -25940;
	// 82611C44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82611C48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82611C4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82611C50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82611C54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82611C58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82611C5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82611C60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82611C64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82611C68: 4BE551B9  bl 0x82466e20
	ctx.lr = 0x82611C6C;
	sub_82466E20(ctx, base);
	// 82611C6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82611C70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82611C74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82611C78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82611C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82611C80 size=108
    let mut pc: u32 = 0x82611C80;
    'dispatch: loop {
        match pc {
            0x82611C80 => {
    //   block [0x82611C80..0x82611CEC)
	// 82611C80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82611C84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82611C88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82611C8C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82611C90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82611C94: 38EB57A0  addi r7, r11, 0x57a0
	ctx.r[7].s64 = ctx.r[11].s64 + 22432;
	// 82611C98: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 82611C9C: 388A31A4  addi r4, r10, 0x31a4
	ctx.r[4].s64 = ctx.r[10].s64 + 12708;
	// 82611CA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82611CA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611CA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82611CAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82611CB0: 386A9ADC  addi r3, r10, -0x6524
	ctx.r[3].s64 = ctx.r[10].s64 + -25892;
	// 82611CB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82611CB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82611CBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82611CC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82611CC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82611CC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82611CCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82611CD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82611CD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82611CD8: 4BE55149  bl 0x82466e20
	ctx.lr = 0x82611CDC;
	sub_82466E20(ctx, base);
	// 82611CDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82611CE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82611CE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82611CE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82611CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82611CF0 size=108
    let mut pc: u32 = 0x82611CF0;
    'dispatch: loop {
        match pc {
            0x82611CF0 => {
    //   block [0x82611CF0..0x82611D5C)
	// 82611CF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82611CF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82611CF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82611CFC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82611D00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82611D04: 38EB58A8  addi r7, r11, 0x58a8
	ctx.r[7].s64 = ctx.r[11].s64 + 22696;
	// 82611D08: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82611D0C: 388A31BC  addi r4, r10, 0x31bc
	ctx.r[4].s64 = ctx.r[10].s64 + 12732;
	// 82611D10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82611D14: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611D18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82611D1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82611D20: 386A9B0C  addi r3, r10, -0x64f4
	ctx.r[3].s64 = ctx.r[10].s64 + -25844;
	// 82611D24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82611D28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82611D2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82611D30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82611D34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82611D38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82611D3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82611D40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82611D44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82611D48: 4BE550D9  bl 0x82466e20
	ctx.lr = 0x82611D4C;
	sub_82466E20(ctx, base);
	// 82611D4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82611D50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82611D54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82611D58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82611D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82611D60 size=112
    let mut pc: u32 = 0x82611D60;
    'dispatch: loop {
        match pc {
            0x82611D60 => {
    //   block [0x82611D60..0x82611DD0)
	// 82611D60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82611D64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82611D68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82611D6C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611D70: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82611D74: 38AA998C  addi r5, r10, -0x6674
	ctx.r[5].s64 = ctx.r[10].s64 + -26228;
	// 82611D78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82611D7C: 390B5908  addi r8, r11, 0x5908
	ctx.r[8].s64 = ctx.r[11].s64 + 22792;
	// 82611D80: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 82611D84: 388A31E0  addi r4, r10, 0x31e0
	ctx.r[4].s64 = ctx.r[10].s64 + 12768;
	// 82611D88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82611D8C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611D90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82611D94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82611D98: 386A9B3C  addi r3, r10, -0x64c4
	ctx.r[3].s64 = ctx.r[10].s64 + -25796;
	// 82611D9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82611DA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82611DA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82611DA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82611DAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82611DB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82611DB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82611DB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82611DBC: 4BE55065  bl 0x82466e20
	ctx.lr = 0x82611DC0;
	sub_82466E20(ctx, base);
	// 82611DC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82611DC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82611DC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82611DCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82611DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82611DD0 size=112
    let mut pc: u32 = 0x82611DD0;
    'dispatch: loop {
        match pc {
            0x82611DD0 => {
    //   block [0x82611DD0..0x82611E40)
	// 82611DD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82611DD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82611DD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82611DDC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611DE0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82611DE4: 38AA998C  addi r5, r10, -0x6674
	ctx.r[5].s64 = ctx.r[10].s64 + -26228;
	// 82611DE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82611DEC: 390B5A28  addi r8, r11, 0x5a28
	ctx.r[8].s64 = ctx.r[11].s64 + 23080;
	// 82611DF0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82611DF4: 388A31F4  addi r4, r10, 0x31f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12788;
	// 82611DF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82611DFC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611E00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82611E04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82611E08: 386A9B6C  addi r3, r10, -0x6494
	ctx.r[3].s64 = ctx.r[10].s64 + -25748;
	// 82611E0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82611E10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82611E14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82611E18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82611E1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82611E20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82611E24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82611E28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82611E2C: 4BE54FF5  bl 0x82466e20
	ctx.lr = 0x82611E30;
	sub_82466E20(ctx, base);
	// 82611E30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82611E34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82611E38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82611E3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82611E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82611E40 size=116
    let mut pc: u32 = 0x82611E40;
    'dispatch: loop {
        match pc {
            0x82611E40 => {
    //   block [0x82611E40..0x82611EB4)
	// 82611E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82611E44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82611E48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82611E4C: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82611E50: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 82611E54: 390A5A40  addi r8, r10, 0x5a40
	ctx.r[8].s64 = ctx.r[10].s64 + 23104;
	// 82611E58: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611E5C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82611E60: 38AA998C  addi r5, r10, -0x6674
	ctx.r[5].s64 = ctx.r[10].s64 + -26228;
	// 82611E64: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82611E68: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82611E6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82611E70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82611E74: 388A3210  addi r4, r10, 0x3210
	ctx.r[4].s64 = ctx.r[10].s64 + 12816;
	// 82611E78: 396BF0E4  addi r11, r11, -0xf1c
	ctx.r[11].s64 = ctx.r[11].s64 + -3868;
	// 82611E7C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611E80: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82611E84: 386A9B9C  addi r3, r10, -0x6464
	ctx.r[3].s64 = ctx.r[10].s64 + -25700;
	// 82611E88: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82611E8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82611E90: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82611E94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82611E98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82611E9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82611EA0: 4BE54F81  bl 0x82466e20
	ctx.lr = 0x82611EA4;
	sub_82466E20(ctx, base);
	// 82611EA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82611EA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82611EAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82611EB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82611EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82611EB8 size=108
    let mut pc: u32 = 0x82611EB8;
    'dispatch: loop {
        match pc {
            0x82611EB8 => {
    //   block [0x82611EB8..0x82611F24)
	// 82611EB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82611EBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82611EC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82611EC4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82611EC8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82611ECC: 38EB5AA0  addi r7, r11, 0x5aa0
	ctx.r[7].s64 = ctx.r[11].s64 + 23200;
	// 82611ED0: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 82611ED4: 388AA8F8  addi r4, r10, -0x5708
	ctx.r[4].s64 = ctx.r[10].s64 + -22280;
	// 82611ED8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82611EDC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611EE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82611EE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82611EE8: 386A9BCC  addi r3, r10, -0x6434
	ctx.r[3].s64 = ctx.r[10].s64 + -25652;
	// 82611EEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82611EF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82611EF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82611EF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82611EFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82611F00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82611F04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82611F08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82611F0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82611F10: 4BE54F11  bl 0x82466e20
	ctx.lr = 0x82611F14;
	sub_82466E20(ctx, base);
	// 82611F14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82611F18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82611F1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82611F20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82611F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82611F28 size=112
    let mut pc: u32 = 0x82611F28;
    'dispatch: loop {
        match pc {
            0x82611F28 => {
    //   block [0x82611F28..0x82611F98)
	// 82611F28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82611F2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82611F30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82611F34: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82611F38: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82611F3C: 38EA5B48  addi r7, r10, 0x5b48
	ctx.r[7].s64 = ctx.r[10].s64 + 23368;
	// 82611F40: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82611F44: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82611F48: 388AA910  addi r4, r10, -0x56f0
	ctx.r[4].s64 = ctx.r[10].s64 + -22256;
	// 82611F4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82611F50: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82611F54: 396BF0F8  addi r11, r11, -0xf08
	ctx.r[11].s64 = ctx.r[11].s64 + -3848;
	// 82611F58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82611F5C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611F60: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82611F64: 386A9BFC  addi r3, r10, -0x6404
	ctx.r[3].s64 = ctx.r[10].s64 + -25604;
	// 82611F68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82611F6C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82611F70: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82611F74: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82611F78: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82611F7C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82611F80: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82611F84: 4BE54E9D  bl 0x82466e20
	ctx.lr = 0x82611F88;
	sub_82466E20(ctx, base);
	// 82611F88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82611F8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82611F90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82611F94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82611F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82611F98 size=112
    let mut pc: u32 = 0x82611F98;
    'dispatch: loop {
        match pc {
            0x82611F98 => {
    //   block [0x82611F98..0x82612008)
	// 82611F98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82611F9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82611FA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82611FA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611FA8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82611FAC: 38AA998C  addi r5, r10, -0x6674
	ctx.r[5].s64 = ctx.r[10].s64 + -26228;
	// 82611FB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82611FB4: 390B5BC0  addi r8, r11, 0x5bc0
	ctx.r[8].s64 = ctx.r[11].s64 + 23488;
	// 82611FB8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82611FBC: 388A3260  addi r4, r10, 0x3260
	ctx.r[4].s64 = ctx.r[10].s64 + 12896;
	// 82611FC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82611FC4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82611FC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82611FCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82611FD0: 386A9C2C  addi r3, r10, -0x63d4
	ctx.r[3].s64 = ctx.r[10].s64 + -25556;
	// 82611FD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82611FD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82611FDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82611FE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82611FE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82611FE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82611FEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82611FF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82611FF4: 4BE54E2D  bl 0x82466e20
	ctx.lr = 0x82611FF8;
	sub_82466E20(ctx, base);
	// 82611FF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82611FFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82612000: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82612004: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82612008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82612008 size=112
    let mut pc: u32 = 0x82612008;
    'dispatch: loop {
        match pc {
            0x82612008 => {
    //   block [0x82612008..0x82612078)
	// 82612008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261200C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82612010: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82612014: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612018: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8261201C: 38AA998C  addi r5, r10, -0x6674
	ctx.r[5].s64 = ctx.r[10].s64 + -26228;
	// 82612020: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82612024: 390B5C08  addi r8, r11, 0x5c08
	ctx.r[8].s64 = ctx.r[11].s64 + 23560;
	// 82612028: 3920000B  li r9, 0xb
	ctx.r[9].s64 = 11;
	// 8261202C: 388A3274  addi r4, r10, 0x3274
	ctx.r[4].s64 = ctx.r[10].s64 + 12916;
	// 82612030: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82612034: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612038: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261203C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82612040: 386A9C5C  addi r3, r10, -0x63a4
	ctx.r[3].s64 = ctx.r[10].s64 + -25508;
	// 82612044: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82612048: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261204C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82612050: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82612054: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82612058: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261205C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82612060: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82612064: 4BE54DBD  bl 0x82466e20
	ctx.lr = 0x82612068;
	sub_82466E20(ctx, base);
	// 82612068: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261206C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82612070: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82612074: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82612078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82612078 size=112
    let mut pc: u32 = 0x82612078;
    'dispatch: loop {
        match pc {
            0x82612078 => {
    //   block [0x82612078..0x826120E8)
	// 82612078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261207C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82612080: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82612084: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612088: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8261208C: 38AA998C  addi r5, r10, -0x6674
	ctx.r[5].s64 = ctx.r[10].s64 + -26228;
	// 82612090: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82612094: 390B5D10  addi r8, r11, 0x5d10
	ctx.r[8].s64 = ctx.r[11].s64 + 23824;
	// 82612098: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261209C: 388A3288  addi r4, r10, 0x3288
	ctx.r[4].s64 = ctx.r[10].s64 + 12936;
	// 826120A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826120A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826120A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826120AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826120B0: 386A9C8C  addi r3, r10, -0x6374
	ctx.r[3].s64 = ctx.r[10].s64 + -25460;
	// 826120B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826120B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826120BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826120C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826120C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826120C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826120CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826120D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826120D4: 4BE54D4D  bl 0x82466e20
	ctx.lr = 0x826120D8;
	sub_82466E20(ctx, base);
	// 826120D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826120DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826120E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826120E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826120E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826120E8 size=112
    let mut pc: u32 = 0x826120E8;
    'dispatch: loop {
        match pc {
            0x826120E8 => {
    //   block [0x826120E8..0x82612158)
	// 826120E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826120EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826120F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826120F4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826120F8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826120FC: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 82612100: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82612104: 390B5D28  addi r8, r11, 0x5d28
	ctx.r[8].s64 = ctx.r[11].s64 + 23848;
	// 82612108: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8261210C: 388A32A4  addi r4, r10, 0x32a4
	ctx.r[4].s64 = ctx.r[10].s64 + 12964;
	// 82612110: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82612114: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612118: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261211C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82612120: 386A9CBC  addi r3, r10, -0x6344
	ctx.r[3].s64 = ctx.r[10].s64 + -25412;
	// 82612124: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82612128: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261212C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82612130: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82612134: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82612138: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261213C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82612140: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82612144: 4BE54CDD  bl 0x82466e20
	ctx.lr = 0x82612148;
	sub_82466E20(ctx, base);
	// 82612148: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261214C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82612150: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82612154: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82612158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82612158 size=108
    let mut pc: u32 = 0x82612158;
    'dispatch: loop {
        match pc {
            0x82612158 => {
    //   block [0x82612158..0x826121C4)
	// 82612158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261215C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82612160: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82612164: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82612168: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261216C: 38EB5D58  addi r7, r11, 0x5d58
	ctx.r[7].s64 = ctx.r[11].s64 + 23896;
	// 82612170: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82612174: 388A32B4  addi r4, r10, 0x32b4
	ctx.r[4].s64 = ctx.r[10].s64 + 12980;
	// 82612178: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261217C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612180: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82612184: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82612188: 386A9CEC  addi r3, r10, -0x6314
	ctx.r[3].s64 = ctx.r[10].s64 + -25364;
	// 8261218C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82612190: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82612194: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82612198: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261219C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826121A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826121A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826121A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826121AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826121B0: 4BE54C71  bl 0x82466e20
	ctx.lr = 0x826121B4;
	sub_82466E20(ctx, base);
	// 826121B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826121B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826121BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826121C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826121C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826121C8 size=24
    let mut pc: u32 = 0x826121C8;
    'dispatch: loop {
        match pc {
            0x826121C8 => {
    //   block [0x826121C8..0x826121E0)
	// 826121C8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826121CC: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 826121D0: 394AD1B0  addi r10, r10, -0x2e50
	ctx.r[10].s64 = ctx.r[10].s64 + -11856;
	// 826121D4: 816B555C  lwz r11, 0x555c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(21852 as u32) ) } as u64;
	// 826121D8: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826121DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826121E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826121E0 size=116
    let mut pc: u32 = 0x826121E0;
    'dispatch: loop {
        match pc {
            0x826121E0 => {
    //   block [0x826121E0..0x82612254)
	// 826121E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826121E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826121E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826121EC: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826121F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826121F4: 390BD1B0  addi r8, r11, -0x2e50
	ctx.r[8].s64 = ctx.r[11].s64 + -11856;
	// 826121F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826121FC: 392AF130  addi r9, r10, -0xed0
	ctx.r[9].s64 = ctx.r[10].s64 + -3792;
	// 82612200: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612204: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 82612208: 38AA998C  addi r5, r10, -0x6674
	ctx.r[5].s64 = ctx.r[10].s64 + -26228;
	// 8261220C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82612210: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82612214: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82612218: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261221C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82612220: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82612224: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82612228: 388A3300  addi r4, r10, 0x3300
	ctx.r[4].s64 = ctx.r[10].s64 + 13056;
	// 8261222C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82612230: 386B9D1C  addi r3, r11, -0x62e4
	ctx.r[3].s64 = ctx.r[11].s64 + -25316;
	// 82612234: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82612238: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261223C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82612240: 4BE54BE1  bl 0x82466e20
	ctx.lr = 0x82612244;
	sub_82466E20(ctx, base);
	// 82612244: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82612248: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261224C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82612250: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82612258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82612258 size=112
    let mut pc: u32 = 0x82612258;
    'dispatch: loop {
        match pc {
            0x82612258 => {
    //   block [0x82612258..0x826122C8)
	// 82612258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261225C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82612260: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82612264: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612268: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8261226C: 38AA998C  addi r5, r10, -0x6674
	ctx.r[5].s64 = ctx.r[10].s64 + -26228;
	// 82612270: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82612274: 390B5DD0  addi r8, r11, 0x5dd0
	ctx.r[8].s64 = ctx.r[11].s64 + 24016;
	// 82612278: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8261227C: 388A331C  addi r4, r10, 0x331c
	ctx.r[4].s64 = ctx.r[10].s64 + 13084;
	// 82612280: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82612284: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612288: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261228C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82612290: 386A9D4C  addi r3, r10, -0x62b4
	ctx.r[3].s64 = ctx.r[10].s64 + -25268;
	// 82612294: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82612298: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261229C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826122A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826122A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826122A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826122AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826122B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826122B4: 4BE54B6D  bl 0x82466e20
	ctx.lr = 0x826122B8;
	sub_82466E20(ctx, base);
	// 826122B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826122BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826122C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826122C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826122C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826122C8 size=116
    let mut pc: u32 = 0x826122C8;
    'dispatch: loop {
        match pc {
            0x826122C8 => {
    //   block [0x826122C8..0x8261233C)
	// 826122C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826122CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826122D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826122D4: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 826122D8: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826122DC: 390A5E00  addi r8, r10, 0x5e00
	ctx.r[8].s64 = ctx.r[10].s64 + 24064;
	// 826122E0: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826122E4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 826122E8: 38AA998C  addi r5, r10, -0x6674
	ctx.r[5].s64 = ctx.r[10].s64 + -26228;
	// 826122EC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826122F0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826122F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826122F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826122FC: 388A3340  addi r4, r10, 0x3340
	ctx.r[4].s64 = ctx.r[10].s64 + 13120;
	// 82612300: 396BF144  addi r11, r11, -0xebc
	ctx.r[11].s64 = ctx.r[11].s64 + -3772;
	// 82612304: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612308: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261230C: 386A9D7C  addi r3, r10, -0x6284
	ctx.r[3].s64 = ctx.r[10].s64 + -25220;
	// 82612310: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82612314: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82612318: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8261231C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82612320: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82612324: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82612328: 4BE54AF9  bl 0x82466e20
	ctx.lr = 0x8261232C;
	sub_82466E20(ctx, base);
	// 8261232C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82612330: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82612334: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82612338: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82612340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82612340 size=112
    let mut pc: u32 = 0x82612340;
    'dispatch: loop {
        match pc {
            0x82612340 => {
    //   block [0x82612340..0x826123B0)
	// 82612340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82612344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82612348: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261234C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612350: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82612354: 38AA998C  addi r5, r10, -0x6674
	ctx.r[5].s64 = ctx.r[10].s64 + -26228;
	// 82612358: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261235C: 390B5EC0  addi r8, r11, 0x5ec0
	ctx.r[8].s64 = ctx.r[11].s64 + 24256;
	// 82612360: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82612364: 388A335C  addi r4, r10, 0x335c
	ctx.r[4].s64 = ctx.r[10].s64 + 13148;
	// 82612368: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261236C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612370: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82612374: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82612378: 386A9DAC  addi r3, r10, -0x6254
	ctx.r[3].s64 = ctx.r[10].s64 + -25172;
	// 8261237C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82612380: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82612384: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82612388: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261238C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82612390: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82612394: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82612398: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261239C: 4BE54A85  bl 0x82466e20
	ctx.lr = 0x826123A0;
	sub_82466E20(ctx, base);
	// 826123A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826123A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826123A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826123AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826123B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826123B0 size=108
    let mut pc: u32 = 0x826123B0;
    'dispatch: loop {
        match pc {
            0x826123B0 => {
    //   block [0x826123B0..0x8261241C)
	// 826123B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826123B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826123B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826123BC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826123C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826123C4: 38EB5ED8  addi r7, r11, 0x5ed8
	ctx.r[7].s64 = ctx.r[11].s64 + 24280;
	// 826123C8: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 826123CC: 388A3374  addi r4, r10, 0x3374
	ctx.r[4].s64 = ctx.r[10].s64 + 13172;
	// 826123D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826123D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826123D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826123DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826123E0: 386A9DDC  addi r3, r10, -0x6224
	ctx.r[3].s64 = ctx.r[10].s64 + -25124;
	// 826123E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826123E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826123EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826123F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826123F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826123F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826123FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82612400: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82612404: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82612408: 4BE54A19  bl 0x82466e20
	ctx.lr = 0x8261240C;
	sub_82466E20(ctx, base);
	// 8261240C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82612410: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82612414: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82612418: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82612420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82612420 size=116
    let mut pc: u32 = 0x82612420;
    'dispatch: loop {
        match pc {
            0x82612420 => {
    //   block [0x82612420..0x82612494)
	// 82612420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82612424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82612428: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261242C: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82612430: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82612434: 390A6010  addi r8, r10, 0x6010
	ctx.r[8].s64 = ctx.r[10].s64 + 24592;
	// 82612438: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261243C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82612440: 38AA998C  addi r5, r10, -0x6674
	ctx.r[5].s64 = ctx.r[10].s64 + -26228;
	// 82612444: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82612448: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8261244C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82612450: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82612454: 388A3394  addi r4, r10, 0x3394
	ctx.r[4].s64 = ctx.r[10].s64 + 13204;
	// 82612458: 396BF168  addi r11, r11, -0xe98
	ctx.r[11].s64 = ctx.r[11].s64 + -3736;
	// 8261245C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612460: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82612464: 386A9E0C  addi r3, r10, -0x61f4
	ctx.r[3].s64 = ctx.r[10].s64 + -25076;
	// 82612468: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8261246C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82612470: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82612474: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82612478: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261247C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82612480: 4BE549A1  bl 0x82466e20
	ctx.lr = 0x82612484;
	sub_82466E20(ctx, base);
	// 82612484: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82612488: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261248C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82612490: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82612498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82612498 size=112
    let mut pc: u32 = 0x82612498;
    'dispatch: loop {
        match pc {
            0x82612498 => {
    //   block [0x82612498..0x82612508)
	// 82612498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261249C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826124A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826124A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826124A8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826124AC: 38AA998C  addi r5, r10, -0x6674
	ctx.r[5].s64 = ctx.r[10].s64 + -26228;
	// 826124B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826124B4: 390B60B8  addi r8, r11, 0x60b8
	ctx.r[8].s64 = ctx.r[11].s64 + 24760;
	// 826124B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826124BC: 388A33B0  addi r4, r10, 0x33b0
	ctx.r[4].s64 = ctx.r[10].s64 + 13232;
	// 826124C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826124C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826124C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826124CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826124D0: 386A9E3C  addi r3, r10, -0x61c4
	ctx.r[3].s64 = ctx.r[10].s64 + -25028;
	// 826124D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826124D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826124DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826124E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826124E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826124E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826124EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826124F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826124F4: 4BE5492D  bl 0x82466e20
	ctx.lr = 0x826124F8;
	sub_82466E20(ctx, base);
	// 826124F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826124FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82612500: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82612504: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82612508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82612508 size=112
    let mut pc: u32 = 0x82612508;
    'dispatch: loop {
        match pc {
            0x82612508 => {
    //   block [0x82612508..0x82612578)
	// 82612508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261250C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82612510: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82612514: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612518: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8261251C: 38AA998C  addi r5, r10, -0x6674
	ctx.r[5].s64 = ctx.r[10].s64 + -26228;
	// 82612520: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82612524: 390B60D0  addi r8, r11, 0x60d0
	ctx.r[8].s64 = ctx.r[11].s64 + 24784;
	// 82612528: 3920000D  li r9, 0xd
	ctx.r[9].s64 = 13;
	// 8261252C: 388A33D4  addi r4, r10, 0x33d4
	ctx.r[4].s64 = ctx.r[10].s64 + 13268;
	// 82612530: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82612534: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612538: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261253C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82612540: 386A9E6C  addi r3, r10, -0x6194
	ctx.r[3].s64 = ctx.r[10].s64 + -24980;
	// 82612544: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82612548: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261254C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82612550: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82612554: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82612558: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261255C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82612560: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82612564: 4BE548BD  bl 0x82466e20
	ctx.lr = 0x82612568;
	sub_82466E20(ctx, base);
	// 82612568: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261256C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82612570: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82612574: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82612578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82612578 size=112
    let mut pc: u32 = 0x82612578;
    'dispatch: loop {
        match pc {
            0x82612578 => {
    //   block [0x82612578..0x826125E8)
	// 82612578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261257C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82612580: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82612584: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612588: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8261258C: 38AA998C  addi r5, r10, -0x6674
	ctx.r[5].s64 = ctx.r[10].s64 + -26228;
	// 82612590: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82612594: 390B6208  addi r8, r11, 0x6208
	ctx.r[8].s64 = ctx.r[11].s64 + 25096;
	// 82612598: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261259C: 388A33E8  addi r4, r10, 0x33e8
	ctx.r[4].s64 = ctx.r[10].s64 + 13288;
	// 826125A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826125A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826125A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826125AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826125B0: 386A9E9C  addi r3, r10, -0x6164
	ctx.r[3].s64 = ctx.r[10].s64 + -24932;
	// 826125B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826125B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826125BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826125C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826125C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826125C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826125CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826125D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826125D4: 4BE5484D  bl 0x82466e20
	ctx.lr = 0x826125D8;
	sub_82466E20(ctx, base);
	// 826125D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826125DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826125E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826125E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826125E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826125E8 size=116
    let mut pc: u32 = 0x826125E8;
    'dispatch: loop {
        match pc {
            0x826125E8 => {
    //   block [0x826125E8..0x8261265C)
	// 826125E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826125EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826125F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826125F4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826125F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826125FC: 390B6224  addi r8, r11, 0x6224
	ctx.r[8].s64 = ctx.r[11].s64 + 25124;
	// 82612600: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82612604: 392AF1A0  addi r9, r10, -0xe60
	ctx.r[9].s64 = ctx.r[10].s64 + -3680;
	// 82612608: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261260C: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82612610: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 82612614: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82612618: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261261C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82612620: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82612624: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82612628: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261262C: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82612630: 388A33FC  addi r4, r10, 0x33fc
	ctx.r[4].s64 = ctx.r[10].s64 + 13308;
	// 82612634: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82612638: 386B9ECC  addi r3, r11, -0x6134
	ctx.r[3].s64 = ctx.r[11].s64 + -24884;
	// 8261263C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82612640: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82612644: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82612648: 4BE547D9  bl 0x82466e20
	ctx.lr = 0x8261264C;
	sub_82466E20(ctx, base);
	// 8261264C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82612650: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82612654: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82612658: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82612660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82612660 size=100
    let mut pc: u32 = 0x82612660;
    'dispatch: loop {
        match pc {
            0x82612660 => {
    //   block [0x82612660..0x826126C4)
	// 82612660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82612664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82612668: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261266C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612670: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82612674: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 82612678: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261267C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82612680: 388A3404  addi r4, r10, 0x3404
	ctx.r[4].s64 = ctx.r[10].s64 + 13316;
	// 82612684: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612688: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261268C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82612690: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82612694: 386A9EFC  addi r3, r10, -0x6104
	ctx.r[3].s64 = ctx.r[10].s64 + -24836;
	// 82612698: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261269C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826126A0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826126A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826126A8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826126AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826126B0: 4BE54771  bl 0x82466e20
	ctx.lr = 0x826126B4;
	sub_82466E20(ctx, base);
	// 826126B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826126B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826126BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826126C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826126C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826126C8 size=112
    let mut pc: u32 = 0x826126C8;
    'dispatch: loop {
        match pc {
            0x826126C8 => {
    //   block [0x826126C8..0x82612738)
	// 826126C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826126CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826126D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826126D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826126D8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826126DC: 38AA9EFC  addi r5, r10, -0x6104
	ctx.r[5].s64 = ctx.r[10].s64 + -24836;
	// 826126E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826126E4: 390B6254  addi r8, r11, 0x6254
	ctx.r[8].s64 = ctx.r[11].s64 + 25172;
	// 826126E8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826126EC: 388A3414  addi r4, r10, 0x3414
	ctx.r[4].s64 = ctx.r[10].s64 + 13332;
	// 826126F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826126F4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826126F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826126FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82612700: 386A9F2C  addi r3, r10, -0x60d4
	ctx.r[3].s64 = ctx.r[10].s64 + -24788;
	// 82612704: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82612708: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261270C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82612710: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82612714: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82612718: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261271C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82612720: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82612724: 4BE546FD  bl 0x82466e20
	ctx.lr = 0x82612728;
	sub_82466E20(ctx, base);
	// 82612728: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261272C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82612730: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82612734: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82612738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82612738 size=112
    let mut pc: u32 = 0x82612738;
    'dispatch: loop {
        match pc {
            0x82612738 => {
    //   block [0x82612738..0x826127A8)
	// 82612738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261273C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82612740: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82612744: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612748: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8261274C: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 82612750: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82612754: 390B6270  addi r8, r11, 0x6270
	ctx.r[8].s64 = ctx.r[11].s64 + 25200;
	// 82612758: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8261275C: 388A3428  addi r4, r10, 0x3428
	ctx.r[4].s64 = ctx.r[10].s64 + 13352;
	// 82612760: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82612764: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612768: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261276C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82612770: 386A9F5C  addi r3, r10, -0x60a4
	ctx.r[3].s64 = ctx.r[10].s64 + -24740;
	// 82612774: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82612778: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261277C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82612780: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82612784: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82612788: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261278C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82612790: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82612794: 4BE5468D  bl 0x82466e20
	ctx.lr = 0x82612798;
	sub_82466E20(ctx, base);
	// 82612798: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261279C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826127A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826127A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826127A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826127A8 size=112
    let mut pc: u32 = 0x826127A8;
    'dispatch: loop {
        match pc {
            0x826127A8 => {
    //   block [0x826127A8..0x82612818)
	// 826127A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826127AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826127B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826127B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826127B8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826127BC: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 826127C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826127C4: 390B6318  addi r8, r11, 0x6318
	ctx.r[8].s64 = ctx.r[11].s64 + 25368;
	// 826127C8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826127CC: 388A3440  addi r4, r10, 0x3440
	ctx.r[4].s64 = ctx.r[10].s64 + 13376;
	// 826127D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826127D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826127D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826127DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826127E0: 386A9F8C  addi r3, r10, -0x6074
	ctx.r[3].s64 = ctx.r[10].s64 + -24692;
	// 826127E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826127E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826127EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826127F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826127F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826127F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826127FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82612800: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82612804: 4BE5461D  bl 0x82466e20
	ctx.lr = 0x82612808;
	sub_82466E20(ctx, base);
	// 82612808: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261280C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82612810: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82612814: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82612818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82612818 size=112
    let mut pc: u32 = 0x82612818;
    'dispatch: loop {
        match pc {
            0x82612818 => {
    //   block [0x82612818..0x82612888)
	// 82612818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261281C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82612820: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82612824: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612828: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8261282C: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 82612830: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82612834: 390B6360  addi r8, r11, 0x6360
	ctx.r[8].s64 = ctx.r[11].s64 + 25440;
	// 82612838: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8261283C: 388A3450  addi r4, r10, 0x3450
	ctx.r[4].s64 = ctx.r[10].s64 + 13392;
	// 82612840: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82612844: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612848: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261284C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82612850: 386A9FBC  addi r3, r10, -0x6044
	ctx.r[3].s64 = ctx.r[10].s64 + -24644;
	// 82612854: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82612858: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261285C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82612860: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82612864: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82612868: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261286C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82612870: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82612874: 4BE545AD  bl 0x82466e20
	ctx.lr = 0x82612878;
	sub_82466E20(ctx, base);
	// 82612878: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261287C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82612880: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82612884: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82612888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82612888 size=116
    let mut pc: u32 = 0x82612888;
    'dispatch: loop {
        match pc {
            0x82612888 => {
    //   block [0x82612888..0x826128FC)
	// 82612888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261288C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82612890: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82612894: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82612898: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 8261289C: 390A6390  addi r8, r10, 0x6390
	ctx.r[8].s64 = ctx.r[10].s64 + 25488;
	// 826128A0: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826128A4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 826128A8: 38AA998C  addi r5, r10, -0x6674
	ctx.r[5].s64 = ctx.r[10].s64 + -26228;
	// 826128AC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826128B0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826128B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826128B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826128BC: 388A3468  addi r4, r10, 0x3468
	ctx.r[4].s64 = ctx.r[10].s64 + 13416;
	// 826128C0: 396BF1B4  addi r11, r11, -0xe4c
	ctx.r[11].s64 = ctx.r[11].s64 + -3660;
	// 826128C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826128C8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826128CC: 386A9FEC  addi r3, r10, -0x6014
	ctx.r[3].s64 = ctx.r[10].s64 + -24596;
	// 826128D0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826128D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826128D8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826128DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826128E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826128E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826128E8: 4BE54539  bl 0x82466e20
	ctx.lr = 0x826128EC;
	sub_82466E20(ctx, base);
	// 826128EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826128F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826128F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826128F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82612900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82612900 size=100
    let mut pc: u32 = 0x82612900;
    'dispatch: loop {
        match pc {
            0x82612900 => {
    //   block [0x82612900..0x82612964)
	// 82612900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82612904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82612908: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261290C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612910: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82612914: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 82612918: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261291C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82612920: 388A3474  addi r4, r10, 0x3474
	ctx.r[4].s64 = ctx.r[10].s64 + 13428;
	// 82612924: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612928: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261292C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82612930: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82612934: 386AA01C  addi r3, r10, -0x5fe4
	ctx.r[3].s64 = ctx.r[10].s64 + -24548;
	// 82612938: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261293C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82612940: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82612944: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82612948: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261294C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82612950: 4BE544D1  bl 0x82466e20
	ctx.lr = 0x82612954;
	sub_82466E20(ctx, base);
	// 82612954: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82612958: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261295C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82612960: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82612968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82612968 size=108
    let mut pc: u32 = 0x82612968;
    'dispatch: loop {
        match pc {
            0x82612968 => {
    //   block [0x82612968..0x826129D4)
	// 82612968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261296C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82612970: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82612974: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82612978: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261297C: 38EB6450  addi r7, r11, 0x6450
	ctx.r[7].s64 = ctx.r[11].s64 + 25680;
	// 82612980: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82612984: 388A3488  addi r4, r10, 0x3488
	ctx.r[4].s64 = ctx.r[10].s64 + 13448;
	// 82612988: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261298C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612990: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82612994: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82612998: 386AA04C  addi r3, r10, -0x5fb4
	ctx.r[3].s64 = ctx.r[10].s64 + -24500;
	// 8261299C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826129A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826129A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826129A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826129AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826129B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826129B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826129B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826129BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826129C0: 4BE54461  bl 0x82466e20
	ctx.lr = 0x826129C4;
	sub_82466E20(ctx, base);
	// 826129C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826129C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826129CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826129D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826129D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826129D8 size=112
    let mut pc: u32 = 0x826129D8;
    'dispatch: loop {
        match pc {
            0x826129D8 => {
    //   block [0x826129D8..0x82612A48)
	// 826129D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826129DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826129E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826129E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826129E8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826129EC: 38AAA01C  addi r5, r10, -0x5fe4
	ctx.r[5].s64 = ctx.r[10].s64 + -24548;
	// 826129F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826129F4: 390B6480  addi r8, r11, 0x6480
	ctx.r[8].s64 = ctx.r[11].s64 + 25728;
	// 826129F8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826129FC: 388A34AC  addi r4, r10, 0x34ac
	ctx.r[4].s64 = ctx.r[10].s64 + 13484;
	// 82612A00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82612A04: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612A08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82612A0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82612A10: 386AA07C  addi r3, r10, -0x5f84
	ctx.r[3].s64 = ctx.r[10].s64 + -24452;
	// 82612A14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82612A18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82612A1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82612A20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82612A24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82612A28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82612A2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82612A30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82612A34: 4BE543ED  bl 0x82466e20
	ctx.lr = 0x82612A38;
	sub_82466E20(ctx, base);
	// 82612A38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82612A3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82612A40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82612A44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82612A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82612A48 size=108
    let mut pc: u32 = 0x82612A48;
    'dispatch: loop {
        match pc {
            0x82612A48 => {
    //   block [0x82612A48..0x82612AB4)
	// 82612A48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82612A4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82612A50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82612A54: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82612A58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82612A5C: 38EB64B0  addi r7, r11, 0x64b0
	ctx.r[7].s64 = ctx.r[11].s64 + 25776;
	// 82612A60: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82612A64: 388A34C4  addi r4, r10, 0x34c4
	ctx.r[4].s64 = ctx.r[10].s64 + 13508;
	// 82612A68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82612A6C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612A70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82612A74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82612A78: 386AA0AC  addi r3, r10, -0x5f54
	ctx.r[3].s64 = ctx.r[10].s64 + -24404;
	// 82612A7C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82612A80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82612A84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82612A88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82612A8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82612A90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82612A94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82612A98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82612A9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82612AA0: 4BE54381  bl 0x82466e20
	ctx.lr = 0x82612AA4;
	sub_82466E20(ctx, base);
	// 82612AA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82612AA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82612AAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82612AB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82612AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82612AB8 size=112
    let mut pc: u32 = 0x82612AB8;
    'dispatch: loop {
        match pc {
            0x82612AB8 => {
    //   block [0x82612AB8..0x82612B28)
	// 82612AB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82612ABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82612AC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82612AC4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612AC8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82612ACC: 38AAA01C  addi r5, r10, -0x5fe4
	ctx.r[5].s64 = ctx.r[10].s64 + -24548;
	// 82612AD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82612AD4: 390B64E0  addi r8, r11, 0x64e0
	ctx.r[8].s64 = ctx.r[11].s64 + 25824;
	// 82612AD8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82612ADC: 388A34F4  addi r4, r10, 0x34f4
	ctx.r[4].s64 = ctx.r[10].s64 + 13556;
	// 82612AE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82612AE4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612AE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82612AEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82612AF0: 386AA0DC  addi r3, r10, -0x5f24
	ctx.r[3].s64 = ctx.r[10].s64 + -24356;
	// 82612AF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82612AF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82612AFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82612B00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82612B04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82612B08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82612B0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82612B10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82612B14: 4BE5430D  bl 0x82466e20
	ctx.lr = 0x82612B18;
	sub_82466E20(ctx, base);
	// 82612B18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82612B1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82612B20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82612B24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82612B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82612B28 size=108
    let mut pc: u32 = 0x82612B28;
    'dispatch: loop {
        match pc {
            0x82612B28 => {
    //   block [0x82612B28..0x82612B94)
	// 82612B28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82612B2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82612B30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82612B34: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82612B38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82612B3C: 38EB6528  addi r7, r11, 0x6528
	ctx.r[7].s64 = ctx.r[11].s64 + 25896;
	// 82612B40: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82612B44: 388A3514  addi r4, r10, 0x3514
	ctx.r[4].s64 = ctx.r[10].s64 + 13588;
	// 82612B48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82612B4C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612B50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82612B54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82612B58: 386AA10C  addi r3, r10, -0x5ef4
	ctx.r[3].s64 = ctx.r[10].s64 + -24308;
	// 82612B5C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82612B60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82612B64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82612B68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82612B6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82612B70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82612B74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82612B78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82612B7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82612B80: 4BE542A1  bl 0x82466e20
	ctx.lr = 0x82612B84;
	sub_82466E20(ctx, base);
	// 82612B84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82612B88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82612B8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82612B90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82612B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82612B98 size=112
    let mut pc: u32 = 0x82612B98;
    'dispatch: loop {
        match pc {
            0x82612B98 => {
    //   block [0x82612B98..0x82612C08)
	// 82612B98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82612B9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82612BA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82612BA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612BA8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82612BAC: 38AAA01C  addi r5, r10, -0x5fe4
	ctx.r[5].s64 = ctx.r[10].s64 + -24548;
	// 82612BB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82612BB4: 390B6558  addi r8, r11, 0x6558
	ctx.r[8].s64 = ctx.r[11].s64 + 25944;
	// 82612BB8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82612BBC: 388A3544  addi r4, r10, 0x3544
	ctx.r[4].s64 = ctx.r[10].s64 + 13636;
	// 82612BC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82612BC4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612BC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82612BCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82612BD0: 386AA13C  addi r3, r10, -0x5ec4
	ctx.r[3].s64 = ctx.r[10].s64 + -24260;
	// 82612BD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82612BD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82612BDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82612BE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82612BE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82612BE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82612BEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82612BF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82612BF4: 4BE5422D  bl 0x82466e20
	ctx.lr = 0x82612BF8;
	sub_82466E20(ctx, base);
	// 82612BF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82612BFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82612C00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82612C04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82612C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82612C08 size=108
    let mut pc: u32 = 0x82612C08;
    'dispatch: loop {
        match pc {
            0x82612C08 => {
    //   block [0x82612C08..0x82612C74)
	// 82612C08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82612C0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82612C10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82612C14: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82612C18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82612C1C: 38EB65A0  addi r7, r11, 0x65a0
	ctx.r[7].s64 = ctx.r[11].s64 + 26016;
	// 82612C20: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82612C24: 388A3564  addi r4, r10, 0x3564
	ctx.r[4].s64 = ctx.r[10].s64 + 13668;
	// 82612C28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82612C2C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612C30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82612C34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82612C38: 386AA16C  addi r3, r10, -0x5e94
	ctx.r[3].s64 = ctx.r[10].s64 + -24212;
	// 82612C3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82612C40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82612C44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82612C48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82612C4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82612C50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82612C54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82612C58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82612C5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82612C60: 4BE541C1  bl 0x82466e20
	ctx.lr = 0x82612C64;
	sub_82466E20(ctx, base);
	// 82612C64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82612C68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82612C6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82612C70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82612C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82612C78 size=112
    let mut pc: u32 = 0x82612C78;
    'dispatch: loop {
        match pc {
            0x82612C78 => {
    //   block [0x82612C78..0x82612CE8)
	// 82612C78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82612C7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82612C80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82612C84: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612C88: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82612C8C: 38AAA01C  addi r5, r10, -0x5fe4
	ctx.r[5].s64 = ctx.r[10].s64 + -24548;
	// 82612C90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82612C94: 390B65D0  addi r8, r11, 0x65d0
	ctx.r[8].s64 = ctx.r[11].s64 + 26064;
	// 82612C98: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82612C9C: 388A3594  addi r4, r10, 0x3594
	ctx.r[4].s64 = ctx.r[10].s64 + 13716;
	// 82612CA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82612CA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612CA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82612CAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82612CB0: 386AA19C  addi r3, r10, -0x5e64
	ctx.r[3].s64 = ctx.r[10].s64 + -24164;
	// 82612CB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82612CB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82612CBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82612CC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82612CC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82612CC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82612CCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82612CD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82612CD4: 4BE5414D  bl 0x82466e20
	ctx.lr = 0x82612CD8;
	sub_82466E20(ctx, base);
	// 82612CD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82612CDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82612CE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82612CE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82612CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82612CE8 size=108
    let mut pc: u32 = 0x82612CE8;
    'dispatch: loop {
        match pc {
            0x82612CE8 => {
    //   block [0x82612CE8..0x82612D54)
	// 82612CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82612CEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82612CF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82612CF4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82612CF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82612CFC: 38EB6618  addi r7, r11, 0x6618
	ctx.r[7].s64 = ctx.r[11].s64 + 26136;
	// 82612D00: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82612D04: 388A35B0  addi r4, r10, 0x35b0
	ctx.r[4].s64 = ctx.r[10].s64 + 13744;
	// 82612D08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82612D0C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612D10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82612D14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82612D18: 386AA1CC  addi r3, r10, -0x5e34
	ctx.r[3].s64 = ctx.r[10].s64 + -24116;
	// 82612D1C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82612D20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82612D24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82612D28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82612D2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82612D30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82612D34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82612D38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82612D3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82612D40: 4BE540E1  bl 0x82466e20
	ctx.lr = 0x82612D44;
	sub_82466E20(ctx, base);
	// 82612D44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82612D48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82612D4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82612D50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82612D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82612D58 size=112
    let mut pc: u32 = 0x82612D58;
    'dispatch: loop {
        match pc {
            0x82612D58 => {
    //   block [0x82612D58..0x82612DC8)
	// 82612D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82612D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82612D60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82612D64: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82612D68: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82612D6C: 392AF290  addi r9, r10, -0xd70
	ctx.r[9].s64 = ctx.r[10].s64 + -3440;
	// 82612D70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82612D74: 390B6680  addi r8, r11, 0x6680
	ctx.r[8].s64 = ctx.r[11].s64 + 26240;
	// 82612D78: 38E0000C  li r7, 0xc
	ctx.r[7].s64 = 12;
	// 82612D7C: 388A35CC  addi r4, r10, 0x35cc
	ctx.r[4].s64 = ctx.r[10].s64 + 13772;
	// 82612D80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82612D84: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612D88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82612D8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82612D90: 386AA1FC  addi r3, r10, -0x5e04
	ctx.r[3].s64 = ctx.r[10].s64 + -24068;
	// 82612D94: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82612D98: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82612D9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82612DA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82612DA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82612DA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82612DAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82612DB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82612DB4: 4BE5406D  bl 0x82466e20
	ctx.lr = 0x82612DB8;
	sub_82466E20(ctx, base);
	// 82612DB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82612DBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82612DC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82612DC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82612DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82612DC8 size=108
    let mut pc: u32 = 0x82612DC8;
    'dispatch: loop {
        match pc {
            0x82612DC8 => {
    //   block [0x82612DC8..0x82612E34)
	// 82612DC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82612DCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82612DD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82612DD4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82612DD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82612DDC: 38EB67A0  addi r7, r11, 0x67a0
	ctx.r[7].s64 = ctx.r[11].s64 + 26528;
	// 82612DE0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82612DE4: 388A35EC  addi r4, r10, 0x35ec
	ctx.r[4].s64 = ctx.r[10].s64 + 13804;
	// 82612DE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82612DEC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612DF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82612DF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82612DF8: 386AA22C  addi r3, r10, -0x5dd4
	ctx.r[3].s64 = ctx.r[10].s64 + -24020;
	// 82612DFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82612E00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82612E04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82612E08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82612E0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82612E10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82612E14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82612E18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82612E1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82612E20: 4BE54001  bl 0x82466e20
	ctx.lr = 0x82612E24;
	sub_82466E20(ctx, base);
	// 82612E24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82612E28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82612E2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82612E30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82612E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82612E38 size=108
    let mut pc: u32 = 0x82612E38;
    'dispatch: loop {
        match pc {
            0x82612E38 => {
    //   block [0x82612E38..0x82612EA4)
	// 82612E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82612E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82612E40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82612E44: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82612E48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82612E4C: 38EB6800  addi r7, r11, 0x6800
	ctx.r[7].s64 = ctx.r[11].s64 + 26624;
	// 82612E50: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 82612E54: 388A3610  addi r4, r10, 0x3610
	ctx.r[4].s64 = ctx.r[10].s64 + 13840;
	// 82612E58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82612E5C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612E60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82612E64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82612E68: 386AA25C  addi r3, r10, -0x5da4
	ctx.r[3].s64 = ctx.r[10].s64 + -23972;
	// 82612E6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82612E70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82612E74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82612E78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82612E7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82612E80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82612E84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82612E88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82612E8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82612E90: 4BE53F91  bl 0x82466e20
	ctx.lr = 0x82612E94;
	sub_82466E20(ctx, base);
	// 82612E94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82612E98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82612E9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82612EA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82612EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82612EA8 size=24
    let mut pc: u32 = 0x82612EA8;
    'dispatch: loop {
        match pc {
            0x82612EA8 => {
    //   block [0x82612EA8..0x82612EC0)
	// 82612EA8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82612EAC: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82612EB0: 394AD2B8  addi r10, r10, -0x2d48
	ctx.r[10].s64 = ctx.r[10].s64 + -11592;
	// 82612EB4: 816B626C  lwz r11, 0x626c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(25196 as u32) ) } as u64;
	// 82612EB8: 916A00E0  stw r11, 0xe0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(224 as u32), ctx.r[11].u32 ) };
	// 82612EBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82612EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82612EC0 size=116
    let mut pc: u32 = 0x82612EC0;
    'dispatch: loop {
        match pc {
            0x82612EC0 => {
    //   block [0x82612EC0..0x82612F34)
	// 82612EC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82612EC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82612EC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82612ECC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82612ED0: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612ED4: 392BF1FC  addi r9, r11, -0xe04
	ctx.r[9].s64 = ctx.r[11].s64 + -3588;
	// 82612ED8: 38AA971C  addi r5, r10, -0x68e4
	ctx.r[5].s64 = ctx.r[10].s64 + -26852;
	// 82612EDC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82612EE0: 38E900AC  addi r7, r9, 0xac
	ctx.r[7].s64 = ctx.r[9].s64 + 172;
	// 82612EE4: 38C0001A  li r6, 0x1a
	ctx.r[6].s64 = 26;
	// 82612EE8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82612EEC: 388A362C  addi r4, r10, 0x362c
	ctx.r[4].s64 = ctx.r[10].s64 + 13868;
	// 82612EF0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82612EF4: 396BD2B8  addi r11, r11, -0x2d48
	ctx.r[11].s64 = ctx.r[11].s64 + -11592;
	// 82612EF8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82612EFC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612F00: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82612F04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82612F08: 386AA28C  addi r3, r10, -0x5d74
	ctx.r[3].s64 = ctx.r[10].s64 + -23924;
	// 82612F0C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82612F10: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82612F14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82612F18: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82612F1C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82612F20: 4BE53F01  bl 0x82466e20
	ctx.lr = 0x82612F24;
	sub_82466E20(ctx, base);
	// 82612F24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82612F28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82612F2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82612F30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82612F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82612F38 size=100
    let mut pc: u32 = 0x82612F38;
    'dispatch: loop {
        match pc {
            0x82612F38 => {
    //   block [0x82612F38..0x82612F9C)
	// 82612F38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82612F3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82612F40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82612F44: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612F48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82612F4C: 38AA971C  addi r5, r10, -0x68e4
	ctx.r[5].s64 = ctx.r[10].s64 + -26852;
	// 82612F50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82612F54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82612F58: 388A363C  addi r4, r10, 0x363c
	ctx.r[4].s64 = ctx.r[10].s64 + 13884;
	// 82612F5C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612F60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82612F64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82612F68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82612F6C: 386AA2BC  addi r3, r10, -0x5d44
	ctx.r[3].s64 = ctx.r[10].s64 + -23876;
	// 82612F70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82612F74: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82612F78: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82612F7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82612F80: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82612F84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82612F88: 4BE53E99  bl 0x82466e20
	ctx.lr = 0x82612F8C;
	sub_82466E20(ctx, base);
	// 82612F8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82612F90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82612F94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82612F98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82612FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82612FA0 size=24
    let mut pc: u32 = 0x82612FA0;
    'dispatch: loop {
        match pc {
            0x82612FA0 => {
    //   block [0x82612FA0..0x82612FB8)
	// 82612FA0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82612FA4: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82612FA8: 394AD528  addi r10, r10, -0x2ad8
	ctx.r[10].s64 = ctx.r[10].s64 + -10968;
	// 82612FAC: 816B68A8  lwz r11, 0x68a8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26792 as u32) ) } as u64;
	// 82612FB0: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82612FB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82612FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82612FB8 size=116
    let mut pc: u32 = 0x82612FB8;
    'dispatch: loop {
        match pc {
            0x82612FB8 => {
    //   block [0x82612FB8..0x8261302C)
	// 82612FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82612FBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82612FC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82612FC4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82612FC8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82612FCC: 390BD528  addi r8, r11, -0x2ad8
	ctx.r[8].s64 = ctx.r[11].s64 + -10968;
	// 82612FD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82612FD4: 392AF364  addi r9, r10, -0xc9c
	ctx.r[9].s64 = ctx.r[10].s64 + -3228;
	// 82612FD8: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82612FDC: 38E0000F  li r7, 0xf
	ctx.r[7].s64 = 15;
	// 82612FE0: 38AAA2BC  addi r5, r10, -0x5d44
	ctx.r[5].s64 = ctx.r[10].s64 + -23876;
	// 82612FE4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82612FE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82612FEC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82612FF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82612FF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82612FF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82612FFC: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82613000: 388A3690  addi r4, r10, 0x3690
	ctx.r[4].s64 = ctx.r[10].s64 + 13968;
	// 82613004: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82613008: 386BA2EC  addi r3, r11, -0x5d14
	ctx.r[3].s64 = ctx.r[11].s64 + -23828;
	// 8261300C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82613010: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82613014: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82613018: 4BE53E09  bl 0x82466e20
	ctx.lr = 0x8261301C;
	sub_82466E20(ctx, base);
	// 8261301C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82613020: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82613024: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82613028: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82613030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82613030 size=112
    let mut pc: u32 = 0x82613030;
    'dispatch: loop {
        match pc {
            0x82613030 => {
    //   block [0x82613030..0x826130A0)
	// 82613030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82613034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82613038: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261303C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82613040: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82613044: 38AAA2BC  addi r5, r10, -0x5d44
	ctx.r[5].s64 = ctx.r[10].s64 + -23876;
	// 82613048: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8261304C: 390B68B0  addi r8, r11, 0x68b0
	ctx.r[8].s64 = ctx.r[11].s64 + 26800;
	// 82613050: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 82613054: 388AAAE8  addi r4, r10, -0x5518
	ctx.r[4].s64 = ctx.r[10].s64 + -21784;
	// 82613058: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261305C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82613060: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82613064: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82613068: 386AA31C  addi r3, r10, -0x5ce4
	ctx.r[3].s64 = ctx.r[10].s64 + -23780;
	// 8261306C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82613070: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82613074: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82613078: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261307C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82613080: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82613084: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82613088: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261308C: 4BE53D95  bl 0x82466e20
	ctx.lr = 0x82613090;
	sub_82466E20(ctx, base);
	// 82613090: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82613094: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82613098: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261309C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826130A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826130A0 size=112
    let mut pc: u32 = 0x826130A0;
    'dispatch: loop {
        match pc {
            0x826130A0 => {
    //   block [0x826130A0..0x82613110)
	// 826130A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826130A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826130A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826130AC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826130B0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826130B4: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 826130B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826130BC: 390B6988  addi r8, r11, 0x6988
	ctx.r[8].s64 = ctx.r[11].s64 + 27016;
	// 826130C0: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 826130C4: 388A36AC  addi r4, r10, 0x36ac
	ctx.r[4].s64 = ctx.r[10].s64 + 13996;
	// 826130C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826130CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826130D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826130D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826130D8: 386AA34C  addi r3, r10, -0x5cb4
	ctx.r[3].s64 = ctx.r[10].s64 + -23732;
	// 826130DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826130E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826130E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826130E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826130EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826130F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826130F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826130F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826130FC: 4BE53D25  bl 0x82466e20
	ctx.lr = 0x82613100;
	sub_82466E20(ctx, base);
	// 82613100: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82613104: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82613108: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261310C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82613110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82613110 size=108
    let mut pc: u32 = 0x82613110;
    'dispatch: loop {
        match pc {
            0x82613110 => {
    //   block [0x82613110..0x8261317C)
	// 82613110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82613114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82613118: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261311C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82613120: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82613124: 38EB6A60  addi r7, r11, 0x6a60
	ctx.r[7].s64 = ctx.r[11].s64 + 27232;
	// 82613128: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8261312C: 388A36C8  addi r4, r10, 0x36c8
	ctx.r[4].s64 = ctx.r[10].s64 + 14024;
	// 82613130: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82613134: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82613138: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261313C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82613140: 386AA37C  addi r3, r10, -0x5c84
	ctx.r[3].s64 = ctx.r[10].s64 + -23684;
	// 82613144: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82613148: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261314C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82613150: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82613154: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82613158: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261315C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82613160: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82613164: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82613168: 4BE53CB9  bl 0x82466e20
	ctx.lr = 0x8261316C;
	sub_82466E20(ctx, base);
	// 8261316C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82613170: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82613174: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82613178: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82613180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82613180 size=108
    let mut pc: u32 = 0x82613180;
    'dispatch: loop {
        match pc {
            0x82613180 => {
    //   block [0x82613180..0x826131EC)
	// 82613180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82613184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82613188: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261318C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82613190: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82613194: 38EB6AD8  addi r7, r11, 0x6ad8
	ctx.r[7].s64 = ctx.r[11].s64 + 27352;
	// 82613198: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8261319C: 388A36E4  addi r4, r10, 0x36e4
	ctx.r[4].s64 = ctx.r[10].s64 + 14052;
	// 826131A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826131A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826131A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826131AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826131B0: 386AA3AC  addi r3, r10, -0x5c54
	ctx.r[3].s64 = ctx.r[10].s64 + -23636;
	// 826131B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826131B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826131BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826131C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826131C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826131C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826131CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826131D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826131D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826131D8: 4BE53C49  bl 0x82466e20
	ctx.lr = 0x826131DC;
	sub_82466E20(ctx, base);
	// 826131DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826131E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826131E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826131E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826131F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826131F0 size=112
    let mut pc: u32 = 0x826131F0;
    'dispatch: loop {
        match pc {
            0x826131F0 => {
    //   block [0x826131F0..0x82613260)
	// 826131F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826131F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826131F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826131FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82613200: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82613204: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 82613208: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261320C: 390B6B20  addi r8, r11, 0x6b20
	ctx.r[8].s64 = ctx.r[11].s64 + 27424;
	// 82613210: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82613214: 388A3704  addi r4, r10, 0x3704
	ctx.r[4].s64 = ctx.r[10].s64 + 14084;
	// 82613218: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261321C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82613220: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82613224: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82613228: 386AA3DC  addi r3, r10, -0x5c24
	ctx.r[3].s64 = ctx.r[10].s64 + -23588;
	// 8261322C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82613230: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82613234: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82613238: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261323C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82613240: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82613244: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82613248: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261324C: 4BE53BD5  bl 0x82466e20
	ctx.lr = 0x82613250;
	sub_82466E20(ctx, base);
	// 82613250: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82613254: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82613258: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261325C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82613260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82613260 size=108
    let mut pc: u32 = 0x82613260;
    'dispatch: loop {
        match pc {
            0x82613260 => {
    //   block [0x82613260..0x826132CC)
	// 82613260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82613264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82613268: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261326C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82613270: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82613274: 38EB6D00  addi r7, r11, 0x6d00
	ctx.r[7].s64 = ctx.r[11].s64 + 27904;
	// 82613278: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8261327C: 388A3714  addi r4, r10, 0x3714
	ctx.r[4].s64 = ctx.r[10].s64 + 14100;
	// 82613280: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82613284: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82613288: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261328C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82613290: 386AA40C  addi r3, r10, -0x5bf4
	ctx.r[3].s64 = ctx.r[10].s64 + -23540;
	// 82613294: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82613298: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261329C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826132A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826132A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826132A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826132AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826132B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826132B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826132B8: 4BE53B69  bl 0x82466e20
	ctx.lr = 0x826132BC;
	sub_82466E20(ctx, base);
	// 826132BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826132C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826132C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826132C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826132D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826132D0 size=24
    let mut pc: u32 = 0x826132D0;
    'dispatch: loop {
        match pc {
            0x826132D0 => {
    //   block [0x826132D0..0x826132E8)
	// 826132D0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826132D4: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 826132D8: 394AD690  addi r10, r10, -0x2970
	ctx.r[10].s64 = ctx.r[10].s64 + -10608;
	// 826132DC: 816B68AC  lwz r11, 0x68ac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26796 as u32) ) } as u64;
	// 826132E0: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826132E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826132E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826132E8 size=112
    let mut pc: u32 = 0x826132E8;
    'dispatch: loop {
        match pc {
            0x826132E8 => {
    //   block [0x826132E8..0x82613358)
	// 826132E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826132EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826132F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826132F4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826132F8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826132FC: 392AF3BC  addi r9, r10, -0xc44
	ctx.r[9].s64 = ctx.r[10].s64 + -3140;
	// 82613300: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82613304: 390BD690  addi r8, r11, -0x2970
	ctx.r[8].s64 = ctx.r[11].s64 + -10608;
	// 82613308: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 8261330C: 388A3728  addi r4, r10, 0x3728
	ctx.r[4].s64 = ctx.r[10].s64 + 14120;
	// 82613310: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82613314: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82613318: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261331C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82613320: 386AA43C  addi r3, r10, -0x5bc4
	ctx.r[3].s64 = ctx.r[10].s64 + -23492;
	// 82613324: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82613328: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8261332C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82613330: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82613334: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82613338: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261333C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82613340: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82613344: 4BE53ADD  bl 0x82466e20
	ctx.lr = 0x82613348;
	sub_82466E20(ctx, base);
	// 82613348: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261334C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82613350: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82613354: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82613358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82613358 size=112
    let mut pc: u32 = 0x82613358;
    'dispatch: loop {
        match pc {
            0x82613358 => {
    //   block [0x82613358..0x826133C8)
	// 82613358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261335C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82613360: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82613364: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82613368: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8261336C: 38EA6D18  addi r7, r10, 0x6d18
	ctx.r[7].s64 = ctx.r[10].s64 + 27928;
	// 82613370: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82613374: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82613378: 388A3738  addi r4, r10, 0x3738
	ctx.r[4].s64 = ctx.r[10].s64 + 14136;
	// 8261337C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82613380: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82613384: 396BF3D0  addi r11, r11, -0xc30
	ctx.r[11].s64 = ctx.r[11].s64 + -3120;
	// 82613388: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261338C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82613390: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82613394: 386AA46C  addi r3, r10, -0x5b94
	ctx.r[3].s64 = ctx.r[10].s64 + -23444;
	// 82613398: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261339C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826133A0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826133A4: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826133A8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826133AC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826133B0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826133B4: 4BE53A6D  bl 0x82466e20
	ctx.lr = 0x826133B8;
	sub_82466E20(ctx, base);
	// 826133B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826133BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826133C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826133C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826133C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826133C8 size=112
    let mut pc: u32 = 0x826133C8;
    'dispatch: loop {
        match pc {
            0x826133C8 => {
    //   block [0x826133C8..0x82613438)
	// 826133C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826133CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826133D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826133D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826133D8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826133DC: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 826133E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826133E4: 390B6DA8  addi r8, r11, 0x6da8
	ctx.r[8].s64 = ctx.r[11].s64 + 28072;
	// 826133E8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826133EC: 388A3758  addi r4, r10, 0x3758
	ctx.r[4].s64 = ctx.r[10].s64 + 14168;
	// 826133F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826133F4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826133F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826133FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82613400: 386AA49C  addi r3, r10, -0x5b64
	ctx.r[3].s64 = ctx.r[10].s64 + -23396;
	// 82613404: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82613408: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261340C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82613410: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82613414: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82613418: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261341C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82613420: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82613424: 4BE539FD  bl 0x82466e20
	ctx.lr = 0x82613428;
	sub_82466E20(ctx, base);
	// 82613428: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261342C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82613430: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82613434: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82613438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82613438 size=108
    let mut pc: u32 = 0x82613438;
    'dispatch: loop {
        match pc {
            0x82613438 => {
    //   block [0x82613438..0x826134A4)
	// 82613438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261343C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82613440: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82613444: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82613448: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261344C: 38EB6DC8  addi r7, r11, 0x6dc8
	ctx.r[7].s64 = ctx.r[11].s64 + 28104;
	// 82613450: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82613454: 388A3770  addi r4, r10, 0x3770
	ctx.r[4].s64 = ctx.r[10].s64 + 14192;
	// 82613458: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261345C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82613460: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82613464: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82613468: 386AA4CC  addi r3, r10, -0x5b34
	ctx.r[3].s64 = ctx.r[10].s64 + -23348;
	// 8261346C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82613470: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82613474: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82613478: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261347C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82613480: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82613484: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82613488: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261348C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82613490: 4BE53991  bl 0x82466e20
	ctx.lr = 0x82613494;
	sub_82466E20(ctx, base);
	// 82613494: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82613498: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261349C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826134A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826134A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826134A8 size=108
    let mut pc: u32 = 0x826134A8;
    'dispatch: loop {
        match pc {
            0x826134A8 => {
    //   block [0x826134A8..0x82613514)
	// 826134A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826134AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826134B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826134B4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826134B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826134BC: 38EB6E28  addi r7, r11, 0x6e28
	ctx.r[7].s64 = ctx.r[11].s64 + 28200;
	// 826134C0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826134C4: 388A3788  addi r4, r10, 0x3788
	ctx.r[4].s64 = ctx.r[10].s64 + 14216;
	// 826134C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826134CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826134D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826134D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826134D8: 386AA4FC  addi r3, r10, -0x5b04
	ctx.r[3].s64 = ctx.r[10].s64 + -23300;
	// 826134DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826134E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826134E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826134E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826134EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826134F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826134F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826134F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826134FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82613500: 4BE53921  bl 0x82466e20
	ctx.lr = 0x82613504;
	sub_82466E20(ctx, base);
	// 82613504: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82613508: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261350C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82613510: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82613518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82613518 size=116
    let mut pc: u32 = 0x82613518;
    'dispatch: loop {
        match pc {
            0x82613518 => {
    //   block [0x82613518..0x8261358C)
	// 82613518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261351C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82613520: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82613524: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82613528: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8261352C: 390B6E58  addi r8, r11, 0x6e58
	ctx.r[8].s64 = ctx.r[11].s64 + 28248;
	// 82613530: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82613534: 392AF404  addi r9, r10, -0xbfc
	ctx.r[9].s64 = ctx.r[10].s64 + -3068;
	// 82613538: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261353C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82613540: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 82613544: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82613548: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261354C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82613550: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82613554: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82613558: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261355C: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82613560: 388A37A0  addi r4, r10, 0x37a0
	ctx.r[4].s64 = ctx.r[10].s64 + 14240;
	// 82613564: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82613568: 386BA52C  addi r3, r11, -0x5ad4
	ctx.r[3].s64 = ctx.r[11].s64 + -23252;
	// 8261356C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82613570: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82613574: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82613578: 4BE538A9  bl 0x82466e20
	ctx.lr = 0x8261357C;
	sub_82466E20(ctx, base);
	// 8261357C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82613580: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82613584: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82613588: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82613590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82613590 size=96
    let mut pc: u32 = 0x82613590;
    'dispatch: loop {
        match pc {
            0x82613590 => {
    //   block [0x82613590..0x826135F0)
	// 82613590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82613594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82613598: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261359C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826135A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826135A4: 388A6DD4  addi r4, r10, 0x6dd4
	ctx.r[4].s64 = ctx.r[10].s64 + 28116;
	// 826135A8: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826135AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826135B0: 386AA55C  addi r3, r10, -0x5aa4
	ctx.r[3].s64 = ctx.r[10].s64 + -23204;
	// 826135B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826135B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826135BC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826135C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826135C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826135C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826135CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826135D0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826135D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826135D8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826135DC: 4BE53845  bl 0x82466e20
	ctx.lr = 0x826135E0;
	sub_82466E20(ctx, base);
	// 826135E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826135E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826135E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826135EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826135F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826135F0 size=112
    let mut pc: u32 = 0x826135F0;
    'dispatch: loop {
        match pc {
            0x826135F0 => {
    //   block [0x826135F0..0x82613660)
	// 826135F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826135F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826135F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826135FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82613600: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82613604: 38AAA55C  addi r5, r10, -0x5aa4
	ctx.r[5].s64 = ctx.r[10].s64 + -23204;
	// 82613608: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8261360C: 390B6E70  addi r8, r11, 0x6e70
	ctx.r[8].s64 = ctx.r[11].s64 + 28272;
	// 82613610: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82613614: 388A7380  addi r4, r10, 0x7380
	ctx.r[4].s64 = ctx.r[10].s64 + 29568;
	// 82613618: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261361C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82613620: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82613624: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82613628: 386AA58C  addi r3, r10, -0x5a74
	ctx.r[3].s64 = ctx.r[10].s64 + -23156;
	// 8261362C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82613630: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82613634: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82613638: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261363C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82613640: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82613644: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82613648: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261364C: 4BE537D5  bl 0x82466e20
	ctx.lr = 0x82613650;
	sub_82466E20(ctx, base);
	// 82613650: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82613654: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82613658: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261365C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82613660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82613660 size=112
    let mut pc: u32 = 0x82613660;
    'dispatch: loop {
        match pc {
            0x82613660 => {
    //   block [0x82613660..0x826136D0)
	// 82613660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82613664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82613668: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261366C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82613670: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82613674: 392AF420  addi r9, r10, -0xbe0
	ctx.r[9].s64 = ctx.r[10].s64 + -3040;
	// 82613678: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8261367C: 390B6EA0  addi r8, r11, 0x6ea0
	ctx.r[8].s64 = ctx.r[11].s64 + 28320;
	// 82613680: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82613684: 388A7468  addi r4, r10, 0x7468
	ctx.r[4].s64 = ctx.r[10].s64 + 29800;
	// 82613688: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261368C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82613690: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82613694: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82613698: 386AA5BC  addi r3, r10, -0x5a44
	ctx.r[3].s64 = ctx.r[10].s64 + -23108;
	// 8261369C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826136A0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826136A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826136A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826136AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826136B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826136B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826136B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826136BC: 4BE53765  bl 0x82466e20
	ctx.lr = 0x826136C0;
	sub_82466E20(ctx, base);
	// 826136C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826136C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826136C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826136CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826136D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826136D0 size=108
    let mut pc: u32 = 0x826136D0;
    'dispatch: loop {
        match pc {
            0x826136D0 => {
    //   block [0x826136D0..0x8261373C)
	// 826136D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826136D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826136D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826136DC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826136E0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826136E4: 38EB6F48  addi r7, r11, 0x6f48
	ctx.r[7].s64 = ctx.r[11].s64 + 28488;
	// 826136E8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826136EC: 388A7888  addi r4, r10, 0x7888
	ctx.r[4].s64 = ctx.r[10].s64 + 30856;
	// 826136F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826136F4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826136F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826136FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82613700: 386AA5EC  addi r3, r10, -0x5a14
	ctx.r[3].s64 = ctx.r[10].s64 + -23060;
	// 82613704: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82613708: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261370C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82613710: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82613714: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82613718: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261371C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82613720: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82613724: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82613728: 4BE536F9  bl 0x82466e20
	ctx.lr = 0x8261372C;
	sub_82466E20(ctx, base);
	// 8261372C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82613730: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82613734: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82613738: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82613740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82613740 size=108
    let mut pc: u32 = 0x82613740;
    'dispatch: loop {
        match pc {
            0x82613740 => {
    //   block [0x82613740..0x826137AC)
	// 82613740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82613744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82613748: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261374C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82613750: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82613754: 38EB6F78  addi r7, r11, 0x6f78
	ctx.r[7].s64 = ctx.r[11].s64 + 28536;
	// 82613758: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8261375C: 388A7898  addi r4, r10, 0x7898
	ctx.r[4].s64 = ctx.r[10].s64 + 30872;
	// 82613760: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82613764: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82613768: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261376C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82613770: 386AA61C  addi r3, r10, -0x59e4
	ctx.r[3].s64 = ctx.r[10].s64 + -23012;
	// 82613774: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82613778: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261377C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82613780: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82613784: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82613788: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261378C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82613790: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82613794: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82613798: 4BE53689  bl 0x82466e20
	ctx.lr = 0x8261379C;
	sub_82466E20(ctx, base);
	// 8261379C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826137A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826137A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826137A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826137B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826137B0 size=28
    let mut pc: u32 = 0x826137B0;
    'dispatch: loop {
        match pc {
            0x826137B0 => {
    //   block [0x826137B0..0x826137CC)
	// 826137B0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826137B4: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 826137B8: 394AD6C0  addi r10, r10, -0x2940
	ctx.r[10].s64 = ctx.r[10].s64 + -10560;
	// 826137BC: 816B6FA8  lwz r11, 0x6fa8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28584 as u32) ) } as u64;
	// 826137C0: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826137C4: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826137C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826137D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826137D0 size=112
    let mut pc: u32 = 0x826137D0;
    'dispatch: loop {
        match pc {
            0x826137D0 => {
    //   block [0x826137D0..0x82613840)
	// 826137D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826137D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826137D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826137DC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826137E0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826137E4: 392AF5D0  addi r9, r10, -0xa30
	ctx.r[9].s64 = ctx.r[10].s64 + -2608;
	// 826137E8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826137EC: 390BD6C0  addi r8, r11, -0x2940
	ctx.r[8].s64 = ctx.r[11].s64 + -10560;
	// 826137F0: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826137F4: 388A72F4  addi r4, r10, 0x72f4
	ctx.r[4].s64 = ctx.r[10].s64 + 29428;
	// 826137F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826137FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82613800: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82613804: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82613808: 386AA64C  addi r3, r10, -0x59b4
	ctx.r[3].s64 = ctx.r[10].s64 + -22964;
	// 8261380C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82613810: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 82613814: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82613818: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261381C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82613820: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82613824: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82613828: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261382C: 4BE535F5  bl 0x82466e20
	ctx.lr = 0x82613830;
	sub_82466E20(ctx, base);
	// 82613830: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82613834: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82613838: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261383C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82613840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82613840 size=108
    let mut pc: u32 = 0x82613840;
    'dispatch: loop {
        match pc {
            0x82613840 => {
    //   block [0x82613840..0x826138AC)
	// 82613840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82613844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82613848: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261384C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82613850: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82613854: 38EB6FB4  addi r7, r11, 0x6fb4
	ctx.r[7].s64 = ctx.r[11].s64 + 28596;
	// 82613858: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8261385C: 388A7914  addi r4, r10, 0x7914
	ctx.r[4].s64 = ctx.r[10].s64 + 30996;
	// 82613860: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82613864: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82613868: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261386C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82613870: 386AA67C  addi r3, r10, -0x5984
	ctx.r[3].s64 = ctx.r[10].s64 + -22916;
	// 82613874: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82613878: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261387C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82613880: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82613884: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82613888: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261388C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82613890: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82613894: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82613898: 4BE53589  bl 0x82466e20
	ctx.lr = 0x8261389C;
	sub_82466E20(ctx, base);
	// 8261389C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826138A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826138A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826138A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826138B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826138B0 size=108
    let mut pc: u32 = 0x826138B0;
    'dispatch: loop {
        match pc {
            0x826138B0 => {
    //   block [0x826138B0..0x8261391C)
	// 826138B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826138B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826138B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826138BC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826138C0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826138C4: 38EB6FE4  addi r7, r11, 0x6fe4
	ctx.r[7].s64 = ctx.r[11].s64 + 28644;
	// 826138C8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826138CC: 388A7788  addi r4, r10, 0x7788
	ctx.r[4].s64 = ctx.r[10].s64 + 30600;
	// 826138D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826138D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826138D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826138DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826138E0: 386AA6AC  addi r3, r10, -0x5954
	ctx.r[3].s64 = ctx.r[10].s64 + -22868;
	// 826138E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826138E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826138EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826138F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826138F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826138F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826138FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82613900: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82613904: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82613908: 4BE53519  bl 0x82466e20
	ctx.lr = 0x8261390C;
	sub_82466E20(ctx, base);
	// 8261390C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82613910: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82613914: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82613918: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82613920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82613920 size=108
    let mut pc: u32 = 0x82613920;
    'dispatch: loop {
        match pc {
            0x82613920 => {
    //   block [0x82613920..0x8261398C)
	// 82613920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82613924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82613928: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261392C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82613930: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82613934: 38EB7014  addi r7, r11, 0x7014
	ctx.r[7].s64 = ctx.r[11].s64 + 28692;
	// 82613938: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8261393C: 388A77AC  addi r4, r10, 0x77ac
	ctx.r[4].s64 = ctx.r[10].s64 + 30636;
	// 82613940: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82613944: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82613948: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261394C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82613950: 386AA6DC  addi r3, r10, -0x5924
	ctx.r[3].s64 = ctx.r[10].s64 + -22820;
	// 82613954: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82613958: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261395C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82613960: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82613964: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82613968: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261396C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82613970: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82613974: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82613978: 4BE534A9  bl 0x82466e20
	ctx.lr = 0x8261397C;
	sub_82466E20(ctx, base);
	// 8261397C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82613980: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82613984: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82613988: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82613990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82613990 size=24
    let mut pc: u32 = 0x82613990;
    'dispatch: loop {
        match pc {
            0x82613990 => {
    //   block [0x82613990..0x826139A8)
	// 82613990: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82613994: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82613998: 394AD780  addi r10, r10, -0x2880
	ctx.r[10].s64 = ctx.r[10].s64 + -10368;
	// 8261399C: 816B702C  lwz r11, 0x702c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28716 as u32) ) } as u64;
	// 826139A0: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826139A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826139A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826139A8 size=112
    let mut pc: u32 = 0x826139A8;
    'dispatch: loop {
        match pc {
            0x826139A8 => {
    //   block [0x826139A8..0x82613A18)
	// 826139A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826139AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826139B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826139B4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826139B8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826139BC: 392AF624  addi r9, r10, -0x9dc
	ctx.r[9].s64 = ctx.r[10].s64 + -2524;
	// 826139C0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826139C4: 390BD780  addi r8, r11, -0x2880
	ctx.r[8].s64 = ctx.r[11].s64 + -10368;
	// 826139C8: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 826139CC: 388A77C8  addi r4, r10, 0x77c8
	ctx.r[4].s64 = ctx.r[10].s64 + 30664;
	// 826139D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826139D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826139D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826139DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826139E0: 386AA70C  addi r3, r10, -0x58f4
	ctx.r[3].s64 = ctx.r[10].s64 + -22772;
	// 826139E4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826139E8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826139EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826139F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826139F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826139F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826139FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82613A00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82613A04: 4BE5341D  bl 0x82466e20
	ctx.lr = 0x82613A08;
	sub_82466E20(ctx, base);
	// 82613A08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82613A0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82613A10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82613A14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82613A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82613A18 size=112
    let mut pc: u32 = 0x82613A18;
    'dispatch: loop {
        match pc {
            0x82613A18 => {
    //   block [0x82613A18..0x82613A88)
	// 82613A18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82613A1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82613A20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82613A24: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82613A28: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82613A2C: 392AF660  addi r9, r10, -0x9a0
	ctx.r[9].s64 = ctx.r[10].s64 + -2464;
	// 82613A30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82613A34: 390B7038  addi r8, r11, 0x7038
	ctx.r[8].s64 = ctx.r[11].s64 + 28728;
	// 82613A38: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 82613A3C: 388A8198  addi r4, r10, -0x7e68
	ctx.r[4].s64 = ctx.r[10].s64 + -32360;
	// 82613A40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82613A44: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82613A48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82613A4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82613A50: 386AA73C  addi r3, r10, -0x58c4
	ctx.r[3].s64 = ctx.r[10].s64 + -22724;
	// 82613A54: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82613A58: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82613A5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82613A60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82613A64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82613A68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82613A6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82613A70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82613A74: 4BE533AD  bl 0x82466e20
	ctx.lr = 0x82613A78;
	sub_82466E20(ctx, base);
	// 82613A78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82613A7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82613A80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82613A84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82613A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82613A88 size=108
    let mut pc: u32 = 0x82613A88;
    'dispatch: loop {
        match pc {
            0x82613A88 => {
    //   block [0x82613A88..0x82613AF4)
	// 82613A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82613A8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82613A90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82613A94: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82613A98: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82613A9C: 38EB7080  addi r7, r11, 0x7080
	ctx.r[7].s64 = ctx.r[11].s64 + 28800;
	// 82613AA0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82613AA4: 388A7BB0  addi r4, r10, 0x7bb0
	ctx.r[4].s64 = ctx.r[10].s64 + 31664;
	// 82613AA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82613AAC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82613AB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82613AB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82613AB8: 386AA76C  addi r3, r10, -0x5894
	ctx.r[3].s64 = ctx.r[10].s64 + -22676;
	// 82613ABC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82613AC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82613AC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82613AC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82613ACC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82613AD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82613AD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82613AD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82613ADC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82613AE0: 4BE53341  bl 0x82466e20
	ctx.lr = 0x82613AE4;
	sub_82466E20(ctx, base);
	// 82613AE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82613AE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82613AEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82613AF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82613AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82613AF8 size=108
    let mut pc: u32 = 0x82613AF8;
    'dispatch: loop {
        match pc {
            0x82613AF8 => {
    //   block [0x82613AF8..0x82613B64)
	// 82613AF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82613AFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82613B00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82613B04: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82613B08: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82613B0C: 38EB70B0  addi r7, r11, 0x70b0
	ctx.r[7].s64 = ctx.r[11].s64 + 28848;
	// 82613B10: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82613B14: 388A7F88  addi r4, r10, 0x7f88
	ctx.r[4].s64 = ctx.r[10].s64 + 32648;
	// 82613B18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82613B1C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82613B20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82613B24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82613B28: 386AA79C  addi r3, r10, -0x5864
	ctx.r[3].s64 = ctx.r[10].s64 + -22628;
	// 82613B2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82613B30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82613B34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82613B38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82613B3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82613B40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82613B44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82613B48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82613B4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82613B50: 4BE532D1  bl 0x82466e20
	ctx.lr = 0x82613B54;
	sub_82466E20(ctx, base);
	// 82613B54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82613B58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82613B5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82613B60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82613B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82613B68 size=112
    let mut pc: u32 = 0x82613B68;
    'dispatch: loop {
        match pc {
            0x82613B68 => {
    //   block [0x82613B68..0x82613BD8)
	// 82613B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82613B6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82613B70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82613B74: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82613B78: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82613B7C: 392AF6A0  addi r9, r10, -0x960
	ctx.r[9].s64 = ctx.r[10].s64 + -2400;
	// 82613B80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82613B84: 390B70E8  addi r8, r11, 0x70e8
	ctx.r[8].s64 = ctx.r[11].s64 + 28904;
	// 82613B88: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82613B8C: 388A82E8  addi r4, r10, -0x7d18
	ctx.r[4].s64 = ctx.r[10].s64 + -32024;
	// 82613B90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82613B94: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82613B98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82613B9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82613BA0: 386AA7CC  addi r3, r10, -0x5834
	ctx.r[3].s64 = ctx.r[10].s64 + -22580;
	// 82613BA4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82613BA8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82613BAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82613BB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82613BB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82613BB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82613BBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82613BC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82613BC4: 4BE5325D  bl 0x82466e20
	ctx.lr = 0x82613BC8;
	sub_82466E20(ctx, base);
	// 82613BC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82613BCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82613BD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82613BD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82613BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82613BD8 size=108
    let mut pc: u32 = 0x82613BD8;
    'dispatch: loop {
        match pc {
            0x82613BD8 => {
    //   block [0x82613BD8..0x82613C44)
	// 82613BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82613BDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82613BE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82613BE4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82613BE8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82613BEC: 38EB7160  addi r7, r11, 0x7160
	ctx.r[7].s64 = ctx.r[11].s64 + 29024;
	// 82613BF0: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 82613BF4: 388A7F28  addi r4, r10, 0x7f28
	ctx.r[4].s64 = ctx.r[10].s64 + 32552;
	// 82613BF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82613BFC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82613C00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82613C04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82613C08: 386AA7FC  addi r3, r10, -0x5804
	ctx.r[3].s64 = ctx.r[10].s64 + -22532;
	// 82613C0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82613C10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82613C14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82613C18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82613C1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82613C20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82613C24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82613C28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82613C2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82613C30: 4BE531F1  bl 0x82466e20
	ctx.lr = 0x82613C34;
	sub_82466E20(ctx, base);
	// 82613C34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82613C38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82613C3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82613C40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82613C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82613C48 size=108
    let mut pc: u32 = 0x82613C48;
    'dispatch: loop {
        match pc {
            0x82613C48 => {
    //   block [0x82613C48..0x82613CB4)
	// 82613C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82613C4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82613C50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82613C54: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82613C58: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82613C5C: 38EB7268  addi r7, r11, 0x7268
	ctx.r[7].s64 = ctx.r[11].s64 + 29288;
	// 82613C60: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82613C64: 388A78C0  addi r4, r10, 0x78c0
	ctx.r[4].s64 = ctx.r[10].s64 + 30912;
	// 82613C68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82613C6C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82613C70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82613C74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82613C78: 386AA82C  addi r3, r10, -0x57d4
	ctx.r[3].s64 = ctx.r[10].s64 + -22484;
	// 82613C7C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82613C80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82613C84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82613C88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82613C8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82613C90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82613C94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82613C98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82613C9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82613CA0: 4BE53181  bl 0x82466e20
	ctx.lr = 0x82613CA4;
	sub_82466E20(ctx, base);
	// 82613CA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82613CA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82613CAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82613CB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82613CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82613CB8 size=108
    let mut pc: u32 = 0x82613CB8;
    'dispatch: loop {
        match pc {
            0x82613CB8 => {
    //   block [0x82613CB8..0x82613D24)
	// 82613CB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82613CBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82613CC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82613CC4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82613CC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82613CCC: 38EB7280  addi r7, r11, 0x7280
	ctx.r[7].s64 = ctx.r[11].s64 + 29312;
	// 82613CD0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82613CD4: 388A8060  addi r4, r10, -0x7fa0
	ctx.r[4].s64 = ctx.r[10].s64 + -32672;
	// 82613CD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82613CDC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82613CE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82613CE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82613CE8: 386AA85C  addi r3, r10, -0x57a4
	ctx.r[3].s64 = ctx.r[10].s64 + -22436;
	// 82613CEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82613CF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82613CF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82613CF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82613CFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82613D00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82613D04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82613D08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82613D0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82613D10: 4BE53111  bl 0x82466e20
	ctx.lr = 0x82613D14;
	sub_82466E20(ctx, base);
	// 82613D14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82613D18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82613D1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82613D20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82613D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82613D28 size=24
    let mut pc: u32 = 0x82613D28;
    'dispatch: loop {
        match pc {
            0x82613D28 => {
    //   block [0x82613D28..0x82613D40)
	// 82613D28: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82613D2C: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82613D30: 394AD858  addi r10, r10, -0x27a8
	ctx.r[10].s64 = ctx.r[10].s64 + -10152;
	// 82613D34: 816B70E4  lwz r11, 0x70e4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28900 as u32) ) } as u64;
	// 82613D38: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82613D3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82613D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82613D40 size=108
    let mut pc: u32 = 0x82613D40;
    'dispatch: loop {
        match pc {
            0x82613D40 => {
    //   block [0x82613D40..0x82613DAC)
	// 82613D40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82613D44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82613D48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82613D4C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82613D50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82613D54: 38EBD858  addi r7, r11, -0x27a8
	ctx.r[7].s64 = ctx.r[11].s64 + -10152;
	// 82613D58: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82613D5C: 388AB020  addi r4, r10, -0x4fe0
	ctx.r[4].s64 = ctx.r[10].s64 + -20448;
	// 82613D60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82613D64: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82613D68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82613D6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82613D70: 386AA88C  addi r3, r10, -0x5774
	ctx.r[3].s64 = ctx.r[10].s64 + -22388;
	// 82613D74: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82613D78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82613D7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82613D80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82613D84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82613D88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82613D8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82613D90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82613D94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82613D98: 4BE53089  bl 0x82466e20
	ctx.lr = 0x82613D9C;
	sub_82466E20(ctx, base);
	// 82613D9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82613DA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82613DA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82613DA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82613DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82613DB0 size=24
    let mut pc: u32 = 0x82613DB0;
    'dispatch: loop {
        match pc {
            0x82613DB0 => {
    //   block [0x82613DB0..0x82613DC8)
	// 82613DB0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82613DB4: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82613DB8: 394AD888  addi r10, r10, -0x2778
	ctx.r[10].s64 = ctx.r[10].s64 + -10104;
	// 82613DBC: 816B70E4  lwz r11, 0x70e4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28900 as u32) ) } as u64;
	// 82613DC0: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82613DC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82613DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82613DC8 size=108
    let mut pc: u32 = 0x82613DC8;
    'dispatch: loop {
        match pc {
            0x82613DC8 => {
    //   block [0x82613DC8..0x82613E34)
	// 82613DC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82613DCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82613DD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82613DD4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82613DD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82613DDC: 38EBD888  addi r7, r11, -0x2778
	ctx.r[7].s64 = ctx.r[11].s64 + -10104;
	// 82613DE0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82613DE4: 388A9C48  addi r4, r10, -0x63b8
	ctx.r[4].s64 = ctx.r[10].s64 + -25528;
	// 82613DE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82613DEC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82613DF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82613DF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82613DF8: 386AA8BC  addi r3, r10, -0x5744
	ctx.r[3].s64 = ctx.r[10].s64 + -22340;
	// 82613DFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82613E00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82613E04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82613E08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82613E0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82613E10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82613E14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82613E18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82613E1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82613E20: 4BE53001  bl 0x82466e20
	ctx.lr = 0x82613E24;
	sub_82466E20(ctx, base);
	// 82613E24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82613E28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82613E2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82613E30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82613E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82613E38 size=108
    let mut pc: u32 = 0x82613E38;
    'dispatch: loop {
        match pc {
            0x82613E38 => {
    //   block [0x82613E38..0x82613EA4)
	// 82613E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82613E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82613E40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82613E44: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82613E48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82613E4C: 38EB72F8  addi r7, r11, 0x72f8
	ctx.r[7].s64 = ctx.r[11].s64 + 29432;
	// 82613E50: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82613E54: 388AA080  addi r4, r10, -0x5f80
	ctx.r[4].s64 = ctx.r[10].s64 + -24448;
	// 82613E58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82613E5C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82613E60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82613E64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82613E68: 386AA8EC  addi r3, r10, -0x5714
	ctx.r[3].s64 = ctx.r[10].s64 + -22292;
	// 82613E6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82613E70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82613E74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82613E78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82613E7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82613E80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82613E84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82613E88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82613E8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82613E90: 4BE52F91  bl 0x82466e20
	ctx.lr = 0x82613E94;
	sub_82466E20(ctx, base);
	// 82613E94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82613E98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82613E9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82613EA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82613EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82613EA8 size=24
    let mut pc: u32 = 0x82613EA8;
    'dispatch: loop {
        match pc {
            0x82613EA8 => {
    //   block [0x82613EA8..0x82613EC0)
	// 82613EA8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82613EAC: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82613EB0: 394AD8B8  addi r10, r10, -0x2748
	ctx.r[10].s64 = ctx.r[10].s64 + -10056;
	// 82613EB4: 816B70E4  lwz r11, 0x70e4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28900 as u32) ) } as u64;
	// 82613EB8: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82613EBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82613EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82613EC0 size=108
    let mut pc: u32 = 0x82613EC0;
    'dispatch: loop {
        match pc {
            0x82613EC0 => {
    //   block [0x82613EC0..0x82613F2C)
	// 82613EC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82613EC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82613EC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82613ECC: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82613ED0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82613ED4: 38EBD8B8  addi r7, r11, -0x2748
	ctx.r[7].s64 = ctx.r[11].s64 + -10056;
	// 82613ED8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82613EDC: 388A9BE8  addi r4, r10, -0x6418
	ctx.r[4].s64 = ctx.r[10].s64 + -25624;
	// 82613EE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82613EE4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82613EE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82613EEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82613EF0: 386AA91C  addi r3, r10, -0x56e4
	ctx.r[3].s64 = ctx.r[10].s64 + -22244;
	// 82613EF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82613EF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82613EFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82613F00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82613F04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82613F08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82613F0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82613F10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82613F14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82613F18: 4BE52F09  bl 0x82466e20
	ctx.lr = 0x82613F1C;
	sub_82466E20(ctx, base);
	// 82613F1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82613F20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82613F24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82613F28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82613F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82613F30 size=112
    let mut pc: u32 = 0x82613F30;
    'dispatch: loop {
        match pc {
            0x82613F30 => {
    //   block [0x82613F30..0x82613FA0)
	// 82613F30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82613F34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82613F38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82613F3C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82613F40: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82613F44: 392AF6E4  addi r9, r10, -0x91c
	ctx.r[9].s64 = ctx.r[10].s64 + -2332;
	// 82613F48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82613F4C: 390B7310  addi r8, r11, 0x7310
	ctx.r[8].s64 = ctx.r[11].s64 + 29456;
	// 82613F50: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82613F54: 388AA778  addi r4, r10, -0x5888
	ctx.r[4].s64 = ctx.r[10].s64 + -22664;
	// 82613F58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82613F5C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82613F60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82613F64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82613F68: 386AA94C  addi r3, r10, -0x56b4
	ctx.r[3].s64 = ctx.r[10].s64 + -22196;
	// 82613F6C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82613F70: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82613F74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82613F78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82613F7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82613F80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82613F84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82613F88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82613F8C: 4BE52E95  bl 0x82466e20
	ctx.lr = 0x82613F90;
	sub_82466E20(ctx, base);
	// 82613F90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82613F94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82613F98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82613F9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82613FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82613FA0 size=108
    let mut pc: u32 = 0x82613FA0;
    'dispatch: loop {
        match pc {
            0x82613FA0 => {
    //   block [0x82613FA0..0x8261400C)
	// 82613FA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82613FA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82613FA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82613FAC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82613FB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82613FB4: 38EB7340  addi r7, r11, 0x7340
	ctx.r[7].s64 = ctx.r[11].s64 + 29504;
	// 82613FB8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82613FBC: 388AA350  addi r4, r10, -0x5cb0
	ctx.r[4].s64 = ctx.r[10].s64 + -23728;
	// 82613FC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82613FC4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82613FC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82613FCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82613FD0: 386AA97C  addi r3, r10, -0x5684
	ctx.r[3].s64 = ctx.r[10].s64 + -22148;
	// 82613FD4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82613FD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82613FDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82613FE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82613FE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82613FE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82613FEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82613FF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82613FF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82613FF8: 4BE52E29  bl 0x82466e20
	ctx.lr = 0x82613FFC;
	sub_82466E20(ctx, base);
	// 82613FFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82614000: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82614004: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82614008: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82614010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82614010 size=108
    let mut pc: u32 = 0x82614010;
    'dispatch: loop {
        match pc {
            0x82614010 => {
    //   block [0x82614010..0x8261407C)
	// 82614010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82614014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82614018: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261401C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82614020: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82614024: 38EB7370  addi r7, r11, 0x7370
	ctx.r[7].s64 = ctx.r[11].s64 + 29552;
	// 82614028: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8261402C: 388AB2D8  addi r4, r10, -0x4d28
	ctx.r[4].s64 = ctx.r[10].s64 + -19752;
	// 82614030: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82614034: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82614038: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261403C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82614040: 386AA9AC  addi r3, r10, -0x5654
	ctx.r[3].s64 = ctx.r[10].s64 + -22100;
	// 82614044: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82614048: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261404C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82614050: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82614054: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82614058: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261405C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82614060: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82614064: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82614068: 4BE52DB9  bl 0x82466e20
	ctx.lr = 0x8261406C;
	sub_82466E20(ctx, base);
	// 8261406C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82614070: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82614074: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82614078: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82614080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82614080 size=108
    let mut pc: u32 = 0x82614080;
    'dispatch: loop {
        match pc {
            0x82614080 => {
    //   block [0x82614080..0x826140EC)
	// 82614080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82614084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82614088: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261408C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82614090: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82614094: 38EB7388  addi r7, r11, 0x7388
	ctx.r[7].s64 = ctx.r[11].s64 + 29576;
	// 82614098: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8261409C: 388AAA44  addi r4, r10, -0x55bc
	ctx.r[4].s64 = ctx.r[10].s64 + -21948;
	// 826140A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826140A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826140A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826140AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826140B0: 386AA9DC  addi r3, r10, -0x5624
	ctx.r[3].s64 = ctx.r[10].s64 + -22052;
	// 826140B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826140B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826140BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826140C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826140C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826140C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826140CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826140D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826140D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826140D8: 4BE52D49  bl 0x82466e20
	ctx.lr = 0x826140DC;
	sub_82466E20(ctx, base);
	// 826140DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826140E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826140E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826140E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826140F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826140F0 size=112
    let mut pc: u32 = 0x826140F0;
    'dispatch: loop {
        match pc {
            0x826140F0 => {
    //   block [0x826140F0..0x82614160)
	// 826140F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826140F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826140F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826140FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82614100: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82614104: 38AAAA3C  addi r5, r10, -0x55c4
	ctx.r[5].s64 = ctx.r[10].s64 + -21956;
	// 82614108: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261410C: 390B73B8  addi r8, r11, 0x73b8
	ctx.r[8].s64 = ctx.r[11].s64 + 29624;
	// 82614110: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82614114: 388AA9A0  addi r4, r10, -0x5660
	ctx.r[4].s64 = ctx.r[10].s64 + -22112;
	// 82614118: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261411C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82614120: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82614124: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82614128: 386AAA0C  addi r3, r10, -0x55f4
	ctx.r[3].s64 = ctx.r[10].s64 + -22004;
	// 8261412C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82614130: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82614134: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82614138: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261413C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82614140: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82614144: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82614148: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261414C: 4BE52CD5  bl 0x82466e20
	ctx.lr = 0x82614150;
	sub_82466E20(ctx, base);
	// 82614150: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82614154: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82614158: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261415C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82614160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82614160 size=108
    let mut pc: u32 = 0x82614160;
    'dispatch: loop {
        match pc {
            0x82614160 => {
    //   block [0x82614160..0x826141CC)
	// 82614160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82614164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82614168: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261416C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82614170: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82614174: 38EB73D0  addi r7, r11, 0x73d0
	ctx.r[7].s64 = ctx.r[11].s64 + 29648;
	// 82614178: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8261417C: 388AA640  addi r4, r10, -0x59c0
	ctx.r[4].s64 = ctx.r[10].s64 + -22976;
	// 82614180: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82614184: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82614188: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261418C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82614190: 386AAA3C  addi r3, r10, -0x55c4
	ctx.r[3].s64 = ctx.r[10].s64 + -21956;
	// 82614194: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82614198: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261419C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826141A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826141A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826141A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826141AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826141B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826141B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826141B8: 4BE52C69  bl 0x82466e20
	ctx.lr = 0x826141BC;
	sub_82466E20(ctx, base);
	// 826141BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826141C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826141C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826141C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826141D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826141D0 size=108
    let mut pc: u32 = 0x826141D0;
    'dispatch: loop {
        match pc {
            0x826141D0 => {
    //   block [0x826141D0..0x8261423C)
	// 826141D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826141D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826141D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826141DC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826141E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826141E4: 38EB7400  addi r7, r11, 0x7400
	ctx.r[7].s64 = ctx.r[11].s64 + 29696;
	// 826141E8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826141EC: 388AA2E0  addi r4, r10, -0x5d20
	ctx.r[4].s64 = ctx.r[10].s64 + -23840;
	// 826141F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826141F4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826141F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826141FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82614200: 386AAA6C  addi r3, r10, -0x5594
	ctx.r[3].s64 = ctx.r[10].s64 + -21908;
	// 82614204: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82614208: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261420C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82614210: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82614214: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82614218: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261421C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82614220: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82614224: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82614228: 4BE52BF9  bl 0x82466e20
	ctx.lr = 0x8261422C;
	sub_82466E20(ctx, base);
	// 8261422C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82614230: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82614234: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82614238: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82614240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82614240 size=108
    let mut pc: u32 = 0x82614240;
    'dispatch: loop {
        match pc {
            0x82614240 => {
    //   block [0x82614240..0x826142AC)
	// 82614240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82614244: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82614248: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261424C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82614250: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82614254: 38EB7418  addi r7, r11, 0x7418
	ctx.r[7].s64 = ctx.r[11].s64 + 29720;
	// 82614258: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8261425C: 388AA304  addi r4, r10, -0x5cfc
	ctx.r[4].s64 = ctx.r[10].s64 + -23804;
	// 82614260: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82614264: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82614268: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261426C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82614270: 386AAA9C  addi r3, r10, -0x5564
	ctx.r[3].s64 = ctx.r[10].s64 + -21860;
	// 82614274: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82614278: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261427C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82614280: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82614284: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82614288: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261428C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82614290: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82614294: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82614298: 4BE52B89  bl 0x82466e20
	ctx.lr = 0x8261429C;
	sub_82466E20(ctx, base);
	// 8261429C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826142A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826142A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826142A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826142B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826142B0 size=108
    let mut pc: u32 = 0x826142B0;
    'dispatch: loop {
        match pc {
            0x826142B0 => {
    //   block [0x826142B0..0x8261431C)
	// 826142B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826142B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826142B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826142BC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826142C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826142C4: 38EB7448  addi r7, r11, 0x7448
	ctx.r[7].s64 = ctx.r[11].s64 + 29768;
	// 826142C8: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826142CC: 388AAED0  addi r4, r10, -0x5130
	ctx.r[4].s64 = ctx.r[10].s64 + -20784;
	// 826142D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826142D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826142D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826142DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826142E0: 386AAACC  addi r3, r10, -0x5534
	ctx.r[3].s64 = ctx.r[10].s64 + -21812;
	// 826142E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826142E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826142EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826142F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826142F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826142F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826142FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82614300: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82614304: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82614308: 4BE52B19  bl 0x82466e20
	ctx.lr = 0x8261430C;
	sub_82466E20(ctx, base);
	// 8261430C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82614310: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82614314: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82614318: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82614320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82614320 size=108
    let mut pc: u32 = 0x82614320;
    'dispatch: loop {
        match pc {
            0x82614320 => {
    //   block [0x82614320..0x8261438C)
	// 82614320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82614324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82614328: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261432C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82614330: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82614334: 38EB74F0  addi r7, r11, 0x74f0
	ctx.r[7].s64 = ctx.r[11].s64 + 29936;
	// 82614338: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8261433C: 388AA130  addi r4, r10, -0x5ed0
	ctx.r[4].s64 = ctx.r[10].s64 + -24272;
	// 82614340: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82614344: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82614348: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261434C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82614350: 386AAAFC  addi r3, r10, -0x5504
	ctx.r[3].s64 = ctx.r[10].s64 + -21764;
	// 82614354: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82614358: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261435C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82614360: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82614364: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82614368: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261436C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82614370: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82614374: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82614378: 4BE52AA9  bl 0x82466e20
	ctx.lr = 0x8261437C;
	sub_82466E20(ctx, base);
	// 8261437C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82614380: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82614384: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82614388: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82614390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82614390 size=108
    let mut pc: u32 = 0x82614390;
    'dispatch: loop {
        match pc {
            0x82614390 => {
    //   block [0x82614390..0x826143FC)
	// 82614390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82614394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82614398: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261439C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826143A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826143A4: 38EB7520  addi r7, r11, 0x7520
	ctx.r[7].s64 = ctx.r[11].s64 + 29984;
	// 826143A8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826143AC: 388AA148  addi r4, r10, -0x5eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -24248;
	// 826143B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826143B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826143B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826143BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826143C0: 386AAB2C  addi r3, r10, -0x54d4
	ctx.r[3].s64 = ctx.r[10].s64 + -21716;
	// 826143C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826143C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826143CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826143D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826143D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826143D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826143DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826143E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826143E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826143E8: 4BE52A39  bl 0x82466e20
	ctx.lr = 0x826143EC;
	sub_82466E20(ctx, base);
	// 826143EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826143F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826143F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826143F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82614400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82614400 size=108
    let mut pc: u32 = 0x82614400;
    'dispatch: loop {
        match pc {
            0x82614400 => {
    //   block [0x82614400..0x8261446C)
	// 82614400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82614404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82614408: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261440C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82614410: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82614414: 38EB7538  addi r7, r11, 0x7538
	ctx.r[7].s64 = ctx.r[11].s64 + 30008;
	// 82614418: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8261441C: 388AB290  addi r4, r10, -0x4d70
	ctx.r[4].s64 = ctx.r[10].s64 + -19824;
	// 82614420: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82614424: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82614428: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261442C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82614430: 386AAB5C  addi r3, r10, -0x54a4
	ctx.r[3].s64 = ctx.r[10].s64 + -21668;
	// 82614434: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82614438: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261443C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82614440: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82614444: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82614448: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261444C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82614450: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82614454: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82614458: 4BE529C9  bl 0x82466e20
	ctx.lr = 0x8261445C;
	sub_82466E20(ctx, base);
	// 8261445C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82614460: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82614464: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82614468: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82614470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82614470 size=112
    let mut pc: u32 = 0x82614470;
    'dispatch: loop {
        match pc {
            0x82614470 => {
    //   block [0x82614470..0x826144E0)
	// 82614470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82614474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82614478: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261447C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82614480: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82614484: 38AAA9AC  addi r5, r10, -0x5654
	ctx.r[5].s64 = ctx.r[10].s64 + -22100;
	// 82614488: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261448C: 390B7568  addi r8, r11, 0x7568
	ctx.r[8].s64 = ctx.r[11].s64 + 30056;
	// 82614490: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82614494: 388AB2A8  addi r4, r10, -0x4d58
	ctx.r[4].s64 = ctx.r[10].s64 + -19800;
	// 82614498: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261449C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826144A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826144A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826144A8: 386AAB8C  addi r3, r10, -0x5474
	ctx.r[3].s64 = ctx.r[10].s64 + -21620;
	// 826144AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826144B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826144B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826144B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826144BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826144C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826144C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826144C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826144CC: 4BE52955  bl 0x82466e20
	ctx.lr = 0x826144D0;
	sub_82466E20(ctx, base);
	// 826144D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826144D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826144D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826144DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826144E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826144E0 size=24
    let mut pc: u32 = 0x826144E0;
    'dispatch: loop {
        match pc {
            0x826144E0 => {
    //   block [0x826144E0..0x826144F8)
	// 826144E0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826144E4: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 826144E8: 394AD8E8  addi r10, r10, -0x2718
	ctx.r[10].s64 = ctx.r[10].s64 + -10008;
	// 826144EC: 816B7610  lwz r11, 0x7610(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30224 as u32) ) } as u64;
	// 826144F0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826144F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826144F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826144F8 size=112
    let mut pc: u32 = 0x826144F8;
    'dispatch: loop {
        match pc {
            0x826144F8 => {
    //   block [0x826144F8..0x82614568)
	// 826144F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826144FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82614500: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82614504: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82614508: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261450C: 392AF710  addi r9, r10, -0x8f0
	ctx.r[9].s64 = ctx.r[10].s64 + -2288;
	// 82614510: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82614514: 390BD8E8  addi r8, r11, -0x2718
	ctx.r[8].s64 = ctx.r[11].s64 + -10008;
	// 82614518: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8261451C: 388AB098  addi r4, r10, -0x4f68
	ctx.r[4].s64 = ctx.r[10].s64 + -20328;
	// 82614520: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82614524: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82614528: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261452C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82614530: 386AABBC  addi r3, r10, -0x5444
	ctx.r[3].s64 = ctx.r[10].s64 + -21572;
	// 82614534: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82614538: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8261453C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82614540: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82614544: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82614548: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261454C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82614550: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82614554: 4BE528CD  bl 0x82466e20
	ctx.lr = 0x82614558;
	sub_82466E20(ctx, base);
	// 82614558: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261455C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82614560: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82614564: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82614568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82614568 size=108
    let mut pc: u32 = 0x82614568;
    'dispatch: loop {
        match pc {
            0x82614568 => {
    //   block [0x82614568..0x826145D4)
	// 82614568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261456C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82614570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82614574: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82614578: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261457C: 38EB7618  addi r7, r11, 0x7618
	ctx.r[7].s64 = ctx.r[11].s64 + 30232;
	// 82614580: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82614584: 388AAC70  addi r4, r10, -0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + -21392;
	// 82614588: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261458C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82614590: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82614594: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82614598: 386AABEC  addi r3, r10, -0x5414
	ctx.r[3].s64 = ctx.r[10].s64 + -21524;
	// 8261459C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826145A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826145A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826145A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826145AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826145B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826145B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826145B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826145BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826145C0: 4BE52861  bl 0x82466e20
	ctx.lr = 0x826145C4;
	sub_82466E20(ctx, base);
	// 826145C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826145C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826145CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826145D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826145D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826145D8 size=116
    let mut pc: u32 = 0x826145D8;
    'dispatch: loop {
        match pc {
            0x826145D8 => {
    //   block [0x826145D8..0x8261464C)
	// 826145D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826145DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826145E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826145E4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826145E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826145EC: 390B7648  addi r8, r11, 0x7648
	ctx.r[8].s64 = ctx.r[11].s64 + 30280;
	// 826145F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826145F4: 392AF754  addi r9, r10, -0x8ac
	ctx.r[9].s64 = ctx.r[10].s64 + -2220;
	// 826145F8: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826145FC: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 82614600: 38AAA9AC  addi r5, r10, -0x5654
	ctx.r[5].s64 = ctx.r[10].s64 + -22100;
	// 82614604: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82614608: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261460C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82614610: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82614614: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82614618: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261461C: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82614620: 388AAC88  addi r4, r10, -0x5378
	ctx.r[4].s64 = ctx.r[10].s64 + -21368;
	// 82614624: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82614628: 386BAC1C  addi r3, r11, -0x53e4
	ctx.r[3].s64 = ctx.r[11].s64 + -21476;
	// 8261462C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82614630: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82614634: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82614638: 4BE527E9  bl 0x82466e20
	ctx.lr = 0x8261463C;
	sub_82466E20(ctx, base);
	// 8261463C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82614640: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82614644: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82614648: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82614650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82614650 size=24
    let mut pc: u32 = 0x82614650;
    'dispatch: loop {
        match pc {
            0x82614650 => {
    //   block [0x82614650..0x82614668)
	// 82614650: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82614654: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82614658: 394AD960  addi r10, r10, -0x26a0
	ctx.r[10].s64 = ctx.r[10].s64 + -9888;
	// 8261465C: 816B7708  lwz r11, 0x7708(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30472 as u32) ) } as u64;
	// 82614660: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82614664: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82614668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82614668 size=112
    let mut pc: u32 = 0x82614668;
    'dispatch: loop {
        match pc {
            0x82614668 => {
    //   block [0x82614668..0x826146D8)
	// 82614668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261466C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82614670: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82614674: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82614678: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261467C: 392AF790  addi r9, r10, -0x870
	ctx.r[9].s64 = ctx.r[10].s64 + -2160;
	// 82614680: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82614684: 390BD960  addi r8, r11, -0x26a0
	ctx.r[8].s64 = ctx.r[11].s64 + -9888;
	// 82614688: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8261468C: 388AB004  addi r4, r10, -0x4ffc
	ctx.r[4].s64 = ctx.r[10].s64 + -20476;
	// 82614690: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82614694: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82614698: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261469C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826146A0: 386AAC4C  addi r3, r10, -0x53b4
	ctx.r[3].s64 = ctx.r[10].s64 + -21428;
	// 826146A4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826146A8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826146AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826146B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826146B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826146B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826146BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826146C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826146C4: 4BE5275D  bl 0x82466e20
	ctx.lr = 0x826146C8;
	sub_82466E20(ctx, base);
	// 826146C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826146CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826146D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826146D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826146D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826146D8 size=108
    let mut pc: u32 = 0x826146D8;
    'dispatch: loop {
        match pc {
            0x826146D8 => {
    //   block [0x826146D8..0x82614744)
	// 826146D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826146DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826146E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826146E4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826146E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826146EC: 38EB770C  addi r7, r11, 0x770c
	ctx.r[7].s64 = ctx.r[11].s64 + 30476;
	// 826146F0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826146F4: 388AAF00  addi r4, r10, -0x5100
	ctx.r[4].s64 = ctx.r[10].s64 + -20736;
	// 826146F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826146FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82614700: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82614704: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82614708: 386AAC7C  addi r3, r10, -0x5384
	ctx.r[3].s64 = ctx.r[10].s64 + -21380;
	// 8261470C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82614710: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82614714: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82614718: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261471C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82614720: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82614724: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82614728: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261472C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82614730: 4BE526F1  bl 0x82466e20
	ctx.lr = 0x82614734;
	sub_82466E20(ctx, base);
	// 82614734: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82614738: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261473C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82614740: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82614748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82614748 size=108
    let mut pc: u32 = 0x82614748;
    'dispatch: loop {
        match pc {
            0x82614748 => {
    //   block [0x82614748..0x826147B4)
	// 82614748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261474C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82614750: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82614754: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82614758: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261475C: 38EB7724  addi r7, r11, 0x7724
	ctx.r[7].s64 = ctx.r[11].s64 + 30500;
	// 82614760: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82614764: 388A9D38  addi r4, r10, -0x62c8
	ctx.r[4].s64 = ctx.r[10].s64 + -25288;
	// 82614768: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261476C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82614770: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82614774: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82614778: 386AACAC  addi r3, r10, -0x5354
	ctx.r[3].s64 = ctx.r[10].s64 + -21332;
	// 8261477C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82614780: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82614784: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82614788: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261478C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82614790: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82614794: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82614798: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261479C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826147A0: 4BE52681  bl 0x82466e20
	ctx.lr = 0x826147A4;
	sub_82466E20(ctx, base);
	// 826147A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826147A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826147AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826147B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826147B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826147B8 size=24
    let mut pc: u32 = 0x826147B8;
    'dispatch: loop {
        match pc {
            0x826147B8 => {
    //   block [0x826147B8..0x826147D0)
	// 826147B8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826147BC: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 826147C0: 394AD9A8  addi r10, r10, -0x2658
	ctx.r[10].s64 = ctx.r[10].s64 + -9816;
	// 826147C4: 816B7754  lwz r11, 0x7754(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30548 as u32) ) } as u64;
	// 826147C8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826147CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826147D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826147D0 size=112
    let mut pc: u32 = 0x826147D0;
    'dispatch: loop {
        match pc {
            0x826147D0 => {
    //   block [0x826147D0..0x82614840)
	// 826147D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826147D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826147D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826147DC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826147E0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826147E4: 392AF7CC  addi r9, r10, -0x834
	ctx.r[9].s64 = ctx.r[10].s64 + -2100;
	// 826147E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826147EC: 390BD9A8  addi r8, r11, -0x2658
	ctx.r[8].s64 = ctx.r[11].s64 + -9816;
	// 826147F0: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826147F4: 388AB450  addi r4, r10, -0x4bb0
	ctx.r[4].s64 = ctx.r[10].s64 + -19376;
	// 826147F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826147FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82614800: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82614804: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82614808: 386AACDC  addi r3, r10, -0x5324
	ctx.r[3].s64 = ctx.r[10].s64 + -21284;
	// 8261480C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82614810: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82614814: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82614818: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261481C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82614820: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82614824: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82614828: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261482C: 4BE525F5  bl 0x82466e20
	ctx.lr = 0x82614830;
	sub_82466E20(ctx, base);
	// 82614830: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82614834: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82614838: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261483C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82614840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82614840 size=112
    let mut pc: u32 = 0x82614840;
    'dispatch: loop {
        match pc {
            0x82614840 => {
    //   block [0x82614840..0x826148B0)
	// 82614840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82614844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82614848: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261484C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82614850: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82614854: 38AAA9AC  addi r5, r10, -0x5654
	ctx.r[5].s64 = ctx.r[10].s64 + -22100;
	// 82614858: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261485C: 390B7758  addi r8, r11, 0x7758
	ctx.r[8].s64 = ctx.r[11].s64 + 30552;
	// 82614860: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82614864: 388A9CDC  addi r4, r10, -0x6324
	ctx.r[4].s64 = ctx.r[10].s64 + -25380;
	// 82614868: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261486C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82614870: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82614874: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82614878: 386AAD0C  addi r3, r10, -0x52f4
	ctx.r[3].s64 = ctx.r[10].s64 + -21236;
	// 8261487C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82614880: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82614884: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82614888: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261488C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82614890: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82614894: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82614898: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261489C: 4BE52585  bl 0x82466e20
	ctx.lr = 0x826148A0;
	sub_82466E20(ctx, base);
	// 826148A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826148A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826148A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826148AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826148B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826148B0 size=108
    let mut pc: u32 = 0x826148B0;
    'dispatch: loop {
        match pc {
            0x826148B0 => {
    //   block [0x826148B0..0x8261491C)
	// 826148B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826148B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826148B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826148BC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826148C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826148C4: 38EB7788  addi r7, r11, 0x7788
	ctx.r[7].s64 = ctx.r[11].s64 + 30600;
	// 826148C8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826148CC: 388A9CF4  addi r4, r10, -0x630c
	ctx.r[4].s64 = ctx.r[10].s64 + -25356;
	// 826148D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826148D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826148D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826148DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826148E0: 386AAD3C  addi r3, r10, -0x52c4
	ctx.r[3].s64 = ctx.r[10].s64 + -21188;
	// 826148E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826148E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826148EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826148F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826148F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826148F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826148FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82614900: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82614904: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82614908: 4BE52519  bl 0x82466e20
	ctx.lr = 0x8261490C;
	sub_82466E20(ctx, base);
	// 8261490C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82614910: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82614914: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82614918: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82614920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82614920 size=108
    let mut pc: u32 = 0x82614920;
    'dispatch: loop {
        match pc {
            0x82614920 => {
    //   block [0x82614920..0x8261498C)
	// 82614920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82614924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82614928: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261492C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82614930: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82614934: 38EB77B8  addi r7, r11, 0x77b8
	ctx.r[7].s64 = ctx.r[11].s64 + 30648;
	// 82614938: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8261493C: 388AB148  addi r4, r10, -0x4eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -20152;
	// 82614940: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82614944: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82614948: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261494C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82614950: 386AAD6C  addi r3, r10, -0x5294
	ctx.r[3].s64 = ctx.r[10].s64 + -21140;
	// 82614954: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82614958: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261495C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82614960: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82614964: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82614968: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261496C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82614970: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82614974: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82614978: 4BE524A9  bl 0x82466e20
	ctx.lr = 0x8261497C;
	sub_82466E20(ctx, base);
	// 8261497C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82614980: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82614984: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82614988: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82614990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82614990 size=108
    let mut pc: u32 = 0x82614990;
    'dispatch: loop {
        match pc {
            0x82614990 => {
    //   block [0x82614990..0x826149FC)
	// 82614990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82614994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82614998: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261499C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826149A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826149A4: 38EB7818  addi r7, r11, 0x7818
	ctx.r[7].s64 = ctx.r[11].s64 + 30744;
	// 826149A8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826149AC: 388AA9FC  addi r4, r10, -0x5604
	ctx.r[4].s64 = ctx.r[10].s64 + -22020;
	// 826149B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826149B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826149B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826149BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826149C0: 386AAD9C  addi r3, r10, -0x5264
	ctx.r[3].s64 = ctx.r[10].s64 + -21092;
	// 826149C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826149C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826149CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826149D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826149D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826149D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826149DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826149E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826149E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826149E8: 4BE52439  bl 0x82466e20
	ctx.lr = 0x826149EC;
	sub_82466E20(ctx, base);
	// 826149EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826149F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826149F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826149F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82614A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82614A00 size=108
    let mut pc: u32 = 0x82614A00;
    'dispatch: loop {
        match pc {
            0x82614A00 => {
    //   block [0x82614A00..0x82614A6C)
	// 82614A00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82614A04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82614A08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82614A0C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82614A10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82614A14: 38EB7848  addi r7, r11, 0x7848
	ctx.r[7].s64 = ctx.r[11].s64 + 30792;
	// 82614A18: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 82614A1C: 388AA978  addi r4, r10, -0x5688
	ctx.r[4].s64 = ctx.r[10].s64 + -22152;
	// 82614A20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82614A24: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82614A28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82614A2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82614A30: 386AADCC  addi r3, r10, -0x5234
	ctx.r[3].s64 = ctx.r[10].s64 + -21044;
	// 82614A34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82614A38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82614A3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82614A40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82614A44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82614A48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82614A4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82614A50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82614A54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82614A58: 4BE523C9  bl 0x82466e20
	ctx.lr = 0x82614A5C;
	sub_82466E20(ctx, base);
	// 82614A5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82614A60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82614A64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82614A68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82614A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82614A70 size=108
    let mut pc: u32 = 0x82614A70;
    'dispatch: loop {
        match pc {
            0x82614A70 => {
    //   block [0x82614A70..0x82614ADC)
	// 82614A70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82614A74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82614A78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82614A7C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82614A80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82614A84: 38EB7968  addi r7, r11, 0x7968
	ctx.r[7].s64 = ctx.r[11].s64 + 31080;
	// 82614A88: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82614A8C: 388AA044  addi r4, r10, -0x5fbc
	ctx.r[4].s64 = ctx.r[10].s64 + -24508;
	// 82614A90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82614A94: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82614A98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82614A9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82614AA0: 386AADFC  addi r3, r10, -0x5204
	ctx.r[3].s64 = ctx.r[10].s64 + -20996;
	// 82614AA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82614AA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82614AAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82614AB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82614AB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82614AB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82614ABC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82614AC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82614AC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82614AC8: 4BE52359  bl 0x82466e20
	ctx.lr = 0x82614ACC;
	sub_82466E20(ctx, base);
	// 82614ACC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82614AD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82614AD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82614AD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82614AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82614AE0 size=108
    let mut pc: u32 = 0x82614AE0;
    'dispatch: loop {
        match pc {
            0x82614AE0 => {
    //   block [0x82614AE0..0x82614B4C)
	// 82614AE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82614AE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82614AE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82614AEC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82614AF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82614AF4: 38EB7980  addi r7, r11, 0x7980
	ctx.r[7].s64 = ctx.r[11].s64 + 31104;
	// 82614AF8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82614AFC: 388AAF3C  addi r4, r10, -0x50c4
	ctx.r[4].s64 = ctx.r[10].s64 + -20676;
	// 82614B00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82614B04: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82614B08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82614B0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82614B10: 386AAE2C  addi r3, r10, -0x51d4
	ctx.r[3].s64 = ctx.r[10].s64 + -20948;
	// 82614B14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82614B18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82614B1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82614B20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82614B24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82614B28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82614B2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82614B30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82614B34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82614B38: 4BE522E9  bl 0x82466e20
	ctx.lr = 0x82614B3C;
	sub_82466E20(ctx, base);
	// 82614B3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82614B40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82614B44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82614B48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82614B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82614B50 size=108
    let mut pc: u32 = 0x82614B50;
    'dispatch: loop {
        match pc {
            0x82614B50 => {
    //   block [0x82614B50..0x82614BBC)
	// 82614B50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82614B54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82614B58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82614B5C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82614B60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82614B64: 38EB7998  addi r7, r11, 0x7998
	ctx.r[7].s64 = ctx.r[11].s64 + 31128;
	// 82614B68: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82614B6C: 388AA0C0  addi r4, r10, -0x5f40
	ctx.r[4].s64 = ctx.r[10].s64 + -24384;
	// 82614B70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82614B74: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82614B78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82614B7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82614B80: 386AAE5C  addi r3, r10, -0x51a4
	ctx.r[3].s64 = ctx.r[10].s64 + -20900;
	// 82614B84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82614B88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82614B8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82614B90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82614B94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82614B98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82614B9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82614BA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82614BA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82614BA8: 4BE52279  bl 0x82466e20
	ctx.lr = 0x82614BAC;
	sub_82466E20(ctx, base);
	// 82614BAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82614BB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82614BB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82614BB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82614BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82614BC0 size=108
    let mut pc: u32 = 0x82614BC0;
    'dispatch: loop {
        match pc {
            0x82614BC0 => {
    //   block [0x82614BC0..0x82614C2C)
	// 82614BC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82614BC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82614BC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82614BCC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82614BD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82614BD4: 38EB79B0  addi r7, r11, 0x79b0
	ctx.r[7].s64 = ctx.r[11].s64 + 31152;
	// 82614BD8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82614BDC: 388A9C24  addi r4, r10, -0x63dc
	ctx.r[4].s64 = ctx.r[10].s64 + -25564;
	// 82614BE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82614BE4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82614BE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82614BEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82614BF0: 386AAE8C  addi r3, r10, -0x5174
	ctx.r[3].s64 = ctx.r[10].s64 + -20852;
	// 82614BF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82614BF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82614BFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82614C00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82614C04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82614C08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82614C0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82614C10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82614C14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82614C18: 4BE52209  bl 0x82466e20
	ctx.lr = 0x82614C1C;
	sub_82466E20(ctx, base);
	// 82614C1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82614C20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82614C24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82614C28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82614C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82614C30 size=108
    let mut pc: u32 = 0x82614C30;
    'dispatch: loop {
        match pc {
            0x82614C30 => {
    //   block [0x82614C30..0x82614C9C)
	// 82614C30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82614C34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82614C38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82614C3C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82614C40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82614C44: 38EB79C8  addi r7, r11, 0x79c8
	ctx.r[7].s64 = ctx.r[11].s64 + 31176;
	// 82614C48: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82614C4C: 388AA684  addi r4, r10, -0x597c
	ctx.r[4].s64 = ctx.r[10].s64 + -22908;
	// 82614C50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82614C54: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82614C58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82614C5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82614C60: 386AAEBC  addi r3, r10, -0x5144
	ctx.r[3].s64 = ctx.r[10].s64 + -20804;
	// 82614C64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82614C68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82614C6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82614C70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82614C74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82614C78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82614C7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82614C80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82614C84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82614C88: 4BE52199  bl 0x82466e20
	ctx.lr = 0x82614C8C;
	sub_82466E20(ctx, base);
	// 82614C8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82614C90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82614C94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82614C98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82614CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82614CA0 size=108
    let mut pc: u32 = 0x82614CA0;
    'dispatch: loop {
        match pc {
            0x82614CA0 => {
    //   block [0x82614CA0..0x82614D0C)
	// 82614CA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82614CA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82614CA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82614CAC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82614CB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82614CB4: 38EB79E0  addi r7, r11, 0x79e0
	ctx.r[7].s64 = ctx.r[11].s64 + 31200;
	// 82614CB8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82614CBC: 388AA26C  addi r4, r10, -0x5d94
	ctx.r[4].s64 = ctx.r[10].s64 + -23956;
	// 82614CC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82614CC4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82614CC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82614CCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82614CD0: 386AAEEC  addi r3, r10, -0x5114
	ctx.r[3].s64 = ctx.r[10].s64 + -20756;
	// 82614CD4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82614CD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82614CDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82614CE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82614CE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82614CE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82614CEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82614CF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82614CF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82614CF8: 4BE52129  bl 0x82466e20
	ctx.lr = 0x82614CFC;
	sub_82466E20(ctx, base);
	// 82614CFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82614D00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82614D04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82614D08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82614D10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82614D10 size=108
    let mut pc: u32 = 0x82614D10;
    'dispatch: loop {
        match pc {
            0x82614D10 => {
    //   block [0x82614D10..0x82614D7C)
	// 82614D10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82614D14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82614D18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82614D1C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82614D20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82614D24: 38EB79F8  addi r7, r11, 0x79f8
	ctx.r[7].s64 = ctx.r[11].s64 + 31224;
	// 82614D28: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82614D2C: 388AB380  addi r4, r10, -0x4c80
	ctx.r[4].s64 = ctx.r[10].s64 + -19584;
	// 82614D30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82614D34: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82614D38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82614D3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82614D40: 386AAF1C  addi r3, r10, -0x50e4
	ctx.r[3].s64 = ctx.r[10].s64 + -20708;
	// 82614D44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82614D48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82614D4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82614D50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82614D54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82614D58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82614D5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82614D60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82614D64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82614D68: 4BE520B9  bl 0x82466e20
	ctx.lr = 0x82614D6C;
	sub_82466E20(ctx, base);
	// 82614D6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82614D70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82614D74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82614D78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82614D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82614D80 size=108
    let mut pc: u32 = 0x82614D80;
    'dispatch: loop {
        match pc {
            0x82614D80 => {
    //   block [0x82614D80..0x82614DEC)
	// 82614D80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82614D84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82614D88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82614D8C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82614D90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82614D94: 38EB7A88  addi r7, r11, 0x7a88
	ctx.r[7].s64 = ctx.r[11].s64 + 31368;
	// 82614D98: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 82614D9C: 388AA228  addi r4, r10, -0x5dd8
	ctx.r[4].s64 = ctx.r[10].s64 + -24024;
	// 82614DA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82614DA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82614DA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82614DAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82614DB0: 386AAF4C  addi r3, r10, -0x50b4
	ctx.r[3].s64 = ctx.r[10].s64 + -20660;
	// 82614DB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82614DB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82614DBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82614DC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82614DC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82614DC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82614DCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82614DD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82614DD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82614DD8: 4BE52049  bl 0x82466e20
	ctx.lr = 0x82614DDC;
	sub_82466E20(ctx, base);
	// 82614DDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82614DE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82614DE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82614DE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82614DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82614DF0 size=108
    let mut pc: u32 = 0x82614DF0;
    'dispatch: loop {
        match pc {
            0x82614DF0 => {
    //   block [0x82614DF0..0x82614E5C)
	// 82614DF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82614DF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82614DF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82614DFC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82614E00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82614E04: 38EB7B48  addi r7, r11, 0x7b48
	ctx.r[7].s64 = ctx.r[11].s64 + 31560;
	// 82614E08: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 82614E0C: 388AA000  addi r4, r10, -0x6000
	ctx.r[4].s64 = ctx.r[10].s64 + -24576;
	// 82614E10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82614E14: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82614E18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82614E1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82614E20: 386AAF7C  addi r3, r10, -0x5084
	ctx.r[3].s64 = ctx.r[10].s64 + -20612;
	// 82614E24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82614E28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82614E2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82614E30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82614E34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82614E38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82614E3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82614E40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82614E44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82614E48: 4BE51FD9  bl 0x82466e20
	ctx.lr = 0x82614E4C;
	sub_82466E20(ctx, base);
	// 82614E4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82614E50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82614E54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82614E58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82614E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82614E60 size=108
    let mut pc: u32 = 0x82614E60;
    'dispatch: loop {
        match pc {
            0x82614E60 => {
    //   block [0x82614E60..0x82614ECC)
	// 82614E60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82614E64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82614E68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82614E6C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82614E70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82614E74: 38EB7C20  addi r7, r11, 0x7c20
	ctx.r[7].s64 = ctx.r[11].s64 + 31776;
	// 82614E78: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 82614E7C: 388A9F00  addi r4, r10, -0x6100
	ctx.r[4].s64 = ctx.r[10].s64 + -24832;
	// 82614E80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82614E84: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82614E88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82614E8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82614E90: 386AAFAC  addi r3, r10, -0x5054
	ctx.r[3].s64 = ctx.r[10].s64 + -20564;
	// 82614E94: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82614E98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82614E9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82614EA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82614EA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82614EA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82614EAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82614EB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82614EB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82614EB8: 4BE51F69  bl 0x82466e20
	ctx.lr = 0x82614EBC;
	sub_82466E20(ctx, base);
	// 82614EBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82614EC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82614EC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82614EC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82614ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82614ED0 size=108
    let mut pc: u32 = 0x82614ED0;
    'dispatch: loop {
        match pc {
            0x82614ED0 => {
    //   block [0x82614ED0..0x82614F3C)
	// 82614ED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82614ED4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82614ED8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82614EDC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82614EE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82614EE4: 38EB7CE0  addi r7, r11, 0x7ce0
	ctx.r[7].s64 = ctx.r[11].s64 + 31968;
	// 82614EE8: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 82614EEC: 388AADE0  addi r4, r10, -0x5220
	ctx.r[4].s64 = ctx.r[10].s64 + -21024;
	// 82614EF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82614EF4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82614EF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82614EFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82614F00: 386AAFDC  addi r3, r10, -0x5024
	ctx.r[3].s64 = ctx.r[10].s64 + -20516;
	// 82614F04: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82614F08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82614F0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82614F10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82614F14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82614F18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82614F1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82614F20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82614F24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82614F28: 4BE51EF9  bl 0x82466e20
	ctx.lr = 0x82614F2C;
	sub_82466E20(ctx, base);
	// 82614F2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82614F30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82614F34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82614F38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82614F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82614F40 size=112
    let mut pc: u32 = 0x82614F40;
    'dispatch: loop {
        match pc {
            0x82614F40 => {
    //   block [0x82614F40..0x82614FB0)
	// 82614F40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82614F44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82614F48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82614F4C: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82614F50: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 82614F54: 38EA7D88  addi r7, r10, 0x7d88
	ctx.r[7].s64 = ctx.r[10].s64 + 32136;
	// 82614F58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82614F5C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82614F60: 388AA5FC  addi r4, r10, -0x5a04
	ctx.r[4].s64 = ctx.r[10].s64 + -23044;
	// 82614F64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82614F68: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82614F6C: 396BF7E0  addi r11, r11, -0x820
	ctx.r[11].s64 = ctx.r[11].s64 + -2080;
	// 82614F70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82614F74: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82614F78: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82614F7C: 386AB00C  addi r3, r10, -0x4ff4
	ctx.r[3].s64 = ctx.r[10].s64 + -20468;
	// 82614F80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82614F84: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82614F88: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82614F8C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82614F90: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82614F94: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82614F98: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82614F9C: 4BE51E85  bl 0x82466e20
	ctx.lr = 0x82614FA0;
	sub_82466E20(ctx, base);
	// 82614FA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82614FA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82614FA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82614FAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82614FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82614FB0 size=108
    let mut pc: u32 = 0x82614FB0;
    'dispatch: loop {
        match pc {
            0x82614FB0 => {
    //   block [0x82614FB0..0x8261501C)
	// 82614FB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82614FB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82614FB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82614FBC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82614FC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82614FC4: 38EB7EA8  addi r7, r11, 0x7ea8
	ctx.r[7].s64 = ctx.r[11].s64 + 32424;
	// 82614FC8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82614FCC: 388AAD28  addi r4, r10, -0x52d8
	ctx.r[4].s64 = ctx.r[10].s64 + -21208;
	// 82614FD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82614FD4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82614FD8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82614FDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82614FE0: 386AB03C  addi r3, r10, -0x4fc4
	ctx.r[3].s64 = ctx.r[10].s64 + -20420;
	// 82614FE4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82614FE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82614FEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82614FF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82614FF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82614FF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82614FFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82615000: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82615004: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82615008: 4BE51E19  bl 0x82466e20
	ctx.lr = 0x8261500C;
	sub_82466E20(ctx, base);
	// 8261500C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82615010: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82615014: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82615018: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82615020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82615020 size=108
    let mut pc: u32 = 0x82615020;
    'dispatch: loop {
        match pc {
            0x82615020 => {
    //   block [0x82615020..0x8261508C)
	// 82615020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82615024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82615028: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261502C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82615030: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82615034: 38EB7F08  addi r7, r11, 0x7f08
	ctx.r[7].s64 = ctx.r[11].s64 + 32520;
	// 82615038: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 8261503C: 388A9070  addi r4, r10, -0x6f90
	ctx.r[4].s64 = ctx.r[10].s64 + -28560;
	// 82615040: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82615044: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615048: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261504C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82615050: 386AB06C  addi r3, r10, -0x4f94
	ctx.r[3].s64 = ctx.r[10].s64 + -20372;
	// 82615054: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82615058: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261505C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82615060: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82615064: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82615068: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261506C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82615070: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82615074: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82615078: 4BE51DA9  bl 0x82466e20
	ctx.lr = 0x8261507C;
	sub_82466E20(ctx, base);
	// 8261507C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82615080: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82615084: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82615088: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82615090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82615090 size=108
    let mut pc: u32 = 0x82615090;
    'dispatch: loop {
        match pc {
            0x82615090 => {
    //   block [0x82615090..0x826150FC)
	// 82615090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82615094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82615098: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261509C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826150A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826150A4: 38EB8010  addi r7, r11, -0x7ff0
	ctx.r[7].s64 = ctx.r[11].s64 + -32752;
	// 826150A8: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826150AC: 388A8E98  addi r4, r10, -0x7168
	ctx.r[4].s64 = ctx.r[10].s64 + -29032;
	// 826150B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826150B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826150B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826150BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826150C0: 386AB09C  addi r3, r10, -0x4f64
	ctx.r[3].s64 = ctx.r[10].s64 + -20324;
	// 826150C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826150C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826150CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826150D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826150D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826150D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826150DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826150E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826150E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826150E8: 4BE51D39  bl 0x82466e20
	ctx.lr = 0x826150EC;
	sub_82466E20(ctx, base);
	// 826150EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826150F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826150F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826150F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82615100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82615100 size=108
    let mut pc: u32 = 0x82615100;
    'dispatch: loop {
        match pc {
            0x82615100 => {
    //   block [0x82615100..0x8261516C)
	// 82615100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82615104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82615108: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261510C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82615110: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82615114: 38EB80E8  addi r7, r11, -0x7f18
	ctx.r[7].s64 = ctx.r[11].s64 + -32536;
	// 82615118: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8261511C: 388A8CF0  addi r4, r10, -0x7310
	ctx.r[4].s64 = ctx.r[10].s64 + -29456;
	// 82615120: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82615124: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615128: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261512C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82615130: 386AB0CC  addi r3, r10, -0x4f34
	ctx.r[3].s64 = ctx.r[10].s64 + -20276;
	// 82615134: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82615138: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261513C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82615140: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82615144: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82615148: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261514C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82615150: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82615154: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82615158: 4BE51CC9  bl 0x82466e20
	ctx.lr = 0x8261515C;
	sub_82466E20(ctx, base);
	// 8261515C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82615160: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82615164: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82615168: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82615170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82615170 size=108
    let mut pc: u32 = 0x82615170;
    'dispatch: loop {
        match pc {
            0x82615170 => {
    //   block [0x82615170..0x826151DC)
	// 82615170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82615174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82615178: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261517C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82615180: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82615184: 38EB8130  addi r7, r11, -0x7ed0
	ctx.r[7].s64 = ctx.r[11].s64 + -32464;
	// 82615188: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8261518C: 388A8D14  addi r4, r10, -0x72ec
	ctx.r[4].s64 = ctx.r[10].s64 + -29420;
	// 82615190: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82615194: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615198: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261519C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826151A0: 386AB0FC  addi r3, r10, -0x4f04
	ctx.r[3].s64 = ctx.r[10].s64 + -20228;
	// 826151A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826151A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826151AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826151B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826151B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826151B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826151BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826151C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826151C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826151C8: 4BE51C59  bl 0x82466e20
	ctx.lr = 0x826151CC;
	sub_82466E20(ctx, base);
	// 826151CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826151D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826151D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826151D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826151E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826151E0 size=112
    let mut pc: u32 = 0x826151E0;
    'dispatch: loop {
        match pc {
            0x826151E0 => {
    //   block [0x826151E0..0x82615250)
	// 826151E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826151E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826151E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826151EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826151F0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826151F4: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 826151F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826151FC: 390B8148  addi r8, r11, -0x7eb8
	ctx.r[8].s64 = ctx.r[11].s64 + -32440;
	// 82615200: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82615204: 388A3810  addi r4, r10, 0x3810
	ctx.r[4].s64 = ctx.r[10].s64 + 14352;
	// 82615208: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261520C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615210: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82615214: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82615218: 386AB12C  addi r3, r10, -0x4ed4
	ctx.r[3].s64 = ctx.r[10].s64 + -20180;
	// 8261521C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82615220: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82615224: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82615228: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261522C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82615230: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82615234: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82615238: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261523C: 4BE51BE5  bl 0x82466e20
	ctx.lr = 0x82615240;
	sub_82466E20(ctx, base);
	// 82615240: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82615244: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82615248: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261524C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82615250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82615250 size=112
    let mut pc: u32 = 0x82615250;
    'dispatch: loop {
        match pc {
            0x82615250 => {
    //   block [0x82615250..0x826152C0)
	// 82615250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82615254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82615258: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261525C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615260: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82615264: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 82615268: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261526C: 390B81A8  addi r8, r11, -0x7e58
	ctx.r[8].s64 = ctx.r[11].s64 + -32344;
	// 82615270: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82615274: 388A3824  addi r4, r10, 0x3824
	ctx.r[4].s64 = ctx.r[10].s64 + 14372;
	// 82615278: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261527C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615280: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82615284: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82615288: 386AB15C  addi r3, r10, -0x4ea4
	ctx.r[3].s64 = ctx.r[10].s64 + -20132;
	// 8261528C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82615290: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82615294: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82615298: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261529C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826152A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826152A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826152A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826152AC: 4BE51B75  bl 0x82466e20
	ctx.lr = 0x826152B0;
	sub_82466E20(ctx, base);
	// 826152B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826152B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826152B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826152BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826152C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826152C0 size=108
    let mut pc: u32 = 0x826152C0;
    'dispatch: loop {
        match pc {
            0x826152C0 => {
    //   block [0x826152C0..0x8261532C)
	// 826152C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826152C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826152C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826152CC: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826152D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826152D4: 38EB81F0  addi r7, r11, -0x7e10
	ctx.r[7].s64 = ctx.r[11].s64 + -32272;
	// 826152D8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826152DC: 388A3838  addi r4, r10, 0x3838
	ctx.r[4].s64 = ctx.r[10].s64 + 14392;
	// 826152E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826152E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826152E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826152EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826152F0: 386AB18C  addi r3, r10, -0x4e74
	ctx.r[3].s64 = ctx.r[10].s64 + -20084;
	// 826152F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826152F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826152FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82615300: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82615304: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82615308: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261530C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82615310: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82615314: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82615318: 4BE51B09  bl 0x82466e20
	ctx.lr = 0x8261531C;
	sub_82466E20(ctx, base);
	// 8261531C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82615320: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82615324: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82615328: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82615330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82615330 size=24
    let mut pc: u32 = 0x82615330;
    'dispatch: loop {
        match pc {
            0x82615330 => {
    //   block [0x82615330..0x82615348)
	// 82615330: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82615334: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82615338: 394ADA20  addi r10, r10, -0x25e0
	ctx.r[10].s64 = ctx.r[10].s64 + -9696;
	// 8261533C: 816B8730  lwz r11, -0x78d0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-30928 as u32) ) } as u64;
	// 82615340: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82615344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82615348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82615348 size=112
    let mut pc: u32 = 0x82615348;
    'dispatch: loop {
        match pc {
            0x82615348 => {
    //   block [0x82615348..0x826153B8)
	// 82615348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261534C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82615350: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82615354: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615358: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261535C: 38AAB39C  addi r5, r10, -0x4c64
	ctx.r[5].s64 = ctx.r[10].s64 + -19556;
	// 82615360: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82615364: 390BDA20  addi r8, r11, -0x25e0
	ctx.r[8].s64 = ctx.r[11].s64 + -9696;
	// 82615368: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8261536C: 388A3888  addi r4, r10, 0x3888
	ctx.r[4].s64 = ctx.r[10].s64 + 14472;
	// 82615370: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82615374: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615378: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261537C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82615380: 386AB1BC  addi r3, r10, -0x4e44
	ctx.r[3].s64 = ctx.r[10].s64 + -20036;
	// 82615384: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82615388: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261538C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82615390: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82615394: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82615398: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261539C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826153A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826153A4: 4BE51A7D  bl 0x82466e20
	ctx.lr = 0x826153A8;
	sub_82466E20(ctx, base);
	// 826153A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826153AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826153B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826153B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826153B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826153B8 size=108
    let mut pc: u32 = 0x826153B8;
    'dispatch: loop {
        match pc {
            0x826153B8 => {
    //   block [0x826153B8..0x82615424)
	// 826153B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826153BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826153C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826153C4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826153C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826153CC: 38EB8208  addi r7, r11, -0x7df8
	ctx.r[7].s64 = ctx.r[11].s64 + -32248;
	// 826153D0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826153D4: 388A38A0  addi r4, r10, 0x38a0
	ctx.r[4].s64 = ctx.r[10].s64 + 14496;
	// 826153D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826153DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826153E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826153E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826153E8: 386AB1EC  addi r3, r10, -0x4e14
	ctx.r[3].s64 = ctx.r[10].s64 + -19988;
	// 826153EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826153F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826153F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826153F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826153FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82615400: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82615404: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82615408: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261540C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82615410: 4BE51A11  bl 0x82466e20
	ctx.lr = 0x82615414;
	sub_82466E20(ctx, base);
	// 82615414: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82615418: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261541C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82615420: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82615428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82615428 size=112
    let mut pc: u32 = 0x82615428;
    'dispatch: loop {
        match pc {
            0x82615428 => {
    //   block [0x82615428..0x82615498)
	// 82615428: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261542C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82615430: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82615434: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615438: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261543C: 38AAB39C  addi r5, r10, -0x4c64
	ctx.r[5].s64 = ctx.r[10].s64 + -19556;
	// 82615440: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82615444: 390B8268  addi r8, r11, -0x7d98
	ctx.r[8].s64 = ctx.r[11].s64 + -32152;
	// 82615448: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8261544C: 388A38BC  addi r4, r10, 0x38bc
	ctx.r[4].s64 = ctx.r[10].s64 + 14524;
	// 82615450: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82615454: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615458: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261545C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82615460: 386AB21C  addi r3, r10, -0x4de4
	ctx.r[3].s64 = ctx.r[10].s64 + -19940;
	// 82615464: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82615468: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261546C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82615470: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82615474: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82615478: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261547C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82615480: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82615484: 4BE5199D  bl 0x82466e20
	ctx.lr = 0x82615488;
	sub_82466E20(ctx, base);
	// 82615488: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261548C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82615490: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82615494: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82615498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82615498 size=108
    let mut pc: u32 = 0x82615498;
    'dispatch: loop {
        match pc {
            0x82615498 => {
    //   block [0x82615498..0x82615504)
	// 82615498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261549C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826154A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826154A4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826154A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826154AC: 38EB8328  addi r7, r11, -0x7cd8
	ctx.r[7].s64 = ctx.r[11].s64 + -31960;
	// 826154B0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826154B4: 388A38D0  addi r4, r10, 0x38d0
	ctx.r[4].s64 = ctx.r[10].s64 + 14544;
	// 826154B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826154BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826154C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826154C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826154C8: 386AB24C  addi r3, r10, -0x4db4
	ctx.r[3].s64 = ctx.r[10].s64 + -19892;
	// 826154CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826154D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826154D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826154D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826154DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826154E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826154E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826154E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826154EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826154F0: 4BE51931  bl 0x82466e20
	ctx.lr = 0x826154F4;
	sub_82466E20(ctx, base);
	// 826154F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826154F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826154FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82615500: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82615508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82615508 size=108
    let mut pc: u32 = 0x82615508;
    'dispatch: loop {
        match pc {
            0x82615508 => {
    //   block [0x82615508..0x82615574)
	// 82615508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261550C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82615510: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82615514: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82615518: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261551C: 38EB83A0  addi r7, r11, -0x7c60
	ctx.r[7].s64 = ctx.r[11].s64 + -31840;
	// 82615520: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82615524: 388A38E0  addi r4, r10, 0x38e0
	ctx.r[4].s64 = ctx.r[10].s64 + 14560;
	// 82615528: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261552C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615530: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82615534: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82615538: 386AB27C  addi r3, r10, -0x4d84
	ctx.r[3].s64 = ctx.r[10].s64 + -19844;
	// 8261553C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82615540: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82615544: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82615548: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261554C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82615550: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82615554: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82615558: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261555C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82615560: 4BE518C1  bl 0x82466e20
	ctx.lr = 0x82615564;
	sub_82466E20(ctx, base);
	// 82615564: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82615568: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261556C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82615570: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82615578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82615578 size=108
    let mut pc: u32 = 0x82615578;
    'dispatch: loop {
        match pc {
            0x82615578 => {
    //   block [0x82615578..0x826155E4)
	// 82615578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261557C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82615580: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82615584: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82615588: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261558C: 38EB83E8  addi r7, r11, -0x7c18
	ctx.r[7].s64 = ctx.r[11].s64 + -31768;
	// 82615590: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82615594: 388A3904  addi r4, r10, 0x3904
	ctx.r[4].s64 = ctx.r[10].s64 + 14596;
	// 82615598: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261559C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826155A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826155A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826155A8: 386AB2AC  addi r3, r10, -0x4d54
	ctx.r[3].s64 = ctx.r[10].s64 + -19796;
	// 826155AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826155B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826155B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826155B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826155BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826155C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826155C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826155C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826155CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826155D0: 4BE51851  bl 0x82466e20
	ctx.lr = 0x826155D4;
	sub_82466E20(ctx, base);
	// 826155D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826155D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826155DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826155E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826155E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826155E8 size=112
    let mut pc: u32 = 0x826155E8;
    'dispatch: loop {
        match pc {
            0x826155E8 => {
    //   block [0x826155E8..0x82615658)
	// 826155E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826155EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826155F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826155F4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826155F8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826155FC: 38AAB2AC  addi r5, r10, -0x4d54
	ctx.r[5].s64 = ctx.r[10].s64 + -19796;
	// 82615600: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82615604: 390B8430  addi r8, r11, -0x7bd0
	ctx.r[8].s64 = ctx.r[11].s64 + -31696;
	// 82615608: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8261560C: 388A3928  addi r4, r10, 0x3928
	ctx.r[4].s64 = ctx.r[10].s64 + 14632;
	// 82615610: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82615614: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615618: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261561C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82615620: 386AB2DC  addi r3, r10, -0x4d24
	ctx.r[3].s64 = ctx.r[10].s64 + -19748;
	// 82615624: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82615628: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261562C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82615630: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82615634: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82615638: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261563C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82615640: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82615644: 4BE517DD  bl 0x82466e20
	ctx.lr = 0x82615648;
	sub_82466E20(ctx, base);
	// 82615648: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261564C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82615650: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82615654: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82615658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82615658 size=108
    let mut pc: u32 = 0x82615658;
    'dispatch: loop {
        match pc {
            0x82615658 => {
    //   block [0x82615658..0x826156C4)
	// 82615658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261565C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82615660: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82615664: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82615668: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261566C: 38EB84A8  addi r7, r11, -0x7b58
	ctx.r[7].s64 = ctx.r[11].s64 + -31576;
	// 82615670: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82615674: 388A394C  addi r4, r10, 0x394c
	ctx.r[4].s64 = ctx.r[10].s64 + 14668;
	// 82615678: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261567C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615680: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82615684: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82615688: 386AB30C  addi r3, r10, -0x4cf4
	ctx.r[3].s64 = ctx.r[10].s64 + -19700;
	// 8261568C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82615690: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82615694: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82615698: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261569C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826156A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826156A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826156A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826156AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826156B0: 4BE51771  bl 0x82466e20
	ctx.lr = 0x826156B4;
	sub_82466E20(ctx, base);
	// 826156B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826156B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826156BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826156C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826156C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826156C8 size=108
    let mut pc: u32 = 0x826156C8;
    'dispatch: loop {
        match pc {
            0x826156C8 => {
    //   block [0x826156C8..0x82615734)
	// 826156C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826156CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826156D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826156D4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826156D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826156DC: 38EB84F0  addi r7, r11, -0x7b10
	ctx.r[7].s64 = ctx.r[11].s64 + -31504;
	// 826156E0: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826156E4: 388A3968  addi r4, r10, 0x3968
	ctx.r[4].s64 = ctx.r[10].s64 + 14696;
	// 826156E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826156EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826156F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826156F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826156F8: 386AB33C  addi r3, r10, -0x4cc4
	ctx.r[3].s64 = ctx.r[10].s64 + -19652;
	// 826156FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82615700: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82615704: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82615708: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261570C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82615710: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82615714: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82615718: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261571C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82615720: 4BE51701  bl 0x82466e20
	ctx.lr = 0x82615724;
	sub_82466E20(ctx, base);
	// 82615724: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82615728: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261572C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82615730: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82615738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82615738 size=108
    let mut pc: u32 = 0x82615738;
    'dispatch: loop {
        match pc {
            0x82615738 => {
    //   block [0x82615738..0x826157A4)
	// 82615738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261573C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82615740: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82615744: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82615748: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261574C: 38EB85B0  addi r7, r11, -0x7a50
	ctx.r[7].s64 = ctx.r[11].s64 + -31312;
	// 82615750: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 82615754: 388A3980  addi r4, r10, 0x3980
	ctx.r[4].s64 = ctx.r[10].s64 + 14720;
	// 82615758: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261575C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615760: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82615764: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82615768: 386AB36C  addi r3, r10, -0x4c94
	ctx.r[3].s64 = ctx.r[10].s64 + -19604;
	// 8261576C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82615770: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82615774: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82615778: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261577C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82615780: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82615784: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82615788: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261578C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82615790: 4BE51691  bl 0x82466e20
	ctx.lr = 0x82615794;
	sub_82466E20(ctx, base);
	// 82615794: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82615798: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261579C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826157A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826157A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826157A8 size=112
    let mut pc: u32 = 0x826157A8;
    'dispatch: loop {
        match pc {
            0x826157A8 => {
    //   block [0x826157A8..0x82615818)
	// 826157A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826157AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826157B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826157B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826157B8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826157BC: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 826157C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826157C4: 390B8738  addi r8, r11, -0x78c8
	ctx.r[8].s64 = ctx.r[11].s64 + -30920;
	// 826157C8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826157CC: 388A3990  addi r4, r10, 0x3990
	ctx.r[4].s64 = ctx.r[10].s64 + 14736;
	// 826157D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826157D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826157D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826157DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826157E0: 386AB39C  addi r3, r10, -0x4c64
	ctx.r[3].s64 = ctx.r[10].s64 + -19556;
	// 826157E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826157E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826157EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826157F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826157F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826157F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826157FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82615800: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82615804: 4BE5161D  bl 0x82466e20
	ctx.lr = 0x82615808;
	sub_82466E20(ctx, base);
	// 82615808: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261580C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82615810: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82615814: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82615818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82615818 size=108
    let mut pc: u32 = 0x82615818;
    'dispatch: loop {
        match pc {
            0x82615818 => {
    //   block [0x82615818..0x82615884)
	// 82615818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261581C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82615820: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82615824: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82615828: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261582C: 38EB8798  addi r7, r11, -0x7868
	ctx.r[7].s64 = ctx.r[11].s64 + -30824;
	// 82615830: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82615834: 388A39A0  addi r4, r10, 0x39a0
	ctx.r[4].s64 = ctx.r[10].s64 + 14752;
	// 82615838: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261583C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615840: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82615844: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82615848: 386AB3CC  addi r3, r10, -0x4c34
	ctx.r[3].s64 = ctx.r[10].s64 + -19508;
	// 8261584C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82615850: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82615854: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82615858: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261585C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82615860: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82615864: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82615868: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261586C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82615870: 4BE515B1  bl 0x82466e20
	ctx.lr = 0x82615874;
	sub_82466E20(ctx, base);
	// 82615874: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82615878: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261587C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82615880: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82615888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82615888 size=112
    let mut pc: u32 = 0x82615888;
    'dispatch: loop {
        match pc {
            0x82615888 => {
    //   block [0x82615888..0x826158F8)
	// 82615888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261588C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82615890: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82615894: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615898: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261589C: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 826158A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826158A4: 390B8810  addi r8, r11, -0x77f0
	ctx.r[8].s64 = ctx.r[11].s64 + -30704;
	// 826158A8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826158AC: 388A39B4  addi r4, r10, 0x39b4
	ctx.r[4].s64 = ctx.r[10].s64 + 14772;
	// 826158B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826158B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826158B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826158BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826158C0: 386AB3FC  addi r3, r10, -0x4c04
	ctx.r[3].s64 = ctx.r[10].s64 + -19460;
	// 826158C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826158C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826158CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826158D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826158D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826158D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826158DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826158E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826158E4: 4BE5153D  bl 0x82466e20
	ctx.lr = 0x826158E8;
	sub_82466E20(ctx, base);
	// 826158E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826158EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826158F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826158F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826158F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826158F8 size=108
    let mut pc: u32 = 0x826158F8;
    'dispatch: loop {
        match pc {
            0x826158F8 => {
    //   block [0x826158F8..0x82615964)
	// 826158F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826158FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82615900: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82615904: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82615908: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261590C: 38EB8858  addi r7, r11, -0x77a8
	ctx.r[7].s64 = ctx.r[11].s64 + -30632;
	// 82615910: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82615914: 388A39C0  addi r4, r10, 0x39c0
	ctx.r[4].s64 = ctx.r[10].s64 + 14784;
	// 82615918: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261591C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615920: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82615924: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82615928: 386AB42C  addi r3, r10, -0x4bd4
	ctx.r[3].s64 = ctx.r[10].s64 + -19412;
	// 8261592C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82615930: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82615934: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82615938: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261593C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82615940: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82615944: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82615948: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261594C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82615950: 4BE514D1  bl 0x82466e20
	ctx.lr = 0x82615954;
	sub_82466E20(ctx, base);
	// 82615954: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82615958: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261595C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82615960: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82615968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82615968 size=108
    let mut pc: u32 = 0x82615968;
    'dispatch: loop {
        match pc {
            0x82615968 => {
    //   block [0x82615968..0x826159D4)
	// 82615968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261596C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82615970: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82615974: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82615978: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8261597C: 38EB88B8  addi r7, r11, -0x7748
	ctx.r[7].s64 = ctx.r[11].s64 + -30536;
	// 82615980: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82615984: 388A0DBC  addi r4, r10, 0xdbc
	ctx.r[4].s64 = ctx.r[10].s64 + 3516;
	// 82615988: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261598C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615990: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82615994: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82615998: 386AB45C  addi r3, r10, -0x4ba4
	ctx.r[3].s64 = ctx.r[10].s64 + -19364;
	// 8261599C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826159A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826159A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826159A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826159AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826159B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826159B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826159B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826159BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826159C0: 4BE51461  bl 0x82466e20
	ctx.lr = 0x826159C4;
	sub_82466E20(ctx, base);
	// 826159C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826159C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826159CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826159D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826159D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826159D8 size=108
    let mut pc: u32 = 0x826159D8;
    'dispatch: loop {
        match pc {
            0x826159D8 => {
    //   block [0x826159D8..0x82615A44)
	// 826159D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826159DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826159E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826159E4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826159E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826159EC: 38EB8900  addi r7, r11, -0x7700
	ctx.r[7].s64 = ctx.r[11].s64 + -30464;
	// 826159F0: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826159F4: 388A39D8  addi r4, r10, 0x39d8
	ctx.r[4].s64 = ctx.r[10].s64 + 14808;
	// 826159F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826159FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615A00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82615A04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82615A08: 386AB48C  addi r3, r10, -0x4b74
	ctx.r[3].s64 = ctx.r[10].s64 + -19316;
	// 82615A0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82615A10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82615A14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82615A18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82615A1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82615A20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82615A24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82615A28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82615A2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82615A30: 4BE513F1  bl 0x82466e20
	ctx.lr = 0x82615A34;
	sub_82466E20(ctx, base);
	// 82615A34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82615A38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82615A3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82615A40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82615A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82615A48 size=108
    let mut pc: u32 = 0x82615A48;
    'dispatch: loop {
        match pc {
            0x82615A48 => {
    //   block [0x82615A48..0x82615AB4)
	// 82615A48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82615A4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82615A50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82615A54: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82615A58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82615A5C: 38EB89C0  addi r7, r11, -0x7640
	ctx.r[7].s64 = ctx.r[11].s64 + -30272;
	// 82615A60: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82615A64: 388A39F4  addi r4, r10, 0x39f4
	ctx.r[4].s64 = ctx.r[10].s64 + 14836;
	// 82615A68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82615A6C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615A70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82615A74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82615A78: 386AB4BC  addi r3, r10, -0x4b44
	ctx.r[3].s64 = ctx.r[10].s64 + -19268;
	// 82615A7C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82615A80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82615A84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82615A88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82615A8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82615A90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82615A94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82615A98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82615A9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82615AA0: 4BE51381  bl 0x82466e20
	ctx.lr = 0x82615AA4;
	sub_82466E20(ctx, base);
	// 82615AA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82615AA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82615AAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82615AB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82615AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82615AB8 size=108
    let mut pc: u32 = 0x82615AB8;
    'dispatch: loop {
        match pc {
            0x82615AB8 => {
    //   block [0x82615AB8..0x82615B24)
	// 82615AB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82615ABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82615AC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82615AC4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82615AC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82615ACC: 38EB8A50  addi r7, r11, -0x75b0
	ctx.r[7].s64 = ctx.r[11].s64 + -30128;
	// 82615AD0: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 82615AD4: 388A3A14  addi r4, r10, 0x3a14
	ctx.r[4].s64 = ctx.r[10].s64 + 14868;
	// 82615AD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82615ADC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615AE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82615AE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82615AE8: 386AB4EC  addi r3, r10, -0x4b14
	ctx.r[3].s64 = ctx.r[10].s64 + -19220;
	// 82615AEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82615AF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82615AF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82615AF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82615AFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82615B00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82615B04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82615B08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82615B0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82615B10: 4BE51311  bl 0x82466e20
	ctx.lr = 0x82615B14;
	sub_82466E20(ctx, base);
	// 82615B14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82615B18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82615B1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82615B20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82615B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82615B28 size=108
    let mut pc: u32 = 0x82615B28;
    'dispatch: loop {
        match pc {
            0x82615B28 => {
    //   block [0x82615B28..0x82615B94)
	// 82615B28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82615B2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82615B30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82615B34: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82615B38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82615B3C: 38EB8B88  addi r7, r11, -0x7478
	ctx.r[7].s64 = ctx.r[11].s64 + -29816;
	// 82615B40: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82615B44: 388A3A30  addi r4, r10, 0x3a30
	ctx.r[4].s64 = ctx.r[10].s64 + 14896;
	// 82615B48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82615B4C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615B50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82615B54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82615B58: 386AB51C  addi r3, r10, -0x4ae4
	ctx.r[3].s64 = ctx.r[10].s64 + -19172;
	// 82615B5C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82615B60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82615B64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82615B68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82615B6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82615B70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82615B74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82615B78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82615B7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82615B80: 4BE512A1  bl 0x82466e20
	ctx.lr = 0x82615B84;
	sub_82466E20(ctx, base);
	// 82615B84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82615B88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82615B8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82615B90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82615B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82615B98 size=116
    let mut pc: u32 = 0x82615B98;
    'dispatch: loop {
        match pc {
            0x82615B98 => {
    //   block [0x82615B98..0x82615C0C)
	// 82615B98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82615B9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82615BA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82615BA4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82615BA8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82615BAC: 390B8BE8  addi r8, r11, -0x7418
	ctx.r[8].s64 = ctx.r[11].s64 + -29720;
	// 82615BB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82615BB4: 392AF894  addi r9, r10, -0x76c
	ctx.r[9].s64 = ctx.r[10].s64 + -1900;
	// 82615BB8: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615BBC: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82615BC0: 38AAB51C  addi r5, r10, -0x4ae4
	ctx.r[5].s64 = ctx.r[10].s64 + -19172;
	// 82615BC4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82615BC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82615BCC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82615BD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82615BD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82615BD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82615BDC: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82615BE0: 388A3A3C  addi r4, r10, 0x3a3c
	ctx.r[4].s64 = ctx.r[10].s64 + 14908;
	// 82615BE4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82615BE8: 386BB54C  addi r3, r11, -0x4ab4
	ctx.r[3].s64 = ctx.r[11].s64 + -19124;
	// 82615BEC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82615BF0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82615BF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82615BF8: 4BE51229  bl 0x82466e20
	ctx.lr = 0x82615BFC;
	sub_82466E20(ctx, base);
	// 82615BFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82615C00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82615C04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82615C08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82615C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82615C10 size=96
    let mut pc: u32 = 0x82615C10;
    'dispatch: loop {
        match pc {
            0x82615C10 => {
    //   block [0x82615C10..0x82615C70)
	// 82615C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82615C14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82615C18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82615C1C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82615C20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82615C24: 388A3A4C  addi r4, r10, 0x3a4c
	ctx.r[4].s64 = ctx.r[10].s64 + 14924;
	// 82615C28: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615C2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82615C30: 386AB57C  addi r3, r10, -0x4a84
	ctx.r[3].s64 = ctx.r[10].s64 + -19076;
	// 82615C34: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82615C38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82615C3C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82615C40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82615C44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82615C48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82615C4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82615C50: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82615C54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82615C58: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82615C5C: 4BE511C5  bl 0x82466e20
	ctx.lr = 0x82615C60;
	sub_82466E20(ctx, base);
	// 82615C60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82615C64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82615C68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82615C6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82615C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82615C70 size=112
    let mut pc: u32 = 0x82615C70;
    'dispatch: loop {
        match pc {
            0x82615C70 => {
    //   block [0x82615C70..0x82615CE0)
	// 82615C70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82615C74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82615C78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82615C7C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615C80: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82615C84: 38AAD70C  addi r5, r10, -0x28f4
	ctx.r[5].s64 = ctx.r[10].s64 + -10484;
	// 82615C88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82615C8C: 390B8C60  addi r8, r11, -0x73a0
	ctx.r[8].s64 = ctx.r[11].s64 + -29600;
	// 82615C90: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82615C94: 388A3A64  addi r4, r10, 0x3a64
	ctx.r[4].s64 = ctx.r[10].s64 + 14948;
	// 82615C98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82615C9C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615CA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82615CA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82615CA8: 386AB5AC  addi r3, r10, -0x4a54
	ctx.r[3].s64 = ctx.r[10].s64 + -19028;
	// 82615CAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82615CB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82615CB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82615CB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82615CBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82615CC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82615CC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82615CC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82615CCC: 4BE51155  bl 0x82466e20
	ctx.lr = 0x82615CD0;
	sub_82466E20(ctx, base);
	// 82615CD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82615CD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82615CD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82615CDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82615CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82615CE0 size=96
    let mut pc: u32 = 0x82615CE0;
    'dispatch: loop {
        match pc {
            0x82615CE0 => {
    //   block [0x82615CE0..0x82615D40)
	// 82615CE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82615CE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82615CE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82615CEC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82615CF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82615CF4: 388A3A7C  addi r4, r10, 0x3a7c
	ctx.r[4].s64 = ctx.r[10].s64 + 14972;
	// 82615CF8: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615CFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82615D00: 386AB5DC  addi r3, r10, -0x4a24
	ctx.r[3].s64 = ctx.r[10].s64 + -18980;
	// 82615D04: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82615D08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82615D0C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82615D10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82615D14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82615D18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82615D1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82615D20: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82615D24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82615D28: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82615D2C: 4BE510F5  bl 0x82466e20
	ctx.lr = 0x82615D30;
	sub_82466E20(ctx, base);
	// 82615D30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82615D34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82615D38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82615D3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82615D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82615D40 size=24
    let mut pc: u32 = 0x82615D40;
    'dispatch: loop {
        match pc {
            0x82615D40 => {
    //   block [0x82615D40..0x82615D58)
	// 82615D40: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82615D44: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82615D48: 394ADAE0  addi r10, r10, -0x2520
	ctx.r[10].s64 = ctx.r[10].s64 + -9504;
	// 82615D4C: 816B8CC0  lwz r11, -0x7340(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29504 as u32) ) } as u64;
	// 82615D50: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82615D54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82615D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82615D58 size=116
    let mut pc: u32 = 0x82615D58;
    'dispatch: loop {
        match pc {
            0x82615D58 => {
    //   block [0x82615D58..0x82615DCC)
	// 82615D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82615D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82615D60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82615D64: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82615D68: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82615D6C: 390BDAE0  addi r8, r11, -0x2520
	ctx.r[8].s64 = ctx.r[11].s64 + -9504;
	// 82615D70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82615D74: 392AF8D0  addi r9, r10, -0x730
	ctx.r[9].s64 = ctx.r[10].s64 + -1840;
	// 82615D78: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615D7C: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82615D80: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 82615D84: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82615D88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82615D8C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82615D90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82615D94: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82615D98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82615D9C: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82615DA0: 388A3A9C  addi r4, r10, 0x3a9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15004;
	// 82615DA4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82615DA8: 386BB60C  addi r3, r11, -0x49f4
	ctx.r[3].s64 = ctx.r[11].s64 + -18932;
	// 82615DAC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82615DB0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82615DB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82615DB8: 4BE51069  bl 0x82466e20
	ctx.lr = 0x82615DBC;
	sub_82466E20(ctx, base);
	// 82615DBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82615DC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82615DC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82615DC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82615DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82615DD0 size=104
    let mut pc: u32 = 0x82615DD0;
    'dispatch: loop {
        match pc {
            0x82615DD0 => {
    //   block [0x82615DD0..0x82615E38)
	// 82615DD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82615DD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82615DD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82615DDC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82615DE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82615DE4: 392AF8FC  addi r9, r10, -0x704
	ctx.r[9].s64 = ctx.r[10].s64 + -1796;
	// 82615DE8: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615DEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82615DF0: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 82615DF4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82615DF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82615DFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82615E00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82615E04: 388A3AB0  addi r4, r10, 0x3ab0
	ctx.r[4].s64 = ctx.r[10].s64 + 15024;
	// 82615E08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82615E0C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615E10: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82615E14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82615E18: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82615E1C: 386AB63C  addi r3, r10, -0x49c4
	ctx.r[3].s64 = ctx.r[10].s64 + -18884;
	// 82615E20: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82615E24: 4BE50FFD  bl 0x82466e20
	ctx.lr = 0x82615E28;
	sub_82466E20(ctx, base);
	// 82615E28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82615E2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82615E30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82615E34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82615E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82615E38 size=96
    let mut pc: u32 = 0x82615E38;
    'dispatch: loop {
        match pc {
            0x82615E38 => {
    //   block [0x82615E38..0x82615E98)
	// 82615E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82615E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82615E40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82615E44: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82615E48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82615E4C: 388A3AC4  addi r4, r10, 0x3ac4
	ctx.r[4].s64 = ctx.r[10].s64 + 15044;
	// 82615E50: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615E54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82615E58: 386AB66C  addi r3, r10, -0x4994
	ctx.r[3].s64 = ctx.r[10].s64 + -18836;
	// 82615E5C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82615E60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82615E64: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82615E68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82615E6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82615E70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82615E74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82615E78: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82615E7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82615E80: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82615E84: 4BE50F9D  bl 0x82466e20
	ctx.lr = 0x82615E88;
	sub_82466E20(ctx, base);
	// 82615E88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82615E8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82615E90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82615E94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82615E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82615E98 size=100
    let mut pc: u32 = 0x82615E98;
    'dispatch: loop {
        match pc {
            0x82615E98 => {
    //   block [0x82615E98..0x82615EFC)
	// 82615E98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82615E9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82615EA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82615EA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615EA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82615EAC: 38AAB63C  addi r5, r10, -0x49c4
	ctx.r[5].s64 = ctx.r[10].s64 + -18884;
	// 82615EB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82615EB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82615EB8: 388A3ADC  addi r4, r10, 0x3adc
	ctx.r[4].s64 = ctx.r[10].s64 + 15068;
	// 82615EBC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615EC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82615EC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82615EC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82615ECC: 386AB69C  addi r3, r10, -0x4964
	ctx.r[3].s64 = ctx.r[10].s64 + -18788;
	// 82615ED0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82615ED4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82615ED8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82615EDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82615EE0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82615EE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82615EE8: 4BE50F39  bl 0x82466e20
	ctx.lr = 0x82615EEC;
	sub_82466E20(ctx, base);
	// 82615EEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82615EF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82615EF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82615EF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82615F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82615F00 size=112
    let mut pc: u32 = 0x82615F00;
    'dispatch: loop {
        match pc {
            0x82615F00 => {
    //   block [0x82615F00..0x82615F70)
	// 82615F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82615F04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82615F08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82615F0C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615F10: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82615F14: 38AAB60C  addi r5, r10, -0x49f4
	ctx.r[5].s64 = ctx.r[10].s64 + -18932;
	// 82615F18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82615F1C: 390B8CC8  addi r8, r11, -0x7338
	ctx.r[8].s64 = ctx.r[11].s64 + -29496;
	// 82615F20: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82615F24: 388A3AF8  addi r4, r10, 0x3af8
	ctx.r[4].s64 = ctx.r[10].s64 + 15096;
	// 82615F28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82615F2C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615F30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82615F34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82615F38: 386AB6CC  addi r3, r10, -0x4934
	ctx.r[3].s64 = ctx.r[10].s64 + -18740;
	// 82615F3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82615F40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82615F44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82615F48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82615F4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82615F50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82615F54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82615F58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82615F5C: 4BE50EC5  bl 0x82466e20
	ctx.lr = 0x82615F60;
	sub_82466E20(ctx, base);
	// 82615F60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82615F64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82615F68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82615F6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82615F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82615F70 size=112
    let mut pc: u32 = 0x82615F70;
    'dispatch: loop {
        match pc {
            0x82615F70 => {
    //   block [0x82615F70..0x82615FE0)
	// 82615F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82615F74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82615F78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82615F7C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615F80: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82615F84: 38AAB60C  addi r5, r10, -0x49f4
	ctx.r[5].s64 = ctx.r[10].s64 + -18932;
	// 82615F88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82615F8C: 390B8D10  addi r8, r11, -0x72f0
	ctx.r[8].s64 = ctx.r[11].s64 + -29424;
	// 82615F90: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82615F94: 388A3B08  addi r4, r10, 0x3b08
	ctx.r[4].s64 = ctx.r[10].s64 + 15112;
	// 82615F98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82615F9C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615FA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82615FA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82615FA8: 386AB6FC  addi r3, r10, -0x4904
	ctx.r[3].s64 = ctx.r[10].s64 + -18692;
	// 82615FAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82615FB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82615FB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82615FB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82615FBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82615FC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82615FC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82615FC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82615FCC: 4BE50E55  bl 0x82466e20
	ctx.lr = 0x82615FD0;
	sub_82466E20(ctx, base);
	// 82615FD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82615FD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82615FD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82615FDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


