pub fn sub_82679908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82679908 size=108
    let mut pc: u32 = 0x82679908;
    'dispatch: loop {
        match pc {
            0x82679908 => {
    //   block [0x82679908..0x82679974)
	// 82679908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267990C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82679910: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82679914: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82679918: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267991C: 38EB7318  addi r7, r11, 0x7318
	ctx.r[7].s64 = ctx.r[11].s64 + 29464;
	// 82679920: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82679924: 388A2BC8  addi r4, r10, 0x2bc8
	ctx.r[4].s64 = ctx.r[10].s64 + 11208;
	// 82679928: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267992C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82679930: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82679934: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82679938: 386A5958  addi r3, r10, 0x5958
	ctx.r[3].s64 = ctx.r[10].s64 + 22872;
	// 8267993C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82679940: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82679944: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82679948: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267994C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82679950: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82679954: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82679958: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267995C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82679960: 4BDED4C1  bl 0x82466e20
	ctx.lr = 0x82679964;
	sub_82466E20(ctx, base);
	// 82679964: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82679968: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267996C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82679970: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82679978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82679978 size=112
    let mut pc: u32 = 0x82679978;
    'dispatch: loop {
        match pc {
            0x82679978 => {
    //   block [0x82679978..0x826799E8)
	// 82679978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267997C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82679980: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82679984: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82679988: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267998C: 38AA5928  addi r5, r10, 0x5928
	ctx.r[5].s64 = ctx.r[10].s64 + 22824;
	// 82679990: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82679994: 390B73A8  addi r8, r11, 0x73a8
	ctx.r[8].s64 = ctx.r[11].s64 + 29608;
	// 82679998: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 8267999C: 388A2C00  addi r4, r10, 0x2c00
	ctx.r[4].s64 = ctx.r[10].s64 + 11264;
	// 826799A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826799A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826799A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826799AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826799B0: 386A5988  addi r3, r10, 0x5988
	ctx.r[3].s64 = ctx.r[10].s64 + 22920;
	// 826799B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826799B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826799BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826799C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826799C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826799C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826799CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826799D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826799D4: 4BDED44D  bl 0x82466e20
	ctx.lr = 0x826799D8;
	sub_82466E20(ctx, base);
	// 826799D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826799DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826799E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826799E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826799E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826799E8 size=112
    let mut pc: u32 = 0x826799E8;
    'dispatch: loop {
        match pc {
            0x826799E8 => {
    //   block [0x826799E8..0x82679A58)
	// 826799E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826799EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826799F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826799F4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826799F8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826799FC: 38AA5928  addi r5, r10, 0x5928
	ctx.r[5].s64 = ctx.r[10].s64 + 22824;
	// 82679A00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82679A04: 390B74C8  addi r8, r11, 0x74c8
	ctx.r[8].s64 = ctx.r[11].s64 + 29896;
	// 82679A08: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82679A0C: 388A2C24  addi r4, r10, 0x2c24
	ctx.r[4].s64 = ctx.r[10].s64 + 11300;
	// 82679A10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82679A14: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82679A18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82679A1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82679A20: 386A59B8  addi r3, r10, 0x59b8
	ctx.r[3].s64 = ctx.r[10].s64 + 22968;
	// 82679A24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82679A28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82679A2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82679A30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82679A34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82679A38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82679A3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82679A40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82679A44: 4BDED3DD  bl 0x82466e20
	ctx.lr = 0x82679A48;
	sub_82466E20(ctx, base);
	// 82679A48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82679A4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82679A50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82679A54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82679A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82679A58 size=108
    let mut pc: u32 = 0x82679A58;
    'dispatch: loop {
        match pc {
            0x82679A58 => {
    //   block [0x82679A58..0x82679AC4)
	// 82679A58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82679A5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82679A60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82679A64: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82679A68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82679A6C: 38EB74E0  addi r7, r11, 0x74e0
	ctx.r[7].s64 = ctx.r[11].s64 + 29920;
	// 82679A70: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82679A74: 388A2C44  addi r4, r10, 0x2c44
	ctx.r[4].s64 = ctx.r[10].s64 + 11332;
	// 82679A78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82679A7C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82679A80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82679A84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82679A88: 386A59E8  addi r3, r10, 0x59e8
	ctx.r[3].s64 = ctx.r[10].s64 + 23016;
	// 82679A8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82679A90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82679A94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82679A98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82679A9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82679AA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82679AA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82679AA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82679AAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82679AB0: 4BDED371  bl 0x82466e20
	ctx.lr = 0x82679AB4;
	sub_82466E20(ctx, base);
	// 82679AB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82679AB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82679ABC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82679AC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82679AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82679AC8 size=112
    let mut pc: u32 = 0x82679AC8;
    'dispatch: loop {
        match pc {
            0x82679AC8 => {
    //   block [0x82679AC8..0x82679B38)
	// 82679AC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82679ACC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82679AD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82679AD4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82679AD8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82679ADC: 38AA5928  addi r5, r10, 0x5928
	ctx.r[5].s64 = ctx.r[10].s64 + 22824;
	// 82679AE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82679AE4: 390B7570  addi r8, r11, 0x7570
	ctx.r[8].s64 = ctx.r[11].s64 + 30064;
	// 82679AE8: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 82679AEC: 388A2C74  addi r4, r10, 0x2c74
	ctx.r[4].s64 = ctx.r[10].s64 + 11380;
	// 82679AF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82679AF4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82679AF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82679AFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82679B00: 386A5A18  addi r3, r10, 0x5a18
	ctx.r[3].s64 = ctx.r[10].s64 + 23064;
	// 82679B04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82679B08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82679B0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82679B10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82679B14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82679B18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82679B1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82679B20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82679B24: 4BDED2FD  bl 0x82466e20
	ctx.lr = 0x82679B28;
	sub_82466E20(ctx, base);
	// 82679B28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82679B2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82679B30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82679B34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82679B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82679B38 size=108
    let mut pc: u32 = 0x82679B38;
    'dispatch: loop {
        match pc {
            0x82679B38 => {
    //   block [0x82679B38..0x82679BA4)
	// 82679B38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82679B3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82679B40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82679B44: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82679B48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82679B4C: 38EB7660  addi r7, r11, 0x7660
	ctx.r[7].s64 = ctx.r[11].s64 + 30304;
	// 82679B50: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82679B54: 388A2C90  addi r4, r10, 0x2c90
	ctx.r[4].s64 = ctx.r[10].s64 + 11408;
	// 82679B58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82679B5C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82679B60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82679B64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82679B68: 386A5A48  addi r3, r10, 0x5a48
	ctx.r[3].s64 = ctx.r[10].s64 + 23112;
	// 82679B6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82679B70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82679B74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82679B78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82679B7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82679B80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82679B84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82679B88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82679B8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82679B90: 4BDED291  bl 0x82466e20
	ctx.lr = 0x82679B94;
	sub_82466E20(ctx, base);
	// 82679B94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82679B98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82679B9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82679BA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82679BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82679BA8 size=108
    let mut pc: u32 = 0x82679BA8;
    'dispatch: loop {
        match pc {
            0x82679BA8 => {
    //   block [0x82679BA8..0x82679C14)
	// 82679BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82679BAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82679BB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82679BB4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82679BB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82679BBC: 38EB7678  addi r7, r11, 0x7678
	ctx.r[7].s64 = ctx.r[11].s64 + 30328;
	// 82679BC0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82679BC4: 388A2CA8  addi r4, r10, 0x2ca8
	ctx.r[4].s64 = ctx.r[10].s64 + 11432;
	// 82679BC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82679BCC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82679BD0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82679BD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82679BD8: 386A5A78  addi r3, r10, 0x5a78
	ctx.r[3].s64 = ctx.r[10].s64 + 23160;
	// 82679BDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82679BE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82679BE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82679BE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82679BEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82679BF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82679BF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82679BF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82679BFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82679C00: 4BDED221  bl 0x82466e20
	ctx.lr = 0x82679C04;
	sub_82466E20(ctx, base);
	// 82679C04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82679C08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82679C0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82679C10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82679C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82679C18 size=116
    let mut pc: u32 = 0x82679C18;
    'dispatch: loop {
        match pc {
            0x82679C18 => {
    //   block [0x82679C18..0x82679C8C)
	// 82679C18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82679C1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82679C20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82679C24: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82679C28: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82679C2C: 390B76DC  addi r8, r11, 0x76dc
	ctx.r[8].s64 = ctx.r[11].s64 + 30428;
	// 82679C30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82679C34: 392A3C44  addi r9, r10, 0x3c44
	ctx.r[9].s64 = ctx.r[10].s64 + 15428;
	// 82679C38: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82679C3C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82679C40: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 82679C44: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82679C48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82679C4C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82679C50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82679C54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82679C58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82679C5C: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 82679C60: 388A2CB8  addi r4, r10, 0x2cb8
	ctx.r[4].s64 = ctx.r[10].s64 + 11448;
	// 82679C64: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82679C68: 386B5AA8  addi r3, r11, 0x5aa8
	ctx.r[3].s64 = ctx.r[11].s64 + 23208;
	// 82679C6C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82679C70: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82679C74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82679C78: 4BDED1A9  bl 0x82466e20
	ctx.lr = 0x82679C7C;
	sub_82466E20(ctx, base);
	// 82679C7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82679C80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82679C84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82679C88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82679C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82679C90 size=108
    let mut pc: u32 = 0x82679C90;
    'dispatch: loop {
        match pc {
            0x82679C90 => {
    //   block [0x82679C90..0x82679CFC)
	// 82679C90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82679C94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82679C98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82679C9C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82679CA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82679CA4: 38EB76F8  addi r7, r11, 0x76f8
	ctx.r[7].s64 = ctx.r[11].s64 + 30456;
	// 82679CA8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82679CAC: 388A2CCC  addi r4, r10, 0x2ccc
	ctx.r[4].s64 = ctx.r[10].s64 + 11468;
	// 82679CB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82679CB4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82679CB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82679CBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82679CC0: 386A5AD8  addi r3, r10, 0x5ad8
	ctx.r[3].s64 = ctx.r[10].s64 + 23256;
	// 82679CC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82679CC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82679CCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82679CD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82679CD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82679CD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82679CDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82679CE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82679CE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82679CE8: 4BDED139  bl 0x82466e20
	ctx.lr = 0x82679CEC;
	sub_82466E20(ctx, base);
	// 82679CEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82679CF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82679CF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82679CF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82679D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82679D00 size=108
    let mut pc: u32 = 0x82679D00;
    'dispatch: loop {
        match pc {
            0x82679D00 => {
    //   block [0x82679D00..0x82679D6C)
	// 82679D00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82679D04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82679D08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82679D0C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82679D10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82679D14: 38EB7740  addi r7, r11, 0x7740
	ctx.r[7].s64 = ctx.r[11].s64 + 30528;
	// 82679D18: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82679D1C: 388A2CF0  addi r4, r10, 0x2cf0
	ctx.r[4].s64 = ctx.r[10].s64 + 11504;
	// 82679D20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82679D24: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82679D28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82679D2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82679D30: 386A5B08  addi r3, r10, 0x5b08
	ctx.r[3].s64 = ctx.r[10].s64 + 23304;
	// 82679D34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82679D38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82679D3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82679D40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82679D44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82679D48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82679D4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82679D50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82679D54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82679D58: 4BDED0C9  bl 0x82466e20
	ctx.lr = 0x82679D5C;
	sub_82466E20(ctx, base);
	// 82679D5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82679D60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82679D64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82679D68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82679D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82679D70 size=108
    let mut pc: u32 = 0x82679D70;
    'dispatch: loop {
        match pc {
            0x82679D70 => {
    //   block [0x82679D70..0x82679DDC)
	// 82679D70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82679D74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82679D78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82679D7C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82679D80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82679D84: 38EB77D0  addi r7, r11, 0x77d0
	ctx.r[7].s64 = ctx.r[11].s64 + 30672;
	// 82679D88: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82679D8C: 388A2D14  addi r4, r10, 0x2d14
	ctx.r[4].s64 = ctx.r[10].s64 + 11540;
	// 82679D90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82679D94: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82679D98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82679D9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82679DA0: 386A5B38  addi r3, r10, 0x5b38
	ctx.r[3].s64 = ctx.r[10].s64 + 23352;
	// 82679DA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82679DA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82679DAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82679DB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82679DB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82679DB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82679DBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82679DC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82679DC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82679DC8: 4BDED059  bl 0x82466e20
	ctx.lr = 0x82679DCC;
	sub_82466E20(ctx, base);
	// 82679DCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82679DD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82679DD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82679DD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82679DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82679DE0 size=100
    let mut pc: u32 = 0x82679DE0;
    'dispatch: loop {
        match pc {
            0x82679DE0 => {
    //   block [0x82679DE0..0x82679E44)
	// 82679DE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82679DE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82679DE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82679DEC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82679DF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82679DF4: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 82679DF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82679DFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82679E00: 388A2D2C  addi r4, r10, 0x2d2c
	ctx.r[4].s64 = ctx.r[10].s64 + 11564;
	// 82679E04: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82679E08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82679E0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82679E10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82679E14: 386A5B68  addi r3, r10, 0x5b68
	ctx.r[3].s64 = ctx.r[10].s64 + 23400;
	// 82679E18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82679E1C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82679E20: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82679E24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82679E28: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82679E2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82679E30: 4BDECFF1  bl 0x82466e20
	ctx.lr = 0x82679E34;
	sub_82466E20(ctx, base);
	// 82679E34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82679E38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82679E3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82679E40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82679E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82679E48 size=112
    let mut pc: u32 = 0x82679E48;
    'dispatch: loop {
        match pc {
            0x82679E48 => {
    //   block [0x82679E48..0x82679EB8)
	// 82679E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82679E4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82679E50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82679E54: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82679E58: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82679E5C: 38AA5B68  addi r5, r10, 0x5b68
	ctx.r[5].s64 = ctx.r[10].s64 + 23400;
	// 82679E60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82679E64: 390B7860  addi r8, r11, 0x7860
	ctx.r[8].s64 = ctx.r[11].s64 + 30816;
	// 82679E68: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82679E6C: 388A2D48  addi r4, r10, 0x2d48
	ctx.r[4].s64 = ctx.r[10].s64 + 11592;
	// 82679E70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82679E74: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82679E78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82679E7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82679E80: 386A5B98  addi r3, r10, 0x5b98
	ctx.r[3].s64 = ctx.r[10].s64 + 23448;
	// 82679E84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82679E88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82679E8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82679E90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82679E94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82679E98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82679E9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82679EA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82679EA4: 4BDECF7D  bl 0x82466e20
	ctx.lr = 0x82679EA8;
	sub_82466E20(ctx, base);
	// 82679EA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82679EAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82679EB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82679EB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82679EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82679EB8 size=108
    let mut pc: u32 = 0x82679EB8;
    'dispatch: loop {
        match pc {
            0x82679EB8 => {
    //   block [0x82679EB8..0x82679F24)
	// 82679EB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82679EBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82679EC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82679EC4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82679EC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82679ECC: 38EB78C0  addi r7, r11, 0x78c0
	ctx.r[7].s64 = ctx.r[11].s64 + 30912;
	// 82679ED0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82679ED4: 388A2D68  addi r4, r10, 0x2d68
	ctx.r[4].s64 = ctx.r[10].s64 + 11624;
	// 82679ED8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82679EDC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82679EE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82679EE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82679EE8: 386A5BC8  addi r3, r10, 0x5bc8
	ctx.r[3].s64 = ctx.r[10].s64 + 23496;
	// 82679EEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82679EF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82679EF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82679EF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82679EFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82679F00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82679F04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82679F08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82679F0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82679F10: 4BDECF11  bl 0x82466e20
	ctx.lr = 0x82679F14;
	sub_82466E20(ctx, base);
	// 82679F14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82679F18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82679F1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82679F20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82679F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82679F28 size=108
    let mut pc: u32 = 0x82679F28;
    'dispatch: loop {
        match pc {
            0x82679F28 => {
    //   block [0x82679F28..0x82679F94)
	// 82679F28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82679F2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82679F30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82679F34: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82679F38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82679F3C: 38EB78F0  addi r7, r11, 0x78f0
	ctx.r[7].s64 = ctx.r[11].s64 + 30960;
	// 82679F40: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82679F44: 388A2D70  addi r4, r10, 0x2d70
	ctx.r[4].s64 = ctx.r[10].s64 + 11632;
	// 82679F48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82679F4C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82679F50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82679F54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82679F58: 386A5BF8  addi r3, r10, 0x5bf8
	ctx.r[3].s64 = ctx.r[10].s64 + 23544;
	// 82679F5C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82679F60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82679F64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82679F68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82679F6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82679F70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82679F74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82679F78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82679F7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82679F80: 4BDECEA1  bl 0x82466e20
	ctx.lr = 0x82679F84;
	sub_82466E20(ctx, base);
	// 82679F84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82679F88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82679F8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82679F90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82679F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82679F98 size=108
    let mut pc: u32 = 0x82679F98;
    'dispatch: loop {
        match pc {
            0x82679F98 => {
    //   block [0x82679F98..0x8267A004)
	// 82679F98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82679F9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82679FA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82679FA4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82679FA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82679FAC: 38EB7950  addi r7, r11, 0x7950
	ctx.r[7].s64 = ctx.r[11].s64 + 31056;
	// 82679FB0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82679FB4: 388A2D84  addi r4, r10, 0x2d84
	ctx.r[4].s64 = ctx.r[10].s64 + 11652;
	// 82679FB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82679FBC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82679FC0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82679FC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82679FC8: 386A5C28  addi r3, r10, 0x5c28
	ctx.r[3].s64 = ctx.r[10].s64 + 23592;
	// 82679FCC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82679FD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82679FD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82679FD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82679FDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82679FE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82679FE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82679FE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82679FEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82679FF0: 4BDECE31  bl 0x82466e20
	ctx.lr = 0x82679FF4;
	sub_82466E20(ctx, base);
	// 82679FF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82679FF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82679FFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267A000: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267A008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8267A008 size=24
    let mut pc: u32 = 0x8267A008;
    'dispatch: loop {
        match pc {
            0x8267A008 => {
    //   block [0x8267A008..0x8267A020)
	// 8267A008: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267A00C: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267A010: 394AE498  addi r10, r10, -0x1b68
	ctx.r[10].s64 = ctx.r[10].s64 + -7016;
	// 8267A014: 816B76F4  lwz r11, 0x76f4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30452 as u32) ) } as u64;
	// 8267A018: 916A0128  stw r11, 0x128(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(296 as u32), ctx.r[11].u32 ) };
	// 8267A01C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267A020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267A020 size=116
    let mut pc: u32 = 0x8267A020;
    'dispatch: loop {
        match pc {
            0x8267A020 => {
    //   block [0x8267A020..0x8267A094)
	// 8267A020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267A024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267A028: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267A02C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267A030: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8267A034: 390BE498  addi r8, r11, -0x1b68
	ctx.r[8].s64 = ctx.r[11].s64 + -7016;
	// 8267A038: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267A03C: 392A3C78  addi r9, r10, 0x3c78
	ctx.r[9].s64 = ctx.r[10].s64 + 15480;
	// 8267A040: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A044: 38E0000D  li r7, 0xd
	ctx.r[7].s64 = 13;
	// 8267A048: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 8267A04C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267A050: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267A054: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267A058: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267A05C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267A060: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267A064: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 8267A068: 388A2D90  addi r4, r10, 0x2d90
	ctx.r[4].s64 = ctx.r[10].s64 + 11664;
	// 8267A06C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267A070: 386B5C58  addi r3, r11, 0x5c58
	ctx.r[3].s64 = ctx.r[11].s64 + 23640;
	// 8267A074: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8267A078: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267A07C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267A080: 4BDECDA1  bl 0x82466e20
	ctx.lr = 0x8267A084;
	sub_82466E20(ctx, base);
	// 8267A084: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267A088: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267A08C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267A090: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267A098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267A098 size=112
    let mut pc: u32 = 0x8267A098;
    'dispatch: loop {
        match pc {
            0x8267A098 => {
    //   block [0x8267A098..0x8267A108)
	// 8267A098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267A09C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267A0A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267A0A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A0A8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267A0AC: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 8267A0B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267A0B4: 390B79B0  addi r8, r11, 0x79b0
	ctx.r[8].s64 = ctx.r[11].s64 + 31152;
	// 8267A0B8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8267A0BC: 388A2DA4  addi r4, r10, 0x2da4
	ctx.r[4].s64 = ctx.r[10].s64 + 11684;
	// 8267A0C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267A0C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A0C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267A0CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267A0D0: 386A5C88  addi r3, r10, 0x5c88
	ctx.r[3].s64 = ctx.r[10].s64 + 23688;
	// 8267A0D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267A0D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267A0DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267A0E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267A0E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267A0E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267A0EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267A0F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267A0F4: 4BDECD2D  bl 0x82466e20
	ctx.lr = 0x8267A0F8;
	sub_82466E20(ctx, base);
	// 8267A0F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267A0FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267A100: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267A104: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267A108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267A108 size=112
    let mut pc: u32 = 0x8267A108;
    'dispatch: loop {
        match pc {
            0x8267A108 => {
    //   block [0x8267A108..0x8267A178)
	// 8267A108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267A10C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267A110: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267A114: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A118: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267A11C: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 8267A120: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267A124: 390B79F8  addi r8, r11, 0x79f8
	ctx.r[8].s64 = ctx.r[11].s64 + 31224;
	// 8267A128: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8267A12C: 388A2DBC  addi r4, r10, 0x2dbc
	ctx.r[4].s64 = ctx.r[10].s64 + 11708;
	// 8267A130: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267A134: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A138: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267A13C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267A140: 386A5CB8  addi r3, r10, 0x5cb8
	ctx.r[3].s64 = ctx.r[10].s64 + 23736;
	// 8267A144: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267A148: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267A14C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267A150: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267A154: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267A158: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267A15C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267A160: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267A164: 4BDECCBD  bl 0x82466e20
	ctx.lr = 0x8267A168;
	sub_82466E20(ctx, base);
	// 8267A168: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267A16C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267A170: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267A174: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267A178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267A178 size=112
    let mut pc: u32 = 0x8267A178;
    'dispatch: loop {
        match pc {
            0x8267A178 => {
    //   block [0x8267A178..0x8267A1E8)
	// 8267A178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267A17C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267A180: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267A184: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A188: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267A18C: 38AA5DD8  addi r5, r10, 0x5dd8
	ctx.r[5].s64 = ctx.r[10].s64 + 24024;
	// 8267A190: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267A194: 390B7A40  addi r8, r11, 0x7a40
	ctx.r[8].s64 = ctx.r[11].s64 + 31296;
	// 8267A198: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 8267A19C: 388A2DCC  addi r4, r10, 0x2dcc
	ctx.r[4].s64 = ctx.r[10].s64 + 11724;
	// 8267A1A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267A1A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A1A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267A1AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267A1B0: 386A5CE8  addi r3, r10, 0x5ce8
	ctx.r[3].s64 = ctx.r[10].s64 + 23784;
	// 8267A1B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267A1B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267A1BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267A1C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267A1C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267A1C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267A1CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267A1D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267A1D4: 4BDECC4D  bl 0x82466e20
	ctx.lr = 0x8267A1D8;
	sub_82466E20(ctx, base);
	// 8267A1D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267A1DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267A1E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267A1E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267A1E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267A1E8 size=112
    let mut pc: u32 = 0x8267A1E8;
    'dispatch: loop {
        match pc {
            0x8267A1E8 => {
    //   block [0x8267A1E8..0x8267A258)
	// 8267A1E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267A1EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267A1F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267A1F4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8267A1F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267A1FC: 392B3CAC  addi r9, r11, 0x3cac
	ctx.r[9].s64 = ctx.r[11].s64 + 15532;
	// 8267A200: 38C0000E  li r6, 0xe
	ctx.r[6].s64 = 14;
	// 8267A204: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 8267A208: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267A20C: 388A2E00  addi r4, r10, 0x2e00
	ctx.r[4].s64 = ctx.r[10].s64 + 11776;
	// 8267A210: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267A214: 396B7B20  addi r11, r11, 0x7b20
	ctx.r[11].s64 = ctx.r[11].s64 + 31520;
	// 8267A218: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8267A21C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A220: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8267A224: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267A228: 386A5D18  addi r3, r10, 0x5d18
	ctx.r[3].s64 = ctx.r[10].s64 + 23832;
	// 8267A22C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8267A230: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8267A234: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267A238: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8267A23C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267A240: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267A244: 4BDECBDD  bl 0x82466e20
	ctx.lr = 0x8267A248;
	sub_82466E20(ctx, base);
	// 8267A248: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267A24C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267A250: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267A254: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267A258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267A258 size=112
    let mut pc: u32 = 0x8267A258;
    'dispatch: loop {
        match pc {
            0x8267A258 => {
    //   block [0x8267A258..0x8267A2C8)
	// 8267A258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267A25C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267A260: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267A264: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A268: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267A26C: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 8267A270: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267A274: 390B7C70  addi r8, r11, 0x7c70
	ctx.r[8].s64 = ctx.r[11].s64 + 31856;
	// 8267A278: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8267A27C: 388A2E18  addi r4, r10, 0x2e18
	ctx.r[4].s64 = ctx.r[10].s64 + 11800;
	// 8267A280: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267A284: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A288: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267A28C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267A290: 386A5D48  addi r3, r10, 0x5d48
	ctx.r[3].s64 = ctx.r[10].s64 + 23880;
	// 8267A294: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267A298: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267A29C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267A2A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267A2A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267A2A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267A2AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267A2B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267A2B4: 4BDECB6D  bl 0x82466e20
	ctx.lr = 0x8267A2B8;
	sub_82466E20(ctx, base);
	// 8267A2B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267A2BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267A2C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267A2C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267A2C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267A2C8 size=112
    let mut pc: u32 = 0x8267A2C8;
    'dispatch: loop {
        match pc {
            0x8267A2C8 => {
    //   block [0x8267A2C8..0x8267A338)
	// 8267A2C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267A2CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267A2D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267A2D4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A2D8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267A2DC: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 8267A2E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267A2E4: 390B7D18  addi r8, r11, 0x7d18
	ctx.r[8].s64 = ctx.r[11].s64 + 32024;
	// 8267A2E8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8267A2EC: 388A2E30  addi r4, r10, 0x2e30
	ctx.r[4].s64 = ctx.r[10].s64 + 11824;
	// 8267A2F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267A2F4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A2F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267A2FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267A300: 386A5D78  addi r3, r10, 0x5d78
	ctx.r[3].s64 = ctx.r[10].s64 + 23928;
	// 8267A304: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267A308: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267A30C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267A310: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267A314: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267A318: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267A31C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267A320: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267A324: 4BDECAFD  bl 0x82466e20
	ctx.lr = 0x8267A328;
	sub_82466E20(ctx, base);
	// 8267A328: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267A32C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267A330: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267A334: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267A338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267A338 size=112
    let mut pc: u32 = 0x8267A338;
    'dispatch: loop {
        match pc {
            0x8267A338 => {
    //   block [0x8267A338..0x8267A3A8)
	// 8267A338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267A33C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267A340: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267A344: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8267A348: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267A34C: 392A3D08  addi r9, r10, 0x3d08
	ctx.r[9].s64 = ctx.r[10].s64 + 15624;
	// 8267A350: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267A354: 390B7DA8  addi r8, r11, 0x7da8
	ctx.r[8].s64 = ctx.r[11].s64 + 32168;
	// 8267A358: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 8267A35C: 388A2E64  addi r4, r10, 0x2e64
	ctx.r[4].s64 = ctx.r[10].s64 + 11876;
	// 8267A360: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267A364: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A368: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267A36C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267A370: 386A5DA8  addi r3, r10, 0x5da8
	ctx.r[3].s64 = ctx.r[10].s64 + 23976;
	// 8267A374: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267A378: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8267A37C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267A380: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267A384: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267A388: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267A38C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267A390: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267A394: 4BDECA8D  bl 0x82466e20
	ctx.lr = 0x8267A398;
	sub_82466E20(ctx, base);
	// 8267A398: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267A39C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267A3A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267A3A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267A3A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267A3A8 size=100
    let mut pc: u32 = 0x8267A3A8;
    'dispatch: loop {
        match pc {
            0x8267A3A8 => {
    //   block [0x8267A3A8..0x8267A40C)
	// 8267A3A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267A3AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267A3B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267A3B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A3B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267A3BC: 38AA64F8  addi r5, r10, 0x64f8
	ctx.r[5].s64 = ctx.r[10].s64 + 25848;
	// 8267A3C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267A3C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267A3C8: 388A2E70  addi r4, r10, 0x2e70
	ctx.r[4].s64 = ctx.r[10].s64 + 11888;
	// 8267A3CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A3D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267A3D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267A3D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267A3DC: 386A5DD8  addi r3, r10, 0x5dd8
	ctx.r[3].s64 = ctx.r[10].s64 + 24024;
	// 8267A3E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267A3E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267A3E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8267A3EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267A3F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267A3F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267A3F8: 4BDECA29  bl 0x82466e20
	ctx.lr = 0x8267A3FC;
	sub_82466E20(ctx, base);
	// 8267A3FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267A400: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267A404: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267A408: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267A410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267A410 size=112
    let mut pc: u32 = 0x8267A410;
    'dispatch: loop {
        match pc {
            0x8267A410 => {
    //   block [0x8267A410..0x8267A480)
	// 8267A410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267A414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267A418: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267A41C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A420: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267A424: 38AA5DD8  addi r5, r10, 0x5dd8
	ctx.r[5].s64 = ctx.r[10].s64 + 24024;
	// 8267A428: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267A42C: 390B7DD8  addi r8, r11, 0x7dd8
	ctx.r[8].s64 = ctx.r[11].s64 + 32216;
	// 8267A430: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8267A434: 388A2EBC  addi r4, r10, 0x2ebc
	ctx.r[4].s64 = ctx.r[10].s64 + 11964;
	// 8267A438: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267A43C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A440: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267A444: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267A448: 386A5E08  addi r3, r10, 0x5e08
	ctx.r[3].s64 = ctx.r[10].s64 + 24072;
	// 8267A44C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267A450: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267A454: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267A458: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267A45C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267A460: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267A464: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267A468: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267A46C: 4BDEC9B5  bl 0x82466e20
	ctx.lr = 0x8267A470;
	sub_82466E20(ctx, base);
	// 8267A470: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267A474: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267A478: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267A47C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267A480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267A480 size=116
    let mut pc: u32 = 0x8267A480;
    'dispatch: loop {
        match pc {
            0x8267A480 => {
    //   block [0x8267A480..0x8267A4F4)
	// 8267A480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267A484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267A488: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267A48C: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 8267A490: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8267A494: 390A7E08  addi r8, r10, 0x7e08
	ctx.r[8].s64 = ctx.r[10].s64 + 32264;
	// 8267A498: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A49C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8267A4A0: 38AA5DD8  addi r5, r10, 0x5dd8
	ctx.r[5].s64 = ctx.r[10].s64 + 24024;
	// 8267A4A4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267A4A8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267A4AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267A4B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267A4B4: 388A2EDC  addi r4, r10, 0x2edc
	ctx.r[4].s64 = ctx.r[10].s64 + 11996;
	// 8267A4B8: 396B3D1C  addi r11, r11, 0x3d1c
	ctx.r[11].s64 = ctx.r[11].s64 + 15644;
	// 8267A4BC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A4C0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267A4C4: 386A5E38  addi r3, r10, 0x5e38
	ctx.r[3].s64 = ctx.r[10].s64 + 24120;
	// 8267A4C8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8267A4CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267A4D0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8267A4D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267A4D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267A4DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267A4E0: 4BDEC941  bl 0x82466e20
	ctx.lr = 0x8267A4E4;
	sub_82466E20(ctx, base);
	// 8267A4E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267A4E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267A4EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267A4F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267A4F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267A4F8 size=100
    let mut pc: u32 = 0x8267A4F8;
    'dispatch: loop {
        match pc {
            0x8267A4F8 => {
    //   block [0x8267A4F8..0x8267A55C)
	// 8267A4F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267A4FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267A500: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267A504: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A508: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267A50C: 38AA5E38  addi r5, r10, 0x5e38
	ctx.r[5].s64 = ctx.r[10].s64 + 24120;
	// 8267A510: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267A514: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267A518: 388A2EF8  addi r4, r10, 0x2ef8
	ctx.r[4].s64 = ctx.r[10].s64 + 12024;
	// 8267A51C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A520: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267A524: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267A528: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267A52C: 386A5E68  addi r3, r10, 0x5e68
	ctx.r[3].s64 = ctx.r[10].s64 + 24168;
	// 8267A530: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267A534: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267A538: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8267A53C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267A540: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267A544: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267A548: 4BDEC8D9  bl 0x82466e20
	ctx.lr = 0x8267A54C;
	sub_82466E20(ctx, base);
	// 8267A54C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267A550: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267A554: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267A558: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267A560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8267A560 size=24
    let mut pc: u32 = 0x8267A560;
    'dispatch: loop {
        match pc {
            0x8267A560 => {
    //   block [0x8267A560..0x8267A578)
	// 8267A560: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267A564: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267A568: 394AE5D0  addi r10, r10, -0x1a30
	ctx.r[10].s64 = ctx.r[10].s64 + -6704;
	// 8267A56C: 816B7EB0  lwz r11, 0x7eb0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32432 as u32) ) } as u64;
	// 8267A570: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8267A574: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267A578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267A578 size=116
    let mut pc: u32 = 0x8267A578;
    'dispatch: loop {
        match pc {
            0x8267A578 => {
    //   block [0x8267A578..0x8267A5EC)
	// 8267A578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267A57C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267A580: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267A584: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8267A588: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A58C: 392B3D58  addi r9, r11, 0x3d58
	ctx.r[9].s64 = ctx.r[11].s64 + 15704;
	// 8267A590: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 8267A594: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267A598: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 8267A59C: 38C00005  li r6, 5
	ctx.r[6].s64 = 5;
	// 8267A5A0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267A5A4: 388A2F1C  addi r4, r10, 0x2f1c
	ctx.r[4].s64 = ctx.r[10].s64 + 12060;
	// 8267A5A8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267A5AC: 396BE5D0  addi r11, r11, -0x1a30
	ctx.r[11].s64 = ctx.r[11].s64 + -6704;
	// 8267A5B0: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8267A5B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A5B8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8267A5BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267A5C0: 386A5E98  addi r3, r10, 0x5e98
	ctx.r[3].s64 = ctx.r[10].s64 + 24216;
	// 8267A5C4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8267A5C8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8267A5CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267A5D0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8267A5D4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267A5D8: 4BDEC849  bl 0x82466e20
	ctx.lr = 0x8267A5DC;
	sub_82466E20(ctx, base);
	// 8267A5DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267A5E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267A5E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267A5E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267A5F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267A5F0 size=116
    let mut pc: u32 = 0x8267A5F0;
    'dispatch: loop {
        match pc {
            0x8267A5F0 => {
    //   block [0x8267A5F0..0x8267A664)
	// 8267A5F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267A5F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267A5F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267A5FC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8267A600: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A604: 392B3D9C  addi r9, r11, 0x3d9c
	ctx.r[9].s64 = ctx.r[11].s64 + 15772;
	// 8267A608: 38AA5DD8  addi r5, r10, 0x5dd8
	ctx.r[5].s64 = ctx.r[10].s64 + 24024;
	// 8267A60C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267A610: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 8267A614: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 8267A618: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267A61C: 388A2F38  addi r4, r10, 0x2f38
	ctx.r[4].s64 = ctx.r[10].s64 + 12088;
	// 8267A620: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267A624: 396B7EB8  addi r11, r11, 0x7eb8
	ctx.r[11].s64 = ctx.r[11].s64 + 32440;
	// 8267A628: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8267A62C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A630: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8267A634: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267A638: 386A5EC8  addi r3, r10, 0x5ec8
	ctx.r[3].s64 = ctx.r[10].s64 + 24264;
	// 8267A63C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8267A640: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8267A644: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267A648: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8267A64C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267A650: 4BDEC7D1  bl 0x82466e20
	ctx.lr = 0x8267A654;
	sub_82466E20(ctx, base);
	// 8267A654: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267A658: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267A65C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267A660: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267A668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267A668 size=108
    let mut pc: u32 = 0x8267A668;
    'dispatch: loop {
        match pc {
            0x8267A668 => {
    //   block [0x8267A668..0x8267A6D4)
	// 8267A668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267A66C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267A670: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267A674: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267A678: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267A67C: 38EB7F60  addi r7, r11, 0x7f60
	ctx.r[7].s64 = ctx.r[11].s64 + 32608;
	// 8267A680: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8267A684: 388A2F4C  addi r4, r10, 0x2f4c
	ctx.r[4].s64 = ctx.r[10].s64 + 12108;
	// 8267A688: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267A68C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A690: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267A694: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267A698: 386A5EF8  addi r3, r10, 0x5ef8
	ctx.r[3].s64 = ctx.r[10].s64 + 24312;
	// 8267A69C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267A6A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267A6A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267A6A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267A6AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267A6B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267A6B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267A6B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267A6BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267A6C0: 4BDEC761  bl 0x82466e20
	ctx.lr = 0x8267A6C4;
	sub_82466E20(ctx, base);
	// 8267A6C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267A6C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267A6CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267A6D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267A6D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8267A6D8 size=24
    let mut pc: u32 = 0x8267A6D8;
    'dispatch: loop {
        match pc {
            0x8267A6D8 => {
    //   block [0x8267A6D8..0x8267A6F0)
	// 8267A6D8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267A6DC: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267A6E0: 394AE648  addi r10, r10, -0x19b8
	ctx.r[10].s64 = ctx.r[10].s64 + -6584;
	// 8267A6E4: 816B7FA8  lwz r11, 0x7fa8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32680 as u32) ) } as u64;
	// 8267A6E8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8267A6EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267A6F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267A6F0 size=116
    let mut pc: u32 = 0x8267A6F0;
    'dispatch: loop {
        match pc {
            0x8267A6F0 => {
    //   block [0x8267A6F0..0x8267A764)
	// 8267A6F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267A6F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267A6F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267A6FC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267A700: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8267A704: 390BE648  addi r8, r11, -0x19b8
	ctx.r[8].s64 = ctx.r[11].s64 + -6584;
	// 8267A708: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267A70C: 392A3E08  addi r9, r10, 0x3e08
	ctx.r[9].s64 = ctx.r[10].s64 + 15880;
	// 8267A710: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A714: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 8267A718: 38AA5DD8  addi r5, r10, 0x5dd8
	ctx.r[5].s64 = ctx.r[10].s64 + 24024;
	// 8267A71C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267A720: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267A724: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267A728: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267A72C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267A730: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267A734: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 8267A738: 388A3040  addi r4, r10, 0x3040
	ctx.r[4].s64 = ctx.r[10].s64 + 12352;
	// 8267A73C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267A740: 386B5F28  addi r3, r11, 0x5f28
	ctx.r[3].s64 = ctx.r[11].s64 + 24360;
	// 8267A744: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8267A748: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267A74C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267A750: 4BDEC6D1  bl 0x82466e20
	ctx.lr = 0x8267A754;
	sub_82466E20(ctx, base);
	// 8267A754: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267A758: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267A75C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267A760: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267A768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267A768 size=112
    let mut pc: u32 = 0x8267A768;
    'dispatch: loop {
        match pc {
            0x8267A768 => {
    //   block [0x8267A768..0x8267A7D8)
	// 8267A768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267A76C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267A770: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267A774: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A778: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267A77C: 38AA5DD8  addi r5, r10, 0x5dd8
	ctx.r[5].s64 = ctx.r[10].s64 + 24024;
	// 8267A780: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267A784: 390B7FB0  addi r8, r11, 0x7fb0
	ctx.r[8].s64 = ctx.r[11].s64 + 32688;
	// 8267A788: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8267A78C: 388A3054  addi r4, r10, 0x3054
	ctx.r[4].s64 = ctx.r[10].s64 + 12372;
	// 8267A790: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267A794: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A798: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267A79C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267A7A0: 386A5F58  addi r3, r10, 0x5f58
	ctx.r[3].s64 = ctx.r[10].s64 + 24408;
	// 8267A7A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267A7A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267A7AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267A7B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267A7B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267A7B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267A7BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267A7C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267A7C4: 4BDEC65D  bl 0x82466e20
	ctx.lr = 0x8267A7C8;
	sub_82466E20(ctx, base);
	// 8267A7C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267A7CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267A7D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267A7D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267A7D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8267A7D8 size=24
    let mut pc: u32 = 0x8267A7D8;
    'dispatch: loop {
        match pc {
            0x8267A7D8 => {
    //   block [0x8267A7D8..0x8267A7F0)
	// 8267A7D8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267A7DC: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267A7E0: 394AE7C8  addi r10, r10, -0x1838
	ctx.r[10].s64 = ctx.r[10].s64 + -6200;
	// 8267A7E4: 816B7FE0  lwz r11, 0x7fe0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32736 as u32) ) } as u64;
	// 8267A7E8: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8267A7EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267A7F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267A7F0 size=116
    let mut pc: u32 = 0x8267A7F0;
    'dispatch: loop {
        match pc {
            0x8267A7F0 => {
    //   block [0x8267A7F0..0x8267A864)
	// 8267A7F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267A7F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267A7F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267A7FC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267A800: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8267A804: 390BE7C8  addi r8, r11, -0x1838
	ctx.r[8].s64 = ctx.r[11].s64 + -6200;
	// 8267A808: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267A80C: 392A3E40  addi r9, r10, 0x3e40
	ctx.r[9].s64 = ctx.r[10].s64 + 15936;
	// 8267A810: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A814: 38E0000A  li r7, 0xa
	ctx.r[7].s64 = 10;
	// 8267A818: 38AA5EC8  addi r5, r10, 0x5ec8
	ctx.r[5].s64 = ctx.r[10].s64 + 24264;
	// 8267A81C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267A820: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267A824: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267A828: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267A82C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267A830: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267A834: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 8267A838: 388A30BC  addi r4, r10, 0x30bc
	ctx.r[4].s64 = ctx.r[10].s64 + 12476;
	// 8267A83C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267A840: 386B5F88  addi r3, r11, 0x5f88
	ctx.r[3].s64 = ctx.r[11].s64 + 24456;
	// 8267A844: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8267A848: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267A84C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267A850: 4BDEC5D1  bl 0x82466e20
	ctx.lr = 0x8267A854;
	sub_82466E20(ctx, base);
	// 8267A854: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267A858: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267A85C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267A860: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267A868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267A868 size=112
    let mut pc: u32 = 0x8267A868;
    'dispatch: loop {
        match pc {
            0x8267A868 => {
    //   block [0x8267A868..0x8267A8D8)
	// 8267A868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267A86C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267A870: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267A874: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A878: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267A87C: 38AA5DD8  addi r5, r10, 0x5dd8
	ctx.r[5].s64 = ctx.r[10].s64 + 24024;
	// 8267A880: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267A884: 390B7FE4  addi r8, r11, 0x7fe4
	ctx.r[8].s64 = ctx.r[11].s64 + 32740;
	// 8267A888: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8267A88C: 388A30D8  addi r4, r10, 0x30d8
	ctx.r[4].s64 = ctx.r[10].s64 + 12504;
	// 8267A890: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267A894: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A898: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267A89C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267A8A0: 386A5FB8  addi r3, r10, 0x5fb8
	ctx.r[3].s64 = ctx.r[10].s64 + 24504;
	// 8267A8A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267A8A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267A8AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267A8B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267A8B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267A8B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267A8BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267A8C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267A8C4: 4BDEC55D  bl 0x82466e20
	ctx.lr = 0x8267A8C8;
	sub_82466E20(ctx, base);
	// 8267A8C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267A8CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267A8D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267A8D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267A8D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267A8D8 size=100
    let mut pc: u32 = 0x8267A8D8;
    'dispatch: loop {
        match pc {
            0x8267A8D8 => {
    //   block [0x8267A8D8..0x8267A93C)
	// 8267A8D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267A8DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267A8E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267A8E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A8E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267A8EC: 38AA64F8  addi r5, r10, 0x64f8
	ctx.r[5].s64 = ctx.r[10].s64 + 25848;
	// 8267A8F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267A8F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267A8F8: 388A30F4  addi r4, r10, 0x30f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12532;
	// 8267A8FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A900: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267A904: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267A908: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267A90C: 386A5FE8  addi r3, r10, 0x5fe8
	ctx.r[3].s64 = ctx.r[10].s64 + 24552;
	// 8267A910: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267A914: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267A918: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8267A91C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267A920: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267A924: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267A928: 4BDEC4F9  bl 0x82466e20
	ctx.lr = 0x8267A92C;
	sub_82466E20(ctx, base);
	// 8267A92C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267A930: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267A934: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267A938: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267A940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267A940 size=108
    let mut pc: u32 = 0x8267A940;
    'dispatch: loop {
        match pc {
            0x8267A940 => {
    //   block [0x8267A940..0x8267A9AC)
	// 8267A940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267A944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267A948: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267A94C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267A950: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267A954: 38EB8000  addi r7, r11, -0x8000
	ctx.r[7].s64 = ctx.r[11].s64 + -32768;
	// 8267A958: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 8267A95C: 388A3100  addi r4, r10, 0x3100
	ctx.r[4].s64 = ctx.r[10].s64 + 12544;
	// 8267A960: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267A964: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A968: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267A96C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267A970: 386A6018  addi r3, r10, 0x6018
	ctx.r[3].s64 = ctx.r[10].s64 + 24600;
	// 8267A974: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267A978: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267A97C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267A980: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267A984: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267A988: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267A98C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267A990: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267A994: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267A998: 4BDEC489  bl 0x82466e20
	ctx.lr = 0x8267A99C;
	sub_82466E20(ctx, base);
	// 8267A99C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267A9A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267A9A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267A9A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267A9B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267A9B0 size=112
    let mut pc: u32 = 0x8267A9B0;
    'dispatch: loop {
        match pc {
            0x8267A9B0 => {
    //   block [0x8267A9B0..0x8267AA20)
	// 8267A9B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267A9B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267A9B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267A9BC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A9C0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267A9C4: 38AA5FE8  addi r5, r10, 0x5fe8
	ctx.r[5].s64 = ctx.r[10].s64 + 24552;
	// 8267A9C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267A9CC: 390B80D8  addi r8, r11, -0x7f28
	ctx.r[8].s64 = ctx.r[11].s64 + -32552;
	// 8267A9D0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8267A9D4: 388A312C  addi r4, r10, 0x312c
	ctx.r[4].s64 = ctx.r[10].s64 + 12588;
	// 8267A9D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267A9DC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267A9E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267A9E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267A9E8: 386A6048  addi r3, r10, 0x6048
	ctx.r[3].s64 = ctx.r[10].s64 + 24648;
	// 8267A9EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267A9F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267A9F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267A9F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267A9FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267AA00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267AA04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267AA08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267AA0C: 4BDEC415  bl 0x82466e20
	ctx.lr = 0x8267AA10;
	sub_82466E20(ctx, base);
	// 8267AA10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267AA14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267AA18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267AA1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267AA20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267AA20 size=108
    let mut pc: u32 = 0x8267AA20;
    'dispatch: loop {
        match pc {
            0x8267AA20 => {
    //   block [0x8267AA20..0x8267AA8C)
	// 8267AA20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267AA24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267AA28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267AA2C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267AA30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267AA34: 38EB8108  addi r7, r11, -0x7ef8
	ctx.r[7].s64 = ctx.r[11].s64 + -32504;
	// 8267AA38: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8267AA3C: 388A3144  addi r4, r10, 0x3144
	ctx.r[4].s64 = ctx.r[10].s64 + 12612;
	// 8267AA40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267AA44: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267AA48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267AA4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267AA50: 386A6078  addi r3, r10, 0x6078
	ctx.r[3].s64 = ctx.r[10].s64 + 24696;
	// 8267AA54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267AA58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267AA5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267AA60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267AA64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267AA68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267AA6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267AA70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267AA74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267AA78: 4BDEC3A9  bl 0x82466e20
	ctx.lr = 0x8267AA7C;
	sub_82466E20(ctx, base);
	// 8267AA7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267AA80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267AA84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267AA88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267AA90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267AA90 size=112
    let mut pc: u32 = 0x8267AA90;
    'dispatch: loop {
        match pc {
            0x8267AA90 => {
    //   block [0x8267AA90..0x8267AB00)
	// 8267AA90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267AA94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267AA98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267AA9C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267AAA0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267AAA4: 38AA5FE8  addi r5, r10, 0x5fe8
	ctx.r[5].s64 = ctx.r[10].s64 + 24552;
	// 8267AAA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267AAAC: 390B8138  addi r8, r11, -0x7ec8
	ctx.r[8].s64 = ctx.r[11].s64 + -32456;
	// 8267AAB0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8267AAB4: 388A3164  addi r4, r10, 0x3164
	ctx.r[4].s64 = ctx.r[10].s64 + 12644;
	// 8267AAB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267AABC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267AAC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267AAC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267AAC8: 386A60A8  addi r3, r10, 0x60a8
	ctx.r[3].s64 = ctx.r[10].s64 + 24744;
	// 8267AACC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267AAD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267AAD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267AAD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267AADC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267AAE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267AAE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267AAE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267AAEC: 4BDEC335  bl 0x82466e20
	ctx.lr = 0x8267AAF0;
	sub_82466E20(ctx, base);
	// 8267AAF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267AAF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267AAF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267AAFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267AB00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267AB00 size=112
    let mut pc: u32 = 0x8267AB00;
    'dispatch: loop {
        match pc {
            0x8267AB00 => {
    //   block [0x8267AB00..0x8267AB70)
	// 8267AB00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267AB04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267AB08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267AB0C: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267AB10: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 8267AB14: 38EA8150  addi r7, r10, -0x7eb0
	ctx.r[7].s64 = ctx.r[10].s64 + -32432;
	// 8267AB18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267AB1C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8267AB20: 388A317C  addi r4, r10, 0x317c
	ctx.r[4].s64 = ctx.r[10].s64 + 12668;
	// 8267AB24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267AB28: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267AB2C: 396B3E54  addi r11, r11, 0x3e54
	ctx.r[11].s64 = ctx.r[11].s64 + 15956;
	// 8267AB30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267AB34: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267AB38: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267AB3C: 386A60D8  addi r3, r10, 0x60d8
	ctx.r[3].s64 = ctx.r[10].s64 + 24792;
	// 8267AB40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267AB44: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8267AB48: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267AB4C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8267AB50: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267AB54: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267AB58: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267AB5C: 4BDEC2C5  bl 0x82466e20
	ctx.lr = 0x8267AB60;
	sub_82466E20(ctx, base);
	// 8267AB60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267AB64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267AB68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267AB6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267AB70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267AB70 size=108
    let mut pc: u32 = 0x8267AB70;
    'dispatch: loop {
        match pc {
            0x8267AB70 => {
    //   block [0x8267AB70..0x8267ABDC)
	// 8267AB70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267AB74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267AB78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267AB7C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267AB80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267AB84: 38EB8228  addi r7, r11, -0x7dd8
	ctx.r[7].s64 = ctx.r[11].s64 + -32216;
	// 8267AB88: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8267AB8C: 388A318C  addi r4, r10, 0x318c
	ctx.r[4].s64 = ctx.r[10].s64 + 12684;
	// 8267AB90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267AB94: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267AB98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267AB9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267ABA0: 386A6108  addi r3, r10, 0x6108
	ctx.r[3].s64 = ctx.r[10].s64 + 24840;
	// 8267ABA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267ABA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267ABAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267ABB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267ABB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267ABB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267ABBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267ABC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267ABC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267ABC8: 4BDEC259  bl 0x82466e20
	ctx.lr = 0x8267ABCC;
	sub_82466E20(ctx, base);
	// 8267ABCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267ABD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267ABD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267ABD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267ABE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267ABE0 size=108
    let mut pc: u32 = 0x8267ABE0;
    'dispatch: loop {
        match pc {
            0x8267ABE0 => {
    //   block [0x8267ABE0..0x8267AC4C)
	// 8267ABE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267ABE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267ABE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267ABEC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267ABF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267ABF4: 38EB8240  addi r7, r11, -0x7dc0
	ctx.r[7].s64 = ctx.r[11].s64 + -32192;
	// 8267ABF8: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 8267ABFC: 388A31A4  addi r4, r10, 0x31a4
	ctx.r[4].s64 = ctx.r[10].s64 + 12708;
	// 8267AC00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267AC04: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267AC08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267AC0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267AC10: 386A6138  addi r3, r10, 0x6138
	ctx.r[3].s64 = ctx.r[10].s64 + 24888;
	// 8267AC14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267AC18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267AC1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267AC20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267AC24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267AC28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267AC2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267AC30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267AC34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267AC38: 4BDEC1E9  bl 0x82466e20
	ctx.lr = 0x8267AC3C;
	sub_82466E20(ctx, base);
	// 8267AC3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267AC40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267AC44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267AC48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267AC50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267AC50 size=112
    let mut pc: u32 = 0x8267AC50;
    'dispatch: loop {
        match pc {
            0x8267AC50 => {
    //   block [0x8267AC50..0x8267ACC0)
	// 8267AC50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267AC54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267AC58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267AC5C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267AC60: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267AC64: 38AA5FE8  addi r5, r10, 0x5fe8
	ctx.r[5].s64 = ctx.r[10].s64 + 24552;
	// 8267AC68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267AC6C: 390B8348  addi r8, r11, -0x7cb8
	ctx.r[8].s64 = ctx.r[11].s64 + -31928;
	// 8267AC70: 3920000B  li r9, 0xb
	ctx.r[9].s64 = 11;
	// 8267AC74: 388A31E0  addi r4, r10, 0x31e0
	ctx.r[4].s64 = ctx.r[10].s64 + 12768;
	// 8267AC78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267AC7C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267AC80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267AC84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267AC88: 386A6168  addi r3, r10, 0x6168
	ctx.r[3].s64 = ctx.r[10].s64 + 24936;
	// 8267AC8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267AC90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267AC94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267AC98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267AC9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267ACA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267ACA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267ACA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267ACAC: 4BDEC175  bl 0x82466e20
	ctx.lr = 0x8267ACB0;
	sub_82466E20(ctx, base);
	// 8267ACB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267ACB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267ACB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267ACBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267ACC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267ACC0 size=112
    let mut pc: u32 = 0x8267ACC0;
    'dispatch: loop {
        match pc {
            0x8267ACC0 => {
    //   block [0x8267ACC0..0x8267AD30)
	// 8267ACC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267ACC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267ACC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267ACCC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267ACD0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267ACD4: 38AA5FE8  addi r5, r10, 0x5fe8
	ctx.r[5].s64 = ctx.r[10].s64 + 24552;
	// 8267ACD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267ACDC: 390B8450  addi r8, r11, -0x7bb0
	ctx.r[8].s64 = ctx.r[11].s64 + -31664;
	// 8267ACE0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8267ACE4: 388A31F4  addi r4, r10, 0x31f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12788;
	// 8267ACE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267ACEC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267ACF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267ACF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267ACF8: 386A6198  addi r3, r10, 0x6198
	ctx.r[3].s64 = ctx.r[10].s64 + 24984;
	// 8267ACFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267AD00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267AD04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267AD08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267AD0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267AD10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267AD14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267AD18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267AD1C: 4BDEC105  bl 0x82466e20
	ctx.lr = 0x8267AD20;
	sub_82466E20(ctx, base);
	// 8267AD20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267AD24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267AD28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267AD2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267AD30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267AD30 size=116
    let mut pc: u32 = 0x8267AD30;
    'dispatch: loop {
        match pc {
            0x8267AD30 => {
    //   block [0x8267AD30..0x8267ADA4)
	// 8267AD30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267AD34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267AD38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267AD3C: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267AD40: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 8267AD44: 390A8468  addi r8, r10, -0x7b98
	ctx.r[8].s64 = ctx.r[10].s64 + -31640;
	// 8267AD48: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267AD4C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8267AD50: 38AA5FE8  addi r5, r10, 0x5fe8
	ctx.r[5].s64 = ctx.r[10].s64 + 24552;
	// 8267AD54: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267AD58: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267AD5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267AD60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267AD64: 388A3210  addi r4, r10, 0x3210
	ctx.r[4].s64 = ctx.r[10].s64 + 12816;
	// 8267AD68: 396B3E84  addi r11, r11, 0x3e84
	ctx.r[11].s64 = ctx.r[11].s64 + 16004;
	// 8267AD6C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267AD70: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267AD74: 386A61C8  addi r3, r10, 0x61c8
	ctx.r[3].s64 = ctx.r[10].s64 + 25032;
	// 8267AD78: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8267AD7C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267AD80: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8267AD84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267AD88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267AD8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267AD90: 4BDEC091  bl 0x82466e20
	ctx.lr = 0x8267AD94;
	sub_82466E20(ctx, base);
	// 8267AD94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267AD98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267AD9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267ADA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267ADA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267ADA8 size=108
    let mut pc: u32 = 0x8267ADA8;
    'dispatch: loop {
        match pc {
            0x8267ADA8 => {
    //   block [0x8267ADA8..0x8267AE14)
	// 8267ADA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267ADAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267ADB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267ADB4: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267ADB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267ADBC: 38EB84C8  addi r7, r11, -0x7b38
	ctx.r[7].s64 = ctx.r[11].s64 + -31544;
	// 8267ADC0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8267ADC4: 388A3224  addi r4, r10, 0x3224
	ctx.r[4].s64 = ctx.r[10].s64 + 12836;
	// 8267ADC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267ADCC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267ADD0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267ADD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267ADD8: 386A61F8  addi r3, r10, 0x61f8
	ctx.r[3].s64 = ctx.r[10].s64 + 25080;
	// 8267ADDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267ADE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267ADE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267ADE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267ADEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267ADF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267ADF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267ADF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267ADFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267AE00: 4BDEC021  bl 0x82466e20
	ctx.lr = 0x8267AE04;
	sub_82466E20(ctx, base);
	// 8267AE04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267AE08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267AE0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267AE10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267AE18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267AE18 size=108
    let mut pc: u32 = 0x8267AE18;
    'dispatch: loop {
        match pc {
            0x8267AE18 => {
    //   block [0x8267AE18..0x8267AE84)
	// 8267AE18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267AE1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267AE20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267AE24: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267AE28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267AE2C: 38EB8510  addi r7, r11, -0x7af0
	ctx.r[7].s64 = ctx.r[11].s64 + -31472;
	// 8267AE30: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8267AE34: 388A3244  addi r4, r10, 0x3244
	ctx.r[4].s64 = ctx.r[10].s64 + 12868;
	// 8267AE38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267AE3C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267AE40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267AE44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267AE48: 386A6228  addi r3, r10, 0x6228
	ctx.r[3].s64 = ctx.r[10].s64 + 25128;
	// 8267AE4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267AE50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267AE54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267AE58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267AE5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267AE60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267AE64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267AE68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267AE6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267AE70: 4BDEBFB1  bl 0x82466e20
	ctx.lr = 0x8267AE74;
	sub_82466E20(ctx, base);
	// 8267AE74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267AE78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267AE7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267AE80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267AE88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267AE88 size=112
    let mut pc: u32 = 0x8267AE88;
    'dispatch: loop {
        match pc {
            0x8267AE88 => {
    //   block [0x8267AE88..0x8267AEF8)
	// 8267AE88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267AE8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267AE90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267AE94: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267AE98: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267AE9C: 38AA5FE8  addi r5, r10, 0x5fe8
	ctx.r[5].s64 = ctx.r[10].s64 + 24552;
	// 8267AEA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267AEA4: 390B8558  addi r8, r11, -0x7aa8
	ctx.r[8].s64 = ctx.r[11].s64 + -31400;
	// 8267AEA8: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 8267AEAC: 388A3260  addi r4, r10, 0x3260
	ctx.r[4].s64 = ctx.r[10].s64 + 12896;
	// 8267AEB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267AEB4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267AEB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267AEBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267AEC0: 386A6258  addi r3, r10, 0x6258
	ctx.r[3].s64 = ctx.r[10].s64 + 25176;
	// 8267AEC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267AEC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267AECC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267AED0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267AED4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267AED8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267AEDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267AEE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267AEE4: 4BDEBF3D  bl 0x82466e20
	ctx.lr = 0x8267AEE8;
	sub_82466E20(ctx, base);
	// 8267AEE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267AEEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267AEF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267AEF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267AEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267AEF8 size=112
    let mut pc: u32 = 0x8267AEF8;
    'dispatch: loop {
        match pc {
            0x8267AEF8 => {
    //   block [0x8267AEF8..0x8267AF68)
	// 8267AEF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267AEFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267AF00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267AF04: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267AF08: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267AF0C: 38AA5FE8  addi r5, r10, 0x5fe8
	ctx.r[5].s64 = ctx.r[10].s64 + 24552;
	// 8267AF10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267AF14: 390B8630  addi r8, r11, -0x79d0
	ctx.r[8].s64 = ctx.r[11].s64 + -31184;
	// 8267AF18: 3920000B  li r9, 0xb
	ctx.r[9].s64 = 11;
	// 8267AF1C: 388A3274  addi r4, r10, 0x3274
	ctx.r[4].s64 = ctx.r[10].s64 + 12916;
	// 8267AF20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267AF24: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267AF28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267AF2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267AF30: 386A6288  addi r3, r10, 0x6288
	ctx.r[3].s64 = ctx.r[10].s64 + 25224;
	// 8267AF34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267AF38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267AF3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267AF40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267AF44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267AF48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267AF4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267AF50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267AF54: 4BDEBECD  bl 0x82466e20
	ctx.lr = 0x8267AF58;
	sub_82466E20(ctx, base);
	// 8267AF58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267AF5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267AF60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267AF64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267AF68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267AF68 size=112
    let mut pc: u32 = 0x8267AF68;
    'dispatch: loop {
        match pc {
            0x8267AF68 => {
    //   block [0x8267AF68..0x8267AFD8)
	// 8267AF68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267AF6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267AF70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267AF74: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267AF78: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267AF7C: 38AA5FE8  addi r5, r10, 0x5fe8
	ctx.r[5].s64 = ctx.r[10].s64 + 24552;
	// 8267AF80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267AF84: 390B8738  addi r8, r11, -0x78c8
	ctx.r[8].s64 = ctx.r[11].s64 + -30920;
	// 8267AF88: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8267AF8C: 388A3288  addi r4, r10, 0x3288
	ctx.r[4].s64 = ctx.r[10].s64 + 12936;
	// 8267AF90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267AF94: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267AF98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267AF9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267AFA0: 386A62B8  addi r3, r10, 0x62b8
	ctx.r[3].s64 = ctx.r[10].s64 + 25272;
	// 8267AFA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267AFA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267AFAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267AFB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267AFB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267AFB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267AFBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267AFC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267AFC4: 4BDEBE5D  bl 0x82466e20
	ctx.lr = 0x8267AFC8;
	sub_82466E20(ctx, base);
	// 8267AFC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267AFCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267AFD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267AFD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267AFD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267AFD8 size=112
    let mut pc: u32 = 0x8267AFD8;
    'dispatch: loop {
        match pc {
            0x8267AFD8 => {
    //   block [0x8267AFD8..0x8267B048)
	// 8267AFD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267AFDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267AFE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267AFE4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267AFE8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267AFEC: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 8267AFF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267AFF4: 390B8750  addi r8, r11, -0x78b0
	ctx.r[8].s64 = ctx.r[11].s64 + -30896;
	// 8267AFF8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8267AFFC: 388A32A4  addi r4, r10, 0x32a4
	ctx.r[4].s64 = ctx.r[10].s64 + 12964;
	// 8267B000: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267B004: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B008: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267B00C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267B010: 386A62E8  addi r3, r10, 0x62e8
	ctx.r[3].s64 = ctx.r[10].s64 + 25320;
	// 8267B014: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267B018: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267B01C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267B020: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267B024: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267B028: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267B02C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267B030: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267B034: 4BDEBDED  bl 0x82466e20
	ctx.lr = 0x8267B038;
	sub_82466E20(ctx, base);
	// 8267B038: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267B03C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267B040: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267B044: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267B048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267B048 size=108
    let mut pc: u32 = 0x8267B048;
    'dispatch: loop {
        match pc {
            0x8267B048 => {
    //   block [0x8267B048..0x8267B0B4)
	// 8267B048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267B04C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267B050: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267B054: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267B058: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267B05C: 38EB8780  addi r7, r11, -0x7880
	ctx.r[7].s64 = ctx.r[11].s64 + -30848;
	// 8267B060: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8267B064: 388A32B4  addi r4, r10, 0x32b4
	ctx.r[4].s64 = ctx.r[10].s64 + 12980;
	// 8267B068: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267B06C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B070: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267B074: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267B078: 386A6318  addi r3, r10, 0x6318
	ctx.r[3].s64 = ctx.r[10].s64 + 25368;
	// 8267B07C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267B080: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267B084: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267B088: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267B08C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267B090: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267B094: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267B098: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267B09C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267B0A0: 4BDEBD81  bl 0x82466e20
	ctx.lr = 0x8267B0A4;
	sub_82466E20(ctx, base);
	// 8267B0A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267B0A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267B0AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267B0B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267B0B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8267B0B8 size=24
    let mut pc: u32 = 0x8267B0B8;
    'dispatch: loop {
        match pc {
            0x8267B0B8 => {
    //   block [0x8267B0B8..0x8267B0D0)
	// 8267B0B8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267B0BC: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267B0C0: 394AE8B8  addi r10, r10, -0x1748
	ctx.r[10].s64 = ctx.r[10].s64 + -5960;
	// 8267B0C4: 816B7FFC  lwz r11, 0x7ffc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32764 as u32) ) } as u64;
	// 8267B0C8: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8267B0CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267B0D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267B0D0 size=116
    let mut pc: u32 = 0x8267B0D0;
    'dispatch: loop {
        match pc {
            0x8267B0D0 => {
    //   block [0x8267B0D0..0x8267B144)
	// 8267B0D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267B0D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267B0D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267B0DC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267B0E0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8267B0E4: 390BE8B8  addi r8, r11, -0x1748
	ctx.r[8].s64 = ctx.r[11].s64 + -5960;
	// 8267B0E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267B0EC: 392A3EB0  addi r9, r10, 0x3eb0
	ctx.r[9].s64 = ctx.r[10].s64 + 16048;
	// 8267B0F0: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B0F4: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 8267B0F8: 38AA5FE8  addi r5, r10, 0x5fe8
	ctx.r[5].s64 = ctx.r[10].s64 + 24552;
	// 8267B0FC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267B100: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267B104: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267B108: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267B10C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267B110: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267B114: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 8267B118: 388A3300  addi r4, r10, 0x3300
	ctx.r[4].s64 = ctx.r[10].s64 + 13056;
	// 8267B11C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267B120: 386B6348  addi r3, r11, 0x6348
	ctx.r[3].s64 = ctx.r[11].s64 + 25416;
	// 8267B124: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8267B128: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267B12C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267B130: 4BDEBCF1  bl 0x82466e20
	ctx.lr = 0x8267B134;
	sub_82466E20(ctx, base);
	// 8267B134: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267B138: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267B13C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267B140: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267B148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267B148 size=112
    let mut pc: u32 = 0x8267B148;
    'dispatch: loop {
        match pc {
            0x8267B148 => {
    //   block [0x8267B148..0x8267B1B8)
	// 8267B148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267B14C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267B150: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267B154: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B158: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267B15C: 38AA5FE8  addi r5, r10, 0x5fe8
	ctx.r[5].s64 = ctx.r[10].s64 + 24552;
	// 8267B160: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267B164: 390B87F8  addi r8, r11, -0x7808
	ctx.r[8].s64 = ctx.r[11].s64 + -30728;
	// 8267B168: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8267B16C: 388A331C  addi r4, r10, 0x331c
	ctx.r[4].s64 = ctx.r[10].s64 + 13084;
	// 8267B170: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267B174: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B178: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267B17C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267B180: 386A6378  addi r3, r10, 0x6378
	ctx.r[3].s64 = ctx.r[10].s64 + 25464;
	// 8267B184: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267B188: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267B18C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267B190: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267B194: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267B198: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267B19C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267B1A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267B1A4: 4BDEBC7D  bl 0x82466e20
	ctx.lr = 0x8267B1A8;
	sub_82466E20(ctx, base);
	// 8267B1A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267B1AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267B1B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267B1B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267B1B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267B1B8 size=116
    let mut pc: u32 = 0x8267B1B8;
    'dispatch: loop {
        match pc {
            0x8267B1B8 => {
    //   block [0x8267B1B8..0x8267B22C)
	// 8267B1B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267B1BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267B1C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267B1C4: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267B1C8: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 8267B1CC: 390A8828  addi r8, r10, -0x77d8
	ctx.r[8].s64 = ctx.r[10].s64 + -30680;
	// 8267B1D0: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B1D4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8267B1D8: 38AA5FE8  addi r5, r10, 0x5fe8
	ctx.r[5].s64 = ctx.r[10].s64 + 24552;
	// 8267B1DC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267B1E0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267B1E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267B1E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267B1EC: 388A3340  addi r4, r10, 0x3340
	ctx.r[4].s64 = ctx.r[10].s64 + 13120;
	// 8267B1F0: 396B3EC4  addi r11, r11, 0x3ec4
	ctx.r[11].s64 = ctx.r[11].s64 + 16068;
	// 8267B1F4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B1F8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267B1FC: 386A63A8  addi r3, r10, 0x63a8
	ctx.r[3].s64 = ctx.r[10].s64 + 25512;
	// 8267B200: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8267B204: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267B208: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8267B20C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267B210: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267B214: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267B218: 4BDEBC09  bl 0x82466e20
	ctx.lr = 0x8267B21C;
	sub_82466E20(ctx, base);
	// 8267B21C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267B220: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267B224: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267B228: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267B230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267B230 size=112
    let mut pc: u32 = 0x8267B230;
    'dispatch: loop {
        match pc {
            0x8267B230 => {
    //   block [0x8267B230..0x8267B2A0)
	// 8267B230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267B234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267B238: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267B23C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B240: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267B244: 38AA5FE8  addi r5, r10, 0x5fe8
	ctx.r[5].s64 = ctx.r[10].s64 + 24552;
	// 8267B248: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267B24C: 390B88E8  addi r8, r11, -0x7718
	ctx.r[8].s64 = ctx.r[11].s64 + -30488;
	// 8267B250: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8267B254: 388A335C  addi r4, r10, 0x335c
	ctx.r[4].s64 = ctx.r[10].s64 + 13148;
	// 8267B258: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267B25C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B260: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267B264: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267B268: 386A63D8  addi r3, r10, 0x63d8
	ctx.r[3].s64 = ctx.r[10].s64 + 25560;
	// 8267B26C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267B270: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267B274: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267B278: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267B27C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267B280: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267B284: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267B288: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267B28C: 4BDEBB95  bl 0x82466e20
	ctx.lr = 0x8267B290;
	sub_82466E20(ctx, base);
	// 8267B290: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267B294: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267B298: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267B29C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267B2A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267B2A0 size=108
    let mut pc: u32 = 0x8267B2A0;
    'dispatch: loop {
        match pc {
            0x8267B2A0 => {
    //   block [0x8267B2A0..0x8267B30C)
	// 8267B2A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267B2A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267B2A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267B2AC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267B2B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267B2B4: 38EB8900  addi r7, r11, -0x7700
	ctx.r[7].s64 = ctx.r[11].s64 + -30464;
	// 8267B2B8: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 8267B2BC: 388A3374  addi r4, r10, 0x3374
	ctx.r[4].s64 = ctx.r[10].s64 + 13172;
	// 8267B2C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267B2C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B2C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267B2CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267B2D0: 386A6408  addi r3, r10, 0x6408
	ctx.r[3].s64 = ctx.r[10].s64 + 25608;
	// 8267B2D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267B2D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267B2DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267B2E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267B2E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267B2E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267B2EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267B2F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267B2F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267B2F8: 4BDEBB29  bl 0x82466e20
	ctx.lr = 0x8267B2FC;
	sub_82466E20(ctx, base);
	// 8267B2FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267B300: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267B304: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267B308: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267B310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267B310 size=116
    let mut pc: u32 = 0x8267B310;
    'dispatch: loop {
        match pc {
            0x8267B310 => {
    //   block [0x8267B310..0x8267B384)
	// 8267B310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267B314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267B318: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267B31C: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267B320: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8267B324: 390A8A38  addi r8, r10, -0x75c8
	ctx.r[8].s64 = ctx.r[10].s64 + -30152;
	// 8267B328: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B32C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8267B330: 38AA5FE8  addi r5, r10, 0x5fe8
	ctx.r[5].s64 = ctx.r[10].s64 + 24552;
	// 8267B334: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267B338: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267B33C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267B340: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267B344: 388A3394  addi r4, r10, 0x3394
	ctx.r[4].s64 = ctx.r[10].s64 + 13204;
	// 8267B348: 396B3EE8  addi r11, r11, 0x3ee8
	ctx.r[11].s64 = ctx.r[11].s64 + 16104;
	// 8267B34C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B350: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267B354: 386A6438  addi r3, r10, 0x6438
	ctx.r[3].s64 = ctx.r[10].s64 + 25656;
	// 8267B358: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8267B35C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267B360: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8267B364: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267B368: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267B36C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267B370: 4BDEBAB1  bl 0x82466e20
	ctx.lr = 0x8267B374;
	sub_82466E20(ctx, base);
	// 8267B374: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267B378: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267B37C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267B380: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267B388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267B388 size=112
    let mut pc: u32 = 0x8267B388;
    'dispatch: loop {
        match pc {
            0x8267B388 => {
    //   block [0x8267B388..0x8267B3F8)
	// 8267B388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267B38C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267B390: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267B394: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B398: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267B39C: 38AA5FE8  addi r5, r10, 0x5fe8
	ctx.r[5].s64 = ctx.r[10].s64 + 24552;
	// 8267B3A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267B3A4: 390B8AE0  addi r8, r11, -0x7520
	ctx.r[8].s64 = ctx.r[11].s64 + -29984;
	// 8267B3A8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8267B3AC: 388A33B0  addi r4, r10, 0x33b0
	ctx.r[4].s64 = ctx.r[10].s64 + 13232;
	// 8267B3B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267B3B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B3B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267B3BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267B3C0: 386A6468  addi r3, r10, 0x6468
	ctx.r[3].s64 = ctx.r[10].s64 + 25704;
	// 8267B3C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267B3C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267B3CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267B3D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267B3D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267B3D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267B3DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267B3E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267B3E4: 4BDEBA3D  bl 0x82466e20
	ctx.lr = 0x8267B3E8;
	sub_82466E20(ctx, base);
	// 8267B3E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267B3EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267B3F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267B3F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267B3F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267B3F8 size=112
    let mut pc: u32 = 0x8267B3F8;
    'dispatch: loop {
        match pc {
            0x8267B3F8 => {
    //   block [0x8267B3F8..0x8267B468)
	// 8267B3F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267B3FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267B400: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267B404: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B408: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267B40C: 38AA5FE8  addi r5, r10, 0x5fe8
	ctx.r[5].s64 = ctx.r[10].s64 + 24552;
	// 8267B410: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267B414: 390B8AF8  addi r8, r11, -0x7508
	ctx.r[8].s64 = ctx.r[11].s64 + -29960;
	// 8267B418: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 8267B41C: 388A33D4  addi r4, r10, 0x33d4
	ctx.r[4].s64 = ctx.r[10].s64 + 13268;
	// 8267B420: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267B424: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B428: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267B42C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267B430: 386A6498  addi r3, r10, 0x6498
	ctx.r[3].s64 = ctx.r[10].s64 + 25752;
	// 8267B434: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267B438: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267B43C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267B440: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267B444: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267B448: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267B44C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267B450: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267B454: 4BDEB9CD  bl 0x82466e20
	ctx.lr = 0x8267B458;
	sub_82466E20(ctx, base);
	// 8267B458: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267B45C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267B460: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267B464: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267B468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267B468 size=112
    let mut pc: u32 = 0x8267B468;
    'dispatch: loop {
        match pc {
            0x8267B468 => {
    //   block [0x8267B468..0x8267B4D8)
	// 8267B468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267B46C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267B470: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267B474: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B478: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267B47C: 38AA5FE8  addi r5, r10, 0x5fe8
	ctx.r[5].s64 = ctx.r[10].s64 + 24552;
	// 8267B480: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267B484: 390B8C18  addi r8, r11, -0x73e8
	ctx.r[8].s64 = ctx.r[11].s64 + -29672;
	// 8267B488: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8267B48C: 388A33E8  addi r4, r10, 0x33e8
	ctx.r[4].s64 = ctx.r[10].s64 + 13288;
	// 8267B490: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267B494: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B498: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267B49C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267B4A0: 386A64C8  addi r3, r10, 0x64c8
	ctx.r[3].s64 = ctx.r[10].s64 + 25800;
	// 8267B4A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267B4A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267B4AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267B4B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267B4B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267B4B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267B4BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267B4C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267B4C4: 4BDEB95D  bl 0x82466e20
	ctx.lr = 0x8267B4C8;
	sub_82466E20(ctx, base);
	// 8267B4C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267B4CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267B4D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267B4D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267B4D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267B4D8 size=116
    let mut pc: u32 = 0x8267B4D8;
    'dispatch: loop {
        match pc {
            0x8267B4D8 => {
    //   block [0x8267B4D8..0x8267B54C)
	// 8267B4D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267B4DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267B4E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267B4E4: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267B4E8: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8267B4EC: 390B8C34  addi r8, r11, -0x73cc
	ctx.r[8].s64 = ctx.r[11].s64 + -29644;
	// 8267B4F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267B4F4: 392A3F20  addi r9, r10, 0x3f20
	ctx.r[9].s64 = ctx.r[10].s64 + 16160;
	// 8267B4F8: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B4FC: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 8267B500: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 8267B504: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267B508: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267B50C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267B510: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267B514: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267B518: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267B51C: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 8267B520: 388A33FC  addi r4, r10, 0x33fc
	ctx.r[4].s64 = ctx.r[10].s64 + 13308;
	// 8267B524: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267B528: 386B64F8  addi r3, r11, 0x64f8
	ctx.r[3].s64 = ctx.r[11].s64 + 25848;
	// 8267B52C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8267B530: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267B534: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267B538: 4BDEB8E9  bl 0x82466e20
	ctx.lr = 0x8267B53C;
	sub_82466E20(ctx, base);
	// 8267B53C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267B540: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267B544: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267B548: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267B550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267B550 size=100
    let mut pc: u32 = 0x8267B550;
    'dispatch: loop {
        match pc {
            0x8267B550 => {
    //   block [0x8267B550..0x8267B5B4)
	// 8267B550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267B554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267B558: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267B55C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B560: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267B564: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 8267B568: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267B56C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267B570: 388A3404  addi r4, r10, 0x3404
	ctx.r[4].s64 = ctx.r[10].s64 + 13316;
	// 8267B574: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B578: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267B57C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267B580: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267B584: 386A6528  addi r3, r10, 0x6528
	ctx.r[3].s64 = ctx.r[10].s64 + 25896;
	// 8267B588: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267B58C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267B590: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8267B594: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267B598: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267B59C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267B5A0: 4BDEB881  bl 0x82466e20
	ctx.lr = 0x8267B5A4;
	sub_82466E20(ctx, base);
	// 8267B5A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267B5A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267B5AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267B5B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267B5B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267B5B8 size=112
    let mut pc: u32 = 0x8267B5B8;
    'dispatch: loop {
        match pc {
            0x8267B5B8 => {
    //   block [0x8267B5B8..0x8267B628)
	// 8267B5B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267B5BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267B5C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267B5C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B5C8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267B5CC: 38AA6528  addi r5, r10, 0x6528
	ctx.r[5].s64 = ctx.r[10].s64 + 25896;
	// 8267B5D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267B5D4: 390B8C64  addi r8, r11, -0x739c
	ctx.r[8].s64 = ctx.r[11].s64 + -29596;
	// 8267B5D8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8267B5DC: 388A3414  addi r4, r10, 0x3414
	ctx.r[4].s64 = ctx.r[10].s64 + 13332;
	// 8267B5E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267B5E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B5E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267B5EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267B5F0: 386A6558  addi r3, r10, 0x6558
	ctx.r[3].s64 = ctx.r[10].s64 + 25944;
	// 8267B5F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267B5F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267B5FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267B600: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267B604: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267B608: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267B60C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267B610: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267B614: 4BDEB80D  bl 0x82466e20
	ctx.lr = 0x8267B618;
	sub_82466E20(ctx, base);
	// 8267B618: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267B61C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267B620: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267B624: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267B628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267B628 size=112
    let mut pc: u32 = 0x8267B628;
    'dispatch: loop {
        match pc {
            0x8267B628 => {
    //   block [0x8267B628..0x8267B698)
	// 8267B628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267B62C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267B630: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267B634: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B638: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267B63C: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 8267B640: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267B644: 390B8C80  addi r8, r11, -0x7380
	ctx.r[8].s64 = ctx.r[11].s64 + -29568;
	// 8267B648: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8267B64C: 388A3428  addi r4, r10, 0x3428
	ctx.r[4].s64 = ctx.r[10].s64 + 13352;
	// 8267B650: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267B654: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B658: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267B65C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267B660: 386A6588  addi r3, r10, 0x6588
	ctx.r[3].s64 = ctx.r[10].s64 + 25992;
	// 8267B664: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267B668: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267B66C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267B670: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267B674: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267B678: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267B67C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267B680: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267B684: 4BDEB79D  bl 0x82466e20
	ctx.lr = 0x8267B688;
	sub_82466E20(ctx, base);
	// 8267B688: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267B68C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267B690: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267B694: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267B698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267B698 size=112
    let mut pc: u32 = 0x8267B698;
    'dispatch: loop {
        match pc {
            0x8267B698 => {
    //   block [0x8267B698..0x8267B708)
	// 8267B698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267B69C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267B6A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267B6A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B6A8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267B6AC: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 8267B6B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267B6B4: 390B8CE0  addi r8, r11, -0x7320
	ctx.r[8].s64 = ctx.r[11].s64 + -29472;
	// 8267B6B8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8267B6BC: 388A3440  addi r4, r10, 0x3440
	ctx.r[4].s64 = ctx.r[10].s64 + 13376;
	// 8267B6C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267B6C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B6C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267B6CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267B6D0: 386A65B8  addi r3, r10, 0x65b8
	ctx.r[3].s64 = ctx.r[10].s64 + 26040;
	// 8267B6D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267B6D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267B6DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267B6E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267B6E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267B6E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267B6EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267B6F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267B6F4: 4BDEB72D  bl 0x82466e20
	ctx.lr = 0x8267B6F8;
	sub_82466E20(ctx, base);
	// 8267B6F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267B6FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267B700: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267B704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267B708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267B708 size=112
    let mut pc: u32 = 0x8267B708;
    'dispatch: loop {
        match pc {
            0x8267B708 => {
    //   block [0x8267B708..0x8267B778)
	// 8267B708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267B70C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267B710: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267B714: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B718: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267B71C: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 8267B720: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267B724: 390B8D28  addi r8, r11, -0x72d8
	ctx.r[8].s64 = ctx.r[11].s64 + -29400;
	// 8267B728: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8267B72C: 388A3450  addi r4, r10, 0x3450
	ctx.r[4].s64 = ctx.r[10].s64 + 13392;
	// 8267B730: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267B734: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B738: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267B73C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267B740: 386A65E8  addi r3, r10, 0x65e8
	ctx.r[3].s64 = ctx.r[10].s64 + 26088;
	// 8267B744: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267B748: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267B74C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267B750: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267B754: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267B758: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267B75C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267B760: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267B764: 4BDEB6BD  bl 0x82466e20
	ctx.lr = 0x8267B768;
	sub_82466E20(ctx, base);
	// 8267B768: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267B76C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267B770: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267B774: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267B778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267B778 size=112
    let mut pc: u32 = 0x8267B778;
    'dispatch: loop {
        match pc {
            0x8267B778 => {
    //   block [0x8267B778..0x8267B7E8)
	// 8267B778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267B77C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267B780: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267B784: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B788: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267B78C: 38AA5FE8  addi r5, r10, 0x5fe8
	ctx.r[5].s64 = ctx.r[10].s64 + 24552;
	// 8267B790: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267B794: 390B8D58  addi r8, r11, -0x72a8
	ctx.r[8].s64 = ctx.r[11].s64 + -29352;
	// 8267B798: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8267B79C: 388A3468  addi r4, r10, 0x3468
	ctx.r[4].s64 = ctx.r[10].s64 + 13416;
	// 8267B7A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267B7A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B7A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267B7AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267B7B0: 386A6618  addi r3, r10, 0x6618
	ctx.r[3].s64 = ctx.r[10].s64 + 26136;
	// 8267B7B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267B7B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267B7BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267B7C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267B7C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267B7C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267B7CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267B7D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267B7D4: 4BDEB64D  bl 0x82466e20
	ctx.lr = 0x8267B7D8;
	sub_82466E20(ctx, base);
	// 8267B7D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267B7DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267B7E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267B7E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267B7E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267B7E8 size=100
    let mut pc: u32 = 0x8267B7E8;
    'dispatch: loop {
        match pc {
            0x8267B7E8 => {
    //   block [0x8267B7E8..0x8267B84C)
	// 8267B7E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267B7EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267B7F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267B7F4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B7F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267B7FC: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 8267B800: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267B804: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267B808: 388A3474  addi r4, r10, 0x3474
	ctx.r[4].s64 = ctx.r[10].s64 + 13428;
	// 8267B80C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B810: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267B814: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267B818: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267B81C: 386A6648  addi r3, r10, 0x6648
	ctx.r[3].s64 = ctx.r[10].s64 + 26184;
	// 8267B820: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267B824: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267B828: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8267B82C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267B830: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267B834: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267B838: 4BDEB5E9  bl 0x82466e20
	ctx.lr = 0x8267B83C;
	sub_82466E20(ctx, base);
	// 8267B83C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267B840: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267B844: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267B848: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267B850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267B850 size=108
    let mut pc: u32 = 0x8267B850;
    'dispatch: loop {
        match pc {
            0x8267B850 => {
    //   block [0x8267B850..0x8267B8BC)
	// 8267B850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267B854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267B858: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267B85C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267B860: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267B864: 38EB8DD0  addi r7, r11, -0x7230
	ctx.r[7].s64 = ctx.r[11].s64 + -29232;
	// 8267B868: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8267B86C: 388A3488  addi r4, r10, 0x3488
	ctx.r[4].s64 = ctx.r[10].s64 + 13448;
	// 8267B870: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267B874: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B878: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267B87C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267B880: 386A6678  addi r3, r10, 0x6678
	ctx.r[3].s64 = ctx.r[10].s64 + 26232;
	// 8267B884: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267B888: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267B88C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267B890: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267B894: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267B898: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267B89C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267B8A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267B8A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267B8A8: 4BDEB579  bl 0x82466e20
	ctx.lr = 0x8267B8AC;
	sub_82466E20(ctx, base);
	// 8267B8AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267B8B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267B8B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267B8B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267B8C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267B8C0 size=112
    let mut pc: u32 = 0x8267B8C0;
    'dispatch: loop {
        match pc {
            0x8267B8C0 => {
    //   block [0x8267B8C0..0x8267B930)
	// 8267B8C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267B8C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267B8C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267B8CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B8D0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267B8D4: 38AA6648  addi r5, r10, 0x6648
	ctx.r[5].s64 = ctx.r[10].s64 + 26184;
	// 8267B8D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267B8DC: 390B8E00  addi r8, r11, -0x7200
	ctx.r[8].s64 = ctx.r[11].s64 + -29184;
	// 8267B8E0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8267B8E4: 388A34AC  addi r4, r10, 0x34ac
	ctx.r[4].s64 = ctx.r[10].s64 + 13484;
	// 8267B8E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267B8EC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B8F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267B8F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267B8F8: 386A66A8  addi r3, r10, 0x66a8
	ctx.r[3].s64 = ctx.r[10].s64 + 26280;
	// 8267B8FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267B900: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267B904: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267B908: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267B90C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267B910: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267B914: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267B918: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267B91C: 4BDEB505  bl 0x82466e20
	ctx.lr = 0x8267B920;
	sub_82466E20(ctx, base);
	// 8267B920: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267B924: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267B928: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267B92C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267B930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267B930 size=108
    let mut pc: u32 = 0x8267B930;
    'dispatch: loop {
        match pc {
            0x8267B930 => {
    //   block [0x8267B930..0x8267B99C)
	// 8267B930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267B934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267B938: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267B93C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267B940: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267B944: 38EB8E30  addi r7, r11, -0x71d0
	ctx.r[7].s64 = ctx.r[11].s64 + -29136;
	// 8267B948: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8267B94C: 388A34C4  addi r4, r10, 0x34c4
	ctx.r[4].s64 = ctx.r[10].s64 + 13508;
	// 8267B950: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267B954: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B958: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267B95C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267B960: 386A66D8  addi r3, r10, 0x66d8
	ctx.r[3].s64 = ctx.r[10].s64 + 26328;
	// 8267B964: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267B968: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267B96C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267B970: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267B974: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267B978: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267B97C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267B980: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267B984: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267B988: 4BDEB499  bl 0x82466e20
	ctx.lr = 0x8267B98C;
	sub_82466E20(ctx, base);
	// 8267B98C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267B990: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267B994: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267B998: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267B9A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267B9A0 size=112
    let mut pc: u32 = 0x8267B9A0;
    'dispatch: loop {
        match pc {
            0x8267B9A0 => {
    //   block [0x8267B9A0..0x8267BA10)
	// 8267B9A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267B9A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267B9A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267B9AC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B9B0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267B9B4: 38AA6648  addi r5, r10, 0x6648
	ctx.r[5].s64 = ctx.r[10].s64 + 26184;
	// 8267B9B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267B9BC: 390B8E60  addi r8, r11, -0x71a0
	ctx.r[8].s64 = ctx.r[11].s64 + -29088;
	// 8267B9C0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8267B9C4: 388A34F4  addi r4, r10, 0x34f4
	ctx.r[4].s64 = ctx.r[10].s64 + 13556;
	// 8267B9C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267B9CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267B9D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267B9D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267B9D8: 386A6708  addi r3, r10, 0x6708
	ctx.r[3].s64 = ctx.r[10].s64 + 26376;
	// 8267B9DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267B9E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267B9E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267B9E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267B9EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267B9F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267B9F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267B9F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267B9FC: 4BDEB425  bl 0x82466e20
	ctx.lr = 0x8267BA00;
	sub_82466E20(ctx, base);
	// 8267BA00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267BA04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267BA08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267BA0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267BA10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267BA10 size=108
    let mut pc: u32 = 0x8267BA10;
    'dispatch: loop {
        match pc {
            0x8267BA10 => {
    //   block [0x8267BA10..0x8267BA7C)
	// 8267BA10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267BA14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267BA18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267BA1C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267BA20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267BA24: 38EB8EA8  addi r7, r11, -0x7158
	ctx.r[7].s64 = ctx.r[11].s64 + -29016;
	// 8267BA28: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8267BA2C: 388A3514  addi r4, r10, 0x3514
	ctx.r[4].s64 = ctx.r[10].s64 + 13588;
	// 8267BA30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267BA34: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267BA38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267BA3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267BA40: 386A6738  addi r3, r10, 0x6738
	ctx.r[3].s64 = ctx.r[10].s64 + 26424;
	// 8267BA44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267BA48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267BA4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267BA50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267BA54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267BA58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267BA5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267BA60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267BA64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267BA68: 4BDEB3B9  bl 0x82466e20
	ctx.lr = 0x8267BA6C;
	sub_82466E20(ctx, base);
	// 8267BA6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267BA70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267BA74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267BA78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267BA80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267BA80 size=112
    let mut pc: u32 = 0x8267BA80;
    'dispatch: loop {
        match pc {
            0x8267BA80 => {
    //   block [0x8267BA80..0x8267BAF0)
	// 8267BA80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267BA84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267BA88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267BA8C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267BA90: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267BA94: 38AA6648  addi r5, r10, 0x6648
	ctx.r[5].s64 = ctx.r[10].s64 + 26184;
	// 8267BA98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267BA9C: 390B8ED8  addi r8, r11, -0x7128
	ctx.r[8].s64 = ctx.r[11].s64 + -28968;
	// 8267BAA0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8267BAA4: 388A3544  addi r4, r10, 0x3544
	ctx.r[4].s64 = ctx.r[10].s64 + 13636;
	// 8267BAA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267BAAC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267BAB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267BAB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267BAB8: 386A6768  addi r3, r10, 0x6768
	ctx.r[3].s64 = ctx.r[10].s64 + 26472;
	// 8267BABC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267BAC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267BAC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267BAC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267BACC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267BAD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267BAD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267BAD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267BADC: 4BDEB345  bl 0x82466e20
	ctx.lr = 0x8267BAE0;
	sub_82466E20(ctx, base);
	// 8267BAE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267BAE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267BAE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267BAEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267BAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267BAF0 size=108
    let mut pc: u32 = 0x8267BAF0;
    'dispatch: loop {
        match pc {
            0x8267BAF0 => {
    //   block [0x8267BAF0..0x8267BB5C)
	// 8267BAF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267BAF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267BAF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267BAFC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267BB00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267BB04: 38EB8F20  addi r7, r11, -0x70e0
	ctx.r[7].s64 = ctx.r[11].s64 + -28896;
	// 8267BB08: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8267BB0C: 388A3564  addi r4, r10, 0x3564
	ctx.r[4].s64 = ctx.r[10].s64 + 13668;
	// 8267BB10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267BB14: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267BB18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267BB1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267BB20: 386A6798  addi r3, r10, 0x6798
	ctx.r[3].s64 = ctx.r[10].s64 + 26520;
	// 8267BB24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267BB28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267BB2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267BB30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267BB34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267BB38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267BB3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267BB40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267BB44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267BB48: 4BDEB2D9  bl 0x82466e20
	ctx.lr = 0x8267BB4C;
	sub_82466E20(ctx, base);
	// 8267BB4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267BB50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267BB54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267BB58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267BB60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267BB60 size=112
    let mut pc: u32 = 0x8267BB60;
    'dispatch: loop {
        match pc {
            0x8267BB60 => {
    //   block [0x8267BB60..0x8267BBD0)
	// 8267BB60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267BB64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267BB68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267BB6C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267BB70: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267BB74: 38AA6648  addi r5, r10, 0x6648
	ctx.r[5].s64 = ctx.r[10].s64 + 26184;
	// 8267BB78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267BB7C: 390B8F50  addi r8, r11, -0x70b0
	ctx.r[8].s64 = ctx.r[11].s64 + -28848;
	// 8267BB80: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8267BB84: 388A3594  addi r4, r10, 0x3594
	ctx.r[4].s64 = ctx.r[10].s64 + 13716;
	// 8267BB88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267BB8C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267BB90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267BB94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267BB98: 386A67C8  addi r3, r10, 0x67c8
	ctx.r[3].s64 = ctx.r[10].s64 + 26568;
	// 8267BB9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267BBA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267BBA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267BBA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267BBAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267BBB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267BBB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267BBB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267BBBC: 4BDEB265  bl 0x82466e20
	ctx.lr = 0x8267BBC0;
	sub_82466E20(ctx, base);
	// 8267BBC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267BBC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267BBC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267BBCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267BBD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267BBD0 size=108
    let mut pc: u32 = 0x8267BBD0;
    'dispatch: loop {
        match pc {
            0x8267BBD0 => {
    //   block [0x8267BBD0..0x8267BC3C)
	// 8267BBD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267BBD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267BBD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267BBDC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267BBE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267BBE4: 38EB8F98  addi r7, r11, -0x7068
	ctx.r[7].s64 = ctx.r[11].s64 + -28776;
	// 8267BBE8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8267BBEC: 388A35B0  addi r4, r10, 0x35b0
	ctx.r[4].s64 = ctx.r[10].s64 + 13744;
	// 8267BBF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267BBF4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267BBF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267BBFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267BC00: 386A67F8  addi r3, r10, 0x67f8
	ctx.r[3].s64 = ctx.r[10].s64 + 26616;
	// 8267BC04: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267BC08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267BC0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267BC10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267BC14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267BC18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267BC1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267BC20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267BC24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267BC28: 4BDEB1F9  bl 0x82466e20
	ctx.lr = 0x8267BC2C;
	sub_82466E20(ctx, base);
	// 8267BC2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267BC30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267BC34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267BC38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267BC40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267BC40 size=112
    let mut pc: u32 = 0x8267BC40;
    'dispatch: loop {
        match pc {
            0x8267BC40 => {
    //   block [0x8267BC40..0x8267BCB0)
	// 8267BC40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267BC44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267BC48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267BC4C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8267BC50: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267BC54: 392A3F90  addi r9, r10, 0x3f90
	ctx.r[9].s64 = ctx.r[10].s64 + 16272;
	// 8267BC58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267BC5C: 390B8FF8  addi r8, r11, -0x7008
	ctx.r[8].s64 = ctx.r[11].s64 + -28680;
	// 8267BC60: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 8267BC64: 388A35CC  addi r4, r10, 0x35cc
	ctx.r[4].s64 = ctx.r[10].s64 + 13772;
	// 8267BC68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267BC6C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267BC70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267BC74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267BC78: 386A6828  addi r3, r10, 0x6828
	ctx.r[3].s64 = ctx.r[10].s64 + 26664;
	// 8267BC7C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267BC80: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8267BC84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267BC88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267BC8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267BC90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267BC94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267BC98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267BC9C: 4BDEB185  bl 0x82466e20
	ctx.lr = 0x8267BCA0;
	sub_82466E20(ctx, base);
	// 8267BCA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267BCA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267BCA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267BCAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267BCB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267BCB0 size=108
    let mut pc: u32 = 0x8267BCB0;
    'dispatch: loop {
        match pc {
            0x8267BCB0 => {
    //   block [0x8267BCB0..0x8267BD1C)
	// 8267BCB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267BCB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267BCB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267BCBC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267BCC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267BCC4: 38EB90B8  addi r7, r11, -0x6f48
	ctx.r[7].s64 = ctx.r[11].s64 + -28488;
	// 8267BCC8: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8267BCCC: 388A3610  addi r4, r10, 0x3610
	ctx.r[4].s64 = ctx.r[10].s64 + 13840;
	// 8267BCD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267BCD4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267BCD8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267BCDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267BCE0: 386A6858  addi r3, r10, 0x6858
	ctx.r[3].s64 = ctx.r[10].s64 + 26712;
	// 8267BCE4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267BCE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267BCEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267BCF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267BCF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267BCF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267BCFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267BD00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267BD04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267BD08: 4BDEB119  bl 0x82466e20
	ctx.lr = 0x8267BD0C;
	sub_82466E20(ctx, base);
	// 8267BD0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267BD10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267BD14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267BD18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267BD20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267BD20 size=116
    let mut pc: u32 = 0x8267BD20;
    'dispatch: loop {
        match pc {
            0x8267BD20 => {
    //   block [0x8267BD20..0x8267BD94)
	// 8267BD20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267BD24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267BD28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267BD2C: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267BD30: 38E00011  li r7, 0x11
	ctx.r[7].s64 = 17;
	// 8267BD34: 390A9130  addi r8, r10, -0x6ed0
	ctx.r[8].s64 = ctx.r[10].s64 + -28368;
	// 8267BD38: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267BD3C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8267BD40: 38AA5DD8  addi r5, r10, 0x5dd8
	ctx.r[5].s64 = ctx.r[10].s64 + 24024;
	// 8267BD44: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267BD48: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267BD4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267BD50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267BD54: 388A362C  addi r4, r10, 0x362c
	ctx.r[4].s64 = ctx.r[10].s64 + 13868;
	// 8267BD58: 396B3FA8  addi r11, r11, 0x3fa8
	ctx.r[11].s64 = ctx.r[11].s64 + 16296;
	// 8267BD5C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267BD60: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267BD64: 386A6888  addi r3, r10, 0x6888
	ctx.r[3].s64 = ctx.r[10].s64 + 26760;
	// 8267BD68: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8267BD6C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267BD70: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8267BD74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267BD78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267BD7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267BD80: 4BDEB0A1  bl 0x82466e20
	ctx.lr = 0x8267BD84;
	sub_82466E20(ctx, base);
	// 8267BD84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267BD88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267BD8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267BD90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267BD98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267BD98 size=100
    let mut pc: u32 = 0x8267BD98;
    'dispatch: loop {
        match pc {
            0x8267BD98 => {
    //   block [0x8267BD98..0x8267BDFC)
	// 8267BD98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267BD9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267BDA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267BDA4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267BDA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267BDAC: 38AA5DD8  addi r5, r10, 0x5dd8
	ctx.r[5].s64 = ctx.r[10].s64 + 24024;
	// 8267BDB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267BDB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267BDB8: 388A363C  addi r4, r10, 0x363c
	ctx.r[4].s64 = ctx.r[10].s64 + 13884;
	// 8267BDBC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267BDC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267BDC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267BDC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267BDCC: 386A68B8  addi r3, r10, 0x68b8
	ctx.r[3].s64 = ctx.r[10].s64 + 26808;
	// 8267BDD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267BDD4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267BDD8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8267BDDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267BDE0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267BDE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267BDE8: 4BDEB039  bl 0x82466e20
	ctx.lr = 0x8267BDEC;
	sub_82466E20(ctx, base);
	// 8267BDEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267BDF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267BDF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267BDF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267BE00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8267BE00 size=24
    let mut pc: u32 = 0x8267BE00;
    'dispatch: loop {
        match pc {
            0x8267BE00 => {
    //   block [0x8267BE00..0x8267BE18)
	// 8267BE00: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267BE04: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267BE08: 394AE9C0  addi r10, r10, -0x1640
	ctx.r[10].s64 = ctx.r[10].s64 + -5696;
	// 8267BE0C: 816B92CC  lwz r11, -0x6d34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27956 as u32) ) } as u64;
	// 8267BE10: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8267BE14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267BE18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267BE18 size=116
    let mut pc: u32 = 0x8267BE18;
    'dispatch: loop {
        match pc {
            0x8267BE18 => {
    //   block [0x8267BE18..0x8267BE8C)
	// 8267BE18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267BE1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267BE20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267BE24: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267BE28: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8267BE2C: 390BE9C0  addi r8, r11, -0x1640
	ctx.r[8].s64 = ctx.r[11].s64 + -5696;
	// 8267BE30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267BE34: 392A4020  addi r9, r10, 0x4020
	ctx.r[9].s64 = ctx.r[10].s64 + 16416;
	// 8267BE38: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267BE3C: 38E0000C  li r7, 0xc
	ctx.r[7].s64 = 12;
	// 8267BE40: 38AA68B8  addi r5, r10, 0x68b8
	ctx.r[5].s64 = ctx.r[10].s64 + 26808;
	// 8267BE44: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267BE48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267BE4C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267BE50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267BE54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267BE58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267BE5C: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 8267BE60: 388A3690  addi r4, r10, 0x3690
	ctx.r[4].s64 = ctx.r[10].s64 + 13968;
	// 8267BE64: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267BE68: 386B68E8  addi r3, r11, 0x68e8
	ctx.r[3].s64 = ctx.r[11].s64 + 26856;
	// 8267BE6C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8267BE70: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267BE74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267BE78: 4BDEAFA9  bl 0x82466e20
	ctx.lr = 0x8267BE7C;
	sub_82466E20(ctx, base);
	// 8267BE7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267BE80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267BE84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267BE88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267BE90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267BE90 size=108
    let mut pc: u32 = 0x8267BE90;
    'dispatch: loop {
        match pc {
            0x8267BE90 => {
    //   block [0x8267BE90..0x8267BEFC)
	// 8267BE90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267BE94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267BE98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267BE9C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267BEA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267BEA4: 38EB92D0  addi r7, r11, -0x6d30
	ctx.r[7].s64 = ctx.r[11].s64 + -27952;
	// 8267BEA8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8267BEAC: 388A3714  addi r4, r10, 0x3714
	ctx.r[4].s64 = ctx.r[10].s64 + 14100;
	// 8267BEB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267BEB4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267BEB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267BEBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267BEC0: 386A6918  addi r3, r10, 0x6918
	ctx.r[3].s64 = ctx.r[10].s64 + 26904;
	// 8267BEC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267BEC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267BECC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267BED0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267BED4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267BED8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267BEDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267BEE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267BEE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267BEE8: 4BDEAF39  bl 0x82466e20
	ctx.lr = 0x8267BEEC;
	sub_82466E20(ctx, base);
	// 8267BEEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267BEF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267BEF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267BEF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267BF00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8267BF00 size=24
    let mut pc: u32 = 0x8267BF00;
    'dispatch: loop {
        match pc {
            0x8267BF00 => {
    //   block [0x8267BF00..0x8267BF18)
	// 8267BF00: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267BF04: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267BF08: 394AEAE0  addi r10, r10, -0x1520
	ctx.r[10].s64 = ctx.r[10].s64 + -5408;
	// 8267BF0C: 816B92E8  lwz r11, -0x6d18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27928 as u32) ) } as u64;
	// 8267BF10: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8267BF14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267BF18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267BF18 size=112
    let mut pc: u32 = 0x8267BF18;
    'dispatch: loop {
        match pc {
            0x8267BF18 => {
    //   block [0x8267BF18..0x8267BF88)
	// 8267BF18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267BF1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267BF20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267BF24: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8267BF28: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267BF2C: 392A4078  addi r9, r10, 0x4078
	ctx.r[9].s64 = ctx.r[10].s64 + 16504;
	// 8267BF30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267BF34: 390BEAE0  addi r8, r11, -0x1520
	ctx.r[8].s64 = ctx.r[11].s64 + -5408;
	// 8267BF38: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 8267BF3C: 388A3728  addi r4, r10, 0x3728
	ctx.r[4].s64 = ctx.r[10].s64 + 14120;
	// 8267BF40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267BF44: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267BF48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267BF4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267BF50: 386A6948  addi r3, r10, 0x6948
	ctx.r[3].s64 = ctx.r[10].s64 + 26952;
	// 8267BF54: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267BF58: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8267BF5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267BF60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267BF64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267BF68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267BF6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267BF70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267BF74: 4BDEAEAD  bl 0x82466e20
	ctx.lr = 0x8267BF78;
	sub_82466E20(ctx, base);
	// 8267BF78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267BF7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267BF80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267BF84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267BF88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267BF88 size=108
    let mut pc: u32 = 0x8267BF88;
    'dispatch: loop {
        match pc {
            0x8267BF88 => {
    //   block [0x8267BF88..0x8267BFF4)
	// 8267BF88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267BF8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267BF90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267BF94: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267BF98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267BF9C: 38EB92F0  addi r7, r11, -0x6d10
	ctx.r[7].s64 = ctx.r[11].s64 + -27920;
	// 8267BFA0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8267BFA4: 388A3738  addi r4, r10, 0x3738
	ctx.r[4].s64 = ctx.r[10].s64 + 14136;
	// 8267BFA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267BFAC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267BFB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267BFB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267BFB8: 386A6978  addi r3, r10, 0x6978
	ctx.r[3].s64 = ctx.r[10].s64 + 27000;
	// 8267BFBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267BFC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267BFC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267BFC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267BFCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267BFD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267BFD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267BFD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267BFDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267BFE0: 4BDEAE41  bl 0x82466e20
	ctx.lr = 0x8267BFE4;
	sub_82466E20(ctx, base);
	// 8267BFE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267BFE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267BFEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267BFF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267BFF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267BFF8 size=112
    let mut pc: u32 = 0x8267BFF8;
    'dispatch: loop {
        match pc {
            0x8267BFF8 => {
    //   block [0x8267BFF8..0x8267C068)
	// 8267BFF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267BFFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267C000: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267C004: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267C008: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267C00C: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 8267C010: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267C014: 390B9368  addi r8, r11, -0x6c98
	ctx.r[8].s64 = ctx.r[11].s64 + -27800;
	// 8267C018: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8267C01C: 388A3758  addi r4, r10, 0x3758
	ctx.r[4].s64 = ctx.r[10].s64 + 14168;
	// 8267C020: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267C024: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267C028: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267C02C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267C030: 386A69A8  addi r3, r10, 0x69a8
	ctx.r[3].s64 = ctx.r[10].s64 + 27048;
	// 8267C034: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267C038: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267C03C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267C040: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267C044: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267C048: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267C04C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267C050: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267C054: 4BDEADCD  bl 0x82466e20
	ctx.lr = 0x8267C058;
	sub_82466E20(ctx, base);
	// 8267C058: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267C05C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267C060: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267C064: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267C068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267C068 size=108
    let mut pc: u32 = 0x8267C068;
    'dispatch: loop {
        match pc {
            0x8267C068 => {
    //   block [0x8267C068..0x8267C0D4)
	// 8267C068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267C06C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267C070: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267C074: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267C078: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267C07C: 38EB9380  addi r7, r11, -0x6c80
	ctx.r[7].s64 = ctx.r[11].s64 + -27776;
	// 8267C080: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8267C084: 388A3770  addi r4, r10, 0x3770
	ctx.r[4].s64 = ctx.r[10].s64 + 14192;
	// 8267C088: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267C08C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267C090: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267C094: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267C098: 386A69D8  addi r3, r10, 0x69d8
	ctx.r[3].s64 = ctx.r[10].s64 + 27096;
	// 8267C09C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267C0A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267C0A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267C0A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267C0AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267C0B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267C0B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267C0B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267C0BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267C0C0: 4BDEAD61  bl 0x82466e20
	ctx.lr = 0x8267C0C4;
	sub_82466E20(ctx, base);
	// 8267C0C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267C0C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267C0CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267C0D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267C0D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267C0D8 size=108
    let mut pc: u32 = 0x8267C0D8;
    'dispatch: loop {
        match pc {
            0x8267C0D8 => {
    //   block [0x8267C0D8..0x8267C144)
	// 8267C0D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267C0DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267C0E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267C0E4: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267C0E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267C0EC: 38EB93E0  addi r7, r11, -0x6c20
	ctx.r[7].s64 = ctx.r[11].s64 + -27680;
	// 8267C0F0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8267C0F4: 388A3788  addi r4, r10, 0x3788
	ctx.r[4].s64 = ctx.r[10].s64 + 14216;
	// 8267C0F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267C0FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267C100: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267C104: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267C108: 386A6A08  addi r3, r10, 0x6a08
	ctx.r[3].s64 = ctx.r[10].s64 + 27144;
	// 8267C10C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267C110: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267C114: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267C118: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267C11C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267C120: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267C124: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267C128: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267C12C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267C130: 4BDEACF1  bl 0x82466e20
	ctx.lr = 0x8267C134;
	sub_82466E20(ctx, base);
	// 8267C134: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267C138: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267C13C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267C140: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267C148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267C148 size=116
    let mut pc: u32 = 0x8267C148;
    'dispatch: loop {
        match pc {
            0x8267C148 => {
    //   block [0x8267C148..0x8267C1BC)
	// 8267C148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267C14C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267C150: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267C154: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267C158: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8267C15C: 390B9410  addi r8, r11, -0x6bf0
	ctx.r[8].s64 = ctx.r[11].s64 + -27632;
	// 8267C160: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267C164: 392A40A4  addi r9, r10, 0x40a4
	ctx.r[9].s64 = ctx.r[10].s64 + 16548;
	// 8267C168: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267C16C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8267C170: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 8267C174: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267C178: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267C17C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267C180: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267C184: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267C188: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267C18C: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 8267C190: 388A37A0  addi r4, r10, 0x37a0
	ctx.r[4].s64 = ctx.r[10].s64 + 14240;
	// 8267C194: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267C198: 386B6A38  addi r3, r11, 0x6a38
	ctx.r[3].s64 = ctx.r[11].s64 + 27192;
	// 8267C19C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8267C1A0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267C1A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267C1A8: 4BDEAC79  bl 0x82466e20
	ctx.lr = 0x8267C1AC;
	sub_82466E20(ctx, base);
	// 8267C1AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267C1B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267C1B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267C1B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267C1C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267C1C0 size=96
    let mut pc: u32 = 0x8267C1C0;
    'dispatch: loop {
        match pc {
            0x8267C1C0 => {
    //   block [0x8267C1C0..0x8267C220)
	// 8267C1C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267C1C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267C1C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267C1CC: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8267C1D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267C1D4: 388A6DD4  addi r4, r10, 0x6dd4
	ctx.r[4].s64 = ctx.r[10].s64 + 28116;
	// 8267C1D8: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267C1DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267C1E0: 386A6A68  addi r3, r10, 0x6a68
	ctx.r[3].s64 = ctx.r[10].s64 + 27240;
	// 8267C1E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267C1E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267C1EC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8267C1F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267C1F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267C1F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267C1FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267C200: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8267C204: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267C208: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267C20C: 4BDEAC15  bl 0x82466e20
	ctx.lr = 0x8267C210;
	sub_82466E20(ctx, base);
	// 8267C210: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267C214: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267C218: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267C21C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267C220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267C220 size=112
    let mut pc: u32 = 0x8267C220;
    'dispatch: loop {
        match pc {
            0x8267C220 => {
    //   block [0x8267C220..0x8267C290)
	// 8267C220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267C224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267C228: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267C22C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267C230: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267C234: 38AA6A68  addi r5, r10, 0x6a68
	ctx.r[5].s64 = ctx.r[10].s64 + 27240;
	// 8267C238: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8267C23C: 390B9428  addi r8, r11, -0x6bd8
	ctx.r[8].s64 = ctx.r[11].s64 + -27608;
	// 8267C240: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8267C244: 388A7380  addi r4, r10, 0x7380
	ctx.r[4].s64 = ctx.r[10].s64 + 29568;
	// 8267C248: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267C24C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267C250: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267C254: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267C258: 386A6A98  addi r3, r10, 0x6a98
	ctx.r[3].s64 = ctx.r[10].s64 + 27288;
	// 8267C25C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267C260: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267C264: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267C268: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267C26C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267C270: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267C274: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267C278: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267C27C: 4BDEABA5  bl 0x82466e20
	ctx.lr = 0x8267C280;
	sub_82466E20(ctx, base);
	// 8267C280: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267C284: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267C288: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267C28C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267C290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267C290 size=112
    let mut pc: u32 = 0x8267C290;
    'dispatch: loop {
        match pc {
            0x8267C290 => {
    //   block [0x8267C290..0x8267C300)
	// 8267C290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267C294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267C298: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267C29C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8267C2A0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267C2A4: 392A40C0  addi r9, r10, 0x40c0
	ctx.r[9].s64 = ctx.r[10].s64 + 16576;
	// 8267C2A8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8267C2AC: 390B9460  addi r8, r11, -0x6ba0
	ctx.r[8].s64 = ctx.r[11].s64 + -27552;
	// 8267C2B0: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8267C2B4: 388A7468  addi r4, r10, 0x7468
	ctx.r[4].s64 = ctx.r[10].s64 + 29800;
	// 8267C2B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267C2BC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267C2C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267C2C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267C2C8: 386A6AC8  addi r3, r10, 0x6ac8
	ctx.r[3].s64 = ctx.r[10].s64 + 27336;
	// 8267C2CC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267C2D0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8267C2D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267C2D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267C2DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267C2E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267C2E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267C2E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267C2EC: 4BDEAB35  bl 0x82466e20
	ctx.lr = 0x8267C2F0;
	sub_82466E20(ctx, base);
	// 8267C2F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267C2F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267C2F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267C2FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267C300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267C300 size=108
    let mut pc: u32 = 0x8267C300;
    'dispatch: loop {
        match pc {
            0x8267C300 => {
    //   block [0x8267C300..0x8267C36C)
	// 8267C300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267C304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267C308: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267C30C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267C310: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8267C314: 38EB9508  addi r7, r11, -0x6af8
	ctx.r[7].s64 = ctx.r[11].s64 + -27384;
	// 8267C318: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8267C31C: 388A7888  addi r4, r10, 0x7888
	ctx.r[4].s64 = ctx.r[10].s64 + 30856;
	// 8267C320: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267C324: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267C328: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267C32C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267C330: 386A6AF8  addi r3, r10, 0x6af8
	ctx.r[3].s64 = ctx.r[10].s64 + 27384;
	// 8267C334: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267C338: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267C33C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267C340: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267C344: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267C348: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267C34C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267C350: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267C354: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267C358: 4BDEAAC9  bl 0x82466e20
	ctx.lr = 0x8267C35C;
	sub_82466E20(ctx, base);
	// 8267C35C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267C360: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267C364: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267C368: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267C370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267C370 size=108
    let mut pc: u32 = 0x8267C370;
    'dispatch: loop {
        match pc {
            0x8267C370 => {
    //   block [0x8267C370..0x8267C3DC)
	// 8267C370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267C374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267C378: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267C37C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267C380: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8267C384: 38EB9538  addi r7, r11, -0x6ac8
	ctx.r[7].s64 = ctx.r[11].s64 + -27336;
	// 8267C388: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8267C38C: 388A7898  addi r4, r10, 0x7898
	ctx.r[4].s64 = ctx.r[10].s64 + 30872;
	// 8267C390: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267C394: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267C398: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267C39C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267C3A0: 386A6B28  addi r3, r10, 0x6b28
	ctx.r[3].s64 = ctx.r[10].s64 + 27432;
	// 8267C3A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267C3A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267C3AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267C3B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267C3B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267C3B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267C3BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267C3C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267C3C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267C3C8: 4BDEAA59  bl 0x82466e20
	ctx.lr = 0x8267C3CC;
	sub_82466E20(ctx, base);
	// 8267C3CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267C3D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267C3D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267C3D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267C3E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8267C3E0 size=28
    let mut pc: u32 = 0x8267C3E0;
    'dispatch: loop {
        match pc {
            0x8267C3E0 => {
    //   block [0x8267C3E0..0x8267C3FC)
	// 8267C3E0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267C3E4: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267C3E8: 394AEB10  addi r10, r10, -0x14f0
	ctx.r[10].s64 = ctx.r[10].s64 + -5360;
	// 8267C3EC: 816B945C  lwz r11, -0x6ba4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27556 as u32) ) } as u64;
	// 8267C3F0: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8267C3F4: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8267C3F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267C400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267C400 size=112
    let mut pc: u32 = 0x8267C400;
    'dispatch: loop {
        match pc {
            0x8267C400 => {
    //   block [0x8267C400..0x8267C470)
	// 8267C400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267C404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267C408: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267C40C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8267C410: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267C414: 392A4240  addi r9, r10, 0x4240
	ctx.r[9].s64 = ctx.r[10].s64 + 16960;
	// 8267C418: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8267C41C: 390BEB10  addi r8, r11, -0x14f0
	ctx.r[8].s64 = ctx.r[11].s64 + -5360;
	// 8267C420: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 8267C424: 388A72F4  addi r4, r10, 0x72f4
	ctx.r[4].s64 = ctx.r[10].s64 + 29428;
	// 8267C428: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267C42C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267C430: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267C434: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267C438: 386A6B58  addi r3, r10, 0x6b58
	ctx.r[3].s64 = ctx.r[10].s64 + 27480;
	// 8267C43C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267C440: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8267C444: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267C448: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267C44C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267C450: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267C454: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267C458: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267C45C: 4BDEA9C5  bl 0x82466e20
	ctx.lr = 0x8267C460;
	sub_82466E20(ctx, base);
	// 8267C460: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267C464: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267C468: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267C46C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267C470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267C470 size=108
    let mut pc: u32 = 0x8267C470;
    'dispatch: loop {
        match pc {
            0x8267C470 => {
    //   block [0x8267C470..0x8267C4DC)
	// 8267C470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267C474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267C478: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267C47C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267C480: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8267C484: 38EB9570  addi r7, r11, -0x6a90
	ctx.r[7].s64 = ctx.r[11].s64 + -27280;
	// 8267C488: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8267C48C: 388A7914  addi r4, r10, 0x7914
	ctx.r[4].s64 = ctx.r[10].s64 + 30996;
	// 8267C490: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267C494: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267C498: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267C49C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267C4A0: 386A6B88  addi r3, r10, 0x6b88
	ctx.r[3].s64 = ctx.r[10].s64 + 27528;
	// 8267C4A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267C4A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267C4AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267C4B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267C4B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267C4B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267C4BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267C4C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267C4C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267C4C8: 4BDEA959  bl 0x82466e20
	ctx.lr = 0x8267C4CC;
	sub_82466E20(ctx, base);
	// 8267C4CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267C4D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267C4D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267C4D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267C4E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267C4E0 size=108
    let mut pc: u32 = 0x8267C4E0;
    'dispatch: loop {
        match pc {
            0x8267C4E0 => {
    //   block [0x8267C4E0..0x8267C54C)
	// 8267C4E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267C4E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267C4E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267C4EC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267C4F0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8267C4F4: 38EB95A0  addi r7, r11, -0x6a60
	ctx.r[7].s64 = ctx.r[11].s64 + -27232;
	// 8267C4F8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8267C4FC: 388A7788  addi r4, r10, 0x7788
	ctx.r[4].s64 = ctx.r[10].s64 + 30600;
	// 8267C500: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267C504: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267C508: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267C50C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267C510: 386A6BB8  addi r3, r10, 0x6bb8
	ctx.r[3].s64 = ctx.r[10].s64 + 27576;
	// 8267C514: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267C518: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267C51C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267C520: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267C524: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267C528: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267C52C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267C530: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267C534: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267C538: 4BDEA8E9  bl 0x82466e20
	ctx.lr = 0x8267C53C;
	sub_82466E20(ctx, base);
	// 8267C53C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267C540: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267C544: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267C548: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267C550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267C550 size=108
    let mut pc: u32 = 0x8267C550;
    'dispatch: loop {
        match pc {
            0x8267C550 => {
    //   block [0x8267C550..0x8267C5BC)
	// 8267C550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267C554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267C558: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267C55C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267C560: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8267C564: 38EB95D0  addi r7, r11, -0x6a30
	ctx.r[7].s64 = ctx.r[11].s64 + -27184;
	// 8267C568: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8267C56C: 388A77AC  addi r4, r10, 0x77ac
	ctx.r[4].s64 = ctx.r[10].s64 + 30636;
	// 8267C570: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267C574: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267C578: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267C57C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267C580: 386A6BE8  addi r3, r10, 0x6be8
	ctx.r[3].s64 = ctx.r[10].s64 + 27624;
	// 8267C584: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267C588: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267C58C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267C590: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267C594: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267C598: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267C59C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267C5A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267C5A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267C5A8: 4BDEA879  bl 0x82466e20
	ctx.lr = 0x8267C5AC;
	sub_82466E20(ctx, base);
	// 8267C5AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267C5B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267C5B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267C5B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267C5C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8267C5C0 size=24
    let mut pc: u32 = 0x8267C5C0;
    'dispatch: loop {
        match pc {
            0x8267C5C0 => {
    //   block [0x8267C5C0..0x8267C5D8)
	// 8267C5C0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267C5C4: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267C5C8: 394AEBD0  addi r10, r10, -0x1430
	ctx.r[10].s64 = ctx.r[10].s64 + -5168;
	// 8267C5CC: 816B95E8  lwz r11, -0x6a18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27160 as u32) ) } as u64;
	// 8267C5D0: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8267C5D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267C5D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267C5D8 size=112
    let mut pc: u32 = 0x8267C5D8;
    'dispatch: loop {
        match pc {
            0x8267C5D8 => {
    //   block [0x8267C5D8..0x8267C648)
	// 8267C5D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267C5DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267C5E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267C5E4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8267C5E8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267C5EC: 392A4294  addi r9, r10, 0x4294
	ctx.r[9].s64 = ctx.r[10].s64 + 17044;
	// 8267C5F0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8267C5F4: 390BEBD0  addi r8, r11, -0x1430
	ctx.r[8].s64 = ctx.r[11].s64 + -5168;
	// 8267C5F8: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 8267C5FC: 388A77C8  addi r4, r10, 0x77c8
	ctx.r[4].s64 = ctx.r[10].s64 + 30664;
	// 8267C600: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267C604: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267C608: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267C60C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267C610: 386A6C18  addi r3, r10, 0x6c18
	ctx.r[3].s64 = ctx.r[10].s64 + 27672;
	// 8267C614: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267C618: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8267C61C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267C620: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267C624: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267C628: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267C62C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267C630: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267C634: 4BDEA7ED  bl 0x82466e20
	ctx.lr = 0x8267C638;
	sub_82466E20(ctx, base);
	// 8267C638: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267C63C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267C640: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267C644: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267C648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267C648 size=112
    let mut pc: u32 = 0x8267C648;
    'dispatch: loop {
        match pc {
            0x8267C648 => {
    //   block [0x8267C648..0x8267C6B8)
	// 8267C648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267C64C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267C650: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267C654: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8267C658: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267C65C: 392A42D0  addi r9, r10, 0x42d0
	ctx.r[9].s64 = ctx.r[10].s64 + 17104;
	// 8267C660: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267C664: 390B95F8  addi r8, r11, -0x6a08
	ctx.r[8].s64 = ctx.r[11].s64 + -27144;
	// 8267C668: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8267C66C: 388A37B0  addi r4, r10, 0x37b0
	ctx.r[4].s64 = ctx.r[10].s64 + 14256;
	// 8267C670: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267C674: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267C678: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267C67C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267C680: 386A6C48  addi r3, r10, 0x6c48
	ctx.r[3].s64 = ctx.r[10].s64 + 27720;
	// 8267C684: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267C688: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8267C68C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267C690: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267C694: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267C698: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267C69C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267C6A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267C6A4: 4BDEA77D  bl 0x82466e20
	ctx.lr = 0x8267C6A8;
	sub_82466E20(ctx, base);
	// 8267C6A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267C6AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267C6B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267C6B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267C6B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267C6B8 size=108
    let mut pc: u32 = 0x8267C6B8;
    'dispatch: loop {
        match pc {
            0x8267C6B8 => {
    //   block [0x8267C6B8..0x8267C724)
	// 8267C6B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267C6BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267C6C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267C6C4: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267C6C8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8267C6CC: 38EB9640  addi r7, r11, -0x69c0
	ctx.r[7].s64 = ctx.r[11].s64 + -27072;
	// 8267C6D0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8267C6D4: 388A7BB0  addi r4, r10, 0x7bb0
	ctx.r[4].s64 = ctx.r[10].s64 + 31664;
	// 8267C6D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267C6DC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267C6E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267C6E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267C6E8: 386A6C78  addi r3, r10, 0x6c78
	ctx.r[3].s64 = ctx.r[10].s64 + 27768;
	// 8267C6EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267C6F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267C6F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267C6F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267C6FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267C700: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267C704: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267C708: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267C70C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267C710: 4BDEA711  bl 0x82466e20
	ctx.lr = 0x8267C714;
	sub_82466E20(ctx, base);
	// 8267C714: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267C718: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267C71C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267C720: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267C728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267C728 size=108
    let mut pc: u32 = 0x8267C728;
    'dispatch: loop {
        match pc {
            0x8267C728 => {
    //   block [0x8267C728..0x8267C794)
	// 8267C728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267C72C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267C730: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267C734: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267C738: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8267C73C: 38EB9670  addi r7, r11, -0x6990
	ctx.r[7].s64 = ctx.r[11].s64 + -27024;
	// 8267C740: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8267C744: 388A7F88  addi r4, r10, 0x7f88
	ctx.r[4].s64 = ctx.r[10].s64 + 32648;
	// 8267C748: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267C74C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267C750: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267C754: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267C758: 386A6CA8  addi r3, r10, 0x6ca8
	ctx.r[3].s64 = ctx.r[10].s64 + 27816;
	// 8267C75C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267C760: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267C764: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267C768: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267C76C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267C770: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267C774: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267C778: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267C77C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267C780: 4BDEA6A1  bl 0x82466e20
	ctx.lr = 0x8267C784;
	sub_82466E20(ctx, base);
	// 8267C784: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267C788: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267C78C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267C790: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267C798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267C798 size=112
    let mut pc: u32 = 0x8267C798;
    'dispatch: loop {
        match pc {
            0x8267C798 => {
    //   block [0x8267C798..0x8267C808)
	// 8267C798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267C79C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267C7A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267C7A4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8267C7A8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267C7AC: 392A4308  addi r9, r10, 0x4308
	ctx.r[9].s64 = ctx.r[10].s64 + 17160;
	// 8267C7B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267C7B4: 390B96A0  addi r8, r11, -0x6960
	ctx.r[8].s64 = ctx.r[11].s64 + -26976;
	// 8267C7B8: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 8267C7BC: 388A82E8  addi r4, r10, -0x7d18
	ctx.r[4].s64 = ctx.r[10].s64 + -32024;
	// 8267C7C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267C7C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267C7C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267C7CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267C7D0: 386A6CD8  addi r3, r10, 0x6cd8
	ctx.r[3].s64 = ctx.r[10].s64 + 27864;
	// 8267C7D4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267C7D8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8267C7DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267C7E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267C7E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267C7E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267C7EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267C7F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267C7F4: 4BDEA62D  bl 0x82466e20
	ctx.lr = 0x8267C7F8;
	sub_82466E20(ctx, base);
	// 8267C7F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267C7FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267C800: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267C804: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267C808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267C808 size=108
    let mut pc: u32 = 0x8267C808;
    'dispatch: loop {
        match pc {
            0x8267C808 => {
    //   block [0x8267C808..0x8267C874)
	// 8267C808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267C80C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267C810: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267C814: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267C818: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8267C81C: 38EB9700  addi r7, r11, -0x6900
	ctx.r[7].s64 = ctx.r[11].s64 + -26880;
	// 8267C820: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 8267C824: 388A7F28  addi r4, r10, 0x7f28
	ctx.r[4].s64 = ctx.r[10].s64 + 32552;
	// 8267C828: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267C82C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267C830: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267C834: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267C838: 386A6D08  addi r3, r10, 0x6d08
	ctx.r[3].s64 = ctx.r[10].s64 + 27912;
	// 8267C83C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267C840: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267C844: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267C848: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267C84C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267C850: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267C854: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267C858: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267C85C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267C860: 4BDEA5C1  bl 0x82466e20
	ctx.lr = 0x8267C864;
	sub_82466E20(ctx, base);
	// 8267C864: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267C868: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267C86C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267C870: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267C878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267C878 size=108
    let mut pc: u32 = 0x8267C878;
    'dispatch: loop {
        match pc {
            0x8267C878 => {
    //   block [0x8267C878..0x8267C8E4)
	// 8267C878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267C87C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267C880: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267C884: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267C888: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8267C88C: 38EB9808  addi r7, r11, -0x67f8
	ctx.r[7].s64 = ctx.r[11].s64 + -26616;
	// 8267C890: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8267C894: 388A78C0  addi r4, r10, 0x78c0
	ctx.r[4].s64 = ctx.r[10].s64 + 30912;
	// 8267C898: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267C89C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267C8A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267C8A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267C8A8: 386A6D38  addi r3, r10, 0x6d38
	ctx.r[3].s64 = ctx.r[10].s64 + 27960;
	// 8267C8AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267C8B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267C8B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267C8B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267C8BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267C8C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267C8C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267C8C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267C8CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267C8D0: 4BDEA551  bl 0x82466e20
	ctx.lr = 0x8267C8D4;
	sub_82466E20(ctx, base);
	// 8267C8D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267C8D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267C8DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267C8E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267C8E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267C8E8 size=108
    let mut pc: u32 = 0x8267C8E8;
    'dispatch: loop {
        match pc {
            0x8267C8E8 => {
    //   block [0x8267C8E8..0x8267C954)
	// 8267C8E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267C8EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267C8F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267C8F4: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267C8F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267C8FC: 38EB9820  addi r7, r11, -0x67e0
	ctx.r[7].s64 = ctx.r[11].s64 + -26592;
	// 8267C900: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8267C904: 388A8060  addi r4, r10, -0x7fa0
	ctx.r[4].s64 = ctx.r[10].s64 + -32672;
	// 8267C908: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267C90C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267C910: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267C914: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267C918: 386A6D68  addi r3, r10, 0x6d68
	ctx.r[3].s64 = ctx.r[10].s64 + 28008;
	// 8267C91C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267C920: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267C924: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267C928: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267C92C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267C930: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267C934: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267C938: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267C93C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267C940: 4BDEA4E1  bl 0x82466e20
	ctx.lr = 0x8267C944;
	sub_82466E20(ctx, base);
	// 8267C944: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267C948: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267C94C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267C950: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267C958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8267C958 size=24
    let mut pc: u32 = 0x8267C958;
    'dispatch: loop {
        match pc {
            0x8267C958 => {
    //   block [0x8267C958..0x8267C970)
	// 8267C958: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267C95C: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267C960: 394AECA8  addi r10, r10, -0x1358
	ctx.r[10].s64 = ctx.r[10].s64 + -4952;
	// 8267C964: 816B98B0  lwz r11, -0x6750(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26448 as u32) ) } as u64;
	// 8267C968: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8267C96C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267C970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267C970 size=108
    let mut pc: u32 = 0x8267C970;
    'dispatch: loop {
        match pc {
            0x8267C970 => {
    //   block [0x8267C970..0x8267C9DC)
	// 8267C970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267C974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267C978: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267C97C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267C980: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267C984: 38EBECA8  addi r7, r11, -0x1358
	ctx.r[7].s64 = ctx.r[11].s64 + -4952;
	// 8267C988: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8267C98C: 388AB020  addi r4, r10, -0x4fe0
	ctx.r[4].s64 = ctx.r[10].s64 + -20448;
	// 8267C990: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267C994: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267C998: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267C99C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267C9A0: 386A6D98  addi r3, r10, 0x6d98
	ctx.r[3].s64 = ctx.r[10].s64 + 28056;
	// 8267C9A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267C9A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267C9AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267C9B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267C9B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267C9B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267C9BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267C9C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267C9C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267C9C8: 4BDEA459  bl 0x82466e20
	ctx.lr = 0x8267C9CC;
	sub_82466E20(ctx, base);
	// 8267C9CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267C9D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267C9D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267C9D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267C9E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8267C9E0 size=24
    let mut pc: u32 = 0x8267C9E0;
    'dispatch: loop {
        match pc {
            0x8267C9E0 => {
    //   block [0x8267C9E0..0x8267C9F8)
	// 8267C9E0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267C9E4: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267C9E8: 394AECD8  addi r10, r10, -0x1328
	ctx.r[10].s64 = ctx.r[10].s64 + -4904;
	// 8267C9EC: 816B98B0  lwz r11, -0x6750(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26448 as u32) ) } as u64;
	// 8267C9F0: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8267C9F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267C9F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267C9F8 size=108
    let mut pc: u32 = 0x8267C9F8;
    'dispatch: loop {
        match pc {
            0x8267C9F8 => {
    //   block [0x8267C9F8..0x8267CA64)
	// 8267C9F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267C9FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267CA00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267CA04: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267CA08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267CA0C: 38EBECD8  addi r7, r11, -0x1328
	ctx.r[7].s64 = ctx.r[11].s64 + -4904;
	// 8267CA10: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8267CA14: 388A9C48  addi r4, r10, -0x63b8
	ctx.r[4].s64 = ctx.r[10].s64 + -25528;
	// 8267CA18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267CA1C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267CA20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267CA24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267CA28: 386A6DC8  addi r3, r10, 0x6dc8
	ctx.r[3].s64 = ctx.r[10].s64 + 28104;
	// 8267CA2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267CA30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267CA34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267CA38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267CA3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267CA40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267CA44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267CA48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267CA4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267CA50: 4BDEA3D1  bl 0x82466e20
	ctx.lr = 0x8267CA54;
	sub_82466E20(ctx, base);
	// 8267CA54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267CA58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267CA5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267CA60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267CA68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267CA68 size=108
    let mut pc: u32 = 0x8267CA68;
    'dispatch: loop {
        match pc {
            0x8267CA68 => {
    //   block [0x8267CA68..0x8267CAD4)
	// 8267CA68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267CA6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267CA70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267CA74: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267CA78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267CA7C: 38EB9898  addi r7, r11, -0x6768
	ctx.r[7].s64 = ctx.r[11].s64 + -26472;
	// 8267CA80: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8267CA84: 388AA080  addi r4, r10, -0x5f80
	ctx.r[4].s64 = ctx.r[10].s64 + -24448;
	// 8267CA88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267CA8C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267CA90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267CA94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267CA98: 386A6DF8  addi r3, r10, 0x6df8
	ctx.r[3].s64 = ctx.r[10].s64 + 28152;
	// 8267CA9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267CAA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267CAA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267CAA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267CAAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267CAB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267CAB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267CAB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267CABC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267CAC0: 4BDEA361  bl 0x82466e20
	ctx.lr = 0x8267CAC4;
	sub_82466E20(ctx, base);
	// 8267CAC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267CAC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267CACC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267CAD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267CAD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8267CAD8 size=24
    let mut pc: u32 = 0x8267CAD8;
    'dispatch: loop {
        match pc {
            0x8267CAD8 => {
    //   block [0x8267CAD8..0x8267CAF0)
	// 8267CAD8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267CADC: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267CAE0: 394AED08  addi r10, r10, -0x12f8
	ctx.r[10].s64 = ctx.r[10].s64 + -4856;
	// 8267CAE4: 816B98B0  lwz r11, -0x6750(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26448 as u32) ) } as u64;
	// 8267CAE8: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8267CAEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267CAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267CAF0 size=108
    let mut pc: u32 = 0x8267CAF0;
    'dispatch: loop {
        match pc {
            0x8267CAF0 => {
    //   block [0x8267CAF0..0x8267CB5C)
	// 8267CAF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267CAF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267CAF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267CAFC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267CB00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267CB04: 38EBED08  addi r7, r11, -0x12f8
	ctx.r[7].s64 = ctx.r[11].s64 + -4856;
	// 8267CB08: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8267CB0C: 388A9BE8  addi r4, r10, -0x6418
	ctx.r[4].s64 = ctx.r[10].s64 + -25624;
	// 8267CB10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267CB14: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267CB18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267CB1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267CB20: 386A6E28  addi r3, r10, 0x6e28
	ctx.r[3].s64 = ctx.r[10].s64 + 28200;
	// 8267CB24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267CB28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267CB2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267CB30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267CB34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267CB38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267CB3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267CB40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267CB44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267CB48: 4BDEA2D9  bl 0x82466e20
	ctx.lr = 0x8267CB4C;
	sub_82466E20(ctx, base);
	// 8267CB4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267CB50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267CB54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267CB58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267CB60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267CB60 size=112
    let mut pc: u32 = 0x8267CB60;
    'dispatch: loop {
        match pc {
            0x8267CB60 => {
    //   block [0x8267CB60..0x8267CBD0)
	// 8267CB60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267CB64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267CB68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267CB6C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8267CB70: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267CB74: 392A434C  addi r9, r10, 0x434c
	ctx.r[9].s64 = ctx.r[10].s64 + 17228;
	// 8267CB78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267CB7C: 390B98B4  addi r8, r11, -0x674c
	ctx.r[8].s64 = ctx.r[11].s64 + -26444;
	// 8267CB80: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 8267CB84: 388AA778  addi r4, r10, -0x5888
	ctx.r[4].s64 = ctx.r[10].s64 + -22664;
	// 8267CB88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267CB8C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267CB90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267CB94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267CB98: 386A6E58  addi r3, r10, 0x6e58
	ctx.r[3].s64 = ctx.r[10].s64 + 28248;
	// 8267CB9C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267CBA0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8267CBA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267CBA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267CBAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267CBB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267CBB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267CBB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267CBBC: 4BDEA265  bl 0x82466e20
	ctx.lr = 0x8267CBC0;
	sub_82466E20(ctx, base);
	// 8267CBC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267CBC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267CBC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267CBCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267CBD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267CBD0 size=108
    let mut pc: u32 = 0x8267CBD0;
    'dispatch: loop {
        match pc {
            0x8267CBD0 => {
    //   block [0x8267CBD0..0x8267CC3C)
	// 8267CBD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267CBD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267CBD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267CBDC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267CBE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267CBE4: 38EB98E4  addi r7, r11, -0x671c
	ctx.r[7].s64 = ctx.r[11].s64 + -26396;
	// 8267CBE8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8267CBEC: 388AA350  addi r4, r10, -0x5cb0
	ctx.r[4].s64 = ctx.r[10].s64 + -23728;
	// 8267CBF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267CBF4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267CBF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267CBFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267CC00: 386A6E88  addi r3, r10, 0x6e88
	ctx.r[3].s64 = ctx.r[10].s64 + 28296;
	// 8267CC04: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267CC08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267CC0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267CC10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267CC14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267CC18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267CC1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267CC20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267CC24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267CC28: 4BDEA1F9  bl 0x82466e20
	ctx.lr = 0x8267CC2C;
	sub_82466E20(ctx, base);
	// 8267CC2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267CC30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267CC34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267CC38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267CC40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267CC40 size=108
    let mut pc: u32 = 0x8267CC40;
    'dispatch: loop {
        match pc {
            0x8267CC40 => {
    //   block [0x8267CC40..0x8267CCAC)
	// 8267CC40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267CC44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267CC48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267CC4C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267CC50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267CC54: 38EB9914  addi r7, r11, -0x66ec
	ctx.r[7].s64 = ctx.r[11].s64 + -26348;
	// 8267CC58: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8267CC5C: 388AB2D8  addi r4, r10, -0x4d28
	ctx.r[4].s64 = ctx.r[10].s64 + -19752;
	// 8267CC60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267CC64: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267CC68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267CC6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267CC70: 386A6EB8  addi r3, r10, 0x6eb8
	ctx.r[3].s64 = ctx.r[10].s64 + 28344;
	// 8267CC74: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267CC78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267CC7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267CC80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267CC84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267CC88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267CC8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267CC90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267CC94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267CC98: 4BDEA189  bl 0x82466e20
	ctx.lr = 0x8267CC9C;
	sub_82466E20(ctx, base);
	// 8267CC9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267CCA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267CCA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267CCA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267CCB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267CCB0 size=108
    let mut pc: u32 = 0x8267CCB0;
    'dispatch: loop {
        match pc {
            0x8267CCB0 => {
    //   block [0x8267CCB0..0x8267CD1C)
	// 8267CCB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267CCB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267CCB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267CCBC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267CCC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267CCC4: 38EB992C  addi r7, r11, -0x66d4
	ctx.r[7].s64 = ctx.r[11].s64 + -26324;
	// 8267CCC8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8267CCCC: 388AAA44  addi r4, r10, -0x55bc
	ctx.r[4].s64 = ctx.r[10].s64 + -21948;
	// 8267CCD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267CCD4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267CCD8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267CCDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267CCE0: 386A6EE8  addi r3, r10, 0x6ee8
	ctx.r[3].s64 = ctx.r[10].s64 + 28392;
	// 8267CCE4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267CCE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267CCEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267CCF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267CCF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267CCF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267CCFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267CD00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267CD04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267CD08: 4BDEA119  bl 0x82466e20
	ctx.lr = 0x8267CD0C;
	sub_82466E20(ctx, base);
	// 8267CD0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267CD10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267CD14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267CD18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267CD20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267CD20 size=112
    let mut pc: u32 = 0x8267CD20;
    'dispatch: loop {
        match pc {
            0x8267CD20 => {
    //   block [0x8267CD20..0x8267CD90)
	// 8267CD20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267CD24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267CD28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267CD2C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267CD30: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267CD34: 38AA6F48  addi r5, r10, 0x6f48
	ctx.r[5].s64 = ctx.r[10].s64 + 28488;
	// 8267CD38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267CD3C: 390B995C  addi r8, r11, -0x66a4
	ctx.r[8].s64 = ctx.r[11].s64 + -26276;
	// 8267CD40: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8267CD44: 388AA9A0  addi r4, r10, -0x5660
	ctx.r[4].s64 = ctx.r[10].s64 + -22112;
	// 8267CD48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267CD4C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267CD50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267CD54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267CD58: 386A6F18  addi r3, r10, 0x6f18
	ctx.r[3].s64 = ctx.r[10].s64 + 28440;
	// 8267CD5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267CD60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267CD64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267CD68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267CD6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267CD70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267CD74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267CD78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267CD7C: 4BDEA0A5  bl 0x82466e20
	ctx.lr = 0x8267CD80;
	sub_82466E20(ctx, base);
	// 8267CD80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267CD84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267CD88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267CD8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267CD90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267CD90 size=108
    let mut pc: u32 = 0x8267CD90;
    'dispatch: loop {
        match pc {
            0x8267CD90 => {
    //   block [0x8267CD90..0x8267CDFC)
	// 8267CD90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267CD94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267CD98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267CD9C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267CDA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267CDA4: 38EB9974  addi r7, r11, -0x668c
	ctx.r[7].s64 = ctx.r[11].s64 + -26252;
	// 8267CDA8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8267CDAC: 388AA640  addi r4, r10, -0x59c0
	ctx.r[4].s64 = ctx.r[10].s64 + -22976;
	// 8267CDB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267CDB4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267CDB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267CDBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267CDC0: 386A6F48  addi r3, r10, 0x6f48
	ctx.r[3].s64 = ctx.r[10].s64 + 28488;
	// 8267CDC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267CDC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267CDCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267CDD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267CDD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267CDD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267CDDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267CDE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267CDE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267CDE8: 4BDEA039  bl 0x82466e20
	ctx.lr = 0x8267CDEC;
	sub_82466E20(ctx, base);
	// 8267CDEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267CDF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267CDF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267CDF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267CE00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267CE00 size=108
    let mut pc: u32 = 0x8267CE00;
    'dispatch: loop {
        match pc {
            0x8267CE00 => {
    //   block [0x8267CE00..0x8267CE6C)
	// 8267CE00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267CE04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267CE08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267CE0C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267CE10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267CE14: 38EB99A4  addi r7, r11, -0x665c
	ctx.r[7].s64 = ctx.r[11].s64 + -26204;
	// 8267CE18: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8267CE1C: 388AA2E0  addi r4, r10, -0x5d20
	ctx.r[4].s64 = ctx.r[10].s64 + -23840;
	// 8267CE20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267CE24: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267CE28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267CE2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267CE30: 386A6F78  addi r3, r10, 0x6f78
	ctx.r[3].s64 = ctx.r[10].s64 + 28536;
	// 8267CE34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267CE38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267CE3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267CE40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267CE44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267CE48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267CE4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267CE50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267CE54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267CE58: 4BDE9FC9  bl 0x82466e20
	ctx.lr = 0x8267CE5C;
	sub_82466E20(ctx, base);
	// 8267CE5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267CE60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267CE64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267CE68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267CE70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267CE70 size=108
    let mut pc: u32 = 0x8267CE70;
    'dispatch: loop {
        match pc {
            0x8267CE70 => {
    //   block [0x8267CE70..0x8267CEDC)
	// 8267CE70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267CE74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267CE78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267CE7C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267CE80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267CE84: 38EB99BC  addi r7, r11, -0x6644
	ctx.r[7].s64 = ctx.r[11].s64 + -26180;
	// 8267CE88: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8267CE8C: 388AA304  addi r4, r10, -0x5cfc
	ctx.r[4].s64 = ctx.r[10].s64 + -23804;
	// 8267CE90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267CE94: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267CE98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267CE9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267CEA0: 386A6FA8  addi r3, r10, 0x6fa8
	ctx.r[3].s64 = ctx.r[10].s64 + 28584;
	// 8267CEA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267CEA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267CEAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267CEB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267CEB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267CEB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267CEBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267CEC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267CEC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267CEC8: 4BDE9F59  bl 0x82466e20
	ctx.lr = 0x8267CECC;
	sub_82466E20(ctx, base);
	// 8267CECC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267CED0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267CED4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267CED8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267CEE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267CEE0 size=108
    let mut pc: u32 = 0x8267CEE0;
    'dispatch: loop {
        match pc {
            0x8267CEE0 => {
    //   block [0x8267CEE0..0x8267CF4C)
	// 8267CEE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267CEE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267CEE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267CEEC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267CEF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267CEF4: 38EB99F0  addi r7, r11, -0x6610
	ctx.r[7].s64 = ctx.r[11].s64 + -26128;
	// 8267CEF8: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 8267CEFC: 388AAED0  addi r4, r10, -0x5130
	ctx.r[4].s64 = ctx.r[10].s64 + -20784;
	// 8267CF00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267CF04: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267CF08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267CF0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267CF10: 386A6FD8  addi r3, r10, 0x6fd8
	ctx.r[3].s64 = ctx.r[10].s64 + 28632;
	// 8267CF14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267CF18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267CF1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267CF20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267CF24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267CF28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267CF2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267CF30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267CF34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267CF38: 4BDE9EE9  bl 0x82466e20
	ctx.lr = 0x8267CF3C;
	sub_82466E20(ctx, base);
	// 8267CF3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267CF40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267CF44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267CF48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267CF50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267CF50 size=108
    let mut pc: u32 = 0x8267CF50;
    'dispatch: loop {
        match pc {
            0x8267CF50 => {
    //   block [0x8267CF50..0x8267CFBC)
	// 8267CF50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267CF54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267CF58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267CF5C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267CF60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267CF64: 38EB9A98  addi r7, r11, -0x6568
	ctx.r[7].s64 = ctx.r[11].s64 + -25960;
	// 8267CF68: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8267CF6C: 388AA130  addi r4, r10, -0x5ed0
	ctx.r[4].s64 = ctx.r[10].s64 + -24272;
	// 8267CF70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267CF74: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267CF78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267CF7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267CF80: 386A7008  addi r3, r10, 0x7008
	ctx.r[3].s64 = ctx.r[10].s64 + 28680;
	// 8267CF84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267CF88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267CF8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267CF90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267CF94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267CF98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267CF9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267CFA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267CFA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267CFA8: 4BDE9E79  bl 0x82466e20
	ctx.lr = 0x8267CFAC;
	sub_82466E20(ctx, base);
	// 8267CFAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267CFB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267CFB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267CFB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267CFC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267CFC0 size=108
    let mut pc: u32 = 0x8267CFC0;
    'dispatch: loop {
        match pc {
            0x8267CFC0 => {
    //   block [0x8267CFC0..0x8267D02C)
	// 8267CFC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267CFC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267CFC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267CFCC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267CFD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267CFD4: 38EB9AC8  addi r7, r11, -0x6538
	ctx.r[7].s64 = ctx.r[11].s64 + -25912;
	// 8267CFD8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8267CFDC: 388AA148  addi r4, r10, -0x5eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -24248;
	// 8267CFE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267CFE4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267CFE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267CFEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267CFF0: 386A7038  addi r3, r10, 0x7038
	ctx.r[3].s64 = ctx.r[10].s64 + 28728;
	// 8267CFF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267CFF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267CFFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267D000: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267D004: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267D008: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267D00C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267D010: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267D014: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267D018: 4BDE9E09  bl 0x82466e20
	ctx.lr = 0x8267D01C;
	sub_82466E20(ctx, base);
	// 8267D01C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267D020: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267D024: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267D028: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267D030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267D030 size=108
    let mut pc: u32 = 0x8267D030;
    'dispatch: loop {
        match pc {
            0x8267D030 => {
    //   block [0x8267D030..0x8267D09C)
	// 8267D030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267D034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267D038: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267D03C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267D040: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267D044: 38EB9AE0  addi r7, r11, -0x6520
	ctx.r[7].s64 = ctx.r[11].s64 + -25888;
	// 8267D048: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8267D04C: 388AB290  addi r4, r10, -0x4d70
	ctx.r[4].s64 = ctx.r[10].s64 + -19824;
	// 8267D050: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267D054: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267D058: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267D05C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267D060: 386A7068  addi r3, r10, 0x7068
	ctx.r[3].s64 = ctx.r[10].s64 + 28776;
	// 8267D064: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267D068: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267D06C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267D070: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267D074: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267D078: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267D07C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267D080: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267D084: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267D088: 4BDE9D99  bl 0x82466e20
	ctx.lr = 0x8267D08C;
	sub_82466E20(ctx, base);
	// 8267D08C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267D090: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267D094: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267D098: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267D0A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267D0A0 size=112
    let mut pc: u32 = 0x8267D0A0;
    'dispatch: loop {
        match pc {
            0x8267D0A0 => {
    //   block [0x8267D0A0..0x8267D110)
	// 8267D0A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267D0A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267D0A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267D0AC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267D0B0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267D0B4: 38AA6EB8  addi r5, r10, 0x6eb8
	ctx.r[5].s64 = ctx.r[10].s64 + 28344;
	// 8267D0B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267D0BC: 390B9B10  addi r8, r11, -0x64f0
	ctx.r[8].s64 = ctx.r[11].s64 + -25840;
	// 8267D0C0: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8267D0C4: 388AB2A8  addi r4, r10, -0x4d58
	ctx.r[4].s64 = ctx.r[10].s64 + -19800;
	// 8267D0C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267D0CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267D0D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267D0D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267D0D8: 386A7098  addi r3, r10, 0x7098
	ctx.r[3].s64 = ctx.r[10].s64 + 28824;
	// 8267D0DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267D0E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267D0E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267D0E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267D0EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267D0F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267D0F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267D0F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267D0FC: 4BDE9D25  bl 0x82466e20
	ctx.lr = 0x8267D100;
	sub_82466E20(ctx, base);
	// 8267D100: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267D104: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267D108: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267D10C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267D110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8267D110 size=24
    let mut pc: u32 = 0x8267D110;
    'dispatch: loop {
        match pc {
            0x8267D110 => {
    //   block [0x8267D110..0x8267D128)
	// 8267D110: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267D114: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267D118: 394AED38  addi r10, r10, -0x12c8
	ctx.r[10].s64 = ctx.r[10].s64 + -4808;
	// 8267D11C: 816B99EC  lwz r11, -0x6614(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26132 as u32) ) } as u64;
	// 8267D120: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8267D124: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267D128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267D128 size=112
    let mut pc: u32 = 0x8267D128;
    'dispatch: loop {
        match pc {
            0x8267D128 => {
    //   block [0x8267D128..0x8267D198)
	// 8267D128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267D12C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267D130: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267D134: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8267D138: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267D13C: 392A4378  addi r9, r10, 0x4378
	ctx.r[9].s64 = ctx.r[10].s64 + 17272;
	// 8267D140: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267D144: 390BED38  addi r8, r11, -0x12c8
	ctx.r[8].s64 = ctx.r[11].s64 + -4808;
	// 8267D148: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8267D14C: 388AB098  addi r4, r10, -0x4f68
	ctx.r[4].s64 = ctx.r[10].s64 + -20328;
	// 8267D150: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267D154: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267D158: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267D15C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267D160: 386A70C8  addi r3, r10, 0x70c8
	ctx.r[3].s64 = ctx.r[10].s64 + 28872;
	// 8267D164: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267D168: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8267D16C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267D170: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267D174: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267D178: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267D17C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267D180: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267D184: 4BDE9C9D  bl 0x82466e20
	ctx.lr = 0x8267D188;
	sub_82466E20(ctx, base);
	// 8267D188: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267D18C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267D190: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267D194: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267D198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267D198 size=108
    let mut pc: u32 = 0x8267D198;
    'dispatch: loop {
        match pc {
            0x8267D198 => {
    //   block [0x8267D198..0x8267D204)
	// 8267D198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267D19C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267D1A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267D1A4: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267D1A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267D1AC: 38EB9BBC  addi r7, r11, -0x6444
	ctx.r[7].s64 = ctx.r[11].s64 + -25668;
	// 8267D1B0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8267D1B4: 388AAC70  addi r4, r10, -0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + -21392;
	// 8267D1B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267D1BC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267D1C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267D1C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267D1C8: 386A70F8  addi r3, r10, 0x70f8
	ctx.r[3].s64 = ctx.r[10].s64 + 28920;
	// 8267D1CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267D1D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267D1D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267D1D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267D1DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267D1E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267D1E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267D1E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267D1EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267D1F0: 4BDE9C31  bl 0x82466e20
	ctx.lr = 0x8267D1F4;
	sub_82466E20(ctx, base);
	// 8267D1F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267D1F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267D1FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267D200: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267D208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267D208 size=116
    let mut pc: u32 = 0x8267D208;
    'dispatch: loop {
        match pc {
            0x8267D208 => {
    //   block [0x8267D208..0x8267D27C)
	// 8267D208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267D20C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267D210: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267D214: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267D218: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8267D21C: 390B9BF0  addi r8, r11, -0x6410
	ctx.r[8].s64 = ctx.r[11].s64 + -25616;
	// 8267D220: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267D224: 392A43BC  addi r9, r10, 0x43bc
	ctx.r[9].s64 = ctx.r[10].s64 + 17340;
	// 8267D228: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267D22C: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 8267D230: 38AA6EB8  addi r5, r10, 0x6eb8
	ctx.r[5].s64 = ctx.r[10].s64 + 28344;
	// 8267D234: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267D238: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267D23C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267D240: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267D244: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267D248: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267D24C: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 8267D250: 388AAC88  addi r4, r10, -0x5378
	ctx.r[4].s64 = ctx.r[10].s64 + -21368;
	// 8267D254: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267D258: 386B7128  addi r3, r11, 0x7128
	ctx.r[3].s64 = ctx.r[11].s64 + 28968;
	// 8267D25C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8267D260: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267D264: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267D268: 4BDE9BB9  bl 0x82466e20
	ctx.lr = 0x8267D26C;
	sub_82466E20(ctx, base);
	// 8267D26C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267D270: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267D274: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267D278: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267D280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8267D280 size=24
    let mut pc: u32 = 0x8267D280;
    'dispatch: loop {
        match pc {
            0x8267D280 => {
    //   block [0x8267D280..0x8267D298)
	// 8267D280: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267D284: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267D288: 394AEDB0  addi r10, r10, -0x1250
	ctx.r[10].s64 = ctx.r[10].s64 + -4688;
	// 8267D28C: 816B9BEC  lwz r11, -0x6414(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-25620 as u32) ) } as u64;
	// 8267D290: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8267D294: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267D298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267D298 size=112
    let mut pc: u32 = 0x8267D298;
    'dispatch: loop {
        match pc {
            0x8267D298 => {
    //   block [0x8267D298..0x8267D308)
	// 8267D298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267D29C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267D2A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267D2A4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8267D2A8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267D2AC: 392A43F8  addi r9, r10, 0x43f8
	ctx.r[9].s64 = ctx.r[10].s64 + 17400;
	// 8267D2B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267D2B4: 390BEDB0  addi r8, r11, -0x1250
	ctx.r[8].s64 = ctx.r[11].s64 + -4688;
	// 8267D2B8: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8267D2BC: 388AB004  addi r4, r10, -0x4ffc
	ctx.r[4].s64 = ctx.r[10].s64 + -20476;
	// 8267D2C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267D2C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267D2C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267D2CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267D2D0: 386A7158  addi r3, r10, 0x7158
	ctx.r[3].s64 = ctx.r[10].s64 + 29016;
	// 8267D2D4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267D2D8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8267D2DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267D2E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267D2E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267D2E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267D2EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267D2F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267D2F4: 4BDE9B2D  bl 0x82466e20
	ctx.lr = 0x8267D2F8;
	sub_82466E20(ctx, base);
	// 8267D2F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267D2FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267D300: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267D304: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267D308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267D308 size=108
    let mut pc: u32 = 0x8267D308;
    'dispatch: loop {
        match pc {
            0x8267D308 => {
    //   block [0x8267D308..0x8267D374)
	// 8267D308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267D30C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267D310: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267D314: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267D318: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267D31C: 38EB9CB0  addi r7, r11, -0x6350
	ctx.r[7].s64 = ctx.r[11].s64 + -25424;
	// 8267D320: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8267D324: 388AAF00  addi r4, r10, -0x5100
	ctx.r[4].s64 = ctx.r[10].s64 + -20736;
	// 8267D328: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267D32C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267D330: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267D334: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267D338: 386A7188  addi r3, r10, 0x7188
	ctx.r[3].s64 = ctx.r[10].s64 + 29064;
	// 8267D33C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267D340: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267D344: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267D348: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267D34C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267D350: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267D354: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267D358: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267D35C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267D360: 4BDE9AC1  bl 0x82466e20
	ctx.lr = 0x8267D364;
	sub_82466E20(ctx, base);
	// 8267D364: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267D368: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267D36C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267D370: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267D378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267D378 size=108
    let mut pc: u32 = 0x8267D378;
    'dispatch: loop {
        match pc {
            0x8267D378 => {
    //   block [0x8267D378..0x8267D3E4)
	// 8267D378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267D37C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267D380: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267D384: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267D388: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267D38C: 38EB9CC8  addi r7, r11, -0x6338
	ctx.r[7].s64 = ctx.r[11].s64 + -25400;
	// 8267D390: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8267D394: 388A9D38  addi r4, r10, -0x62c8
	ctx.r[4].s64 = ctx.r[10].s64 + -25288;
	// 8267D398: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267D39C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267D3A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267D3A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267D3A8: 386A71B8  addi r3, r10, 0x71b8
	ctx.r[3].s64 = ctx.r[10].s64 + 29112;
	// 8267D3AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267D3B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267D3B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267D3B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267D3BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267D3C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267D3C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267D3C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267D3CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267D3D0: 4BDE9A51  bl 0x82466e20
	ctx.lr = 0x8267D3D4;
	sub_82466E20(ctx, base);
	// 8267D3D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267D3D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267D3DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267D3E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267D3E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8267D3E8 size=24
    let mut pc: u32 = 0x8267D3E8;
    'dispatch: loop {
        match pc {
            0x8267D3E8 => {
    //   block [0x8267D3E8..0x8267D400)
	// 8267D3E8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267D3EC: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267D3F0: 394AEDF8  addi r10, r10, -0x1208
	ctx.r[10].s64 = ctx.r[10].s64 + -4616;
	// 8267D3F4: 816B9CF8  lwz r11, -0x6308(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-25352 as u32) ) } as u64;
	// 8267D3F8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8267D3FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267D400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267D400 size=112
    let mut pc: u32 = 0x8267D400;
    'dispatch: loop {
        match pc {
            0x8267D400 => {
    //   block [0x8267D400..0x8267D470)
	// 8267D400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267D404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267D408: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267D40C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8267D410: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267D414: 392A4434  addi r9, r10, 0x4434
	ctx.r[9].s64 = ctx.r[10].s64 + 17460;
	// 8267D418: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267D41C: 390BEDF8  addi r8, r11, -0x1208
	ctx.r[8].s64 = ctx.r[11].s64 + -4616;
	// 8267D420: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8267D424: 388AB450  addi r4, r10, -0x4bb0
	ctx.r[4].s64 = ctx.r[10].s64 + -19376;
	// 8267D428: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267D42C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267D430: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267D434: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267D438: 386A71E8  addi r3, r10, 0x71e8
	ctx.r[3].s64 = ctx.r[10].s64 + 29160;
	// 8267D43C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267D440: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8267D444: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267D448: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267D44C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267D450: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267D454: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267D458: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267D45C: 4BDE99C5  bl 0x82466e20
	ctx.lr = 0x8267D460;
	sub_82466E20(ctx, base);
	// 8267D460: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267D464: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267D468: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267D46C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267D470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267D470 size=112
    let mut pc: u32 = 0x8267D470;
    'dispatch: loop {
        match pc {
            0x8267D470 => {
    //   block [0x8267D470..0x8267D4E0)
	// 8267D470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267D474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267D478: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267D47C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267D480: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267D484: 38AA6EB8  addi r5, r10, 0x6eb8
	ctx.r[5].s64 = ctx.r[10].s64 + 28344;
	// 8267D488: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267D48C: 390B9CFC  addi r8, r11, -0x6304
	ctx.r[8].s64 = ctx.r[11].s64 + -25348;
	// 8267D490: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8267D494: 388A9CDC  addi r4, r10, -0x6324
	ctx.r[4].s64 = ctx.r[10].s64 + -25380;
	// 8267D498: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267D49C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267D4A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267D4A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267D4A8: 386A7218  addi r3, r10, 0x7218
	ctx.r[3].s64 = ctx.r[10].s64 + 29208;
	// 8267D4AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267D4B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267D4B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267D4B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267D4BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267D4C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267D4C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267D4C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267D4CC: 4BDE9955  bl 0x82466e20
	ctx.lr = 0x8267D4D0;
	sub_82466E20(ctx, base);
	// 8267D4D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267D4D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267D4D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267D4DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267D4E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267D4E0 size=108
    let mut pc: u32 = 0x8267D4E0;
    'dispatch: loop {
        match pc {
            0x8267D4E0 => {
    //   block [0x8267D4E0..0x8267D54C)
	// 8267D4E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267D4E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267D4E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267D4EC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267D4F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267D4F4: 38EB9D2C  addi r7, r11, -0x62d4
	ctx.r[7].s64 = ctx.r[11].s64 + -25300;
	// 8267D4F8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8267D4FC: 388A9CF4  addi r4, r10, -0x630c
	ctx.r[4].s64 = ctx.r[10].s64 + -25356;
	// 8267D500: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267D504: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267D508: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267D50C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267D510: 386A7248  addi r3, r10, 0x7248
	ctx.r[3].s64 = ctx.r[10].s64 + 29256;
	// 8267D514: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267D518: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267D51C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267D520: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267D524: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267D528: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267D52C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267D530: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267D534: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267D538: 4BDE98E9  bl 0x82466e20
	ctx.lr = 0x8267D53C;
	sub_82466E20(ctx, base);
	// 8267D53C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267D540: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267D544: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267D548: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267D550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267D550 size=108
    let mut pc: u32 = 0x8267D550;
    'dispatch: loop {
        match pc {
            0x8267D550 => {
    //   block [0x8267D550..0x8267D5BC)
	// 8267D550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267D554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267D558: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267D55C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267D560: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267D564: 38EB9D60  addi r7, r11, -0x62a0
	ctx.r[7].s64 = ctx.r[11].s64 + -25248;
	// 8267D568: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8267D56C: 388AB148  addi r4, r10, -0x4eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -20152;
	// 8267D570: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267D574: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267D578: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267D57C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267D580: 386A7278  addi r3, r10, 0x7278
	ctx.r[3].s64 = ctx.r[10].s64 + 29304;
	// 8267D584: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267D588: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267D58C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267D590: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267D594: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267D598: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267D59C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267D5A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267D5A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267D5A8: 4BDE9879  bl 0x82466e20
	ctx.lr = 0x8267D5AC;
	sub_82466E20(ctx, base);
	// 8267D5AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267D5B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267D5B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267D5B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267D5C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267D5C0 size=108
    let mut pc: u32 = 0x8267D5C0;
    'dispatch: loop {
        match pc {
            0x8267D5C0 => {
    //   block [0x8267D5C0..0x8267D62C)
	// 8267D5C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267D5C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267D5C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267D5CC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267D5D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267D5D4: 38EB9DC0  addi r7, r11, -0x6240
	ctx.r[7].s64 = ctx.r[11].s64 + -25152;
	// 8267D5D8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8267D5DC: 388AA9FC  addi r4, r10, -0x5604
	ctx.r[4].s64 = ctx.r[10].s64 + -22020;
	// 8267D5E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267D5E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267D5E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267D5EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267D5F0: 386A72A8  addi r3, r10, 0x72a8
	ctx.r[3].s64 = ctx.r[10].s64 + 29352;
	// 8267D5F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267D5F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267D5FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267D600: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267D604: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267D608: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267D60C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267D610: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267D614: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267D618: 4BDE9809  bl 0x82466e20
	ctx.lr = 0x8267D61C;
	sub_82466E20(ctx, base);
	// 8267D61C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267D620: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267D624: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267D628: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267D630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267D630 size=108
    let mut pc: u32 = 0x8267D630;
    'dispatch: loop {
        match pc {
            0x8267D630 => {
    //   block [0x8267D630..0x8267D69C)
	// 8267D630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267D634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267D638: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267D63C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267D640: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267D644: 38EB9DF0  addi r7, r11, -0x6210
	ctx.r[7].s64 = ctx.r[11].s64 + -25104;
	// 8267D648: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 8267D64C: 388AA978  addi r4, r10, -0x5688
	ctx.r[4].s64 = ctx.r[10].s64 + -22152;
	// 8267D650: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267D654: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267D658: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267D65C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267D660: 386A72D8  addi r3, r10, 0x72d8
	ctx.r[3].s64 = ctx.r[10].s64 + 29400;
	// 8267D664: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267D668: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267D66C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267D670: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267D674: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267D678: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267D67C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267D680: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267D684: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267D688: 4BDE9799  bl 0x82466e20
	ctx.lr = 0x8267D68C;
	sub_82466E20(ctx, base);
	// 8267D68C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267D690: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267D694: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267D698: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267D6A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267D6A0 size=108
    let mut pc: u32 = 0x8267D6A0;
    'dispatch: loop {
        match pc {
            0x8267D6A0 => {
    //   block [0x8267D6A0..0x8267D70C)
	// 8267D6A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267D6A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267D6A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267D6AC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267D6B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267D6B4: 38EB9F10  addi r7, r11, -0x60f0
	ctx.r[7].s64 = ctx.r[11].s64 + -24816;
	// 8267D6B8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8267D6BC: 388AA044  addi r4, r10, -0x5fbc
	ctx.r[4].s64 = ctx.r[10].s64 + -24508;
	// 8267D6C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267D6C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267D6C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267D6CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267D6D0: 386A7308  addi r3, r10, 0x7308
	ctx.r[3].s64 = ctx.r[10].s64 + 29448;
	// 8267D6D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267D6D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267D6DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267D6E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267D6E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267D6E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267D6EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267D6F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267D6F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267D6F8: 4BDE9729  bl 0x82466e20
	ctx.lr = 0x8267D6FC;
	sub_82466E20(ctx, base);
	// 8267D6FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267D700: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267D704: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267D708: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267D710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267D710 size=108
    let mut pc: u32 = 0x8267D710;
    'dispatch: loop {
        match pc {
            0x8267D710 => {
    //   block [0x8267D710..0x8267D77C)
	// 8267D710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267D714: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267D718: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267D71C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267D720: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267D724: 38EB9F28  addi r7, r11, -0x60d8
	ctx.r[7].s64 = ctx.r[11].s64 + -24792;
	// 8267D728: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8267D72C: 388AAF3C  addi r4, r10, -0x50c4
	ctx.r[4].s64 = ctx.r[10].s64 + -20676;
	// 8267D730: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267D734: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267D738: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267D73C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267D740: 386A7338  addi r3, r10, 0x7338
	ctx.r[3].s64 = ctx.r[10].s64 + 29496;
	// 8267D744: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267D748: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267D74C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267D750: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267D754: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267D758: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267D75C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267D760: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267D764: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267D768: 4BDE96B9  bl 0x82466e20
	ctx.lr = 0x8267D76C;
	sub_82466E20(ctx, base);
	// 8267D76C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267D770: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267D774: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267D778: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267D780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267D780 size=108
    let mut pc: u32 = 0x8267D780;
    'dispatch: loop {
        match pc {
            0x8267D780 => {
    //   block [0x8267D780..0x8267D7EC)
	// 8267D780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267D784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267D788: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267D78C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267D790: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267D794: 38EB9F40  addi r7, r11, -0x60c0
	ctx.r[7].s64 = ctx.r[11].s64 + -24768;
	// 8267D798: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8267D79C: 388AA0C0  addi r4, r10, -0x5f40
	ctx.r[4].s64 = ctx.r[10].s64 + -24384;
	// 8267D7A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267D7A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267D7A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267D7AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267D7B0: 386A7368  addi r3, r10, 0x7368
	ctx.r[3].s64 = ctx.r[10].s64 + 29544;
	// 8267D7B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267D7B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267D7BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267D7C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267D7C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267D7C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267D7CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267D7D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267D7D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267D7D8: 4BDE9649  bl 0x82466e20
	ctx.lr = 0x8267D7DC;
	sub_82466E20(ctx, base);
	// 8267D7DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267D7E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267D7E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267D7E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267D7F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267D7F0 size=108
    let mut pc: u32 = 0x8267D7F0;
    'dispatch: loop {
        match pc {
            0x8267D7F0 => {
    //   block [0x8267D7F0..0x8267D85C)
	// 8267D7F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267D7F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267D7F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267D7FC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267D800: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267D804: 38EB9F58  addi r7, r11, -0x60a8
	ctx.r[7].s64 = ctx.r[11].s64 + -24744;
	// 8267D808: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8267D80C: 388A9C24  addi r4, r10, -0x63dc
	ctx.r[4].s64 = ctx.r[10].s64 + -25564;
	// 8267D810: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267D814: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267D818: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267D81C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267D820: 386A7398  addi r3, r10, 0x7398
	ctx.r[3].s64 = ctx.r[10].s64 + 29592;
	// 8267D824: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267D828: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267D82C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267D830: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267D834: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267D838: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267D83C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267D840: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267D844: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267D848: 4BDE95D9  bl 0x82466e20
	ctx.lr = 0x8267D84C;
	sub_82466E20(ctx, base);
	// 8267D84C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267D850: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267D854: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267D858: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267D860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267D860 size=108
    let mut pc: u32 = 0x8267D860;
    'dispatch: loop {
        match pc {
            0x8267D860 => {
    //   block [0x8267D860..0x8267D8CC)
	// 8267D860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267D864: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267D868: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267D86C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267D870: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267D874: 38EB9F70  addi r7, r11, -0x6090
	ctx.r[7].s64 = ctx.r[11].s64 + -24720;
	// 8267D878: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8267D87C: 388AA684  addi r4, r10, -0x597c
	ctx.r[4].s64 = ctx.r[10].s64 + -22908;
	// 8267D880: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267D884: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267D888: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267D88C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267D890: 386A73C8  addi r3, r10, 0x73c8
	ctx.r[3].s64 = ctx.r[10].s64 + 29640;
	// 8267D894: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267D898: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267D89C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267D8A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267D8A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267D8A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267D8AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267D8B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267D8B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267D8B8: 4BDE9569  bl 0x82466e20
	ctx.lr = 0x8267D8BC;
	sub_82466E20(ctx, base);
	// 8267D8BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267D8C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267D8C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267D8C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267D8D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267D8D0 size=108
    let mut pc: u32 = 0x8267D8D0;
    'dispatch: loop {
        match pc {
            0x8267D8D0 => {
    //   block [0x8267D8D0..0x8267D93C)
	// 8267D8D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267D8D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267D8D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267D8DC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267D8E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267D8E4: 38EB9F88  addi r7, r11, -0x6078
	ctx.r[7].s64 = ctx.r[11].s64 + -24696;
	// 8267D8E8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8267D8EC: 388AA26C  addi r4, r10, -0x5d94
	ctx.r[4].s64 = ctx.r[10].s64 + -23956;
	// 8267D8F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267D8F4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267D8F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267D8FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267D900: 386A73F8  addi r3, r10, 0x73f8
	ctx.r[3].s64 = ctx.r[10].s64 + 29688;
	// 8267D904: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267D908: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267D90C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267D910: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267D914: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267D918: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267D91C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267D920: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267D924: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267D928: 4BDE94F9  bl 0x82466e20
	ctx.lr = 0x8267D92C;
	sub_82466E20(ctx, base);
	// 8267D92C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267D930: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267D934: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267D938: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267D940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267D940 size=108
    let mut pc: u32 = 0x8267D940;
    'dispatch: loop {
        match pc {
            0x8267D940 => {
    //   block [0x8267D940..0x8267D9AC)
	// 8267D940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267D944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267D948: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267D94C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267D950: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267D954: 38EB9FA0  addi r7, r11, -0x6060
	ctx.r[7].s64 = ctx.r[11].s64 + -24672;
	// 8267D958: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8267D95C: 388AB380  addi r4, r10, -0x4c80
	ctx.r[4].s64 = ctx.r[10].s64 + -19584;
	// 8267D960: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267D964: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267D968: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267D96C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267D970: 386A7428  addi r3, r10, 0x7428
	ctx.r[3].s64 = ctx.r[10].s64 + 29736;
	// 8267D974: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267D978: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267D97C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267D980: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267D984: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267D988: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267D98C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267D990: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267D994: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267D998: 4BDE9489  bl 0x82466e20
	ctx.lr = 0x8267D99C;
	sub_82466E20(ctx, base);
	// 8267D99C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267D9A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267D9A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267D9A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267D9B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267D9B0 size=108
    let mut pc: u32 = 0x8267D9B0;
    'dispatch: loop {
        match pc {
            0x8267D9B0 => {
    //   block [0x8267D9B0..0x8267DA1C)
	// 8267D9B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267D9B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267D9B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267D9BC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267D9C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267D9C4: 38EBA030  addi r7, r11, -0x5fd0
	ctx.r[7].s64 = ctx.r[11].s64 + -24528;
	// 8267D9C8: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 8267D9CC: 388AA228  addi r4, r10, -0x5dd8
	ctx.r[4].s64 = ctx.r[10].s64 + -24024;
	// 8267D9D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267D9D4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267D9D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267D9DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267D9E0: 386A7458  addi r3, r10, 0x7458
	ctx.r[3].s64 = ctx.r[10].s64 + 29784;
	// 8267D9E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267D9E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267D9EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267D9F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267D9F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267D9F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267D9FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267DA00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267DA04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267DA08: 4BDE9419  bl 0x82466e20
	ctx.lr = 0x8267DA0C;
	sub_82466E20(ctx, base);
	// 8267DA0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267DA10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267DA14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267DA18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267DA20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267DA20 size=108
    let mut pc: u32 = 0x8267DA20;
    'dispatch: loop {
        match pc {
            0x8267DA20 => {
    //   block [0x8267DA20..0x8267DA8C)
	// 8267DA20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267DA24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267DA28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267DA2C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267DA30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267DA34: 38EBA0F0  addi r7, r11, -0x5f10
	ctx.r[7].s64 = ctx.r[11].s64 + -24336;
	// 8267DA38: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 8267DA3C: 388AA000  addi r4, r10, -0x6000
	ctx.r[4].s64 = ctx.r[10].s64 + -24576;
	// 8267DA40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267DA44: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267DA48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267DA4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267DA50: 386A7488  addi r3, r10, 0x7488
	ctx.r[3].s64 = ctx.r[10].s64 + 29832;
	// 8267DA54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267DA58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267DA5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267DA60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267DA64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267DA68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267DA6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267DA70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267DA74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267DA78: 4BDE93A9  bl 0x82466e20
	ctx.lr = 0x8267DA7C;
	sub_82466E20(ctx, base);
	// 8267DA7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267DA80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267DA84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267DA88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267DA90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267DA90 size=108
    let mut pc: u32 = 0x8267DA90;
    'dispatch: loop {
        match pc {
            0x8267DA90 => {
    //   block [0x8267DA90..0x8267DAFC)
	// 8267DA90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267DA94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267DA98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267DA9C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267DAA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267DAA4: 38EBA1C8  addi r7, r11, -0x5e38
	ctx.r[7].s64 = ctx.r[11].s64 + -24120;
	// 8267DAA8: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 8267DAAC: 388A9F00  addi r4, r10, -0x6100
	ctx.r[4].s64 = ctx.r[10].s64 + -24832;
	// 8267DAB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267DAB4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267DAB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267DABC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267DAC0: 386A74B8  addi r3, r10, 0x74b8
	ctx.r[3].s64 = ctx.r[10].s64 + 29880;
	// 8267DAC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267DAC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267DACC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267DAD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267DAD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267DAD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267DADC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267DAE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267DAE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267DAE8: 4BDE9339  bl 0x82466e20
	ctx.lr = 0x8267DAEC;
	sub_82466E20(ctx, base);
	// 8267DAEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267DAF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267DAF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267DAF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267DB00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267DB00 size=108
    let mut pc: u32 = 0x8267DB00;
    'dispatch: loop {
        match pc {
            0x8267DB00 => {
    //   block [0x8267DB00..0x8267DB6C)
	// 8267DB00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267DB04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267DB08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267DB0C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267DB10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267DB14: 38EBA288  addi r7, r11, -0x5d78
	ctx.r[7].s64 = ctx.r[11].s64 + -23928;
	// 8267DB18: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 8267DB1C: 388AADE0  addi r4, r10, -0x5220
	ctx.r[4].s64 = ctx.r[10].s64 + -21024;
	// 8267DB20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267DB24: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267DB28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267DB2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267DB30: 386A74E8  addi r3, r10, 0x74e8
	ctx.r[3].s64 = ctx.r[10].s64 + 29928;
	// 8267DB34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267DB38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267DB3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267DB40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267DB44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267DB48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267DB4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267DB50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267DB54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267DB58: 4BDE92C9  bl 0x82466e20
	ctx.lr = 0x8267DB5C;
	sub_82466E20(ctx, base);
	// 8267DB5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267DB60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267DB64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267DB68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267DB70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267DB70 size=112
    let mut pc: u32 = 0x8267DB70;
    'dispatch: loop {
        match pc {
            0x8267DB70 => {
    //   block [0x8267DB70..0x8267DBE0)
	// 8267DB70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267DB74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267DB78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267DB7C: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267DB80: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 8267DB84: 38EAA330  addi r7, r10, -0x5cd0
	ctx.r[7].s64 = ctx.r[10].s64 + -23760;
	// 8267DB88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267DB8C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8267DB90: 388AA5FC  addi r4, r10, -0x5a04
	ctx.r[4].s64 = ctx.r[10].s64 + -23044;
	// 8267DB94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267DB98: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267DB9C: 396B4448  addi r11, r11, 0x4448
	ctx.r[11].s64 = ctx.r[11].s64 + 17480;
	// 8267DBA0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267DBA4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267DBA8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267DBAC: 386A7518  addi r3, r10, 0x7518
	ctx.r[3].s64 = ctx.r[10].s64 + 29976;
	// 8267DBB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267DBB4: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8267DBB8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267DBBC: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8267DBC0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267DBC4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267DBC8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267DBCC: 4BDE9255  bl 0x82466e20
	ctx.lr = 0x8267DBD0;
	sub_82466E20(ctx, base);
	// 8267DBD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267DBD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267DBD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267DBDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267DBE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267DBE0 size=108
    let mut pc: u32 = 0x8267DBE0;
    'dispatch: loop {
        match pc {
            0x8267DBE0 => {
    //   block [0x8267DBE0..0x8267DC4C)
	// 8267DBE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267DBE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267DBE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267DBEC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267DBF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267DBF4: 38EBA450  addi r7, r11, -0x5bb0
	ctx.r[7].s64 = ctx.r[11].s64 + -23472;
	// 8267DBF8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8267DBFC: 388AAD28  addi r4, r10, -0x52d8
	ctx.r[4].s64 = ctx.r[10].s64 + -21208;
	// 8267DC00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267DC04: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267DC08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267DC0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267DC10: 386A7548  addi r3, r10, 0x7548
	ctx.r[3].s64 = ctx.r[10].s64 + 30024;
	// 8267DC14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267DC18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267DC1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267DC20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267DC24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267DC28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267DC2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267DC30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267DC34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267DC38: 4BDE91E9  bl 0x82466e20
	ctx.lr = 0x8267DC3C;
	sub_82466E20(ctx, base);
	// 8267DC3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267DC40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267DC44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267DC48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267DC50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267DC50 size=108
    let mut pc: u32 = 0x8267DC50;
    'dispatch: loop {
        match pc {
            0x8267DC50 => {
    //   block [0x8267DC50..0x8267DCBC)
	// 8267DC50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267DC54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267DC58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267DC5C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267DC60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267DC64: 38EBA4B0  addi r7, r11, -0x5b50
	ctx.r[7].s64 = ctx.r[11].s64 + -23376;
	// 8267DC68: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 8267DC6C: 388A9070  addi r4, r10, -0x6f90
	ctx.r[4].s64 = ctx.r[10].s64 + -28560;
	// 8267DC70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267DC74: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267DC78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267DC7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267DC80: 386A7578  addi r3, r10, 0x7578
	ctx.r[3].s64 = ctx.r[10].s64 + 30072;
	// 8267DC84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267DC88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267DC8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267DC90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267DC94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267DC98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267DC9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267DCA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267DCA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267DCA8: 4BDE9179  bl 0x82466e20
	ctx.lr = 0x8267DCAC;
	sub_82466E20(ctx, base);
	// 8267DCAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267DCB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267DCB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267DCB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267DCC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267DCC0 size=108
    let mut pc: u32 = 0x8267DCC0;
    'dispatch: loop {
        match pc {
            0x8267DCC0 => {
    //   block [0x8267DCC0..0x8267DD2C)
	// 8267DCC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267DCC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267DCC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267DCCC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267DCD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267DCD4: 38EBA5B8  addi r7, r11, -0x5a48
	ctx.r[7].s64 = ctx.r[11].s64 + -23112;
	// 8267DCD8: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 8267DCDC: 388A8E98  addi r4, r10, -0x7168
	ctx.r[4].s64 = ctx.r[10].s64 + -29032;
	// 8267DCE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267DCE4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267DCE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267DCEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267DCF0: 386A75A8  addi r3, r10, 0x75a8
	ctx.r[3].s64 = ctx.r[10].s64 + 30120;
	// 8267DCF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267DCF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267DCFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267DD00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267DD04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267DD08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267DD0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267DD10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267DD14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267DD18: 4BDE9109  bl 0x82466e20
	ctx.lr = 0x8267DD1C;
	sub_82466E20(ctx, base);
	// 8267DD1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267DD20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267DD24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267DD28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267DD30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267DD30 size=108
    let mut pc: u32 = 0x8267DD30;
    'dispatch: loop {
        match pc {
            0x8267DD30 => {
    //   block [0x8267DD30..0x8267DD9C)
	// 8267DD30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267DD34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267DD38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267DD3C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267DD40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267DD44: 38EBA690  addi r7, r11, -0x5970
	ctx.r[7].s64 = ctx.r[11].s64 + -22896;
	// 8267DD48: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8267DD4C: 388A8CF0  addi r4, r10, -0x7310
	ctx.r[4].s64 = ctx.r[10].s64 + -29456;
	// 8267DD50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267DD54: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267DD58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267DD5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267DD60: 386A75D8  addi r3, r10, 0x75d8
	ctx.r[3].s64 = ctx.r[10].s64 + 30168;
	// 8267DD64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267DD68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267DD6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267DD70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267DD74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267DD78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267DD7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267DD80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267DD84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267DD88: 4BDE9099  bl 0x82466e20
	ctx.lr = 0x8267DD8C;
	sub_82466E20(ctx, base);
	// 8267DD8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267DD90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267DD94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267DD98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267DDA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267DDA0 size=108
    let mut pc: u32 = 0x8267DDA0;
    'dispatch: loop {
        match pc {
            0x8267DDA0 => {
    //   block [0x8267DDA0..0x8267DE0C)
	// 8267DDA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267DDA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267DDA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267DDAC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267DDB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267DDB4: 38EBA6D8  addi r7, r11, -0x5928
	ctx.r[7].s64 = ctx.r[11].s64 + -22824;
	// 8267DDB8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8267DDBC: 388A8D14  addi r4, r10, -0x72ec
	ctx.r[4].s64 = ctx.r[10].s64 + -29420;
	// 8267DDC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267DDC4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267DDC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267DDCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267DDD0: 386A7608  addi r3, r10, 0x7608
	ctx.r[3].s64 = ctx.r[10].s64 + 30216;
	// 8267DDD4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267DDD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267DDDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267DDE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267DDE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267DDE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267DDEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267DDF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267DDF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267DDF8: 4BDE9029  bl 0x82466e20
	ctx.lr = 0x8267DDFC;
	sub_82466E20(ctx, base);
	// 8267DDFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267DE00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267DE04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267DE08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267DE10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267DE10 size=108
    let mut pc: u32 = 0x8267DE10;
    'dispatch: loop {
        match pc {
            0x8267DE10 => {
    //   block [0x8267DE10..0x8267DE7C)
	// 8267DE10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267DE14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267DE18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267DE1C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267DE20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267DE24: 38EBA6F0  addi r7, r11, -0x5910
	ctx.r[7].s64 = ctx.r[11].s64 + -22800;
	// 8267DE28: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8267DE2C: 388A37C4  addi r4, r10, 0x37c4
	ctx.r[4].s64 = ctx.r[10].s64 + 14276;
	// 8267DE30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267DE34: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267DE38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267DE3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267DE40: 386A7638  addi r3, r10, 0x7638
	ctx.r[3].s64 = ctx.r[10].s64 + 30264;
	// 8267DE44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267DE48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267DE4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267DE50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267DE54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267DE58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267DE5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267DE60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267DE64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267DE68: 4BDE8FB9  bl 0x82466e20
	ctx.lr = 0x8267DE6C;
	sub_82466E20(ctx, base);
	// 8267DE6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267DE70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267DE74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267DE78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267DE80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267DE80 size=108
    let mut pc: u32 = 0x8267DE80;
    'dispatch: loop {
        match pc {
            0x8267DE80 => {
    //   block [0x8267DE80..0x8267DEEC)
	// 8267DE80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267DE84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267DE88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267DE8C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267DE90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267DE94: 38EBA738  addi r7, r11, -0x58c8
	ctx.r[7].s64 = ctx.r[11].s64 + -22728;
	// 8267DE98: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8267DE9C: 388A37F4  addi r4, r10, 0x37f4
	ctx.r[4].s64 = ctx.r[10].s64 + 14324;
	// 8267DEA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267DEA4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267DEA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267DEAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267DEB0: 386A7668  addi r3, r10, 0x7668
	ctx.r[3].s64 = ctx.r[10].s64 + 30312;
	// 8267DEB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267DEB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267DEBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267DEC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267DEC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267DEC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267DECC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267DED0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267DED4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267DED8: 4BDE8F49  bl 0x82466e20
	ctx.lr = 0x8267DEDC;
	sub_82466E20(ctx, base);
	// 8267DEDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267DEE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267DEE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267DEE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267DEF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267DEF0 size=112
    let mut pc: u32 = 0x8267DEF0;
    'dispatch: loop {
        match pc {
            0x8267DEF0 => {
    //   block [0x8267DEF0..0x8267DF60)
	// 8267DEF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267DEF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267DEF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267DEFC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267DF00: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267DF04: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 8267DF08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267DF0C: 390BA750  addi r8, r11, -0x58b0
	ctx.r[8].s64 = ctx.r[11].s64 + -22704;
	// 8267DF10: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8267DF14: 388A3810  addi r4, r10, 0x3810
	ctx.r[4].s64 = ctx.r[10].s64 + 14352;
	// 8267DF18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267DF1C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267DF20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267DF24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267DF28: 386A7698  addi r3, r10, 0x7698
	ctx.r[3].s64 = ctx.r[10].s64 + 30360;
	// 8267DF2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267DF30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267DF34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267DF38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267DF3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267DF40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267DF44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267DF48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267DF4C: 4BDE8ED5  bl 0x82466e20
	ctx.lr = 0x8267DF50;
	sub_82466E20(ctx, base);
	// 8267DF50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267DF54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267DF58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267DF5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267DF60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267DF60 size=108
    let mut pc: u32 = 0x8267DF60;
    'dispatch: loop {
        match pc {
            0x8267DF60 => {
    //   block [0x8267DF60..0x8267DFCC)
	// 8267DF60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267DF64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267DF68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267DF6C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267DF70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267DF74: 38EBA798  addi r7, r11, -0x5868
	ctx.r[7].s64 = ctx.r[11].s64 + -22632;
	// 8267DF78: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8267DF7C: 388A3A30  addi r4, r10, 0x3a30
	ctx.r[4].s64 = ctx.r[10].s64 + 14896;
	// 8267DF80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267DF84: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267DF88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267DF8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267DF90: 386A76C8  addi r3, r10, 0x76c8
	ctx.r[3].s64 = ctx.r[10].s64 + 30408;
	// 8267DF94: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267DF98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267DF9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267DFA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267DFA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267DFA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267DFAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267DFB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267DFB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267DFB8: 4BDE8E69  bl 0x82466e20
	ctx.lr = 0x8267DFBC;
	sub_82466E20(ctx, base);
	// 8267DFBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267DFC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267DFC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267DFC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267DFD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267DFD0 size=116
    let mut pc: u32 = 0x8267DFD0;
    'dispatch: loop {
        match pc {
            0x8267DFD0 => {
    //   block [0x8267DFD0..0x8267E044)
	// 8267DFD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267DFD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267DFD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267DFDC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267DFE0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8267DFE4: 390BA7F8  addi r8, r11, -0x5808
	ctx.r[8].s64 = ctx.r[11].s64 + -22536;
	// 8267DFE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267DFEC: 392A44C0  addi r9, r10, 0x44c0
	ctx.r[9].s64 = ctx.r[10].s64 + 17600;
	// 8267DFF0: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267DFF4: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8267DFF8: 38AA76C8  addi r5, r10, 0x76c8
	ctx.r[5].s64 = ctx.r[10].s64 + 30408;
	// 8267DFFC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267E000: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267E004: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267E008: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267E00C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267E010: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267E014: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 8267E018: 388A3A3C  addi r4, r10, 0x3a3c
	ctx.r[4].s64 = ctx.r[10].s64 + 14908;
	// 8267E01C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267E020: 386B76F8  addi r3, r11, 0x76f8
	ctx.r[3].s64 = ctx.r[11].s64 + 30456;
	// 8267E024: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8267E028: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267E02C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267E030: 4BDE8DF1  bl 0x82466e20
	ctx.lr = 0x8267E034;
	sub_82466E20(ctx, base);
	// 8267E034: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267E038: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267E03C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267E040: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267E048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267E048 size=96
    let mut pc: u32 = 0x8267E048;
    'dispatch: loop {
        match pc {
            0x8267E048 => {
    //   block [0x8267E048..0x8267E0A8)
	// 8267E048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267E04C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267E050: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267E054: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267E058: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267E05C: 388A3A4C  addi r4, r10, 0x3a4c
	ctx.r[4].s64 = ctx.r[10].s64 + 14924;
	// 8267E060: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E064: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267E068: 386A7728  addi r3, r10, 0x7728
	ctx.r[3].s64 = ctx.r[10].s64 + 30504;
	// 8267E06C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267E070: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267E074: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8267E078: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267E07C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267E080: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267E084: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267E088: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8267E08C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267E090: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267E094: 4BDE8D8D  bl 0x82466e20
	ctx.lr = 0x8267E098;
	sub_82466E20(ctx, base);
	// 8267E098: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267E09C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267E0A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267E0A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267E0A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267E0A8 size=112
    let mut pc: u32 = 0x8267E0A8;
    'dispatch: loop {
        match pc {
            0x8267E0A8 => {
    //   block [0x8267E0A8..0x8267E118)
	// 8267E0A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267E0AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267E0B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267E0B4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8267E0B8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267E0BC: 38AA9738  addi r5, r10, -0x68c8
	ctx.r[5].s64 = ctx.r[10].s64 + -26824;
	// 8267E0C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267E0C4: 390BA870  addi r8, r11, -0x5790
	ctx.r[8].s64 = ctx.r[11].s64 + -22416;
	// 8267E0C8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8267E0CC: 388A3A64  addi r4, r10, 0x3a64
	ctx.r[4].s64 = ctx.r[10].s64 + 14948;
	// 8267E0D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267E0D4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E0D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267E0DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267E0E0: 386A7758  addi r3, r10, 0x7758
	ctx.r[3].s64 = ctx.r[10].s64 + 30552;
	// 8267E0E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267E0E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267E0EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267E0F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267E0F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267E0F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267E0FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267E100: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267E104: 4BDE8D1D  bl 0x82466e20
	ctx.lr = 0x8267E108;
	sub_82466E20(ctx, base);
	// 8267E108: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267E10C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267E110: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267E114: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267E118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267E118 size=96
    let mut pc: u32 = 0x8267E118;
    'dispatch: loop {
        match pc {
            0x8267E118 => {
    //   block [0x8267E118..0x8267E178)
	// 8267E118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267E11C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267E120: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267E124: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267E128: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267E12C: 388A3A7C  addi r4, r10, 0x3a7c
	ctx.r[4].s64 = ctx.r[10].s64 + 14972;
	// 8267E130: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E134: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267E138: 386A7788  addi r3, r10, 0x7788
	ctx.r[3].s64 = ctx.r[10].s64 + 30600;
	// 8267E13C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267E140: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267E144: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8267E148: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267E14C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267E150: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267E154: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267E158: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8267E15C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267E160: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267E164: 4BDE8CBD  bl 0x82466e20
	ctx.lr = 0x8267E168;
	sub_82466E20(ctx, base);
	// 8267E168: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267E16C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267E170: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267E174: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267E178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8267E178 size=24
    let mut pc: u32 = 0x8267E178;
    'dispatch: loop {
        match pc {
            0x8267E178 => {
    //   block [0x8267E178..0x8267E190)
	// 8267E178: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267E17C: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267E180: 394AEE70  addi r10, r10, -0x1190
	ctx.r[10].s64 = ctx.r[10].s64 + -4496;
	// 8267E184: 816BA8D0  lwz r11, -0x5730(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-22320 as u32) ) } as u64;
	// 8267E188: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8267E18C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267E190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267E190 size=116
    let mut pc: u32 = 0x8267E190;
    'dispatch: loop {
        match pc {
            0x8267E190 => {
    //   block [0x8267E190..0x8267E204)
	// 8267E190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267E194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267E198: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267E19C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267E1A0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8267E1A4: 390BEE70  addi r8, r11, -0x1190
	ctx.r[8].s64 = ctx.r[11].s64 + -4496;
	// 8267E1A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267E1AC: 392A44FC  addi r9, r10, 0x44fc
	ctx.r[9].s64 = ctx.r[10].s64 + 17660;
	// 8267E1B0: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E1B4: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 8267E1B8: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 8267E1BC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267E1C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267E1C4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267E1C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267E1CC: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8267E1D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267E1D4: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 8267E1D8: 388A3A9C  addi r4, r10, 0x3a9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15004;
	// 8267E1DC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267E1E0: 386B77B8  addi r3, r11, 0x77b8
	ctx.r[3].s64 = ctx.r[11].s64 + 30648;
	// 8267E1E4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8267E1E8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267E1EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267E1F0: 4BDE8C31  bl 0x82466e20
	ctx.lr = 0x8267E1F4;
	sub_82466E20(ctx, base);
	// 8267E1F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267E1F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267E1FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267E200: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267E208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267E208 size=104
    let mut pc: u32 = 0x8267E208;
    'dispatch: loop {
        match pc {
            0x8267E208 => {
    //   block [0x8267E208..0x8267E270)
	// 8267E208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267E20C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267E210: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267E214: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8267E218: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267E21C: 392A4528  addi r9, r10, 0x4528
	ctx.r[9].s64 = ctx.r[10].s64 + 17704;
	// 8267E220: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E224: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267E228: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 8267E22C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267E230: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267E234: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267E238: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267E23C: 388A3AB0  addi r4, r10, 0x3ab0
	ctx.r[4].s64 = ctx.r[10].s64 + 15024;
	// 8267E240: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267E244: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E248: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8267E24C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267E250: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267E254: 386A77E8  addi r3, r10, 0x77e8
	ctx.r[3].s64 = ctx.r[10].s64 + 30696;
	// 8267E258: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8267E25C: 4BDE8BC5  bl 0x82466e20
	ctx.lr = 0x8267E260;
	sub_82466E20(ctx, base);
	// 8267E260: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267E264: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267E268: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267E26C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267E270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267E270 size=96
    let mut pc: u32 = 0x8267E270;
    'dispatch: loop {
        match pc {
            0x8267E270 => {
    //   block [0x8267E270..0x8267E2D0)
	// 8267E270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267E274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267E278: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267E27C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267E280: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267E284: 388A3AC4  addi r4, r10, 0x3ac4
	ctx.r[4].s64 = ctx.r[10].s64 + 15044;
	// 8267E288: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E28C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267E290: 386A7818  addi r3, r10, 0x7818
	ctx.r[3].s64 = ctx.r[10].s64 + 30744;
	// 8267E294: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267E298: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267E29C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8267E2A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267E2A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267E2A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267E2AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267E2B0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8267E2B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267E2B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267E2BC: 4BDE8B65  bl 0x82466e20
	ctx.lr = 0x8267E2C0;
	sub_82466E20(ctx, base);
	// 8267E2C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267E2C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267E2C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267E2CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267E2D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267E2D0 size=100
    let mut pc: u32 = 0x8267E2D0;
    'dispatch: loop {
        match pc {
            0x8267E2D0 => {
    //   block [0x8267E2D0..0x8267E334)
	// 8267E2D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267E2D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267E2D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267E2DC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E2E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267E2E4: 38AA77E8  addi r5, r10, 0x77e8
	ctx.r[5].s64 = ctx.r[10].s64 + 30696;
	// 8267E2E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267E2EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267E2F0: 388A3ADC  addi r4, r10, 0x3adc
	ctx.r[4].s64 = ctx.r[10].s64 + 15068;
	// 8267E2F4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E2F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267E2FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267E300: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267E304: 386A7848  addi r3, r10, 0x7848
	ctx.r[3].s64 = ctx.r[10].s64 + 30792;
	// 8267E308: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267E30C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267E310: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8267E314: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267E318: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267E31C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267E320: 4BDE8B01  bl 0x82466e20
	ctx.lr = 0x8267E324;
	sub_82466E20(ctx, base);
	// 8267E324: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267E328: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267E32C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267E330: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267E338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267E338 size=112
    let mut pc: u32 = 0x8267E338;
    'dispatch: loop {
        match pc {
            0x8267E338 => {
    //   block [0x8267E338..0x8267E3A8)
	// 8267E338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267E33C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267E340: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267E344: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E348: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267E34C: 38AA77B8  addi r5, r10, 0x77b8
	ctx.r[5].s64 = ctx.r[10].s64 + 30648;
	// 8267E350: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267E354: 390BA8D8  addi r8, r11, -0x5728
	ctx.r[8].s64 = ctx.r[11].s64 + -22312;
	// 8267E358: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8267E35C: 388A3AF8  addi r4, r10, 0x3af8
	ctx.r[4].s64 = ctx.r[10].s64 + 15096;
	// 8267E360: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267E364: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E368: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267E36C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267E370: 386A7878  addi r3, r10, 0x7878
	ctx.r[3].s64 = ctx.r[10].s64 + 30840;
	// 8267E374: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267E378: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267E37C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267E380: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267E384: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267E388: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267E38C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267E390: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267E394: 4BDE8A8D  bl 0x82466e20
	ctx.lr = 0x8267E398;
	sub_82466E20(ctx, base);
	// 8267E398: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267E39C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267E3A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267E3A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267E3A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267E3A8 size=112
    let mut pc: u32 = 0x8267E3A8;
    'dispatch: loop {
        match pc {
            0x8267E3A8 => {
    //   block [0x8267E3A8..0x8267E418)
	// 8267E3A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267E3AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267E3B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267E3B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E3B8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267E3BC: 38AA77B8  addi r5, r10, 0x77b8
	ctx.r[5].s64 = ctx.r[10].s64 + 30648;
	// 8267E3C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267E3C4: 390BA920  addi r8, r11, -0x56e0
	ctx.r[8].s64 = ctx.r[11].s64 + -22240;
	// 8267E3C8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8267E3CC: 388A3B08  addi r4, r10, 0x3b08
	ctx.r[4].s64 = ctx.r[10].s64 + 15112;
	// 8267E3D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267E3D4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E3D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267E3DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267E3E0: 386A78A8  addi r3, r10, 0x78a8
	ctx.r[3].s64 = ctx.r[10].s64 + 30888;
	// 8267E3E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267E3E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267E3EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267E3F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267E3F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267E3F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267E3FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267E400: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267E404: 4BDE8A1D  bl 0x82466e20
	ctx.lr = 0x8267E408;
	sub_82466E20(ctx, base);
	// 8267E408: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267E40C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267E410: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267E414: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267E418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267E418 size=100
    let mut pc: u32 = 0x8267E418;
    'dispatch: loop {
        match pc {
            0x8267E418 => {
    //   block [0x8267E418..0x8267E47C)
	// 8267E418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267E41C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267E420: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267E424: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E428: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267E42C: 38AA77B8  addi r5, r10, 0x77b8
	ctx.r[5].s64 = ctx.r[10].s64 + 30648;
	// 8267E430: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267E434: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267E438: 388A3B20  addi r4, r10, 0x3b20
	ctx.r[4].s64 = ctx.r[10].s64 + 15136;
	// 8267E43C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E440: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267E444: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267E448: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267E44C: 386A78D8  addi r3, r10, 0x78d8
	ctx.r[3].s64 = ctx.r[10].s64 + 30936;
	// 8267E450: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267E454: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267E458: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8267E45C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267E460: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267E464: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267E468: 4BDE89B9  bl 0x82466e20
	ctx.lr = 0x8267E46C;
	sub_82466E20(ctx, base);
	// 8267E46C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267E470: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267E474: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267E478: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267E480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267E480 size=96
    let mut pc: u32 = 0x8267E480;
    'dispatch: loop {
        match pc {
            0x8267E480 => {
    //   block [0x8267E480..0x8267E4E0)
	// 8267E480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267E484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267E488: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267E48C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267E490: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267E494: 388A3B38  addi r4, r10, 0x3b38
	ctx.r[4].s64 = ctx.r[10].s64 + 15160;
	// 8267E498: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E49C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267E4A0: 386A7908  addi r3, r10, 0x7908
	ctx.r[3].s64 = ctx.r[10].s64 + 30984;
	// 8267E4A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267E4A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267E4AC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8267E4B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267E4B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267E4B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267E4BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267E4C0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8267E4C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267E4C8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267E4CC: 4BDE8955  bl 0x82466e20
	ctx.lr = 0x8267E4D0;
	sub_82466E20(ctx, base);
	// 8267E4D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267E4D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267E4D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267E4DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267E4E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267E4E0 size=112
    let mut pc: u32 = 0x8267E4E0;
    'dispatch: loop {
        match pc {
            0x8267E4E0 => {
    //   block [0x8267E4E0..0x8267E550)
	// 8267E4E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267E4E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267E4E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267E4EC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E4F0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267E4F4: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 8267E4F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267E4FC: 390BA938  addi r8, r11, -0x56c8
	ctx.r[8].s64 = ctx.r[11].s64 + -22216;
	// 8267E500: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8267E504: 388A3B54  addi r4, r10, 0x3b54
	ctx.r[4].s64 = ctx.r[10].s64 + 15188;
	// 8267E508: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267E50C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E510: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267E514: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267E518: 386A7938  addi r3, r10, 0x7938
	ctx.r[3].s64 = ctx.r[10].s64 + 31032;
	// 8267E51C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267E520: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267E524: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267E528: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267E52C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267E530: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267E534: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267E538: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267E53C: 4BDE88E5  bl 0x82466e20
	ctx.lr = 0x8267E540;
	sub_82466E20(ctx, base);
	// 8267E540: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267E544: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267E548: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267E54C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267E550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267E550 size=96
    let mut pc: u32 = 0x8267E550;
    'dispatch: loop {
        match pc {
            0x8267E550 => {
    //   block [0x8267E550..0x8267E5B0)
	// 8267E550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267E554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267E558: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267E55C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267E560: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267E564: 388A3B5C  addi r4, r10, 0x3b5c
	ctx.r[4].s64 = ctx.r[10].s64 + 15196;
	// 8267E568: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E56C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267E570: 386A7968  addi r3, r10, 0x7968
	ctx.r[3].s64 = ctx.r[10].s64 + 31080;
	// 8267E574: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267E578: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267E57C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8267E580: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267E584: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267E588: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267E58C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267E590: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8267E594: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267E598: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267E59C: 4BDE8885  bl 0x82466e20
	ctx.lr = 0x8267E5A0;
	sub_82466E20(ctx, base);
	// 8267E5A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267E5A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267E5A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267E5AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267E5B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267E5B0 size=112
    let mut pc: u32 = 0x8267E5B0;
    'dispatch: loop {
        match pc {
            0x8267E5B0 => {
    //   block [0x8267E5B0..0x8267E620)
	// 8267E5B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267E5B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267E5B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267E5BC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E5C0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267E5C4: 38AA7968  addi r5, r10, 0x7968
	ctx.r[5].s64 = ctx.r[10].s64 + 31080;
	// 8267E5C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267E5CC: 390BA968  addi r8, r11, -0x5698
	ctx.r[8].s64 = ctx.r[11].s64 + -22168;
	// 8267E5D0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8267E5D4: 388A3B70  addi r4, r10, 0x3b70
	ctx.r[4].s64 = ctx.r[10].s64 + 15216;
	// 8267E5D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267E5DC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E5E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267E5E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267E5E8: 386A7998  addi r3, r10, 0x7998
	ctx.r[3].s64 = ctx.r[10].s64 + 31128;
	// 8267E5EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267E5F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267E5F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267E5F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267E5FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267E600: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267E604: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267E608: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267E60C: 4BDE8815  bl 0x82466e20
	ctx.lr = 0x8267E610;
	sub_82466E20(ctx, base);
	// 8267E610: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267E614: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267E618: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267E61C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267E620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267E620 size=108
    let mut pc: u32 = 0x8267E620;
    'dispatch: loop {
        match pc {
            0x8267E620 => {
    //   block [0x8267E620..0x8267E68C)
	// 8267E620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267E624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267E628: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267E62C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267E630: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267E634: 38EBA980  addi r7, r11, -0x5680
	ctx.r[7].s64 = ctx.r[11].s64 + -22144;
	// 8267E638: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8267E63C: 388A3B88  addi r4, r10, 0x3b88
	ctx.r[4].s64 = ctx.r[10].s64 + 15240;
	// 8267E640: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267E644: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E648: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267E64C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267E650: 386A79C8  addi r3, r10, 0x79c8
	ctx.r[3].s64 = ctx.r[10].s64 + 31176;
	// 8267E654: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267E658: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267E65C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267E660: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267E664: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267E668: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267E66C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267E670: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267E674: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267E678: 4BDE87A9  bl 0x82466e20
	ctx.lr = 0x8267E67C;
	sub_82466E20(ctx, base);
	// 8267E67C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267E680: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267E684: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267E688: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267E690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267E690 size=112
    let mut pc: u32 = 0x8267E690;
    'dispatch: loop {
        match pc {
            0x8267E690 => {
    //   block [0x8267E690..0x8267E700)
	// 8267E690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267E694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267E698: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267E69C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E6A0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267E6A4: 38AA7AE8  addi r5, r10, 0x7ae8
	ctx.r[5].s64 = ctx.r[10].s64 + 31464;
	// 8267E6A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267E6AC: 390BA9E0  addi r8, r11, -0x5620
	ctx.r[8].s64 = ctx.r[11].s64 + -22048;
	// 8267E6B0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8267E6B4: 388A3B9C  addi r4, r10, 0x3b9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15260;
	// 8267E6B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267E6BC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E6C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267E6C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267E6C8: 386A79F8  addi r3, r10, 0x79f8
	ctx.r[3].s64 = ctx.r[10].s64 + 31224;
	// 8267E6CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267E6D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267E6D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267E6D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267E6DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267E6E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267E6E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267E6E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267E6EC: 4BDE8735  bl 0x82466e20
	ctx.lr = 0x8267E6F0;
	sub_82466E20(ctx, base);
	// 8267E6F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267E6F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267E6F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267E6FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267E700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267E700 size=112
    let mut pc: u32 = 0x8267E700;
    'dispatch: loop {
        match pc {
            0x8267E700 => {
    //   block [0x8267E700..0x8267E770)
	// 8267E700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267E704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267E708: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267E70C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E710: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267E714: 38AA7938  addi r5, r10, 0x7938
	ctx.r[5].s64 = ctx.r[10].s64 + 31032;
	// 8267E718: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267E71C: 390BA9F8  addi r8, r11, -0x5608
	ctx.r[8].s64 = ctx.r[11].s64 + -22024;
	// 8267E720: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8267E724: 388A3BA8  addi r4, r10, 0x3ba8
	ctx.r[4].s64 = ctx.r[10].s64 + 15272;
	// 8267E728: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267E72C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E730: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267E734: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267E738: 386A7A28  addi r3, r10, 0x7a28
	ctx.r[3].s64 = ctx.r[10].s64 + 31272;
	// 8267E73C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267E740: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267E744: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267E748: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267E74C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267E750: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267E754: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267E758: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267E75C: 4BDE86C5  bl 0x82466e20
	ctx.lr = 0x8267E760;
	sub_82466E20(ctx, base);
	// 8267E760: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267E764: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267E768: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267E76C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267E770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267E770 size=100
    let mut pc: u32 = 0x8267E770;
    'dispatch: loop {
        match pc {
            0x8267E770 => {
    //   block [0x8267E770..0x8267E7D4)
	// 8267E770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267E774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267E778: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267E77C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E780: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267E784: 38AA7938  addi r5, r10, 0x7938
	ctx.r[5].s64 = ctx.r[10].s64 + 31032;
	// 8267E788: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267E78C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267E790: 388A3BB4  addi r4, r10, 0x3bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 15284;
	// 8267E794: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E798: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267E79C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267E7A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267E7A4: 386A7A58  addi r3, r10, 0x7a58
	ctx.r[3].s64 = ctx.r[10].s64 + 31320;
	// 8267E7A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267E7AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267E7B0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8267E7B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267E7B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267E7BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267E7C0: 4BDE8661  bl 0x82466e20
	ctx.lr = 0x8267E7C4;
	sub_82466E20(ctx, base);
	// 8267E7C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267E7C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267E7CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267E7D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267E7D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267E7D8 size=116
    let mut pc: u32 = 0x8267E7D8;
    'dispatch: loop {
        match pc {
            0x8267E7D8 => {
    //   block [0x8267E7D8..0x8267E84C)
	// 8267E7D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267E7DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267E7E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267E7E4: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267E7E8: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8267E7EC: 390BAA2C  addi r8, r11, -0x55d4
	ctx.r[8].s64 = ctx.r[11].s64 + -21972;
	// 8267E7F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267E7F4: 392A4554  addi r9, r10, 0x4554
	ctx.r[9].s64 = ctx.r[10].s64 + 17748;
	// 8267E7F8: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E7FC: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 8267E800: 38AA7AE8  addi r5, r10, 0x7ae8
	ctx.r[5].s64 = ctx.r[10].s64 + 31464;
	// 8267E804: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267E808: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267E80C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267E810: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267E814: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267E818: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267E81C: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 8267E820: 388A3BC4  addi r4, r10, 0x3bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15300;
	// 8267E824: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267E828: 386B7A88  addi r3, r11, 0x7a88
	ctx.r[3].s64 = ctx.r[11].s64 + 31368;
	// 8267E82C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8267E830: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267E834: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267E838: 4BDE85E9  bl 0x82466e20
	ctx.lr = 0x8267E83C;
	sub_82466E20(ctx, base);
	// 8267E83C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267E840: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267E844: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267E848: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267E850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267E850 size=112
    let mut pc: u32 = 0x8267E850;
    'dispatch: loop {
        match pc {
            0x8267E850 => {
    //   block [0x8267E850..0x8267E8C0)
	// 8267E850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267E854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267E858: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267E85C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E860: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267E864: 38AA7938  addi r5, r10, 0x7938
	ctx.r[5].s64 = ctx.r[10].s64 + 31032;
	// 8267E868: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267E86C: 390BAA5C  addi r8, r11, -0x55a4
	ctx.r[8].s64 = ctx.r[11].s64 + -21924;
	// 8267E870: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8267E874: 388A3BD4  addi r4, r10, 0x3bd4
	ctx.r[4].s64 = ctx.r[10].s64 + 15316;
	// 8267E878: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267E87C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E880: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267E884: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267E888: 386A7AB8  addi r3, r10, 0x7ab8
	ctx.r[3].s64 = ctx.r[10].s64 + 31416;
	// 8267E88C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267E890: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267E894: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267E898: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267E89C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8267E8A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267E8A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267E8A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267E8AC: 4BDE8575  bl 0x82466e20
	ctx.lr = 0x8267E8B0;
	sub_82466E20(ctx, base);
	// 8267E8B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267E8B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267E8B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267E8BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267E8C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267E8C0 size=116
    let mut pc: u32 = 0x8267E8C0;
    'dispatch: loop {
        match pc {
            0x8267E8C0 => {
    //   block [0x8267E8C0..0x8267E934)
	// 8267E8C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267E8C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267E8C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267E8CC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267E8D0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8267E8D4: 390BAA78  addi r8, r11, -0x5588
	ctx.r[8].s64 = ctx.r[11].s64 + -21896;
	// 8267E8D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267E8DC: 392A4580  addi r9, r10, 0x4580
	ctx.r[9].s64 = ctx.r[10].s64 + 17792;
	// 8267E8E0: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8267E8E4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8267E8E8: 38AA8118  addi r5, r10, -0x7ee8
	ctx.r[5].s64 = ctx.r[10].s64 + -32488;
	// 8267E8EC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267E8F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267E8F4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267E8F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267E8FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267E900: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267E904: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 8267E908: 388A3BE8  addi r4, r10, 0x3be8
	ctx.r[4].s64 = ctx.r[10].s64 + 15336;
	// 8267E90C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267E910: 386B7AE8  addi r3, r11, 0x7ae8
	ctx.r[3].s64 = ctx.r[11].s64 + 31464;
	// 8267E914: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8267E918: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267E91C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267E920: 4BDE8501  bl 0x82466e20
	ctx.lr = 0x8267E924;
	sub_82466E20(ctx, base);
	// 8267E924: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267E928: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267E92C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267E930: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267E938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267E938 size=112
    let mut pc: u32 = 0x8267E938;
    'dispatch: loop {
        match pc {
            0x8267E938 => {
    //   block [0x8267E938..0x8267E9A8)
	// 8267E938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267E93C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267E940: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267E944: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E948: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267E94C: 38AA7AE8  addi r5, r10, 0x7ae8
	ctx.r[5].s64 = ctx.r[10].s64 + 31464;
	// 8267E950: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267E954: 390BAA90  addi r8, r11, -0x5570
	ctx.r[8].s64 = ctx.r[11].s64 + -21872;
	// 8267E958: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8267E95C: 388A3BF8  addi r4, r10, 0x3bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 15352;
	// 8267E960: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267E964: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E968: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267E96C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267E970: 386A7B18  addi r3, r10, 0x7b18
	ctx.r[3].s64 = ctx.r[10].s64 + 31512;
	// 8267E974: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267E978: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267E97C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267E980: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267E984: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8267E988: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267E98C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267E990: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267E994: 4BDE848D  bl 0x82466e20
	ctx.lr = 0x8267E998;
	sub_82466E20(ctx, base);
	// 8267E998: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267E99C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267E9A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267E9A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267E9A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267E9A8 size=112
    let mut pc: u32 = 0x8267E9A8;
    'dispatch: loop {
        match pc {
            0x8267E9A8 => {
    //   block [0x8267E9A8..0x8267EA18)
	// 8267E9A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267E9AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267E9B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267E9B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E9B8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267E9BC: 38AA7AB8  addi r5, r10, 0x7ab8
	ctx.r[5].s64 = ctx.r[10].s64 + 31416;
	// 8267E9C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267E9C4: 390BAB08  addi r8, r11, -0x54f8
	ctx.r[8].s64 = ctx.r[11].s64 + -21752;
	// 8267E9C8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8267E9CC: 388A3C0C  addi r4, r10, 0x3c0c
	ctx.r[4].s64 = ctx.r[10].s64 + 15372;
	// 8267E9D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267E9D4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E9D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267E9DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267E9E0: 386A7B48  addi r3, r10, 0x7b48
	ctx.r[3].s64 = ctx.r[10].s64 + 31560;
	// 8267E9E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267E9E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267E9EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267E9F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267E9F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267E9F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267E9FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267EA00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267EA04: 4BDE841D  bl 0x82466e20
	ctx.lr = 0x8267EA08;
	sub_82466E20(ctx, base);
	// 8267EA08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267EA0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267EA10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267EA14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267EA18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267EA18 size=112
    let mut pc: u32 = 0x8267EA18;
    'dispatch: loop {
        match pc {
            0x8267EA18 => {
    //   block [0x8267EA18..0x8267EA88)
	// 8267EA18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267EA1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267EA20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267EA24: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267EA28: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267EA2C: 38AA7AE8  addi r5, r10, 0x7ae8
	ctx.r[5].s64 = ctx.r[10].s64 + 31464;
	// 8267EA30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267EA34: 390BAB50  addi r8, r11, -0x54b0
	ctx.r[8].s64 = ctx.r[11].s64 + -21680;
	// 8267EA38: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8267EA3C: 388A3C24  addi r4, r10, 0x3c24
	ctx.r[4].s64 = ctx.r[10].s64 + 15396;
	// 8267EA40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267EA44: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267EA48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267EA4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267EA50: 386A7B78  addi r3, r10, 0x7b78
	ctx.r[3].s64 = ctx.r[10].s64 + 31608;
	// 8267EA54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267EA58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267EA5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267EA60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267EA64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267EA68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267EA6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267EA70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267EA74: 4BDE83AD  bl 0x82466e20
	ctx.lr = 0x8267EA78;
	sub_82466E20(ctx, base);
	// 8267EA78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267EA7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267EA80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267EA84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267EA88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267EA88 size=112
    let mut pc: u32 = 0x8267EA88;
    'dispatch: loop {
        match pc {
            0x8267EA88 => {
    //   block [0x8267EA88..0x8267EAF8)
	// 8267EA88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267EA8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267EA90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267EA94: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267EA98: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267EA9C: 38AA7AE8  addi r5, r10, 0x7ae8
	ctx.r[5].s64 = ctx.r[10].s64 + 31464;
	// 8267EAA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267EAA4: 390BAB98  addi r8, r11, -0x5468
	ctx.r[8].s64 = ctx.r[11].s64 + -21608;
	// 8267EAA8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8267EAAC: 388A3C3C  addi r4, r10, 0x3c3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15420;
	// 8267EAB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267EAB4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267EAB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267EABC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267EAC0: 386A7BA8  addi r3, r10, 0x7ba8
	ctx.r[3].s64 = ctx.r[10].s64 + 31656;
	// 8267EAC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267EAC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267EACC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267EAD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267EAD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267EAD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267EADC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267EAE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267EAE4: 4BDE833D  bl 0x82466e20
	ctx.lr = 0x8267EAE8;
	sub_82466E20(ctx, base);
	// 8267EAE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267EAEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267EAF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267EAF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267EAF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267EAF8 size=108
    let mut pc: u32 = 0x8267EAF8;
    'dispatch: loop {
        match pc {
            0x8267EAF8 => {
    //   block [0x8267EAF8..0x8267EB64)
	// 8267EAF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267EAFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267EB00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267EB04: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267EB08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267EB0C: 38EBABE0  addi r7, r11, -0x5420
	ctx.r[7].s64 = ctx.r[11].s64 + -21536;
	// 8267EB10: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8267EB14: 388A3C54  addi r4, r10, 0x3c54
	ctx.r[4].s64 = ctx.r[10].s64 + 15444;
	// 8267EB18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267EB1C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267EB20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267EB24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267EB28: 386A7BD8  addi r3, r10, 0x7bd8
	ctx.r[3].s64 = ctx.r[10].s64 + 31704;
	// 8267EB2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267EB30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267EB34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267EB38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267EB3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267EB40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267EB44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267EB48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267EB4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267EB50: 4BDE82D1  bl 0x82466e20
	ctx.lr = 0x8267EB54;
	sub_82466E20(ctx, base);
	// 8267EB54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267EB58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267EB5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267EB60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267EB68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267EB68 size=112
    let mut pc: u32 = 0x8267EB68;
    'dispatch: loop {
        match pc {
            0x8267EB68 => {
    //   block [0x8267EB68..0x8267EBD8)
	// 8267EB68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267EB6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267EB70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267EB74: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267EB78: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267EB7C: 38AA7AE8  addi r5, r10, 0x7ae8
	ctx.r[5].s64 = ctx.r[10].s64 + 31464;
	// 8267EB80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267EB84: 390BAC28  addi r8, r11, -0x53d8
	ctx.r[8].s64 = ctx.r[11].s64 + -21464;
	// 8267EB88: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8267EB8C: 388A3C78  addi r4, r10, 0x3c78
	ctx.r[4].s64 = ctx.r[10].s64 + 15480;
	// 8267EB90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267EB94: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267EB98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267EB9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267EBA0: 386A7C08  addi r3, r10, 0x7c08
	ctx.r[3].s64 = ctx.r[10].s64 + 31752;
	// 8267EBA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267EBA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267EBAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267EBB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267EBB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267EBB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267EBBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267EBC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267EBC4: 4BDE825D  bl 0x82466e20
	ctx.lr = 0x8267EBC8;
	sub_82466E20(ctx, base);
	// 8267EBC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267EBCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267EBD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267EBD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267EBD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267EBD8 size=116
    let mut pc: u32 = 0x8267EBD8;
    'dispatch: loop {
        match pc {
            0x8267EBD8 => {
    //   block [0x8267EBD8..0x8267EC4C)
	// 8267EBD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267EBDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267EBE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267EBE4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8267EBE8: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267EBEC: 392B45BC  addi r9, r11, 0x45bc
	ctx.r[9].s64 = ctx.r[11].s64 + 17852;
	// 8267EBF0: 38AA7AE8  addi r5, r10, 0x7ae8
	ctx.r[5].s64 = ctx.r[10].s64 + 31464;
	// 8267EBF4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267EBF8: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 8267EBFC: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 8267EC00: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267EC04: 388A3C90  addi r4, r10, 0x3c90
	ctx.r[4].s64 = ctx.r[10].s64 + 15504;
	// 8267EC08: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267EC0C: 396BACA8  addi r11, r11, -0x5358
	ctx.r[11].s64 = ctx.r[11].s64 + -21336;
	// 8267EC10: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8267EC14: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267EC18: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8267EC1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267EC20: 386A7C38  addi r3, r10, 0x7c38
	ctx.r[3].s64 = ctx.r[10].s64 + 31800;
	// 8267EC24: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8267EC28: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8267EC2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267EC30: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8267EC34: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267EC38: 4BDE81E9  bl 0x82466e20
	ctx.lr = 0x8267EC3C;
	sub_82466E20(ctx, base);
	// 8267EC3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267EC40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267EC44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267EC48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267EC50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8267EC50 size=36
    let mut pc: u32 = 0x8267EC50;
    'dispatch: loop {
        match pc {
            0x8267EC50 => {
    //   block [0x8267EC50..0x8267EC74)
	// 8267EC50: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267EC54: 814BAD3C  lwz r10, -0x52c4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-21188 as u32) ) } as u64;
	// 8267EC58: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267EC5C: 396BEEA0  addi r11, r11, -0x1160
	ctx.r[11].s64 = ctx.r[11].s64 + -4448;
	// 8267EC60: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8267EC64: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267EC68: 814AACA4  lwz r10, -0x535c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-21340 as u32) ) } as u64;
	// 8267EC6C: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 8267EC70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267EC78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267EC78 size=108
    let mut pc: u32 = 0x8267EC78;
    'dispatch: loop {
        match pc {
            0x8267EC78 => {
    //   block [0x8267EC78..0x8267ECE4)
	// 8267EC78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267EC7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267EC80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267EC84: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267EC88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267EC8C: 38EBEEA0  addi r7, r11, -0x1160
	ctx.r[7].s64 = ctx.r[11].s64 + -4448;
	// 8267EC90: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 8267EC94: 388A3D1C  addi r4, r10, 0x3d1c
	ctx.r[4].s64 = ctx.r[10].s64 + 15644;
	// 8267EC98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267EC9C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267ECA0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267ECA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267ECA8: 386A7C68  addi r3, r10, 0x7c68
	ctx.r[3].s64 = ctx.r[10].s64 + 31848;
	// 8267ECAC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267ECB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267ECB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267ECB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267ECBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267ECC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267ECC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267ECC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267ECCC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267ECD0: 4BDE8151  bl 0x82466e20
	ctx.lr = 0x8267ECD4;
	sub_82466E20(ctx, base);
	// 8267ECD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267ECD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267ECDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267ECE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267ECE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8267ECE8 size=24
    let mut pc: u32 = 0x8267ECE8;
    'dispatch: loop {
        match pc {
            0x8267ECE8 => {
    //   block [0x8267ECE8..0x8267ED00)
	// 8267ECE8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267ECEC: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267ECF0: 394AEF48  addi r10, r10, -0x10b8
	ctx.r[10].s64 = ctx.r[10].s64 + -4280;
	// 8267ECF4: 816BACA4  lwz r11, -0x535c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-21340 as u32) ) } as u64;
	// 8267ECF8: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 8267ECFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267ED00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267ED00 size=116
    let mut pc: u32 = 0x8267ED00;
    'dispatch: loop {
        match pc {
            0x8267ED00 => {
    //   block [0x8267ED00..0x8267ED74)
	// 8267ED00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267ED04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267ED08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267ED0C: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267ED10: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 8267ED14: 390AEF48  addi r8, r10, -0x10b8
	ctx.r[8].s64 = ctx.r[10].s64 + -4280;
	// 8267ED18: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267ED1C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8267ED20: 38AA7C68  addi r5, r10, 0x7c68
	ctx.r[5].s64 = ctx.r[10].s64 + 31848;
	// 8267ED24: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267ED28: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267ED2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267ED30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267ED34: 388A3D58  addi r4, r10, 0x3d58
	ctx.r[4].s64 = ctx.r[10].s64 + 15704;
	// 8267ED38: 396B4678  addi r11, r11, 0x4678
	ctx.r[11].s64 = ctx.r[11].s64 + 18040;
	// 8267ED3C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267ED40: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267ED44: 386A7C98  addi r3, r10, 0x7c98
	ctx.r[3].s64 = ctx.r[10].s64 + 31896;
	// 8267ED48: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8267ED4C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267ED50: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8267ED54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267ED58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267ED5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267ED60: 4BDE80C1  bl 0x82466e20
	ctx.lr = 0x8267ED64;
	sub_82466E20(ctx, base);
	// 8267ED64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267ED68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267ED6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267ED70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267ED78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267ED78 size=112
    let mut pc: u32 = 0x8267ED78;
    'dispatch: loop {
        match pc {
            0x8267ED78 => {
    //   block [0x8267ED78..0x8267EDE8)
	// 8267ED78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267ED7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267ED80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267ED84: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267ED88: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267ED8C: 38AA7C68  addi r5, r10, 0x7c68
	ctx.r[5].s64 = ctx.r[10].s64 + 31848;
	// 8267ED90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267ED94: 390BAD40  addi r8, r11, -0x52c0
	ctx.r[8].s64 = ctx.r[11].s64 + -21184;
	// 8267ED98: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8267ED9C: 388A3D7C  addi r4, r10, 0x3d7c
	ctx.r[4].s64 = ctx.r[10].s64 + 15740;
	// 8267EDA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267EDA4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267EDA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267EDAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267EDB0: 386A7CC8  addi r3, r10, 0x7cc8
	ctx.r[3].s64 = ctx.r[10].s64 + 31944;
	// 8267EDB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267EDB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267EDBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267EDC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267EDC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267EDC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267EDCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267EDD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267EDD4: 4BDE804D  bl 0x82466e20
	ctx.lr = 0x8267EDD8;
	sub_82466E20(ctx, base);
	// 8267EDD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267EDDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267EDE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267EDE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267EDE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8267EDE8 size=24
    let mut pc: u32 = 0x8267EDE8;
    'dispatch: loop {
        match pc {
            0x8267EDE8 => {
    //   block [0x8267EDE8..0x8267EE00)
	// 8267EDE8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267EDEC: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267EDF0: 394AF008  addi r10, r10, -0xff8
	ctx.r[10].s64 = ctx.r[10].s64 + -4088;
	// 8267EDF4: 816BB3D8  lwz r11, -0x4c28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19496 as u32) ) } as u64;
	// 8267EDF8: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 8267EDFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267EE00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267EE00 size=116
    let mut pc: u32 = 0x8267EE00;
    'dispatch: loop {
        match pc {
            0x8267EE00 => {
    //   block [0x8267EE00..0x8267EE74)
	// 8267EE00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267EE04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267EE08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267EE0C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8267EE10: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267EE14: 392B463C  addi r9, r11, 0x463c
	ctx.r[9].s64 = ctx.r[11].s64 + 17980;
	// 8267EE18: 38AA7AB8  addi r5, r10, 0x7ab8
	ctx.r[5].s64 = ctx.r[10].s64 + 31416;
	// 8267EE1C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267EE20: 38E90060  addi r7, r9, 0x60
	ctx.r[7].s64 = ctx.r[9].s64 + 96;
	// 8267EE24: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8267EE28: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267EE2C: 388A3DA0  addi r4, r10, 0x3da0
	ctx.r[4].s64 = ctx.r[10].s64 + 15776;
	// 8267EE30: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267EE34: 396BF008  addi r11, r11, -0xff8
	ctx.r[11].s64 = ctx.r[11].s64 + -4088;
	// 8267EE38: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8267EE3C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267EE40: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8267EE44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267EE48: 386A7CF8  addi r3, r10, 0x7cf8
	ctx.r[3].s64 = ctx.r[10].s64 + 31992;
	// 8267EE4C: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8267EE50: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8267EE54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267EE58: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8267EE5C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267EE60: 4BDE7FC1  bl 0x82466e20
	ctx.lr = 0x8267EE64;
	sub_82466E20(ctx, base);
	// 8267EE64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267EE68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267EE6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267EE70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267EE78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267EE78 size=100
    let mut pc: u32 = 0x8267EE78;
    'dispatch: loop {
        match pc {
            0x8267EE78 => {
    //   block [0x8267EE78..0x8267EEDC)
	// 8267EE78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267EE7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267EE80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267EE84: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267EE88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267EE8C: 38AA7E48  addi r5, r10, 0x7e48
	ctx.r[5].s64 = ctx.r[10].s64 + 32328;
	// 8267EE90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267EE94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267EE98: 388A3DB4  addi r4, r10, 0x3db4
	ctx.r[4].s64 = ctx.r[10].s64 + 15796;
	// 8267EE9C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267EEA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267EEA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267EEA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267EEAC: 386A7D28  addi r3, r10, 0x7d28
	ctx.r[3].s64 = ctx.r[10].s64 + 32040;
	// 8267EEB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267EEB4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267EEB8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8267EEBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267EEC0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267EEC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267EEC8: 4BDE7F59  bl 0x82466e20
	ctx.lr = 0x8267EECC;
	sub_82466E20(ctx, base);
	// 8267EECC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267EED0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267EED4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267EED8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267EEE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267EEE0 size=100
    let mut pc: u32 = 0x8267EEE0;
    'dispatch: loop {
        match pc {
            0x8267EEE0 => {
    //   block [0x8267EEE0..0x8267EF44)
	// 8267EEE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267EEE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267EEE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267EEEC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267EEF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267EEF4: 38AA7938  addi r5, r10, 0x7938
	ctx.r[5].s64 = ctx.r[10].s64 + 31032;
	// 8267EEF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267EEFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267EF00: 388A3DC4  addi r4, r10, 0x3dc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15812;
	// 8267EF04: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267EF08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267EF0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267EF10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267EF14: 386A7D58  addi r3, r10, 0x7d58
	ctx.r[3].s64 = ctx.r[10].s64 + 32088;
	// 8267EF18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267EF1C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267EF20: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8267EF24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267EF28: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267EF2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267EF30: 4BDE7EF1  bl 0x82466e20
	ctx.lr = 0x8267EF34;
	sub_82466E20(ctx, base);
	// 8267EF34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267EF38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267EF3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267EF40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267EF48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267EF48 size=108
    let mut pc: u32 = 0x8267EF48;
    'dispatch: loop {
        match pc {
            0x8267EF48 => {
    //   block [0x8267EF48..0x8267EFB4)
	// 8267EF48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267EF4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267EF50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267EF54: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267EF58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267EF5C: 38EBADA0  addi r7, r11, -0x5260
	ctx.r[7].s64 = ctx.r[11].s64 + -21088;
	// 8267EF60: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8267EF64: 388A3DD8  addi r4, r10, 0x3dd8
	ctx.r[4].s64 = ctx.r[10].s64 + 15832;
	// 8267EF68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267EF6C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267EF70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267EF74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267EF78: 386A7D88  addi r3, r10, 0x7d88
	ctx.r[3].s64 = ctx.r[10].s64 + 32136;
	// 8267EF7C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267EF80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267EF84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267EF88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267EF8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267EF90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267EF94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267EF98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267EF9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267EFA0: 4BDE7E81  bl 0x82466e20
	ctx.lr = 0x8267EFA4;
	sub_82466E20(ctx, base);
	// 8267EFA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267EFA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267EFAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267EFB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267EFB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267EFB8 size=112
    let mut pc: u32 = 0x8267EFB8;
    'dispatch: loop {
        match pc {
            0x8267EFB8 => {
    //   block [0x8267EFB8..0x8267F028)
	// 8267EFB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267EFBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267EFC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267EFC4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267EFC8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267EFCC: 38AA7AB8  addi r5, r10, 0x7ab8
	ctx.r[5].s64 = ctx.r[10].s64 + 31416;
	// 8267EFD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267EFD4: 390BADE8  addi r8, r11, -0x5218
	ctx.r[8].s64 = ctx.r[11].s64 + -21016;
	// 8267EFD8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8267EFDC: 388A3DF0  addi r4, r10, 0x3df0
	ctx.r[4].s64 = ctx.r[10].s64 + 15856;
	// 8267EFE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267EFE4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267EFE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267EFEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267EFF0: 386A7DB8  addi r3, r10, 0x7db8
	ctx.r[3].s64 = ctx.r[10].s64 + 32184;
	// 8267EFF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267EFF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267EFFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267F000: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267F004: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267F008: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267F00C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267F010: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267F014: 4BDE7E0D  bl 0x82466e20
	ctx.lr = 0x8267F018;
	sub_82466E20(ctx, base);
	// 8267F018: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267F01C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267F020: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267F024: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267F028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267F028 size=108
    let mut pc: u32 = 0x8267F028;
    'dispatch: loop {
        match pc {
            0x8267F028 => {
    //   block [0x8267F028..0x8267F094)
	// 8267F028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267F02C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267F030: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267F034: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267F038: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267F03C: 38EBAE30  addi r7, r11, -0x51d0
	ctx.r[7].s64 = ctx.r[11].s64 + -20944;
	// 8267F040: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8267F044: 388A3DFC  addi r4, r10, 0x3dfc
	ctx.r[4].s64 = ctx.r[10].s64 + 15868;
	// 8267F048: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267F04C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267F050: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267F054: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267F058: 386A7DE8  addi r3, r10, 0x7de8
	ctx.r[3].s64 = ctx.r[10].s64 + 32232;
	// 8267F05C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267F060: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267F064: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267F068: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267F06C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267F070: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267F074: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267F078: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267F07C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267F080: 4BDE7DA1  bl 0x82466e20
	ctx.lr = 0x8267F084;
	sub_82466E20(ctx, base);
	// 8267F084: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267F088: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267F08C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267F090: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267F098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8267F098 size=28
    let mut pc: u32 = 0x8267F098;
    'dispatch: loop {
        match pc {
            0x8267F098 => {
    //   block [0x8267F098..0x8267F0B4)
	// 8267F098: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267F09C: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267F0A0: 394AF0C8  addi r10, r10, -0xf38
	ctx.r[10].s64 = ctx.r[10].s64 + -3896;
	// 8267F0A4: 816BAE48  lwz r11, -0x51b8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20920 as u32) ) } as u64;
	// 8267F0A8: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8267F0AC: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 8267F0B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267F0B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267F0B8 size=112
    let mut pc: u32 = 0x8267F0B8;
    'dispatch: loop {
        match pc {
            0x8267F0B8 => {
    //   block [0x8267F0B8..0x8267F128)
	// 8267F0B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267F0BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267F0C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267F0C4: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267F0C8: 3900000E  li r8, 0xe
	ctx.r[8].s64 = 14;
	// 8267F0CC: 38EAF0C8  addi r7, r10, -0xf38
	ctx.r[7].s64 = ctx.r[10].s64 + -3896;
	// 8267F0D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267F0D4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8267F0D8: 388A3E1C  addi r4, r10, 0x3e1c
	ctx.r[4].s64 = ctx.r[10].s64 + 15900;
	// 8267F0DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267F0E0: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267F0E4: 396B4728  addi r11, r11, 0x4728
	ctx.r[11].s64 = ctx.r[11].s64 + 18216;
	// 8267F0E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267F0EC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267F0F0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267F0F4: 386A7E18  addi r3, r10, 0x7e18
	ctx.r[3].s64 = ctx.r[10].s64 + 32280;
	// 8267F0F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267F0FC: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8267F100: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267F104: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8267F108: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267F10C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267F110: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267F114: 4BDE7D0D  bl 0x82466e20
	ctx.lr = 0x8267F118;
	sub_82466E20(ctx, base);
	// 8267F118: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267F11C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267F120: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267F124: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267F128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8267F128 size=24
    let mut pc: u32 = 0x8267F128;
    'dispatch: loop {
        match pc {
            0x8267F128 => {
    //   block [0x8267F128..0x8267F140)
	// 8267F128: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267F12C: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267F130: 394AF218  addi r10, r10, -0xde8
	ctx.r[10].s64 = ctx.r[10].s64 + -3560;
	// 8267F134: 816BB3D8  lwz r11, -0x4c28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19496 as u32) ) } as u64;
	// 8267F138: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8267F13C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267F140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267F140 size=116
    let mut pc: u32 = 0x8267F140;
    'dispatch: loop {
        match pc {
            0x8267F140 => {
    //   block [0x8267F140..0x8267F1B4)
	// 8267F140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267F144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267F148: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267F14C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8267F150: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267F154: 392B4700  addi r9, r11, 0x4700
	ctx.r[9].s64 = ctx.r[11].s64 + 18176;
	// 8267F158: 38AA7AB8  addi r5, r10, 0x7ab8
	ctx.r[5].s64 = ctx.r[10].s64 + 31416;
	// 8267F15C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267F160: 38E90064  addi r7, r9, 0x64
	ctx.r[7].s64 = ctx.r[9].s64 + 100;
	// 8267F164: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 8267F168: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267F16C: 388A3E30  addi r4, r10, 0x3e30
	ctx.r[4].s64 = ctx.r[10].s64 + 15920;
	// 8267F170: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267F174: 396BF218  addi r11, r11, -0xde8
	ctx.r[11].s64 = ctx.r[11].s64 + -3560;
	// 8267F178: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8267F17C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267F180: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8267F184: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267F188: 386A7E48  addi r3, r10, 0x7e48
	ctx.r[3].s64 = ctx.r[10].s64 + 32328;
	// 8267F18C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8267F190: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8267F194: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267F198: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8267F19C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267F1A0: 4BDE7C81  bl 0x82466e20
	ctx.lr = 0x8267F1A4;
	sub_82466E20(ctx, base);
	// 8267F1A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267F1A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267F1AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267F1B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267F1B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267F1B8 size=112
    let mut pc: u32 = 0x8267F1B8;
    'dispatch: loop {
        match pc {
            0x8267F1B8 => {
    //   block [0x8267F1B8..0x8267F228)
	// 8267F1B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267F1BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267F1C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267F1C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267F1C8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267F1CC: 38AA7A58  addi r5, r10, 0x7a58
	ctx.r[5].s64 = ctx.r[10].s64 + 31320;
	// 8267F1D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267F1D4: 390BAE50  addi r8, r11, -0x51b0
	ctx.r[8].s64 = ctx.r[11].s64 + -20912;
	// 8267F1D8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8267F1DC: 388A3E3C  addi r4, r10, 0x3e3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15932;
	// 8267F1E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267F1E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267F1E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267F1EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267F1F0: 386A7E78  addi r3, r10, 0x7e78
	ctx.r[3].s64 = ctx.r[10].s64 + 32376;
	// 8267F1F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267F1F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267F1FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267F200: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267F204: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267F208: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267F20C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267F210: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267F214: 4BDE7C0D  bl 0x82466e20
	ctx.lr = 0x8267F218;
	sub_82466E20(ctx, base);
	// 8267F218: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267F21C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267F220: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267F224: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267F228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8267F228 size=24
    let mut pc: u32 = 0x8267F228;
    'dispatch: loop {
        match pc {
            0x8267F228 => {
    //   block [0x8267F228..0x8267F240)
	// 8267F228: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267F22C: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267F230: 394AF2C0  addi r10, r10, -0xd40
	ctx.r[10].s64 = ctx.r[10].s64 + -3392;
	// 8267F234: 816BB3D8  lwz r11, -0x4c28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19496 as u32) ) } as u64;
	// 8267F238: 916A00B0  stw r11, 0xb0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 8267F23C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267F240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267F240 size=116
    let mut pc: u32 = 0x8267F240;
    'dispatch: loop {
        match pc {
            0x8267F240 => {
    //   block [0x8267F240..0x8267F2B4)
	// 8267F240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267F244: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267F248: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267F24C: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267F250: 38E0000C  li r7, 0xc
	ctx.r[7].s64 = 12;
	// 8267F254: 390AF2C0  addi r8, r10, -0xd40
	ctx.r[8].s64 = ctx.r[10].s64 + -3392;
	// 8267F258: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267F25C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8267F260: 38AA7A58  addi r5, r10, 0x7a58
	ctx.r[5].s64 = ctx.r[10].s64 + 31320;
	// 8267F264: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267F268: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267F26C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267F270: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267F274: 388A3E64  addi r4, r10, 0x3e64
	ctx.r[4].s64 = ctx.r[10].s64 + 15972;
	// 8267F278: 396B4784  addi r11, r11, 0x4784
	ctx.r[11].s64 = ctx.r[11].s64 + 18308;
	// 8267F27C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267F280: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8267F284: 386A7EA8  addi r3, r10, 0x7ea8
	ctx.r[3].s64 = ctx.r[10].s64 + 32424;
	// 8267F288: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8267F28C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267F290: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8267F294: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267F298: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267F29C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267F2A0: 4BDE7B81  bl 0x82466e20
	ctx.lr = 0x8267F2A4;
	sub_82466E20(ctx, base);
	// 8267F2A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267F2A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267F2AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267F2B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267F2B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267F2B8 size=112
    let mut pc: u32 = 0x8267F2B8;
    'dispatch: loop {
        match pc {
            0x8267F2B8 => {
    //   block [0x8267F2B8..0x8267F328)
	// 8267F2B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267F2BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267F2C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267F2C4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8267F2C8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267F2CC: 38AA9828  addi r5, r10, -0x67d8
	ctx.r[5].s64 = ctx.r[10].s64 + -26584;
	// 8267F2D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267F2D4: 390BAE80  addi r8, r11, -0x5180
	ctx.r[8].s64 = ctx.r[11].s64 + -20864;
	// 8267F2D8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8267F2DC: 388A3E78  addi r4, r10, 0x3e78
	ctx.r[4].s64 = ctx.r[10].s64 + 15992;
	// 8267F2E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267F2E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267F2E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267F2EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267F2F0: 386A7ED8  addi r3, r10, 0x7ed8
	ctx.r[3].s64 = ctx.r[10].s64 + 32472;
	// 8267F2F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267F2F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267F2FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267F300: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267F304: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267F308: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267F30C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267F310: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267F314: 4BDE7B0D  bl 0x82466e20
	ctx.lr = 0x8267F318;
	sub_82466E20(ctx, base);
	// 8267F318: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267F31C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267F320: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267F324: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267F328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267F328 size=108
    let mut pc: u32 = 0x8267F328;
    'dispatch: loop {
        match pc {
            0x8267F328 => {
    //   block [0x8267F328..0x8267F394)
	// 8267F328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267F32C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267F330: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267F334: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267F338: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267F33C: 38EBAEB0  addi r7, r11, -0x5150
	ctx.r[7].s64 = ctx.r[11].s64 + -20816;
	// 8267F340: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8267F344: 388A3E98  addi r4, r10, 0x3e98
	ctx.r[4].s64 = ctx.r[10].s64 + 16024;
	// 8267F348: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267F34C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267F350: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267F354: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267F358: 386A7F08  addi r3, r10, 0x7f08
	ctx.r[3].s64 = ctx.r[10].s64 + 32520;
	// 8267F35C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267F360: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267F364: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267F368: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267F36C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267F370: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267F374: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267F378: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267F37C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267F380: 4BDE7AA1  bl 0x82466e20
	ctx.lr = 0x8267F384;
	sub_82466E20(ctx, base);
	// 8267F384: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267F388: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267F38C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267F390: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267F398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267F398 size=112
    let mut pc: u32 = 0x8267F398;
    'dispatch: loop {
        match pc {
            0x8267F398 => {
    //   block [0x8267F398..0x8267F408)
	// 8267F398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267F39C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267F3A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267F3A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267F3A8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267F3AC: 38AA7938  addi r5, r10, 0x7938
	ctx.r[5].s64 = ctx.r[10].s64 + 31032;
	// 8267F3B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267F3B4: 390BAEE0  addi r8, r11, -0x5120
	ctx.r[8].s64 = ctx.r[11].s64 + -20768;
	// 8267F3B8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8267F3BC: 388A3EAC  addi r4, r10, 0x3eac
	ctx.r[4].s64 = ctx.r[10].s64 + 16044;
	// 8267F3C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267F3C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267F3C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267F3CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267F3D0: 386A7F38  addi r3, r10, 0x7f38
	ctx.r[3].s64 = ctx.r[10].s64 + 32568;
	// 8267F3D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267F3D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267F3DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267F3E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267F3E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267F3E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267F3EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267F3F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267F3F4: 4BDE7A2D  bl 0x82466e20
	ctx.lr = 0x8267F3F8;
	sub_82466E20(ctx, base);
	// 8267F3F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267F3FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267F400: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267F404: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267F408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267F408 size=112
    let mut pc: u32 = 0x8267F408;
    'dispatch: loop {
        match pc {
            0x8267F408 => {
    //   block [0x8267F408..0x8267F478)
	// 8267F408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267F40C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267F410: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267F414: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8267F418: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267F41C: 38AA8118  addi r5, r10, -0x7ee8
	ctx.r[5].s64 = ctx.r[10].s64 + -32488;
	// 8267F420: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267F424: 390BAF10  addi r8, r11, -0x50f0
	ctx.r[8].s64 = ctx.r[11].s64 + -20720;
	// 8267F428: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8267F42C: 388A3EBC  addi r4, r10, 0x3ebc
	ctx.r[4].s64 = ctx.r[10].s64 + 16060;
	// 8267F430: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267F434: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267F438: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267F43C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267F440: 386A7F68  addi r3, r10, 0x7f68
	ctx.r[3].s64 = ctx.r[10].s64 + 32616;
	// 8267F444: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267F448: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267F44C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267F450: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267F454: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267F458: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267F45C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267F460: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267F464: 4BDE79BD  bl 0x82466e20
	ctx.lr = 0x8267F468;
	sub_82466E20(ctx, base);
	// 8267F468: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267F46C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267F470: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267F474: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267F478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267F478 size=108
    let mut pc: u32 = 0x8267F478;
    'dispatch: loop {
        match pc {
            0x8267F478 => {
    //   block [0x8267F478..0x8267F4E4)
	// 8267F478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267F47C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267F480: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267F484: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267F488: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267F48C: 38EBAF40  addi r7, r11, -0x50c0
	ctx.r[7].s64 = ctx.r[11].s64 + -20672;
	// 8267F490: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8267F494: 388A3ED0  addi r4, r10, 0x3ed0
	ctx.r[4].s64 = ctx.r[10].s64 + 16080;
	// 8267F498: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267F49C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267F4A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267F4A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267F4A8: 386A7F98  addi r3, r10, 0x7f98
	ctx.r[3].s64 = ctx.r[10].s64 + 32664;
	// 8267F4AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267F4B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267F4B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267F4B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267F4BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267F4C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267F4C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267F4C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267F4CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267F4D0: 4BDE7951  bl 0x82466e20
	ctx.lr = 0x8267F4D4;
	sub_82466E20(ctx, base);
	// 8267F4D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267F4D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267F4DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267F4E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267F4E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267F4E8 size=112
    let mut pc: u32 = 0x8267F4E8;
    'dispatch: loop {
        match pc {
            0x8267F4E8 => {
    //   block [0x8267F4E8..0x8267F558)
	// 8267F4E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267F4EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267F4F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267F4F4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267F4F8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267F4FC: 38AA7AE8  addi r5, r10, 0x7ae8
	ctx.r[5].s64 = ctx.r[10].s64 + 31464;
	// 8267F500: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267F504: 390BAF88  addi r8, r11, -0x5078
	ctx.r[8].s64 = ctx.r[11].s64 + -20600;
	// 8267F508: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8267F50C: 388A3EF8  addi r4, r10, 0x3ef8
	ctx.r[4].s64 = ctx.r[10].s64 + 16120;
	// 8267F510: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267F514: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267F518: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267F51C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267F520: 386A7FC8  addi r3, r10, 0x7fc8
	ctx.r[3].s64 = ctx.r[10].s64 + 32712;
	// 8267F524: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267F528: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267F52C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267F530: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267F534: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267F538: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267F53C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267F540: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267F544: 4BDE78DD  bl 0x82466e20
	ctx.lr = 0x8267F548;
	sub_82466E20(ctx, base);
	// 8267F548: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267F54C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267F550: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267F554: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267F558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267F558 size=100
    let mut pc: u32 = 0x8267F558;
    'dispatch: loop {
        match pc {
            0x8267F558 => {
    //   block [0x8267F558..0x8267F5BC)
	// 8267F558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267F55C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267F560: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267F564: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267F568: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267F56C: 38AA7938  addi r5, r10, 0x7938
	ctx.r[5].s64 = ctx.r[10].s64 + 31032;
	// 8267F570: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267F574: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267F578: 388A3F14  addi r4, r10, 0x3f14
	ctx.r[4].s64 = ctx.r[10].s64 + 16148;
	// 8267F57C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267F580: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267F584: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267F588: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267F58C: 386A7FF8  addi r3, r10, 0x7ff8
	ctx.r[3].s64 = ctx.r[10].s64 + 32760;
	// 8267F590: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267F594: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267F598: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8267F59C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267F5A0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267F5A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267F5A8: 4BDE7879  bl 0x82466e20
	ctx.lr = 0x8267F5AC;
	sub_82466E20(ctx, base);
	// 8267F5AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267F5B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267F5B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267F5B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267F5C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267F5C0 size=112
    let mut pc: u32 = 0x8267F5C0;
    'dispatch: loop {
        match pc {
            0x8267F5C0 => {
    //   block [0x8267F5C0..0x8267F630)
	// 8267F5C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267F5C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267F5C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267F5CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267F5D0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267F5D4: 38AA7D58  addi r5, r10, 0x7d58
	ctx.r[5].s64 = ctx.r[10].s64 + 32088;
	// 8267F5D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267F5DC: 390BAFE8  addi r8, r11, -0x5018
	ctx.r[8].s64 = ctx.r[11].s64 + -20504;
	// 8267F5E0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8267F5E4: 388A3F2C  addi r4, r10, 0x3f2c
	ctx.r[4].s64 = ctx.r[10].s64 + 16172;
	// 8267F5E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267F5EC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8267F5F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267F5F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267F5F8: 386A8028  addi r3, r10, -0x7fd8
	ctx.r[3].s64 = ctx.r[10].s64 + -32728;
	// 8267F5FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267F600: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267F604: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267F608: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267F60C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267F610: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267F614: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267F618: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267F61C: 4BDE7805  bl 0x82466e20
	ctx.lr = 0x8267F620;
	sub_82466E20(ctx, base);
	// 8267F620: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267F624: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267F628: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267F62C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267F630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267F630 size=112
    let mut pc: u32 = 0x8267F630;
    'dispatch: loop {
        match pc {
            0x8267F630 => {
    //   block [0x8267F630..0x8267F6A0)
	// 8267F630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267F634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267F638: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267F63C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267F640: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267F644: 38AA7D58  addi r5, r10, 0x7d58
	ctx.r[5].s64 = ctx.r[10].s64 + 32088;
	// 8267F648: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267F64C: 390BB030  addi r8, r11, -0x4fd0
	ctx.r[8].s64 = ctx.r[11].s64 + -20432;
	// 8267F650: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8267F654: 388A3F3C  addi r4, r10, 0x3f3c
	ctx.r[4].s64 = ctx.r[10].s64 + 16188;
	// 8267F658: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267F65C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8267F660: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267F664: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267F668: 386A8058  addi r3, r10, -0x7fa8
	ctx.r[3].s64 = ctx.r[10].s64 + -32680;
	// 8267F66C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267F670: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267F674: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267F678: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267F67C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267F680: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267F684: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267F688: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267F68C: 4BDE7795  bl 0x82466e20
	ctx.lr = 0x8267F690;
	sub_82466E20(ctx, base);
	// 8267F690: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267F694: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267F698: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267F69C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267F6A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267F6A0 size=108
    let mut pc: u32 = 0x8267F6A0;
    'dispatch: loop {
        match pc {
            0x8267F6A0 => {
    //   block [0x8267F6A0..0x8267F70C)
	// 8267F6A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267F6A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267F6A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267F6AC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267F6B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267F6B4: 38EBB0D8  addi r7, r11, -0x4f28
	ctx.r[7].s64 = ctx.r[11].s64 + -20264;
	// 8267F6B8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8267F6BC: 388A3F58  addi r4, r10, 0x3f58
	ctx.r[4].s64 = ctx.r[10].s64 + 16216;
	// 8267F6C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267F6C4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8267F6C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267F6CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267F6D0: 386A8088  addi r3, r10, -0x7f78
	ctx.r[3].s64 = ctx.r[10].s64 + -32632;
	// 8267F6D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267F6D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267F6DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267F6E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267F6E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267F6E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267F6EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267F6F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267F6F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267F6F8: 4BDE7729  bl 0x82466e20
	ctx.lr = 0x8267F6FC;
	sub_82466E20(ctx, base);
	// 8267F6FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267F700: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267F704: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267F708: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267F710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8267F710 size=24
    let mut pc: u32 = 0x8267F710;
    'dispatch: loop {
        match pc {
            0x8267F710 => {
    //   block [0x8267F710..0x8267F728)
	// 8267F710: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267F714: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267F718: 394AF3E0  addi r10, r10, -0xc20
	ctx.r[10].s64 = ctx.r[10].s64 + -3104;
	// 8267F71C: 816BB3D8  lwz r11, -0x4c28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19496 as u32) ) } as u64;
	// 8267F720: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8267F724: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267F728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267F728 size=116
    let mut pc: u32 = 0x8267F728;
    'dispatch: loop {
        match pc {
            0x8267F728 => {
    //   block [0x8267F728..0x8267F79C)
	// 8267F728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267F72C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267F730: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267F734: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267F738: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8267F73C: 390AF3E0  addi r8, r10, -0xc20
	ctx.r[8].s64 = ctx.r[10].s64 + -3104;
	// 8267F740: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267F744: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8267F748: 38AA7AB8  addi r5, r10, 0x7ab8
	ctx.r[5].s64 = ctx.r[10].s64 + 31416;
	// 8267F74C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267F750: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267F754: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267F758: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267F75C: 388A3F74  addi r4, r10, 0x3f74
	ctx.r[4].s64 = ctx.r[10].s64 + 16244;
	// 8267F760: 396B47B8  addi r11, r11, 0x47b8
	ctx.r[11].s64 = ctx.r[11].s64 + 18360;
	// 8267F764: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8267F768: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267F76C: 386A80B8  addi r3, r10, -0x7f48
	ctx.r[3].s64 = ctx.r[10].s64 + -32584;
	// 8267F770: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8267F774: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267F778: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8267F77C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267F780: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267F784: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267F788: 4BDE7699  bl 0x82466e20
	ctx.lr = 0x8267F78C;
	sub_82466E20(ctx, base);
	// 8267F78C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267F790: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267F794: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267F798: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267F7A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267F7A0 size=100
    let mut pc: u32 = 0x8267F7A0;
    'dispatch: loop {
        match pc {
            0x8267F7A0 => {
    //   block [0x8267F7A0..0x8267F804)
	// 8267F7A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267F7A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267F7A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267F7AC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267F7B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267F7B4: 38AA7AE8  addi r5, r10, 0x7ae8
	ctx.r[5].s64 = ctx.r[10].s64 + 31464;
	// 8267F7B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267F7BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267F7C0: 388A3F88  addi r4, r10, 0x3f88
	ctx.r[4].s64 = ctx.r[10].s64 + 16264;
	// 8267F7C4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8267F7C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267F7CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267F7D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267F7D4: 386A80E8  addi r3, r10, -0x7f18
	ctx.r[3].s64 = ctx.r[10].s64 + -32536;
	// 8267F7D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267F7DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267F7E0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8267F7E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267F7E8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267F7EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267F7F0: 4BDE7631  bl 0x82466e20
	ctx.lr = 0x8267F7F4;
	sub_82466E20(ctx, base);
	// 8267F7F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267F7F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267F7FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267F800: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267F808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267F808 size=100
    let mut pc: u32 = 0x8267F808;
    'dispatch: loop {
        match pc {
            0x8267F808 => {
    //   block [0x8267F808..0x8267F86C)
	// 8267F808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267F80C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267F810: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267F814: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267F818: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267F81C: 38AA7938  addi r5, r10, 0x7938
	ctx.r[5].s64 = ctx.r[10].s64 + 31032;
	// 8267F820: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267F824: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267F828: 388A3F98  addi r4, r10, 0x3f98
	ctx.r[4].s64 = ctx.r[10].s64 + 16280;
	// 8267F82C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8267F830: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267F834: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267F838: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267F83C: 386A8118  addi r3, r10, -0x7ee8
	ctx.r[3].s64 = ctx.r[10].s64 + -32488;
	// 8267F840: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267F844: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267F848: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8267F84C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267F850: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267F854: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267F858: 4BDE75C9  bl 0x82466e20
	ctx.lr = 0x8267F85C;
	sub_82466E20(ctx, base);
	// 8267F85C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267F860: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267F864: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267F868: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267F870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267F870 size=112
    let mut pc: u32 = 0x8267F870;
    'dispatch: loop {
        match pc {
            0x8267F870 => {
    //   block [0x8267F870..0x8267F8E0)
	// 8267F870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267F874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267F878: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267F87C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267F880: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267F884: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 8267F888: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267F88C: 390BB138  addi r8, r11, -0x4ec8
	ctx.r[8].s64 = ctx.r[11].s64 + -20168;
	// 8267F890: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8267F894: 388A3FAC  addi r4, r10, 0x3fac
	ctx.r[4].s64 = ctx.r[10].s64 + 16300;
	// 8267F898: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267F89C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8267F8A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267F8A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267F8A8: 386A8148  addi r3, r10, -0x7eb8
	ctx.r[3].s64 = ctx.r[10].s64 + -32440;
	// 8267F8AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267F8B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267F8B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267F8B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267F8BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267F8C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267F8C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267F8C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267F8CC: 4BDE7555  bl 0x82466e20
	ctx.lr = 0x8267F8D0;
	sub_82466E20(ctx, base);
	// 8267F8D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267F8D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267F8D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267F8DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267F8E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267F8E0 size=112
    let mut pc: u32 = 0x8267F8E0;
    'dispatch: loop {
        match pc {
            0x8267F8E0 => {
    //   block [0x8267F8E0..0x8267F950)
	// 8267F8E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267F8E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267F8E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267F8EC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267F8F0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267F8F4: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 8267F8F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267F8FC: 390BB1C8  addi r8, r11, -0x4e38
	ctx.r[8].s64 = ctx.r[11].s64 + -20024;
	// 8267F900: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8267F904: 388A3FDC  addi r4, r10, 0x3fdc
	ctx.r[4].s64 = ctx.r[10].s64 + 16348;
	// 8267F908: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267F90C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8267F910: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267F914: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267F918: 386A8178  addi r3, r10, -0x7e88
	ctx.r[3].s64 = ctx.r[10].s64 + -32392;
	// 8267F91C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267F920: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267F924: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267F928: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267F92C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267F930: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267F934: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267F938: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267F93C: 4BDE74E5  bl 0x82466e20
	ctx.lr = 0x8267F940;
	sub_82466E20(ctx, base);
	// 8267F940: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267F944: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267F948: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267F94C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267F950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267F950 size=112
    let mut pc: u32 = 0x8267F950;
    'dispatch: loop {
        match pc {
            0x8267F950 => {
    //   block [0x8267F950..0x8267F9C0)
	// 8267F950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267F954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267F958: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267F95C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267F960: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267F964: 38AA7CF8  addi r5, r10, 0x7cf8
	ctx.r[5].s64 = ctx.r[10].s64 + 31992;
	// 8267F968: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267F96C: 390BB228  addi r8, r11, -0x4dd8
	ctx.r[8].s64 = ctx.r[11].s64 + -19928;
	// 8267F970: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8267F974: 388A400C  addi r4, r10, 0x400c
	ctx.r[4].s64 = ctx.r[10].s64 + 16396;
	// 8267F978: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267F97C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8267F980: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267F984: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267F988: 386A81A8  addi r3, r10, -0x7e58
	ctx.r[3].s64 = ctx.r[10].s64 + -32344;
	// 8267F98C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267F990: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267F994: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267F998: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267F99C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267F9A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267F9A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267F9A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267F9AC: 4BDE7475  bl 0x82466e20
	ctx.lr = 0x8267F9B0;
	sub_82466E20(ctx, base);
	// 8267F9B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267F9B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267F9B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267F9BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267F9C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267F9C0 size=112
    let mut pc: u32 = 0x8267F9C0;
    'dispatch: loop {
        match pc {
            0x8267F9C0 => {
    //   block [0x8267F9C0..0x8267FA30)
	// 8267F9C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267F9C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267F9C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267F9CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267F9D0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267F9D4: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 8267F9D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267F9DC: 390BB258  addi r8, r11, -0x4da8
	ctx.r[8].s64 = ctx.r[11].s64 + -19880;
	// 8267F9E0: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8267F9E4: 388A4028  addi r4, r10, 0x4028
	ctx.r[4].s64 = ctx.r[10].s64 + 16424;
	// 8267F9E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267F9EC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8267F9F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267F9F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267F9F8: 386A81D8  addi r3, r10, -0x7e28
	ctx.r[3].s64 = ctx.r[10].s64 + -32296;
	// 8267F9FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267FA00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267FA04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267FA08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267FA0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267FA10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267FA14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267FA18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267FA1C: 4BDE7405  bl 0x82466e20
	ctx.lr = 0x8267FA20;
	sub_82466E20(ctx, base);
	// 8267FA20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267FA24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267FA28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267FA2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267FA30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267FA30 size=112
    let mut pc: u32 = 0x8267FA30;
    'dispatch: loop {
        match pc {
            0x8267FA30 => {
    //   block [0x8267FA30..0x8267FAA0)
	// 8267FA30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267FA34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267FA38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267FA3C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267FA40: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267FA44: 38AA7E48  addi r5, r10, 0x7e48
	ctx.r[5].s64 = ctx.r[10].s64 + 32328;
	// 8267FA48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267FA4C: 390BB2E8  addi r8, r11, -0x4d18
	ctx.r[8].s64 = ctx.r[11].s64 + -19736;
	// 8267FA50: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8267FA54: 388A404C  addi r4, r10, 0x404c
	ctx.r[4].s64 = ctx.r[10].s64 + 16460;
	// 8267FA58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267FA5C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8267FA60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267FA64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267FA68: 386A8208  addi r3, r10, -0x7df8
	ctx.r[3].s64 = ctx.r[10].s64 + -32248;
	// 8267FA6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267FA70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267FA74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267FA78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267FA7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267FA80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267FA84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267FA88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267FA8C: 4BDE7395  bl 0x82466e20
	ctx.lr = 0x8267FA90;
	sub_82466E20(ctx, base);
	// 8267FA90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267FA94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267FA98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267FA9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267FAA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267FAA0 size=112
    let mut pc: u32 = 0x8267FAA0;
    'dispatch: loop {
        match pc {
            0x8267FAA0 => {
    //   block [0x8267FAA0..0x8267FB10)
	// 8267FAA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267FAA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267FAA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267FAAC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8267FAB0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267FAB4: 38AA8058  addi r5, r10, -0x7fa8
	ctx.r[5].s64 = ctx.r[10].s64 + -32680;
	// 8267FAB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267FABC: 390BB300  addi r8, r11, -0x4d00
	ctx.r[8].s64 = ctx.r[11].s64 + -19712;
	// 8267FAC0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8267FAC4: 388A4060  addi r4, r10, 0x4060
	ctx.r[4].s64 = ctx.r[10].s64 + 16480;
	// 8267FAC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267FACC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8267FAD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267FAD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267FAD8: 386A8238  addi r3, r10, -0x7dc8
	ctx.r[3].s64 = ctx.r[10].s64 + -32200;
	// 8267FADC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267FAE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267FAE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267FAE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267FAEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267FAF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267FAF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267FAF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267FAFC: 4BDE7325  bl 0x82466e20
	ctx.lr = 0x8267FB00;
	sub_82466E20(ctx, base);
	// 8267FB00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267FB04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267FB08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267FB0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267FB10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267FB10 size=112
    let mut pc: u32 = 0x8267FB10;
    'dispatch: loop {
        match pc {
            0x8267FB10 => {
    //   block [0x8267FB10..0x8267FB80)
	// 8267FB10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267FB14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267FB18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267FB1C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267FB20: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267FB24: 38AA7938  addi r5, r10, 0x7938
	ctx.r[5].s64 = ctx.r[10].s64 + 31032;
	// 8267FB28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267FB2C: 390BB330  addi r8, r11, -0x4cd0
	ctx.r[8].s64 = ctx.r[11].s64 + -19664;
	// 8267FB30: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8267FB34: 388A4084  addi r4, r10, 0x4084
	ctx.r[4].s64 = ctx.r[10].s64 + 16516;
	// 8267FB38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267FB3C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8267FB40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267FB44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267FB48: 386A8268  addi r3, r10, -0x7d98
	ctx.r[3].s64 = ctx.r[10].s64 + -32152;
	// 8267FB4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267FB50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267FB54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267FB58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267FB5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267FB60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267FB64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267FB68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267FB6C: 4BDE72B5  bl 0x82466e20
	ctx.lr = 0x8267FB70;
	sub_82466E20(ctx, base);
	// 8267FB70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267FB74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267FB78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267FB7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267FB80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8267FB80 size=24
    let mut pc: u32 = 0x8267FB80;
    'dispatch: loop {
        match pc {
            0x8267FB80 => {
    //   block [0x8267FB80..0x8267FB98)
	// 8267FB80: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267FB84: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267FB88: 394AF458  addi r10, r10, -0xba8
	ctx.r[10].s64 = ctx.r[10].s64 + -2984;
	// 8267FB8C: 816BB3D8  lwz r11, -0x4c28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19496 as u32) ) } as u64;
	// 8267FB90: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8267FB94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267FB98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267FB98 size=116
    let mut pc: u32 = 0x8267FB98;
    'dispatch: loop {
        match pc {
            0x8267FB98 => {
    //   block [0x8267FB98..0x8267FC0C)
	// 8267FB98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267FB9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267FBA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267FBA4: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267FBA8: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8267FBAC: 390AF458  addi r8, r10, -0xba8
	ctx.r[8].s64 = ctx.r[10].s64 + -2984;
	// 8267FBB0: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267FBB4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8267FBB8: 38AA7AE8  addi r5, r10, 0x7ae8
	ctx.r[5].s64 = ctx.r[10].s64 + 31464;
	// 8267FBBC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267FBC0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267FBC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267FBC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267FBCC: 388A40B8  addi r4, r10, 0x40b8
	ctx.r[4].s64 = ctx.r[10].s64 + 16568;
	// 8267FBD0: 396B47D0  addi r11, r11, 0x47d0
	ctx.r[11].s64 = ctx.r[11].s64 + 18384;
	// 8267FBD4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8267FBD8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267FBDC: 386A8298  addi r3, r10, -0x7d68
	ctx.r[3].s64 = ctx.r[10].s64 + -32104;
	// 8267FBE0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8267FBE4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267FBE8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8267FBEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267FBF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267FBF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267FBF8: 4BDE7229  bl 0x82466e20
	ctx.lr = 0x8267FBFC;
	sub_82466E20(ctx, base);
	// 8267FBFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267FC00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267FC04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267FC08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267FC10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267FC10 size=112
    let mut pc: u32 = 0x8267FC10;
    'dispatch: loop {
        match pc {
            0x8267FC10 => {
    //   block [0x8267FC10..0x8267FC80)
	// 8267FC10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267FC14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267FC18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267FC1C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267FC20: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267FC24: 38AA7A58  addi r5, r10, 0x7a58
	ctx.r[5].s64 = ctx.r[10].s64 + 31320;
	// 8267FC28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267FC2C: 390BB378  addi r8, r11, -0x4c88
	ctx.r[8].s64 = ctx.r[11].s64 + -19592;
	// 8267FC30: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8267FC34: 388A40C8  addi r4, r10, 0x40c8
	ctx.r[4].s64 = ctx.r[10].s64 + 16584;
	// 8267FC38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267FC3C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8267FC40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267FC44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267FC48: 386A82C8  addi r3, r10, -0x7d38
	ctx.r[3].s64 = ctx.r[10].s64 + -32056;
	// 8267FC4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267FC50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267FC54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267FC58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267FC5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267FC60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267FC64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267FC68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267FC6C: 4BDE71B5  bl 0x82466e20
	ctx.lr = 0x8267FC70;
	sub_82466E20(ctx, base);
	// 8267FC70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267FC74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267FC78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267FC7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267FC80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267FC80 size=112
    let mut pc: u32 = 0x8267FC80;
    'dispatch: loop {
        match pc {
            0x8267FC80 => {
    //   block [0x8267FC80..0x8267FCF0)
	// 8267FC80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267FC84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267FC88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267FC8C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267FC90: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267FC94: 38AA7AB8  addi r5, r10, 0x7ab8
	ctx.r[5].s64 = ctx.r[10].s64 + 31416;
	// 8267FC98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267FC9C: 390BB3A8  addi r8, r11, -0x4c58
	ctx.r[8].s64 = ctx.r[11].s64 + -19544;
	// 8267FCA0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8267FCA4: 388A40EC  addi r4, r10, 0x40ec
	ctx.r[4].s64 = ctx.r[10].s64 + 16620;
	// 8267FCA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267FCAC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8267FCB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267FCB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267FCB8: 386A82F8  addi r3, r10, -0x7d08
	ctx.r[3].s64 = ctx.r[10].s64 + -32008;
	// 8267FCBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267FCC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267FCC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267FCC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267FCCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267FCD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267FCD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267FCD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267FCDC: 4BDE7145  bl 0x82466e20
	ctx.lr = 0x8267FCE0;
	sub_82466E20(ctx, base);
	// 8267FCE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267FCE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267FCE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267FCEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267FCF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267FCF0 size=100
    let mut pc: u32 = 0x8267FCF0;
    'dispatch: loop {
        match pc {
            0x8267FCF0 => {
    //   block [0x8267FCF0..0x8267FD54)
	// 8267FCF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267FCF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267FCF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267FCFC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8267FD00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267FD04: 392A4840  addi r9, r10, 0x4840
	ctx.r[9].s64 = ctx.r[10].s64 + 18496;
	// 8267FD08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267FD0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267FD10: 388A4110  addi r4, r10, 0x4110
	ctx.r[4].s64 = ctx.r[10].s64 + 16656;
	// 8267FD14: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8267FD18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267FD1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267FD20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267FD24: 386A8328  addi r3, r10, -0x7cd8
	ctx.r[3].s64 = ctx.r[10].s64 + -31960;
	// 8267FD28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267FD2C: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8267FD30: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8267FD34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267FD38: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267FD3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267FD40: 4BDE70E1  bl 0x82466e20
	ctx.lr = 0x8267FD44;
	sub_82466E20(ctx, base);
	// 8267FD44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267FD48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267FD4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267FD50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267FD58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8267FD58 size=24
    let mut pc: u32 = 0x8267FD58;
    'dispatch: loop {
        match pc {
            0x8267FD58 => {
    //   block [0x8267FD58..0x8267FD70)
	// 8267FD58: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267FD5C: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267FD60: 394AF500  addi r10, r10, -0xb00
	ctx.r[10].s64 = ctx.r[10].s64 + -2816;
	// 8267FD64: 816BB3E4  lwz r11, -0x4c1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19484 as u32) ) } as u64;
	// 8267FD68: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8267FD6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267FD70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267FD70 size=112
    let mut pc: u32 = 0x8267FD70;
    'dispatch: loop {
        match pc {
            0x8267FD70 => {
    //   block [0x8267FD70..0x8267FDE0)
	// 8267FD70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267FD74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267FD78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267FD7C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8267FD80: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267FD84: 392A4980  addi r9, r10, 0x4980
	ctx.r[9].s64 = ctx.r[10].s64 + 18816;
	// 8267FD88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267FD8C: 390BF500  addi r8, r11, -0xb00
	ctx.r[8].s64 = ctx.r[11].s64 + -2816;
	// 8267FD90: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8267FD94: 388A4124  addi r4, r10, 0x4124
	ctx.r[4].s64 = ctx.r[10].s64 + 16676;
	// 8267FD98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267FD9C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8267FDA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267FDA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267FDA8: 386A8358  addi r3, r10, -0x7ca8
	ctx.r[3].s64 = ctx.r[10].s64 + -31912;
	// 8267FDAC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267FDB0: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8267FDB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267FDB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267FDBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267FDC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267FDC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267FDC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267FDCC: 4BDE7055  bl 0x82466e20
	ctx.lr = 0x8267FDD0;
	sub_82466E20(ctx, base);
	// 8267FDD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267FDD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267FDD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267FDDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


