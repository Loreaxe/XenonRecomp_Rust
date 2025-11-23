pub fn sub_832A2998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2998 size=80
    let mut pc: u32 = 0x832A2998;
    'dispatch: loop {
        match pc {
            0x832A2998 => {
    //   block [0x832A2998..0x832A29E8)
	// 832A2998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A299C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A29A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A29A4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A29A8: 3D6082E7  lis r11, -0x7d19
	ctx.r[11].s64 = -2098790400;
	// 832A29AC: 388A88F4  addi r4, r10, -0x770c
	ctx.r[4].s64 = ctx.r[10].s64 + -30476;
	// 832A29B0: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A29B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A29B8: 386AB9E8  addi r3, r10, -0x4618
	ctx.r[3].s64 = ctx.r[10].s64 + -17944;
	// 832A29BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A29C0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A29C4: 38AB2118  addi r5, r11, 0x2118
	ctx.r[5].s64 = ctx.r[11].s64 + 8472;
	// 832A29C8: 4BBE6CD9  bl 0x82e896a0
	ctx.lr = 0x832A29CC;
	sub_82E896A0(ctx, base);
	// 832A29CC: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832A29D0: 386B7F48  addi r3, r11, 0x7f48
	ctx.r[3].s64 = ctx.r[11].s64 + 32584;
	// 832A29D4: 4BA0754D  bl 0x82ca9f20
	ctx.lr = 0x832A29D8;
	sub_82CA9F20(ctx, base);
	// 832A29D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A29DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A29E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A29E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A29E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A29E8 size=80
    let mut pc: u32 = 0x832A29E8;
    'dispatch: loop {
        match pc {
            0x832A29E8 => {
    //   block [0x832A29E8..0x832A2A38)
	// 832A29E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A29EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A29F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A29F4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A29F8: 3D6082E7  lis r11, -0x7d19
	ctx.r[11].s64 = -2098790400;
	// 832A29FC: 388A8938  addi r4, r10, -0x76c8
	ctx.r[4].s64 = ctx.r[10].s64 + -30408;
	// 832A2A00: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2A04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A2A08: 386ABAFC  addi r3, r10, -0x4504
	ctx.r[3].s64 = ctx.r[10].s64 + -17668;
	// 832A2A0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A2A10: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A2A14: 38AB3410  addi r5, r11, 0x3410
	ctx.r[5].s64 = ctx.r[11].s64 + 13328;
	// 832A2A18: 4BBE6C89  bl 0x82e896a0
	ctx.lr = 0x832A2A1C;
	sub_82E896A0(ctx, base);
	// 832A2A1C: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832A2A20: 386B7F60  addi r3, r11, 0x7f60
	ctx.r[3].s64 = ctx.r[11].s64 + 32608;
	// 832A2A24: 4BA074FD  bl 0x82ca9f20
	ctx.lr = 0x832A2A28;
	sub_82CA9F20(ctx, base);
	// 832A2A28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A2A2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2A30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2A34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A2A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2A38 size=80
    let mut pc: u32 = 0x832A2A38;
    'dispatch: loop {
        match pc {
            0x832A2A38 => {
    //   block [0x832A2A38..0x832A2A88)
	// 832A2A38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A2A3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2A40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2A44: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A2A48: 3D6082E7  lis r11, -0x7d19
	ctx.r[11].s64 = -2098790400;
	// 832A2A4C: 388A894C  addi r4, r10, -0x76b4
	ctx.r[4].s64 = ctx.r[10].s64 + -30388;
	// 832A2A50: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2A54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A2A58: 386ABC10  addi r3, r10, -0x43f0
	ctx.r[3].s64 = ctx.r[10].s64 + -17392;
	// 832A2A5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A2A60: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A2A64: 38AB3928  addi r5, r11, 0x3928
	ctx.r[5].s64 = ctx.r[11].s64 + 14632;
	// 832A2A68: 4BBE6C39  bl 0x82e896a0
	ctx.lr = 0x832A2A6C;
	sub_82E896A0(ctx, base);
	// 832A2A6C: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832A2A70: 386B7F78  addi r3, r11, 0x7f78
	ctx.r[3].s64 = ctx.r[11].s64 + 32632;
	// 832A2A74: 4BA074AD  bl 0x82ca9f20
	ctx.lr = 0x832A2A78;
	sub_82CA9F20(ctx, base);
	// 832A2A78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A2A7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2A80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2A84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A2A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2A88 size=72
    let mut pc: u32 = 0x832A2A88;
    'dispatch: loop {
        match pc {
            0x832A2A88 => {
    //   block [0x832A2A88..0x832A2AD0)
	// 832A2A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A2A8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2A90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2A94: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A2A98: 3D6082E7  lis r11, -0x7d19
	ctx.r[11].s64 = -2098790400;
	// 832A2A9C: 388A89B0  addi r4, r10, -0x7650
	ctx.r[4].s64 = ctx.r[10].s64 + -30288;
	// 832A2AA0: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2AA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A2AA8: 386ABD28  addi r3, r10, -0x42d8
	ctx.r[3].s64 = ctx.r[10].s64 + -17112;
	// 832A2AAC: 38AB50F8  addi r5, r11, 0x50f8
	ctx.r[5].s64 = ctx.r[11].s64 + 20728;
	// 832A2AB0: 4BBE9CC1  bl 0x82e8c770
	ctx.lr = 0x832A2AB4;
	sub_82E8C770(ctx, base);
	// 832A2AB4: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832A2AB8: 386B7F90  addi r3, r11, 0x7f90
	ctx.r[3].s64 = ctx.r[11].s64 + 32656;
	// 832A2ABC: 4BA07465  bl 0x82ca9f20
	ctx.lr = 0x832A2AC0;
	sub_82CA9F20(ctx, base);
	// 832A2AC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A2AC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2AC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2ACC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A2AD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2AD0 size=80
    let mut pc: u32 = 0x832A2AD0;
    'dispatch: loop {
        match pc {
            0x832A2AD0 => {
    //   block [0x832A2AD0..0x832A2B20)
	// 832A2AD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A2AD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2AD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2ADC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A2AE0: 3D6082E7  lis r11, -0x7d19
	ctx.r[11].s64 = -2098790400;
	// 832A2AE4: 388A8AF4  addi r4, r10, -0x750c
	ctx.r[4].s64 = ctx.r[10].s64 + -29964;
	// 832A2AE8: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2AEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A2AF0: 386ABE3C  addi r3, r10, -0x41c4
	ctx.r[3].s64 = ctx.r[10].s64 + -16836;
	// 832A2AF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A2AF8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A2AFC: 38AB57C0  addi r5, r11, 0x57c0
	ctx.r[5].s64 = ctx.r[11].s64 + 22464;
	// 832A2B00: 4BBE6BA1  bl 0x82e896a0
	ctx.lr = 0x832A2B04;
	sub_82E896A0(ctx, base);
	// 832A2B04: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832A2B08: 386B7FA8  addi r3, r11, 0x7fa8
	ctx.r[3].s64 = ctx.r[11].s64 + 32680;
	// 832A2B0C: 4BA07415  bl 0x82ca9f20
	ctx.lr = 0x832A2B10;
	sub_82CA9F20(ctx, base);
	// 832A2B10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A2B14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2B18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2B1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A2B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2B20 size=80
    let mut pc: u32 = 0x832A2B20;
    'dispatch: loop {
        match pc {
            0x832A2B20 => {
    //   block [0x832A2B20..0x832A2B70)
	// 832A2B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A2B24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2B28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2B2C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A2B30: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 832A2B34: 388A8B90  addi r4, r10, -0x7470
	ctx.r[4].s64 = ctx.r[10].s64 + -29808;
	// 832A2B38: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2B3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A2B40: 386ABF50  addi r3, r10, -0x40b0
	ctx.r[3].s64 = ctx.r[10].s64 + -16560;
	// 832A2B44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A2B48: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A2B4C: 38AB8708  addi r5, r11, -0x78f8
	ctx.r[5].s64 = ctx.r[11].s64 + -30968;
	// 832A2B50: 4BBE6B51  bl 0x82e896a0
	ctx.lr = 0x832A2B54;
	sub_82E896A0(ctx, base);
	// 832A2B54: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832A2B58: 386B7FC0  addi r3, r11, 0x7fc0
	ctx.r[3].s64 = ctx.r[11].s64 + 32704;
	// 832A2B5C: 4BA073C5  bl 0x82ca9f20
	ctx.lr = 0x832A2B60;
	sub_82CA9F20(ctx, base);
	// 832A2B60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A2B64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2B68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2B6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A2B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2B70 size=72
    let mut pc: u32 = 0x832A2B70;
    'dispatch: loop {
        match pc {
            0x832A2B70 => {
    //   block [0x832A2B70..0x832A2BB8)
	// 832A2B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A2B74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2B78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2B7C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A2B80: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 832A2B84: 388A8BB8  addi r4, r10, -0x7448
	ctx.r[4].s64 = ctx.r[10].s64 + -29768;
	// 832A2B88: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2B8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A2B90: 386AC064  addi r3, r10, -0x3f9c
	ctx.r[3].s64 = ctx.r[10].s64 + -16284;
	// 832A2B94: 38AB8EA0  addi r5, r11, -0x7160
	ctx.r[5].s64 = ctx.r[11].s64 + -29024;
	// 832A2B98: 4BBDCA11  bl 0x82e7f5a8
	ctx.lr = 0x832A2B9C;
	sub_82E7F5A8(ctx, base);
	// 832A2B9C: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832A2BA0: 386B7FD8  addi r3, r11, 0x7fd8
	ctx.r[3].s64 = ctx.r[11].s64 + 32728;
	// 832A2BA4: 4BA0737D  bl 0x82ca9f20
	ctx.lr = 0x832A2BA8;
	sub_82CA9F20(ctx, base);
	// 832A2BA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A2BAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2BB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2BB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A2BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2BB8 size=72
    let mut pc: u32 = 0x832A2BB8;
    'dispatch: loop {
        match pc {
            0x832A2BB8 => {
    //   block [0x832A2BB8..0x832A2C00)
	// 832A2BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A2BBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2BC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2BC4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A2BC8: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 832A2BCC: 388A8BE0  addi r4, r10, -0x7420
	ctx.r[4].s64 = ctx.r[10].s64 + -29728;
	// 832A2BD0: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2BD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A2BD8: 386AC178  addi r3, r10, -0x3e88
	ctx.r[3].s64 = ctx.r[10].s64 + -16008;
	// 832A2BDC: 38AB8F08  addi r5, r11, -0x70f8
	ctx.r[5].s64 = ctx.r[11].s64 + -28920;
	// 832A2BE0: 4BBDC9C9  bl 0x82e7f5a8
	ctx.lr = 0x832A2BE4;
	sub_82E7F5A8(ctx, base);
	// 832A2BE4: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832A2BE8: 386B7FF0  addi r3, r11, 0x7ff0
	ctx.r[3].s64 = ctx.r[11].s64 + 32752;
	// 832A2BEC: 4BA07335  bl 0x82ca9f20
	ctx.lr = 0x832A2BF0;
	sub_82CA9F20(ctx, base);
	// 832A2BF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A2BF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2BF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2BFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A2C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2C00 size=72
    let mut pc: u32 = 0x832A2C00;
    'dispatch: loop {
        match pc {
            0x832A2C00 => {
    //   block [0x832A2C00..0x832A2C48)
	// 832A2C00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A2C04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2C08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2C0C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A2C10: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 832A2C14: 388A8C04  addi r4, r10, -0x73fc
	ctx.r[4].s64 = ctx.r[10].s64 + -29692;
	// 832A2C18: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2C1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A2C20: 386AC28C  addi r3, r10, -0x3d74
	ctx.r[3].s64 = ctx.r[10].s64 + -15732;
	// 832A2C24: 38AB8FE8  addi r5, r11, -0x7018
	ctx.r[5].s64 = ctx.r[11].s64 + -28696;
	// 832A2C28: 4BBDC981  bl 0x82e7f5a8
	ctx.lr = 0x832A2C2C;
	sub_82E7F5A8(ctx, base);
	// 832A2C2C: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A2C30: 386B8008  addi r3, r11, -0x7ff8
	ctx.r[3].s64 = ctx.r[11].s64 + -32760;
	// 832A2C34: 4BA072ED  bl 0x82ca9f20
	ctx.lr = 0x832A2C38;
	sub_82CA9F20(ctx, base);
	// 832A2C38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A2C3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2C40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2C44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A2C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2C48 size=72
    let mut pc: u32 = 0x832A2C48;
    'dispatch: loop {
        match pc {
            0x832A2C48 => {
    //   block [0x832A2C48..0x832A2C90)
	// 832A2C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A2C4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2C50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2C54: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A2C58: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 832A2C5C: 388A8C28  addi r4, r10, -0x73d8
	ctx.r[4].s64 = ctx.r[10].s64 + -29656;
	// 832A2C60: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2C64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A2C68: 386AC3A0  addi r3, r10, -0x3c60
	ctx.r[3].s64 = ctx.r[10].s64 + -15456;
	// 832A2C6C: 38AB9068  addi r5, r11, -0x6f98
	ctx.r[5].s64 = ctx.r[11].s64 + -28568;
	// 832A2C70: 4BBDC939  bl 0x82e7f5a8
	ctx.lr = 0x832A2C74;
	sub_82E7F5A8(ctx, base);
	// 832A2C74: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A2C78: 386B8020  addi r3, r11, -0x7fe0
	ctx.r[3].s64 = ctx.r[11].s64 + -32736;
	// 832A2C7C: 4BA072A5  bl 0x82ca9f20
	ctx.lr = 0x832A2C80;
	sub_82CA9F20(ctx, base);
	// 832A2C80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A2C84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2C88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2C8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A2C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2C90 size=72
    let mut pc: u32 = 0x832A2C90;
    'dispatch: loop {
        match pc {
            0x832A2C90 => {
    //   block [0x832A2C90..0x832A2CD8)
	// 832A2C90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A2C94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2C98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2C9C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A2CA0: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 832A2CA4: 388A8C4C  addi r4, r10, -0x73b4
	ctx.r[4].s64 = ctx.r[10].s64 + -29620;
	// 832A2CA8: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2CAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A2CB0: 386AC4B4  addi r3, r10, -0x3b4c
	ctx.r[3].s64 = ctx.r[10].s64 + -15180;
	// 832A2CB4: 38AB9170  addi r5, r11, -0x6e90
	ctx.r[5].s64 = ctx.r[11].s64 + -28304;
	// 832A2CB8: 4BBDC8F1  bl 0x82e7f5a8
	ctx.lr = 0x832A2CBC;
	sub_82E7F5A8(ctx, base);
	// 832A2CBC: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A2CC0: 386B8038  addi r3, r11, -0x7fc8
	ctx.r[3].s64 = ctx.r[11].s64 + -32712;
	// 832A2CC4: 4BA0725D  bl 0x82ca9f20
	ctx.lr = 0x832A2CC8;
	sub_82CA9F20(ctx, base);
	// 832A2CC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A2CCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2CD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2CD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A2CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2CD8 size=72
    let mut pc: u32 = 0x832A2CD8;
    'dispatch: loop {
        match pc {
            0x832A2CD8 => {
    //   block [0x832A2CD8..0x832A2D20)
	// 832A2CD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A2CDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2CE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2CE4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A2CE8: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 832A2CEC: 388A8C78  addi r4, r10, -0x7388
	ctx.r[4].s64 = ctx.r[10].s64 + -29576;
	// 832A2CF0: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2CF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A2CF8: 386AC5C8  addi r3, r10, -0x3a38
	ctx.r[3].s64 = ctx.r[10].s64 + -14904;
	// 832A2CFC: 38AB9260  addi r5, r11, -0x6da0
	ctx.r[5].s64 = ctx.r[11].s64 + -28064;
	// 832A2D00: 4BBDC8A9  bl 0x82e7f5a8
	ctx.lr = 0x832A2D04;
	sub_82E7F5A8(ctx, base);
	// 832A2D04: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A2D08: 386B8050  addi r3, r11, -0x7fb0
	ctx.r[3].s64 = ctx.r[11].s64 + -32688;
	// 832A2D0C: 4BA07215  bl 0x82ca9f20
	ctx.lr = 0x832A2D10;
	sub_82CA9F20(ctx, base);
	// 832A2D10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A2D14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2D18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2D1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A2D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2D20 size=72
    let mut pc: u32 = 0x832A2D20;
    'dispatch: loop {
        match pc {
            0x832A2D20 => {
    //   block [0x832A2D20..0x832A2D68)
	// 832A2D20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A2D24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2D28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2D2C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A2D30: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 832A2D34: 388A8C8C  addi r4, r10, -0x7374
	ctx.r[4].s64 = ctx.r[10].s64 + -29556;
	// 832A2D38: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2D3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A2D40: 386AC6DC  addi r3, r10, -0x3924
	ctx.r[3].s64 = ctx.r[10].s64 + -14628;
	// 832A2D44: 38AB92E0  addi r5, r11, -0x6d20
	ctx.r[5].s64 = ctx.r[11].s64 + -27936;
	// 832A2D48: 4BBDAFC1  bl 0x82e7dd08
	ctx.lr = 0x832A2D4C;
	sub_82E7DD08(ctx, base);
	// 832A2D4C: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A2D50: 386B8068  addi r3, r11, -0x7f98
	ctx.r[3].s64 = ctx.r[11].s64 + -32664;
	// 832A2D54: 4BA071CD  bl 0x82ca9f20
	ctx.lr = 0x832A2D58;
	sub_82CA9F20(ctx, base);
	// 832A2D58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A2D5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2D60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2D64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A2D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2D68 size=72
    let mut pc: u32 = 0x832A2D68;
    'dispatch: loop {
        match pc {
            0x832A2D68 => {
    //   block [0x832A2D68..0x832A2DB0)
	// 832A2D68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A2D6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2D70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2D74: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A2D78: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 832A2D7C: 388A8CA0  addi r4, r10, -0x7360
	ctx.r[4].s64 = ctx.r[10].s64 + -29536;
	// 832A2D80: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2D84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A2D88: 386AC7F0  addi r3, r10, -0x3810
	ctx.r[3].s64 = ctx.r[10].s64 + -14352;
	// 832A2D8C: 38AB9328  addi r5, r11, -0x6cd8
	ctx.r[5].s64 = ctx.r[11].s64 + -27864;
	// 832A2D90: 4BBDAF79  bl 0x82e7dd08
	ctx.lr = 0x832A2D94;
	sub_82E7DD08(ctx, base);
	// 832A2D94: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A2D98: 386B8080  addi r3, r11, -0x7f80
	ctx.r[3].s64 = ctx.r[11].s64 + -32640;
	// 832A2D9C: 4BA07185  bl 0x82ca9f20
	ctx.lr = 0x832A2DA0;
	sub_82CA9F20(ctx, base);
	// 832A2DA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A2DA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2DA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2DAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A2DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2DB0 size=72
    let mut pc: u32 = 0x832A2DB0;
    'dispatch: loop {
        match pc {
            0x832A2DB0 => {
    //   block [0x832A2DB0..0x832A2DF8)
	// 832A2DB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A2DB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2DB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2DBC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A2DC0: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 832A2DC4: 388A8CC8  addi r4, r10, -0x7338
	ctx.r[4].s64 = ctx.r[10].s64 + -29496;
	// 832A2DC8: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2DCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A2DD0: 386AC904  addi r3, r10, -0x36fc
	ctx.r[3].s64 = ctx.r[10].s64 + -14076;
	// 832A2DD4: 38AB93D0  addi r5, r11, -0x6c30
	ctx.r[5].s64 = ctx.r[11].s64 + -27696;
	// 832A2DD8: 4BBDAF31  bl 0x82e7dd08
	ctx.lr = 0x832A2DDC;
	sub_82E7DD08(ctx, base);
	// 832A2DDC: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A2DE0: 386B8098  addi r3, r11, -0x7f68
	ctx.r[3].s64 = ctx.r[11].s64 + -32616;
	// 832A2DE4: 4BA0713D  bl 0x82ca9f20
	ctx.lr = 0x832A2DE8;
	sub_82CA9F20(ctx, base);
	// 832A2DE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A2DEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2DF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2DF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A2DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2DF8 size=72
    let mut pc: u32 = 0x832A2DF8;
    'dispatch: loop {
        match pc {
            0x832A2DF8 => {
    //   block [0x832A2DF8..0x832A2E40)
	// 832A2DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A2DFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2E00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2E04: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A2E08: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 832A2E0C: 388A8CF4  addi r4, r10, -0x730c
	ctx.r[4].s64 = ctx.r[10].s64 + -29452;
	// 832A2E10: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2E14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A2E18: 386ACA18  addi r3, r10, -0x35e8
	ctx.r[3].s64 = ctx.r[10].s64 + -13800;
	// 832A2E1C: 38AB9428  addi r5, r11, -0x6bd8
	ctx.r[5].s64 = ctx.r[11].s64 + -27608;
	// 832A2E20: 4BBDAEE9  bl 0x82e7dd08
	ctx.lr = 0x832A2E24;
	sub_82E7DD08(ctx, base);
	// 832A2E24: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A2E28: 386B80B0  addi r3, r11, -0x7f50
	ctx.r[3].s64 = ctx.r[11].s64 + -32592;
	// 832A2E2C: 4BA070F5  bl 0x82ca9f20
	ctx.lr = 0x832A2E30;
	sub_82CA9F20(ctx, base);
	// 832A2E30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A2E34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2E38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2E3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A2E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2E40 size=72
    let mut pc: u32 = 0x832A2E40;
    'dispatch: loop {
        match pc {
            0x832A2E40 => {
    //   block [0x832A2E40..0x832A2E88)
	// 832A2E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A2E44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2E48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2E4C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A2E50: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 832A2E54: 388A8D1C  addi r4, r10, -0x72e4
	ctx.r[4].s64 = ctx.r[10].s64 + -29412;
	// 832A2E58: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2E5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A2E60: 386ACB2C  addi r3, r10, -0x34d4
	ctx.r[3].s64 = ctx.r[10].s64 + -13524;
	// 832A2E64: 38AB9538  addi r5, r11, -0x6ac8
	ctx.r[5].s64 = ctx.r[11].s64 + -27336;
	// 832A2E68: 4BBDAEA1  bl 0x82e7dd08
	ctx.lr = 0x832A2E6C;
	sub_82E7DD08(ctx, base);
	// 832A2E6C: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A2E70: 386B80C8  addi r3, r11, -0x7f38
	ctx.r[3].s64 = ctx.r[11].s64 + -32568;
	// 832A2E74: 4BA070AD  bl 0x82ca9f20
	ctx.lr = 0x832A2E78;
	sub_82CA9F20(ctx, base);
	// 832A2E78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A2E7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2E80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2E84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A2E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2E88 size=72
    let mut pc: u32 = 0x832A2E88;
    'dispatch: loop {
        match pc {
            0x832A2E88 => {
    //   block [0x832A2E88..0x832A2ED0)
	// 832A2E88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A2E8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2E90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2E94: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A2E98: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 832A2E9C: 388A8D40  addi r4, r10, -0x72c0
	ctx.r[4].s64 = ctx.r[10].s64 + -29376;
	// 832A2EA0: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2EA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A2EA8: 386ACC40  addi r3, r10, -0x33c0
	ctx.r[3].s64 = ctx.r[10].s64 + -13248;
	// 832A2EAC: 38AB9628  addi r5, r11, -0x69d8
	ctx.r[5].s64 = ctx.r[11].s64 + -27096;
	// 832A2EB0: 4BBDC6F9  bl 0x82e7f5a8
	ctx.lr = 0x832A2EB4;
	sub_82E7F5A8(ctx, base);
	// 832A2EB4: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A2EB8: 386B80E0  addi r3, r11, -0x7f20
	ctx.r[3].s64 = ctx.r[11].s64 + -32544;
	// 832A2EBC: 4BA07065  bl 0x82ca9f20
	ctx.lr = 0x832A2EC0;
	sub_82CA9F20(ctx, base);
	// 832A2EC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A2EC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2EC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2ECC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A2ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2ED0 size=72
    let mut pc: u32 = 0x832A2ED0;
    'dispatch: loop {
        match pc {
            0x832A2ED0 => {
    //   block [0x832A2ED0..0x832A2F18)
	// 832A2ED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A2ED4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2ED8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2EDC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A2EE0: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 832A2EE4: 388A8D6C  addi r4, r10, -0x7294
	ctx.r[4].s64 = ctx.r[10].s64 + -29332;
	// 832A2EE8: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2EEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A2EF0: 386ACD54  addi r3, r10, -0x32ac
	ctx.r[3].s64 = ctx.r[10].s64 + -12972;
	// 832A2EF4: 38AB9690  addi r5, r11, -0x6970
	ctx.r[5].s64 = ctx.r[11].s64 + -26992;
	// 832A2EF8: 4BBDAE11  bl 0x82e7dd08
	ctx.lr = 0x832A2EFC;
	sub_82E7DD08(ctx, base);
	// 832A2EFC: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A2F00: 386B80F8  addi r3, r11, -0x7f08
	ctx.r[3].s64 = ctx.r[11].s64 + -32520;
	// 832A2F04: 4BA0701D  bl 0x82ca9f20
	ctx.lr = 0x832A2F08;
	sub_82CA9F20(ctx, base);
	// 832A2F08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A2F0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2F10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2F14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A2F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2F18 size=72
    let mut pc: u32 = 0x832A2F18;
    'dispatch: loop {
        match pc {
            0x832A2F18 => {
    //   block [0x832A2F18..0x832A2F60)
	// 832A2F18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A2F1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2F20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2F24: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A2F28: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 832A2F2C: 388A8D90  addi r4, r10, -0x7270
	ctx.r[4].s64 = ctx.r[10].s64 + -29296;
	// 832A2F30: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2F34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A2F38: 386ACE68  addi r3, r10, -0x3198
	ctx.r[3].s64 = ctx.r[10].s64 + -12696;
	// 832A2F3C: 38AB9750  addi r5, r11, -0x68b0
	ctx.r[5].s64 = ctx.r[11].s64 + -26800;
	// 832A2F40: 4BBDADC9  bl 0x82e7dd08
	ctx.lr = 0x832A2F44;
	sub_82E7DD08(ctx, base);
	// 832A2F44: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A2F48: 386B8110  addi r3, r11, -0x7ef0
	ctx.r[3].s64 = ctx.r[11].s64 + -32496;
	// 832A2F4C: 4BA06FD5  bl 0x82ca9f20
	ctx.lr = 0x832A2F50;
	sub_82CA9F20(ctx, base);
	// 832A2F50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A2F54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2F58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2F5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A2F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2F60 size=72
    let mut pc: u32 = 0x832A2F60;
    'dispatch: loop {
        match pc {
            0x832A2F60 => {
    //   block [0x832A2F60..0x832A2FA8)
	// 832A2F60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A2F64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2F68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2F6C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A2F70: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 832A2F74: 388A8DB8  addi r4, r10, -0x7248
	ctx.r[4].s64 = ctx.r[10].s64 + -29256;
	// 832A2F78: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2F7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A2F80: 386ACF7C  addi r3, r10, -0x3084
	ctx.r[3].s64 = ctx.r[10].s64 + -12420;
	// 832A2F84: 38AB97A8  addi r5, r11, -0x6858
	ctx.r[5].s64 = ctx.r[11].s64 + -26712;
	// 832A2F88: 4BBDAD81  bl 0x82e7dd08
	ctx.lr = 0x832A2F8C;
	sub_82E7DD08(ctx, base);
	// 832A2F8C: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A2F90: 386B8128  addi r3, r11, -0x7ed8
	ctx.r[3].s64 = ctx.r[11].s64 + -32472;
	// 832A2F94: 4BA06F8D  bl 0x82ca9f20
	ctx.lr = 0x832A2F98;
	sub_82CA9F20(ctx, base);
	// 832A2F98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A2F9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2FA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2FA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A2FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2FA8 size=72
    let mut pc: u32 = 0x832A2FA8;
    'dispatch: loop {
        match pc {
            0x832A2FA8 => {
    //   block [0x832A2FA8..0x832A2FF0)
	// 832A2FA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A2FAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2FB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2FB4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A2FB8: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 832A2FBC: 388A8DEC  addi r4, r10, -0x7214
	ctx.r[4].s64 = ctx.r[10].s64 + -29204;
	// 832A2FC0: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A2FC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A2FC8: 386AD098  addi r3, r10, -0x2f68
	ctx.r[3].s64 = ctx.r[10].s64 + -12136;
	// 832A2FCC: 38AB9D08  addi r5, r11, -0x62f8
	ctx.r[5].s64 = ctx.r[11].s64 + -25336;
	// 832A2FD0: 4BBDC5D9  bl 0x82e7f5a8
	ctx.lr = 0x832A2FD4;
	sub_82E7F5A8(ctx, base);
	// 832A2FD4: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A2FD8: 386B8140  addi r3, r11, -0x7ec0
	ctx.r[3].s64 = ctx.r[11].s64 + -32448;
	// 832A2FDC: 4BA06F45  bl 0x82ca9f20
	ctx.lr = 0x832A2FE0;
	sub_82CA9F20(ctx, base);
	// 832A2FE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A2FE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A2FE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A2FEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A2FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A2FF0 size=72
    let mut pc: u32 = 0x832A2FF0;
    'dispatch: loop {
        match pc {
            0x832A2FF0 => {
    //   block [0x832A2FF0..0x832A3038)
	// 832A2FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A2FF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A2FF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A2FFC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A3000: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 832A3004: 388A8E10  addi r4, r10, -0x71f0
	ctx.r[4].s64 = ctx.r[10].s64 + -29168;
	// 832A3008: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A300C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A3010: 386AD1AC  addi r3, r10, -0x2e54
	ctx.r[3].s64 = ctx.r[10].s64 + -11860;
	// 832A3014: 38AB9ED0  addi r5, r11, -0x6130
	ctx.r[5].s64 = ctx.r[11].s64 + -24880;
	// 832A3018: 4BBDC591  bl 0x82e7f5a8
	ctx.lr = 0x832A301C;
	sub_82E7F5A8(ctx, base);
	// 832A301C: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A3020: 386B8158  addi r3, r11, -0x7ea8
	ctx.r[3].s64 = ctx.r[11].s64 + -32424;
	// 832A3024: 4BA06EFD  bl 0x82ca9f20
	ctx.lr = 0x832A3028;
	sub_82CA9F20(ctx, base);
	// 832A3028: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A302C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3030: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3034: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A3038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3038 size=72
    let mut pc: u32 = 0x832A3038;
    'dispatch: loop {
        match pc {
            0x832A3038 => {
    //   block [0x832A3038..0x832A3080)
	// 832A3038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A303C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3040: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A3044: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A3048: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 832A304C: 388A8E34  addi r4, r10, -0x71cc
	ctx.r[4].s64 = ctx.r[10].s64 + -29132;
	// 832A3050: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A3054: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A3058: 386AD2C0  addi r3, r10, -0x2d40
	ctx.r[3].s64 = ctx.r[10].s64 + -11584;
	// 832A305C: 38AB9F88  addi r5, r11, -0x6078
	ctx.r[5].s64 = ctx.r[11].s64 + -24696;
	// 832A3060: 4BBDC549  bl 0x82e7f5a8
	ctx.lr = 0x832A3064;
	sub_82E7F5A8(ctx, base);
	// 832A3064: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A3068: 386B8170  addi r3, r11, -0x7e90
	ctx.r[3].s64 = ctx.r[11].s64 + -32400;
	// 832A306C: 4BA06EB5  bl 0x82ca9f20
	ctx.lr = 0x832A3070;
	sub_82CA9F20(ctx, base);
	// 832A3070: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A3074: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3078: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A307C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A3080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3080 size=72
    let mut pc: u32 = 0x832A3080;
    'dispatch: loop {
        match pc {
            0x832A3080 => {
    //   block [0x832A3080..0x832A30C8)
	// 832A3080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A3084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3088: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A308C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A3090: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 832A3094: 388A8E60  addi r4, r10, -0x71a0
	ctx.r[4].s64 = ctx.r[10].s64 + -29088;
	// 832A3098: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A309C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A30A0: 386AD3D4  addi r3, r10, -0x2c2c
	ctx.r[3].s64 = ctx.r[10].s64 + -11308;
	// 832A30A4: 38AB9FF0  addi r5, r11, -0x6010
	ctx.r[5].s64 = ctx.r[11].s64 + -24592;
	// 832A30A8: 4BBDC501  bl 0x82e7f5a8
	ctx.lr = 0x832A30AC;
	sub_82E7F5A8(ctx, base);
	// 832A30AC: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A30B0: 386B8188  addi r3, r11, -0x7e78
	ctx.r[3].s64 = ctx.r[11].s64 + -32376;
	// 832A30B4: 4BA06E6D  bl 0x82ca9f20
	ctx.lr = 0x832A30B8;
	sub_82CA9F20(ctx, base);
	// 832A30B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A30BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A30C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A30C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A30C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A30C8 size=72
    let mut pc: u32 = 0x832A30C8;
    'dispatch: loop {
        match pc {
            0x832A30C8 => {
    //   block [0x832A30C8..0x832A3110)
	// 832A30C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A30CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A30D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A30D4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A30D8: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 832A30DC: 388A8E88  addi r4, r10, -0x7178
	ctx.r[4].s64 = ctx.r[10].s64 + -29048;
	// 832A30E0: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A30E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A30E8: 386AD4E8  addi r3, r10, -0x2b18
	ctx.r[3].s64 = ctx.r[10].s64 + -11032;
	// 832A30EC: 38ABA058  addi r5, r11, -0x5fa8
	ctx.r[5].s64 = ctx.r[11].s64 + -24488;
	// 832A30F0: 4BBDC4B9  bl 0x82e7f5a8
	ctx.lr = 0x832A30F4;
	sub_82E7F5A8(ctx, base);
	// 832A30F4: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A30F8: 386B81A0  addi r3, r11, -0x7e60
	ctx.r[3].s64 = ctx.r[11].s64 + -32352;
	// 832A30FC: 4BA06E25  bl 0x82ca9f20
	ctx.lr = 0x832A3100;
	sub_82CA9F20(ctx, base);
	// 832A3100: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A3104: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3108: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A310C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A3110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3110 size=72
    let mut pc: u32 = 0x832A3110;
    'dispatch: loop {
        match pc {
            0x832A3110 => {
    //   block [0x832A3110..0x832A3158)
	// 832A3110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A3114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3118: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A311C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A3120: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 832A3124: 388A8EB4  addi r4, r10, -0x714c
	ctx.r[4].s64 = ctx.r[10].s64 + -29004;
	// 832A3128: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A312C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A3130: 386AD5FC  addi r3, r10, -0x2a04
	ctx.r[3].s64 = ctx.r[10].s64 + -10756;
	// 832A3134: 38ABA128  addi r5, r11, -0x5ed8
	ctx.r[5].s64 = ctx.r[11].s64 + -24280;
	// 832A3138: 4BBDC471  bl 0x82e7f5a8
	ctx.lr = 0x832A313C;
	sub_82E7F5A8(ctx, base);
	// 832A313C: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A3140: 386B81B8  addi r3, r11, -0x7e48
	ctx.r[3].s64 = ctx.r[11].s64 + -32328;
	// 832A3144: 4BA06DDD  bl 0x82ca9f20
	ctx.lr = 0x832A3148;
	sub_82CA9F20(ctx, base);
	// 832A3148: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A314C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3150: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3154: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A3158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3158 size=72
    let mut pc: u32 = 0x832A3158;
    'dispatch: loop {
        match pc {
            0x832A3158 => {
    //   block [0x832A3158..0x832A31A0)
	// 832A3158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A315C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3160: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A3164: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A3168: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 832A316C: 388A8EE0  addi r4, r10, -0x7120
	ctx.r[4].s64 = ctx.r[10].s64 + -28960;
	// 832A3170: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A3174: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A3178: 386AD710  addi r3, r10, -0x28f0
	ctx.r[3].s64 = ctx.r[10].s64 + -10480;
	// 832A317C: 38ABA200  addi r5, r11, -0x5e00
	ctx.r[5].s64 = ctx.r[11].s64 + -24064;
	// 832A3180: 4BBDAB89  bl 0x82e7dd08
	ctx.lr = 0x832A3184;
	sub_82E7DD08(ctx, base);
	// 832A3184: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A3188: 386B81D0  addi r3, r11, -0x7e30
	ctx.r[3].s64 = ctx.r[11].s64 + -32304;
	// 832A318C: 4BA06D95  bl 0x82ca9f20
	ctx.lr = 0x832A3190;
	sub_82CA9F20(ctx, base);
	// 832A3190: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A3194: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3198: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A319C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A31A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A31A0 size=72
    let mut pc: u32 = 0x832A31A0;
    'dispatch: loop {
        match pc {
            0x832A31A0 => {
    //   block [0x832A31A0..0x832A31E8)
	// 832A31A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A31A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A31A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A31AC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A31B0: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 832A31B4: 388A8F10  addi r4, r10, -0x70f0
	ctx.r[4].s64 = ctx.r[10].s64 + -28912;
	// 832A31B8: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A31BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A31C0: 386AD824  addi r3, r10, -0x27dc
	ctx.r[3].s64 = ctx.r[10].s64 + -10204;
	// 832A31C4: 38ABA258  addi r5, r11, -0x5da8
	ctx.r[5].s64 = ctx.r[11].s64 + -23976;
	// 832A31C8: 4BBDAB41  bl 0x82e7dd08
	ctx.lr = 0x832A31CC;
	sub_82E7DD08(ctx, base);
	// 832A31CC: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A31D0: 386B81E8  addi r3, r11, -0x7e18
	ctx.r[3].s64 = ctx.r[11].s64 + -32280;
	// 832A31D4: 4BA06D4D  bl 0x82ca9f20
	ctx.lr = 0x832A31D8;
	sub_82CA9F20(ctx, base);
	// 832A31D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A31DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A31E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A31E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A31E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A31E8 size=72
    let mut pc: u32 = 0x832A31E8;
    'dispatch: loop {
        match pc {
            0x832A31E8 => {
    //   block [0x832A31E8..0x832A3230)
	// 832A31E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A31EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A31F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A31F4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A31F8: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 832A31FC: 388A8F3C  addi r4, r10, -0x70c4
	ctx.r[4].s64 = ctx.r[10].s64 + -28868;
	// 832A3200: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A3204: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A3208: 386AD938  addi r3, r10, -0x26c8
	ctx.r[3].s64 = ctx.r[10].s64 + -9928;
	// 832A320C: 38ABA320  addi r5, r11, -0x5ce0
	ctx.r[5].s64 = ctx.r[11].s64 + -23776;
	// 832A3210: 4BBDAAF9  bl 0x82e7dd08
	ctx.lr = 0x832A3214;
	sub_82E7DD08(ctx, base);
	// 832A3214: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A3218: 386B8200  addi r3, r11, -0x7e00
	ctx.r[3].s64 = ctx.r[11].s64 + -32256;
	// 832A321C: 4BA06D05  bl 0x82ca9f20
	ctx.lr = 0x832A3220;
	sub_82CA9F20(ctx, base);
	// 832A3220: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A3224: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3228: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A322C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A3230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3230 size=72
    let mut pc: u32 = 0x832A3230;
    'dispatch: loop {
        match pc {
            0x832A3230 => {
    //   block [0x832A3230..0x832A3278)
	// 832A3230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A3234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3238: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A323C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A3240: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 832A3244: 388A8F60  addi r4, r10, -0x70a0
	ctx.r[4].s64 = ctx.r[10].s64 + -28832;
	// 832A3248: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A324C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A3250: 386ADA4C  addi r3, r10, -0x25b4
	ctx.r[3].s64 = ctx.r[10].s64 + -9652;
	// 832A3254: 38ABA378  addi r5, r11, -0x5c88
	ctx.r[5].s64 = ctx.r[11].s64 + -23688;
	// 832A3258: 4BBDC351  bl 0x82e7f5a8
	ctx.lr = 0x832A325C;
	sub_82E7F5A8(ctx, base);
	// 832A325C: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A3260: 386B8218  addi r3, r11, -0x7de8
	ctx.r[3].s64 = ctx.r[11].s64 + -32232;
	// 832A3264: 4BA06CBD  bl 0x82ca9f20
	ctx.lr = 0x832A3268;
	sub_82CA9F20(ctx, base);
	// 832A3268: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A326C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3270: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3274: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A3278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3278 size=80
    let mut pc: u32 = 0x832A3278;
    'dispatch: loop {
        match pc {
            0x832A3278 => {
    //   block [0x832A3278..0x832A32C8)
	// 832A3278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A327C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3280: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A3284: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A3288: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 832A328C: 388A9014  addi r4, r10, -0x6fec
	ctx.r[4].s64 = ctx.r[10].s64 + -28652;
	// 832A3290: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A3294: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A3298: 386ADF68  addi r3, r10, -0x2098
	ctx.r[3].s64 = ctx.r[10].s64 + -8344;
	// 832A329C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A32A0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A32A4: 38ABB3F8  addi r5, r11, -0x4c08
	ctx.r[5].s64 = ctx.r[11].s64 + -19464;
	// 832A32A8: 4BBE2519  bl 0x82e857c0
	ctx.lr = 0x832A32AC;
	sub_82E857C0(ctx, base);
	// 832A32AC: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A32B0: 386B8230  addi r3, r11, -0x7dd0
	ctx.r[3].s64 = ctx.r[11].s64 + -32208;
	// 832A32B4: 4BA06C6D  bl 0x82ca9f20
	ctx.lr = 0x832A32B8;
	sub_82CA9F20(ctx, base);
	// 832A32B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A32BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A32C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A32C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A32C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A32C8 size=80
    let mut pc: u32 = 0x832A32C8;
    'dispatch: loop {
        match pc {
            0x832A32C8 => {
    //   block [0x832A32C8..0x832A3318)
	// 832A32C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A32CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A32D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A32D4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A32D8: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 832A32DC: 388A9048  addi r4, r10, -0x6fb8
	ctx.r[4].s64 = ctx.r[10].s64 + -28600;
	// 832A32E0: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A32E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A32E8: 386AE07C  addi r3, r10, -0x1f84
	ctx.r[3].s64 = ctx.r[10].s64 + -8068;
	// 832A32EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A32F0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A32F4: 38ABC190  addi r5, r11, -0x3e70
	ctx.r[5].s64 = ctx.r[11].s64 + -15984;
	// 832A32F8: 4BBE24C9  bl 0x82e857c0
	ctx.lr = 0x832A32FC;
	sub_82E857C0(ctx, base);
	// 832A32FC: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A3300: 386B8260  addi r3, r11, -0x7da0
	ctx.r[3].s64 = ctx.r[11].s64 + -32160;
	// 832A3304: 4BA06C1D  bl 0x82ca9f20
	ctx.lr = 0x832A3308;
	sub_82CA9F20(ctx, base);
	// 832A3308: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A330C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3310: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3314: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A3318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3318 size=80
    let mut pc: u32 = 0x832A3318;
    'dispatch: loop {
        match pc {
            0x832A3318 => {
    //   block [0x832A3318..0x832A3368)
	// 832A3318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A331C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3320: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A3324: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A3328: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 832A332C: 388A9080  addi r4, r10, -0x6f80
	ctx.r[4].s64 = ctx.r[10].s64 + -28544;
	// 832A3330: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A3334: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A3338: 386AE190  addi r3, r10, -0x1e70
	ctx.r[3].s64 = ctx.r[10].s64 + -7792;
	// 832A333C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A3340: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A3344: 38ABC6E0  addi r5, r11, -0x3920
	ctx.r[5].s64 = ctx.r[11].s64 + -14624;
	// 832A3348: 4BBE2479  bl 0x82e857c0
	ctx.lr = 0x832A334C;
	sub_82E857C0(ctx, base);
	// 832A334C: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A3350: 386B8278  addi r3, r11, -0x7d88
	ctx.r[3].s64 = ctx.r[11].s64 + -32136;
	// 832A3354: 4BA06BCD  bl 0x82ca9f20
	ctx.lr = 0x832A3358;
	sub_82CA9F20(ctx, base);
	// 832A3358: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A335C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3360: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3364: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A3368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A3368 size=12
    let mut pc: u32 = 0x832A3368;
    'dispatch: loop {
        match pc {
            0x832A3368 => {
    //   block [0x832A3368..0x832A3374)
	// 832A3368: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A336C: 386B8338  addi r3, r11, -0x7cc8
	ctx.r[3].s64 = ctx.r[11].s64 + -31944;
	// 832A3370: 4BA06BB0  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A3378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3378 size=104
    let mut pc: u32 = 0x832A3378;
    'dispatch: loop {
        match pc {
            0x832A3378 => {
    //   block [0x832A3378..0x832A33E0)
	// 832A3378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A337C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3380: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A3384: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A3388: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A338C: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 832A3390: 388A9244  addi r4, r10, -0x6dbc
	ctx.r[4].s64 = ctx.r[10].s64 + -28092;
	// 832A3394: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A3398: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A339C: 3BEA071C  addi r31, r10, 0x71c
	ctx.r[31].s64 = ctx.r[10].s64 + 1820;
	// 832A33A0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A33A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A33A8: 38AB33C0  addi r5, r11, 0x33c0
	ctx.r[5].s64 = ctx.r[11].s64 + 13248;
	// 832A33AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A33B0: 4BBE09E9  bl 0x82e83d98
	ctx.lr = 0x832A33B4;
	sub_82E83D98(ctx, base);
	// 832A33B4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832A33B8: 3D40832C  lis r10, -0x7cd4
	ctx.r[10].s64 = -2094268416;
	// 832A33BC: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832A33C0: 386A8370  addi r3, r10, -0x7c90
	ctx.r[3].s64 = ctx.r[10].s64 + -31888;
	// 832A33C4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832A33C8: 4BA06B59  bl 0x82ca9f20
	ctx.lr = 0x832A33CC;
	sub_82CA9F20(ctx, base);
	// 832A33CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A33D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A33D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A33D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A33DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A33E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A33E0 size=76
    let mut pc: u32 = 0x832A33E0;
    'dispatch: loop {
        match pc {
            0x832A33E0 => {
    //   block [0x832A33E0..0x832A342C)
	// 832A33E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A33E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A33E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A33EC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832A33F0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A33F4: 388B945C  addi r4, r11, -0x6ba4
	ctx.r[4].s64 = ctx.r[11].s64 + -27556;
	// 832A33F8: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A33FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A3400: 386B1048  addi r3, r11, 0x1048
	ctx.r[3].s64 = ctx.r[11].s64 + 4168;
	// 832A3404: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A3408: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A340C: 4BBE2375  bl 0x82e85780
	ctx.lr = 0x832A3410;
	sub_82E85780(ctx, base);
	// 832A3410: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A3414: 386B83A0  addi r3, r11, -0x7c60
	ctx.r[3].s64 = ctx.r[11].s64 + -31840;
	// 832A3418: 4BA06B09  bl 0x82ca9f20
	ctx.lr = 0x832A341C;
	sub_82CA9F20(ctx, base);
	// 832A341C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A3420: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3424: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3428: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A3430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3430 size=100
    let mut pc: u32 = 0x832A3430;
    'dispatch: loop {
        match pc {
            0x832A3430 => {
    //   block [0x832A3430..0x832A3494)
	// 832A3430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A3434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3438: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A343C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A3440: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832A3444: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A3448: 388B94FC  addi r4, r11, -0x6b04
	ctx.r[4].s64 = ctx.r[11].s64 + -27396;
	// 832A344C: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A3450: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A3454: 3BEB1A8C  addi r31, r11, 0x1a8c
	ctx.r[31].s64 = ctx.r[11].s64 + 6796;
	// 832A3458: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A345C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A3460: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A3464: 4BBE380D  bl 0x82e86c70
	ctx.lr = 0x832A3468;
	sub_82E86C70(ctx, base);
	// 832A3468: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832A346C: 3D40832C  lis r10, -0x7cd4
	ctx.r[10].s64 = -2094268416;
	// 832A3470: 396BB230  addi r11, r11, -0x4dd0
	ctx.r[11].s64 = ctx.r[11].s64 + -19920;
	// 832A3474: 386A8400  addi r3, r10, -0x7c00
	ctx.r[3].s64 = ctx.r[10].s64 + -31744;
	// 832A3478: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832A347C: 4BA06AA5  bl 0x82ca9f20
	ctx.lr = 0x832A3480;
	sub_82CA9F20(ctx, base);
	// 832A3480: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A3484: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3488: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A348C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A3490: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A3498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3498 size=100
    let mut pc: u32 = 0x832A3498;
    'dispatch: loop {
        match pc {
            0x832A3498 => {
    //   block [0x832A3498..0x832A34FC)
	// 832A3498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A349C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A34A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A34A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A34A8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832A34AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A34B0: 388B950C  addi r4, r11, -0x6af4
	ctx.r[4].s64 = ctx.r[11].s64 + -27380;
	// 832A34B4: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A34B8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A34BC: 3BEB1978  addi r31, r11, 0x1978
	ctx.r[31].s64 = ctx.r[11].s64 + 6520;
	// 832A34C0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A34C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A34C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A34CC: 4BBE37A5  bl 0x82e86c70
	ctx.lr = 0x832A34D0;
	sub_82E86C70(ctx, base);
	// 832A34D0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832A34D4: 3D40832C  lis r10, -0x7cd4
	ctx.r[10].s64 = -2094268416;
	// 832A34D8: 396BB230  addi r11, r11, -0x4dd0
	ctx.r[11].s64 = ctx.r[11].s64 + -19920;
	// 832A34DC: 386A83E8  addi r3, r10, -0x7c18
	ctx.r[3].s64 = ctx.r[10].s64 + -31768;
	// 832A34E0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832A34E4: 4BA06A3D  bl 0x82ca9f20
	ctx.lr = 0x832A34E8;
	sub_82CA9F20(ctx, base);
	// 832A34E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A34EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A34F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A34F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A34F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A3500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3500 size=80
    let mut pc: u32 = 0x832A3500;
    'dispatch: loop {
        match pc {
            0x832A3500 => {
    //   block [0x832A3500..0x832A3550)
	// 832A3500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A3504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3508: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A350C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A3510: 3D6082E9  lis r11, -0x7d17
	ctx.r[11].s64 = -2098659328;
	// 832A3514: 388A95F4  addi r4, r10, -0x6a0c
	ctx.r[4].s64 = ctx.r[10].s64 + -27148;
	// 832A3518: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A351C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A3520: 386A1BA0  addi r3, r10, 0x1ba0
	ctx.r[3].s64 = ctx.r[10].s64 + 7072;
	// 832A3524: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A3528: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A352C: 38AB8608  addi r5, r11, -0x79f8
	ctx.r[5].s64 = ctx.r[11].s64 + -31224;
	// 832A3530: 4BBE2291  bl 0x82e857c0
	ctx.lr = 0x832A3534;
	sub_82E857C0(ctx, base);
	// 832A3534: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A3538: 386B8430  addi r3, r11, -0x7bd0
	ctx.r[3].s64 = ctx.r[11].s64 + -31696;
	// 832A353C: 4BA069E5  bl 0x82ca9f20
	ctx.lr = 0x832A3540;
	sub_82CA9F20(ctx, base);
	// 832A3540: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A3544: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3548: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A354C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A3550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3550 size=100
    let mut pc: u32 = 0x832A3550;
    'dispatch: loop {
        match pc {
            0x832A3550 => {
    //   block [0x832A3550..0x832A35B4)
	// 832A3550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A3554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3558: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A355C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A3560: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832A3564: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A3568: 388B964C  addi r4, r11, -0x69b4
	ctx.r[4].s64 = ctx.r[11].s64 + -27060;
	// 832A356C: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A3570: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A3574: 3BEB20C0  addi r31, r11, 0x20c0
	ctx.r[31].s64 = ctx.r[11].s64 + 8384;
	// 832A3578: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A357C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A3580: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A3584: 4BBE6445  bl 0x82e899c8
	ctx.lr = 0x832A3588;
	sub_82E899C8(ctx, base);
	// 832A3588: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832A358C: 3D40832C  lis r10, -0x7cd4
	ctx.r[10].s64 = -2094268416;
	// 832A3590: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832A3594: 386A8448  addi r3, r10, -0x7bb8
	ctx.r[3].s64 = ctx.r[10].s64 + -31672;
	// 832A3598: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832A359C: 4BA06985  bl 0x82ca9f20
	ctx.lr = 0x832A35A0;
	sub_82CA9F20(ctx, base);
	// 832A35A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A35A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A35A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A35AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A35B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A35B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A35B8 size=104
    let mut pc: u32 = 0x832A35B8;
    'dispatch: loop {
        match pc {
            0x832A35B8 => {
    //   block [0x832A35B8..0x832A3620)
	// 832A35B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A35BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A35C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A35C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A35C8: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A35CC: 3D6082E9  lis r11, -0x7d17
	ctx.r[11].s64 = -2098659328;
	// 832A35D0: 388A96A8  addi r4, r10, -0x6958
	ctx.r[4].s64 = ctx.r[10].s64 + -26968;
	// 832A35D4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A35D8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A35DC: 3BEA21D8  addi r31, r10, 0x21d8
	ctx.r[31].s64 = ctx.r[10].s64 + 8664;
	// 832A35E0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A35E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A35E8: 38ABA350  addi r5, r11, -0x5cb0
	ctx.r[5].s64 = ctx.r[11].s64 + -23728;
	// 832A35EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A35F0: 4BBE6F81  bl 0x82e8a570
	ctx.lr = 0x832A35F4;
	sub_82E8A570(ctx, base);
	// 832A35F4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832A35F8: 3D40832C  lis r10, -0x7cd4
	ctx.r[10].s64 = -2094268416;
	// 832A35FC: 396BB230  addi r11, r11, -0x4dd0
	ctx.r[11].s64 = ctx.r[11].s64 + -19920;
	// 832A3600: 386A8478  addi r3, r10, -0x7b88
	ctx.r[3].s64 = ctx.r[10].s64 + -31624;
	// 832A3604: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832A3608: 4BA06919  bl 0x82ca9f20
	ctx.lr = 0x832A360C;
	sub_82CA9F20(ctx, base);
	// 832A360C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A3610: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3614: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3618: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A361C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A3620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3620 size=72
    let mut pc: u32 = 0x832A3620;
    'dispatch: loop {
        match pc {
            0x832A3620 => {
    //   block [0x832A3620..0x832A3668)
	// 832A3620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A3624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3628: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A362C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A3630: 3D6082E9  lis r11, -0x7d17
	ctx.r[11].s64 = -2098659328;
	// 832A3634: 388A971C  addi r4, r10, -0x68e4
	ctx.r[4].s64 = ctx.r[10].s64 + -26852;
	// 832A3638: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A363C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A3640: 386A22EC  addi r3, r10, 0x22ec
	ctx.r[3].s64 = ctx.r[10].s64 + 8940;
	// 832A3644: 38ABB520  addi r5, r11, -0x4ae0
	ctx.r[5].s64 = ctx.r[11].s64 + -19168;
	// 832A3648: 4BBE2C89  bl 0x82e862d0
	ctx.lr = 0x832A364C;
	sub_82E862D0(ctx, base);
	// 832A364C: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A3650: 386B8490  addi r3, r11, -0x7b70
	ctx.r[3].s64 = ctx.r[11].s64 + -31600;
	// 832A3654: 4BA068CD  bl 0x82ca9f20
	ctx.lr = 0x832A3658;
	sub_82CA9F20(ctx, base);
	// 832A3658: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A365C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3660: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3664: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A3668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3668 size=72
    let mut pc: u32 = 0x832A3668;
    'dispatch: loop {
        match pc {
            0x832A3668 => {
    //   block [0x832A3668..0x832A36B0)
	// 832A3668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A366C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3670: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A3674: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A3678: 3D6082E9  lis r11, -0x7d17
	ctx.r[11].s64 = -2098659328;
	// 832A367C: 388A974C  addi r4, r10, -0x68b4
	ctx.r[4].s64 = ctx.r[10].s64 + -26804;
	// 832A3680: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A3684: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A3688: 386A2400  addi r3, r10, 0x2400
	ctx.r[3].s64 = ctx.r[10].s64 + 9216;
	// 832A368C: 38ABB800  addi r5, r11, -0x4800
	ctx.r[5].s64 = ctx.r[11].s64 + -18432;
	// 832A3690: 4BBE2C41  bl 0x82e862d0
	ctx.lr = 0x832A3694;
	sub_82E862D0(ctx, base);
	// 832A3694: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A3698: 386B84A8  addi r3, r11, -0x7b58
	ctx.r[3].s64 = ctx.r[11].s64 + -31576;
	// 832A369C: 4BA06885  bl 0x82ca9f20
	ctx.lr = 0x832A36A0;
	sub_82CA9F20(ctx, base);
	// 832A36A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A36A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A36A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A36AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A36B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A36B0 size=72
    let mut pc: u32 = 0x832A36B0;
    'dispatch: loop {
        match pc {
            0x832A36B0 => {
    //   block [0x832A36B0..0x832A36F8)
	// 832A36B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A36B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A36B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A36BC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A36C0: 3D6082E9  lis r11, -0x7d17
	ctx.r[11].s64 = -2098659328;
	// 832A36C4: 388A97A0  addi r4, r10, -0x6860
	ctx.r[4].s64 = ctx.r[10].s64 + -26720;
	// 832A36C8: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A36CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A36D0: 386A2514  addi r3, r10, 0x2514
	ctx.r[3].s64 = ctx.r[10].s64 + 9492;
	// 832A36D4: 38ABBC90  addi r5, r11, -0x4370
	ctx.r[5].s64 = ctx.r[11].s64 + -17264;
	// 832A36D8: 4BBE2BF9  bl 0x82e862d0
	ctx.lr = 0x832A36DC;
	sub_82E862D0(ctx, base);
	// 832A36DC: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A36E0: 386B84C0  addi r3, r11, -0x7b40
	ctx.r[3].s64 = ctx.r[11].s64 + -31552;
	// 832A36E4: 4BA0683D  bl 0x82ca9f20
	ctx.lr = 0x832A36E8;
	sub_82CA9F20(ctx, base);
	// 832A36E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A36EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A36F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A36F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A36F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A36F8 size=80
    let mut pc: u32 = 0x832A36F8;
    'dispatch: loop {
        match pc {
            0x832A36F8 => {
    //   block [0x832A36F8..0x832A3748)
	// 832A36F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A36FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3700: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A3704: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A3708: 3D6082E9  lis r11, -0x7d17
	ctx.r[11].s64 = -2098659328;
	// 832A370C: 388A989C  addi r4, r10, -0x6764
	ctx.r[4].s64 = ctx.r[10].s64 + -26468;
	// 832A3710: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A3714: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A3718: 386A2E38  addi r3, r10, 0x2e38
	ctx.r[3].s64 = ctx.r[10].s64 + 11832;
	// 832A371C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A3720: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A3724: 38ABD820  addi r5, r11, -0x27e0
	ctx.r[5].s64 = ctx.r[11].s64 + -10208;
	// 832A3728: 4BBE2099  bl 0x82e857c0
	ctx.lr = 0x832A372C;
	sub_82E857C0(ctx, base);
	// 832A372C: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A3730: 386B84F0  addi r3, r11, -0x7b10
	ctx.r[3].s64 = ctx.r[11].s64 + -31504;
	// 832A3734: 4BA067ED  bl 0x82ca9f20
	ctx.lr = 0x832A3738;
	sub_82CA9F20(ctx, base);
	// 832A3738: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A373C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3740: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3744: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A3748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3748 size=68
    let mut pc: u32 = 0x832A3748;
    'dispatch: loop {
        match pc {
            0x832A3748 => {
    //   block [0x832A3748..0x832A378C)
	// 832A3748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A374C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3750: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A3754: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832A3758: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A375C: 388B996C  addi r4, r11, -0x6694
	ctx.r[4].s64 = ctx.r[11].s64 + -26260;
	// 832A3760: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A3764: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A3768: 386B581C  addi r3, r11, 0x581c
	ctx.r[3].s64 = ctx.r[11].s64 + 22556;
	// 832A376C: 4BBE248D  bl 0x82e85bf8
	ctx.lr = 0x832A3770;
	sub_82E85BF8(ctx, base);
	// 832A3770: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A3774: 386B85F8  addi r3, r11, -0x7a08
	ctx.r[3].s64 = ctx.r[11].s64 + -31240;
	// 832A3778: 4BA067A9  bl 0x82ca9f20
	ctx.lr = 0x832A377C;
	sub_82CA9F20(ctx, base);
	// 832A377C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A3780: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3784: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3788: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A3790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3790 size=76
    let mut pc: u32 = 0x832A3790;
    'dispatch: loop {
        match pc {
            0x832A3790 => {
    //   block [0x832A3790..0x832A37DC)
	// 832A3790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A3794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3798: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A379C: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A37A0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A37A4: 38CB581C  addi r6, r11, 0x581c
	ctx.r[6].s64 = ctx.r[11].s64 + 22556;
	// 832A37A8: 388A9978  addi r4, r10, -0x6688
	ctx.r[4].s64 = ctx.r[10].s64 + -26248;
	// 832A37AC: 3D6082E9  lis r11, -0x7d17
	ctx.r[11].s64 = -2098659328;
	// 832A37B0: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A37B4: 38ABF078  addi r5, r11, -0xf88
	ctx.r[5].s64 = ctx.r[11].s64 + -3976;
	// 832A37B8: 386A52B8  addi r3, r10, 0x52b8
	ctx.r[3].s64 = ctx.r[10].s64 + 21176;
	// 832A37BC: 4BBE243D  bl 0x82e85bf8
	ctx.lr = 0x832A37C0;
	sub_82E85BF8(ctx, base);
	// 832A37C0: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A37C4: 386B8538  addi r3, r11, -0x7ac8
	ctx.r[3].s64 = ctx.r[11].s64 + -31432;
	// 832A37C8: 4BA06759  bl 0x82ca9f20
	ctx.lr = 0x832A37CC;
	sub_82CA9F20(ctx, base);
	// 832A37CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A37D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A37D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A37D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A37E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A37E0 size=76
    let mut pc: u32 = 0x832A37E0;
    'dispatch: loop {
        match pc {
            0x832A37E0 => {
    //   block [0x832A37E0..0x832A382C)
	// 832A37E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A37E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A37E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A37EC: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A37F0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A37F4: 38CB581C  addi r6, r11, 0x581c
	ctx.r[6].s64 = ctx.r[11].s64 + 22556;
	// 832A37F8: 388A9990  addi r4, r10, -0x6670
	ctx.r[4].s64 = ctx.r[10].s64 + -26224;
	// 832A37FC: 3D6082E9  lis r11, -0x7d17
	ctx.r[11].s64 = -2098659328;
	// 832A3800: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A3804: 38AB0198  addi r5, r11, 0x198
	ctx.r[5].s64 = ctx.r[11].s64 + 408;
	// 832A3808: 386A5708  addi r3, r10, 0x5708
	ctx.r[3].s64 = ctx.r[10].s64 + 22280;
	// 832A380C: 4BBE23ED  bl 0x82e85bf8
	ctx.lr = 0x832A3810;
	sub_82E85BF8(ctx, base);
	// 832A3810: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A3814: 386B8550  addi r3, r11, -0x7ab0
	ctx.r[3].s64 = ctx.r[11].s64 + -31408;
	// 832A3818: 4BA06709  bl 0x82ca9f20
	ctx.lr = 0x832A381C;
	sub_82CA9F20(ctx, base);
	// 832A381C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A3820: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3824: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3828: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A3830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3830 size=76
    let mut pc: u32 = 0x832A3830;
    'dispatch: loop {
        match pc {
            0x832A3830 => {
    //   block [0x832A3830..0x832A387C)
	// 832A3830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A3834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3838: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A383C: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A3840: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A3844: 38CB581C  addi r6, r11, 0x581c
	ctx.r[6].s64 = ctx.r[11].s64 + 22556;
	// 832A3848: 388A99A8  addi r4, r10, -0x6658
	ctx.r[4].s64 = ctx.r[10].s64 + -26200;
	// 832A384C: 3D6082E9  lis r11, -0x7d17
	ctx.r[11].s64 = -2098659328;
	// 832A3850: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A3854: 38ABFAF0  addi r5, r11, -0x510
	ctx.r[5].s64 = ctx.r[11].s64 + -1296;
	// 832A3858: 386A53CC  addi r3, r10, 0x53cc
	ctx.r[3].s64 = ctx.r[10].s64 + 21452;
	// 832A385C: 4BBE239D  bl 0x82e85bf8
	ctx.lr = 0x832A3860;
	sub_82E85BF8(ctx, base);
	// 832A3860: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A3864: 386B8568  addi r3, r11, -0x7a98
	ctx.r[3].s64 = ctx.r[11].s64 + -31384;
	// 832A3868: 4BA066B9  bl 0x82ca9f20
	ctx.lr = 0x832A386C;
	sub_82CA9F20(ctx, base);
	// 832A386C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A3870: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3874: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3878: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A3880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3880 size=76
    let mut pc: u32 = 0x832A3880;
    'dispatch: loop {
        match pc {
            0x832A3880 => {
    //   block [0x832A3880..0x832A38CC)
	// 832A3880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A3884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3888: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A388C: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A3890: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A3894: 38CB581C  addi r6, r11, 0x581c
	ctx.r[6].s64 = ctx.r[11].s64 + 22556;
	// 832A3898: 388A99CC  addi r4, r10, -0x6634
	ctx.r[4].s64 = ctx.r[10].s64 + -26164;
	// 832A389C: 3D6082E9  lis r11, -0x7d17
	ctx.r[11].s64 = -2098659328;
	// 832A38A0: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A38A4: 38ABFDB0  addi r5, r11, -0x250
	ctx.r[5].s64 = ctx.r[11].s64 + -592;
	// 832A38A8: 386A5B58  addi r3, r10, 0x5b58
	ctx.r[3].s64 = ctx.r[10].s64 + 23384;
	// 832A38AC: 4BBE234D  bl 0x82e85bf8
	ctx.lr = 0x832A38B0;
	sub_82E85BF8(ctx, base);
	// 832A38B0: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A38B4: 386B8580  addi r3, r11, -0x7a80
	ctx.r[3].s64 = ctx.r[11].s64 + -31360;
	// 832A38B8: 4BA06669  bl 0x82ca9f20
	ctx.lr = 0x832A38BC;
	sub_82CA9F20(ctx, base);
	// 832A38BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A38C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A38C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A38C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A38D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A38D0 size=76
    let mut pc: u32 = 0x832A38D0;
    'dispatch: loop {
        match pc {
            0x832A38D0 => {
    //   block [0x832A38D0..0x832A391C)
	// 832A38D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A38D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A38D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A38DC: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A38E0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A38E4: 38CB52B8  addi r6, r11, 0x52b8
	ctx.r[6].s64 = ctx.r[11].s64 + 21176;
	// 832A38E8: 388A99E8  addi r4, r10, -0x6618
	ctx.r[4].s64 = ctx.r[10].s64 + -26136;
	// 832A38EC: 3D6082E9  lis r11, -0x7d17
	ctx.r[11].s64 = -2098659328;
	// 832A38F0: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A38F4: 38ABF360  addi r5, r11, -0xca0
	ctx.r[5].s64 = ctx.r[11].s64 + -3232;
	// 832A38F8: 386A5930  addi r3, r10, 0x5930
	ctx.r[3].s64 = ctx.r[10].s64 + 22832;
	// 832A38FC: 4BBE22FD  bl 0x82e85bf8
	ctx.lr = 0x832A3900;
	sub_82E85BF8(ctx, base);
	// 832A3900: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A3904: 386B8598  addi r3, r11, -0x7a68
	ctx.r[3].s64 = ctx.r[11].s64 + -31336;
	// 832A3908: 4BA06619  bl 0x82ca9f20
	ctx.lr = 0x832A390C;
	sub_82CA9F20(ctx, base);
	// 832A390C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A3910: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3914: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3918: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A3920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3920 size=104
    let mut pc: u32 = 0x832A3920;
    'dispatch: loop {
        match pc {
            0x832A3920 => {
    //   block [0x832A3920..0x832A3988)
	// 832A3920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A3924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3928: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A392C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A3930: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A3934: 3D6082E9  lis r11, -0x7d17
	ctx.r[11].s64 = -2098659328;
	// 832A3938: 388A99FC  addi r4, r10, -0x6604
	ctx.r[4].s64 = ctx.r[10].s64 + -26116;
	// 832A393C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A3940: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A3944: 3BEA55F4  addi r31, r10, 0x55f4
	ctx.r[31].s64 = ctx.r[10].s64 + 22004;
	// 832A3948: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A394C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A3950: 38ABF450  addi r5, r11, -0xbb0
	ctx.r[5].s64 = ctx.r[11].s64 + -2992;
	// 832A3954: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A3958: 4BBEC9B9  bl 0x82e90310
	ctx.lr = 0x832A395C;
	sub_82E90310(ctx, base);
	// 832A395C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832A3960: 3D40832C  lis r10, -0x7cd4
	ctx.r[10].s64 = -2094268416;
	// 832A3964: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832A3968: 386A85B0  addi r3, r10, -0x7a50
	ctx.r[3].s64 = ctx.r[10].s64 + -31312;
	// 832A396C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832A3970: 4BA065B1  bl 0x82ca9f20
	ctx.lr = 0x832A3974;
	sub_82CA9F20(ctx, base);
	// 832A3974: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A3978: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A397C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3980: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A3984: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A3988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3988 size=104
    let mut pc: u32 = 0x832A3988;
    'dispatch: loop {
        match pc {
            0x832A3988 => {
    //   block [0x832A3988..0x832A39F0)
	// 832A3988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A398C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3990: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A3994: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A3998: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A399C: 3D6082E9  lis r11, -0x7d17
	ctx.r[11].s64 = -2098659328;
	// 832A39A0: 388A9A18  addi r4, r10, -0x65e8
	ctx.r[4].s64 = ctx.r[10].s64 + -26088;
	// 832A39A4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A39A8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A39AC: 3BEA5A44  addi r31, r10, 0x5a44
	ctx.r[31].s64 = ctx.r[10].s64 + 23108;
	// 832A39B0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A39B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A39B8: 38ABF498  addi r5, r11, -0xb68
	ctx.r[5].s64 = ctx.r[11].s64 + -2920;
	// 832A39BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A39C0: 4BBEC951  bl 0x82e90310
	ctx.lr = 0x832A39C4;
	sub_82E90310(ctx, base);
	// 832A39C4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832A39C8: 3D40832C  lis r10, -0x7cd4
	ctx.r[10].s64 = -2094268416;
	// 832A39CC: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832A39D0: 386A85C8  addi r3, r10, -0x7a38
	ctx.r[3].s64 = ctx.r[10].s64 + -31288;
	// 832A39D4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832A39D8: 4BA06549  bl 0x82ca9f20
	ctx.lr = 0x832A39DC;
	sub_82CA9F20(ctx, base);
	// 832A39DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A39E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A39E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A39E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A39EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A39F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A39F0 size=104
    let mut pc: u32 = 0x832A39F0;
    'dispatch: loop {
        match pc {
            0x832A39F0 => {
    //   block [0x832A39F0..0x832A3A58)
	// 832A39F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A39F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A39F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A39FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A3A00: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A3A04: 3D6082E9  lis r11, -0x7d17
	ctx.r[11].s64 = -2098659328;
	// 832A3A08: 388A9A28  addi r4, r10, -0x65d8
	ctx.r[4].s64 = ctx.r[10].s64 + -26072;
	// 832A3A0C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A3A10: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A3A14: 3BEA54E0  addi r31, r10, 0x54e0
	ctx.r[31].s64 = ctx.r[10].s64 + 21728;
	// 832A3A18: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A3A1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A3A20: 38ABF4E0  addi r5, r11, -0xb20
	ctx.r[5].s64 = ctx.r[11].s64 + -2848;
	// 832A3A24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A3A28: 4BBEC8E9  bl 0x82e90310
	ctx.lr = 0x832A3A2C;
	sub_82E90310(ctx, base);
	// 832A3A2C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832A3A30: 3D40832C  lis r10, -0x7cd4
	ctx.r[10].s64 = -2094268416;
	// 832A3A34: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832A3A38: 386A85E0  addi r3, r10, -0x7a20
	ctx.r[3].s64 = ctx.r[10].s64 + -31264;
	// 832A3A3C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832A3A40: 4BA064E1  bl 0x82ca9f20
	ctx.lr = 0x832A3A44;
	sub_82CA9F20(ctx, base);
	// 832A3A44: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A3A48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3A4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3A50: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A3A54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A3A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3A58 size=68
    let mut pc: u32 = 0x832A3A58;
    'dispatch: loop {
        match pc {
            0x832A3A58 => {
    //   block [0x832A3A58..0x832A3A9C)
	// 832A3A58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A3A5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3A60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A3A64: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832A3A68: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A3A6C: 388B9AB8  addi r4, r11, -0x6548
	ctx.r[4].s64 = ctx.r[11].s64 + -25928;
	// 832A3A70: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A3A74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A3A78: 386B5C6C  addi r3, r11, 0x5c6c
	ctx.r[3].s64 = ctx.r[11].s64 + 23660;
	// 832A3A7C: 4BBDBAE5  bl 0x82e7f560
	ctx.lr = 0x832A3A80;
	sub_82E7F560(ctx, base);
	// 832A3A80: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A3A84: 386B8628  addi r3, r11, -0x79d8
	ctx.r[3].s64 = ctx.r[11].s64 + -31192;
	// 832A3A88: 4BA06499  bl 0x82ca9f20
	ctx.lr = 0x832A3A8C;
	sub_82CA9F20(ctx, base);
	// 832A3A8C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A3A90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3A94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3A98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A3AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3AA0 size=68
    let mut pc: u32 = 0x832A3AA0;
    'dispatch: loop {
        match pc {
            0x832A3AA0 => {
    //   block [0x832A3AA0..0x832A3AE4)
	// 832A3AA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A3AA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3AA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A3AAC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A3AB0: 3D6082E9  lis r11, -0x7d17
	ctx.r[11].s64 = -2098659328;
	// 832A3AB4: 388A9B64  addi r4, r10, -0x649c
	ctx.r[4].s64 = ctx.r[10].s64 + -25756;
	// 832A3AB8: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A3ABC: 38AB6E58  addi r5, r11, 0x6e58
	ctx.r[5].s64 = ctx.r[11].s64 + 28248;
	// 832A3AC0: 386A5D84  addi r3, r10, 0x5d84
	ctx.r[3].s64 = ctx.r[10].s64 + 23940;
	// 832A3AC4: 4BBF4D1D  bl 0x82e987e0
	ctx.lr = 0x832A3AC8;
	sub_82E987E0(ctx, base);
	// 832A3AC8: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A3ACC: 386B8640  addi r3, r11, -0x79c0
	ctx.r[3].s64 = ctx.r[11].s64 + -31168;
	// 832A3AD0: 4BA06451  bl 0x82ca9f20
	ctx.lr = 0x832A3AD4;
	sub_82CA9F20(ctx, base);
	// 832A3AD4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A3AD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3ADC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3AE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A3AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3AE8 size=80
    let mut pc: u32 = 0x832A3AE8;
    'dispatch: loop {
        match pc {
            0x832A3AE8 => {
    //   block [0x832A3AE8..0x832A3B38)
	// 832A3AE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A3AEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3AF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A3AF4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A3AF8: 3D6082EA  lis r11, -0x7d16
	ctx.r[11].s64 = -2098593792;
	// 832A3AFC: 388A9BA0  addi r4, r10, -0x6460
	ctx.r[4].s64 = ctx.r[10].s64 + -25696;
	// 832A3B00: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A3B04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A3B08: 386A5E98  addi r3, r10, 0x5e98
	ctx.r[3].s64 = ctx.r[10].s64 + 24216;
	// 832A3B0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A3B10: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A3B14: 38AB85C0  addi r5, r11, -0x7a40
	ctx.r[5].s64 = ctx.r[11].s64 + -31296;
	// 832A3B18: 4BBE1CA9  bl 0x82e857c0
	ctx.lr = 0x832A3B1C;
	sub_82E857C0(ctx, base);
	// 832A3B1C: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A3B20: 386B8658  addi r3, r11, -0x79a8
	ctx.r[3].s64 = ctx.r[11].s64 + -31144;
	// 832A3B24: 4BA063FD  bl 0x82ca9f20
	ctx.lr = 0x832A3B28;
	sub_82CA9F20(ctx, base);
	// 832A3B28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A3B2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3B30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3B34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A3B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3B38 size=68
    let mut pc: u32 = 0x832A3B38;
    'dispatch: loop {
        match pc {
            0x832A3B38 => {
    //   block [0x832A3B38..0x832A3B7C)
	// 832A3B38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A3B3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3B40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A3B44: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832A3B48: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A3B4C: 388B9BD4  addi r4, r11, -0x642c
	ctx.r[4].s64 = ctx.r[11].s64 + -25644;
	// 832A3B50: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A3B54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A3B58: 386B60C0  addi r3, r11, 0x60c0
	ctx.r[3].s64 = ctx.r[11].s64 + 24768;
	// 832A3B5C: 4BBE5F7D  bl 0x82e89ad8
	ctx.lr = 0x832A3B60;
	sub_82E89AD8(ctx, base);
	// 832A3B60: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A3B64: 386B8670  addi r3, r11, -0x7990
	ctx.r[3].s64 = ctx.r[11].s64 + -31120;
	// 832A3B68: 4BA063B9  bl 0x82ca9f20
	ctx.lr = 0x832A3B6C;
	sub_82CA9F20(ctx, base);
	// 832A3B6C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A3B70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3B74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3B78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A3B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3B80 size=72
    let mut pc: u32 = 0x832A3B80;
    'dispatch: loop {
        match pc {
            0x832A3B80 => {
    //   block [0x832A3B80..0x832A3BC8)
	// 832A3B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A3B84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3B88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A3B8C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A3B90: 3D6082EA  lis r11, -0x7d16
	ctx.r[11].s64 = -2098593792;
	// 832A3B94: 388A9BE4  addi r4, r10, -0x641c
	ctx.r[4].s64 = ctx.r[10].s64 + -25628;
	// 832A3B98: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A3B9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A3BA0: 386A5FAC  addi r3, r10, 0x5fac
	ctx.r[3].s64 = ctx.r[10].s64 + 24492;
	// 832A3BA4: 38AB8908  addi r5, r11, -0x76f8
	ctx.r[5].s64 = ctx.r[11].s64 + -30456;
	// 832A3BA8: 4BBE5EE9  bl 0x82e89a90
	ctx.lr = 0x832A3BAC;
	sub_82E89A90(ctx, base);
	// 832A3BAC: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A3BB0: 386B8688  addi r3, r11, -0x7978
	ctx.r[3].s64 = ctx.r[11].s64 + -31096;
	// 832A3BB4: 4BA0636D  bl 0x82ca9f20
	ctx.lr = 0x832A3BB8;
	sub_82CA9F20(ctx, base);
	// 832A3BB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A3BBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3BC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3BC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A3BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3BC8 size=68
    let mut pc: u32 = 0x832A3BC8;
    'dispatch: loop {
        match pc {
            0x832A3BC8 => {
    //   block [0x832A3BC8..0x832A3C0C)
	// 832A3BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A3BCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3BD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A3BD4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A3BD8: 3D6082EA  lis r11, -0x7d16
	ctx.r[11].s64 = -2098593792;
	// 832A3BDC: 388A9C8C  addi r4, r10, -0x6374
	ctx.r[4].s64 = ctx.r[10].s64 + -25460;
	// 832A3BE0: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A3BE4: 38AB9578  addi r5, r11, -0x6a88
	ctx.r[5].s64 = ctx.r[11].s64 + -27272;
	// 832A3BE8: 386A61D4  addi r3, r10, 0x61d4
	ctx.r[3].s64 = ctx.r[10].s64 + 25044;
	// 832A3BEC: 4BC0A7FD  bl 0x82eae3e8
	ctx.lr = 0x832A3BF0;
	sub_82EAE3E8(ctx, base);
	// 832A3BF0: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A3BF4: 386B86A0  addi r3, r11, -0x7960
	ctx.r[3].s64 = ctx.r[11].s64 + -31072;
	// 832A3BF8: 4BA06329  bl 0x82ca9f20
	ctx.lr = 0x832A3BFC;
	sub_82CA9F20(ctx, base);
	// 832A3BFC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A3C00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3C04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3C08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A3C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3C10 size=72
    let mut pc: u32 = 0x832A3C10;
    'dispatch: loop {
        match pc {
            0x832A3C10 => {
    //   block [0x832A3C10..0x832A3C58)
	// 832A3C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A3C14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3C18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A3C1C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A3C20: 3D6082EA  lis r11, -0x7d16
	ctx.r[11].s64 = -2098593792;
	// 832A3C24: 388A9D00  addi r4, r10, -0x6300
	ctx.r[4].s64 = ctx.r[10].s64 + -25344;
	// 832A3C28: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A3C2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A3C30: 386A62EC  addi r3, r10, 0x62ec
	ctx.r[3].s64 = ctx.r[10].s64 + 25324;
	// 832A3C34: 38ABA210  addi r5, r11, -0x5df0
	ctx.r[5].s64 = ctx.r[11].s64 + -24048;
	// 832A3C38: 4BBFD899  bl 0x82ea14d0
	ctx.lr = 0x832A3C3C;
	sub_82EA14D0(ctx, base);
	// 832A3C3C: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A3C40: 386B86B8  addi r3, r11, -0x7948
	ctx.r[3].s64 = ctx.r[11].s64 + -31048;
	// 832A3C44: 4BA062DD  bl 0x82ca9f20
	ctx.lr = 0x832A3C48;
	sub_82CA9F20(ctx, base);
	// 832A3C48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A3C4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3C50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3C54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A3C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3C58 size=72
    let mut pc: u32 = 0x832A3C58;
    'dispatch: loop {
        match pc {
            0x832A3C58 => {
    //   block [0x832A3C58..0x832A3CA0)
	// 832A3C58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A3C5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3C60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A3C64: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A3C68: 3D6082EA  lis r11, -0x7d16
	ctx.r[11].s64 = -2098593792;
	// 832A3C6C: 388A9D78  addi r4, r10, -0x6288
	ctx.r[4].s64 = ctx.r[10].s64 + -25224;
	// 832A3C70: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A3C74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A3C78: 386A6404  addi r3, r10, 0x6404
	ctx.r[3].s64 = ctx.r[10].s64 + 25604;
	// 832A3C7C: 38ABAE28  addi r5, r11, -0x51d8
	ctx.r[5].s64 = ctx.r[11].s64 + -20952;
	// 832A3C80: 4BBFD851  bl 0x82ea14d0
	ctx.lr = 0x832A3C84;
	sub_82EA14D0(ctx, base);
	// 832A3C84: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A3C88: 386B86D0  addi r3, r11, -0x7930
	ctx.r[3].s64 = ctx.r[11].s64 + -31024;
	// 832A3C8C: 4BA06295  bl 0x82ca9f20
	ctx.lr = 0x832A3C90;
	sub_82CA9F20(ctx, base);
	// 832A3C90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A3C94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3C98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3C9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A3CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3CA0 size=104
    let mut pc: u32 = 0x832A3CA0;
    'dispatch: loop {
        match pc {
            0x832A3CA0 => {
    //   block [0x832A3CA0..0x832A3D08)
	// 832A3CA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A3CA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3CA8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A3CAC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A3CB0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A3CB4: 3D6082EA  lis r11, -0x7d16
	ctx.r[11].s64 = -2098593792;
	// 832A3CB8: 388A9DCC  addi r4, r10, -0x6234
	ctx.r[4].s64 = ctx.r[10].s64 + -25140;
	// 832A3CBC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A3CC0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A3CC4: 3BEA6920  addi r31, r10, 0x6920
	ctx.r[31].s64 = ctx.r[10].s64 + 26912;
	// 832A3CC8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A3CCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A3CD0: 38ABAF28  addi r5, r11, -0x50d8
	ctx.r[5].s64 = ctx.r[11].s64 + -20696;
	// 832A3CD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A3CD8: 4BBF73E1  bl 0x82e9b0b8
	ctx.lr = 0x832A3CDC;
	sub_82E9B0B8(ctx, base);
	// 832A3CDC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832A3CE0: 3D40832C  lis r10, -0x7cd4
	ctx.r[10].s64 = -2094268416;
	// 832A3CE4: 396BB230  addi r11, r11, -0x4dd0
	ctx.r[11].s64 = ctx.r[11].s64 + -19920;
	// 832A3CE8: 386A86E8  addi r3, r10, -0x7918
	ctx.r[3].s64 = ctx.r[10].s64 + -31000;
	// 832A3CEC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832A3CF0: 4BA06231  bl 0x82ca9f20
	ctx.lr = 0x832A3CF4;
	sub_82CA9F20(ctx, base);
	// 832A3CF4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A3CF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3CFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3D00: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A3D04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A3D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3D08 size=104
    let mut pc: u32 = 0x832A3D08;
    'dispatch: loop {
        match pc {
            0x832A3D08 => {
    //   block [0x832A3D08..0x832A3D70)
	// 832A3D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A3D0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3D10: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A3D14: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A3D18: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A3D1C: 3D6082EA  lis r11, -0x7d16
	ctx.r[11].s64 = -2098593792;
	// 832A3D20: 388A9E40  addi r4, r10, -0x61c0
	ctx.r[4].s64 = ctx.r[10].s64 + -25024;
	// 832A3D24: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A3D28: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A3D2C: 3BEA6A34  addi r31, r10, 0x6a34
	ctx.r[31].s64 = ctx.r[10].s64 + 27188;
	// 832A3D30: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A3D34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A3D38: 38ABB860  addi r5, r11, -0x47a0
	ctx.r[5].s64 = ctx.r[11].s64 + -18336;
	// 832A3D3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A3D40: 4BBF7379  bl 0x82e9b0b8
	ctx.lr = 0x832A3D44;
	sub_82E9B0B8(ctx, base);
	// 832A3D44: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832A3D48: 3D40832C  lis r10, -0x7cd4
	ctx.r[10].s64 = -2094268416;
	// 832A3D4C: 396BB230  addi r11, r11, -0x4dd0
	ctx.r[11].s64 = ctx.r[11].s64 + -19920;
	// 832A3D50: 386A8718  addi r3, r10, -0x78e8
	ctx.r[3].s64 = ctx.r[10].s64 + -30952;
	// 832A3D54: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832A3D58: 4BA061C9  bl 0x82ca9f20
	ctx.lr = 0x832A3D5C;
	sub_82CA9F20(ctx, base);
	// 832A3D5C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A3D60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3D64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3D68: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A3D6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A3D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3D70 size=104
    let mut pc: u32 = 0x832A3D70;
    'dispatch: loop {
        match pc {
            0x832A3D70 => {
    //   block [0x832A3D70..0x832A3DD8)
	// 832A3D70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A3D74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3D78: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A3D7C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A3D80: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A3D84: 3D6082EA  lis r11, -0x7d16
	ctx.r[11].s64 = -2098593792;
	// 832A3D88: 388A9E9C  addi r4, r10, -0x6164
	ctx.r[4].s64 = ctx.r[10].s64 + -24932;
	// 832A3D8C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A3D90: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A3D94: 3BEA6B48  addi r31, r10, 0x6b48
	ctx.r[31].s64 = ctx.r[10].s64 + 27464;
	// 832A3D98: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A3D9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A3DA0: 38ABBE98  addi r5, r11, -0x4168
	ctx.r[5].s64 = ctx.r[11].s64 + -16744;
	// 832A3DA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A3DA8: 4BBF7311  bl 0x82e9b0b8
	ctx.lr = 0x832A3DAC;
	sub_82E9B0B8(ctx, base);
	// 832A3DAC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832A3DB0: 3D40832C  lis r10, -0x7cd4
	ctx.r[10].s64 = -2094268416;
	// 832A3DB4: 396BB230  addi r11, r11, -0x4dd0
	ctx.r[11].s64 = ctx.r[11].s64 + -19920;
	// 832A3DB8: 386A8730  addi r3, r10, -0x78d0
	ctx.r[3].s64 = ctx.r[10].s64 + -30928;
	// 832A3DBC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832A3DC0: 4BA06161  bl 0x82ca9f20
	ctx.lr = 0x832A3DC4;
	sub_82CA9F20(ctx, base);
	// 832A3DC4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A3DC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3DCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3DD0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A3DD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A3DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3DD8 size=104
    let mut pc: u32 = 0x832A3DD8;
    'dispatch: loop {
        match pc {
            0x832A3DD8 => {
    //   block [0x832A3DD8..0x832A3E40)
	// 832A3DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A3DDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3DE0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A3DE4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A3DE8: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A3DEC: 3D6082EA  lis r11, -0x7d16
	ctx.r[11].s64 = -2098593792;
	// 832A3DF0: 388A9EF4  addi r4, r10, -0x610c
	ctx.r[4].s64 = ctx.r[10].s64 + -24844;
	// 832A3DF4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A3DF8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A3DFC: 3BEA6C5C  addi r31, r10, 0x6c5c
	ctx.r[31].s64 = ctx.r[10].s64 + 27740;
	// 832A3E00: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A3E04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A3E08: 38ABC380  addi r5, r11, -0x3c80
	ctx.r[5].s64 = ctx.r[11].s64 + -15488;
	// 832A3E0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A3E10: 4BBF72A9  bl 0x82e9b0b8
	ctx.lr = 0x832A3E14;
	sub_82E9B0B8(ctx, base);
	// 832A3E14: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832A3E18: 3D40832C  lis r10, -0x7cd4
	ctx.r[10].s64 = -2094268416;
	// 832A3E1C: 396BB230  addi r11, r11, -0x4dd0
	ctx.r[11].s64 = ctx.r[11].s64 + -19920;
	// 832A3E20: 386A8748  addi r3, r10, -0x78b8
	ctx.r[3].s64 = ctx.r[10].s64 + -30904;
	// 832A3E24: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832A3E28: 4BA060F9  bl 0x82ca9f20
	ctx.lr = 0x832A3E2C;
	sub_82CA9F20(ctx, base);
	// 832A3E2C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A3E30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3E34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3E38: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A3E3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A3E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3E40 size=80
    let mut pc: u32 = 0x832A3E40;
    'dispatch: loop {
        match pc {
            0x832A3E40 => {
    //   block [0x832A3E40..0x832A3E90)
	// 832A3E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A3E44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3E48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A3E4C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A3E50: 3D6082EA  lis r11, -0x7d16
	ctx.r[11].s64 = -2098593792;
	// 832A3E54: 388A9F6C  addi r4, r10, -0x6094
	ctx.r[4].s64 = ctx.r[10].s64 + -24724;
	// 832A3E58: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A3E5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A3E60: 386A6D70  addi r3, r10, 0x6d70
	ctx.r[3].s64 = ctx.r[10].s64 + 28016;
	// 832A3E64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A3E68: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A3E6C: 38ABDB00  addi r5, r11, -0x2500
	ctx.r[5].s64 = ctx.r[11].s64 + -9472;
	// 832A3E70: 4BBE1951  bl 0x82e857c0
	ctx.lr = 0x832A3E74;
	sub_82E857C0(ctx, base);
	// 832A3E74: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A3E78: 386B8760  addi r3, r11, -0x78a0
	ctx.r[3].s64 = ctx.r[11].s64 + -30880;
	// 832A3E7C: 4BA060A5  bl 0x82ca9f20
	ctx.lr = 0x832A3E80;
	sub_82CA9F20(ctx, base);
	// 832A3E80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A3E84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3E88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3E8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A3E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3E90 size=68
    let mut pc: u32 = 0x832A3E90;
    'dispatch: loop {
        match pc {
            0x832A3E90 => {
    //   block [0x832A3E90..0x832A3ED4)
	// 832A3E90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A3E94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3E98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A3E9C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A3EA0: 3D6082EA  lis r11, -0x7d16
	ctx.r[11].s64 = -2098593792;
	// 832A3EA4: 388A9FC4  addi r4, r10, -0x603c
	ctx.r[4].s64 = ctx.r[10].s64 + -24636;
	// 832A3EA8: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A3EAC: 38ABE3F8  addi r5, r11, -0x1c08
	ctx.r[5].s64 = ctx.r[11].s64 + -7176;
	// 832A3EB0: 386A6E84  addi r3, r10, 0x6e84
	ctx.r[3].s64 = ctx.r[10].s64 + 28292;
	// 832A3EB4: 4BBF492D  bl 0x82e987e0
	ctx.lr = 0x832A3EB8;
	sub_82E987E0(ctx, base);
	// 832A3EB8: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A3EBC: 386B8778  addi r3, r11, -0x7888
	ctx.r[3].s64 = ctx.r[11].s64 + -30856;
	// 832A3EC0: 4BA06061  bl 0x82ca9f20
	ctx.lr = 0x832A3EC4;
	sub_82CA9F20(ctx, base);
	// 832A3EC4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A3EC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3ECC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3ED0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A3ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3ED8 size=68
    let mut pc: u32 = 0x832A3ED8;
    'dispatch: loop {
        match pc {
            0x832A3ED8 => {
    //   block [0x832A3ED8..0x832A3F1C)
	// 832A3ED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A3EDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3EE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A3EE4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A3EE8: 3D6082EA  lis r11, -0x7d16
	ctx.r[11].s64 = -2098593792;
	// 832A3EEC: 388AA030  addi r4, r10, -0x5fd0
	ctx.r[4].s64 = ctx.r[10].s64 + -24528;
	// 832A3EF0: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A3EF4: 38ABEB08  addi r5, r11, -0x14f8
	ctx.r[5].s64 = ctx.r[11].s64 + -5368;
	// 832A3EF8: 386A6F98  addi r3, r10, 0x6f98
	ctx.r[3].s64 = ctx.r[10].s64 + 28568;
	// 832A3EFC: 4BBF48E5  bl 0x82e987e0
	ctx.lr = 0x832A3F00;
	sub_82E987E0(ctx, base);
	// 832A3F00: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A3F04: 386B8790  addi r3, r11, -0x7870
	ctx.r[3].s64 = ctx.r[11].s64 + -30832;
	// 832A3F08: 4BA06019  bl 0x82ca9f20
	ctx.lr = 0x832A3F0C;
	sub_82CA9F20(ctx, base);
	// 832A3F0C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A3F10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3F14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3F18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A3F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3F20 size=68
    let mut pc: u32 = 0x832A3F20;
    'dispatch: loop {
        match pc {
            0x832A3F20 => {
    //   block [0x832A3F20..0x832A3F64)
	// 832A3F20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A3F24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3F28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A3F2C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A3F30: 3D6082EA  lis r11, -0x7d16
	ctx.r[11].s64 = -2098593792;
	// 832A3F34: 388AA070  addi r4, r10, -0x5f90
	ctx.r[4].s64 = ctx.r[10].s64 + -24464;
	// 832A3F38: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A3F3C: 38ABF7B8  addi r5, r11, -0x848
	ctx.r[5].s64 = ctx.r[11].s64 + -2120;
	// 832A3F40: 386A70AC  addi r3, r10, 0x70ac
	ctx.r[3].s64 = ctx.r[10].s64 + 28844;
	// 832A3F44: 4BBF489D  bl 0x82e987e0
	ctx.lr = 0x832A3F48;
	sub_82E987E0(ctx, base);
	// 832A3F48: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A3F4C: 386B87A8  addi r3, r11, -0x7858
	ctx.r[3].s64 = ctx.r[11].s64 + -30808;
	// 832A3F50: 4BA05FD1  bl 0x82ca9f20
	ctx.lr = 0x832A3F54;
	sub_82CA9F20(ctx, base);
	// 832A3F54: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A3F58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3F5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3F60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A3F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3F68 size=68
    let mut pc: u32 = 0x832A3F68;
    'dispatch: loop {
        match pc {
            0x832A3F68 => {
    //   block [0x832A3F68..0x832A3FAC)
	// 832A3F68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A3F6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3F70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A3F74: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832A3F78: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A3F7C: 388BA1A8  addi r4, r11, -0x5e58
	ctx.r[4].s64 = ctx.r[11].s64 + -24152;
	// 832A3F80: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A3F84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A3F88: 386B75C8  addi r3, r11, 0x75c8
	ctx.r[3].s64 = ctx.r[11].s64 + 30152;
	// 832A3F8C: 4BBE5B4D  bl 0x82e89ad8
	ctx.lr = 0x832A3F90;
	sub_82E89AD8(ctx, base);
	// 832A3F90: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A3F94: 386B87D8  addi r3, r11, -0x7828
	ctx.r[3].s64 = ctx.r[11].s64 + -30760;
	// 832A3F98: 4BA05F89  bl 0x82ca9f20
	ctx.lr = 0x832A3F9C;
	sub_82CA9F20(ctx, base);
	// 832A3F9C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A3FA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3FA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3FA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A3FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3FB0 size=68
    let mut pc: u32 = 0x832A3FB0;
    'dispatch: loop {
        match pc {
            0x832A3FB0 => {
    //   block [0x832A3FB0..0x832A3FF4)
	// 832A3FB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A3FB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A3FB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A3FBC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832A3FC0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A3FC4: 388BA1BC  addi r4, r11, -0x5e44
	ctx.r[4].s64 = ctx.r[11].s64 + -24132;
	// 832A3FC8: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A3FCC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A3FD0: 386B76DC  addi r3, r11, 0x76dc
	ctx.r[3].s64 = ctx.r[11].s64 + 30428;
	// 832A3FD4: 4BBDB58D  bl 0x82e7f560
	ctx.lr = 0x832A3FD8;
	sub_82E7F560(ctx, base);
	// 832A3FD8: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A3FDC: 386B87F0  addi r3, r11, -0x7810
	ctx.r[3].s64 = ctx.r[11].s64 + -30736;
	// 832A3FE0: 4BA05F41  bl 0x82ca9f20
	ctx.lr = 0x832A3FE4;
	sub_82CA9F20(ctx, base);
	// 832A3FE4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A3FE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A3FEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A3FF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A3FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A3FF8 size=80
    let mut pc: u32 = 0x832A3FF8;
    'dispatch: loop {
        match pc {
            0x832A3FF8 => {
    //   block [0x832A3FF8..0x832A4048)
	// 832A3FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A3FFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4000: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A4004: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4008: 3D6082EA  lis r11, -0x7d16
	ctx.r[11].s64 = -2098593792;
	// 832A400C: 388AA214  addi r4, r10, -0x5dec
	ctx.r[4].s64 = ctx.r[10].s64 + -24044;
	// 832A4010: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A4014: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A4018: 386A7838  addi r3, r10, 0x7838
	ctx.r[3].s64 = ctx.r[10].s64 + 30776;
	// 832A401C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A4020: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A4024: 38AB43C0  addi r5, r11, 0x43c0
	ctx.r[5].s64 = ctx.r[11].s64 + 17344;
	// 832A4028: 4BBE1799  bl 0x82e857c0
	ctx.lr = 0x832A402C;
	sub_82E857C0(ctx, base);
	// 832A402C: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4030: 386B8808  addi r3, r11, -0x77f8
	ctx.r[3].s64 = ctx.r[11].s64 + -30712;
	// 832A4034: 4BA05EED  bl 0x82ca9f20
	ctx.lr = 0x832A4038;
	sub_82CA9F20(ctx, base);
	// 832A4038: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A403C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4040: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4044: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A4048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4048 size=68
    let mut pc: u32 = 0x832A4048;
    'dispatch: loop {
        match pc {
            0x832A4048 => {
    //   block [0x832A4048..0x832A408C)
	// 832A4048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A404C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4050: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A4054: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4058: 3D6082EB  lis r11, -0x7d15
	ctx.r[11].s64 = -2098528256;
	// 832A405C: 388AA354  addi r4, r10, -0x5cac
	ctx.r[4].s64 = ctx.r[10].s64 + -23724;
	// 832A4060: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4064: 38ABBBC0  addi r5, r11, -0x4440
	ctx.r[5].s64 = ctx.r[11].s64 + -17472;
	// 832A4068: 386A8570  addi r3, r10, -0x7a90
	ctx.r[3].s64 = ctx.r[10].s64 + -31376;
	// 832A406C: 4BC063D5  bl 0x82eaa440
	ctx.lr = 0x832A4070;
	sub_82EAA440(ctx, base);
	// 832A4070: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4074: 386B8838  addi r3, r11, -0x77c8
	ctx.r[3].s64 = ctx.r[11].s64 + -30664;
	// 832A4078: 4BA05EA9  bl 0x82ca9f20
	ctx.lr = 0x832A407C;
	sub_82CA9F20(ctx, base);
	// 832A407C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4080: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4084: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4088: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A4090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A4090 size=12
    let mut pc: u32 = 0x832A4090;
    'dispatch: loop {
        match pc {
            0x832A4090 => {
    //   block [0x832A4090..0x832A409C)
	// 832A4090: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4094: 386B8890  addi r3, r11, -0x7770
	ctx.r[3].s64 = ctx.r[11].s64 + -30576;
	// 832A4098: 4BA05E88  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A40A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A40A0 size=64
    let mut pc: u32 = 0x832A40A0;
    'dispatch: loop {
        match pc {
            0x832A40A0 => {
    //   block [0x832A40A0..0x832A40E0)
	// 832A40A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A40A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A40A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A40AC: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 832A40B0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A40B4: 386B8698  addi r3, r11, -0x7968
	ctx.r[3].s64 = ctx.r[11].s64 + -31080;
	// 832A40B8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A40BC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 832A40C0: 4BBE1101  bl 0x82e851c0
	ctx.lr = 0x832A40C4;
	sub_82E851C0(ctx, base);
	// 832A40C4: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A40C8: 386B8880  addi r3, r11, -0x7780
	ctx.r[3].s64 = ctx.r[11].s64 + -30592;
	// 832A40CC: 4BA05E55  bl 0x82ca9f20
	ctx.lr = 0x832A40D0;
	sub_82CA9F20(ctx, base);
	// 832A40D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A40D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A40D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A40DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A40E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A40E0 size=84
    let mut pc: u32 = 0x832A40E0;
    'dispatch: loop {
        match pc {
            0x832A40E0 => {
    //   block [0x832A40E0..0x832A4134)
	// 832A40E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A40E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A40E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A40EC: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A40F0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A40F4: 38CB1048  addi r6, r11, 0x1048
	ctx.r[6].s64 = ctx.r[11].s64 + 4168;
	// 832A40F8: 388AA684  addi r4, r10, -0x597c
	ctx.r[4].s64 = ctx.r[10].s64 + -22908;
	// 832A40FC: 3D6082EB  lis r11, -0x7d15
	ctx.r[11].s64 = -2098528256;
	// 832A4100: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4104: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 832A4108: 386A8C64  addi r3, r10, -0x739c
	ctx.r[3].s64 = ctx.r[10].s64 + -29596;
	// 832A410C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 832A4110: 38AB1588  addi r5, r11, 0x1588
	ctx.r[5].s64 = ctx.r[11].s64 + 5512;
	// 832A4114: 4BBE166D  bl 0x82e85780
	ctx.lr = 0x832A4118;
	sub_82E85780(ctx, base);
	// 832A4118: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A411C: 386B88C0  addi r3, r11, -0x7740
	ctx.r[3].s64 = ctx.r[11].s64 + -30528;
	// 832A4120: 4BA05E01  bl 0x82ca9f20
	ctx.lr = 0x832A4124;
	sub_82CA9F20(ctx, base);
	// 832A4124: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4128: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A412C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4130: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A4138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4138 size=68
    let mut pc: u32 = 0x832A4138;
    'dispatch: loop {
        match pc {
            0x832A4138 => {
    //   block [0x832A4138..0x832A417C)
	// 832A4138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A413C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4140: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A4144: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4148: 3D6082EB  lis r11, -0x7d15
	ctx.r[11].s64 = -2098528256;
	// 832A414C: 388AA690  addi r4, r10, -0x5970
	ctx.r[4].s64 = ctx.r[10].s64 + -22896;
	// 832A4150: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4154: 38AB1190  addi r5, r11, 0x1190
	ctx.r[5].s64 = ctx.r[11].s64 + 4496;
	// 832A4158: 386A8B58  addi r3, r10, -0x74a8
	ctx.r[3].s64 = ctx.r[10].s64 + -29864;
	// 832A415C: 4B389FAD  bl 0x8262e108
	ctx.lr = 0x832A4160;
	sub_8262E108(ctx, base);
	// 832A4160: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4164: 386B88A8  addi r3, r11, -0x7758
	ctx.r[3].s64 = ctx.r[11].s64 + -30552;
	// 832A4168: 4BA05DB9  bl 0x82ca9f20
	ctx.lr = 0x832A416C;
	sub_82CA9F20(ctx, base);
	// 832A416C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4170: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4174: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4178: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A4180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4180 size=68
    let mut pc: u32 = 0x832A4180;
    'dispatch: loop {
        match pc {
            0x832A4180 => {
    //   block [0x832A4180..0x832A41C4)
	// 832A4180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A4184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4188: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A418C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4190: 3D6082EB  lis r11, -0x7d15
	ctx.r[11].s64 = -2098528256;
	// 832A4194: 388AA6B0  addi r4, r10, -0x5950
	ctx.r[4].s64 = ctx.r[10].s64 + -22864;
	// 832A4198: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A419C: 38AB4270  addi r5, r11, 0x4270
	ctx.r[5].s64 = ctx.r[11].s64 + 17008;
	// 832A41A0: 386A8E84  addi r3, r10, -0x717c
	ctx.r[3].s64 = ctx.r[10].s64 + -29052;
	// 832A41A4: 4B389DCD  bl 0x8262df70
	ctx.lr = 0x832A41A8;
	sub_8262DF70(ctx, base);
	// 832A41A8: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A41AC: 386B8920  addi r3, r11, -0x76e0
	ctx.r[3].s64 = ctx.r[11].s64 + -30432;
	// 832A41B0: 4BA05D71  bl 0x82ca9f20
	ctx.lr = 0x832A41B4;
	sub_82CA9F20(ctx, base);
	// 832A41B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A41B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A41BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A41C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A41C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A41C8 size=68
    let mut pc: u32 = 0x832A41C8;
    'dispatch: loop {
        match pc {
            0x832A41C8 => {
    //   block [0x832A41C8..0x832A420C)
	// 832A41C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A41CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A41D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A41D4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A41D8: 3D6082EB  lis r11, -0x7d15
	ctx.r[11].s64 = -2098528256;
	// 832A41DC: 388AA6C4  addi r4, r10, -0x593c
	ctx.r[4].s64 = ctx.r[10].s64 + -22844;
	// 832A41E0: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A41E4: 38AB4268  addi r5, r11, 0x4268
	ctx.r[5].s64 = ctx.r[11].s64 + 17000;
	// 832A41E8: 386A8D78  addi r3, r10, -0x7288
	ctx.r[3].s64 = ctx.r[10].s64 + -29320;
	// 832A41EC: 4B389E0D  bl 0x8262dff8
	ctx.lr = 0x832A41F0;
	sub_8262DFF8(ctx, base);
	// 832A41F0: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A41F4: 386B8938  addi r3, r11, -0x76c8
	ctx.r[3].s64 = ctx.r[11].s64 + -30408;
	// 832A41F8: 4BA05D29  bl 0x82ca9f20
	ctx.lr = 0x832A41FC;
	sub_82CA9F20(ctx, base);
	// 832A41FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4200: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4204: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4208: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A4210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4210 size=68
    let mut pc: u32 = 0x832A4210;
    'dispatch: loop {
        match pc {
            0x832A4210 => {
    //   block [0x832A4210..0x832A4254)
	// 832A4210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A4214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4218: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A421C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4220: 3D6082EB  lis r11, -0x7d15
	ctx.r[11].s64 = -2098528256;
	// 832A4224: 388AA6D4  addi r4, r10, -0x592c
	ctx.r[4].s64 = ctx.r[10].s64 + -22828;
	// 832A4228: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A422C: 38AB42E0  addi r5, r11, 0x42e0
	ctx.r[5].s64 = ctx.r[11].s64 + 17120;
	// 832A4230: 386A91A8  addi r3, r10, -0x6e58
	ctx.r[3].s64 = ctx.r[10].s64 + -28248;
	// 832A4234: 4B389DC5  bl 0x8262dff8
	ctx.lr = 0x832A4238;
	sub_8262DFF8(ctx, base);
	// 832A4238: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A423C: 386B8950  addi r3, r11, -0x76b0
	ctx.r[3].s64 = ctx.r[11].s64 + -30384;
	// 832A4240: 4BA05CE1  bl 0x82ca9f20
	ctx.lr = 0x832A4244;
	sub_82CA9F20(ctx, base);
	// 832A4244: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4248: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A424C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4250: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A4258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4258 size=68
    let mut pc: u32 = 0x832A4258;
    'dispatch: loop {
        match pc {
            0x832A4258 => {
    //   block [0x832A4258..0x832A429C)
	// 832A4258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A425C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4260: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A4264: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4268: 3D6082EB  lis r11, -0x7d15
	ctx.r[11].s64 = -2098528256;
	// 832A426C: 388AA6EC  addi r4, r10, -0x5914
	ctx.r[4].s64 = ctx.r[10].s64 + -22804;
	// 832A4270: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4274: 38AB4300  addi r5, r11, 0x4300
	ctx.r[5].s64 = ctx.r[11].s64 + 17152;
	// 832A4278: 386A97F0  addi r3, r10, -0x6810
	ctx.r[3].s64 = ctx.r[10].s64 + -26640;
	// 832A427C: 4B389D7D  bl 0x8262dff8
	ctx.lr = 0x832A4280;
	sub_8262DFF8(ctx, base);
	// 832A4280: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4284: 386B8968  addi r3, r11, -0x7698
	ctx.r[3].s64 = ctx.r[11].s64 + -30360;
	// 832A4288: 4BA05C99  bl 0x82ca9f20
	ctx.lr = 0x832A428C;
	sub_82CA9F20(ctx, base);
	// 832A428C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4290: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4294: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4298: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A42A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A42A0 size=68
    let mut pc: u32 = 0x832A42A0;
    'dispatch: loop {
        match pc {
            0x832A42A0 => {
    //   block [0x832A42A0..0x832A42E4)
	// 832A42A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A42A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A42A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A42AC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A42B0: 3D6082EB  lis r11, -0x7d15
	ctx.r[11].s64 = -2098528256;
	// 832A42B4: 388AA700  addi r4, r10, -0x5900
	ctx.r[4].s64 = ctx.r[10].s64 + -22784;
	// 832A42B8: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A42BC: 38AB4338  addi r5, r11, 0x4338
	ctx.r[5].s64 = ctx.r[11].s64 + 17208;
	// 832A42C0: 386A98FC  addi r3, r10, -0x6704
	ctx.r[3].s64 = ctx.r[10].s64 + -26372;
	// 832A42C4: 4B389D35  bl 0x8262dff8
	ctx.lr = 0x832A42C8;
	sub_8262DFF8(ctx, base);
	// 832A42C8: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A42CC: 386B8980  addi r3, r11, -0x7680
	ctx.r[3].s64 = ctx.r[11].s64 + -30336;
	// 832A42D0: 4BA05C51  bl 0x82ca9f20
	ctx.lr = 0x832A42D4;
	sub_82CA9F20(ctx, base);
	// 832A42D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A42D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A42DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A42E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A42E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A42E8 size=68
    let mut pc: u32 = 0x832A42E8;
    'dispatch: loop {
        match pc {
            0x832A42E8 => {
    //   block [0x832A42E8..0x832A432C)
	// 832A42E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A42EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A42F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A42F4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A42F8: 3D6082EB  lis r11, -0x7d15
	ctx.r[11].s64 = -2098528256;
	// 832A42FC: 388AA718  addi r4, r10, -0x58e8
	ctx.r[4].s64 = ctx.r[10].s64 + -22760;
	// 832A4300: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4304: 38AB4DA0  addi r5, r11, 0x4da0
	ctx.r[5].s64 = ctx.r[11].s64 + 19872;
	// 832A4308: 386A8F90  addi r3, r10, -0x7070
	ctx.r[3].s64 = ctx.r[10].s64 + -28784;
	// 832A430C: 4BC10F15  bl 0x82eb5220
	ctx.lr = 0x832A4310;
	sub_82EB5220(ctx, base);
	// 832A4310: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4314: 386B8998  addi r3, r11, -0x7668
	ctx.r[3].s64 = ctx.r[11].s64 + -30312;
	// 832A4318: 4BA05C09  bl 0x82ca9f20
	ctx.lr = 0x832A431C;
	sub_82CA9F20(ctx, base);
	// 832A431C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4320: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4324: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4328: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A4330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4330 size=68
    let mut pc: u32 = 0x832A4330;
    'dispatch: loop {
        match pc {
            0x832A4330 => {
    //   block [0x832A4330..0x832A4374)
	// 832A4330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A4334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4338: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A433C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4340: 3D6082EA  lis r11, -0x7d16
	ctx.r[11].s64 = -2098593792;
	// 832A4344: 388AA724  addi r4, r10, -0x58dc
	ctx.r[4].s64 = ctx.r[10].s64 + -22748;
	// 832A4348: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A434C: 38AB63A8  addi r5, r11, 0x63a8
	ctx.r[5].s64 = ctx.r[11].s64 + 25512;
	// 832A4350: 386A94CC  addi r3, r10, -0x6b34
	ctx.r[3].s64 = ctx.r[10].s64 + -27444;
	// 832A4354: 4BC10ECD  bl 0x82eb5220
	ctx.lr = 0x832A4358;
	sub_82EB5220(ctx, base);
	// 832A4358: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A435C: 386B89B0  addi r3, r11, -0x7650
	ctx.r[3].s64 = ctx.r[11].s64 + -30288;
	// 832A4360: 4BA05BC1  bl 0x82ca9f20
	ctx.lr = 0x832A4364;
	sub_82CA9F20(ctx, base);
	// 832A4364: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4368: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A436C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4370: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A4378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4378 size=68
    let mut pc: u32 = 0x832A4378;
    'dispatch: loop {
        match pc {
            0x832A4378 => {
    //   block [0x832A4378..0x832A43BC)
	// 832A4378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A437C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4380: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A4384: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4388: 3D6082EB  lis r11, -0x7d15
	ctx.r[11].s64 = -2098528256;
	// 832A438C: 388AA730  addi r4, r10, -0x58d0
	ctx.r[4].s64 = ctx.r[10].s64 + -22736;
	// 832A4390: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4394: 38AB4DF0  addi r5, r11, 0x4df0
	ctx.r[5].s64 = ctx.r[11].s64 + 19952;
	// 832A4398: 386A92B4  addi r3, r10, -0x6d4c
	ctx.r[3].s64 = ctx.r[10].s64 + -27980;
	// 832A439C: 4BC10E85  bl 0x82eb5220
	ctx.lr = 0x832A43A0;
	sub_82EB5220(ctx, base);
	// 832A43A0: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A43A4: 386B89C8  addi r3, r11, -0x7638
	ctx.r[3].s64 = ctx.r[11].s64 + -30264;
	// 832A43A8: 4BA05B79  bl 0x82ca9f20
	ctx.lr = 0x832A43AC;
	sub_82CA9F20(ctx, base);
	// 832A43AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A43B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A43B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A43B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A43C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A43C0 size=68
    let mut pc: u32 = 0x832A43C0;
    'dispatch: loop {
        match pc {
            0x832A43C0 => {
    //   block [0x832A43C0..0x832A4404)
	// 832A43C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A43C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A43C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A43CC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A43D0: 3D6082EB  lis r11, -0x7d15
	ctx.r[11].s64 = -2098528256;
	// 832A43D4: 388AA748  addi r4, r10, -0x58b8
	ctx.r[4].s64 = ctx.r[10].s64 + -22712;
	// 832A43D8: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A43DC: 38AB4370  addi r5, r11, 0x4370
	ctx.r[5].s64 = ctx.r[11].s64 + 17264;
	// 832A43E0: 386A9B14  addi r3, r10, -0x64ec
	ctx.r[3].s64 = ctx.r[10].s64 + -25836;
	// 832A43E4: 4B389C9D  bl 0x8262e080
	ctx.lr = 0x832A43E8;
	sub_8262E080(ctx, base);
	// 832A43E8: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A43EC: 386B89E0  addi r3, r11, -0x7620
	ctx.r[3].s64 = ctx.r[11].s64 + -30240;
	// 832A43F0: 4BA05B31  bl 0x82ca9f20
	ctx.lr = 0x832A43F4;
	sub_82CA9F20(ctx, base);
	// 832A43F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A43F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A43FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4400: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A4408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4408 size=68
    let mut pc: u32 = 0x832A4408;
    'dispatch: loop {
        match pc {
            0x832A4408 => {
    //   block [0x832A4408..0x832A444C)
	// 832A4408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A440C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4410: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A4414: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4418: 3D6082EB  lis r11, -0x7d15
	ctx.r[11].s64 = -2098528256;
	// 832A441C: 388AA754  addi r4, r10, -0x58ac
	ctx.r[4].s64 = ctx.r[10].s64 + -22700;
	// 832A4420: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4424: 38AB4530  addi r5, r11, 0x4530
	ctx.r[5].s64 = ctx.r[11].s64 + 17712;
	// 832A4428: 386A9A08  addi r3, r10, -0x65f8
	ctx.r[3].s64 = ctx.r[10].s64 + -26104;
	// 832A442C: 4B389C55  bl 0x8262e080
	ctx.lr = 0x832A4430;
	sub_8262E080(ctx, base);
	// 832A4430: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4434: 386B89F8  addi r3, r11, -0x7608
	ctx.r[3].s64 = ctx.r[11].s64 + -30216;
	// 832A4438: 4BA05AE9  bl 0x82ca9f20
	ctx.lr = 0x832A443C;
	sub_82CA9F20(ctx, base);
	// 832A443C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4440: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4444: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4448: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A4450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4450 size=68
    let mut pc: u32 = 0x832A4450;
    'dispatch: loop {
        match pc {
            0x832A4450 => {
    //   block [0x832A4450..0x832A4494)
	// 832A4450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A4454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4458: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A445C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4460: 3D6082EB  lis r11, -0x7d15
	ctx.r[11].s64 = -2098528256;
	// 832A4464: 388AA764  addi r4, r10, -0x589c
	ctx.r[4].s64 = ctx.r[10].s64 + -22684;
	// 832A4468: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A446C: 38AB5028  addi r5, r11, 0x5028
	ctx.r[5].s64 = ctx.r[11].s64 + 20520;
	// 832A4470: 386A93C0  addi r3, r10, -0x6c40
	ctx.r[3].s64 = ctx.r[10].s64 + -27712;
	// 832A4474: 4BC10E35  bl 0x82eb52a8
	ctx.lr = 0x832A4478;
	sub_82EB52A8(ctx, base);
	// 832A4478: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A447C: 386B8A10  addi r3, r11, -0x75f0
	ctx.r[3].s64 = ctx.r[11].s64 + -30192;
	// 832A4480: 4BA05AA1  bl 0x82ca9f20
	ctx.lr = 0x832A4484;
	sub_82CA9F20(ctx, base);
	// 832A4484: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4488: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A448C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4490: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A4498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4498 size=68
    let mut pc: u32 = 0x832A4498;
    'dispatch: loop {
        match pc {
            0x832A4498 => {
    //   block [0x832A4498..0x832A44DC)
	// 832A4498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A449C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A44A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A44A4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A44A8: 3D6082EB  lis r11, -0x7d15
	ctx.r[11].s64 = -2098528256;
	// 832A44AC: 388AA774  addi r4, r10, -0x588c
	ctx.r[4].s64 = ctx.r[10].s64 + -22668;
	// 832A44B0: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A44B4: 38AB4E48  addi r5, r11, 0x4e48
	ctx.r[5].s64 = ctx.r[11].s64 + 20040;
	// 832A44B8: 386A9D2C  addi r3, r10, -0x62d4
	ctx.r[3].s64 = ctx.r[10].s64 + -25300;
	// 832A44BC: 4BC10DED  bl 0x82eb52a8
	ctx.lr = 0x832A44C0;
	sub_82EB52A8(ctx, base);
	// 832A44C0: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A44C4: 386B8A28  addi r3, r11, -0x75d8
	ctx.r[3].s64 = ctx.r[11].s64 + -30168;
	// 832A44C8: 4BA05A59  bl 0x82ca9f20
	ctx.lr = 0x832A44CC;
	sub_82CA9F20(ctx, base);
	// 832A44CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A44D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A44D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A44D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A44E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A44E0 size=68
    let mut pc: u32 = 0x832A44E0;
    'dispatch: loop {
        match pc {
            0x832A44E0 => {
    //   block [0x832A44E0..0x832A4524)
	// 832A44E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A44E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A44E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A44EC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A44F0: 3D6082EA  lis r11, -0x7d16
	ctx.r[11].s64 = -2098593792;
	// 832A44F4: 388AA780  addi r4, r10, -0x5880
	ctx.r[4].s64 = ctx.r[10].s64 + -22656;
	// 832A44F8: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A44FC: 38AB63A8  addi r5, r11, 0x63a8
	ctx.r[5].s64 = ctx.r[11].s64 + 25512;
	// 832A4500: 386A95D8  addi r3, r10, -0x6a28
	ctx.r[3].s64 = ctx.r[10].s64 + -27176;
	// 832A4504: 4BC10DA5  bl 0x82eb52a8
	ctx.lr = 0x832A4508;
	sub_82EB52A8(ctx, base);
	// 832A4508: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A450C: 386B8A40  addi r3, r11, -0x75c0
	ctx.r[3].s64 = ctx.r[11].s64 + -30144;
	// 832A4510: 4BA05A11  bl 0x82ca9f20
	ctx.lr = 0x832A4514;
	sub_82CA9F20(ctx, base);
	// 832A4514: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4518: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A451C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4520: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A4528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4528 size=68
    let mut pc: u32 = 0x832A4528;
    'dispatch: loop {
        match pc {
            0x832A4528 => {
    //   block [0x832A4528..0x832A456C)
	// 832A4528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A452C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4530: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A4534: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4538: 3D6082EB  lis r11, -0x7d15
	ctx.r[11].s64 = -2098528256;
	// 832A453C: 388AA788  addi r4, r10, -0x5878
	ctx.r[4].s64 = ctx.r[10].s64 + -22648;
	// 832A4540: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4544: 38AB4BA0  addi r5, r11, 0x4ba0
	ctx.r[5].s64 = ctx.r[11].s64 + 19360;
	// 832A4548: 386A909C  addi r3, r10, -0x6f64
	ctx.r[3].s64 = ctx.r[10].s64 + -28516;
	// 832A454C: 4B389C45  bl 0x8262e190
	ctx.lr = 0x832A4550;
	sub_8262E190(ctx, base);
	// 832A4550: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4554: 386B8A58  addi r3, r11, -0x75a8
	ctx.r[3].s64 = ctx.r[11].s64 + -30120;
	// 832A4558: 4BA059C9  bl 0x82ca9f20
	ctx.lr = 0x832A455C;
	sub_82CA9F20(ctx, base);
	// 832A455C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4560: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4564: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4568: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A4570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4570 size=68
    let mut pc: u32 = 0x832A4570;
    'dispatch: loop {
        match pc {
            0x832A4570 => {
    //   block [0x832A4570..0x832A45B4)
	// 832A4570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A4574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4578: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A457C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4580: 3D6082EB  lis r11, -0x7d15
	ctx.r[11].s64 = -2098528256;
	// 832A4584: 388AA794  addi r4, r10, -0x586c
	ctx.r[4].s64 = ctx.r[10].s64 + -22636;
	// 832A4588: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A458C: 38AB4CD8  addi r5, r11, 0x4cd8
	ctx.r[5].s64 = ctx.r[11].s64 + 19672;
	// 832A4590: 386A96E4  addi r3, r10, -0x691c
	ctx.r[3].s64 = ctx.r[10].s64 + -26908;
	// 832A4594: 4B389BFD  bl 0x8262e190
	ctx.lr = 0x832A4598;
	sub_8262E190(ctx, base);
	// 832A4598: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A459C: 386B8A70  addi r3, r11, -0x7590
	ctx.r[3].s64 = ctx.r[11].s64 + -30096;
	// 832A45A0: 4BA05981  bl 0x82ca9f20
	ctx.lr = 0x832A45A4;
	sub_82CA9F20(ctx, base);
	// 832A45A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A45A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A45AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A45B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A45B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A45B8 size=68
    let mut pc: u32 = 0x832A45B8;
    'dispatch: loop {
        match pc {
            0x832A45B8 => {
    //   block [0x832A45B8..0x832A45FC)
	// 832A45B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A45BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A45C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A45C4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A45C8: 3D6082EB  lis r11, -0x7d15
	ctx.r[11].s64 = -2098528256;
	// 832A45CC: 388AA7A8  addi r4, r10, -0x5858
	ctx.r[4].s64 = ctx.r[10].s64 + -22616;
	// 832A45D0: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A45D4: 38AB4D30  addi r5, r11, 0x4d30
	ctx.r[5].s64 = ctx.r[11].s64 + 19760;
	// 832A45D8: 386A9C20  addi r3, r10, -0x63e0
	ctx.r[3].s64 = ctx.r[10].s64 + -25568;
	// 832A45DC: 4B389BB5  bl 0x8262e190
	ctx.lr = 0x832A45E0;
	sub_8262E190(ctx, base);
	// 832A45E0: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A45E4: 386B8A88  addi r3, r11, -0x7578
	ctx.r[3].s64 = ctx.r[11].s64 + -30072;
	// 832A45E8: 4BA05939  bl 0x82ca9f20
	ctx.lr = 0x832A45EC;
	sub_82CA9F20(ctx, base);
	// 832A45EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A45F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A45F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A45F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A4600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4600 size=84
    let mut pc: u32 = 0x832A4600;
    'dispatch: loop {
        match pc {
            0x832A4600 => {
    //   block [0x832A4600..0x832A4654)
	// 832A4600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A4604: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4608: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A460C: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A4610: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4614: 38CB1048  addi r6, r11, 0x1048
	ctx.r[6].s64 = ctx.r[11].s64 + 4168;
	// 832A4618: 388AACF0  addi r4, r10, -0x5310
	ctx.r[4].s64 = ctx.r[10].s64 + -21264;
	// 832A461C: 3D6082EC  lis r11, -0x7d14
	ctx.r[11].s64 = -2098462720;
	// 832A4620: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4624: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 832A4628: 386A9E5C  addi r3, r10, -0x61a4
	ctx.r[3].s64 = ctx.r[10].s64 + -24996;
	// 832A462C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 832A4630: 38ABBF80  addi r5, r11, -0x4080
	ctx.r[5].s64 = ctx.r[11].s64 + -16512;
	// 832A4634: 4BBE114D  bl 0x82e85780
	ctx.lr = 0x832A4638;
	sub_82E85780(ctx, base);
	// 832A4638: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A463C: 386B8AA0  addi r3, r11, -0x7560
	ctx.r[3].s64 = ctx.r[11].s64 + -30048;
	// 832A4640: 4BA058E1  bl 0x82ca9f20
	ctx.lr = 0x832A4644;
	sub_82CA9F20(ctx, base);
	// 832A4644: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4648: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A464C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4650: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A4658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4658 size=72
    let mut pc: u32 = 0x832A4658;
    'dispatch: loop {
        match pc {
            0x832A4658 => {
    //   block [0x832A4658..0x832A46A0)
	// 832A4658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A465C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4660: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A4664: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4668: 3D6082EC  lis r11, -0x7d14
	ctx.r[11].s64 = -2098462720;
	// 832A466C: 388AAD0C  addi r4, r10, -0x52f4
	ctx.r[4].s64 = ctx.r[10].s64 + -21236;
	// 832A4670: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4674: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A4678: 386ACBC8  addi r3, r10, -0x3438
	ctx.r[3].s64 = ctx.r[10].s64 + -13368;
	// 832A467C: 38ABDCB0  addi r5, r11, -0x2350
	ctx.r[5].s64 = ctx.r[11].s64 + -9040;
	// 832A4680: 4BC21011  bl 0x82ec5690
	ctx.lr = 0x832A4684;
	sub_82EC5690(ctx, base);
	// 832A4684: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4688: 386B8BC0  addi r3, r11, -0x7440
	ctx.r[3].s64 = ctx.r[11].s64 + -29760;
	// 832A468C: 4BA05895  bl 0x82ca9f20
	ctx.lr = 0x832A4690;
	sub_82CA9F20(ctx, base);
	// 832A4690: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4694: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4698: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A469C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A46A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A46A0 size=72
    let mut pc: u32 = 0x832A46A0;
    'dispatch: loop {
        match pc {
            0x832A46A0 => {
    //   block [0x832A46A0..0x832A46E8)
	// 832A46A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A46A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A46A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A46AC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A46B0: 3D6082EC  lis r11, -0x7d14
	ctx.r[11].s64 = -2098462720;
	// 832A46B4: 388AAD58  addi r4, r10, -0x52a8
	ctx.r[4].s64 = ctx.r[10].s64 + -21160;
	// 832A46B8: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A46BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A46C0: 386ACCDC  addi r3, r10, -0x3324
	ctx.r[3].s64 = ctx.r[10].s64 + -13092;
	// 832A46C4: 38ABDFB8  addi r5, r11, -0x2048
	ctx.r[5].s64 = ctx.r[11].s64 + -8264;
	// 832A46C8: 4BC211A1  bl 0x82ec5868
	ctx.lr = 0x832A46CC;
	sub_82EC5868(ctx, base);
	// 832A46CC: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A46D0: 386B8BD8  addi r3, r11, -0x7428
	ctx.r[3].s64 = ctx.r[11].s64 + -29736;
	// 832A46D4: 4BA0584D  bl 0x82ca9f20
	ctx.lr = 0x832A46D8;
	sub_82CA9F20(ctx, base);
	// 832A46D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A46DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A46E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A46E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A46E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A46E8 size=72
    let mut pc: u32 = 0x832A46E8;
    'dispatch: loop {
        match pc {
            0x832A46E8 => {
    //   block [0x832A46E8..0x832A4730)
	// 832A46E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A46EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A46F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A46F4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A46F8: 3D6082EC  lis r11, -0x7d14
	ctx.r[11].s64 = -2098462720;
	// 832A46FC: 388AAD88  addi r4, r10, -0x5278
	ctx.r[4].s64 = ctx.r[10].s64 + -21112;
	// 832A4700: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4704: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A4708: 386ACDF0  addi r3, r10, -0x3210
	ctx.r[3].s64 = ctx.r[10].s64 + -12816;
	// 832A470C: 38ABE1C0  addi r5, r11, -0x1e40
	ctx.r[5].s64 = ctx.r[11].s64 + -7744;
	// 832A4710: 4BC21159  bl 0x82ec5868
	ctx.lr = 0x832A4714;
	sub_82EC5868(ctx, base);
	// 832A4714: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4718: 386B8BF0  addi r3, r11, -0x7410
	ctx.r[3].s64 = ctx.r[11].s64 + -29712;
	// 832A471C: 4BA05805  bl 0x82ca9f20
	ctx.lr = 0x832A4720;
	sub_82CA9F20(ctx, base);
	// 832A4720: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4724: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4728: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A472C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A4730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4730 size=72
    let mut pc: u32 = 0x832A4730;
    'dispatch: loop {
        match pc {
            0x832A4730 => {
    //   block [0x832A4730..0x832A4778)
	// 832A4730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A4734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4738: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A473C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4740: 3D6082EC  lis r11, -0x7d14
	ctx.r[11].s64 = -2098462720;
	// 832A4744: 388AADC0  addi r4, r10, -0x5240
	ctx.r[4].s64 = ctx.r[10].s64 + -21056;
	// 832A4748: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A474C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A4750: 386ACF04  addi r3, r10, -0x30fc
	ctx.r[3].s64 = ctx.r[10].s64 + -12540;
	// 832A4754: 38ABE6E0  addi r5, r11, -0x1920
	ctx.r[5].s64 = ctx.r[11].s64 + -6432;
	// 832A4758: 4BC212E9  bl 0x82ec5a40
	ctx.lr = 0x832A475C;
	sub_82EC5A40(ctx, base);
	// 832A475C: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4760: 386B8C08  addi r3, r11, -0x73f8
	ctx.r[3].s64 = ctx.r[11].s64 + -29688;
	// 832A4764: 4BA057BD  bl 0x82ca9f20
	ctx.lr = 0x832A4768;
	sub_82CA9F20(ctx, base);
	// 832A4768: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A476C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4770: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4774: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A4778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4778 size=72
    let mut pc: u32 = 0x832A4778;
    'dispatch: loop {
        match pc {
            0x832A4778 => {
    //   block [0x832A4778..0x832A47C0)
	// 832A4778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A477C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4780: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A4784: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4788: 3D6082EC  lis r11, -0x7d14
	ctx.r[11].s64 = -2098462720;
	// 832A478C: 388AAE6C  addi r4, r10, -0x5194
	ctx.r[4].s64 = ctx.r[10].s64 + -20884;
	// 832A4790: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4794: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A4798: 386AD018  addi r3, r10, -0x2fe8
	ctx.r[3].s64 = ctx.r[10].s64 + -12264;
	// 832A479C: 38ABEC60  addi r5, r11, -0x13a0
	ctx.r[5].s64 = ctx.r[11].s64 + -5024;
	// 832A47A0: 4BC212A1  bl 0x82ec5a40
	ctx.lr = 0x832A47A4;
	sub_82EC5A40(ctx, base);
	// 832A47A4: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A47A8: 386B8C20  addi r3, r11, -0x73e0
	ctx.r[3].s64 = ctx.r[11].s64 + -29664;
	// 832A47AC: 4BA05775  bl 0x82ca9f20
	ctx.lr = 0x832A47B0;
	sub_82CA9F20(ctx, base);
	// 832A47B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A47B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A47B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A47BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A47C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A47C0 size=72
    let mut pc: u32 = 0x832A47C0;
    'dispatch: loop {
        match pc {
            0x832A47C0 => {
    //   block [0x832A47C0..0x832A4808)
	// 832A47C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A47C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A47C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A47CC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A47D0: 3D6082EC  lis r11, -0x7d14
	ctx.r[11].s64 = -2098462720;
	// 832A47D4: 388AAEC8  addi r4, r10, -0x5138
	ctx.r[4].s64 = ctx.r[10].s64 + -20792;
	// 832A47D8: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A47DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A47E0: 386AD12C  addi r3, r10, -0x2ed4
	ctx.r[3].s64 = ctx.r[10].s64 + -11988;
	// 832A47E4: 38ABF928  addi r5, r11, -0x6d8
	ctx.r[5].s64 = ctx.r[11].s64 + -1752;
	// 832A47E8: 4BC21431  bl 0x82ec5c18
	ctx.lr = 0x832A47EC;
	sub_82EC5C18(ctx, base);
	// 832A47EC: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A47F0: 386B8C38  addi r3, r11, -0x73c8
	ctx.r[3].s64 = ctx.r[11].s64 + -29640;
	// 832A47F4: 4BA0572D  bl 0x82ca9f20
	ctx.lr = 0x832A47F8;
	sub_82CA9F20(ctx, base);
	// 832A47F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A47FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4800: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4804: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A4808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4808 size=72
    let mut pc: u32 = 0x832A4808;
    'dispatch: loop {
        match pc {
            0x832A4808 => {
    //   block [0x832A4808..0x832A4850)
	// 832A4808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A480C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4810: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A4814: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4818: 3D6082EC  lis r11, -0x7d14
	ctx.r[11].s64 = -2098462720;
	// 832A481C: 388AAF24  addi r4, r10, -0x50dc
	ctx.r[4].s64 = ctx.r[10].s64 + -20700;
	// 832A4820: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4824: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A4828: 386AD244  addi r3, r10, -0x2dbc
	ctx.r[3].s64 = ctx.r[10].s64 + -11708;
	// 832A482C: 38ABFDF8  addi r5, r11, -0x208
	ctx.r[5].s64 = ctx.r[11].s64 + -520;
	// 832A4830: 4BC213E9  bl 0x82ec5c18
	ctx.lr = 0x832A4834;
	sub_82EC5C18(ctx, base);
	// 832A4834: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4838: 386B8C50  addi r3, r11, -0x73b0
	ctx.r[3].s64 = ctx.r[11].s64 + -29616;
	// 832A483C: 4BA056E5  bl 0x82ca9f20
	ctx.lr = 0x832A4840;
	sub_82CA9F20(ctx, base);
	// 832A4840: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4844: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4848: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A484C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A4850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x832A4850 size=56
    let mut pc: u32 = 0x832A4850;
    'dispatch: loop {
        match pc {
            0x832A4850 => {
    //   block [0x832A4850..0x832A4868)
	// 832A4850: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A4854: 816B0718  lwz r11, 0x718(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1816 as u32) ) } as u64;
	// 832A4858: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 832A485C: 4182000C  beq 0x832a4868
	if ctx.cr[0].eq {
	pc = 0x832A4868; continue 'dispatch;
	}
	// 832A4860: C1AB000C  lfs f13, 0xc(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 832A4864: 4800000C  b 0x832a4870
	pc = 0x832A4870; continue 'dispatch;
            }
            0x832A4868 => {
    //   block [0x832A4868..0x832A4870)
	// 832A4868: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832A486C: C1AB0C14  lfs f13, 0xc14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	pc = 0x832A4870; continue 'dispatch;
            }
            0x832A4870 => {
    //   block [0x832A4870..0x832A4888)
	// 832A4870: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832A4874: C00B0BE8  lfs f0, 0xbe8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3048 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832A4878: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 832A487C: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 832A4880: D00BD240  stfs f0, -0x2dc0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-11712 as u32), tmp.u32 ) };
	// 832A4884: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A4888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4888 size=72
    let mut pc: u32 = 0x832A4888;
    'dispatch: loop {
        match pc {
            0x832A4888 => {
    //   block [0x832A4888..0x832A48D0)
	// 832A4888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A488C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4890: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A4894: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4898: 3D6082EC  lis r11, -0x7d14
	ctx.r[11].s64 = -2098462720;
	// 832A489C: 388AAF5C  addi r4, r10, -0x50a4
	ctx.r[4].s64 = ctx.r[10].s64 + -20644;
	// 832A48A0: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A48A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A48A8: 386AD358  addi r3, r10, -0x2ca8
	ctx.r[3].s64 = ctx.r[10].s64 + -11432;
	// 832A48AC: 38AB0B38  addi r5, r11, 0xb38
	ctx.r[5].s64 = ctx.r[11].s64 + 2872;
	// 832A48B0: 4BC21369  bl 0x82ec5c18
	ctx.lr = 0x832A48B4;
	sub_82EC5C18(ctx, base);
	// 832A48B4: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A48B8: 386B8C68  addi r3, r11, -0x7398
	ctx.r[3].s64 = ctx.r[11].s64 + -29592;
	// 832A48BC: 4BA05665  bl 0x82ca9f20
	ctx.lr = 0x832A48C0;
	sub_82CA9F20(ctx, base);
	// 832A48C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A48C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A48C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A48CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A48D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A48D0 size=72
    let mut pc: u32 = 0x832A48D0;
    'dispatch: loop {
        match pc {
            0x832A48D0 => {
    //   block [0x832A48D0..0x832A4918)
	// 832A48D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A48D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A48D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A48DC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A48E0: 3D6082EC  lis r11, -0x7d14
	ctx.r[11].s64 = -2098462720;
	// 832A48E4: 388AAFA0  addi r4, r10, -0x5060
	ctx.r[4].s64 = ctx.r[10].s64 + -20576;
	// 832A48E8: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A48EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A48F0: 386AD47C  addi r3, r10, -0x2b84
	ctx.r[3].s64 = ctx.r[10].s64 + -11140;
	// 832A48F4: 38AB0BC8  addi r5, r11, 0xbc8
	ctx.r[5].s64 = ctx.r[11].s64 + 3016;
	// 832A48F8: 4BC214F9  bl 0x82ec5df0
	ctx.lr = 0x832A48FC;
	sub_82EC5DF0(ctx, base);
	// 832A48FC: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4900: 386B8C80  addi r3, r11, -0x7380
	ctx.r[3].s64 = ctx.r[11].s64 + -29568;
	// 832A4904: 4BA0561D  bl 0x82ca9f20
	ctx.lr = 0x832A4908;
	sub_82CA9F20(ctx, base);
	// 832A4908: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A490C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4910: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4914: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A4918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4918 size=72
    let mut pc: u32 = 0x832A4918;
    'dispatch: loop {
        match pc {
            0x832A4918 => {
    //   block [0x832A4918..0x832A4960)
	// 832A4918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A491C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4920: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A4924: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4928: 3D6082EC  lis r11, -0x7d14
	ctx.r[11].s64 = -2098462720;
	// 832A492C: 388AAFC4  addi r4, r10, -0x503c
	ctx.r[4].s64 = ctx.r[10].s64 + -20540;
	// 832A4930: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4934: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A4938: 386AD590  addi r3, r10, -0x2a70
	ctx.r[3].s64 = ctx.r[10].s64 + -10864;
	// 832A493C: 38AB0D28  addi r5, r11, 0xd28
	ctx.r[5].s64 = ctx.r[11].s64 + 3368;
	// 832A4940: 4BC214B1  bl 0x82ec5df0
	ctx.lr = 0x832A4944;
	sub_82EC5DF0(ctx, base);
	// 832A4944: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4948: 386B8C98  addi r3, r11, -0x7368
	ctx.r[3].s64 = ctx.r[11].s64 + -29544;
	// 832A494C: 4BA055D5  bl 0x82ca9f20
	ctx.lr = 0x832A4950;
	sub_82CA9F20(ctx, base);
	// 832A4950: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4954: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4958: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A495C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A4960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4960 size=72
    let mut pc: u32 = 0x832A4960;
    'dispatch: loop {
        match pc {
            0x832A4960 => {
    //   block [0x832A4960..0x832A49A8)
	// 832A4960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A4964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4968: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A496C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4970: 3D6082EC  lis r11, -0x7d14
	ctx.r[11].s64 = -2098462720;
	// 832A4974: 388AAFD8  addi r4, r10, -0x5028
	ctx.r[4].s64 = ctx.r[10].s64 + -20520;
	// 832A4978: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A497C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A4980: 386AD6A4  addi r3, r10, -0x295c
	ctx.r[3].s64 = ctx.r[10].s64 + -10588;
	// 832A4984: 38AB1640  addi r5, r11, 0x1640
	ctx.r[5].s64 = ctx.r[11].s64 + 5696;
	// 832A4988: 4BC21641  bl 0x82ec5fc8
	ctx.lr = 0x832A498C;
	sub_82EC5FC8(ctx, base);
	// 832A498C: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4990: 386B8CB0  addi r3, r11, -0x7350
	ctx.r[3].s64 = ctx.r[11].s64 + -29520;
	// 832A4994: 4BA0558D  bl 0x82ca9f20
	ctx.lr = 0x832A4998;
	sub_82CA9F20(ctx, base);
	// 832A4998: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A499C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A49A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A49A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A49A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A49A8 size=72
    let mut pc: u32 = 0x832A49A8;
    'dispatch: loop {
        match pc {
            0x832A49A8 => {
    //   block [0x832A49A8..0x832A49F0)
	// 832A49A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A49AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A49B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A49B4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A49B8: 3D6082EC  lis r11, -0x7d14
	ctx.r[11].s64 = -2098462720;
	// 832A49BC: 388AB018  addi r4, r10, -0x4fe8
	ctx.r[4].s64 = ctx.r[10].s64 + -20456;
	// 832A49C0: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A49C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A49C8: 386AD7B8  addi r3, r10, -0x2848
	ctx.r[3].s64 = ctx.r[10].s64 + -10312;
	// 832A49CC: 38AB1960  addi r5, r11, 0x1960
	ctx.r[5].s64 = ctx.r[11].s64 + 6496;
	// 832A49D0: 4BC215F9  bl 0x82ec5fc8
	ctx.lr = 0x832A49D4;
	sub_82EC5FC8(ctx, base);
	// 832A49D4: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A49D8: 386B8CC8  addi r3, r11, -0x7338
	ctx.r[3].s64 = ctx.r[11].s64 + -29496;
	// 832A49DC: 4BA05545  bl 0x82ca9f20
	ctx.lr = 0x832A49E0;
	sub_82CA9F20(ctx, base);
	// 832A49E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A49E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A49E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A49EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A49F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A49F0 size=72
    let mut pc: u32 = 0x832A49F0;
    'dispatch: loop {
        match pc {
            0x832A49F0 => {
    //   block [0x832A49F0..0x832A4A38)
	// 832A49F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A49F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A49F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A49FC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4A00: 3D6082EC  lis r11, -0x7d14
	ctx.r[11].s64 = -2098462720;
	// 832A4A04: 388AB068  addi r4, r10, -0x4f98
	ctx.r[4].s64 = ctx.r[10].s64 + -20376;
	// 832A4A08: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4A0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A4A10: 386AD8CC  addi r3, r10, -0x2734
	ctx.r[3].s64 = ctx.r[10].s64 + -10036;
	// 832A4A14: 38AB1B00  addi r5, r11, 0x1b00
	ctx.r[5].s64 = ctx.r[11].s64 + 6912;
	// 832A4A18: 4BC21789  bl 0x82ec61a0
	ctx.lr = 0x832A4A1C;
	sub_82EC61A0(ctx, base);
	// 832A4A1C: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4A20: 386B8CE0  addi r3, r11, -0x7320
	ctx.r[3].s64 = ctx.r[11].s64 + -29472;
	// 832A4A24: 4BA054FD  bl 0x82ca9f20
	ctx.lr = 0x832A4A28;
	sub_82CA9F20(ctx, base);
	// 832A4A28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4A2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4A30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4A34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A4A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4A38 size=72
    let mut pc: u32 = 0x832A4A38;
    'dispatch: loop {
        match pc {
            0x832A4A38 => {
    //   block [0x832A4A38..0x832A4A80)
	// 832A4A38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A4A3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4A40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A4A44: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4A48: 3D6082EC  lis r11, -0x7d14
	ctx.r[11].s64 = -2098462720;
	// 832A4A4C: 388AB0A4  addi r4, r10, -0x4f5c
	ctx.r[4].s64 = ctx.r[10].s64 + -20316;
	// 832A4A50: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4A54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A4A58: 386AD9E0  addi r3, r10, -0x2620
	ctx.r[3].s64 = ctx.r[10].s64 + -9760;
	// 832A4A5C: 38AB1F28  addi r5, r11, 0x1f28
	ctx.r[5].s64 = ctx.r[11].s64 + 7976;
	// 832A4A60: 4BC21919  bl 0x82ec6378
	ctx.lr = 0x832A4A64;
	sub_82EC6378(ctx, base);
	// 832A4A64: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4A68: 386B8CF8  addi r3, r11, -0x7308
	ctx.r[3].s64 = ctx.r[11].s64 + -29448;
	// 832A4A6C: 4BA054B5  bl 0x82ca9f20
	ctx.lr = 0x832A4A70;
	sub_82CA9F20(ctx, base);
	// 832A4A70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4A74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4A78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4A7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A4A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4A80 size=72
    let mut pc: u32 = 0x832A4A80;
    'dispatch: loop {
        match pc {
            0x832A4A80 => {
    //   block [0x832A4A80..0x832A4AC8)
	// 832A4A80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A4A84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4A88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A4A8C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4A90: 3D6082EC  lis r11, -0x7d14
	ctx.r[11].s64 = -2098462720;
	// 832A4A94: 388AB0C4  addi r4, r10, -0x4f3c
	ctx.r[4].s64 = ctx.r[10].s64 + -20284;
	// 832A4A98: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4A9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A4AA0: 386ADAF4  addi r3, r10, -0x250c
	ctx.r[3].s64 = ctx.r[10].s64 + -9484;
	// 832A4AA4: 38AB2348  addi r5, r11, 0x2348
	ctx.r[5].s64 = ctx.r[11].s64 + 9032;
	// 832A4AA8: 4BC21AA9  bl 0x82ec6550
	ctx.lr = 0x832A4AAC;
	sub_82EC6550(ctx, base);
	// 832A4AAC: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4AB0: 386B8D10  addi r3, r11, -0x72f0
	ctx.r[3].s64 = ctx.r[11].s64 + -29424;
	// 832A4AB4: 4BA0546D  bl 0x82ca9f20
	ctx.lr = 0x832A4AB8;
	sub_82CA9F20(ctx, base);
	// 832A4AB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4ABC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4AC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4AC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A4AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4AC8 size=76
    let mut pc: u32 = 0x832A4AC8;
    'dispatch: loop {
        match pc {
            0x832A4AC8 => {
    //   block [0x832A4AC8..0x832A4B14)
	// 832A4AC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A4ACC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4AD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A4AD4: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 832A4AD8: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4ADC: 38CBDD1C  addi r6, r11, -0x22e4
	ctx.r[6].s64 = ctx.r[11].s64 + -8932;
	// 832A4AE0: 388AB100  addi r4, r10, -0x4f00
	ctx.r[4].s64 = ctx.r[10].s64 + -20224;
	// 832A4AE4: 3D6082EC  lis r11, -0x7d14
	ctx.r[11].s64 = -2098462720;
	// 832A4AE8: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4AEC: 38AB2428  addi r5, r11, 0x2428
	ctx.r[5].s64 = ctx.r[11].s64 + 9256;
	// 832A4AF0: 386ADC08  addi r3, r10, -0x23f8
	ctx.r[3].s64 = ctx.r[10].s64 + -9208;
	// 832A4AF4: 4BC21C35  bl 0x82ec6728
	ctx.lr = 0x832A4AF8;
	sub_82EC6728(ctx, base);
	// 832A4AF8: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4AFC: 386B8D28  addi r3, r11, -0x72d8
	ctx.r[3].s64 = ctx.r[11].s64 + -29400;
	// 832A4B00: 4BA05421  bl 0x82ca9f20
	ctx.lr = 0x832A4B04;
	sub_82CA9F20(ctx, base);
	// 832A4B04: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4B08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4B0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4B10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A4B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4B18 size=72
    let mut pc: u32 = 0x832A4B18;
    'dispatch: loop {
        match pc {
            0x832A4B18 => {
    //   block [0x832A4B18..0x832A4B60)
	// 832A4B18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A4B1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4B20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A4B24: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4B28: 3D6082EC  lis r11, -0x7d14
	ctx.r[11].s64 = -2098462720;
	// 832A4B2C: 388AB124  addi r4, r10, -0x4edc
	ctx.r[4].s64 = ctx.r[10].s64 + -20188;
	// 832A4B30: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4B34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A4B38: 386ADD1C  addi r3, r10, -0x22e4
	ctx.r[3].s64 = ctx.r[10].s64 + -8932;
	// 832A4B3C: 38AB2708  addi r5, r11, 0x2708
	ctx.r[5].s64 = ctx.r[11].s64 + 9992;
	// 832A4B40: 4BC21BE9  bl 0x82ec6728
	ctx.lr = 0x832A4B44;
	sub_82EC6728(ctx, base);
	// 832A4B44: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4B48: 386B8D40  addi r3, r11, -0x72c0
	ctx.r[3].s64 = ctx.r[11].s64 + -29376;
	// 832A4B4C: 4BA053D5  bl 0x82ca9f20
	ctx.lr = 0x832A4B50;
	sub_82CA9F20(ctx, base);
	// 832A4B50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4B54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4B58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4B5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A4B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4B60 size=72
    let mut pc: u32 = 0x832A4B60;
    'dispatch: loop {
        match pc {
            0x832A4B60 => {
    //   block [0x832A4B60..0x832A4BA8)
	// 832A4B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A4B64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4B68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A4B6C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4B70: 3D6082EC  lis r11, -0x7d14
	ctx.r[11].s64 = -2098462720;
	// 832A4B74: 388AB160  addi r4, r10, -0x4ea0
	ctx.r[4].s64 = ctx.r[10].s64 + -20128;
	// 832A4B78: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4B7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A4B80: 386ADE30  addi r3, r10, -0x21d0
	ctx.r[3].s64 = ctx.r[10].s64 + -8656;
	// 832A4B84: 38AB2978  addi r5, r11, 0x2978
	ctx.r[5].s64 = ctx.r[11].s64 + 10616;
	// 832A4B88: 4BC21BA1  bl 0x82ec6728
	ctx.lr = 0x832A4B8C;
	sub_82EC6728(ctx, base);
	// 832A4B8C: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4B90: 386B8D58  addi r3, r11, -0x72a8
	ctx.r[3].s64 = ctx.r[11].s64 + -29352;
	// 832A4B94: 4BA0538D  bl 0x82ca9f20
	ctx.lr = 0x832A4B98;
	sub_82CA9F20(ctx, base);
	// 832A4B98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4B9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4BA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4BA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A4BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4BA8 size=72
    let mut pc: u32 = 0x832A4BA8;
    'dispatch: loop {
        match pc {
            0x832A4BA8 => {
    //   block [0x832A4BA8..0x832A4BF0)
	// 832A4BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A4BAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4BB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A4BB4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4BB8: 3D6082EC  lis r11, -0x7d14
	ctx.r[11].s64 = -2098462720;
	// 832A4BBC: 388AB1A4  addi r4, r10, -0x4e5c
	ctx.r[4].s64 = ctx.r[10].s64 + -20060;
	// 832A4BC0: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4BC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A4BC8: 386ADF44  addi r3, r10, -0x20bc
	ctx.r[3].s64 = ctx.r[10].s64 + -8380;
	// 832A4BCC: 38AB3030  addi r5, r11, 0x3030
	ctx.r[5].s64 = ctx.r[11].s64 + 12336;
	// 832A4BD0: 4BC21D31  bl 0x82ec6900
	ctx.lr = 0x832A4BD4;
	sub_82EC6900(ctx, base);
	// 832A4BD4: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4BD8: 386B8D70  addi r3, r11, -0x7290
	ctx.r[3].s64 = ctx.r[11].s64 + -29328;
	// 832A4BDC: 4BA05345  bl 0x82ca9f20
	ctx.lr = 0x832A4BE0;
	sub_82CA9F20(ctx, base);
	// 832A4BE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4BE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4BE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4BEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A4BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4BF0 size=72
    let mut pc: u32 = 0x832A4BF0;
    'dispatch: loop {
        match pc {
            0x832A4BF0 => {
    //   block [0x832A4BF0..0x832A4C38)
	// 832A4BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A4BF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4BF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A4BFC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4C00: 3D6082EC  lis r11, -0x7d14
	ctx.r[11].s64 = -2098462720;
	// 832A4C04: 388AB1C8  addi r4, r10, -0x4e38
	ctx.r[4].s64 = ctx.r[10].s64 + -20024;
	// 832A4C08: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4C0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A4C10: 386AE058  addi r3, r10, -0x1fa8
	ctx.r[3].s64 = ctx.r[10].s64 + -8104;
	// 832A4C14: 38AB3130  addi r5, r11, 0x3130
	ctx.r[5].s64 = ctx.r[11].s64 + 12592;
	// 832A4C18: 4BC21CE9  bl 0x82ec6900
	ctx.lr = 0x832A4C1C;
	sub_82EC6900(ctx, base);
	// 832A4C1C: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4C20: 386B8D88  addi r3, r11, -0x7278
	ctx.r[3].s64 = ctx.r[11].s64 + -29304;
	// 832A4C24: 4BA052FD  bl 0x82ca9f20
	ctx.lr = 0x832A4C28;
	sub_82CA9F20(ctx, base);
	// 832A4C28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4C2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4C30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4C34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A4C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4C38 size=68
    let mut pc: u32 = 0x832A4C38;
    'dispatch: loop {
        match pc {
            0x832A4C38 => {
    //   block [0x832A4C38..0x832A4C7C)
	// 832A4C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A4C3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4C40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A4C44: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4C48: 3D6082EC  lis r11, -0x7d14
	ctx.r[11].s64 = -2098462720;
	// 832A4C4C: 388AB204  addi r4, r10, -0x4dfc
	ctx.r[4].s64 = ctx.r[10].s64 + -19964;
	// 832A4C50: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4C54: 38AB3808  addi r5, r11, 0x3808
	ctx.r[5].s64 = ctx.r[11].s64 + 14344;
	// 832A4C58: 386AE16C  addi r3, r10, -0x1e94
	ctx.r[3].s64 = ctx.r[10].s64 + -7828;
	// 832A4C5C: 4BC3B1C5  bl 0x82edfe20
	ctx.lr = 0x832A4C60;
	sub_82EDFE20(ctx, base);
	// 832A4C60: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4C64: 386B8DA0  addi r3, r11, -0x7260
	ctx.r[3].s64 = ctx.r[11].s64 + -29280;
	// 832A4C68: 4BA052B9  bl 0x82ca9f20
	ctx.lr = 0x832A4C6C;
	sub_82CA9F20(ctx, base);
	// 832A4C6C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4C70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4C74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4C78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A4C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4C80 size=76
    let mut pc: u32 = 0x832A4C80;
    'dispatch: loop {
        match pc {
            0x832A4C80 => {
    //   block [0x832A4C80..0x832A4CCC)
	// 832A4C80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A4C84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4C88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A4C8C: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A4C90: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4C94: 38CB1978  addi r6, r11, 0x1978
	ctx.r[6].s64 = ctx.r[11].s64 + 6520;
	// 832A4C98: 388AB2DC  addi r4, r10, -0x4d24
	ctx.r[4].s64 = ctx.r[10].s64 + -19748;
	// 832A4C9C: 3D6082EC  lis r11, -0x7d14
	ctx.r[11].s64 = -2098462720;
	// 832A4CA0: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4CA4: 38AB7D08  addi r5, r11, 0x7d08
	ctx.r[5].s64 = ctx.r[11].s64 + 32008;
	// 832A4CA8: 386AE284  addi r3, r10, -0x1d7c
	ctx.r[3].s64 = ctx.r[10].s64 + -7548;
	// 832A4CAC: 4BBE2295  bl 0x82e86f40
	ctx.lr = 0x832A4CB0;
	sub_82E86F40(ctx, base);
	// 832A4CB0: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4CB4: 386B8DB8  addi r3, r11, -0x7248
	ctx.r[3].s64 = ctx.r[11].s64 + -29256;
	// 832A4CB8: 4BA05269  bl 0x82ca9f20
	ctx.lr = 0x832A4CBC;
	sub_82CA9F20(ctx, base);
	// 832A4CBC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4CC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4CC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4CC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A4CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4CD0 size=76
    let mut pc: u32 = 0x832A4CD0;
    'dispatch: loop {
        match pc {
            0x832A4CD0 => {
    //   block [0x832A4CD0..0x832A4D1C)
	// 832A4CD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A4CD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4CD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A4CDC: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A4CE0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4CE4: 38CB1978  addi r6, r11, 0x1978
	ctx.r[6].s64 = ctx.r[11].s64 + 6520;
	// 832A4CE8: 388AB330  addi r4, r10, -0x4cd0
	ctx.r[4].s64 = ctx.r[10].s64 + -19664;
	// 832A4CEC: 3D6082ED  lis r11, -0x7d13
	ctx.r[11].s64 = -2098397184;
	// 832A4CF0: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4CF4: 38AB8060  addi r5, r11, -0x7fa0
	ctx.r[5].s64 = ctx.r[11].s64 + -32672;
	// 832A4CF8: 386AE398  addi r3, r10, -0x1c68
	ctx.r[3].s64 = ctx.r[10].s64 + -7272;
	// 832A4CFC: 4BBE2245  bl 0x82e86f40
	ctx.lr = 0x832A4D00;
	sub_82E86F40(ctx, base);
	// 832A4D00: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4D04: 386B8DD0  addi r3, r11, -0x7230
	ctx.r[3].s64 = ctx.r[11].s64 + -29232;
	// 832A4D08: 4BA05219  bl 0x82ca9f20
	ctx.lr = 0x832A4D0C;
	sub_82CA9F20(ctx, base);
	// 832A4D0C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4D10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4D14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4D18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A4D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4D20 size=80
    let mut pc: u32 = 0x832A4D20;
    'dispatch: loop {
        match pc {
            0x832A4D20 => {
    //   block [0x832A4D20..0x832A4D70)
	// 832A4D20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A4D24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4D28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A4D2C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4D30: 3D6082ED  lis r11, -0x7d13
	ctx.r[11].s64 = -2098397184;
	// 832A4D34: 388AB39C  addi r4, r10, -0x4c64
	ctx.r[4].s64 = ctx.r[10].s64 + -19556;
	// 832A4D38: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4D3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A4D40: 386AE4AC  addi r3, r10, -0x1b54
	ctx.r[3].s64 = ctx.r[10].s64 + -6996;
	// 832A4D44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A4D48: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A4D4C: 38AB8D98  addi r5, r11, -0x7268
	ctx.r[5].s64 = ctx.r[11].s64 + -29288;
	// 832A4D50: 4BBE0A71  bl 0x82e857c0
	ctx.lr = 0x832A4D54;
	sub_82E857C0(ctx, base);
	// 832A4D54: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4D58: 386B8DE8  addi r3, r11, -0x7218
	ctx.r[3].s64 = ctx.r[11].s64 + -29208;
	// 832A4D5C: 4BA051C5  bl 0x82ca9f20
	ctx.lr = 0x832A4D60;
	sub_82CA9F20(ctx, base);
	// 832A4D60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4D64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4D68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4D6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A4D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4D70 size=76
    let mut pc: u32 = 0x832A4D70;
    'dispatch: loop {
        match pc {
            0x832A4D70 => {
    //   block [0x832A4D70..0x832A4DBC)
	// 832A4D70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A4D74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4D78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A4D7C: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A4D80: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4D84: 38CB1978  addi r6, r11, 0x1978
	ctx.r[6].s64 = ctx.r[11].s64 + 6520;
	// 832A4D88: 388AB3EC  addi r4, r10, -0x4c14
	ctx.r[4].s64 = ctx.r[10].s64 + -19476;
	// 832A4D8C: 3D6082ED  lis r11, -0x7d13
	ctx.r[11].s64 = -2098397184;
	// 832A4D90: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4D94: 38AB8ED8  addi r5, r11, -0x7128
	ctx.r[5].s64 = ctx.r[11].s64 + -28968;
	// 832A4D98: 386AE5C0  addi r3, r10, -0x1a40
	ctx.r[3].s64 = ctx.r[10].s64 + -6720;
	// 832A4D9C: 4BBE21A5  bl 0x82e86f40
	ctx.lr = 0x832A4DA0;
	sub_82E86F40(ctx, base);
	// 832A4DA0: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4DA4: 386B8E00  addi r3, r11, -0x7200
	ctx.r[3].s64 = ctx.r[11].s64 + -29184;
	// 832A4DA8: 4BA05179  bl 0x82ca9f20
	ctx.lr = 0x832A4DAC;
	sub_82CA9F20(ctx, base);
	// 832A4DAC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4DB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4DB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4DB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A4DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4DC0 size=76
    let mut pc: u32 = 0x832A4DC0;
    'dispatch: loop {
        match pc {
            0x832A4DC0 => {
    //   block [0x832A4DC0..0x832A4E0C)
	// 832A4DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A4DC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4DC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A4DCC: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A4DD0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4DD4: 38CB1978  addi r6, r11, 0x1978
	ctx.r[6].s64 = ctx.r[11].s64 + 6520;
	// 832A4DD8: 388AB4AC  addi r4, r10, -0x4b54
	ctx.r[4].s64 = ctx.r[10].s64 + -19284;
	// 832A4DDC: 3D6082ED  lis r11, -0x7d13
	ctx.r[11].s64 = -2098397184;
	// 832A4DE0: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4DE4: 38ABA100  addi r5, r11, -0x5f00
	ctx.r[5].s64 = ctx.r[11].s64 + -24320;
	// 832A4DE8: 386AE6D4  addi r3, r10, -0x192c
	ctx.r[3].s64 = ctx.r[10].s64 + -6444;
	// 832A4DEC: 4BBE2155  bl 0x82e86f40
	ctx.lr = 0x832A4DF0;
	sub_82E86F40(ctx, base);
	// 832A4DF0: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4DF4: 386B8E18  addi r3, r11, -0x71e8
	ctx.r[3].s64 = ctx.r[11].s64 + -29160;
	// 832A4DF8: 4BA05129  bl 0x82ca9f20
	ctx.lr = 0x832A4DFC;
	sub_82CA9F20(ctx, base);
	// 832A4DFC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4E00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4E04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4E08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A4E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4E10 size=76
    let mut pc: u32 = 0x832A4E10;
    'dispatch: loop {
        match pc {
            0x832A4E10 => {
    //   block [0x832A4E10..0x832A4E5C)
	// 832A4E10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A4E14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4E18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A4E1C: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A4E20: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4E24: 38CB1A8C  addi r6, r11, 0x1a8c
	ctx.r[6].s64 = ctx.r[11].s64 + 6796;
	// 832A4E28: 388AB534  addi r4, r10, -0x4acc
	ctx.r[4].s64 = ctx.r[10].s64 + -19148;
	// 832A4E2C: 3D6082ED  lis r11, -0x7d13
	ctx.r[11].s64 = -2098397184;
	// 832A4E30: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4E34: 38ABA410  addi r5, r11, -0x5bf0
	ctx.r[5].s64 = ctx.r[11].s64 + -23536;
	// 832A4E38: 386AE7E8  addi r3, r10, -0x1818
	ctx.r[3].s64 = ctx.r[10].s64 + -6168;
	// 832A4E3C: 4BBE2105  bl 0x82e86f40
	ctx.lr = 0x832A4E40;
	sub_82E86F40(ctx, base);
	// 832A4E40: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4E44: 386B8E30  addi r3, r11, -0x71d0
	ctx.r[3].s64 = ctx.r[11].s64 + -29136;
	// 832A4E48: 4BA050D9  bl 0x82ca9f20
	ctx.lr = 0x832A4E4C;
	sub_82CA9F20(ctx, base);
	// 832A4E4C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4E50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4E54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4E58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A4E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4E60 size=76
    let mut pc: u32 = 0x832A4E60;
    'dispatch: loop {
        match pc {
            0x832A4E60 => {
    //   block [0x832A4E60..0x832A4EAC)
	// 832A4E60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A4E64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4E68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A4E6C: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A4E70: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4E74: 38CB1A8C  addi r6, r11, 0x1a8c
	ctx.r[6].s64 = ctx.r[11].s64 + 6796;
	// 832A4E78: 388AB5A4  addi r4, r10, -0x4a5c
	ctx.r[4].s64 = ctx.r[10].s64 + -19036;
	// 832A4E7C: 3D6082ED  lis r11, -0x7d13
	ctx.r[11].s64 = -2098397184;
	// 832A4E80: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4E84: 38ABADE8  addi r5, r11, -0x5218
	ctx.r[5].s64 = ctx.r[11].s64 + -21016;
	// 832A4E88: 386AE8FC  addi r3, r10, -0x1704
	ctx.r[3].s64 = ctx.r[10].s64 + -5892;
	// 832A4E8C: 4BBE20B5  bl 0x82e86f40
	ctx.lr = 0x832A4E90;
	sub_82E86F40(ctx, base);
	// 832A4E90: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4E94: 386B8E48  addi r3, r11, -0x71b8
	ctx.r[3].s64 = ctx.r[11].s64 + -29112;
	// 832A4E98: 4BA05089  bl 0x82ca9f20
	ctx.lr = 0x832A4E9C;
	sub_82CA9F20(ctx, base);
	// 832A4E9C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4EA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4EA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4EA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A4EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4EB0 size=76
    let mut pc: u32 = 0x832A4EB0;
    'dispatch: loop {
        match pc {
            0x832A4EB0 => {
    //   block [0x832A4EB0..0x832A4EFC)
	// 832A4EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A4EB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4EB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A4EBC: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A4EC0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4EC4: 38CB1A8C  addi r6, r11, 0x1a8c
	ctx.r[6].s64 = ctx.r[11].s64 + 6796;
	// 832A4EC8: 388AB600  addi r4, r10, -0x4a00
	ctx.r[4].s64 = ctx.r[10].s64 + -18944;
	// 832A4ECC: 3D6082ED  lis r11, -0x7d13
	ctx.r[11].s64 = -2098397184;
	// 832A4ED0: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4ED4: 38ABB110  addi r5, r11, -0x4ef0
	ctx.r[5].s64 = ctx.r[11].s64 + -20208;
	// 832A4ED8: 386AEA10  addi r3, r10, -0x15f0
	ctx.r[3].s64 = ctx.r[10].s64 + -5616;
	// 832A4EDC: 4BBE2065  bl 0x82e86f40
	ctx.lr = 0x832A4EE0;
	sub_82E86F40(ctx, base);
	// 832A4EE0: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4EE4: 386B8E60  addi r3, r11, -0x71a0
	ctx.r[3].s64 = ctx.r[11].s64 + -29088;
	// 832A4EE8: 4BA05039  bl 0x82ca9f20
	ctx.lr = 0x832A4EEC;
	sub_82CA9F20(ctx, base);
	// 832A4EEC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4EF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4EF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4EF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A4F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4F00 size=76
    let mut pc: u32 = 0x832A4F00;
    'dispatch: loop {
        match pc {
            0x832A4F00 => {
    //   block [0x832A4F00..0x832A4F4C)
	// 832A4F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A4F04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4F08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A4F0C: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A4F10: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4F14: 38CB1A8C  addi r6, r11, 0x1a8c
	ctx.r[6].s64 = ctx.r[11].s64 + 6796;
	// 832A4F18: 388AB658  addi r4, r10, -0x49a8
	ctx.r[4].s64 = ctx.r[10].s64 + -18856;
	// 832A4F1C: 3D6082ED  lis r11, -0x7d13
	ctx.r[11].s64 = -2098397184;
	// 832A4F20: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4F24: 38ABB358  addi r5, r11, -0x4ca8
	ctx.r[5].s64 = ctx.r[11].s64 + -19624;
	// 832A4F28: 386AEB24  addi r3, r10, -0x14dc
	ctx.r[3].s64 = ctx.r[10].s64 + -5340;
	// 832A4F2C: 4BBE2015  bl 0x82e86f40
	ctx.lr = 0x832A4F30;
	sub_82E86F40(ctx, base);
	// 832A4F30: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4F34: 386B8E78  addi r3, r11, -0x7188
	ctx.r[3].s64 = ctx.r[11].s64 + -29064;
	// 832A4F38: 4BA04FE9  bl 0x82ca9f20
	ctx.lr = 0x832A4F3C;
	sub_82CA9F20(ctx, base);
	// 832A4F3C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4F40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4F44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4F48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A4F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4F50 size=76
    let mut pc: u32 = 0x832A4F50;
    'dispatch: loop {
        match pc {
            0x832A4F50 => {
    //   block [0x832A4F50..0x832A4F9C)
	// 832A4F50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A4F54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4F58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A4F5C: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A4F60: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4F64: 38CB1A8C  addi r6, r11, 0x1a8c
	ctx.r[6].s64 = ctx.r[11].s64 + 6796;
	// 832A4F68: 388AB6B4  addi r4, r10, -0x494c
	ctx.r[4].s64 = ctx.r[10].s64 + -18764;
	// 832A4F6C: 3D6082ED  lis r11, -0x7d13
	ctx.r[11].s64 = -2098397184;
	// 832A4F70: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4F74: 38ABB7D0  addi r5, r11, -0x4830
	ctx.r[5].s64 = ctx.r[11].s64 + -18480;
	// 832A4F78: 386AEC38  addi r3, r10, -0x13c8
	ctx.r[3].s64 = ctx.r[10].s64 + -5064;
	// 832A4F7C: 4BBE1FC5  bl 0x82e86f40
	ctx.lr = 0x832A4F80;
	sub_82E86F40(ctx, base);
	// 832A4F80: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4F84: 386B8E90  addi r3, r11, -0x7170
	ctx.r[3].s64 = ctx.r[11].s64 + -29040;
	// 832A4F88: 4BA04F99  bl 0x82ca9f20
	ctx.lr = 0x832A4F8C;
	sub_82CA9F20(ctx, base);
	// 832A4F8C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4F90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4F94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4F98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A4FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4FA0 size=76
    let mut pc: u32 = 0x832A4FA0;
    'dispatch: loop {
        match pc {
            0x832A4FA0 => {
    //   block [0x832A4FA0..0x832A4FEC)
	// 832A4FA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A4FA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4FA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A4FAC: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A4FB0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A4FB4: 38CB1A8C  addi r6, r11, 0x1a8c
	ctx.r[6].s64 = ctx.r[11].s64 + 6796;
	// 832A4FB8: 388AB710  addi r4, r10, -0x48f0
	ctx.r[4].s64 = ctx.r[10].s64 + -18672;
	// 832A4FBC: 3D6082ED  lis r11, -0x7d13
	ctx.r[11].s64 = -2098397184;
	// 832A4FC0: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A4FC4: 38ABBAC8  addi r5, r11, -0x4538
	ctx.r[5].s64 = ctx.r[11].s64 + -17720;
	// 832A4FC8: 386AED4C  addi r3, r10, -0x12b4
	ctx.r[3].s64 = ctx.r[10].s64 + -4788;
	// 832A4FCC: 4BBE1F75  bl 0x82e86f40
	ctx.lr = 0x832A4FD0;
	sub_82E86F40(ctx, base);
	// 832A4FD0: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A4FD4: 386B8EA8  addi r3, r11, -0x7158
	ctx.r[3].s64 = ctx.r[11].s64 + -29016;
	// 832A4FD8: 4BA04F49  bl 0x82ca9f20
	ctx.lr = 0x832A4FDC;
	sub_82CA9F20(ctx, base);
	// 832A4FDC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A4FE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A4FE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A4FE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A4FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A4FF0 size=76
    let mut pc: u32 = 0x832A4FF0;
    'dispatch: loop {
        match pc {
            0x832A4FF0 => {
    //   block [0x832A4FF0..0x832A503C)
	// 832A4FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A4FF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A4FF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A4FFC: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A5000: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A5004: 38CB1A8C  addi r6, r11, 0x1a8c
	ctx.r[6].s64 = ctx.r[11].s64 + 6796;
	// 832A5008: 388AB768  addi r4, r10, -0x4898
	ctx.r[4].s64 = ctx.r[10].s64 + -18584;
	// 832A500C: 3D6082ED  lis r11, -0x7d13
	ctx.r[11].s64 = -2098397184;
	// 832A5010: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A5014: 38ABBDD0  addi r5, r11, -0x4230
	ctx.r[5].s64 = ctx.r[11].s64 + -16944;
	// 832A5018: 386AEE60  addi r3, r10, -0x11a0
	ctx.r[3].s64 = ctx.r[10].s64 + -4512;
	// 832A501C: 4BBE1F25  bl 0x82e86f40
	ctx.lr = 0x832A5020;
	sub_82E86F40(ctx, base);
	// 832A5020: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5024: 386B8EC0  addi r3, r11, -0x7140
	ctx.r[3].s64 = ctx.r[11].s64 + -28992;
	// 832A5028: 4BA04EF9  bl 0x82ca9f20
	ctx.lr = 0x832A502C;
	sub_82CA9F20(ctx, base);
	// 832A502C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5030: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5034: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5038: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5040 size=80
    let mut pc: u32 = 0x832A5040;
    'dispatch: loop {
        match pc {
            0x832A5040 => {
    //   block [0x832A5040..0x832A5090)
	// 832A5040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5048: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A504C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A5050: 3D6082ED  lis r11, -0x7d13
	ctx.r[11].s64 = -2098397184;
	// 832A5054: 388AB85C  addi r4, r10, -0x47a4
	ctx.r[4].s64 = ctx.r[10].s64 + -18340;
	// 832A5058: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A505C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A5060: 386AEF78  addi r3, r10, -0x1088
	ctx.r[3].s64 = ctx.r[10].s64 + -4232;
	// 832A5064: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 832A5068: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A506C: 38ABC5A0  addi r5, r11, -0x3a60
	ctx.r[5].s64 = ctx.r[11].s64 + -14944;
	// 832A5070: 4BBE0751  bl 0x82e857c0
	ctx.lr = 0x832A5074;
	sub_82E857C0(ctx, base);
	// 832A5074: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5078: 386B8ED8  addi r3, r11, -0x7128
	ctx.r[3].s64 = ctx.r[11].s64 + -28968;
	// 832A507C: 4BA04EA5  bl 0x82ca9f20
	ctx.lr = 0x832A5080;
	sub_82CA9F20(ctx, base);
	// 832A5080: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5084: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5088: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A508C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5090 size=80
    let mut pc: u32 = 0x832A5090;
    'dispatch: loop {
        match pc {
            0x832A5090 => {
    //   block [0x832A5090..0x832A50E0)
	// 832A5090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5098: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A509C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A50A0: 3D6082ED  lis r11, -0x7d13
	ctx.r[11].s64 = -2098397184;
	// 832A50A4: 388ABA58  addi r4, r10, -0x45a8
	ctx.r[4].s64 = ctx.r[10].s64 + -17832;
	// 832A50A8: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 832A50AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A50B0: 386AF090  addi r3, r10, -0xf70
	ctx.r[3].s64 = ctx.r[10].s64 + -3952;
	// 832A50B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A50B8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A50BC: 38AB0BA0  addi r5, r11, 0xba0
	ctx.r[5].s64 = ctx.r[11].s64 + 2976;
	// 832A50C0: 4BBE0701  bl 0x82e857c0
	ctx.lr = 0x832A50C4;
	sub_82E857C0(ctx, base);
	// 832A50C4: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A50C8: 386B8EF0  addi r3, r11, -0x7110
	ctx.r[3].s64 = ctx.r[11].s64 + -28944;
	// 832A50CC: 4BA04E55  bl 0x82ca9f20
	ctx.lr = 0x832A50D0;
	sub_82CA9F20(ctx, base);
	// 832A50D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A50D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A50D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A50DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A50E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A50E0 size=72
    let mut pc: u32 = 0x832A50E0;
    'dispatch: loop {
        match pc {
            0x832A50E0 => {
    //   block [0x832A50E0..0x832A5128)
	// 832A50E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A50E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A50E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A50EC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A50F0: 3D6082EE  lis r11, -0x7d12
	ctx.r[11].s64 = -2098331648;
	// 832A50F4: 388ABC60  addi r4, r10, -0x43a0
	ctx.r[4].s64 = ctx.r[10].s64 + -17312;
	// 832A50F8: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A50FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A5100: 386A89BC  addi r3, r10, -0x7644
	ctx.r[3].s64 = ctx.r[10].s64 + -30276;
	// 832A5104: 38ABB1F8  addi r5, r11, -0x4e08
	ctx.r[5].s64 = ctx.r[11].s64 + -19976;
	// 832A5108: 4BC20939  bl 0x82ec5a40
	ctx.lr = 0x832A510C;
	sub_82EC5A40(ctx, base);
	// 832A510C: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5110: 386B8F08  addi r3, r11, -0x70f8
	ctx.r[3].s64 = ctx.r[11].s64 + -28920;
	// 832A5114: 4BA04E0D  bl 0x82ca9f20
	ctx.lr = 0x832A5118;
	sub_82CA9F20(ctx, base);
	// 832A5118: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A511C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5120: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5124: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5128 size=72
    let mut pc: u32 = 0x832A5128;
    'dispatch: loop {
        match pc {
            0x832A5128 => {
    //   block [0x832A5128..0x832A5170)
	// 832A5128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A512C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5130: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5134: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A5138: 3D6082EE  lis r11, -0x7d12
	ctx.r[11].s64 = -2098331648;
	// 832A513C: 388ABD28  addi r4, r10, -0x42d8
	ctx.r[4].s64 = ctx.r[10].s64 + -17112;
	// 832A5140: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A5144: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A5148: 386A8AD0  addi r3, r10, -0x7530
	ctx.r[3].s64 = ctx.r[10].s64 + -30000;
	// 832A514C: 38ABC5E8  addi r5, r11, -0x3a18
	ctx.r[5].s64 = ctx.r[11].s64 + -14872;
	// 832A5150: 4BC208F1  bl 0x82ec5a40
	ctx.lr = 0x832A5154;
	sub_82EC5A40(ctx, base);
	// 832A5154: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5158: 386B8F20  addi r3, r11, -0x70e0
	ctx.r[3].s64 = ctx.r[11].s64 + -28896;
	// 832A515C: 4BA04DC5  bl 0x82ca9f20
	ctx.lr = 0x832A5160;
	sub_82CA9F20(ctx, base);
	// 832A5160: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5164: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5168: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A516C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5170 size=80
    let mut pc: u32 = 0x832A5170;
    'dispatch: loop {
        match pc {
            0x832A5170 => {
    //   block [0x832A5170..0x832A51C0)
	// 832A5170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5178: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A517C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A5180: 3D6082EE  lis r11, -0x7d12
	ctx.r[11].s64 = -2098331648;
	// 832A5184: 388ABD54  addi r4, r10, -0x42ac
	ctx.r[4].s64 = ctx.r[10].s64 + -17068;
	// 832A5188: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A518C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A5190: 386A8BE4  addi r3, r10, -0x741c
	ctx.r[3].s64 = ctx.r[10].s64 + -29724;
	// 832A5194: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A5198: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A519C: 38ABCD20  addi r5, r11, -0x32e0
	ctx.r[5].s64 = ctx.r[11].s64 + -13024;
	// 832A51A0: 4BBE0621  bl 0x82e857c0
	ctx.lr = 0x832A51A4;
	sub_82E857C0(ctx, base);
	// 832A51A4: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A51A8: 386B8F38  addi r3, r11, -0x70c8
	ctx.r[3].s64 = ctx.r[11].s64 + -28872;
	// 832A51AC: 4BA04D75  bl 0x82ca9f20
	ctx.lr = 0x832A51B0;
	sub_82CA9F20(ctx, base);
	// 832A51B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A51B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A51B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A51BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A51C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A51C0 size=68
    let mut pc: u32 = 0x832A51C0;
    'dispatch: loop {
        match pc {
            0x832A51C0 => {
    //   block [0x832A51C0..0x832A5204)
	// 832A51C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A51C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A51C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A51CC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A51D0: 3D6082EE  lis r11, -0x7d12
	ctx.r[11].s64 = -2098331648;
	// 832A51D4: 388ABD64  addi r4, r10, -0x429c
	ctx.r[4].s64 = ctx.r[10].s64 + -17052;
	// 832A51D8: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A51DC: 38ABEA40  addi r5, r11, -0x15c0
	ctx.r[5].s64 = ctx.r[11].s64 + -5568;
	// 832A51E0: 386A8CF8  addi r3, r10, -0x7308
	ctx.r[3].s64 = ctx.r[10].s64 + -29448;
	// 832A51E4: 4BC3AC3D  bl 0x82edfe20
	ctx.lr = 0x832A51E8;
	sub_82EDFE20(ctx, base);
	// 832A51E8: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A51EC: 386B8F50  addi r3, r11, -0x70b0
	ctx.r[3].s64 = ctx.r[11].s64 + -28848;
	// 832A51F0: 4BA04D31  bl 0x82ca9f20
	ctx.lr = 0x832A51F4;
	sub_82CA9F20(ctx, base);
	// 832A51F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A51F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A51FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5200: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5208 size=68
    let mut pc: u32 = 0x832A5208;
    'dispatch: loop {
        match pc {
            0x832A5208 => {
    //   block [0x832A5208..0x832A524C)
	// 832A5208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A520C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5210: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5214: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 832A5218: 3D6082EE  lis r11, -0x7d12
	ctx.r[11].s64 = -2098331648;
	// 832A521C: 388ABDC4  addi r4, r10, -0x423c
	ctx.r[4].s64 = ctx.r[10].s64 + -16956;
	// 832A5220: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A5224: 38ABFC48  addi r5, r11, -0x3b8
	ctx.r[5].s64 = ctx.r[11].s64 + -952;
	// 832A5228: 386A8E0C  addi r3, r10, -0x71f4
	ctx.r[3].s64 = ctx.r[10].s64 + -29172;
	// 832A522C: 4BC3ABF5  bl 0x82edfe20
	ctx.lr = 0x832A5230;
	sub_82EDFE20(ctx, base);
	// 832A5230: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5234: 386B8F68  addi r3, r11, -0x7098
	ctx.r[3].s64 = ctx.r[11].s64 + -28824;
	// 832A5238: 4BA04CE9  bl 0x82ca9f20
	ctx.lr = 0x832A523C;
	sub_82CA9F20(ctx, base);
	// 832A523C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5240: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5244: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5248: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5250 size=76
    let mut pc: u32 = 0x832A5250;
    'dispatch: loop {
        match pc {
            0x832A5250 => {
    //   block [0x832A5250..0x832A529C)
	// 832A5250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5258: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A525C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5260: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5264: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 832A5268: 3BEB8F90  addi r31, r11, -0x7070
	ctx.r[31].s64 = ctx.r[11].s64 + -28784;
	// 832A526C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A5270: 4BC53AD1  bl 0x82ef8d40
	ctx.lr = 0x832A5274;
	sub_82EF8D40(ctx, base);
	// 832A5274: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 832A5278: 3D40832C  lis r10, -0x7cd4
	ctx.r[10].s64 = -2094268416;
	// 832A527C: 997F001C  stb r11, 0x1c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u8 ) };
	// 832A5280: 386A8F80  addi r3, r10, -0x7080
	ctx.r[3].s64 = ctx.r[10].s64 + -28800;
	// 832A5284: 4BA04C9D  bl 0x82ca9f20
	ctx.lr = 0x832A5288;
	sub_82CA9F20(ctx, base);
	// 832A5288: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A528C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5290: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5294: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A5298: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A52A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A52A0 size=52
    let mut pc: u32 = 0x832A52A0;
    'dispatch: loop {
        match pc {
            0x832A52A0 => {
    //   block [0x832A52A0..0x832A52D4)
	// 832A52A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A52A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A52A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A52AC: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A52B0: 386B9010  addi r3, r11, -0x6ff0
	ctx.r[3].s64 = ctx.r[11].s64 + -28656;
	// 832A52B4: 4BC5190D  bl 0x82ef6bc0
	ctx.lr = 0x832A52B8;
	sub_82EF6BC0(ctx, base);
	// 832A52B8: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A52BC: 386B8FC8  addi r3, r11, -0x7038
	ctx.r[3].s64 = ctx.r[11].s64 + -28728;
	// 832A52C0: 4BA04C61  bl 0x82ca9f20
	ctx.lr = 0x832A52C4;
	sub_82CA9F20(ctx, base);
	// 832A52C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A52C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A52CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A52D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A52D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A52D8 size=52
    let mut pc: u32 = 0x832A52D8;
    'dispatch: loop {
        match pc {
            0x832A52D8 => {
    //   block [0x832A52D8..0x832A530C)
	// 832A52D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A52DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A52E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A52E4: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A52E8: 386B8FC8  addi r3, r11, -0x7038
	ctx.r[3].s64 = ctx.r[11].s64 + -28728;
	// 832A52EC: 4BC51845  bl 0x82ef6b30
	ctx.lr = 0x832A52F0;
	sub_82EF6B30(ctx, base);
	// 832A52F0: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A52F4: 386B9008  addi r3, r11, -0x6ff8
	ctx.r[3].s64 = ctx.r[11].s64 + -28664;
	// 832A52F8: 4BA04C29  bl 0x82ca9f20
	ctx.lr = 0x832A52FC;
	sub_82CA9F20(ctx, base);
	// 832A52FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5300: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5304: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5308: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A5310 size=12
    let mut pc: u32 = 0x832A5310;
    'dispatch: loop {
        match pc {
            0x832A5310 => {
    //   block [0x832A5310..0x832A531C)
	// 832A5310: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5314: 386B8FB0  addi r3, r11, -0x7050
	ctx.r[3].s64 = ctx.r[11].s64 + -28752;
	// 832A5318: 4BA04C08  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A5320 size=12
    let mut pc: u32 = 0x832A5320;
    'dispatch: loop {
        match pc {
            0x832A5320 => {
    //   block [0x832A5320..0x832A532C)
	// 832A5320: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5324: 386B9020  addi r3, r11, -0x6fe0
	ctx.r[3].s64 = ctx.r[11].s64 + -28640;
	// 832A5328: 4BA04BF8  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A5330 size=12
    let mut pc: u32 = 0x832A5330;
    'dispatch: loop {
        match pc {
            0x832A5330 => {
    //   block [0x832A5330..0x832A533C)
	// 832A5330: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5334: 386B9038  addi r3, r11, -0x6fc8
	ctx.r[3].s64 = ctx.r[11].s64 + -28616;
	// 832A5338: 4BA04BE8  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A5340 size=12
    let mut pc: u32 = 0x832A5340;
    'dispatch: loop {
        match pc {
            0x832A5340 => {
    //   block [0x832A5340..0x832A534C)
	// 832A5340: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5344: 386B90A8  addi r3, r11, -0x6f58
	ctx.r[3].s64 = ctx.r[11].s64 + -28504;
	// 832A5348: 4BA04BD8  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A5350 size=12
    let mut pc: u32 = 0x832A5350;
    'dispatch: loop {
        match pc {
            0x832A5350 => {
    //   block [0x832A5350..0x832A535C)
	// 832A5350: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5354: 386B9100  addi r3, r11, -0x6f00
	ctx.r[3].s64 = ctx.r[11].s64 + -28416;
	// 832A5358: 4BA04BC8  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5360 size=120
    let mut pc: u32 = 0x832A5360;
    'dispatch: loop {
        match pc {
            0x832A5360 => {
    //   block [0x832A5360..0x832A53D8)
	// 832A5360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5368: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A536C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5370: 4BCF9921  bl 0x82f9ec90
	ctx.lr = 0x832A5374;
	sub_82F9EC90(ctx, base);
	// 832A5374: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A5378: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 832A537C: 3BEA9628  addi r31, r10, -0x69d8
	ctx.r[31].s64 = ctx.r[10].s64 + -27096;
	// 832A5380: 996A9628  stb r11, -0x69d8(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(-27096 as u32), ctx.r[11].u8 ) };
	// 832A5384: D83F0008  stfd f1, 8(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.f[1].u64 ) };
	// 832A5388: 4BCF9909  bl 0x82f9ec90
	ctx.lr = 0x832A538C;
	sub_82F9EC90(ctx, base);
	// 832A538C: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 832A5390: D83F0018  stfd f1, 0x18(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.f[1].u64 ) };
	// 832A5394: 997F0010  stb r11, 0x10(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 832A5398: 4BCF98F9  bl 0x82f9ec90
	ctx.lr = 0x832A539C;
	sub_82F9EC90(ctx, base);
	// 832A539C: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 832A53A0: D83F0028  stfd f1, 0x28(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.f[1].u64 ) };
	// 832A53A4: 997F0020  stb r11, 0x20(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u8 ) };
	// 832A53A8: 4BCF98E9  bl 0x82f9ec90
	ctx.lr = 0x832A53AC;
	sub_82F9EC90(ctx, base);
	// 832A53AC: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 832A53B0: D83F0038  stfd f1, 0x38(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.f[1].u64 ) };
	// 832A53B4: 997F0030  stb r11, 0x30(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u8 ) };
	// 832A53B8: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A53BC: 386B9160  addi r3, r11, -0x6ea0
	ctx.r[3].s64 = ctx.r[11].s64 + -28320;
	// 832A53C0: 4BA04B61  bl 0x82ca9f20
	ctx.lr = 0x832A53C4;
	sub_82CA9F20(ctx, base);
	// 832A53C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A53C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A53CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A53D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A53D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A53D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A53D8 size=12
    let mut pc: u32 = 0x832A53D8;
    'dispatch: loop {
        match pc {
            0x832A53D8 => {
    //   block [0x832A53D8..0x832A53E4)
	// 832A53D8: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A53DC: 386B966C  addi r3, r11, -0x6994
	ctx.r[3].s64 = ctx.r[11].s64 + -27028;
	// 832A53E0: 4BCF9838  b 0x82f9ec18
	sub_82F9EC18(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A53E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A53E8 size=12
    let mut pc: u32 = 0x832A53E8;
    'dispatch: loop {
        match pc {
            0x832A53E8 => {
    //   block [0x832A53E8..0x832A53F4)
	// 832A53E8: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A53EC: 386B91C0  addi r3, r11, -0x6e40
	ctx.r[3].s64 = ctx.r[11].s64 + -28224;
	// 832A53F0: 4BA04B30  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A53F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A53F8 size=88
    let mut pc: u32 = 0x832A53F8;
    'dispatch: loop {
        match pc {
            0x832A53F8 => {
    //   block [0x832A53F8..0x832A5450)
	// 832A53F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A53FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5400: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A5404: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5408: 4BCF9889  bl 0x82f9ec90
	ctx.lr = 0x832A540C;
	sub_82F9EC90(ctx, base);
	// 832A540C: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A5410: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 832A5414: 3BEA9698  addi r31, r10, -0x6968
	ctx.r[31].s64 = ctx.r[10].s64 + -26984;
	// 832A5418: 996A9698  stb r11, -0x6968(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(-26984 as u32), ctx.r[11].u8 ) };
	// 832A541C: D83F0008  stfd f1, 8(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.f[1].u64 ) };
	// 832A5420: 4BCF9871  bl 0x82f9ec90
	ctx.lr = 0x832A5424;
	sub_82F9EC90(ctx, base);
	// 832A5424: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 832A5428: D83F0018  stfd f1, 0x18(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.f[1].u64 ) };
	// 832A542C: 997F0010  stb r11, 0x10(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 832A5430: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5434: 386B9220  addi r3, r11, -0x6de0
	ctx.r[3].s64 = ctx.r[11].s64 + -28128;
	// 832A5438: 4BA04AE9  bl 0x82ca9f20
	ctx.lr = 0x832A543C;
	sub_82CA9F20(ctx, base);
	// 832A543C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5440: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5444: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5448: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A544C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A5450 size=12
    let mut pc: u32 = 0x832A5450;
    'dispatch: loop {
        match pc {
            0x832A5450 => {
    //   block [0x832A5450..0x832A545C)
	// 832A5450: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5454: 386B9280  addi r3, r11, -0x6d80
	ctx.r[3].s64 = ctx.r[11].s64 + -28032;
	// 832A5458: 4BA04AC8  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A5460 size=72
    let mut pc: u32 = 0x832A5460;
    'dispatch: loop {
        match pc {
            0x832A5460 => {
    //   block [0x832A5460..0x832A54A8)
	// 832A5460: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 832A5464: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 832A5468: 3D208330  lis r9, -0x7cd0
	ctx.r[9].s64 = -2094006272;
	// 832A546C: 3D008330  lis r8, -0x7cd0
	ctx.r[8].s64 = -2094006272;
	// 832A5470: 3CE08330  lis r7, -0x7cd0
	ctx.r[7].s64 = -2094006272;
	// 832A5474: 3CC08336  lis r6, -0x7cca
	ctx.r[6].s64 = -2093613056;
	// 832A5478: 816BF66C  lwz r11, -0x994(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-2452 as u32) ) } as u64;
	// 832A547C: 814AF670  lwz r10, -0x990(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-2448 as u32) ) } as u64;
	// 832A5480: 8129F674  lwz r9, -0x98c(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-2444 as u32) ) } as u64;
	// 832A5484: 38A69700  addi r5, r6, -0x6900
	ctx.r[5].s64 = ctx.r[6].s64 + -26880;
	// 832A5488: 8108F678  lwz r8, -0x988(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-2440 as u32) ) } as u64;
	// 832A548C: 80E7F67C  lwz r7, -0x984(r7)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(-2436 as u32) ) } as u64;
	// 832A5490: 91669700  stw r11, -0x6900(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(-26880 as u32), ctx.r[11].u32 ) };
	// 832A5494: 91450004  stw r10, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 832A5498: 91250008  stw r9, 8(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 832A549C: 9105000C  stw r8, 0xc(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 832A54A0: 90E50010  stw r7, 0x10(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(16 as u32), ctx.r[7].u32 ) };
	// 832A54A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A54A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A54A8 size=320
    let mut pc: u32 = 0x832A54A8;
    'dispatch: loop {
        match pc {
            0x832A54A8 => {
    //   block [0x832A54A8..0x832A55E8)
	// 832A54A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A54AC: 4BA03F39  bl 0x82ca93e4
	ctx.lr = 0x832A54B0;
	sub_82CA93D0(ctx, base);
	// 832A54B0: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 832A54B4: 3D208330  lis r9, -0x7cd0
	ctx.r[9].s64 = -2094006272;
	// 832A54B8: 3D008330  lis r8, -0x7cd0
	ctx.r[8].s64 = -2094006272;
	// 832A54BC: 3CE08330  lis r7, -0x7cd0
	ctx.r[7].s64 = -2094006272;
	// 832A54C0: 3CC08330  lis r6, -0x7cd0
	ctx.r[6].s64 = -2094006272;
	// 832A54C4: 3CA08336  lis r5, -0x7cca
	ctx.r[5].s64 = -2093613056;
	// 832A54C8: 814BF69C  lwz r10, -0x964(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-2404 as u32) ) } as u64;
	// 832A54CC: 8129F6A0  lwz r9, -0x960(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-2400 as u32) ) } as u64;
	// 832A54D0: 3C808330  lis r4, -0x7cd0
	ctx.r[4].s64 = -2094006272;
	// 832A54D4: 39659718  addi r11, r5, -0x68e8
	ctx.r[11].s64 = ctx.r[5].s64 + -26856;
	// 832A54D8: 8108F6A4  lwz r8, -0x95c(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-2396 as u32) ) } as u64;
	// 832A54DC: 80E7F6AC  lwz r7, -0x954(r7)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(-2388 as u32) ) } as u64;
	// 832A54E0: 3C608330  lis r3, -0x7cd0
	ctx.r[3].s64 = -2094006272;
	// 832A54E4: 80C6F6B0  lwz r6, -0x950(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-2384 as u32) ) } as u64;
	// 832A54E8: 3FE08330  lis r31, -0x7cd0
	ctx.r[31].s64 = -2094006272;
	// 832A54EC: 91459718  stw r10, -0x68e8(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(-26856 as u32), ctx.r[10].u32 ) };
	// 832A54F0: 3FC08330  lis r30, -0x7cd0
	ctx.r[30].s64 = -2094006272;
	// 832A54F4: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 832A54F8: 80A4F6B8  lwz r5, -0x948(r4)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(-2376 as u32) ) } as u64;
	// 832A54FC: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 832A5500: 3D208330  lis r9, -0x7cd0
	ctx.r[9].s64 = -2094006272;
	// 832A5504: 910B0008  stw r8, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 832A5508: 3D008330  lis r8, -0x7cd0
	ctx.r[8].s64 = -2094006272;
	// 832A550C: 90EB000C  stw r7, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 832A5510: 3CE08330  lis r7, -0x7cd0
	ctx.r[7].s64 = -2094006272;
	// 832A5514: 90CB0010  stw r6, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[6].u32 ) };
	// 832A5518: 3CC08330  lis r6, -0x7cd0
	ctx.r[6].s64 = -2094006272;
	// 832A551C: 3FA08330  lis r29, -0x7cd0
	ctx.r[29].s64 = -2094006272;
	// 832A5520: 8083F6C0  lwz r4, -0x940(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-2368 as u32) ) } as u64;
	// 832A5524: 8389F6CC  lwz r28, -0x934(r9)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-2356 as u32) ) } as u64;
	// 832A5528: 3F008330  lis r24, -0x7cd0
	ctx.r[24].s64 = -2094006272;
	// 832A552C: 8368F6D0  lwz r27, -0x930(r8)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-2352 as u32) ) } as u64;
	// 832A5530: 3EE08330  lis r23, -0x7cd0
	ctx.r[23].s64 = -2094006272;
	// 832A5534: 8347F6D4  lwz r26, -0x92c(r7)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(-2348 as u32) ) } as u64;
	// 832A5538: 3EC08330  lis r22, -0x7cd0
	ctx.r[22].s64 = -2094006272;
	// 832A553C: 8326F6D8  lwz r25, -0x928(r6)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-2344 as u32) ) } as u64;
	// 832A5540: 3EA08330  lis r21, -0x7cd0
	ctx.r[21].s64 = -2094006272;
	// 832A5544: 3E808330  lis r20, -0x7cd0
	ctx.r[20].s64 = -2094006272;
	// 832A5548: 807FF6C8  lwz r3, -0x938(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-2360 as u32) ) } as u64;
	// 832A554C: 3D208330  lis r9, -0x7cd0
	ctx.r[9].s64 = -2094006272;
	// 832A5550: 83FEF6B4  lwz r31, -0x94c(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-2380 as u32) ) } as u64;
	// 832A5554: 3D008330  lis r8, -0x7cd0
	ctx.r[8].s64 = -2094006272;
	// 832A5558: 83CAF6BC  lwz r30, -0x944(r10)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-2372 as u32) ) } as u64;
	// 832A555C: 3CE08330  lis r7, -0x7cd0
	ctx.r[7].s64 = -2094006272;
	// 832A5560: 83BDF6C4  lwz r29, -0x93c(r29)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-2364 as u32) ) } as u64;
	// 832A5564: 3CC08330  lis r6, -0x7cd0
	ctx.r[6].s64 = -2094006272;
	// 832A5568: 8318F6E0  lwz r24, -0x920(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-2336 as u32) ) } as u64;
	// 832A556C: 394B0060  addi r10, r11, 0x60
	ctx.r[10].s64 = ctx.r[11].s64 + 96;
	// 832A5570: 82F7F6E4  lwz r23, -0x91c(r23)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(-2332 as u32) ) } as u64;
	// 832A5574: 82D6F6EC  lwz r22, -0x914(r22)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-2324 as u32) ) } as u64;
	// 832A5578: 3A600000  li r19, 0
	ctx.r[19].s64 = 0;
	// 832A557C: 82B5F6F4  lwz r21, -0x90c(r21)
	ctx.r[21].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-2316 as u32) ) } as u64;
	// 832A5580: 8154F6FC  lwz r10, -0x904(r20)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(-2308 as u32) ) } as u64;
	// 832A5584: 8129F6E8  lwz r9, -0x918(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-2328 as u32) ) } as u64;
	// 832A5588: 8108F6F0  lwz r8, -0x910(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-2320 as u32) ) } as u64;
	// 832A558C: 80E7F6F8  lwz r7, -0x908(r7)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(-2312 as u32) ) } as u64;
	// 832A5590: 80C6F700  lwz r6, -0x900(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-2304 as u32) ) } as u64;
	// 832A5594: 90AB0014  stw r5, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[5].u32 ) };
	// 832A5598: 908B0018  stw r4, 0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[4].u32 ) };
	// 832A559C: 906B001C  stw r3, 0x1c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[3].u32 ) };
	// 832A55A0: 93EB0020  stw r31, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[31].u32 ) };
	// 832A55A4: 93CB0024  stw r30, 0x24(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[30].u32 ) };
	// 832A55A8: 93AB0028  stw r29, 0x28(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[29].u32 ) };
	// 832A55AC: 938B002C  stw r28, 0x2c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), ctx.r[28].u32 ) };
	// 832A55B0: 936B0030  stw r27, 0x30(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(48 as u32), ctx.r[27].u32 ) };
	// 832A55B4: 934B0034  stw r26, 0x34(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(52 as u32), ctx.r[26].u32 ) };
	// 832A55B8: 932B0038  stw r25, 0x38(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(56 as u32), ctx.r[25].u32 ) };
	// 832A55BC: 930B003C  stw r24, 0x3c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(60 as u32), ctx.r[24].u32 ) };
	// 832A55C0: 92EB0040  stw r23, 0x40(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), ctx.r[23].u32 ) };
	// 832A55C4: 92CB0044  stw r22, 0x44(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(68 as u32), ctx.r[22].u32 ) };
	// 832A55C8: 92AB0048  stw r21, 0x48(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), ctx.r[21].u32 ) };
	// 832A55CC: 914B004C  stw r10, 0x4c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(76 as u32), ctx.r[10].u32 ) };
	// 832A55D0: 912B0050  stw r9, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 832A55D4: 910B0054  stw r8, 0x54(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A55D8: 90EB0058  stw r7, 0x58(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 832A55DC: 90CB005C  stw r6, 0x5c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 832A55E0: 926B0060  stw r19, 0x60(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(96 as u32), ctx.r[19].u32 ) };
	// 832A55E4: 4BA03E50  b 0x82ca9434
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A55E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A55E8 size=56
    let mut pc: u32 = 0x832A55E8;
    'dispatch: loop {
        match pc {
            0x832A55E8 => {
    //   block [0x832A55E8..0x832A5620)
	// 832A55E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A55EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A55F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A55F4: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 832A55F8: 396BF784  addi r11, r11, -0x87c
	ctx.r[11].s64 = ctx.r[11].s64 + -2172;
	// 832A55FC: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 832A5600: 48014685  bl 0x832b9c84
	ctx.lr = 0x832A5604;
	// extern call 0x832B9C84 → crate::xboxkrnl::RtlInitializeCriticalSection
	crate::xboxkrnl::RtlInitializeCriticalSection(ctx, base);
	// 832A5604: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5608: 386B9358  addi r3, r11, -0x6ca8
	ctx.r[3].s64 = ctx.r[11].s64 + -27816;
	// 832A560C: 4BA04915  bl 0x82ca9f20
	ctx.lr = 0x832A5610;
	sub_82CA9F20(ctx, base);
	// 832A5610: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5614: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5618: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A561C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5620 size=96
    let mut pc: u32 = 0x832A5620;
    'dispatch: loop {
        match pc {
            0x832A5620 => {
    //   block [0x832A5620..0x832A5680)
	// 832A5620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5628: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A562C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5630: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5634: 3BEB9780  addi r31, r11, -0x6880
	ctx.r[31].s64 = ctx.r[11].s64 + -26752;
	// 832A5638: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 832A563C: 4BD5D3E5  bl 0x83002a20
	ctx.lr = 0x832A5640;
	sub_83002A20(ctx, base);
	// 832A5640: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A5644: 48014641  bl 0x832b9c84
	ctx.lr = 0x832A5648;
	// extern call 0x832B9C84 → crate::xboxkrnl::RtlInitializeCriticalSection
	crate::xboxkrnl::RtlInitializeCriticalSection(ctx, base);
	// 832A5648: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A564C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A5650: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A5654: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 832A5658: 915F002C  stw r10, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[10].u32 ) };
	// 832A565C: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5660: 913F0030  stw r9, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[9].u32 ) };
	// 832A5664: 386B9360  addi r3, r11, -0x6ca0
	ctx.r[3].s64 = ctx.r[11].s64 + -27808;
	// 832A5668: 4BA048B9  bl 0x82ca9f20
	ctx.lr = 0x832A566C;
	sub_82CA9F20(ctx, base);
	// 832A566C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5670: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5674: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5678: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A567C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5680 size=52
    let mut pc: u32 = 0x832A5680;
    'dispatch: loop {
        match pc {
            0x832A5680 => {
    //   block [0x832A5680..0x832A56B4)
	// 832A5680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5688: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A568C: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 832A5690: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A5694: 388BB320  addi r4, r11, -0x4ce0
	ctx.r[4].s64 = ctx.r[11].s64 + -19680;
	// 832A5698: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A569C: 386B97C0  addi r3, r11, -0x6840
	ctx.r[3].s64 = ctx.r[11].s64 + -26688;
	// 832A56A0: 4BD5DEF9  bl 0x83003598
	ctx.lr = 0x832A56A4;
	sub_83003598(ctx, base);
	// 832A56A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A56A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A56AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A56B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A56B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A56B8 size=52
    let mut pc: u32 = 0x832A56B8;
    'dispatch: loop {
        match pc {
            0x832A56B8 => {
    //   block [0x832A56B8..0x832A56EC)
	// 832A56B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A56BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A56C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A56C4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 832A56C8: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A56CC: 388BB38C  addi r4, r11, -0x4c74
	ctx.r[4].s64 = ctx.r[11].s64 + -19572;
	// 832A56D0: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A56D4: 386B97C8  addi r3, r11, -0x6838
	ctx.r[3].s64 = ctx.r[11].s64 + -26680;
	// 832A56D8: 4BD5DEC1  bl 0x83003598
	ctx.lr = 0x832A56DC;
	sub_83003598(ctx, base);
	// 832A56DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A56E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A56E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A56E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A56F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A56F0 size=52
    let mut pc: u32 = 0x832A56F0;
    'dispatch: loop {
        match pc {
            0x832A56F0 => {
    //   block [0x832A56F0..0x832A5724)
	// 832A56F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A56F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A56F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A56FC: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 832A5700: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A5704: 388BB778  addi r4, r11, -0x4888
	ctx.r[4].s64 = ctx.r[11].s64 + -18568;
	// 832A5708: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A570C: 386B97D0  addi r3, r11, -0x6830
	ctx.r[3].s64 = ctx.r[11].s64 + -26672;
	// 832A5710: 4BD5DE89  bl 0x83003598
	ctx.lr = 0x832A5714;
	sub_83003598(ctx, base);
	// 832A5714: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5718: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A571C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5720: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5728 size=52
    let mut pc: u32 = 0x832A5728;
    'dispatch: loop {
        match pc {
            0x832A5728 => {
    //   block [0x832A5728..0x832A575C)
	// 832A5728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A572C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5730: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5734: 38A00009  li r5, 9
	ctx.r[5].s64 = 9;
	// 832A5738: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A573C: 388BB808  addi r4, r11, -0x47f8
	ctx.r[4].s64 = ctx.r[11].s64 + -18424;
	// 832A5740: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5744: 386B97D8  addi r3, r11, -0x6828
	ctx.r[3].s64 = ctx.r[11].s64 + -26664;
	// 832A5748: 4BD5DE51  bl 0x83003598
	ctx.lr = 0x832A574C;
	sub_83003598(ctx, base);
	// 832A574C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5750: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5754: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5758: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5760 size=52
    let mut pc: u32 = 0x832A5760;
    'dispatch: loop {
        match pc {
            0x832A5760 => {
    //   block [0x832A5760..0x832A5794)
	// 832A5760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5768: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A576C: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 832A5770: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A5774: 388BB970  addi r4, r11, -0x4690
	ctx.r[4].s64 = ctx.r[11].s64 + -18064;
	// 832A5778: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A577C: 386B97E0  addi r3, r11, -0x6820
	ctx.r[3].s64 = ctx.r[11].s64 + -26656;
	// 832A5780: 4BD5DE19  bl 0x83003598
	ctx.lr = 0x832A5784;
	sub_83003598(ctx, base);
	// 832A5784: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5788: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A578C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5790: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5798 size=52
    let mut pc: u32 = 0x832A5798;
    'dispatch: loop {
        match pc {
            0x832A5798 => {
    //   block [0x832A5798..0x832A57CC)
	// 832A5798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A579C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A57A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A57A4: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 832A57A8: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A57AC: 388BB9B8  addi r4, r11, -0x4648
	ctx.r[4].s64 = ctx.r[11].s64 + -17992;
	// 832A57B0: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A57B4: 386B97E8  addi r3, r11, -0x6818
	ctx.r[3].s64 = ctx.r[11].s64 + -26648;
	// 832A57B8: 4BD5DDE1  bl 0x83003598
	ctx.lr = 0x832A57BC;
	sub_83003598(ctx, base);
	// 832A57BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A57C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A57C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A57C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A57D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A57D0 size=52
    let mut pc: u32 = 0x832A57D0;
    'dispatch: loop {
        match pc {
            0x832A57D0 => {
    //   block [0x832A57D0..0x832A5804)
	// 832A57D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A57D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A57D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A57DC: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 832A57E0: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A57E4: 388BBA48  addi r4, r11, -0x45b8
	ctx.r[4].s64 = ctx.r[11].s64 + -17848;
	// 832A57E8: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A57EC: 386B97F0  addi r3, r11, -0x6810
	ctx.r[3].s64 = ctx.r[11].s64 + -26640;
	// 832A57F0: 4BD5DDA9  bl 0x83003598
	ctx.lr = 0x832A57F4;
	sub_83003598(ctx, base);
	// 832A57F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A57F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A57FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5800: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5808 size=52
    let mut pc: u32 = 0x832A5808;
    'dispatch: loop {
        match pc {
            0x832A5808 => {
    //   block [0x832A5808..0x832A583C)
	// 832A5808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A580C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5810: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5814: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 832A5818: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A581C: 388BBAD8  addi r4, r11, -0x4528
	ctx.r[4].s64 = ctx.r[11].s64 + -17704;
	// 832A5820: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5824: 386B97F8  addi r3, r11, -0x6808
	ctx.r[3].s64 = ctx.r[11].s64 + -26632;
	// 832A5828: 4BD5DD71  bl 0x83003598
	ctx.lr = 0x832A582C;
	sub_83003598(ctx, base);
	// 832A582C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5830: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5834: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5838: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5840 size=52
    let mut pc: u32 = 0x832A5840;
    'dispatch: loop {
        match pc {
            0x832A5840 => {
    //   block [0x832A5840..0x832A5874)
	// 832A5840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5848: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A584C: 38A0000B  li r5, 0xb
	ctx.r[5].s64 = 11;
	// 832A5850: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A5854: 388BBB68  addi r4, r11, -0x4498
	ctx.r[4].s64 = ctx.r[11].s64 + -17560;
	// 832A5858: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A585C: 386B9800  addi r3, r11, -0x6800
	ctx.r[3].s64 = ctx.r[11].s64 + -26624;
	// 832A5860: 4BD5DD39  bl 0x83003598
	ctx.lr = 0x832A5864;
	sub_83003598(ctx, base);
	// 832A5864: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5868: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A586C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5870: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5878 size=52
    let mut pc: u32 = 0x832A5878;
    'dispatch: loop {
        match pc {
            0x832A5878 => {
    //   block [0x832A5878..0x832A58AC)
	// 832A5878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A587C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5880: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5884: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 832A5888: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A588C: 388BBD18  addi r4, r11, -0x42e8
	ctx.r[4].s64 = ctx.r[11].s64 + -17128;
	// 832A5890: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5894: 386B9808  addi r3, r11, -0x67f8
	ctx.r[3].s64 = ctx.r[11].s64 + -26616;
	// 832A5898: 4BD5DD01  bl 0x83003598
	ctx.lr = 0x832A589C;
	sub_83003598(ctx, base);
	// 832A589C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A58A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A58A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A58A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A58B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A58B0 size=52
    let mut pc: u32 = 0x832A58B0;
    'dispatch: loop {
        match pc {
            0x832A58B0 => {
    //   block [0x832A58B0..0x832A58E4)
	// 832A58B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A58B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A58B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A58BC: 38A0000B  li r5, 0xb
	ctx.r[5].s64 = 11;
	// 832A58C0: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A58C4: 388BBDF0  addi r4, r11, -0x4210
	ctx.r[4].s64 = ctx.r[11].s64 + -16912;
	// 832A58C8: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A58CC: 386B9810  addi r3, r11, -0x67f0
	ctx.r[3].s64 = ctx.r[11].s64 + -26608;
	// 832A58D0: 4BD5DCC9  bl 0x83003598
	ctx.lr = 0x832A58D4;
	sub_83003598(ctx, base);
	// 832A58D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A58D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A58DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A58E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A58E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A58E8 size=52
    let mut pc: u32 = 0x832A58E8;
    'dispatch: loop {
        match pc {
            0x832A58E8 => {
    //   block [0x832A58E8..0x832A591C)
	// 832A58E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A58EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A58F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A58F4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 832A58F8: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A58FC: 388BBF7C  addi r4, r11, -0x4084
	ctx.r[4].s64 = ctx.r[11].s64 + -16516;
	// 832A5900: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5904: 386B9818  addi r3, r11, -0x67e8
	ctx.r[3].s64 = ctx.r[11].s64 + -26600;
	// 832A5908: 4BD5DC91  bl 0x83003598
	ctx.lr = 0x832A590C;
	sub_83003598(ctx, base);
	// 832A590C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5910: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5914: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5918: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5920 size=52
    let mut pc: u32 = 0x832A5920;
    'dispatch: loop {
        match pc {
            0x832A5920 => {
    //   block [0x832A5920..0x832A5954)
	// 832A5920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5928: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A592C: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 832A5930: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A5934: 388BBFE8  addi r4, r11, -0x4018
	ctx.r[4].s64 = ctx.r[11].s64 + -16408;
	// 832A5938: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A593C: 386B9820  addi r3, r11, -0x67e0
	ctx.r[3].s64 = ctx.r[11].s64 + -26592;
	// 832A5940: 4BD5DC59  bl 0x83003598
	ctx.lr = 0x832A5944;
	sub_83003598(ctx, base);
	// 832A5944: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5948: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A594C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5950: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5958 size=52
    let mut pc: u32 = 0x832A5958;
    'dispatch: loop {
        match pc {
            0x832A5958 => {
    //   block [0x832A5958..0x832A598C)
	// 832A5958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A595C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5960: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5964: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 832A5968: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A596C: 388BC030  addi r4, r11, -0x3fd0
	ctx.r[4].s64 = ctx.r[11].s64 + -16336;
	// 832A5970: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5974: 386B9828  addi r3, r11, -0x67d8
	ctx.r[3].s64 = ctx.r[11].s64 + -26584;
	// 832A5978: 4BD5DC21  bl 0x83003598
	ctx.lr = 0x832A597C;
	sub_83003598(ctx, base);
	// 832A597C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5980: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5984: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5988: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5990 size=44
    let mut pc: u32 = 0x832A5990;
    'dispatch: loop {
        match pc {
            0x832A5990 => {
    //   block [0x832A5990..0x832A59BC)
	// 832A5990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5998: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A599C: 4BD5DBA5  bl 0x83003540
	ctx.lr = 0x832A59A0;
	sub_83003540(ctx, base);
	// 832A59A0: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A59A4: 396B9830  addi r11, r11, -0x67d0
	ctx.r[11].s64 = ctx.r[11].s64 + -26576;
	// 832A59A8: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 832A59AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A59B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A59B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A59B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A59C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A59C0 size=52
    let mut pc: u32 = 0x832A59C0;
    'dispatch: loop {
        match pc {
            0x832A59C0 => {
    //   block [0x832A59C0..0x832A59F4)
	// 832A59C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A59C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A59C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A59CC: 38A0000A  li r5, 0xa
	ctx.r[5].s64 = 10;
	// 832A59D0: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A59D4: 388BC278  addi r4, r11, -0x3d88
	ctx.r[4].s64 = ctx.r[11].s64 + -15752;
	// 832A59D8: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A59DC: 386B9834  addi r3, r11, -0x67cc
	ctx.r[3].s64 = ctx.r[11].s64 + -26572;
	// 832A59E0: 4BD5DBB9  bl 0x83003598
	ctx.lr = 0x832A59E4;
	sub_83003598(ctx, base);
	// 832A59E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A59E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A59EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A59F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A59F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A59F8 size=52
    let mut pc: u32 = 0x832A59F8;
    'dispatch: loop {
        match pc {
            0x832A59F8 => {
    //   block [0x832A59F8..0x832A5A2C)
	// 832A59F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A59FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5A00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5A04: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5A08: 386B983C  addi r3, r11, -0x67c4
	ctx.r[3].s64 = ctx.r[11].s64 + -26564;
	// 832A5A0C: 4BD5F6E5  bl 0x830050f0
	ctx.lr = 0x832A5A10;
	sub_830050F0(ctx, base);
	// 832A5A10: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5A14: 386B9370  addi r3, r11, -0x6c90
	ctx.r[3].s64 = ctx.r[11].s64 + -27792;
	// 832A5A18: 4BA04509  bl 0x82ca9f20
	ctx.lr = 0x832A5A1C;
	sub_82CA9F20(ctx, base);
	// 832A5A1C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5A20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5A24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5A28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5A30 size=52
    let mut pc: u32 = 0x832A5A30;
    'dispatch: loop {
        match pc {
            0x832A5A30 => {
    //   block [0x832A5A30..0x832A5A64)
	// 832A5A30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5A34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5A38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5A3C: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5A40: 386B9870  addi r3, r11, -0x6790
	ctx.r[3].s64 = ctx.r[11].s64 + -26512;
	// 832A5A44: 4BD5DBE5  bl 0x83003628
	ctx.lr = 0x832A5A48;
	sub_83003628(ctx, base);
	// 832A5A48: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5A4C: 386B9398  addi r3, r11, -0x6c68
	ctx.r[3].s64 = ctx.r[11].s64 + -27752;
	// 832A5A50: 4BA044D1  bl 0x82ca9f20
	ctx.lr = 0x832A5A54;
	sub_82CA9F20(ctx, base);
	// 832A5A54: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5A58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5A5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5A60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5A68 size=52
    let mut pc: u32 = 0x832A5A68;
    'dispatch: loop {
        match pc {
            0x832A5A68 => {
    //   block [0x832A5A68..0x832A5A9C)
	// 832A5A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5A6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5A70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5A74: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5A78: 386B9864  addi r3, r11, -0x679c
	ctx.r[3].s64 = ctx.r[11].s64 + -26524;
	// 832A5A7C: 4BD5EDED  bl 0x83004868
	ctx.lr = 0x832A5A80;
	sub_83004868(ctx, base);
	// 832A5A80: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5A84: 386B93C0  addi r3, r11, -0x6c40
	ctx.r[3].s64 = ctx.r[11].s64 + -27712;
	// 832A5A88: 4BA04499  bl 0x82ca9f20
	ctx.lr = 0x832A5A8C;
	sub_82CA9F20(ctx, base);
	// 832A5A8C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5A90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5A94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5A98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5AA0 size=52
    let mut pc: u32 = 0x832A5AA0;
    'dispatch: loop {
        match pc {
            0x832A5AA0 => {
    //   block [0x832A5AA0..0x832A5AD4)
	// 832A5AA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5AA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5AA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5AAC: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5AB0: 386B98A0  addi r3, r11, -0x6760
	ctx.r[3].s64 = ctx.r[11].s64 + -26464;
	// 832A5AB4: 4BD5DB75  bl 0x83003628
	ctx.lr = 0x832A5AB8;
	sub_83003628(ctx, base);
	// 832A5AB8: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5ABC: 386B93E8  addi r3, r11, -0x6c18
	ctx.r[3].s64 = ctx.r[11].s64 + -27672;
	// 832A5AC0: 4BA04461  bl 0x82ca9f20
	ctx.lr = 0x832A5AC4;
	sub_82CA9F20(ctx, base);
	// 832A5AC4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5AC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5ACC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5AD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5AD8 size=52
    let mut pc: u32 = 0x832A5AD8;
    'dispatch: loop {
        match pc {
            0x832A5AD8 => {
    //   block [0x832A5AD8..0x832A5B0C)
	// 832A5AD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5ADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5AE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5AE4: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5AE8: 386B9894  addi r3, r11, -0x676c
	ctx.r[3].s64 = ctx.r[11].s64 + -26476;
	// 832A5AEC: 4BD5ED7D  bl 0x83004868
	ctx.lr = 0x832A5AF0;
	sub_83004868(ctx, base);
	// 832A5AF0: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5AF4: 386B9410  addi r3, r11, -0x6bf0
	ctx.r[3].s64 = ctx.r[11].s64 + -27632;
	// 832A5AF8: 4BA04429  bl 0x82ca9f20
	ctx.lr = 0x832A5AFC;
	sub_82CA9F20(ctx, base);
	// 832A5AFC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5B00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5B04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5B08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5B10 size=52
    let mut pc: u32 = 0x832A5B10;
    'dispatch: loop {
        match pc {
            0x832A5B10 => {
    //   block [0x832A5B10..0x832A5B44)
	// 832A5B10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5B14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5B18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5B1C: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5B20: 386B98D0  addi r3, r11, -0x6730
	ctx.r[3].s64 = ctx.r[11].s64 + -26416;
	// 832A5B24: 4BD5DB05  bl 0x83003628
	ctx.lr = 0x832A5B28;
	sub_83003628(ctx, base);
	// 832A5B28: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5B2C: 386B9438  addi r3, r11, -0x6bc8
	ctx.r[3].s64 = ctx.r[11].s64 + -27592;
	// 832A5B30: 4BA043F1  bl 0x82ca9f20
	ctx.lr = 0x832A5B34;
	sub_82CA9F20(ctx, base);
	// 832A5B34: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5B38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5B3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5B40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5B48 size=52
    let mut pc: u32 = 0x832A5B48;
    'dispatch: loop {
        match pc {
            0x832A5B48 => {
    //   block [0x832A5B48..0x832A5B7C)
	// 832A5B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5B50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5B54: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5B58: 386B98C4  addi r3, r11, -0x673c
	ctx.r[3].s64 = ctx.r[11].s64 + -26428;
	// 832A5B5C: 4BD5ED0D  bl 0x83004868
	ctx.lr = 0x832A5B60;
	sub_83004868(ctx, base);
	// 832A5B60: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5B64: 386B9460  addi r3, r11, -0x6ba0
	ctx.r[3].s64 = ctx.r[11].s64 + -27552;
	// 832A5B68: 4BA043B9  bl 0x82ca9f20
	ctx.lr = 0x832A5B6C;
	sub_82CA9F20(ctx, base);
	// 832A5B6C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5B70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5B74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5B78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5B80 size=52
    let mut pc: u32 = 0x832A5B80;
    'dispatch: loop {
        match pc {
            0x832A5B80 => {
    //   block [0x832A5B80..0x832A5BB4)
	// 832A5B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5B84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5B88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5B8C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 832A5B90: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A5B94: 388BC490  addi r4, r11, -0x3b70
	ctx.r[4].s64 = ctx.r[11].s64 + -15216;
	// 832A5B98: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5B9C: 386B98F4  addi r3, r11, -0x670c
	ctx.r[3].s64 = ctx.r[11].s64 + -26380;
	// 832A5BA0: 4BD5D9F9  bl 0x83003598
	ctx.lr = 0x832A5BA4;
	sub_83003598(ctx, base);
	// 832A5BA4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5BA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5BAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5BB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5BB8 size=52
    let mut pc: u32 = 0x832A5BB8;
    'dispatch: loop {
        match pc {
            0x832A5BB8 => {
    //   block [0x832A5BB8..0x832A5BEC)
	// 832A5BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5BBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5BC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5BC4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 832A5BC8: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A5BCC: 388BC4B4  addi r4, r11, -0x3b4c
	ctx.r[4].s64 = ctx.r[11].s64 + -15180;
	// 832A5BD0: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5BD4: 386B98FC  addi r3, r11, -0x6704
	ctx.r[3].s64 = ctx.r[11].s64 + -26372;
	// 832A5BD8: 4BD5D9C1  bl 0x83003598
	ctx.lr = 0x832A5BDC;
	sub_83003598(ctx, base);
	// 832A5BDC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5BE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5BE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5BE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5BF0 size=52
    let mut pc: u32 = 0x832A5BF0;
    'dispatch: loop {
        match pc {
            0x832A5BF0 => {
    //   block [0x832A5BF0..0x832A5C24)
	// 832A5BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5BF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5BF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5BFC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 832A5C00: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A5C04: 388BC4D8  addi r4, r11, -0x3b28
	ctx.r[4].s64 = ctx.r[11].s64 + -15144;
	// 832A5C08: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5C0C: 386B9904  addi r3, r11, -0x66fc
	ctx.r[3].s64 = ctx.r[11].s64 + -26364;
	// 832A5C10: 4BD5D989  bl 0x83003598
	ctx.lr = 0x832A5C14;
	sub_83003598(ctx, base);
	// 832A5C14: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5C18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5C1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5C20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5C28 size=52
    let mut pc: u32 = 0x832A5C28;
    'dispatch: loop {
        match pc {
            0x832A5C28 => {
    //   block [0x832A5C28..0x832A5C5C)
	// 832A5C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5C2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5C30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5C34: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 832A5C38: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A5C3C: 388BC4FC  addi r4, r11, -0x3b04
	ctx.r[4].s64 = ctx.r[11].s64 + -15108;
	// 832A5C40: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5C44: 386B990C  addi r3, r11, -0x66f4
	ctx.r[3].s64 = ctx.r[11].s64 + -26356;
	// 832A5C48: 4BD5D951  bl 0x83003598
	ctx.lr = 0x832A5C4C;
	sub_83003598(ctx, base);
	// 832A5C4C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5C50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5C54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5C58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5C60 size=52
    let mut pc: u32 = 0x832A5C60;
    'dispatch: loop {
        match pc {
            0x832A5C60 => {
    //   block [0x832A5C60..0x832A5C94)
	// 832A5C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5C64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5C68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5C6C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 832A5C70: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A5C74: 388BC520  addi r4, r11, -0x3ae0
	ctx.r[4].s64 = ctx.r[11].s64 + -15072;
	// 832A5C78: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5C7C: 386B9914  addi r3, r11, -0x66ec
	ctx.r[3].s64 = ctx.r[11].s64 + -26348;
	// 832A5C80: 4BD5D919  bl 0x83003598
	ctx.lr = 0x832A5C84;
	sub_83003598(ctx, base);
	// 832A5C84: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5C88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5C8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5C90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5C98 size=52
    let mut pc: u32 = 0x832A5C98;
    'dispatch: loop {
        match pc {
            0x832A5C98 => {
    //   block [0x832A5C98..0x832A5CCC)
	// 832A5C98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5C9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5CA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5CA4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 832A5CA8: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A5CAC: 388BC544  addi r4, r11, -0x3abc
	ctx.r[4].s64 = ctx.r[11].s64 + -15036;
	// 832A5CB0: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5CB4: 386B991C  addi r3, r11, -0x66e4
	ctx.r[3].s64 = ctx.r[11].s64 + -26340;
	// 832A5CB8: 4BD5D8E1  bl 0x83003598
	ctx.lr = 0x832A5CBC;
	sub_83003598(ctx, base);
	// 832A5CBC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5CC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5CC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5CC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5CD0 size=52
    let mut pc: u32 = 0x832A5CD0;
    'dispatch: loop {
        match pc {
            0x832A5CD0 => {
    //   block [0x832A5CD0..0x832A5D04)
	// 832A5CD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5CD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5CD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5CDC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 832A5CE0: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A5CE4: 388BC568  addi r4, r11, -0x3a98
	ctx.r[4].s64 = ctx.r[11].s64 + -15000;
	// 832A5CE8: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5CEC: 386B9924  addi r3, r11, -0x66dc
	ctx.r[3].s64 = ctx.r[11].s64 + -26332;
	// 832A5CF0: 4BD5D8A9  bl 0x83003598
	ctx.lr = 0x832A5CF4;
	sub_83003598(ctx, base);
	// 832A5CF4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5CF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5CFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5D00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5D08 size=52
    let mut pc: u32 = 0x832A5D08;
    'dispatch: loop {
        match pc {
            0x832A5D08 => {
    //   block [0x832A5D08..0x832A5D3C)
	// 832A5D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5D0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5D10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5D14: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 832A5D18: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A5D1C: 388BC58C  addi r4, r11, -0x3a74
	ctx.r[4].s64 = ctx.r[11].s64 + -14964;
	// 832A5D20: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5D24: 386B992C  addi r3, r11, -0x66d4
	ctx.r[3].s64 = ctx.r[11].s64 + -26324;
	// 832A5D28: 4BD5D871  bl 0x83003598
	ctx.lr = 0x832A5D2C;
	sub_83003598(ctx, base);
	// 832A5D2C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5D30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5D34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5D38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5D40 size=52
    let mut pc: u32 = 0x832A5D40;
    'dispatch: loop {
        match pc {
            0x832A5D40 => {
    //   block [0x832A5D40..0x832A5D74)
	// 832A5D40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5D44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5D48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5D4C: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5D50: 386B9940  addi r3, r11, -0x66c0
	ctx.r[3].s64 = ctx.r[11].s64 + -26304;
	// 832A5D54: 4BD5D8D5  bl 0x83003628
	ctx.lr = 0x832A5D58;
	sub_83003628(ctx, base);
	// 832A5D58: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5D5C: 386B9488  addi r3, r11, -0x6b78
	ctx.r[3].s64 = ctx.r[11].s64 + -27512;
	// 832A5D60: 4BA041C1  bl 0x82ca9f20
	ctx.lr = 0x832A5D64;
	sub_82CA9F20(ctx, base);
	// 832A5D64: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5D68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5D6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5D70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5D78 size=52
    let mut pc: u32 = 0x832A5D78;
    'dispatch: loop {
        match pc {
            0x832A5D78 => {
    //   block [0x832A5D78..0x832A5DAC)
	// 832A5D78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5D7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5D80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5D84: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5D88: 386B9934  addi r3, r11, -0x66cc
	ctx.r[3].s64 = ctx.r[11].s64 + -26316;
	// 832A5D8C: 4BD5EADD  bl 0x83004868
	ctx.lr = 0x832A5D90;
	sub_83004868(ctx, base);
	// 832A5D90: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5D94: 386B94B0  addi r3, r11, -0x6b50
	ctx.r[3].s64 = ctx.r[11].s64 + -27472;
	// 832A5D98: 4BA04189  bl 0x82ca9f20
	ctx.lr = 0x832A5D9C;
	sub_82CA9F20(ctx, base);
	// 832A5D9C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5DA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5DA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5DA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5DB0 size=52
    let mut pc: u32 = 0x832A5DB0;
    'dispatch: loop {
        match pc {
            0x832A5DB0 => {
    //   block [0x832A5DB0..0x832A5DE4)
	// 832A5DB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5DB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5DB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5DBC: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5DC0: 386B9A00  addi r3, r11, -0x6600
	ctx.r[3].s64 = ctx.r[11].s64 + -26112;
	// 832A5DC4: 4BD5D865  bl 0x83003628
	ctx.lr = 0x832A5DC8;
	sub_83003628(ctx, base);
	// 832A5DC8: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5DCC: 386B94D8  addi r3, r11, -0x6b28
	ctx.r[3].s64 = ctx.r[11].s64 + -27432;
	// 832A5DD0: 4BA04151  bl 0x82ca9f20
	ctx.lr = 0x832A5DD4;
	sub_82CA9F20(ctx, base);
	// 832A5DD4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5DD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5DDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5DE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5DE8 size=52
    let mut pc: u32 = 0x832A5DE8;
    'dispatch: loop {
        match pc {
            0x832A5DE8 => {
    //   block [0x832A5DE8..0x832A5E1C)
	// 832A5DE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5DEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5DF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5DF4: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5DF8: 386B99F4  addi r3, r11, -0x660c
	ctx.r[3].s64 = ctx.r[11].s64 + -26124;
	// 832A5DFC: 4BD5EA6D  bl 0x83004868
	ctx.lr = 0x832A5E00;
	sub_83004868(ctx, base);
	// 832A5E00: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5E04: 386B9500  addi r3, r11, -0x6b00
	ctx.r[3].s64 = ctx.r[11].s64 + -27392;
	// 832A5E08: 4BA04119  bl 0x82ca9f20
	ctx.lr = 0x832A5E0C;
	sub_82CA9F20(ctx, base);
	// 832A5E0C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5E10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5E14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5E18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5E20 size=52
    let mut pc: u32 = 0x832A5E20;
    'dispatch: loop {
        match pc {
            0x832A5E20 => {
    //   block [0x832A5E20..0x832A5E54)
	// 832A5E20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5E24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5E28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5E2C: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5E30: 386B99D0  addi r3, r11, -0x6630
	ctx.r[3].s64 = ctx.r[11].s64 + -26160;
	// 832A5E34: 4BD5D7F5  bl 0x83003628
	ctx.lr = 0x832A5E38;
	sub_83003628(ctx, base);
	// 832A5E38: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5E3C: 386B9528  addi r3, r11, -0x6ad8
	ctx.r[3].s64 = ctx.r[11].s64 + -27352;
	// 832A5E40: 4BA040E1  bl 0x82ca9f20
	ctx.lr = 0x832A5E44;
	sub_82CA9F20(ctx, base);
	// 832A5E44: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5E48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5E4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5E50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5E58 size=52
    let mut pc: u32 = 0x832A5E58;
    'dispatch: loop {
        match pc {
            0x832A5E58 => {
    //   block [0x832A5E58..0x832A5E8C)
	// 832A5E58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5E5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5E60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5E64: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5E68: 386B99C4  addi r3, r11, -0x663c
	ctx.r[3].s64 = ctx.r[11].s64 + -26172;
	// 832A5E6C: 4BD5E9FD  bl 0x83004868
	ctx.lr = 0x832A5E70;
	sub_83004868(ctx, base);
	// 832A5E70: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5E74: 386B9550  addi r3, r11, -0x6ab0
	ctx.r[3].s64 = ctx.r[11].s64 + -27312;
	// 832A5E78: 4BA040A9  bl 0x82ca9f20
	ctx.lr = 0x832A5E7C;
	sub_82CA9F20(ctx, base);
	// 832A5E7C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5E80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5E84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5E88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5E90 size=52
    let mut pc: u32 = 0x832A5E90;
    'dispatch: loop {
        match pc {
            0x832A5E90 => {
    //   block [0x832A5E90..0x832A5EC4)
	// 832A5E90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5E94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5E98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5E9C: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5EA0: 386B99A0  addi r3, r11, -0x6660
	ctx.r[3].s64 = ctx.r[11].s64 + -26208;
	// 832A5EA4: 4BD5D785  bl 0x83003628
	ctx.lr = 0x832A5EA8;
	sub_83003628(ctx, base);
	// 832A5EA8: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5EAC: 386B9578  addi r3, r11, -0x6a88
	ctx.r[3].s64 = ctx.r[11].s64 + -27272;
	// 832A5EB0: 4BA04071  bl 0x82ca9f20
	ctx.lr = 0x832A5EB4;
	sub_82CA9F20(ctx, base);
	// 832A5EB4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5EB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5EBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5EC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5EC8 size=52
    let mut pc: u32 = 0x832A5EC8;
    'dispatch: loop {
        match pc {
            0x832A5EC8 => {
    //   block [0x832A5EC8..0x832A5EFC)
	// 832A5EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5ECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5ED0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5ED4: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5ED8: 386B9994  addi r3, r11, -0x666c
	ctx.r[3].s64 = ctx.r[11].s64 + -26220;
	// 832A5EDC: 4BD5E98D  bl 0x83004868
	ctx.lr = 0x832A5EE0;
	sub_83004868(ctx, base);
	// 832A5EE0: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5EE4: 386B95A0  addi r3, r11, -0x6a60
	ctx.r[3].s64 = ctx.r[11].s64 + -27232;
	// 832A5EE8: 4BA04039  bl 0x82ca9f20
	ctx.lr = 0x832A5EEC;
	sub_82CA9F20(ctx, base);
	// 832A5EEC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5EF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5EF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5EF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5F00 size=52
    let mut pc: u32 = 0x832A5F00;
    'dispatch: loop {
        match pc {
            0x832A5F00 => {
    //   block [0x832A5F00..0x832A5F34)
	// 832A5F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5F04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5F08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5F0C: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5F10: 386B9970  addi r3, r11, -0x6690
	ctx.r[3].s64 = ctx.r[11].s64 + -26256;
	// 832A5F14: 4BD5D715  bl 0x83003628
	ctx.lr = 0x832A5F18;
	sub_83003628(ctx, base);
	// 832A5F18: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5F1C: 386B95C8  addi r3, r11, -0x6a38
	ctx.r[3].s64 = ctx.r[11].s64 + -27192;
	// 832A5F20: 4BA04001  bl 0x82ca9f20
	ctx.lr = 0x832A5F24;
	sub_82CA9F20(ctx, base);
	// 832A5F24: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5F28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5F2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5F30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5F38 size=52
    let mut pc: u32 = 0x832A5F38;
    'dispatch: loop {
        match pc {
            0x832A5F38 => {
    //   block [0x832A5F38..0x832A5F6C)
	// 832A5F38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5F3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5F40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5F44: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5F48: 386B9964  addi r3, r11, -0x669c
	ctx.r[3].s64 = ctx.r[11].s64 + -26268;
	// 832A5F4C: 4BD5E91D  bl 0x83004868
	ctx.lr = 0x832A5F50;
	sub_83004868(ctx, base);
	// 832A5F50: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5F54: 386B95F0  addi r3, r11, -0x6a10
	ctx.r[3].s64 = ctx.r[11].s64 + -27152;
	// 832A5F58: 4BA03FC9  bl 0x82ca9f20
	ctx.lr = 0x832A5F5C;
	sub_82CA9F20(ctx, base);
	// 832A5F5C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5F60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5F64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5F68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5F70 size=52
    let mut pc: u32 = 0x832A5F70;
    'dispatch: loop {
        match pc {
            0x832A5F70 => {
    //   block [0x832A5F70..0x832A5FA4)
	// 832A5F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5F74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5F78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5F7C: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5F80: 386B9A34  addi r3, r11, -0x65cc
	ctx.r[3].s64 = ctx.r[11].s64 + -26060;
	// 832A5F84: 4BD5D6A5  bl 0x83003628
	ctx.lr = 0x832A5F88;
	sub_83003628(ctx, base);
	// 832A5F88: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5F8C: 386B9618  addi r3, r11, -0x69e8
	ctx.r[3].s64 = ctx.r[11].s64 + -27112;
	// 832A5F90: 4BA03F91  bl 0x82ca9f20
	ctx.lr = 0x832A5F94;
	sub_82CA9F20(ctx, base);
	// 832A5F94: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5F98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5F9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5FA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5FA8 size=52
    let mut pc: u32 = 0x832A5FA8;
    'dispatch: loop {
        match pc {
            0x832A5FA8 => {
    //   block [0x832A5FA8..0x832A5FDC)
	// 832A5FA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5FAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5FB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5FB4: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5FB8: 386B9A28  addi r3, r11, -0x65d8
	ctx.r[3].s64 = ctx.r[11].s64 + -26072;
	// 832A5FBC: 4BD5E8AD  bl 0x83004868
	ctx.lr = 0x832A5FC0;
	sub_83004868(ctx, base);
	// 832A5FC0: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5FC4: 386B9640  addi r3, r11, -0x69c0
	ctx.r[3].s64 = ctx.r[11].s64 + -27072;
	// 832A5FC8: 4BA03F59  bl 0x82ca9f20
	ctx.lr = 0x832A5FCC;
	sub_82CA9F20(ctx, base);
	// 832A5FCC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5FD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5FD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5FD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5FE0 size=52
    let mut pc: u32 = 0x832A5FE0;
    'dispatch: loop {
        match pc {
            0x832A5FE0 => {
    //   block [0x832A5FE0..0x832A6014)
	// 832A5FE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5FE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5FE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5FEC: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5FF0: 386B9A64  addi r3, r11, -0x659c
	ctx.r[3].s64 = ctx.r[11].s64 + -26012;
	// 832A5FF4: 4BD5D635  bl 0x83003628
	ctx.lr = 0x832A5FF8;
	sub_83003628(ctx, base);
	// 832A5FF8: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5FFC: 386B9668  addi r3, r11, -0x6998
	ctx.r[3].s64 = ctx.r[11].s64 + -27032;
	// 832A6000: 4BA03F21  bl 0x82ca9f20
	ctx.lr = 0x832A6004;
	sub_82CA9F20(ctx, base);
	// 832A6004: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A6008: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A600C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6010: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6018 size=52
    let mut pc: u32 = 0x832A6018;
    'dispatch: loop {
        match pc {
            0x832A6018 => {
    //   block [0x832A6018..0x832A604C)
	// 832A6018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A601C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6020: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A6024: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A6028: 386B9A58  addi r3, r11, -0x65a8
	ctx.r[3].s64 = ctx.r[11].s64 + -26024;
	// 832A602C: 4BD5E83D  bl 0x83004868
	ctx.lr = 0x832A6030;
	sub_83004868(ctx, base);
	// 832A6030: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A6034: 386B9690  addi r3, r11, -0x6970
	ctx.r[3].s64 = ctx.r[11].s64 + -26992;
	// 832A6038: 4BA03EE9  bl 0x82ca9f20
	ctx.lr = 0x832A603C;
	sub_82CA9F20(ctx, base);
	// 832A603C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A6040: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6044: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6048: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6050 size=52
    let mut pc: u32 = 0x832A6050;
    'dispatch: loop {
        match pc {
            0x832A6050 => {
    //   block [0x832A6050..0x832A6084)
	// 832A6050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6058: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A605C: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A6060: 386B9AC4  addi r3, r11, -0x653c
	ctx.r[3].s64 = ctx.r[11].s64 + -25916;
	// 832A6064: 4BD5D5C5  bl 0x83003628
	ctx.lr = 0x832A6068;
	sub_83003628(ctx, base);
	// 832A6068: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A606C: 386B96B8  addi r3, r11, -0x6948
	ctx.r[3].s64 = ctx.r[11].s64 + -26952;
	// 832A6070: 4BA03EB1  bl 0x82ca9f20
	ctx.lr = 0x832A6074;
	sub_82CA9F20(ctx, base);
	// 832A6074: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A6078: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A607C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6080: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6088 size=52
    let mut pc: u32 = 0x832A6088;
    'dispatch: loop {
        match pc {
            0x832A6088 => {
    //   block [0x832A6088..0x832A60BC)
	// 832A6088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A608C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6090: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A6094: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A6098: 386B9AB8  addi r3, r11, -0x6548
	ctx.r[3].s64 = ctx.r[11].s64 + -25928;
	// 832A609C: 4BD5E7CD  bl 0x83004868
	ctx.lr = 0x832A60A0;
	sub_83004868(ctx, base);
	// 832A60A0: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A60A4: 386B96E0  addi r3, r11, -0x6920
	ctx.r[3].s64 = ctx.r[11].s64 + -26912;
	// 832A60A8: 4BA03E79  bl 0x82ca9f20
	ctx.lr = 0x832A60AC;
	sub_82CA9F20(ctx, base);
	// 832A60AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A60B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A60B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A60B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A60C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A60C0 size=52
    let mut pc: u32 = 0x832A60C0;
    'dispatch: loop {
        match pc {
            0x832A60C0 => {
    //   block [0x832A60C0..0x832A60F4)
	// 832A60C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A60C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A60C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A60CC: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A60D0: 386B9A94  addi r3, r11, -0x656c
	ctx.r[3].s64 = ctx.r[11].s64 + -25964;
	// 832A60D4: 4BD5D555  bl 0x83003628
	ctx.lr = 0x832A60D8;
	sub_83003628(ctx, base);
	// 832A60D8: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A60DC: 386B9708  addi r3, r11, -0x68f8
	ctx.r[3].s64 = ctx.r[11].s64 + -26872;
	// 832A60E0: 4BA03E41  bl 0x82ca9f20
	ctx.lr = 0x832A60E4;
	sub_82CA9F20(ctx, base);
	// 832A60E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A60E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A60EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A60F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A60F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A60F8 size=52
    let mut pc: u32 = 0x832A60F8;
    'dispatch: loop {
        match pc {
            0x832A60F8 => {
    //   block [0x832A60F8..0x832A612C)
	// 832A60F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A60FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6100: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A6104: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A6108: 386B9A88  addi r3, r11, -0x6578
	ctx.r[3].s64 = ctx.r[11].s64 + -25976;
	// 832A610C: 4BD5E75D  bl 0x83004868
	ctx.lr = 0x832A6110;
	sub_83004868(ctx, base);
	// 832A6110: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A6114: 386B9730  addi r3, r11, -0x68d0
	ctx.r[3].s64 = ctx.r[11].s64 + -26832;
	// 832A6118: 4BA03E09  bl 0x82ca9f20
	ctx.lr = 0x832A611C;
	sub_82CA9F20(ctx, base);
	// 832A611C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A6120: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6124: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6128: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6130 size=44
    let mut pc: u32 = 0x832A6130;
    'dispatch: loop {
        match pc {
            0x832A6130 => {
    //   block [0x832A6130..0x832A615C)
	// 832A6130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6138: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A613C: 4BD5EE5D  bl 0x83004f98
	ctx.lr = 0x832A6140;
	sub_83004F98(ctx, base);
	// 832A6140: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A6144: 396B9AE8  addi r11, r11, -0x6518
	ctx.r[11].s64 = ctx.r[11].s64 + -25880;
	// 832A6148: F86B0000  std r3, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u64 ) };
	// 832A614C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A6150: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6154: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6158: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6160 size=44
    let mut pc: u32 = 0x832A6160;
    'dispatch: loop {
        match pc {
            0x832A6160 => {
    //   block [0x832A6160..0x832A618C)
	// 832A6160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6168: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A616C: 4BD5EE5D  bl 0x83004fc8
	ctx.lr = 0x832A6170;
	sub_83004FC8(ctx, base);
	// 832A6170: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A6174: 396B9AF0  addi r11, r11, -0x6510
	ctx.r[11].s64 = ctx.r[11].s64 + -25872;
	// 832A6178: D82B0000  stfd f1, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.f[1].u64 ) };
	// 832A617C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A6180: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6184: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6188: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6190 size=52
    let mut pc: u32 = 0x832A6190;
    'dispatch: loop {
        match pc {
            0x832A6190 => {
    //   block [0x832A6190..0x832A61C4)
	// 832A6190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6198: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A619C: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 832A61A0: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A61A4: 388BD340  addi r4, r11, -0x2cc0
	ctx.r[4].s64 = ctx.r[11].s64 + -11456;
	// 832A61A8: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A61AC: 386B9AFC  addi r3, r11, -0x6504
	ctx.r[3].s64 = ctx.r[11].s64 + -25860;
	// 832A61B0: 4BD5D3E9  bl 0x83003598
	ctx.lr = 0x832A61B4;
	sub_83003598(ctx, base);
	// 832A61B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A61B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A61BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A61C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A61C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A61C8 size=52
    let mut pc: u32 = 0x832A61C8;
    'dispatch: loop {
        match pc {
            0x832A61C8 => {
    //   block [0x832A61C8..0x832A61FC)
	// 832A61C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A61CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A61D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A61D4: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 832A61D8: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A61DC: 388BD3D0  addi r4, r11, -0x2c30
	ctx.r[4].s64 = ctx.r[11].s64 + -11312;
	// 832A61E0: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A61E4: 386B9B04  addi r3, r11, -0x64fc
	ctx.r[3].s64 = ctx.r[11].s64 + -25852;
	// 832A61E8: 4BD5D3B1  bl 0x83003598
	ctx.lr = 0x832A61EC;
	sub_83003598(ctx, base);
	// 832A61EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A61F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A61F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A61F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6200 size=52
    let mut pc: u32 = 0x832A6200;
    'dispatch: loop {
        match pc {
            0x832A6200 => {
    //   block [0x832A6200..0x832A6234)
	// 832A6200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6208: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A620C: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 832A6210: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A6214: 388BD4C8  addi r4, r11, -0x2b38
	ctx.r[4].s64 = ctx.r[11].s64 + -11064;
	// 832A6218: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A621C: 386B9B0C  addi r3, r11, -0x64f4
	ctx.r[3].s64 = ctx.r[11].s64 + -25844;
	// 832A6220: 4BD5D379  bl 0x83003598
	ctx.lr = 0x832A6224;
	sub_83003598(ctx, base);
	// 832A6224: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A6228: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A622C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6230: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6238 size=52
    let mut pc: u32 = 0x832A6238;
    'dispatch: loop {
        match pc {
            0x832A6238 => {
    //   block [0x832A6238..0x832A626C)
	// 832A6238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A623C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6240: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A6244: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 832A6248: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A624C: 388BD558  addi r4, r11, -0x2aa8
	ctx.r[4].s64 = ctx.r[11].s64 + -10920;
	// 832A6250: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A6254: 386B9B14  addi r3, r11, -0x64ec
	ctx.r[3].s64 = ctx.r[11].s64 + -25836;
	// 832A6258: 4BD5D341  bl 0x83003598
	ctx.lr = 0x832A625C;
	sub_83003598(ctx, base);
	// 832A625C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A6260: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6264: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6268: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6270 size=52
    let mut pc: u32 = 0x832A6270;
    'dispatch: loop {
        match pc {
            0x832A6270 => {
    //   block [0x832A6270..0x832A62A4)
	// 832A6270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6278: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A627C: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A6280: 386B9B1C  addi r3, r11, -0x64e4
	ctx.r[3].s64 = ctx.r[11].s64 + -25828;
	// 832A6284: 4BD5EE6D  bl 0x830050f0
	ctx.lr = 0x832A6288;
	sub_830050F0(ctx, base);
	// 832A6288: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A628C: 386B9758  addi r3, r11, -0x68a8
	ctx.r[3].s64 = ctx.r[11].s64 + -26792;
	// 832A6290: 4BA03C91  bl 0x82ca9f20
	ctx.lr = 0x832A6294;
	sub_82CA9F20(ctx, base);
	// 832A6294: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A6298: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A629C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A62A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A62A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A62A8 size=52
    let mut pc: u32 = 0x832A62A8;
    'dispatch: loop {
        match pc {
            0x832A62A8 => {
    //   block [0x832A62A8..0x832A62DC)
	// 832A62A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A62AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A62B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A62B4: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A62B8: 386B9B44  addi r3, r11, -0x64bc
	ctx.r[3].s64 = ctx.r[11].s64 + -25788;
	// 832A62BC: 4BD5D36D  bl 0x83003628
	ctx.lr = 0x832A62C0;
	sub_83003628(ctx, base);
	// 832A62C0: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A62C4: 386B9780  addi r3, r11, -0x6880
	ctx.r[3].s64 = ctx.r[11].s64 + -26752;
	// 832A62C8: 4BA03C59  bl 0x82ca9f20
	ctx.lr = 0x832A62CC;
	sub_82CA9F20(ctx, base);
	// 832A62CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A62D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A62D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A62D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A62E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A62E0 size=52
    let mut pc: u32 = 0x832A62E0;
    'dispatch: loop {
        match pc {
            0x832A62E0 => {
    //   block [0x832A62E0..0x832A6314)
	// 832A62E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A62E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A62E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A62EC: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A62F0: 386B9B68  addi r3, r11, -0x6498
	ctx.r[3].s64 = ctx.r[11].s64 + -25752;
	// 832A62F4: 4BD5FC4D  bl 0x83005f40
	ctx.lr = 0x832A62F8;
	sub_83005F40(ctx, base);
	// 832A62F8: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A62FC: 386B97A8  addi r3, r11, -0x6858
	ctx.r[3].s64 = ctx.r[11].s64 + -26712;
	// 832A6300: 4BA03C21  bl 0x82ca9f20
	ctx.lr = 0x832A6304;
	sub_82CA9F20(ctx, base);
	// 832A6304: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A6308: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A630C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6310: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6318 size=112
    let mut pc: u32 = 0x832A6318;
    'dispatch: loop {
        match pc {
            0x832A6318 => {
    //   block [0x832A6318..0x832A6388)
	// 832A6318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A631C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6320: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A6324: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A6328: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A632C: 38AAB2C4  addi r5, r10, -0x4d3c
	ctx.r[5].s64 = ctx.r[10].s64 + -19772;
	// 832A6330: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 832A6334: 390B86C4  addi r8, r11, -0x793c
	ctx.r[8].s64 = ctx.r[11].s64 + -31036;
	// 832A6338: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 832A633C: 388A86F4  addi r4, r10, -0x790c
	ctx.r[4].s64 = ctx.r[10].s64 + -30988;
	// 832A6340: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A6344: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A6348: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A634C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A6350: 386AAE8C  addi r3, r10, -0x5174
	ctx.r[3].s64 = ctx.r[10].s64 + -20852;
	// 832A6354: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A6358: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A635C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A6360: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A6364: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A6368: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A636C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 832A6370: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A6374: 4BAAF8CD  bl 0x82d55c40
	ctx.lr = 0x832A6378;
	sub_82D55C40(ctx, base);
	// 832A6378: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A637C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6380: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6384: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6388 size=108
    let mut pc: u32 = 0x832A6388;
    'dispatch: loop {
        match pc {
            0x832A6388 => {
    //   block [0x832A6388..0x832A63F4)
	// 832A6388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A638C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6390: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A6394: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A6398: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 832A639C: 38EB87A0  addi r7, r11, -0x7860
	ctx.r[7].s64 = ctx.r[11].s64 + -30816;
	// 832A63A0: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 832A63A4: 388A3CD8  addi r4, r10, 0x3cd8
	ctx.r[4].s64 = ctx.r[10].s64 + 15576;
	// 832A63A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A63AC: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A63B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A63B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A63B8: 386AAEBC  addi r3, r10, -0x5144
	ctx.r[3].s64 = ctx.r[10].s64 + -20804;
	// 832A63BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A63C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A63C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A63C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A63CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A63D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A63D4: 38C00024  li r6, 0x24
	ctx.r[6].s64 = 36;
	// 832A63D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A63DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A63E0: 4BAAF861  bl 0x82d55c40
	ctx.lr = 0x832A63E4;
	sub_82D55C40(ctx, base);
	// 832A63E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A63E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A63EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A63F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A63F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A63F8 size=108
    let mut pc: u32 = 0x832A63F8;
    'dispatch: loop {
        match pc {
            0x832A63F8 => {
    //   block [0x832A63F8..0x832A6464)
	// 832A63F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A63FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6400: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A6404: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A6408: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 832A640C: 38EB8848  addi r7, r11, -0x77b8
	ctx.r[7].s64 = ctx.r[11].s64 + -30648;
	// 832A6410: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 832A6414: 388A3CD8  addi r4, r10, 0x3cd8
	ctx.r[4].s64 = ctx.r[10].s64 + 15576;
	// 832A6418: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A641C: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A6420: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A6424: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A6428: 386AAEEC  addi r3, r10, -0x5114
	ctx.r[3].s64 = ctx.r[10].s64 + -20756;
	// 832A642C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A6430: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A6434: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A6438: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A643C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A6440: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A6444: 38C00024  li r6, 0x24
	ctx.r[6].s64 = 36;
	// 832A6448: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A644C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A6450: 4BAAF7F1  bl 0x82d55c40
	ctx.lr = 0x832A6454;
	sub_82D55C40(ctx, base);
	// 832A6454: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A6458: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A645C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6460: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A6468 size=244
    let mut pc: u32 = 0x832A6468;
    'dispatch: loop {
        match pc {
            0x832A6468 => {
    //   block [0x832A6468..0x832A655C)
	// 832A6468: 3D208330  lis r9, -0x7cd0
	ctx.r[9].s64 = -2094006272;
	// 832A646C: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 832A6470: 3D008202  lis r8, -0x7dfe
	ctx.r[8].s64 = -2113798144;
	// 832A6474: 396B1640  addi r11, r11, 0x1640
	ctx.r[11].s64 = ctx.r[11].s64 + 5696;
	// 832A6478: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A647C: 81291630  lwz r9, 0x1630(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(5680 as u32) ) } as u64;
	// 832A6480: 390841D8  addi r8, r8, 0x41d8
	ctx.r[8].s64 = ctx.r[8].s64 + 16856;
	// 832A6484: 3CE08202  lis r7, -0x7dfe
	ctx.r[7].s64 = -2113798144;
	// 832A6488: 3CC08202  lis r6, -0x7dfe
	ctx.r[6].s64 = -2113798144;
	// 832A648C: 38E741CC  addi r7, r7, 0x41cc
	ctx.r[7].s64 = ctx.r[7].s64 + 16844;
	// 832A6490: 38C63BE0  addi r6, r6, 0x3be0
	ctx.r[6].s64 = ctx.r[6].s64 + 15328;
	// 832A6494: 912B0050  stw r9, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 832A6498: 3CA08202  lis r5, -0x7dfe
	ctx.r[5].s64 = -2113798144;
	// 832A649C: 914B005C  stw r10, 0x5c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 832A64A0: 910B0060  stw r8, 0x60(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(96 as u32), ctx.r[8].u32 ) };
	// 832A64A4: 38A541C4  addi r5, r5, 0x41c4
	ctx.r[5].s64 = ctx.r[5].s64 + 16836;
	// 832A64A8: 914B0064  stw r10, 0x64(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 832A64AC: 912B0068  stw r9, 0x68(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(104 as u32), ctx.r[9].u32 ) };
	// 832A64B0: 39200018  li r9, 0x18
	ctx.r[9].s64 = 24;
	// 832A64B4: 992B006C  stb r9, 0x6c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(108 as u32), ctx.r[9].u8 ) };
	// 832A64B8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 832A64BC: 992B006D  stb r9, 0x6d(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(109 as u32), ctx.r[9].u8 ) };
	// 832A64C0: 3920000D  li r9, 0xd
	ctx.r[9].s64 = 13;
	// 832A64C4: B14B006E  sth r10, 0x6e(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(110 as u32), ctx.r[10].u16 ) };
	// 832A64C8: B14B0070  sth r10, 0x70(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(112 as u32), ctx.r[10].u16 ) };
	// 832A64CC: B12B0072  sth r9, 0x72(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(114 as u32), ctx.r[9].u16 ) };
	// 832A64D0: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 832A64D4: 914B0074  stw r10, 0x74(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(116 as u32), ctx.r[10].u32 ) };
	// 832A64D8: 90EB0078  stw r7, 0x78(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(120 as u32), ctx.r[7].u32 ) };
	// 832A64DC: 914B007C  stw r10, 0x7c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(124 as u32), ctx.r[10].u32 ) };
	// 832A64E0: 914B0080  stw r10, 0x80(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(128 as u32), ctx.r[10].u32 ) };
	// 832A64E4: 992B0084  stb r9, 0x84(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(132 as u32), ctx.r[9].u8 ) };
	// 832A64E8: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 832A64EC: 994B0085  stb r10, 0x85(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(133 as u32), ctx.r[10].u8 ) };
	// 832A64F0: B14B0086  sth r10, 0x86(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(134 as u32), ctx.r[10].u16 ) };
	// 832A64F4: B14B0088  sth r10, 0x88(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(136 as u32), ctx.r[10].u16 ) };
	// 832A64F8: B12B008A  sth r9, 0x8a(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(138 as u32), ctx.r[9].u16 ) };
	// 832A64FC: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 832A6500: 914B008C  stw r10, 0x8c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(140 as u32), ctx.r[10].u32 ) };
	// 832A6504: 90CB0090  stw r6, 0x90(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(144 as u32), ctx.r[6].u32 ) };
	// 832A6508: 914B0094  stw r10, 0x94(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(148 as u32), ctx.r[10].u32 ) };
	// 832A650C: 914B0098  stw r10, 0x98(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(152 as u32), ctx.r[10].u32 ) };
	// 832A6510: 992B009C  stb r9, 0x9c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(156 as u32), ctx.r[9].u8 ) };
	// 832A6514: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 832A6518: 994B009D  stb r10, 0x9d(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(157 as u32), ctx.r[10].u8 ) };
	// 832A651C: B14B009E  sth r10, 0x9e(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(158 as u32), ctx.r[10].u16 ) };
	// 832A6520: B14B00A0  sth r10, 0xa0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(160 as u32), ctx.r[10].u16 ) };
	// 832A6524: B12B00A2  sth r9, 0xa2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(162 as u32), ctx.r[9].u16 ) };
	// 832A6528: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 832A652C: 914B00A4  stw r10, 0xa4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(164 as u32), ctx.r[10].u32 ) };
	// 832A6530: 90AB00A8  stw r5, 0xa8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(168 as u32), ctx.r[5].u32 ) };
	// 832A6534: 914B00AC  stw r10, 0xac(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(172 as u32), ctx.r[10].u32 ) };
	// 832A6538: 914B00B0  stw r10, 0xb0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(176 as u32), ctx.r[10].u32 ) };
	// 832A653C: 992B00B4  stb r9, 0xb4(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(180 as u32), ctx.r[9].u8 ) };
	// 832A6540: 39200012  li r9, 0x12
	ctx.r[9].s64 = 18;
	// 832A6544: 994B00B5  stb r10, 0xb5(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(181 as u32), ctx.r[10].u8 ) };
	// 832A6548: B14B00B6  sth r10, 0xb6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(182 as u32), ctx.r[10].u16 ) };
	// 832A654C: B14B00B8  sth r10, 0xb8(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(184 as u32), ctx.r[10].u16 ) };
	// 832A6550: B12B00BA  sth r9, 0xba(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(186 as u32), ctx.r[9].u16 ) };
	// 832A6554: 914B00BC  stw r10, 0xbc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(188 as u32), ctx.r[10].u32 ) };
	// 832A6558: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6560 size=112
    let mut pc: u32 = 0x832A6560;
    'dispatch: loop {
        match pc {
            0x832A6560 => {
    //   block [0x832A6560..0x832A65D0)
	// 832A6560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6568: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A656C: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 832A6570: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 832A6574: 392A8A60  addi r9, r10, -0x75a0
	ctx.r[9].s64 = ctx.r[10].s64 + -30112;
	// 832A6578: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 832A657C: 390B1640  addi r8, r11, 0x1640
	ctx.r[8].s64 = ctx.r[11].s64 + 5696;
	// 832A6580: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 832A6584: 388A41B4  addi r4, r10, 0x41b4
	ctx.r[4].s64 = ctx.r[10].s64 + 16820;
	// 832A6588: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A658C: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A6590: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A6594: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 832A6598: 386AAF1C  addi r3, r10, -0x50e4
	ctx.r[3].s64 = ctx.r[10].s64 + -20708;
	// 832A659C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 832A65A0: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 832A65A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A65A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A65AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A65B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A65B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A65B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A65BC: 4BAAF685  bl 0x82d55c40
	ctx.lr = 0x832A65C0;
	sub_82D55C40(ctx, base);
	// 832A65C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A65C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A65C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A65CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A65D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A65D0 size=108
    let mut pc: u32 = 0x832A65D0;
    'dispatch: loop {
        match pc {
            0x832A65D0 => {
    //   block [0x832A65D0..0x832A663C)
	// 832A65D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A65D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A65D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A65DC: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A65E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 832A65E4: 38EB8A9C  addi r7, r11, -0x7564
	ctx.r[7].s64 = ctx.r[11].s64 + -30052;
	// 832A65E8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 832A65EC: 388A3D48  addi r4, r10, 0x3d48
	ctx.r[4].s64 = ctx.r[10].s64 + 15688;
	// 832A65F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A65F4: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A65F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A65FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A6600: 386AAF4C  addi r3, r10, -0x50b4
	ctx.r[3].s64 = ctx.r[10].s64 + -20660;
	// 832A6604: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A6608: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A660C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A6610: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A6614: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A6618: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A661C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 832A6620: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A6624: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A6628: 4BAAF619  bl 0x82d55c40
	ctx.lr = 0x832A662C;
	sub_82D55C40(ctx, base);
	// 832A662C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A6630: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6634: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6638: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6640 size=108
    let mut pc: u32 = 0x832A6640;
    'dispatch: loop {
        match pc {
            0x832A6640 => {
    //   block [0x832A6640..0x832A66AC)
	// 832A6640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6648: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A664C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A6650: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 832A6654: 38EB8ACC  addi r7, r11, -0x7534
	ctx.r[7].s64 = ctx.r[11].s64 + -30004;
	// 832A6658: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 832A665C: 388A3D58  addi r4, r10, 0x3d58
	ctx.r[4].s64 = ctx.r[10].s64 + 15704;
	// 832A6660: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A6664: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A6668: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A666C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A6670: 386AAF7C  addi r3, r10, -0x5084
	ctx.r[3].s64 = ctx.r[10].s64 + -20612;
	// 832A6674: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A6678: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A667C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A6680: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A6684: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A6688: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A668C: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 832A6690: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A6694: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A6698: 4BAAF5A9  bl 0x82d55c40
	ctx.lr = 0x832A669C;
	sub_82D55C40(ctx, base);
	// 832A669C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A66A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A66A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A66A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A66B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A66B0 size=108
    let mut pc: u32 = 0x832A66B0;
    'dispatch: loop {
        match pc {
            0x832A66B0 => {
    //   block [0x832A66B0..0x832A671C)
	// 832A66B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A66B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A66B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A66BC: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A66C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 832A66C4: 38EB8B10  addi r7, r11, -0x74f0
	ctx.r[7].s64 = ctx.r[11].s64 + -29936;
	// 832A66C8: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 832A66CC: 388A3CD8  addi r4, r10, 0x3cd8
	ctx.r[4].s64 = ctx.r[10].s64 + 15576;
	// 832A66D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A66D4: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A66D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A66DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A66E0: 386AAFAC  addi r3, r10, -0x5054
	ctx.r[3].s64 = ctx.r[10].s64 + -20564;
	// 832A66E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A66E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A66EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A66F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A66F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A66F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A66FC: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 832A6700: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A6704: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A6708: 4BAAF539  bl 0x82d55c40
	ctx.lr = 0x832A670C;
	sub_82D55C40(ctx, base);
	// 832A670C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A6710: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6714: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6718: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6720 size=108
    let mut pc: u32 = 0x832A6720;
    'dispatch: loop {
        match pc {
            0x832A6720 => {
    //   block [0x832A6720..0x832A678C)
	// 832A6720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6728: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A672C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A6730: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 832A6734: 38EB8C00  addi r7, r11, -0x7400
	ctx.r[7].s64 = ctx.r[11].s64 + -29696;
	// 832A6738: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 832A673C: 388A3CD8  addi r4, r10, 0x3cd8
	ctx.r[4].s64 = ctx.r[10].s64 + 15576;
	// 832A6740: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A6744: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A6748: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A674C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A6750: 386AAFDC  addi r3, r10, -0x5024
	ctx.r[3].s64 = ctx.r[10].s64 + -20516;
	// 832A6754: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A6758: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A675C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A6760: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A6764: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A6768: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A676C: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 832A6770: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A6774: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A6778: 4BAAF4C9  bl 0x82d55c40
	ctx.lr = 0x832A677C;
	sub_82D55C40(ctx, base);
	// 832A677C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A6780: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6784: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6788: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A6790 size=292
    let mut pc: u32 = 0x832A6790;
    'dispatch: loop {
        match pc {
            0x832A6790 => {
    //   block [0x832A6790..0x832A68B4)
	// 832A6790: 3D208330  lis r9, -0x7cd0
	ctx.r[9].s64 = -2094006272;
	// 832A6794: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 832A6798: 3D008202  lis r8, -0x7dfe
	ctx.r[8].s64 = -2113798144;
	// 832A679C: 396B1710  addi r11, r11, 0x1710
	ctx.r[11].s64 = ctx.r[11].s64 + 5904;
	// 832A67A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A67A4: 81291700  lwz r9, 0x1700(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(5888 as u32) ) } as u64;
	// 832A67A8: 390841D8  addi r8, r8, 0x41d8
	ctx.r[8].s64 = ctx.r[8].s64 + 16856;
	// 832A67AC: 3CE08202  lis r7, -0x7dfe
	ctx.r[7].s64 = -2113798144;
	// 832A67B0: 3CC08202  lis r6, -0x7dfe
	ctx.r[6].s64 = -2113798144;
	// 832A67B4: 38E741CC  addi r7, r7, 0x41cc
	ctx.r[7].s64 = ctx.r[7].s64 + 16844;
	// 832A67B8: 38C63BE0  addi r6, r6, 0x3be0
	ctx.r[6].s64 = ctx.r[6].s64 + 15328;
	// 832A67BC: 912B0050  stw r9, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 832A67C0: 3CA08202  lis r5, -0x7dfe
	ctx.r[5].s64 = -2113798144;
	// 832A67C4: 914B005C  stw r10, 0x5c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 832A67C8: 910B0060  stw r8, 0x60(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(96 as u32), ctx.r[8].u32 ) };
	// 832A67CC: 38A541C4  addi r5, r5, 0x41c4
	ctx.r[5].s64 = ctx.r[5].s64 + 16836;
	// 832A67D0: 914B0064  stw r10, 0x64(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 832A67D4: 3C808208  lis r4, -0x7df8
	ctx.r[4].s64 = -2113404928;
	// 832A67D8: 912B0068  stw r9, 0x68(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(104 as u32), ctx.r[9].u32 ) };
	// 832A67DC: 39200018  li r9, 0x18
	ctx.r[9].s64 = 24;
	// 832A67E0: 38848AFC  addi r4, r4, -0x7504
	ctx.r[4].s64 = ctx.r[4].s64 + -29956;
	// 832A67E4: 992B006C  stb r9, 0x6c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(108 as u32), ctx.r[9].u8 ) };
	// 832A67E8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 832A67EC: 992B006D  stb r9, 0x6d(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(109 as u32), ctx.r[9].u8 ) };
	// 832A67F0: 3920000D  li r9, 0xd
	ctx.r[9].s64 = 13;
	// 832A67F4: B14B006E  sth r10, 0x6e(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(110 as u32), ctx.r[10].u16 ) };
	// 832A67F8: B14B0070  sth r10, 0x70(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(112 as u32), ctx.r[10].u16 ) };
	// 832A67FC: B12B0072  sth r9, 0x72(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(114 as u32), ctx.r[9].u16 ) };
	// 832A6800: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 832A6804: 914B0074  stw r10, 0x74(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(116 as u32), ctx.r[10].u32 ) };
	// 832A6808: 90EB0078  stw r7, 0x78(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(120 as u32), ctx.r[7].u32 ) };
	// 832A680C: 914B007C  stw r10, 0x7c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(124 as u32), ctx.r[10].u32 ) };
	// 832A6810: 914B0080  stw r10, 0x80(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(128 as u32), ctx.r[10].u32 ) };
	// 832A6814: 992B0084  stb r9, 0x84(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(132 as u32), ctx.r[9].u8 ) };
	// 832A6818: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 832A681C: 994B0085  stb r10, 0x85(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(133 as u32), ctx.r[10].u8 ) };
	// 832A6820: B14B0086  sth r10, 0x86(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(134 as u32), ctx.r[10].u16 ) };
	// 832A6824: B14B0088  sth r10, 0x88(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(136 as u32), ctx.r[10].u16 ) };
	// 832A6828: B12B008A  sth r9, 0x8a(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(138 as u32), ctx.r[9].u16 ) };
	// 832A682C: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 832A6830: 914B008C  stw r10, 0x8c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(140 as u32), ctx.r[10].u32 ) };
	// 832A6834: 90CB0090  stw r6, 0x90(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(144 as u32), ctx.r[6].u32 ) };
	// 832A6838: 914B0094  stw r10, 0x94(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(148 as u32), ctx.r[10].u32 ) };
	// 832A683C: 914B0098  stw r10, 0x98(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(152 as u32), ctx.r[10].u32 ) };
	// 832A6840: 992B009C  stb r9, 0x9c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(156 as u32), ctx.r[9].u8 ) };
	// 832A6844: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 832A6848: 994B009D  stb r10, 0x9d(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(157 as u32), ctx.r[10].u8 ) };
	// 832A684C: B14B009E  sth r10, 0x9e(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(158 as u32), ctx.r[10].u16 ) };
	// 832A6850: B14B00A0  sth r10, 0xa0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(160 as u32), ctx.r[10].u16 ) };
	// 832A6854: B12B00A2  sth r9, 0xa2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(162 as u32), ctx.r[9].u16 ) };
	// 832A6858: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 832A685C: 914B00A4  stw r10, 0xa4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(164 as u32), ctx.r[10].u32 ) };
	// 832A6860: 90AB00A8  stw r5, 0xa8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(168 as u32), ctx.r[5].u32 ) };
	// 832A6864: 914B00AC  stw r10, 0xac(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(172 as u32), ctx.r[10].u32 ) };
	// 832A6868: 914B00B0  stw r10, 0xb0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(176 as u32), ctx.r[10].u32 ) };
	// 832A686C: 992B00B4  stb r9, 0xb4(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(180 as u32), ctx.r[9].u8 ) };
	// 832A6870: 39200012  li r9, 0x12
	ctx.r[9].s64 = 18;
	// 832A6874: 994B00B5  stb r10, 0xb5(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(181 as u32), ctx.r[10].u8 ) };
	// 832A6878: B14B00B6  sth r10, 0xb6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(182 as u32), ctx.r[10].u16 ) };
	// 832A687C: B14B00B8  sth r10, 0xb8(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(184 as u32), ctx.r[10].u16 ) };
	// 832A6880: B12B00BA  sth r9, 0xba(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(186 as u32), ctx.r[9].u16 ) };
	// 832A6884: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 832A6888: 914B00BC  stw r10, 0xbc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(188 as u32), ctx.r[10].u32 ) };
	// 832A688C: 908B00C0  stw r4, 0xc0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(192 as u32), ctx.r[4].u32 ) };
	// 832A6890: 914B00C4  stw r10, 0xc4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(196 as u32), ctx.r[10].u32 ) };
	// 832A6894: 914B00C8  stw r10, 0xc8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(200 as u32), ctx.r[10].u32 ) };
	// 832A6898: 992B00CC  stb r9, 0xcc(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(204 as u32), ctx.r[9].u8 ) };
	// 832A689C: 994B00CD  stb r10, 0xcd(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(205 as u32), ctx.r[10].u8 ) };
	// 832A68A0: B14B00CE  sth r10, 0xce(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(206 as u32), ctx.r[10].u16 ) };
	// 832A68A4: B14B00D0  sth r10, 0xd0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(208 as u32), ctx.r[10].u16 ) };
	// 832A68A8: B12B00D2  sth r9, 0xd2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(210 as u32), ctx.r[9].u16 ) };
	// 832A68AC: 914B00D4  stw r10, 0xd4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(212 as u32), ctx.r[10].u32 ) };
	// 832A68B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A68B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A68B8 size=112
    let mut pc: u32 = 0x832A68B8;
    'dispatch: loop {
        match pc {
            0x832A68B8 => {
    //   block [0x832A68B8..0x832A6928)
	// 832A68B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A68BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A68C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A68C4: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 832A68C8: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 832A68CC: 392A8E60  addi r9, r10, -0x71a0
	ctx.r[9].s64 = ctx.r[10].s64 + -29088;
	// 832A68D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 832A68D4: 390B1710  addi r8, r11, 0x1710
	ctx.r[8].s64 = ctx.r[11].s64 + 5904;
	// 832A68D8: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 832A68DC: 388A41B4  addi r4, r10, 0x41b4
	ctx.r[4].s64 = ctx.r[10].s64 + 16820;
	// 832A68E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A68E4: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A68E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A68EC: 38C00018  li r6, 0x18
	ctx.r[6].s64 = 24;
	// 832A68F0: 386AB00C  addi r3, r10, -0x4ff4
	ctx.r[3].s64 = ctx.r[10].s64 + -20468;
	// 832A68F4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 832A68F8: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 832A68FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A6900: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A6904: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A6908: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A690C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A6910: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A6914: 4BAAF32D  bl 0x82d55c40
	ctx.lr = 0x832A6918;
	sub_82D55C40(ctx, base);
	// 832A6918: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A691C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6920: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6924: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6928 size=108
    let mut pc: u32 = 0x832A6928;
    'dispatch: loop {
        match pc {
            0x832A6928 => {
    //   block [0x832A6928..0x832A6994)
	// 832A6928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A692C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6930: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A6934: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A6938: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 832A693C: 38EB8E9C  addi r7, r11, -0x7164
	ctx.r[7].s64 = ctx.r[11].s64 + -29028;
	// 832A6940: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 832A6944: 388A3D48  addi r4, r10, 0x3d48
	ctx.r[4].s64 = ctx.r[10].s64 + 15688;
	// 832A6948: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A694C: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A6950: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A6954: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A6958: 386AB03C  addi r3, r10, -0x4fc4
	ctx.r[3].s64 = ctx.r[10].s64 + -20420;
	// 832A695C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A6960: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A6964: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A6968: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A696C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A6970: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A6974: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 832A6978: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A697C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A6980: 4BAAF2C1  bl 0x82d55c40
	ctx.lr = 0x832A6984;
	sub_82D55C40(ctx, base);
	// 832A6984: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A6988: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A698C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6990: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6998 size=108
    let mut pc: u32 = 0x832A6998;
    'dispatch: loop {
        match pc {
            0x832A6998 => {
    //   block [0x832A6998..0x832A6A04)
	// 832A6998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A699C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A69A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A69A4: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A69A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 832A69AC: 38EB8ED0  addi r7, r11, -0x7130
	ctx.r[7].s64 = ctx.r[11].s64 + -28976;
	// 832A69B0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 832A69B4: 388A3D58  addi r4, r10, 0x3d58
	ctx.r[4].s64 = ctx.r[10].s64 + 15704;
	// 832A69B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A69BC: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A69C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A69C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A69C8: 386AB06C  addi r3, r10, -0x4f94
	ctx.r[3].s64 = ctx.r[10].s64 + -20372;
	// 832A69CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A69D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A69D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A69D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A69DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A69E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A69E4: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 832A69E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A69EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A69F0: 4BAAF251  bl 0x82d55c40
	ctx.lr = 0x832A69F4;
	sub_82D55C40(ctx, base);
	// 832A69F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A69F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A69FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6A00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A6A08 size=28
    let mut pc: u32 = 0x832A6A08;
    'dispatch: loop {
        match pc {
            0x832A6A08 => {
    //   block [0x832A6A08..0x832A6A24)
	// 832A6A08: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 832A6A0C: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 832A6A10: 396B17E8  addi r11, r11, 0x17e8
	ctx.r[11].s64 = ctx.r[11].s64 + 6120;
	// 832A6A14: 812A76FC  lwz r9, 0x76fc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(30460 as u32) ) } as u64;
	// 832A6A18: 916A76FC  stw r11, 0x76fc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30460 as u32), ctx.r[11].u32 ) };
	// 832A6A1C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 832A6A20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6A28 size=76
    let mut pc: u32 = 0x832A6A28;
    'dispatch: loop {
        match pc {
            0x832A6A28 => {
    //   block [0x832A6A28..0x832A6A74)
	// 832A6A28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6A2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6A30: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A6A34: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A6A38: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A6A3C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 832A6A40: 3BEBB0A0  addi r31, r11, -0x4f60
	ctx.r[31].s64 = ctx.r[11].s64 + -20320;
	// 832A6A44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A6A48: 4801324D  bl 0x832b9c94
	ctx.lr = 0x832A6A4C;
	// extern call 0x832B9C94 → crate::xboxkrnl::RtlInitializeCriticalSectionAndSpinCount
	crate::xboxkrnl::RtlInitializeCriticalSectionAndSpinCount(ctx, base);
	// 832A6A4C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 832A6A50: 3D40832C  lis r10, -0x7cd4
	ctx.r[10].s64 = -2094268416;
	// 832A6A54: 386A97D0  addi r3, r10, -0x6830
	ctx.r[3].s64 = ctx.r[10].s64 + -26672;
	// 832A6A58: F97F0020  std r11, 0x20(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 832A6A5C: 4BA034C5  bl 0x82ca9f20
	ctx.lr = 0x832A6A60;
	sub_82CA9F20(ctx, base);
	// 832A6A60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A6A64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6A68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6A6C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A6A70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6A78 size=108
    let mut pc: u32 = 0x832A6A78;
    'dispatch: loop {
        match pc {
            0x832A6A78 => {
    //   block [0x832A6A78..0x832A6AE4)
	// 832A6A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6A80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A6A84: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A6A88: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 832A6A8C: 38EB8FB8  addi r7, r11, -0x7048
	ctx.r[7].s64 = ctx.r[11].s64 + -28744;
	// 832A6A90: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 832A6A94: 388A9090  addi r4, r10, -0x6f70
	ctx.r[4].s64 = ctx.r[10].s64 + -28528;
	// 832A6A98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A6A9C: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A6AA0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A6AA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A6AA8: 386AB0DC  addi r3, r10, -0x4f24
	ctx.r[3].s64 = ctx.r[10].s64 + -20260;
	// 832A6AAC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A6AB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A6AB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A6AB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A6ABC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A6AC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A6AC4: 38C000B0  li r6, 0xb0
	ctx.r[6].s64 = 176;
	// 832A6AC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A6ACC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A6AD0: 4BAAF171  bl 0x82d55c40
	ctx.lr = 0x832A6AD4;
	sub_82D55C40(ctx, base);
	// 832A6AD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A6AD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6ADC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6AE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6AE8 size=108
    let mut pc: u32 = 0x832A6AE8;
    'dispatch: loop {
        match pc {
            0x832A6AE8 => {
    //   block [0x832A6AE8..0x832A6B54)
	// 832A6AE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6AEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6AF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A6AF4: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A6AF8: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 832A6AFC: 38EB90B0  addi r7, r11, -0x6f50
	ctx.r[7].s64 = ctx.r[11].s64 + -28496;
	// 832A6B00: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 832A6B04: 388A9170  addi r4, r10, -0x6e90
	ctx.r[4].s64 = ctx.r[10].s64 + -28304;
	// 832A6B08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A6B0C: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A6B10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A6B14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A6B18: 386AB10C  addi r3, r10, -0x4ef4
	ctx.r[3].s64 = ctx.r[10].s64 + -20212;
	// 832A6B1C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A6B20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A6B24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A6B28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A6B2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A6B30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A6B34: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 832A6B38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A6B3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A6B40: 4BAAF101  bl 0x82d55c40
	ctx.lr = 0x832A6B44;
	sub_82D55C40(ctx, base);
	// 832A6B44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A6B48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6B4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6B50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6B58 size=108
    let mut pc: u32 = 0x832A6B58;
    'dispatch: loop {
        match pc {
            0x832A6B58 => {
    //   block [0x832A6B58..0x832A6BC4)
	// 832A6B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6B5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6B60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A6B64: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A6B68: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 832A6B6C: 38EB90E0  addi r7, r11, -0x6f20
	ctx.r[7].s64 = ctx.r[11].s64 + -28448;
	// 832A6B70: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 832A6B74: 388A9178  addi r4, r10, -0x6e88
	ctx.r[4].s64 = ctx.r[10].s64 + -28296;
	// 832A6B78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A6B7C: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A6B80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A6B84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A6B88: 386AB13C  addi r3, r10, -0x4ec4
	ctx.r[3].s64 = ctx.r[10].s64 + -20164;
	// 832A6B8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A6B90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A6B94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A6B98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A6B9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A6BA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A6BA4: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 832A6BA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A6BAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A6BB0: 4BAAF091  bl 0x82d55c40
	ctx.lr = 0x832A6BB4;
	sub_82D55C40(ctx, base);
	// 832A6BB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A6BB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6BBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6BC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6BC8 size=112
    let mut pc: u32 = 0x832A6BC8;
    'dispatch: loop {
        match pc {
            0x832A6BC8 => {
    //   block [0x832A6BC8..0x832A6C38)
	// 832A6BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6BCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6BD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A6BD4: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 832A6BD8: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A6BDC: 392A9248  addi r9, r10, -0x6db8
	ctx.r[9].s64 = ctx.r[10].s64 + -28088;
	// 832A6BE0: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 832A6BE4: 390B9270  addi r8, r11, -0x6d90
	ctx.r[8].s64 = ctx.r[11].s64 + -28048;
	// 832A6BE8: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 832A6BEC: 388A92B8  addi r4, r10, -0x6d48
	ctx.r[4].s64 = ctx.r[10].s64 + -27976;
	// 832A6BF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A6BF4: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A6BF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A6BFC: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 832A6C00: 386AB16C  addi r3, r10, -0x4e94
	ctx.r[3].s64 = ctx.r[10].s64 + -20116;
	// 832A6C04: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 832A6C08: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 832A6C0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A6C10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A6C14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A6C18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A6C1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A6C20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A6C24: 4BAAF01D  bl 0x82d55c40
	ctx.lr = 0x832A6C28;
	sub_82D55C40(ctx, base);
	// 832A6C28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A6C2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6C30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6C34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A6C38 size=28
    let mut pc: u32 = 0x832A6C38;
    'dispatch: loop {
        match pc {
            0x832A6C38 => {
    //   block [0x832A6C38..0x832A6C54)
	// 832A6C38: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 832A6C3C: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 832A6C40: 396B18BC  addi r11, r11, 0x18bc
	ctx.r[11].s64 = ctx.r[11].s64 + 6332;
	// 832A6C44: 812A76FC  lwz r9, 0x76fc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(30460 as u32) ) } as u64;
	// 832A6C48: 916A76FC  stw r11, 0x76fc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30460 as u32), ctx.r[11].u32 ) };
	// 832A6C4C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 832A6C50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6C58 size=108
    let mut pc: u32 = 0x832A6C58;
    'dispatch: loop {
        match pc {
            0x832A6C58 => {
    //   block [0x832A6C58..0x832A6CC4)
	// 832A6C58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6C5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6C60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A6C64: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A6C68: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 832A6C6C: 38EB93D4  addi r7, r11, -0x6c2c
	ctx.r[7].s64 = ctx.r[11].s64 + -27692;
	// 832A6C70: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 832A6C74: 388A9448  addi r4, r10, -0x6bb8
	ctx.r[4].s64 = ctx.r[10].s64 + -27576;
	// 832A6C78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A6C7C: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A6C80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A6C84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A6C88: 386AB1A4  addi r3, r10, -0x4e5c
	ctx.r[3].s64 = ctx.r[10].s64 + -20060;
	// 832A6C8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A6C90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A6C94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A6C98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A6C9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A6CA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A6CA4: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 832A6CA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A6CAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A6CB0: 4BAAEF91  bl 0x82d55c40
	ctx.lr = 0x832A6CB4;
	sub_82D55C40(ctx, base);
	// 832A6CB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A6CB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6CBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6CC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6CC8 size=108
    let mut pc: u32 = 0x832A6CC8;
    'dispatch: loop {
        match pc {
            0x832A6CC8 => {
    //   block [0x832A6CC8..0x832A6D34)
	// 832A6CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6CCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6CD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A6CD4: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A6CD8: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 832A6CDC: 38EB9404  addi r7, r11, -0x6bfc
	ctx.r[7].s64 = ctx.r[11].s64 + -27644;
	// 832A6CE0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 832A6CE4: 388A946C  addi r4, r10, -0x6b94
	ctx.r[4].s64 = ctx.r[10].s64 + -27540;
	// 832A6CE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A6CEC: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A6CF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A6CF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A6CF8: 386AB1D4  addi r3, r10, -0x4e2c
	ctx.r[3].s64 = ctx.r[10].s64 + -20012;
	// 832A6CFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A6D00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A6D04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A6D08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A6D0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A6D10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A6D14: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 832A6D18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A6D1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A6D20: 4BAAEF21  bl 0x82d55c40
	ctx.lr = 0x832A6D24;
	sub_82D55C40(ctx, base);
	// 832A6D24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A6D28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6D2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6D30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A6D38 size=24
    let mut pc: u32 = 0x832A6D38;
    'dispatch: loop {
        match pc {
            0x832A6D38 => {
    //   block [0x832A6D38..0x832A6D50)
	// 832A6D38: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 832A6D3C: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 832A6D40: 394A1900  addi r10, r10, 0x1900
	ctx.r[10].s64 = ctx.r[10].s64 + 6400;
	// 832A6D44: 816B18C8  lwz r11, 0x18c8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6344 as u32) ) } as u64;
	// 832A6D48: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 832A6D4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6D50 size=112
    let mut pc: u32 = 0x832A6D50;
    'dispatch: loop {
        match pc {
            0x832A6D50 => {
    //   block [0x832A6D50..0x832A6DC0)
	// 832A6D50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6D54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6D58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A6D5C: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 832A6D60: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 832A6D64: 392A9434  addi r9, r10, -0x6bcc
	ctx.r[9].s64 = ctx.r[10].s64 + -27596;
	// 832A6D68: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 832A6D6C: 390B1900  addi r8, r11, 0x1900
	ctx.r[8].s64 = ctx.r[11].s64 + 6400;
	// 832A6D70: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 832A6D74: 388A9488  addi r4, r10, -0x6b78
	ctx.r[4].s64 = ctx.r[10].s64 + -27512;
	// 832A6D78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A6D7C: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A6D80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A6D84: 38C00024  li r6, 0x24
	ctx.r[6].s64 = 36;
	// 832A6D88: 386AB204  addi r3, r10, -0x4dfc
	ctx.r[3].s64 = ctx.r[10].s64 + -19964;
	// 832A6D8C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 832A6D90: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 832A6D94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A6D98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A6D9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A6DA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A6DA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A6DA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A6DAC: 4BAAEE95  bl 0x82d55c40
	ctx.lr = 0x832A6DB0;
	sub_82D55C40(ctx, base);
	// 832A6DB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A6DB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6DB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6DBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6DC0 size=108
    let mut pc: u32 = 0x832A6DC0;
    'dispatch: loop {
        match pc {
            0x832A6DC0 => {
    //   block [0x832A6DC0..0x832A6E2C)
	// 832A6DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6DC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6DC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A6DCC: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A6DD0: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 832A6DD4: 38EB9778  addi r7, r11, -0x6888
	ctx.r[7].s64 = ctx.r[11].s64 + -26760;
	// 832A6DD8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 832A6DDC: 388A97A8  addi r4, r10, -0x6858
	ctx.r[4].s64 = ctx.r[10].s64 + -26712;
	// 832A6DE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A6DE4: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A6DE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A6DEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A6DF0: 386AB234  addi r3, r10, -0x4dcc
	ctx.r[3].s64 = ctx.r[10].s64 + -19916;
	// 832A6DF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A6DF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A6DFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A6E00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A6E04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A6E08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A6E0C: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 832A6E10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A6E14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A6E18: 4BAAEE29  bl 0x82d55c40
	ctx.lr = 0x832A6E1C;
	sub_82D55C40(ctx, base);
	// 832A6E1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A6E20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6E24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6E28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6E30 size=108
    let mut pc: u32 = 0x832A6E30;
    'dispatch: loop {
        match pc {
            0x832A6E30 => {
    //   block [0x832A6E30..0x832A6E9C)
	// 832A6E30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6E34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6E38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A6E3C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A6E40: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 832A6E44: 38EB9828  addi r7, r11, -0x67d8
	ctx.r[7].s64 = ctx.r[11].s64 + -26584;
	// 832A6E48: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 832A6E4C: 388A98E8  addi r4, r10, -0x6718
	ctx.r[4].s64 = ctx.r[10].s64 + -26392;
	// 832A6E50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A6E54: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A6E58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A6E5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A6E60: 386AB264  addi r3, r10, -0x4d9c
	ctx.r[3].s64 = ctx.r[10].s64 + -19868;
	// 832A6E64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A6E68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A6E6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A6E70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A6E74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A6E78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A6E7C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 832A6E80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A6E84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A6E88: 4BAAEDB9  bl 0x82d55c40
	ctx.lr = 0x832A6E8C;
	sub_82D55C40(ctx, base);
	// 832A6E8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A6E90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6E94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6E98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6EA0 size=108
    let mut pc: u32 = 0x832A6EA0;
    'dispatch: loop {
        match pc {
            0x832A6EA0 => {
    //   block [0x832A6EA0..0x832A6F0C)
	// 832A6EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6EA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6EA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A6EAC: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A6EB0: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 832A6EB4: 38EB9888  addi r7, r11, -0x6778
	ctx.r[7].s64 = ctx.r[11].s64 + -26488;
	// 832A6EB8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 832A6EBC: 388A9900  addi r4, r10, -0x6700
	ctx.r[4].s64 = ctx.r[10].s64 + -26368;
	// 832A6EC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A6EC4: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A6EC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A6ECC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A6ED0: 386AB294  addi r3, r10, -0x4d6c
	ctx.r[3].s64 = ctx.r[10].s64 + -19820;
	// 832A6ED4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A6ED8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A6EDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A6EE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A6EE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A6EE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A6EEC: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 832A6EF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A6EF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A6EF8: 4BAAED49  bl 0x82d55c40
	ctx.lr = 0x832A6EFC;
	sub_82D55C40(ctx, base);
	// 832A6EFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A6F00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6F04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6F08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6F10 size=96
    let mut pc: u32 = 0x832A6F10;
    'dispatch: loop {
        match pc {
            0x832A6F10 => {
    //   block [0x832A6F10..0x832A6F70)
	// 832A6F10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6F14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6F18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A6F1C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 832A6F20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A6F24: 388A39E4  addi r4, r10, 0x39e4
	ctx.r[4].s64 = ctx.r[10].s64 + 14820;
	// 832A6F28: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A6F2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A6F30: 386AB2C4  addi r3, r10, -0x4d3c
	ctx.r[3].s64 = ctx.r[10].s64 + -19772;
	// 832A6F34: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A6F38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A6F3C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 832A6F40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A6F44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A6F48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A6F4C: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 832A6F50: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 832A6F54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A6F58: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 832A6F5C: 4BAAECE5  bl 0x82d55c40
	ctx.lr = 0x832A6F60;
	sub_82D55C40(ctx, base);
	// 832A6F60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A6F64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6F68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6F6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6F70 size=108
    let mut pc: u32 = 0x832A6F70;
    'dispatch: loop {
        match pc {
            0x832A6F70 => {
    //   block [0x832A6F70..0x832A6FDC)
	// 832A6F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6F74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6F78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A6F7C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A6F80: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 832A6F84: 38EB9968  addi r7, r11, -0x6698
	ctx.r[7].s64 = ctx.r[11].s64 + -26264;
	// 832A6F88: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 832A6F8C: 388A99E0  addi r4, r10, -0x6620
	ctx.r[4].s64 = ctx.r[10].s64 + -26144;
	// 832A6F90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A6F94: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A6F98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A6F9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A6FA0: 386AB2F4  addi r3, r10, -0x4d0c
	ctx.r[3].s64 = ctx.r[10].s64 + -19724;
	// 832A6FA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A6FA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A6FAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A6FB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A6FB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A6FB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A6FBC: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 832A6FC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A6FC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A6FC8: 4BAAEC79  bl 0x82d55c40
	ctx.lr = 0x832A6FCC;
	sub_82D55C40(ctx, base);
	// 832A6FCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A6FD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6FD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6FD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A6FE0 size=28
    let mut pc: u32 = 0x832A6FE0;
    'dispatch: loop {
        match pc {
            0x832A6FE0 => {
    //   block [0x832A6FE0..0x832A6FFC)
	// 832A6FE0: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 832A6FE4: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 832A6FE8: 396B1C24  addi r11, r11, 0x1c24
	ctx.r[11].s64 = ctx.r[11].s64 + 7204;
	// 832A6FEC: 812A76FC  lwz r9, 0x76fc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(30460 as u32) ) } as u64;
	// 832A6FF0: 916A76FC  stw r11, 0x76fc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30460 as u32), ctx.r[11].u32 ) };
	// 832A6FF4: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 832A6FF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A7000 size=108
    let mut pc: u32 = 0x832A7000;
    'dispatch: loop {
        match pc {
            0x832A7000 => {
    //   block [0x832A7000..0x832A706C)
	// 832A7000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A7004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A7008: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A700C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A7010: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 832A7014: 38EBA0D0  addi r7, r11, -0x5f30
	ctx.r[7].s64 = ctx.r[11].s64 + -24368;
	// 832A7018: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 832A701C: 388AA1D8  addi r4, r10, -0x5e28
	ctx.r[4].s64 = ctx.r[10].s64 + -24104;
	// 832A7020: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A7024: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A7028: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A702C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A7030: 386AB324  addi r3, r10, -0x4cdc
	ctx.r[3].s64 = ctx.r[10].s64 + -19676;
	// 832A7034: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A7038: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A703C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A7040: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A7044: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A7048: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A704C: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 832A7050: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A7054: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A7058: 4BAAEBE9  bl 0x82d55c40
	ctx.lr = 0x832A705C;
	sub_82D55C40(ctx, base);
	// 832A705C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A7060: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A7064: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A7068: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A7070 size=108
    let mut pc: u32 = 0x832A7070;
    'dispatch: loop {
        match pc {
            0x832A7070 => {
    //   block [0x832A7070..0x832A70DC)
	// 832A7070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A7074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A7078: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A707C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A7080: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 832A7084: 38EBA280  addi r7, r11, -0x5d80
	ctx.r[7].s64 = ctx.r[11].s64 + -23936;
	// 832A7088: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 832A708C: 388AA358  addi r4, r10, -0x5ca8
	ctx.r[4].s64 = ctx.r[10].s64 + -23720;
	// 832A7090: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A7094: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A7098: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A709C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A70A0: 386AB354  addi r3, r10, -0x4cac
	ctx.r[3].s64 = ctx.r[10].s64 + -19628;
	// 832A70A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A70A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A70AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A70B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A70B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A70B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A70BC: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 832A70C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A70C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A70C8: 4BAAEB79  bl 0x82d55c40
	ctx.lr = 0x832A70CC;
	sub_82D55C40(ctx, base);
	// 832A70CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A70D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A70D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A70D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A70E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A70E0 size=68
    let mut pc: u32 = 0x832A70E0;
    'dispatch: loop {
        match pc {
            0x832A70E0 => {
    //   block [0x832A70E0..0x832A7124)
	// 832A70E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A70E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A70E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A70EC: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A70F0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A70F4: 388BA674  addi r4, r11, -0x598c
	ctx.r[4].s64 = ctx.r[11].s64 + -22924;
	// 832A70F8: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A70FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A7100: 386BB38C  addi r3, r11, -0x4c74
	ctx.r[3].s64 = ctx.r[11].s64 + -19572;
	// 832A7104: 4BBE29D5  bl 0x82e89ad8
	ctx.lr = 0x832A7108;
	sub_82E89AD8(ctx, base);
	// 832A7108: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A710C: 386B9830  addi r3, r11, -0x67d0
	ctx.r[3].s64 = ctx.r[11].s64 + -26576;
	// 832A7110: 4BA02E11  bl 0x82ca9f20
	ctx.lr = 0x832A7114;
	sub_82CA9F20(ctx, base);
	// 832A7114: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A7118: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A711C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A7120: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7128 size=12
    let mut pc: u32 = 0x832A7128;
    'dispatch: loop {
        match pc {
            0x832A7128 => {
    //   block [0x832A7128..0x832A7134)
	// 832A7128: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A712C: 386B97E0  addi r3, r11, -0x6820
	ctx.r[3].s64 = ctx.r[11].s64 + -26656;
	// 832A7130: 4BA02DF0  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A7138 size=80
    let mut pc: u32 = 0x832A7138;
    'dispatch: loop {
        match pc {
            0x832A7138 => {
    //   block [0x832A7138..0x832A7188)
	// 832A7138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A713C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A7140: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A7144: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 832A7148: 3D60830F  lis r11, -0x7cf1
	ctx.r[11].s64 = -2096168960;
	// 832A714C: 388AA6AC  addi r4, r10, -0x5954
	ctx.r[4].s64 = ctx.r[10].s64 + -22868;
	// 832A7150: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A7154: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A7158: 386ABA88  addi r3, r10, -0x4578
	ctx.r[3].s64 = ctx.r[10].s64 + -17784;
	// 832A715C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A7160: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A7164: 38ABF7D8  addi r5, r11, -0x828
	ctx.r[5].s64 = ctx.r[11].s64 + -2088;
	// 832A7168: 4BBDE659  bl 0x82e857c0
	ctx.lr = 0x832A716C;
	sub_82E857C0(ctx, base);
	// 832A716C: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A7170: 386B98B0  addi r3, r11, -0x6750
	ctx.r[3].s64 = ctx.r[11].s64 + -26448;
	// 832A7174: 4BA02DAD  bl 0x82ca9f20
	ctx.lr = 0x832A7178;
	sub_82CA9F20(ctx, base);
	// 832A7178: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A717C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A7180: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A7184: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7188 size=12
    let mut pc: u32 = 0x832A7188;
    'dispatch: loop {
        match pc {
            0x832A7188 => {
    //   block [0x832A7188..0x832A7194)
	// 832A7188: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A718C: 386B98C8  addi r3, r11, -0x6738
	ctx.r[3].s64 = ctx.r[11].s64 + -26424;
	// 832A7190: 4BA02D90  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7198 size=16
    let mut pc: u32 = 0x832A7198;
    'dispatch: loop {
        match pc {
            0x832A7198 => {
    //   block [0x832A7198..0x832A71A8)
	// 832A7198: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A719C: 38800072  li r4, 0x72
	ctx.r[4].s64 = 114;
	// 832A71A0: 386BC344  addi r3, r11, -0x3cbc
	ctx.r[3].s64 = ctx.r[11].s64 + -15548;
	// 832A71A4: 4BF29FDC  b 0x831d1180
	sub_831D1180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A71A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A71A8 size=20
    let mut pc: u32 = 0x832A71A8;
    'dispatch: loop {
        match pc {
            0x832A71A8 => {
    //   block [0x832A71A8..0x832A71BC)
	// 832A71A8: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A71AC: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A71B0: 388BC344  addi r4, r11, -0x3cbc
	ctx.r[4].s64 = ctx.r[11].s64 + -15548;
	// 832A71B4: 386AC370  addi r3, r10, -0x3c90
	ctx.r[3].s64 = ctx.r[10].s64 + -15504;
	// 832A71B8: 4BF2AC00  b 0x831d1db8
	crate::recompiler::externs::call(ctx, base, 0x831D1DB8);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


