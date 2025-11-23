pub fn sub_8326A690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A690 size=64
    let mut pc: u32 = 0x8326A690;
    'dispatch: loop {
        match pc {
            0x8326A690 => {
    //   block [0x8326A690..0x8326A6D0)
	// 8326A690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A698: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326A69C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326A6A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326A6A4: 388BD958  addi r4, r11, -0x26a8
	ctx.r[4].s64 = ctx.r[11].s64 + -9896;
	// 8326A6A8: 386AC1A0  addi r3, r10, -0x3e60
	ctx.r[3].s64 = ctx.r[10].s64 + -15968;
	// 8326A6AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326A6B0: 4AFC2821  bl 0x8222ced0
	ctx.lr = 0x8326A6B4;
	sub_8222CED0(ctx, base);
	// 8326A6B4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326A6B8: 3869E320  addi r3, r9, -0x1ce0
	ctx.r[3].s64 = ctx.r[9].s64 + -7392;
	// 8326A6BC: 4BA3F865  bl 0x82ca9f20
	ctx.lr = 0x8326A6C0;
	sub_82CA9F20(ctx, base);
	// 8326A6C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326A6C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326A6C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326A6CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326A6D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A6D0 size=64
    let mut pc: u32 = 0x8326A6D0;
    'dispatch: loop {
        match pc {
            0x8326A6D0 => {
    //   block [0x8326A6D0..0x8326A710)
	// 8326A6D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A6D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A6D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326A6DC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326A6E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326A6E4: 388BD96C  addi r4, r11, -0x2694
	ctx.r[4].s64 = ctx.r[11].s64 + -9876;
	// 8326A6E8: 386AC1A4  addi r3, r10, -0x3e5c
	ctx.r[3].s64 = ctx.r[10].s64 + -15964;
	// 8326A6EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326A6F0: 4AFC27E1  bl 0x8222ced0
	ctx.lr = 0x8326A6F4;
	sub_8222CED0(ctx, base);
	// 8326A6F4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326A6F8: 3869E330  addi r3, r9, -0x1cd0
	ctx.r[3].s64 = ctx.r[9].s64 + -7376;
	// 8326A6FC: 4BA3F825  bl 0x82ca9f20
	ctx.lr = 0x8326A700;
	sub_82CA9F20(ctx, base);
	// 8326A700: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326A704: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326A708: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326A70C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326A710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A710 size=64
    let mut pc: u32 = 0x8326A710;
    'dispatch: loop {
        match pc {
            0x8326A710 => {
    //   block [0x8326A710..0x8326A750)
	// 8326A710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A714: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A718: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326A71C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326A720: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326A724: 388BD98C  addi r4, r11, -0x2674
	ctx.r[4].s64 = ctx.r[11].s64 + -9844;
	// 8326A728: 386AC1A8  addi r3, r10, -0x3e58
	ctx.r[3].s64 = ctx.r[10].s64 + -15960;
	// 8326A72C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326A730: 4AFC27A1  bl 0x8222ced0
	ctx.lr = 0x8326A734;
	sub_8222CED0(ctx, base);
	// 8326A734: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326A738: 3869E340  addi r3, r9, -0x1cc0
	ctx.r[3].s64 = ctx.r[9].s64 + -7360;
	// 8326A73C: 4BA3F7E5  bl 0x82ca9f20
	ctx.lr = 0x8326A740;
	sub_82CA9F20(ctx, base);
	// 8326A740: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326A744: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326A748: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326A74C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326A750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A750 size=64
    let mut pc: u32 = 0x8326A750;
    'dispatch: loop {
        match pc {
            0x8326A750 => {
    //   block [0x8326A750..0x8326A790)
	// 8326A750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A758: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326A75C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326A760: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326A764: 388BD9A8  addi r4, r11, -0x2658
	ctx.r[4].s64 = ctx.r[11].s64 + -9816;
	// 8326A768: 386AC1AC  addi r3, r10, -0x3e54
	ctx.r[3].s64 = ctx.r[10].s64 + -15956;
	// 8326A76C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326A770: 4AFC2761  bl 0x8222ced0
	ctx.lr = 0x8326A774;
	sub_8222CED0(ctx, base);
	// 8326A774: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326A778: 3869E350  addi r3, r9, -0x1cb0
	ctx.r[3].s64 = ctx.r[9].s64 + -7344;
	// 8326A77C: 4BA3F7A5  bl 0x82ca9f20
	ctx.lr = 0x8326A780;
	sub_82CA9F20(ctx, base);
	// 8326A780: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326A784: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326A788: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326A78C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326A790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A790 size=64
    let mut pc: u32 = 0x8326A790;
    'dispatch: loop {
        match pc {
            0x8326A790 => {
    //   block [0x8326A790..0x8326A7D0)
	// 8326A790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A798: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326A79C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326A7A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326A7A4: 388BD9C4  addi r4, r11, -0x263c
	ctx.r[4].s64 = ctx.r[11].s64 + -9788;
	// 8326A7A8: 386AC1B0  addi r3, r10, -0x3e50
	ctx.r[3].s64 = ctx.r[10].s64 + -15952;
	// 8326A7AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326A7B0: 4AFC2721  bl 0x8222ced0
	ctx.lr = 0x8326A7B4;
	sub_8222CED0(ctx, base);
	// 8326A7B4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326A7B8: 3869E360  addi r3, r9, -0x1ca0
	ctx.r[3].s64 = ctx.r[9].s64 + -7328;
	// 8326A7BC: 4BA3F765  bl 0x82ca9f20
	ctx.lr = 0x8326A7C0;
	sub_82CA9F20(ctx, base);
	// 8326A7C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326A7C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326A7C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326A7CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326A7D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A7D0 size=64
    let mut pc: u32 = 0x8326A7D0;
    'dispatch: loop {
        match pc {
            0x8326A7D0 => {
    //   block [0x8326A7D0..0x8326A810)
	// 8326A7D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A7D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A7D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326A7DC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326A7E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326A7E4: 388BD9D8  addi r4, r11, -0x2628
	ctx.r[4].s64 = ctx.r[11].s64 + -9768;
	// 8326A7E8: 386AC1B4  addi r3, r10, -0x3e4c
	ctx.r[3].s64 = ctx.r[10].s64 + -15948;
	// 8326A7EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326A7F0: 4AFC26E1  bl 0x8222ced0
	ctx.lr = 0x8326A7F4;
	sub_8222CED0(ctx, base);
	// 8326A7F4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326A7F8: 3869E370  addi r3, r9, -0x1c90
	ctx.r[3].s64 = ctx.r[9].s64 + -7312;
	// 8326A7FC: 4BA3F725  bl 0x82ca9f20
	ctx.lr = 0x8326A800;
	sub_82CA9F20(ctx, base);
	// 8326A800: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326A804: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326A808: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326A80C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326A810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A810 size=64
    let mut pc: u32 = 0x8326A810;
    'dispatch: loop {
        match pc {
            0x8326A810 => {
    //   block [0x8326A810..0x8326A850)
	// 8326A810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A818: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326A81C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326A820: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326A824: 388BD9F4  addi r4, r11, -0x260c
	ctx.r[4].s64 = ctx.r[11].s64 + -9740;
	// 8326A828: 386AC1B8  addi r3, r10, -0x3e48
	ctx.r[3].s64 = ctx.r[10].s64 + -15944;
	// 8326A82C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326A830: 4AFC26A1  bl 0x8222ced0
	ctx.lr = 0x8326A834;
	sub_8222CED0(ctx, base);
	// 8326A834: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326A838: 3869E380  addi r3, r9, -0x1c80
	ctx.r[3].s64 = ctx.r[9].s64 + -7296;
	// 8326A83C: 4BA3F6E5  bl 0x82ca9f20
	ctx.lr = 0x8326A840;
	sub_82CA9F20(ctx, base);
	// 8326A840: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326A844: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326A848: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326A84C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326A850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A850 size=64
    let mut pc: u32 = 0x8326A850;
    'dispatch: loop {
        match pc {
            0x8326A850 => {
    //   block [0x8326A850..0x8326A890)
	// 8326A850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A858: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326A85C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326A860: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326A864: 388BDA18  addi r4, r11, -0x25e8
	ctx.r[4].s64 = ctx.r[11].s64 + -9704;
	// 8326A868: 386AC1BC  addi r3, r10, -0x3e44
	ctx.r[3].s64 = ctx.r[10].s64 + -15940;
	// 8326A86C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326A870: 4AFC2661  bl 0x8222ced0
	ctx.lr = 0x8326A874;
	sub_8222CED0(ctx, base);
	// 8326A874: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326A878: 3869E390  addi r3, r9, -0x1c70
	ctx.r[3].s64 = ctx.r[9].s64 + -7280;
	// 8326A87C: 4BA3F6A5  bl 0x82ca9f20
	ctx.lr = 0x8326A880;
	sub_82CA9F20(ctx, base);
	// 8326A880: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326A884: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326A888: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326A88C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326A890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A890 size=64
    let mut pc: u32 = 0x8326A890;
    'dispatch: loop {
        match pc {
            0x8326A890 => {
    //   block [0x8326A890..0x8326A8D0)
	// 8326A890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A898: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326A89C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326A8A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326A8A4: 388BDA2C  addi r4, r11, -0x25d4
	ctx.r[4].s64 = ctx.r[11].s64 + -9684;
	// 8326A8A8: 386AC1C0  addi r3, r10, -0x3e40
	ctx.r[3].s64 = ctx.r[10].s64 + -15936;
	// 8326A8AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326A8B0: 4AFC2621  bl 0x8222ced0
	ctx.lr = 0x8326A8B4;
	sub_8222CED0(ctx, base);
	// 8326A8B4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326A8B8: 3869E3A0  addi r3, r9, -0x1c60
	ctx.r[3].s64 = ctx.r[9].s64 + -7264;
	// 8326A8BC: 4BA3F665  bl 0x82ca9f20
	ctx.lr = 0x8326A8C0;
	sub_82CA9F20(ctx, base);
	// 8326A8C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326A8C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326A8C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326A8CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326A8D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A8D0 size=64
    let mut pc: u32 = 0x8326A8D0;
    'dispatch: loop {
        match pc {
            0x8326A8D0 => {
    //   block [0x8326A8D0..0x8326A910)
	// 8326A8D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A8D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A8D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326A8DC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326A8E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326A8E4: 388BDA48  addi r4, r11, -0x25b8
	ctx.r[4].s64 = ctx.r[11].s64 + -9656;
	// 8326A8E8: 386AC1C4  addi r3, r10, -0x3e3c
	ctx.r[3].s64 = ctx.r[10].s64 + -15932;
	// 8326A8EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326A8F0: 4AFC25E1  bl 0x8222ced0
	ctx.lr = 0x8326A8F4;
	sub_8222CED0(ctx, base);
	// 8326A8F4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326A8F8: 3869E3B0  addi r3, r9, -0x1c50
	ctx.r[3].s64 = ctx.r[9].s64 + -7248;
	// 8326A8FC: 4BA3F625  bl 0x82ca9f20
	ctx.lr = 0x8326A900;
	sub_82CA9F20(ctx, base);
	// 8326A900: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326A904: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326A908: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326A90C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326A910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A910 size=64
    let mut pc: u32 = 0x8326A910;
    'dispatch: loop {
        match pc {
            0x8326A910 => {
    //   block [0x8326A910..0x8326A950)
	// 8326A910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A918: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326A91C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326A920: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326A924: 388BDA58  addi r4, r11, -0x25a8
	ctx.r[4].s64 = ctx.r[11].s64 + -9640;
	// 8326A928: 386AC1C8  addi r3, r10, -0x3e38
	ctx.r[3].s64 = ctx.r[10].s64 + -15928;
	// 8326A92C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326A930: 4AFC25A1  bl 0x8222ced0
	ctx.lr = 0x8326A934;
	sub_8222CED0(ctx, base);
	// 8326A934: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326A938: 3869E3C0  addi r3, r9, -0x1c40
	ctx.r[3].s64 = ctx.r[9].s64 + -7232;
	// 8326A93C: 4BA3F5E5  bl 0x82ca9f20
	ctx.lr = 0x8326A940;
	sub_82CA9F20(ctx, base);
	// 8326A940: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326A944: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326A948: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326A94C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326A950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A950 size=64
    let mut pc: u32 = 0x8326A950;
    'dispatch: loop {
        match pc {
            0x8326A950 => {
    //   block [0x8326A950..0x8326A990)
	// 8326A950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A958: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326A95C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326A960: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326A964: 388BDA64  addi r4, r11, -0x259c
	ctx.r[4].s64 = ctx.r[11].s64 + -9628;
	// 8326A968: 386AC1CC  addi r3, r10, -0x3e34
	ctx.r[3].s64 = ctx.r[10].s64 + -15924;
	// 8326A96C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326A970: 4AFC2561  bl 0x8222ced0
	ctx.lr = 0x8326A974;
	sub_8222CED0(ctx, base);
	// 8326A974: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326A978: 3869E3D0  addi r3, r9, -0x1c30
	ctx.r[3].s64 = ctx.r[9].s64 + -7216;
	// 8326A97C: 4BA3F5A5  bl 0x82ca9f20
	ctx.lr = 0x8326A980;
	sub_82CA9F20(ctx, base);
	// 8326A980: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326A984: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326A988: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326A98C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326A990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A990 size=64
    let mut pc: u32 = 0x8326A990;
    'dispatch: loop {
        match pc {
            0x8326A990 => {
    //   block [0x8326A990..0x8326A9D0)
	// 8326A990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A998: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326A99C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326A9A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326A9A4: 388BDA7C  addi r4, r11, -0x2584
	ctx.r[4].s64 = ctx.r[11].s64 + -9604;
	// 8326A9A8: 386AC1D0  addi r3, r10, -0x3e30
	ctx.r[3].s64 = ctx.r[10].s64 + -15920;
	// 8326A9AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326A9B0: 4AFC2521  bl 0x8222ced0
	ctx.lr = 0x8326A9B4;
	sub_8222CED0(ctx, base);
	// 8326A9B4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326A9B8: 3869E3E0  addi r3, r9, -0x1c20
	ctx.r[3].s64 = ctx.r[9].s64 + -7200;
	// 8326A9BC: 4BA3F565  bl 0x82ca9f20
	ctx.lr = 0x8326A9C0;
	sub_82CA9F20(ctx, base);
	// 8326A9C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326A9C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326A9C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326A9CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326A9D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A9D0 size=64
    let mut pc: u32 = 0x8326A9D0;
    'dispatch: loop {
        match pc {
            0x8326A9D0 => {
    //   block [0x8326A9D0..0x8326AA10)
	// 8326A9D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A9D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A9D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326A9DC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326A9E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326A9E4: 388BDA98  addi r4, r11, -0x2568
	ctx.r[4].s64 = ctx.r[11].s64 + -9576;
	// 8326A9E8: 386AC1D4  addi r3, r10, -0x3e2c
	ctx.r[3].s64 = ctx.r[10].s64 + -15916;
	// 8326A9EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326A9F0: 4AFC24E1  bl 0x8222ced0
	ctx.lr = 0x8326A9F4;
	sub_8222CED0(ctx, base);
	// 8326A9F4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326A9F8: 3869E3F0  addi r3, r9, -0x1c10
	ctx.r[3].s64 = ctx.r[9].s64 + -7184;
	// 8326A9FC: 4BA3F525  bl 0x82ca9f20
	ctx.lr = 0x8326AA00;
	sub_82CA9F20(ctx, base);
	// 8326AA00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326AA04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326AA08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326AA0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326AA10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326AA10 size=64
    let mut pc: u32 = 0x8326AA10;
    'dispatch: loop {
        match pc {
            0x8326AA10 => {
    //   block [0x8326AA10..0x8326AA50)
	// 8326AA10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326AA14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326AA18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326AA1C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326AA20: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326AA24: 388BDAAC  addi r4, r11, -0x2554
	ctx.r[4].s64 = ctx.r[11].s64 + -9556;
	// 8326AA28: 386AC1D8  addi r3, r10, -0x3e28
	ctx.r[3].s64 = ctx.r[10].s64 + -15912;
	// 8326AA2C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326AA30: 4AFC24A1  bl 0x8222ced0
	ctx.lr = 0x8326AA34;
	sub_8222CED0(ctx, base);
	// 8326AA34: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326AA38: 3869E400  addi r3, r9, -0x1c00
	ctx.r[3].s64 = ctx.r[9].s64 + -7168;
	// 8326AA3C: 4BA3F4E5  bl 0x82ca9f20
	ctx.lr = 0x8326AA40;
	sub_82CA9F20(ctx, base);
	// 8326AA40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326AA44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326AA48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326AA4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326AA50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326AA50 size=64
    let mut pc: u32 = 0x8326AA50;
    'dispatch: loop {
        match pc {
            0x8326AA50 => {
    //   block [0x8326AA50..0x8326AA90)
	// 8326AA50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326AA54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326AA58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326AA5C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326AA60: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326AA64: 388BDAC4  addi r4, r11, -0x253c
	ctx.r[4].s64 = ctx.r[11].s64 + -9532;
	// 8326AA68: 386AC1DC  addi r3, r10, -0x3e24
	ctx.r[3].s64 = ctx.r[10].s64 + -15908;
	// 8326AA6C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326AA70: 4AFC2461  bl 0x8222ced0
	ctx.lr = 0x8326AA74;
	sub_8222CED0(ctx, base);
	// 8326AA74: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326AA78: 3869E410  addi r3, r9, -0x1bf0
	ctx.r[3].s64 = ctx.r[9].s64 + -7152;
	// 8326AA7C: 4BA3F4A5  bl 0x82ca9f20
	ctx.lr = 0x8326AA80;
	sub_82CA9F20(ctx, base);
	// 8326AA80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326AA84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326AA88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326AA8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326AA90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326AA90 size=64
    let mut pc: u32 = 0x8326AA90;
    'dispatch: loop {
        match pc {
            0x8326AA90 => {
    //   block [0x8326AA90..0x8326AAD0)
	// 8326AA90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326AA94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326AA98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326AA9C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326AAA0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326AAA4: 388BDAD4  addi r4, r11, -0x252c
	ctx.r[4].s64 = ctx.r[11].s64 + -9516;
	// 8326AAA8: 386AC1E0  addi r3, r10, -0x3e20
	ctx.r[3].s64 = ctx.r[10].s64 + -15904;
	// 8326AAAC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326AAB0: 4AFC2421  bl 0x8222ced0
	ctx.lr = 0x8326AAB4;
	sub_8222CED0(ctx, base);
	// 8326AAB4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326AAB8: 3869E420  addi r3, r9, -0x1be0
	ctx.r[3].s64 = ctx.r[9].s64 + -7136;
	// 8326AABC: 4BA3F465  bl 0x82ca9f20
	ctx.lr = 0x8326AAC0;
	sub_82CA9F20(ctx, base);
	// 8326AAC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326AAC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326AAC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326AACC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326AAD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326AAD0 size=64
    let mut pc: u32 = 0x8326AAD0;
    'dispatch: loop {
        match pc {
            0x8326AAD0 => {
    //   block [0x8326AAD0..0x8326AB10)
	// 8326AAD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326AAD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326AAD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326AADC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326AAE0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326AAE4: 388BDAE8  addi r4, r11, -0x2518
	ctx.r[4].s64 = ctx.r[11].s64 + -9496;
	// 8326AAE8: 386AC1E4  addi r3, r10, -0x3e1c
	ctx.r[3].s64 = ctx.r[10].s64 + -15900;
	// 8326AAEC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326AAF0: 4AFC23E1  bl 0x8222ced0
	ctx.lr = 0x8326AAF4;
	sub_8222CED0(ctx, base);
	// 8326AAF4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326AAF8: 3869E430  addi r3, r9, -0x1bd0
	ctx.r[3].s64 = ctx.r[9].s64 + -7120;
	// 8326AAFC: 4BA3F425  bl 0x82ca9f20
	ctx.lr = 0x8326AB00;
	sub_82CA9F20(ctx, base);
	// 8326AB00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326AB04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326AB08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326AB0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326AB10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326AB10 size=56
    let mut pc: u32 = 0x8326AB10;
    'dispatch: loop {
        match pc {
            0x8326AB10 => {
    //   block [0x8326AB10..0x8326AB48)
	// 8326AB10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326AB14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326AB18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326AB1C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326AB20: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326AB24: 386BDAFC  addi r3, r11, -0x2504
	ctx.r[3].s64 = ctx.r[11].s64 + -9476;
	// 8326AB28: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326AB2C: 4AF8922D  bl 0x821f3d58
	ctx.lr = 0x8326AB30;
	sub_821F3D58(ctx, base);
	// 8326AB30: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326AB34: 906AC1E8  stw r3, -0x3e18(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-15896 as u32), ctx.r[3].u32 ) };
	// 8326AB38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326AB3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326AB40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326AB44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326AB48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326AB48 size=56
    let mut pc: u32 = 0x8326AB48;
    'dispatch: loop {
        match pc {
            0x8326AB48 => {
    //   block [0x8326AB48..0x8326AB80)
	// 8326AB48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326AB4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326AB50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326AB54: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326AB58: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326AB5C: 386BDB10  addi r3, r11, -0x24f0
	ctx.r[3].s64 = ctx.r[11].s64 + -9456;
	// 8326AB60: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326AB64: 4AF891F5  bl 0x821f3d58
	ctx.lr = 0x8326AB68;
	sub_821F3D58(ctx, base);
	// 8326AB68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326AB6C: 906AC1EC  stw r3, -0x3e14(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-15892 as u32), ctx.r[3].u32 ) };
	// 8326AB70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326AB74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326AB78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326AB7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326AB80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326AB80 size=56
    let mut pc: u32 = 0x8326AB80;
    'dispatch: loop {
        match pc {
            0x8326AB80 => {
    //   block [0x8326AB80..0x8326ABB8)
	// 8326AB80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326AB84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326AB88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326AB8C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326AB90: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326AB94: 386BDB24  addi r3, r11, -0x24dc
	ctx.r[3].s64 = ctx.r[11].s64 + -9436;
	// 8326AB98: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326AB9C: 4AF891BD  bl 0x821f3d58
	ctx.lr = 0x8326ABA0;
	sub_821F3D58(ctx, base);
	// 8326ABA0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326ABA4: 906AC1F0  stw r3, -0x3e10(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-15888 as u32), ctx.r[3].u32 ) };
	// 8326ABA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326ABAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326ABB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326ABB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326ABB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326ABB8 size=56
    let mut pc: u32 = 0x8326ABB8;
    'dispatch: loop {
        match pc {
            0x8326ABB8 => {
    //   block [0x8326ABB8..0x8326ABF0)
	// 8326ABB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326ABBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326ABC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326ABC4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326ABC8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326ABCC: 386BDB38  addi r3, r11, -0x24c8
	ctx.r[3].s64 = ctx.r[11].s64 + -9416;
	// 8326ABD0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326ABD4: 4AF89185  bl 0x821f3d58
	ctx.lr = 0x8326ABD8;
	sub_821F3D58(ctx, base);
	// 8326ABD8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326ABDC: 906AC1F4  stw r3, -0x3e0c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-15884 as u32), ctx.r[3].u32 ) };
	// 8326ABE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326ABE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326ABE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326ABEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326ABF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326ABF0 size=64
    let mut pc: u32 = 0x8326ABF0;
    'dispatch: loop {
        match pc {
            0x8326ABF0 => {
    //   block [0x8326ABF0..0x8326AC30)
	// 8326ABF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326ABF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326ABF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326ABFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326AC00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326AC04: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 8326AC08: 386AC1F8  addi r3, r10, -0x3e08
	ctx.r[3].s64 = ctx.r[10].s64 + -15880;
	// 8326AC0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326AC10: 4AFC22C1  bl 0x8222ced0
	ctx.lr = 0x8326AC14;
	sub_8222CED0(ctx, base);
	// 8326AC14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326AC18: 3869E440  addi r3, r9, -0x1bc0
	ctx.r[3].s64 = ctx.r[9].s64 + -7104;
	// 8326AC1C: 4BA3F305  bl 0x82ca9f20
	ctx.lr = 0x8326AC20;
	sub_82CA9F20(ctx, base);
	// 8326AC20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326AC24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326AC28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326AC2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326AC30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326AC30 size=64
    let mut pc: u32 = 0x8326AC30;
    'dispatch: loop {
        match pc {
            0x8326AC30 => {
    //   block [0x8326AC30..0x8326AC70)
	// 8326AC30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326AC34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326AC38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326AC3C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326AC40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326AC44: 388BE14C  addi r4, r11, -0x1eb4
	ctx.r[4].s64 = ctx.r[11].s64 + -7860;
	// 8326AC48: 386AC1FC  addi r3, r10, -0x3e04
	ctx.r[3].s64 = ctx.r[10].s64 + -15876;
	// 8326AC4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326AC50: 4AFC2281  bl 0x8222ced0
	ctx.lr = 0x8326AC54;
	sub_8222CED0(ctx, base);
	// 8326AC54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326AC58: 3869E450  addi r3, r9, -0x1bb0
	ctx.r[3].s64 = ctx.r[9].s64 + -7088;
	// 8326AC5C: 4BA3F2C5  bl 0x82ca9f20
	ctx.lr = 0x8326AC60;
	sub_82CA9F20(ctx, base);
	// 8326AC60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326AC64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326AC68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326AC6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326AC70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326AC70 size=64
    let mut pc: u32 = 0x8326AC70;
    'dispatch: loop {
        match pc {
            0x8326AC70 => {
    //   block [0x8326AC70..0x8326ACB0)
	// 8326AC70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326AC74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326AC78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326AC7C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326AC80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326AC84: 388BE160  addi r4, r11, -0x1ea0
	ctx.r[4].s64 = ctx.r[11].s64 + -7840;
	// 8326AC88: 386AC200  addi r3, r10, -0x3e00
	ctx.r[3].s64 = ctx.r[10].s64 + -15872;
	// 8326AC8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326AC90: 4AFC2241  bl 0x8222ced0
	ctx.lr = 0x8326AC94;
	sub_8222CED0(ctx, base);
	// 8326AC94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326AC98: 3869E460  addi r3, r9, -0x1ba0
	ctx.r[3].s64 = ctx.r[9].s64 + -7072;
	// 8326AC9C: 4BA3F285  bl 0x82ca9f20
	ctx.lr = 0x8326ACA0;
	sub_82CA9F20(ctx, base);
	// 8326ACA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326ACA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326ACA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326ACAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326ACB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326ACB0 size=64
    let mut pc: u32 = 0x8326ACB0;
    'dispatch: loop {
        match pc {
            0x8326ACB0 => {
    //   block [0x8326ACB0..0x8326ACF0)
	// 8326ACB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326ACB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326ACB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326ACBC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326ACC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326ACC4: 388BE184  addi r4, r11, -0x1e7c
	ctx.r[4].s64 = ctx.r[11].s64 + -7804;
	// 8326ACC8: 386AC204  addi r3, r10, -0x3dfc
	ctx.r[3].s64 = ctx.r[10].s64 + -15868;
	// 8326ACCC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326ACD0: 4AFC2201  bl 0x8222ced0
	ctx.lr = 0x8326ACD4;
	sub_8222CED0(ctx, base);
	// 8326ACD4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326ACD8: 3869E470  addi r3, r9, -0x1b90
	ctx.r[3].s64 = ctx.r[9].s64 + -7056;
	// 8326ACDC: 4BA3F245  bl 0x82ca9f20
	ctx.lr = 0x8326ACE0;
	sub_82CA9F20(ctx, base);
	// 8326ACE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326ACE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326ACE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326ACEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326ACF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326ACF0 size=64
    let mut pc: u32 = 0x8326ACF0;
    'dispatch: loop {
        match pc {
            0x8326ACF0 => {
    //   block [0x8326ACF0..0x8326AD30)
	// 8326ACF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326ACF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326ACF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326ACFC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326AD00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326AD04: 388BE1A0  addi r4, r11, -0x1e60
	ctx.r[4].s64 = ctx.r[11].s64 + -7776;
	// 8326AD08: 386AC208  addi r3, r10, -0x3df8
	ctx.r[3].s64 = ctx.r[10].s64 + -15864;
	// 8326AD0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326AD10: 4AFC21C1  bl 0x8222ced0
	ctx.lr = 0x8326AD14;
	sub_8222CED0(ctx, base);
	// 8326AD14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326AD18: 3869E480  addi r3, r9, -0x1b80
	ctx.r[3].s64 = ctx.r[9].s64 + -7040;
	// 8326AD1C: 4BA3F205  bl 0x82ca9f20
	ctx.lr = 0x8326AD20;
	sub_82CA9F20(ctx, base);
	// 8326AD20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326AD24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326AD28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326AD2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326AD30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326AD30 size=64
    let mut pc: u32 = 0x8326AD30;
    'dispatch: loop {
        match pc {
            0x8326AD30 => {
    //   block [0x8326AD30..0x8326AD70)
	// 8326AD30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326AD34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326AD38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326AD3C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326AD40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326AD44: 388BE1BC  addi r4, r11, -0x1e44
	ctx.r[4].s64 = ctx.r[11].s64 + -7748;
	// 8326AD48: 386AC20C  addi r3, r10, -0x3df4
	ctx.r[3].s64 = ctx.r[10].s64 + -15860;
	// 8326AD4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326AD50: 4AFC2181  bl 0x8222ced0
	ctx.lr = 0x8326AD54;
	sub_8222CED0(ctx, base);
	// 8326AD54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326AD58: 3869E490  addi r3, r9, -0x1b70
	ctx.r[3].s64 = ctx.r[9].s64 + -7024;
	// 8326AD5C: 4BA3F1C5  bl 0x82ca9f20
	ctx.lr = 0x8326AD60;
	sub_82CA9F20(ctx, base);
	// 8326AD60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326AD64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326AD68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326AD6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326AD70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326AD70 size=64
    let mut pc: u32 = 0x8326AD70;
    'dispatch: loop {
        match pc {
            0x8326AD70 => {
    //   block [0x8326AD70..0x8326ADB0)
	// 8326AD70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326AD74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326AD78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326AD7C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326AD80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326AD84: 388BA398  addi r4, r11, -0x5c68
	ctx.r[4].s64 = ctx.r[11].s64 + -23656;
	// 8326AD88: 386AC210  addi r3, r10, -0x3df0
	ctx.r[3].s64 = ctx.r[10].s64 + -15856;
	// 8326AD8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326AD90: 4AFC2141  bl 0x8222ced0
	ctx.lr = 0x8326AD94;
	sub_8222CED0(ctx, base);
	// 8326AD94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326AD98: 3869E4A0  addi r3, r9, -0x1b60
	ctx.r[3].s64 = ctx.r[9].s64 + -7008;
	// 8326AD9C: 4BA3F185  bl 0x82ca9f20
	ctx.lr = 0x8326ADA0;
	sub_82CA9F20(ctx, base);
	// 8326ADA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326ADA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326ADA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326ADAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326ADB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326ADB0 size=64
    let mut pc: u32 = 0x8326ADB0;
    'dispatch: loop {
        match pc {
            0x8326ADB0 => {
    //   block [0x8326ADB0..0x8326ADF0)
	// 8326ADB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326ADB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326ADB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326ADBC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326ADC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326ADC4: 388BE1DC  addi r4, r11, -0x1e24
	ctx.r[4].s64 = ctx.r[11].s64 + -7716;
	// 8326ADC8: 386AC214  addi r3, r10, -0x3dec
	ctx.r[3].s64 = ctx.r[10].s64 + -15852;
	// 8326ADCC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326ADD0: 4AFC2101  bl 0x8222ced0
	ctx.lr = 0x8326ADD4;
	sub_8222CED0(ctx, base);
	// 8326ADD4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326ADD8: 3869E4B0  addi r3, r9, -0x1b50
	ctx.r[3].s64 = ctx.r[9].s64 + -6992;
	// 8326ADDC: 4BA3F145  bl 0x82ca9f20
	ctx.lr = 0x8326ADE0;
	sub_82CA9F20(ctx, base);
	// 8326ADE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326ADE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326ADE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326ADEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326ADF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326ADF0 size=64
    let mut pc: u32 = 0x8326ADF0;
    'dispatch: loop {
        match pc {
            0x8326ADF0 => {
    //   block [0x8326ADF0..0x8326AE30)
	// 8326ADF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326ADF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326ADF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326ADFC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326AE00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326AE04: 388BE1FC  addi r4, r11, -0x1e04
	ctx.r[4].s64 = ctx.r[11].s64 + -7684;
	// 8326AE08: 386AC218  addi r3, r10, -0x3de8
	ctx.r[3].s64 = ctx.r[10].s64 + -15848;
	// 8326AE0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326AE10: 4AFC20C1  bl 0x8222ced0
	ctx.lr = 0x8326AE14;
	sub_8222CED0(ctx, base);
	// 8326AE14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326AE18: 3869E4C0  addi r3, r9, -0x1b40
	ctx.r[3].s64 = ctx.r[9].s64 + -6976;
	// 8326AE1C: 4BA3F105  bl 0x82ca9f20
	ctx.lr = 0x8326AE20;
	sub_82CA9F20(ctx, base);
	// 8326AE20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326AE24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326AE28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326AE2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326AE30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326AE30 size=64
    let mut pc: u32 = 0x8326AE30;
    'dispatch: loop {
        match pc {
            0x8326AE30 => {
    //   block [0x8326AE30..0x8326AE70)
	// 8326AE30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326AE34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326AE38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326AE3C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326AE40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326AE44: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8326AE48: 386AC21C  addi r3, r10, -0x3de4
	ctx.r[3].s64 = ctx.r[10].s64 + -15844;
	// 8326AE4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326AE50: 4AFC2081  bl 0x8222ced0
	ctx.lr = 0x8326AE54;
	sub_8222CED0(ctx, base);
	// 8326AE54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326AE58: 3869E4D0  addi r3, r9, -0x1b30
	ctx.r[3].s64 = ctx.r[9].s64 + -6960;
	// 8326AE5C: 4BA3F0C5  bl 0x82ca9f20
	ctx.lr = 0x8326AE60;
	sub_82CA9F20(ctx, base);
	// 8326AE60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326AE64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326AE68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326AE6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326AE70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326AE70 size=64
    let mut pc: u32 = 0x8326AE70;
    'dispatch: loop {
        match pc {
            0x8326AE70 => {
    //   block [0x8326AE70..0x8326AEB0)
	// 8326AE70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326AE74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326AE78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326AE7C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326AE80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326AE84: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8326AE88: 386AC220  addi r3, r10, -0x3de0
	ctx.r[3].s64 = ctx.r[10].s64 + -15840;
	// 8326AE8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326AE90: 4AFC2041  bl 0x8222ced0
	ctx.lr = 0x8326AE94;
	sub_8222CED0(ctx, base);
	// 8326AE94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326AE98: 3869E4E0  addi r3, r9, -0x1b20
	ctx.r[3].s64 = ctx.r[9].s64 + -6944;
	// 8326AE9C: 4BA3F085  bl 0x82ca9f20
	ctx.lr = 0x8326AEA0;
	sub_82CA9F20(ctx, base);
	// 8326AEA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326AEA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326AEA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326AEAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326AEB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326AEB0 size=64
    let mut pc: u32 = 0x8326AEB0;
    'dispatch: loop {
        match pc {
            0x8326AEB0 => {
    //   block [0x8326AEB0..0x8326AEF0)
	// 8326AEB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326AEB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326AEB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326AEBC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326AEC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326AEC4: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8326AEC8: 386AC224  addi r3, r10, -0x3ddc
	ctx.r[3].s64 = ctx.r[10].s64 + -15836;
	// 8326AECC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326AED0: 4AFC2001  bl 0x8222ced0
	ctx.lr = 0x8326AED4;
	sub_8222CED0(ctx, base);
	// 8326AED4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326AED8: 3869E4F0  addi r3, r9, -0x1b10
	ctx.r[3].s64 = ctx.r[9].s64 + -6928;
	// 8326AEDC: 4BA3F045  bl 0x82ca9f20
	ctx.lr = 0x8326AEE0;
	sub_82CA9F20(ctx, base);
	// 8326AEE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326AEE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326AEE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326AEEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326AEF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326AEF0 size=192
    let mut pc: u32 = 0x8326AEF0;
    'dispatch: loop {
        match pc {
            0x8326AEF0 => {
    //   block [0x8326AEF0..0x8326AF48)
	// 8326AEF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326AEF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326AEF8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8326AEFC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326AF00: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326AF04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326AF08: 388BE364  addi r4, r11, -0x1c9c
	ctx.r[4].s64 = ctx.r[11].s64 + -7324;
	// 8326AF0C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8326AF10: 4AFC1FC1  bl 0x8222ced0
	ctx.lr = 0x8326AF14;
	sub_8222CED0(ctx, base);
	// 8326AF14: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8326AF18: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8326AF1C: 4AF83C1D  bl 0x821eeb38
	ctx.lr = 0x8326AF20;
	sub_821EEB38(ctx, base);
	// 8326AF20: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8326AF24: 4B9988CD  bl 0x82c037f0
	ctx.lr = 0x8326AF28;
	sub_82C037F0(ctx, base);
	// 8326AF28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326AF2C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8326AF30: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8326AF34: 916AC228  stw r11, -0x3dd8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-15832 as u32), ctx.r[11].u32 ) };
	// 8326AF38: 4AF5B831  bl 0x821c6768
	ctx.lr = 0x8326AF3C;
	sub_821C6768(ctx, base);
	// 8326AF3C: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 8326AF40: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 8326AF44: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	pc = 0x8326AF48; continue 'dispatch;
            }
            0x8326AF48 => {
    //   block [0x8326AF48..0x8326AF74)
	// 8326AF48: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 8326AF4C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8326AF50: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 8326AF54: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 8326AF58: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8326AF5C: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8326AF60: 4082FFE8  bne 0x8326af48
	if !ctx.cr[0].eq {
	pc = 0x8326AF48; continue 'dispatch;
	}
	// 8326AF64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8326AF68: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8326AF6C: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 8326AF70: 4AF5B7F9  bl 0x821c6768
	ctx.lr = 0x8326AF74;
	sub_821C6768(ctx, base);
	pc = 0x8326AF74; continue 'dispatch;
            }
            0x8326AF74 => {
    //   block [0x8326AF74..0x8326AFB0)
	// 8326AF74: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 8326AF78: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8326AF7C: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 8326AF80: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 8326AF84: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8326AF88: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8326AF8C: 4082FFE8  bne 0x8326af74
	if !ctx.cr[0].eq {
	pc = 0x8326AF74; continue 'dispatch;
	}
	// 8326AF90: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8326AF94: 386BE500  addi r3, r11, -0x1b00
	ctx.r[3].s64 = ctx.r[11].s64 + -6912;
	// 8326AF98: 4BA3EF89  bl 0x82ca9f20
	ctx.lr = 0x8326AF9C;
	sub_82CA9F20(ctx, base);
	// 8326AF9C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8326AFA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326AFA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326AFA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8326AFAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326AFB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326AFB0 size=192
    let mut pc: u32 = 0x8326AFB0;
    'dispatch: loop {
        match pc {
            0x8326AFB0 => {
    //   block [0x8326AFB0..0x8326B008)
	// 8326AFB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326AFB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326AFB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8326AFBC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326AFC0: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326AFC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326AFC8: 388BE394  addi r4, r11, -0x1c6c
	ctx.r[4].s64 = ctx.r[11].s64 + -7276;
	// 8326AFCC: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8326AFD0: 4AFC1F01  bl 0x8222ced0
	ctx.lr = 0x8326AFD4;
	sub_8222CED0(ctx, base);
	// 8326AFD4: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8326AFD8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8326AFDC: 4AF83B5D  bl 0x821eeb38
	ctx.lr = 0x8326AFE0;
	sub_821EEB38(ctx, base);
	// 8326AFE0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8326AFE4: 4B99880D  bl 0x82c037f0
	ctx.lr = 0x8326AFE8;
	sub_82C037F0(ctx, base);
	// 8326AFE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326AFEC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8326AFF0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8326AFF4: 916AC22C  stw r11, -0x3dd4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-15828 as u32), ctx.r[11].u32 ) };
	// 8326AFF8: 4AF5B771  bl 0x821c6768
	ctx.lr = 0x8326AFFC;
	sub_821C6768(ctx, base);
	// 8326AFFC: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 8326B000: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 8326B004: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	pc = 0x8326B008; continue 'dispatch;
            }
            0x8326B008 => {
    //   block [0x8326B008..0x8326B034)
	// 8326B008: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 8326B00C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8326B010: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 8326B014: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 8326B018: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8326B01C: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8326B020: 4082FFE8  bne 0x8326b008
	if !ctx.cr[0].eq {
	pc = 0x8326B008; continue 'dispatch;
	}
	// 8326B024: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8326B028: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8326B02C: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 8326B030: 4AF5B739  bl 0x821c6768
	ctx.lr = 0x8326B034;
	sub_821C6768(ctx, base);
	pc = 0x8326B034; continue 'dispatch;
            }
            0x8326B034 => {
    //   block [0x8326B034..0x8326B070)
	// 8326B034: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 8326B038: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8326B03C: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 8326B040: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 8326B044: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8326B048: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8326B04C: 4082FFE8  bne 0x8326b034
	if !ctx.cr[0].eq {
	pc = 0x8326B034; continue 'dispatch;
	}
	// 8326B050: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8326B054: 386BE508  addi r3, r11, -0x1af8
	ctx.r[3].s64 = ctx.r[11].s64 + -6904;
	// 8326B058: 4BA3EEC9  bl 0x82ca9f20
	ctx.lr = 0x8326B05C;
	sub_82CA9F20(ctx, base);
	// 8326B05C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8326B060: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B064: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B068: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8326B06C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B070 size=192
    let mut pc: u32 = 0x8326B070;
    'dispatch: loop {
        match pc {
            0x8326B070 => {
    //   block [0x8326B070..0x8326B0C8)
	// 8326B070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B078: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8326B07C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B080: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326B084: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326B088: 388BE3C0  addi r4, r11, -0x1c40
	ctx.r[4].s64 = ctx.r[11].s64 + -7232;
	// 8326B08C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8326B090: 4AFC1E41  bl 0x8222ced0
	ctx.lr = 0x8326B094;
	sub_8222CED0(ctx, base);
	// 8326B094: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8326B098: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8326B09C: 4AF83A9D  bl 0x821eeb38
	ctx.lr = 0x8326B0A0;
	sub_821EEB38(ctx, base);
	// 8326B0A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8326B0A4: 4B99874D  bl 0x82c037f0
	ctx.lr = 0x8326B0A8;
	sub_82C037F0(ctx, base);
	// 8326B0A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326B0AC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8326B0B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8326B0B4: 916AC230  stw r11, -0x3dd0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-15824 as u32), ctx.r[11].u32 ) };
	// 8326B0B8: 4AF5B6B1  bl 0x821c6768
	ctx.lr = 0x8326B0BC;
	sub_821C6768(ctx, base);
	// 8326B0BC: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 8326B0C0: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 8326B0C4: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	pc = 0x8326B0C8; continue 'dispatch;
            }
            0x8326B0C8 => {
    //   block [0x8326B0C8..0x8326B0F4)
	// 8326B0C8: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 8326B0CC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8326B0D0: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 8326B0D4: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 8326B0D8: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8326B0DC: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8326B0E0: 4082FFE8  bne 0x8326b0c8
	if !ctx.cr[0].eq {
	pc = 0x8326B0C8; continue 'dispatch;
	}
	// 8326B0E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8326B0E8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8326B0EC: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 8326B0F0: 4AF5B679  bl 0x821c6768
	ctx.lr = 0x8326B0F4;
	sub_821C6768(ctx, base);
	pc = 0x8326B0F4; continue 'dispatch;
            }
            0x8326B0F4 => {
    //   block [0x8326B0F4..0x8326B130)
	// 8326B0F4: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 8326B0F8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8326B0FC: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 8326B100: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 8326B104: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8326B108: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8326B10C: 4082FFE8  bne 0x8326b0f4
	if !ctx.cr[0].eq {
	pc = 0x8326B0F4; continue 'dispatch;
	}
	// 8326B110: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8326B114: 386BE510  addi r3, r11, -0x1af0
	ctx.r[3].s64 = ctx.r[11].s64 + -6896;
	// 8326B118: 4BA3EE09  bl 0x82ca9f20
	ctx.lr = 0x8326B11C;
	sub_82CA9F20(ctx, base);
	// 8326B11C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8326B120: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B124: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B128: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8326B12C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B130 size=192
    let mut pc: u32 = 0x8326B130;
    'dispatch: loop {
        match pc {
            0x8326B130 => {
    //   block [0x8326B130..0x8326B188)
	// 8326B130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B138: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8326B13C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B140: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326B144: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326B148: 388BE3F0  addi r4, r11, -0x1c10
	ctx.r[4].s64 = ctx.r[11].s64 + -7184;
	// 8326B14C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8326B150: 4AFC1D81  bl 0x8222ced0
	ctx.lr = 0x8326B154;
	sub_8222CED0(ctx, base);
	// 8326B154: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8326B158: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8326B15C: 4AF839DD  bl 0x821eeb38
	ctx.lr = 0x8326B160;
	sub_821EEB38(ctx, base);
	// 8326B160: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8326B164: 4B99868D  bl 0x82c037f0
	ctx.lr = 0x8326B168;
	sub_82C037F0(ctx, base);
	// 8326B168: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326B16C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8326B170: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8326B174: 916AC234  stw r11, -0x3dcc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-15820 as u32), ctx.r[11].u32 ) };
	// 8326B178: 4AF5B5F1  bl 0x821c6768
	ctx.lr = 0x8326B17C;
	sub_821C6768(ctx, base);
	// 8326B17C: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 8326B180: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 8326B184: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	pc = 0x8326B188; continue 'dispatch;
            }
            0x8326B188 => {
    //   block [0x8326B188..0x8326B1B4)
	// 8326B188: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 8326B18C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8326B190: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 8326B194: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 8326B198: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8326B19C: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8326B1A0: 4082FFE8  bne 0x8326b188
	if !ctx.cr[0].eq {
	pc = 0x8326B188; continue 'dispatch;
	}
	// 8326B1A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8326B1A8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8326B1AC: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 8326B1B0: 4AF5B5B9  bl 0x821c6768
	ctx.lr = 0x8326B1B4;
	sub_821C6768(ctx, base);
	pc = 0x8326B1B4; continue 'dispatch;
            }
            0x8326B1B4 => {
    //   block [0x8326B1B4..0x8326B1F0)
	// 8326B1B4: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 8326B1B8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8326B1BC: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 8326B1C0: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 8326B1C4: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8326B1C8: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8326B1CC: 4082FFE8  bne 0x8326b1b4
	if !ctx.cr[0].eq {
	pc = 0x8326B1B4; continue 'dispatch;
	}
	// 8326B1D0: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8326B1D4: 386BE518  addi r3, r11, -0x1ae8
	ctx.r[3].s64 = ctx.r[11].s64 + -6888;
	// 8326B1D8: 4BA3ED49  bl 0x82ca9f20
	ctx.lr = 0x8326B1DC;
	sub_82CA9F20(ctx, base);
	// 8326B1DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8326B1E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B1E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B1E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8326B1EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B1F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B1F0 size=192
    let mut pc: u32 = 0x8326B1F0;
    'dispatch: loop {
        match pc {
            0x8326B1F0 => {
    //   block [0x8326B1F0..0x8326B248)
	// 8326B1F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B1F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B1F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8326B1FC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B200: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326B204: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326B208: 388BE420  addi r4, r11, -0x1be0
	ctx.r[4].s64 = ctx.r[11].s64 + -7136;
	// 8326B20C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8326B210: 4AFC1CC1  bl 0x8222ced0
	ctx.lr = 0x8326B214;
	sub_8222CED0(ctx, base);
	// 8326B214: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8326B218: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8326B21C: 4AF8391D  bl 0x821eeb38
	ctx.lr = 0x8326B220;
	sub_821EEB38(ctx, base);
	// 8326B220: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8326B224: 4B9985CD  bl 0x82c037f0
	ctx.lr = 0x8326B228;
	sub_82C037F0(ctx, base);
	// 8326B228: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326B22C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8326B230: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8326B234: 916AC238  stw r11, -0x3dc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-15816 as u32), ctx.r[11].u32 ) };
	// 8326B238: 4AF5B531  bl 0x821c6768
	ctx.lr = 0x8326B23C;
	sub_821C6768(ctx, base);
	// 8326B23C: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 8326B240: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 8326B244: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	pc = 0x8326B248; continue 'dispatch;
            }
            0x8326B248 => {
    //   block [0x8326B248..0x8326B274)
	// 8326B248: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 8326B24C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8326B250: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 8326B254: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 8326B258: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8326B25C: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8326B260: 4082FFE8  bne 0x8326b248
	if !ctx.cr[0].eq {
	pc = 0x8326B248; continue 'dispatch;
	}
	// 8326B264: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8326B268: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8326B26C: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 8326B270: 4AF5B4F9  bl 0x821c6768
	ctx.lr = 0x8326B274;
	sub_821C6768(ctx, base);
	pc = 0x8326B274; continue 'dispatch;
            }
            0x8326B274 => {
    //   block [0x8326B274..0x8326B2B0)
	// 8326B274: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 8326B278: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8326B27C: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 8326B280: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 8326B284: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8326B288: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8326B28C: 4082FFE8  bne 0x8326b274
	if !ctx.cr[0].eq {
	pc = 0x8326B274; continue 'dispatch;
	}
	// 8326B290: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8326B294: 386BE520  addi r3, r11, -0x1ae0
	ctx.r[3].s64 = ctx.r[11].s64 + -6880;
	// 8326B298: 4BA3EC89  bl 0x82ca9f20
	ctx.lr = 0x8326B29C;
	sub_82CA9F20(ctx, base);
	// 8326B29C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8326B2A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B2A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B2A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8326B2AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B2B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B2B0 size=64
    let mut pc: u32 = 0x8326B2B0;
    'dispatch: loop {
        match pc {
            0x8326B2B0 => {
    //   block [0x8326B2B0..0x8326B2F0)
	// 8326B2B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B2B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B2B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B2BC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8326B2C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326B2C4: 388B37B0  addi r4, r11, 0x37b0
	ctx.r[4].s64 = ctx.r[11].s64 + 14256;
	// 8326B2C8: 386AC23C  addi r3, r10, -0x3dc4
	ctx.r[3].s64 = ctx.r[10].s64 + -15812;
	// 8326B2CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326B2D0: 4AFC1C01  bl 0x8222ced0
	ctx.lr = 0x8326B2D4;
	sub_8222CED0(ctx, base);
	// 8326B2D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326B2D8: 3869E548  addi r3, r9, -0x1ab8
	ctx.r[3].s64 = ctx.r[9].s64 + -6840;
	// 8326B2DC: 4BA3EC45  bl 0x82ca9f20
	ctx.lr = 0x8326B2E0;
	sub_82CA9F20(ctx, base);
	// 8326B2E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326B2E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B2E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B2EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B2F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B2F0 size=64
    let mut pc: u32 = 0x8326B2F0;
    'dispatch: loop {
        match pc {
            0x8326B2F0 => {
    //   block [0x8326B2F0..0x8326B330)
	// 8326B2F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B2F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B2F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B2FC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326B300: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326B304: 388BEB24  addi r4, r11, -0x14dc
	ctx.r[4].s64 = ctx.r[11].s64 + -5340;
	// 8326B308: 386AC240  addi r3, r10, -0x3dc0
	ctx.r[3].s64 = ctx.r[10].s64 + -15808;
	// 8326B30C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326B310: 4AFC1BC1  bl 0x8222ced0
	ctx.lr = 0x8326B314;
	sub_8222CED0(ctx, base);
	// 8326B314: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326B318: 3869E558  addi r3, r9, -0x1aa8
	ctx.r[3].s64 = ctx.r[9].s64 + -6824;
	// 8326B31C: 4BA3EC05  bl 0x82ca9f20
	ctx.lr = 0x8326B320;
	sub_82CA9F20(ctx, base);
	// 8326B320: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326B324: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B328: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B32C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B330 size=64
    let mut pc: u32 = 0x8326B330;
    'dispatch: loop {
        match pc {
            0x8326B330 => {
    //   block [0x8326B330..0x8326B370)
	// 8326B330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B338: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B33C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326B340: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326B344: 388BEB30  addi r4, r11, -0x14d0
	ctx.r[4].s64 = ctx.r[11].s64 + -5328;
	// 8326B348: 386AC244  addi r3, r10, -0x3dbc
	ctx.r[3].s64 = ctx.r[10].s64 + -15804;
	// 8326B34C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326B350: 4AFC1B81  bl 0x8222ced0
	ctx.lr = 0x8326B354;
	sub_8222CED0(ctx, base);
	// 8326B354: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326B358: 3869E568  addi r3, r9, -0x1a98
	ctx.r[3].s64 = ctx.r[9].s64 + -6808;
	// 8326B35C: 4BA3EBC5  bl 0x82ca9f20
	ctx.lr = 0x8326B360;
	sub_82CA9F20(ctx, base);
	// 8326B360: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326B364: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B368: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B36C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B370 size=64
    let mut pc: u32 = 0x8326B370;
    'dispatch: loop {
        match pc {
            0x8326B370 => {
    //   block [0x8326B370..0x8326B3B0)
	// 8326B370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B378: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B37C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326B380: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326B384: 388BEB40  addi r4, r11, -0x14c0
	ctx.r[4].s64 = ctx.r[11].s64 + -5312;
	// 8326B388: 386AC248  addi r3, r10, -0x3db8
	ctx.r[3].s64 = ctx.r[10].s64 + -15800;
	// 8326B38C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326B390: 4AFC1B41  bl 0x8222ced0
	ctx.lr = 0x8326B394;
	sub_8222CED0(ctx, base);
	// 8326B394: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326B398: 3869E578  addi r3, r9, -0x1a88
	ctx.r[3].s64 = ctx.r[9].s64 + -6792;
	// 8326B39C: 4BA3EB85  bl 0x82ca9f20
	ctx.lr = 0x8326B3A0;
	sub_82CA9F20(ctx, base);
	// 8326B3A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326B3A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B3A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B3AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B3B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B3B0 size=64
    let mut pc: u32 = 0x8326B3B0;
    'dispatch: loop {
        match pc {
            0x8326B3B0 => {
    //   block [0x8326B3B0..0x8326B3F0)
	// 8326B3B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B3B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B3B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B3BC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326B3C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326B3C4: 388BEB50  addi r4, r11, -0x14b0
	ctx.r[4].s64 = ctx.r[11].s64 + -5296;
	// 8326B3C8: 386AC24C  addi r3, r10, -0x3db4
	ctx.r[3].s64 = ctx.r[10].s64 + -15796;
	// 8326B3CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326B3D0: 4AFC1B01  bl 0x8222ced0
	ctx.lr = 0x8326B3D4;
	sub_8222CED0(ctx, base);
	// 8326B3D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326B3D8: 3869E588  addi r3, r9, -0x1a78
	ctx.r[3].s64 = ctx.r[9].s64 + -6776;
	// 8326B3DC: 4BA3EB45  bl 0x82ca9f20
	ctx.lr = 0x8326B3E0;
	sub_82CA9F20(ctx, base);
	// 8326B3E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326B3E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B3E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B3EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B3F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B3F0 size=64
    let mut pc: u32 = 0x8326B3F0;
    'dispatch: loop {
        match pc {
            0x8326B3F0 => {
    //   block [0x8326B3F0..0x8326B430)
	// 8326B3F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B3F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B3F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B3FC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8326B400: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326B404: 388B0840  addi r4, r11, 0x840
	ctx.r[4].s64 = ctx.r[11].s64 + 2112;
	// 8326B408: 386AC250  addi r3, r10, -0x3db0
	ctx.r[3].s64 = ctx.r[10].s64 + -15792;
	// 8326B40C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326B410: 4AFC1AC1  bl 0x8222ced0
	ctx.lr = 0x8326B414;
	sub_8222CED0(ctx, base);
	// 8326B414: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326B418: 3869E598  addi r3, r9, -0x1a68
	ctx.r[3].s64 = ctx.r[9].s64 + -6760;
	// 8326B41C: 4BA3EB05  bl 0x82ca9f20
	ctx.lr = 0x8326B420;
	sub_82CA9F20(ctx, base);
	// 8326B420: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326B424: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B428: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B42C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B430 size=64
    let mut pc: u32 = 0x8326B430;
    'dispatch: loop {
        match pc {
            0x8326B430 => {
    //   block [0x8326B430..0x8326B470)
	// 8326B430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B438: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B43C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8326B440: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326B444: 388B0754  addi r4, r11, 0x754
	ctx.r[4].s64 = ctx.r[11].s64 + 1876;
	// 8326B448: 386AC254  addi r3, r10, -0x3dac
	ctx.r[3].s64 = ctx.r[10].s64 + -15788;
	// 8326B44C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326B450: 4AFC1A81  bl 0x8222ced0
	ctx.lr = 0x8326B454;
	sub_8222CED0(ctx, base);
	// 8326B454: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326B458: 3869E5A8  addi r3, r9, -0x1a58
	ctx.r[3].s64 = ctx.r[9].s64 + -6744;
	// 8326B45C: 4BA3EAC5  bl 0x82ca9f20
	ctx.lr = 0x8326B460;
	sub_82CA9F20(ctx, base);
	// 8326B460: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326B464: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B468: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B46C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B470 size=64
    let mut pc: u32 = 0x8326B470;
    'dispatch: loop {
        match pc {
            0x8326B470 => {
    //   block [0x8326B470..0x8326B4B0)
	// 8326B470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B478: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B47C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8326B480: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326B484: 388B0768  addi r4, r11, 0x768
	ctx.r[4].s64 = ctx.r[11].s64 + 1896;
	// 8326B488: 386AC258  addi r3, r10, -0x3da8
	ctx.r[3].s64 = ctx.r[10].s64 + -15784;
	// 8326B48C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326B490: 4AFC1A41  bl 0x8222ced0
	ctx.lr = 0x8326B494;
	sub_8222CED0(ctx, base);
	// 8326B494: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326B498: 3869E5B8  addi r3, r9, -0x1a48
	ctx.r[3].s64 = ctx.r[9].s64 + -6728;
	// 8326B49C: 4BA3EA85  bl 0x82ca9f20
	ctx.lr = 0x8326B4A0;
	sub_82CA9F20(ctx, base);
	// 8326B4A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326B4A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B4A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B4AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B4B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B4B0 size=64
    let mut pc: u32 = 0x8326B4B0;
    'dispatch: loop {
        match pc {
            0x8326B4B0 => {
    //   block [0x8326B4B0..0x8326B4F0)
	// 8326B4B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B4B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B4B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B4BC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8326B4C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326B4C4: 388B37BC  addi r4, r11, 0x37bc
	ctx.r[4].s64 = ctx.r[11].s64 + 14268;
	// 8326B4C8: 386AC25C  addi r3, r10, -0x3da4
	ctx.r[3].s64 = ctx.r[10].s64 + -15780;
	// 8326B4CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326B4D0: 4AFC1A01  bl 0x8222ced0
	ctx.lr = 0x8326B4D4;
	sub_8222CED0(ctx, base);
	// 8326B4D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326B4D8: 3869E5C8  addi r3, r9, -0x1a38
	ctx.r[3].s64 = ctx.r[9].s64 + -6712;
	// 8326B4DC: 4BA3EA45  bl 0x82ca9f20
	ctx.lr = 0x8326B4E0;
	sub_82CA9F20(ctx, base);
	// 8326B4E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326B4E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B4E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B4EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B4F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B4F0 size=64
    let mut pc: u32 = 0x8326B4F0;
    'dispatch: loop {
        match pc {
            0x8326B4F0 => {
    //   block [0x8326B4F0..0x8326B530)
	// 8326B4F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B4F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B4F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B4FC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8326B500: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326B504: 388B13B0  addi r4, r11, 0x13b0
	ctx.r[4].s64 = ctx.r[11].s64 + 5040;
	// 8326B508: 386AC260  addi r3, r10, -0x3da0
	ctx.r[3].s64 = ctx.r[10].s64 + -15776;
	// 8326B50C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326B510: 4AFC19C1  bl 0x8222ced0
	ctx.lr = 0x8326B514;
	sub_8222CED0(ctx, base);
	// 8326B514: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326B518: 3869E5D8  addi r3, r9, -0x1a28
	ctx.r[3].s64 = ctx.r[9].s64 + -6696;
	// 8326B51C: 4BA3EA05  bl 0x82ca9f20
	ctx.lr = 0x8326B520;
	sub_82CA9F20(ctx, base);
	// 8326B520: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326B524: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B528: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B52C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B530 size=64
    let mut pc: u32 = 0x8326B530;
    'dispatch: loop {
        match pc {
            0x8326B530 => {
    //   block [0x8326B530..0x8326B570)
	// 8326B530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B538: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B53C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8326B540: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326B544: 388B13A0  addi r4, r11, 0x13a0
	ctx.r[4].s64 = ctx.r[11].s64 + 5024;
	// 8326B548: 386AC264  addi r3, r10, -0x3d9c
	ctx.r[3].s64 = ctx.r[10].s64 + -15772;
	// 8326B54C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326B550: 4AFC1981  bl 0x8222ced0
	ctx.lr = 0x8326B554;
	sub_8222CED0(ctx, base);
	// 8326B554: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326B558: 3869E5E8  addi r3, r9, -0x1a18
	ctx.r[3].s64 = ctx.r[9].s64 + -6680;
	// 8326B55C: 4BA3E9C5  bl 0x82ca9f20
	ctx.lr = 0x8326B560;
	sub_82CA9F20(ctx, base);
	// 8326B560: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326B564: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B568: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B56C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B570 size=64
    let mut pc: u32 = 0x8326B570;
    'dispatch: loop {
        match pc {
            0x8326B570 => {
    //   block [0x8326B570..0x8326B5B0)
	// 8326B570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B578: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B57C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326B580: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326B584: 388BEB60  addi r4, r11, -0x14a0
	ctx.r[4].s64 = ctx.r[11].s64 + -5280;
	// 8326B588: 386AC268  addi r3, r10, -0x3d98
	ctx.r[3].s64 = ctx.r[10].s64 + -15768;
	// 8326B58C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326B590: 4AFC1941  bl 0x8222ced0
	ctx.lr = 0x8326B594;
	sub_8222CED0(ctx, base);
	// 8326B594: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326B598: 3869E5F8  addi r3, r9, -0x1a08
	ctx.r[3].s64 = ctx.r[9].s64 + -6664;
	// 8326B59C: 4BA3E985  bl 0x82ca9f20
	ctx.lr = 0x8326B5A0;
	sub_82CA9F20(ctx, base);
	// 8326B5A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326B5A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B5A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B5AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B5B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B5B0 size=64
    let mut pc: u32 = 0x8326B5B0;
    'dispatch: loop {
        match pc {
            0x8326B5B0 => {
    //   block [0x8326B5B0..0x8326B5F0)
	// 8326B5B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B5B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B5B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B5BC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326B5C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326B5C4: 388BEB74  addi r4, r11, -0x148c
	ctx.r[4].s64 = ctx.r[11].s64 + -5260;
	// 8326B5C8: 386AC26C  addi r3, r10, -0x3d94
	ctx.r[3].s64 = ctx.r[10].s64 + -15764;
	// 8326B5CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326B5D0: 4AFC1901  bl 0x8222ced0
	ctx.lr = 0x8326B5D4;
	sub_8222CED0(ctx, base);
	// 8326B5D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326B5D8: 3869E608  addi r3, r9, -0x19f8
	ctx.r[3].s64 = ctx.r[9].s64 + -6648;
	// 8326B5DC: 4BA3E945  bl 0x82ca9f20
	ctx.lr = 0x8326B5E0;
	sub_82CA9F20(ctx, base);
	// 8326B5E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326B5E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B5E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B5EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B5F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B5F0 size=64
    let mut pc: u32 = 0x8326B5F0;
    'dispatch: loop {
        match pc {
            0x8326B5F0 => {
    //   block [0x8326B5F0..0x8326B630)
	// 8326B5F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B5F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B5F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B5FC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326B600: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326B604: 388BEB88  addi r4, r11, -0x1478
	ctx.r[4].s64 = ctx.r[11].s64 + -5240;
	// 8326B608: 386AC270  addi r3, r10, -0x3d90
	ctx.r[3].s64 = ctx.r[10].s64 + -15760;
	// 8326B60C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326B610: 4AFC18C1  bl 0x8222ced0
	ctx.lr = 0x8326B614;
	sub_8222CED0(ctx, base);
	// 8326B614: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326B618: 3869E618  addi r3, r9, -0x19e8
	ctx.r[3].s64 = ctx.r[9].s64 + -6632;
	// 8326B61C: 4BA3E905  bl 0x82ca9f20
	ctx.lr = 0x8326B620;
	sub_82CA9F20(ctx, base);
	// 8326B620: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326B624: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B628: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B62C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B630 size=64
    let mut pc: u32 = 0x8326B630;
    'dispatch: loop {
        match pc {
            0x8326B630 => {
    //   block [0x8326B630..0x8326B670)
	// 8326B630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B638: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B63C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326B640: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326B644: 388BEB9C  addi r4, r11, -0x1464
	ctx.r[4].s64 = ctx.r[11].s64 + -5220;
	// 8326B648: 386AC274  addi r3, r10, -0x3d8c
	ctx.r[3].s64 = ctx.r[10].s64 + -15756;
	// 8326B64C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326B650: 4AFC1881  bl 0x8222ced0
	ctx.lr = 0x8326B654;
	sub_8222CED0(ctx, base);
	// 8326B654: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326B658: 3869E628  addi r3, r9, -0x19d8
	ctx.r[3].s64 = ctx.r[9].s64 + -6616;
	// 8326B65C: 4BA3E8C5  bl 0x82ca9f20
	ctx.lr = 0x8326B660;
	sub_82CA9F20(ctx, base);
	// 8326B660: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326B664: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B668: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B66C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B670 size=64
    let mut pc: u32 = 0x8326B670;
    'dispatch: loop {
        match pc {
            0x8326B670 => {
    //   block [0x8326B670..0x8326B6B0)
	// 8326B670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B678: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B67C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326B680: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326B684: 388BEBB0  addi r4, r11, -0x1450
	ctx.r[4].s64 = ctx.r[11].s64 + -5200;
	// 8326B688: 386AC278  addi r3, r10, -0x3d88
	ctx.r[3].s64 = ctx.r[10].s64 + -15752;
	// 8326B68C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326B690: 4AFC1841  bl 0x8222ced0
	ctx.lr = 0x8326B694;
	sub_8222CED0(ctx, base);
	// 8326B694: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326B698: 3869E638  addi r3, r9, -0x19c8
	ctx.r[3].s64 = ctx.r[9].s64 + -6600;
	// 8326B69C: 4BA3E885  bl 0x82ca9f20
	ctx.lr = 0x8326B6A0;
	sub_82CA9F20(ctx, base);
	// 8326B6A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326B6A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B6A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B6AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B6B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B6B0 size=64
    let mut pc: u32 = 0x8326B6B0;
    'dispatch: loop {
        match pc {
            0x8326B6B0 => {
    //   block [0x8326B6B0..0x8326B6F0)
	// 8326B6B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B6B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B6B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B6BC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326B6C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326B6C4: 388BEBC4  addi r4, r11, -0x143c
	ctx.r[4].s64 = ctx.r[11].s64 + -5180;
	// 8326B6C8: 386AC27C  addi r3, r10, -0x3d84
	ctx.r[3].s64 = ctx.r[10].s64 + -15748;
	// 8326B6CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326B6D0: 4AFC1801  bl 0x8222ced0
	ctx.lr = 0x8326B6D4;
	sub_8222CED0(ctx, base);
	// 8326B6D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326B6D8: 3869E648  addi r3, r9, -0x19b8
	ctx.r[3].s64 = ctx.r[9].s64 + -6584;
	// 8326B6DC: 4BA3E845  bl 0x82ca9f20
	ctx.lr = 0x8326B6E0;
	sub_82CA9F20(ctx, base);
	// 8326B6E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326B6E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B6E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B6EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B6F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B6F0 size=64
    let mut pc: u32 = 0x8326B6F0;
    'dispatch: loop {
        match pc {
            0x8326B6F0 => {
    //   block [0x8326B6F0..0x8326B730)
	// 8326B6F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B6F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B6F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B6FC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326B700: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326B704: 388BEBD4  addi r4, r11, -0x142c
	ctx.r[4].s64 = ctx.r[11].s64 + -5164;
	// 8326B708: 386AC280  addi r3, r10, -0x3d80
	ctx.r[3].s64 = ctx.r[10].s64 + -15744;
	// 8326B70C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326B710: 4AFC17C1  bl 0x8222ced0
	ctx.lr = 0x8326B714;
	sub_8222CED0(ctx, base);
	// 8326B714: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326B718: 3869E658  addi r3, r9, -0x19a8
	ctx.r[3].s64 = ctx.r[9].s64 + -6568;
	// 8326B71C: 4BA3E805  bl 0x82ca9f20
	ctx.lr = 0x8326B720;
	sub_82CA9F20(ctx, base);
	// 8326B720: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326B724: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B728: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B72C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B730 size=64
    let mut pc: u32 = 0x8326B730;
    'dispatch: loop {
        match pc {
            0x8326B730 => {
    //   block [0x8326B730..0x8326B770)
	// 8326B730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B738: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B73C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326B740: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326B744: 388BEBE4  addi r4, r11, -0x141c
	ctx.r[4].s64 = ctx.r[11].s64 + -5148;
	// 8326B748: 386AC284  addi r3, r10, -0x3d7c
	ctx.r[3].s64 = ctx.r[10].s64 + -15740;
	// 8326B74C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326B750: 4AFC1781  bl 0x8222ced0
	ctx.lr = 0x8326B754;
	sub_8222CED0(ctx, base);
	// 8326B754: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326B758: 3869E668  addi r3, r9, -0x1998
	ctx.r[3].s64 = ctx.r[9].s64 + -6552;
	// 8326B75C: 4BA3E7C5  bl 0x82ca9f20
	ctx.lr = 0x8326B760;
	sub_82CA9F20(ctx, base);
	// 8326B760: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326B764: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B768: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B76C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B770 size=64
    let mut pc: u32 = 0x8326B770;
    'dispatch: loop {
        match pc {
            0x8326B770 => {
    //   block [0x8326B770..0x8326B7B0)
	// 8326B770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B778: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B77C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326B780: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326B784: 388BEBF4  addi r4, r11, -0x140c
	ctx.r[4].s64 = ctx.r[11].s64 + -5132;
	// 8326B788: 386AC288  addi r3, r10, -0x3d78
	ctx.r[3].s64 = ctx.r[10].s64 + -15736;
	// 8326B78C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326B790: 4AFC1741  bl 0x8222ced0
	ctx.lr = 0x8326B794;
	sub_8222CED0(ctx, base);
	// 8326B794: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326B798: 3869E678  addi r3, r9, -0x1988
	ctx.r[3].s64 = ctx.r[9].s64 + -6536;
	// 8326B79C: 4BA3E785  bl 0x82ca9f20
	ctx.lr = 0x8326B7A0;
	sub_82CA9F20(ctx, base);
	// 8326B7A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326B7A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B7A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B7AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B7B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B7B0 size=64
    let mut pc: u32 = 0x8326B7B0;
    'dispatch: loop {
        match pc {
            0x8326B7B0 => {
    //   block [0x8326B7B0..0x8326B7F0)
	// 8326B7B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B7B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B7B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B7BC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326B7C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326B7C4: 388BEC04  addi r4, r11, -0x13fc
	ctx.r[4].s64 = ctx.r[11].s64 + -5116;
	// 8326B7C8: 386AC28C  addi r3, r10, -0x3d74
	ctx.r[3].s64 = ctx.r[10].s64 + -15732;
	// 8326B7CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326B7D0: 4AFC1701  bl 0x8222ced0
	ctx.lr = 0x8326B7D4;
	sub_8222CED0(ctx, base);
	// 8326B7D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326B7D8: 3869E688  addi r3, r9, -0x1978
	ctx.r[3].s64 = ctx.r[9].s64 + -6520;
	// 8326B7DC: 4BA3E745  bl 0x82ca9f20
	ctx.lr = 0x8326B7E0;
	sub_82CA9F20(ctx, base);
	// 8326B7E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326B7E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B7E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B7EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B7F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B7F0 size=64
    let mut pc: u32 = 0x8326B7F0;
    'dispatch: loop {
        match pc {
            0x8326B7F0 => {
    //   block [0x8326B7F0..0x8326B830)
	// 8326B7F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B7F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B7F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B7FC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326B800: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326B804: 388BEC18  addi r4, r11, -0x13e8
	ctx.r[4].s64 = ctx.r[11].s64 + -5096;
	// 8326B808: 386AC290  addi r3, r10, -0x3d70
	ctx.r[3].s64 = ctx.r[10].s64 + -15728;
	// 8326B80C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326B810: 4AFC16C1  bl 0x8222ced0
	ctx.lr = 0x8326B814;
	sub_8222CED0(ctx, base);
	// 8326B814: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326B818: 3869E698  addi r3, r9, -0x1968
	ctx.r[3].s64 = ctx.r[9].s64 + -6504;
	// 8326B81C: 4BA3E705  bl 0x82ca9f20
	ctx.lr = 0x8326B820;
	sub_82CA9F20(ctx, base);
	// 8326B820: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326B824: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B828: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B82C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B830 size=64
    let mut pc: u32 = 0x8326B830;
    'dispatch: loop {
        match pc {
            0x8326B830 => {
    //   block [0x8326B830..0x8326B870)
	// 8326B830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B838: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B83C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326B840: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326B844: 388BEC2C  addi r4, r11, -0x13d4
	ctx.r[4].s64 = ctx.r[11].s64 + -5076;
	// 8326B848: 386AC294  addi r3, r10, -0x3d6c
	ctx.r[3].s64 = ctx.r[10].s64 + -15724;
	// 8326B84C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326B850: 4AFC1681  bl 0x8222ced0
	ctx.lr = 0x8326B854;
	sub_8222CED0(ctx, base);
	// 8326B854: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326B858: 3869E6A8  addi r3, r9, -0x1958
	ctx.r[3].s64 = ctx.r[9].s64 + -6488;
	// 8326B85C: 4BA3E6C5  bl 0x82ca9f20
	ctx.lr = 0x8326B860;
	sub_82CA9F20(ctx, base);
	// 8326B860: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326B864: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B868: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B86C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B870 size=64
    let mut pc: u32 = 0x8326B870;
    'dispatch: loop {
        match pc {
            0x8326B870 => {
    //   block [0x8326B870..0x8326B8B0)
	// 8326B870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B878: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B87C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326B880: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326B884: 388BEC40  addi r4, r11, -0x13c0
	ctx.r[4].s64 = ctx.r[11].s64 + -5056;
	// 8326B888: 386AC298  addi r3, r10, -0x3d68
	ctx.r[3].s64 = ctx.r[10].s64 + -15720;
	// 8326B88C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326B890: 4AFC1641  bl 0x8222ced0
	ctx.lr = 0x8326B894;
	sub_8222CED0(ctx, base);
	// 8326B894: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326B898: 3869E6B8  addi r3, r9, -0x1948
	ctx.r[3].s64 = ctx.r[9].s64 + -6472;
	// 8326B89C: 4BA3E685  bl 0x82ca9f20
	ctx.lr = 0x8326B8A0;
	sub_82CA9F20(ctx, base);
	// 8326B8A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326B8A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B8A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B8AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B8B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B8B0 size=64
    let mut pc: u32 = 0x8326B8B0;
    'dispatch: loop {
        match pc {
            0x8326B8B0 => {
    //   block [0x8326B8B0..0x8326B8F0)
	// 8326B8B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B8B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B8B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B8BC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326B8C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326B8C4: 388BEC54  addi r4, r11, -0x13ac
	ctx.r[4].s64 = ctx.r[11].s64 + -5036;
	// 8326B8C8: 386AC29C  addi r3, r10, -0x3d64
	ctx.r[3].s64 = ctx.r[10].s64 + -15716;
	// 8326B8CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326B8D0: 4AFC1601  bl 0x8222ced0
	ctx.lr = 0x8326B8D4;
	sub_8222CED0(ctx, base);
	// 8326B8D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326B8D8: 3869E6C8  addi r3, r9, -0x1938
	ctx.r[3].s64 = ctx.r[9].s64 + -6456;
	// 8326B8DC: 4BA3E645  bl 0x82ca9f20
	ctx.lr = 0x8326B8E0;
	sub_82CA9F20(ctx, base);
	// 8326B8E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326B8E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B8E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B8EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B8F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B8F0 size=64
    let mut pc: u32 = 0x8326B8F0;
    'dispatch: loop {
        match pc {
            0x8326B8F0 => {
    //   block [0x8326B8F0..0x8326B930)
	// 8326B8F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B8F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B8F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B8FC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326B900: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326B904: 388BEC68  addi r4, r11, -0x1398
	ctx.r[4].s64 = ctx.r[11].s64 + -5016;
	// 8326B908: 386AC2A0  addi r3, r10, -0x3d60
	ctx.r[3].s64 = ctx.r[10].s64 + -15712;
	// 8326B90C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326B910: 4AFC15C1  bl 0x8222ced0
	ctx.lr = 0x8326B914;
	sub_8222CED0(ctx, base);
	// 8326B914: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326B918: 3869E6D8  addi r3, r9, -0x1928
	ctx.r[3].s64 = ctx.r[9].s64 + -6440;
	// 8326B91C: 4BA3E605  bl 0x82ca9f20
	ctx.lr = 0x8326B920;
	sub_82CA9F20(ctx, base);
	// 8326B920: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326B924: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B928: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B92C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B930 size=64
    let mut pc: u32 = 0x8326B930;
    'dispatch: loop {
        match pc {
            0x8326B930 => {
    //   block [0x8326B930..0x8326B970)
	// 8326B930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B938: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B93C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326B940: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326B944: 388BEC7C  addi r4, r11, -0x1384
	ctx.r[4].s64 = ctx.r[11].s64 + -4996;
	// 8326B948: 386AC2A4  addi r3, r10, -0x3d5c
	ctx.r[3].s64 = ctx.r[10].s64 + -15708;
	// 8326B94C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326B950: 4AFC1581  bl 0x8222ced0
	ctx.lr = 0x8326B954;
	sub_8222CED0(ctx, base);
	// 8326B954: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326B958: 3869E6E8  addi r3, r9, -0x1918
	ctx.r[3].s64 = ctx.r[9].s64 + -6424;
	// 8326B95C: 4BA3E5C5  bl 0x82ca9f20
	ctx.lr = 0x8326B960;
	sub_82CA9F20(ctx, base);
	// 8326B960: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326B964: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B968: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B96C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B970 size=64
    let mut pc: u32 = 0x8326B970;
    'dispatch: loop {
        match pc {
            0x8326B970 => {
    //   block [0x8326B970..0x8326B9B0)
	// 8326B970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B978: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B97C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326B980: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326B984: 388BEC8C  addi r4, r11, -0x1374
	ctx.r[4].s64 = ctx.r[11].s64 + -4980;
	// 8326B988: 386AC2A8  addi r3, r10, -0x3d58
	ctx.r[3].s64 = ctx.r[10].s64 + -15704;
	// 8326B98C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326B990: 4AFC1541  bl 0x8222ced0
	ctx.lr = 0x8326B994;
	sub_8222CED0(ctx, base);
	// 8326B994: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326B998: 3869E6F8  addi r3, r9, -0x1908
	ctx.r[3].s64 = ctx.r[9].s64 + -6408;
	// 8326B99C: 4BA3E585  bl 0x82ca9f20
	ctx.lr = 0x8326B9A0;
	sub_82CA9F20(ctx, base);
	// 8326B9A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326B9A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B9A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B9AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B9B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B9B0 size=64
    let mut pc: u32 = 0x8326B9B0;
    'dispatch: loop {
        match pc {
            0x8326B9B0 => {
    //   block [0x8326B9B0..0x8326B9F0)
	// 8326B9B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B9B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B9B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B9BC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326B9C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326B9C4: 388BEC9C  addi r4, r11, -0x1364
	ctx.r[4].s64 = ctx.r[11].s64 + -4964;
	// 8326B9C8: 386AC2AC  addi r3, r10, -0x3d54
	ctx.r[3].s64 = ctx.r[10].s64 + -15700;
	// 8326B9CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326B9D0: 4AFC1501  bl 0x8222ced0
	ctx.lr = 0x8326B9D4;
	sub_8222CED0(ctx, base);
	// 8326B9D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326B9D8: 3869E708  addi r3, r9, -0x18f8
	ctx.r[3].s64 = ctx.r[9].s64 + -6392;
	// 8326B9DC: 4BA3E545  bl 0x82ca9f20
	ctx.lr = 0x8326B9E0;
	sub_82CA9F20(ctx, base);
	// 8326B9E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326B9E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B9E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B9EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B9F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B9F0 size=64
    let mut pc: u32 = 0x8326B9F0;
    'dispatch: loop {
        match pc {
            0x8326B9F0 => {
    //   block [0x8326B9F0..0x8326BA30)
	// 8326B9F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B9F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B9F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B9FC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326BA00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326BA04: 388BECAC  addi r4, r11, -0x1354
	ctx.r[4].s64 = ctx.r[11].s64 + -4948;
	// 8326BA08: 386AC2B0  addi r3, r10, -0x3d50
	ctx.r[3].s64 = ctx.r[10].s64 + -15696;
	// 8326BA0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326BA10: 4AFC14C1  bl 0x8222ced0
	ctx.lr = 0x8326BA14;
	sub_8222CED0(ctx, base);
	// 8326BA14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326BA18: 3869E718  addi r3, r9, -0x18e8
	ctx.r[3].s64 = ctx.r[9].s64 + -6376;
	// 8326BA1C: 4BA3E505  bl 0x82ca9f20
	ctx.lr = 0x8326BA20;
	sub_82CA9F20(ctx, base);
	// 8326BA20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326BA24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326BA28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326BA2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326BA30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326BA30 size=64
    let mut pc: u32 = 0x8326BA30;
    'dispatch: loop {
        match pc {
            0x8326BA30 => {
    //   block [0x8326BA30..0x8326BA70)
	// 8326BA30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326BA34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326BA38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326BA3C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326BA40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326BA44: 388BECBC  addi r4, r11, -0x1344
	ctx.r[4].s64 = ctx.r[11].s64 + -4932;
	// 8326BA48: 386AC2B4  addi r3, r10, -0x3d4c
	ctx.r[3].s64 = ctx.r[10].s64 + -15692;
	// 8326BA4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326BA50: 4AFC1481  bl 0x8222ced0
	ctx.lr = 0x8326BA54;
	sub_8222CED0(ctx, base);
	// 8326BA54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326BA58: 3869E728  addi r3, r9, -0x18d8
	ctx.r[3].s64 = ctx.r[9].s64 + -6360;
	// 8326BA5C: 4BA3E4C5  bl 0x82ca9f20
	ctx.lr = 0x8326BA60;
	sub_82CA9F20(ctx, base);
	// 8326BA60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326BA64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326BA68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326BA6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326BA70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8326BA70 size=12
    let mut pc: u32 = 0x8326BA70;
    'dispatch: loop {
        match pc {
            0x8326BA70 => {
    //   block [0x8326BA70..0x8326BA7C)
	// 8326BA70: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8326BA74: 386BE738  addi r3, r11, -0x18c8
	ctx.r[3].s64 = ctx.r[11].s64 + -6344;
	// 8326BA78: 4BA3E4A8  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326BA80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326BA80 size=64
    let mut pc: u32 = 0x8326BA80;
    'dispatch: loop {
        match pc {
            0x8326BA80 => {
    //   block [0x8326BA80..0x8326BAC0)
	// 8326BA80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326BA84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326BA88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326BA8C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326BA90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326BA94: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8326BA98: 386AC2C8  addi r3, r10, -0x3d38
	ctx.r[3].s64 = ctx.r[10].s64 + -15672;
	// 8326BA9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326BAA0: 4AFC1431  bl 0x8222ced0
	ctx.lr = 0x8326BAA4;
	sub_8222CED0(ctx, base);
	// 8326BAA4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326BAA8: 3869E7B8  addi r3, r9, -0x1848
	ctx.r[3].s64 = ctx.r[9].s64 + -6216;
	// 8326BAAC: 4BA3E475  bl 0x82ca9f20
	ctx.lr = 0x8326BAB0;
	sub_82CA9F20(ctx, base);
	// 8326BAB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326BAB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326BAB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326BABC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326BAC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326BAC0 size=64
    let mut pc: u32 = 0x8326BAC0;
    'dispatch: loop {
        match pc {
            0x8326BAC0 => {
    //   block [0x8326BAC0..0x8326BB00)
	// 8326BAC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326BAC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326BAC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326BACC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326BAD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326BAD4: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8326BAD8: 386AC2CC  addi r3, r10, -0x3d34
	ctx.r[3].s64 = ctx.r[10].s64 + -15668;
	// 8326BADC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326BAE0: 4AFC13F1  bl 0x8222ced0
	ctx.lr = 0x8326BAE4;
	sub_8222CED0(ctx, base);
	// 8326BAE4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326BAE8: 3869E7C8  addi r3, r9, -0x1838
	ctx.r[3].s64 = ctx.r[9].s64 + -6200;
	// 8326BAEC: 4BA3E435  bl 0x82ca9f20
	ctx.lr = 0x8326BAF0;
	sub_82CA9F20(ctx, base);
	// 8326BAF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326BAF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326BAF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326BAFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326BB00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326BB00 size=64
    let mut pc: u32 = 0x8326BB00;
    'dispatch: loop {
        match pc {
            0x8326BB00 => {
    //   block [0x8326BB00..0x8326BB40)
	// 8326BB00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326BB04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326BB08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326BB0C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326BB10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326BB14: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8326BB18: 386AC2D0  addi r3, r10, -0x3d30
	ctx.r[3].s64 = ctx.r[10].s64 + -15664;
	// 8326BB1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326BB20: 4AFC13B1  bl 0x8222ced0
	ctx.lr = 0x8326BB24;
	sub_8222CED0(ctx, base);
	// 8326BB24: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326BB28: 3869E7D8  addi r3, r9, -0x1828
	ctx.r[3].s64 = ctx.r[9].s64 + -6184;
	// 8326BB2C: 4BA3E3F5  bl 0x82ca9f20
	ctx.lr = 0x8326BB30;
	sub_82CA9F20(ctx, base);
	// 8326BB30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326BB34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326BB38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326BB3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326BB40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326BB40 size=56
    let mut pc: u32 = 0x8326BB40;
    'dispatch: loop {
        match pc {
            0x8326BB40 => {
    //   block [0x8326BB40..0x8326BB78)
	// 8326BB40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326BB44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326BB48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326BB4C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8326BB50: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326BB54: 386B0CA0  addi r3, r11, 0xca0
	ctx.r[3].s64 = ctx.r[11].s64 + 3232;
	// 8326BB58: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326BB5C: 4AF881FD  bl 0x821f3d58
	ctx.lr = 0x8326BB60;
	sub_821F3D58(ctx, base);
	// 8326BB60: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326BB64: 906AC2D4  stw r3, -0x3d2c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-15660 as u32), ctx.r[3].u32 ) };
	// 8326BB68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326BB6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326BB70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326BB74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326BB78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8326BB78 size=44
    let mut pc: u32 = 0x8326BB78;
    'dispatch: loop {
        match pc {
            0x8326BB78 => {
    //   block [0x8326BB78..0x8326BBA4)
	// 8326BB78: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 8326BB7C: 3D408210  lis r10, -0x7df0
	ctx.r[10].s64 = -2112880640;
	// 8326BB80: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8326BB84: C80B9660  lfd f0, -0x69a0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-27040 as u32) ) };
	// 8326BB88: C9AA1A28  lfd f13, 0x1a28(r10)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(6696 as u32) ) };
	// 8326BB8C: FC000372  fmul f0, f0, f13
	ctx.f[0].f64 = ctx.f[0].f64 * ctx.f[13].f64;
	// 8326BB90: FDA0001E  fctiwz f13, f0
	ctx.f[13].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 8326BB94: D9A1FFF0  stfd f13, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[13].u64 ) };
	// 8326BB98: 8161FFF4  lwz r11, -0xc(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 8326BB9C: 9169C2D8  stw r11, -0x3d28(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-15656 as u32), ctx.r[11].u32 ) };
	// 8326BBA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326BBA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326BBA8 size=64
    let mut pc: u32 = 0x8326BBA8;
    'dispatch: loop {
        match pc {
            0x8326BBA8 => {
    //   block [0x8326BBA8..0x8326BBE8)
	// 8326BBA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326BBAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326BBB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326BBB4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326BBB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326BBBC: 388BF18C  addi r4, r11, -0xe74
	ctx.r[4].s64 = ctx.r[11].s64 + -3700;
	// 8326BBC0: 386AC2DC  addi r3, r10, -0x3d24
	ctx.r[3].s64 = ctx.r[10].s64 + -15652;
	// 8326BBC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326BBC8: 4AFC1309  bl 0x8222ced0
	ctx.lr = 0x8326BBCC;
	sub_8222CED0(ctx, base);
	// 8326BBCC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326BBD0: 3869E850  addi r3, r9, -0x17b0
	ctx.r[3].s64 = ctx.r[9].s64 + -6064;
	// 8326BBD4: 4BA3E34D  bl 0x82ca9f20
	ctx.lr = 0x8326BBD8;
	sub_82CA9F20(ctx, base);
	// 8326BBD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326BBDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326BBE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326BBE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326BBE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326BBE8 size=64
    let mut pc: u32 = 0x8326BBE8;
    'dispatch: loop {
        match pc {
            0x8326BBE8 => {
    //   block [0x8326BBE8..0x8326BC28)
	// 8326BBE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326BBEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326BBF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326BBF4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326BBF8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326BBFC: 388BF1A8  addi r4, r11, -0xe58
	ctx.r[4].s64 = ctx.r[11].s64 + -3672;
	// 8326BC00: 386AC2E0  addi r3, r10, -0x3d20
	ctx.r[3].s64 = ctx.r[10].s64 + -15648;
	// 8326BC04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326BC08: 4AFC12C9  bl 0x8222ced0
	ctx.lr = 0x8326BC0C;
	sub_8222CED0(ctx, base);
	// 8326BC0C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326BC10: 3869E860  addi r3, r9, -0x17a0
	ctx.r[3].s64 = ctx.r[9].s64 + -6048;
	// 8326BC14: 4BA3E30D  bl 0x82ca9f20
	ctx.lr = 0x8326BC18;
	sub_82CA9F20(ctx, base);
	// 8326BC18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326BC1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326BC20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326BC24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326BC28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326BC28 size=64
    let mut pc: u32 = 0x8326BC28;
    'dispatch: loop {
        match pc {
            0x8326BC28 => {
    //   block [0x8326BC28..0x8326BC68)
	// 8326BC28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326BC2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326BC30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326BC34: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326BC38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326BC3C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8326BC40: 386AC310  addi r3, r10, -0x3cf0
	ctx.r[3].s64 = ctx.r[10].s64 + -15600;
	// 8326BC44: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326BC48: 4AFC1289  bl 0x8222ced0
	ctx.lr = 0x8326BC4C;
	sub_8222CED0(ctx, base);
	// 8326BC4C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326BC50: 3869E870  addi r3, r9, -0x1790
	ctx.r[3].s64 = ctx.r[9].s64 + -6032;
	// 8326BC54: 4BA3E2CD  bl 0x82ca9f20
	ctx.lr = 0x8326BC58;
	sub_82CA9F20(ctx, base);
	// 8326BC58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326BC5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326BC60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326BC64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326BC68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326BC68 size=64
    let mut pc: u32 = 0x8326BC68;
    'dispatch: loop {
        match pc {
            0x8326BC68 => {
    //   block [0x8326BC68..0x8326BCA8)
	// 8326BC68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326BC6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326BC70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326BC74: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326BC78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326BC7C: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8326BC80: 386AC314  addi r3, r10, -0x3cec
	ctx.r[3].s64 = ctx.r[10].s64 + -15596;
	// 8326BC84: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326BC88: 4AFC1249  bl 0x8222ced0
	ctx.lr = 0x8326BC8C;
	sub_8222CED0(ctx, base);
	// 8326BC8C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326BC90: 3869E880  addi r3, r9, -0x1780
	ctx.r[3].s64 = ctx.r[9].s64 + -6016;
	// 8326BC94: 4BA3E28D  bl 0x82ca9f20
	ctx.lr = 0x8326BC98;
	sub_82CA9F20(ctx, base);
	// 8326BC98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326BC9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326BCA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326BCA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326BCA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326BCA8 size=64
    let mut pc: u32 = 0x8326BCA8;
    'dispatch: loop {
        match pc {
            0x8326BCA8 => {
    //   block [0x8326BCA8..0x8326BCE8)
	// 8326BCA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326BCAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326BCB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326BCB4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326BCB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326BCBC: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8326BCC0: 386AC318  addi r3, r10, -0x3ce8
	ctx.r[3].s64 = ctx.r[10].s64 + -15592;
	// 8326BCC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326BCC8: 4AFC1209  bl 0x8222ced0
	ctx.lr = 0x8326BCCC;
	sub_8222CED0(ctx, base);
	// 8326BCCC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326BCD0: 3869E890  addi r3, r9, -0x1770
	ctx.r[3].s64 = ctx.r[9].s64 + -6000;
	// 8326BCD4: 4BA3E24D  bl 0x82ca9f20
	ctx.lr = 0x8326BCD8;
	sub_82CA9F20(ctx, base);
	// 8326BCD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326BCDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326BCE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326BCE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326BCE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326BCE8 size=64
    let mut pc: u32 = 0x8326BCE8;
    'dispatch: loop {
        match pc {
            0x8326BCE8 => {
    //   block [0x8326BCE8..0x8326BD28)
	// 8326BCE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326BCEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326BCF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326BCF4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8326BCF8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326BCFC: 388B0E20  addi r4, r11, 0xe20
	ctx.r[4].s64 = ctx.r[11].s64 + 3616;
	// 8326BD00: 386AC330  addi r3, r10, -0x3cd0
	ctx.r[3].s64 = ctx.r[10].s64 + -15568;
	// 8326BD04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326BD08: 4AFC11C9  bl 0x8222ced0
	ctx.lr = 0x8326BD0C;
	sub_8222CED0(ctx, base);
	// 8326BD0C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326BD10: 3869E8A0  addi r3, r9, -0x1760
	ctx.r[3].s64 = ctx.r[9].s64 + -5984;
	// 8326BD14: 4BA3E20D  bl 0x82ca9f20
	ctx.lr = 0x8326BD18;
	sub_82CA9F20(ctx, base);
	// 8326BD18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326BD1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326BD20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326BD24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326BD28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326BD28 size=64
    let mut pc: u32 = 0x8326BD28;
    'dispatch: loop {
        match pc {
            0x8326BD28 => {
    //   block [0x8326BD28..0x8326BD68)
	// 8326BD28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326BD2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326BD30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326BD34: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8326BD38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326BD3C: 388B67A8  addi r4, r11, 0x67a8
	ctx.r[4].s64 = ctx.r[11].s64 + 26536;
	// 8326BD40: 386AC334  addi r3, r10, -0x3ccc
	ctx.r[3].s64 = ctx.r[10].s64 + -15564;
	// 8326BD44: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326BD48: 4AFC1189  bl 0x8222ced0
	ctx.lr = 0x8326BD4C;
	sub_8222CED0(ctx, base);
	// 8326BD4C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326BD50: 3869E8B0  addi r3, r9, -0x1750
	ctx.r[3].s64 = ctx.r[9].s64 + -5968;
	// 8326BD54: 4BA3E1CD  bl 0x82ca9f20
	ctx.lr = 0x8326BD58;
	sub_82CA9F20(ctx, base);
	// 8326BD58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326BD5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326BD60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326BD64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326BD68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8326BD68 size=44
    let mut pc: u32 = 0x8326BD68;
    'dispatch: loop {
        match pc {
            0x8326BD68 => {
    //   block [0x8326BD68..0x8326BD94)
	// 8326BD68: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 8326BD6C: 3D408210  lis r10, -0x7df0
	ctx.r[10].s64 = -2112880640;
	// 8326BD70: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8326BD74: C80B9660  lfd f0, -0x69a0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-27040 as u32) ) };
	// 8326BD78: C9AA1A28  lfd f13, 0x1a28(r10)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(6696 as u32) ) };
	// 8326BD7C: FC000372  fmul f0, f0, f13
	ctx.f[0].f64 = ctx.f[0].f64 * ctx.f[13].f64;
	// 8326BD80: FDA0001E  fctiwz f13, f0
	ctx.f[13].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 8326BD84: D9A1FFF0  stfd f13, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[13].u64 ) };
	// 8326BD88: 8161FFF4  lwz r11, -0xc(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 8326BD8C: 9169C338  stw r11, -0x3cc8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-15560 as u32), ctx.r[11].u32 ) };
	// 8326BD90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326BD98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326BD98 size=64
    let mut pc: u32 = 0x8326BD98;
    'dispatch: loop {
        match pc {
            0x8326BD98 => {
    //   block [0x8326BD98..0x8326BDD8)
	// 8326BD98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326BD9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326BDA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326BDA4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8326BDA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326BDAC: 388BD4FC  addi r4, r11, -0x2b04
	ctx.r[4].s64 = ctx.r[11].s64 + -11012;
	// 8326BDB0: 386AC33C  addi r3, r10, -0x3cc4
	ctx.r[3].s64 = ctx.r[10].s64 + -15556;
	// 8326BDB4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326BDB8: 4AFC1119  bl 0x8222ced0
	ctx.lr = 0x8326BDBC;
	sub_8222CED0(ctx, base);
	// 8326BDBC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326BDC0: 3869E8C0  addi r3, r9, -0x1740
	ctx.r[3].s64 = ctx.r[9].s64 + -5952;
	// 8326BDC4: 4BA3E15D  bl 0x82ca9f20
	ctx.lr = 0x8326BDC8;
	sub_82CA9F20(ctx, base);
	// 8326BDC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326BDCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326BDD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326BDD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326BDD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326BDD8 size=64
    let mut pc: u32 = 0x8326BDD8;
    'dispatch: loop {
        match pc {
            0x8326BDD8 => {
    //   block [0x8326BDD8..0x8326BE18)
	// 8326BDD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326BDDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326BDE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326BDE4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326BDE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326BDEC: 388B01EC  addi r4, r11, 0x1ec
	ctx.r[4].s64 = ctx.r[11].s64 + 492;
	// 8326BDF0: 386AC340  addi r3, r10, -0x3cc0
	ctx.r[3].s64 = ctx.r[10].s64 + -15552;
	// 8326BDF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326BDF8: 4AFC10D9  bl 0x8222ced0
	ctx.lr = 0x8326BDFC;
	sub_8222CED0(ctx, base);
	// 8326BDFC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326BE00: 3869E8D0  addi r3, r9, -0x1730
	ctx.r[3].s64 = ctx.r[9].s64 + -5936;
	// 8326BE04: 4BA3E11D  bl 0x82ca9f20
	ctx.lr = 0x8326BE08;
	sub_82CA9F20(ctx, base);
	// 8326BE08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326BE0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326BE10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326BE14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326BE18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326BE18 size=64
    let mut pc: u32 = 0x8326BE18;
    'dispatch: loop {
        match pc {
            0x8326BE18 => {
    //   block [0x8326BE18..0x8326BE58)
	// 8326BE18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326BE1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326BE20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326BE24: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326BE28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326BE2C: 388B01F8  addi r4, r11, 0x1f8
	ctx.r[4].s64 = ctx.r[11].s64 + 504;
	// 8326BE30: 386AC344  addi r3, r10, -0x3cbc
	ctx.r[3].s64 = ctx.r[10].s64 + -15548;
	// 8326BE34: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326BE38: 4AFC1099  bl 0x8222ced0
	ctx.lr = 0x8326BE3C;
	sub_8222CED0(ctx, base);
	// 8326BE3C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326BE40: 3869E8E0  addi r3, r9, -0x1720
	ctx.r[3].s64 = ctx.r[9].s64 + -5920;
	// 8326BE44: 4BA3E0DD  bl 0x82ca9f20
	ctx.lr = 0x8326BE48;
	sub_82CA9F20(ctx, base);
	// 8326BE48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326BE4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326BE50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326BE54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326BE58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326BE58 size=64
    let mut pc: u32 = 0x8326BE58;
    'dispatch: loop {
        match pc {
            0x8326BE58 => {
    //   block [0x8326BE58..0x8326BE98)
	// 8326BE58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326BE5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326BE60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326BE64: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326BE68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326BE6C: 388B0200  addi r4, r11, 0x200
	ctx.r[4].s64 = ctx.r[11].s64 + 512;
	// 8326BE70: 386AC348  addi r3, r10, -0x3cb8
	ctx.r[3].s64 = ctx.r[10].s64 + -15544;
	// 8326BE74: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326BE78: 4AFC1059  bl 0x8222ced0
	ctx.lr = 0x8326BE7C;
	sub_8222CED0(ctx, base);
	// 8326BE7C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326BE80: 3869E8F0  addi r3, r9, -0x1710
	ctx.r[3].s64 = ctx.r[9].s64 + -5904;
	// 8326BE84: 4BA3E09D  bl 0x82ca9f20
	ctx.lr = 0x8326BE88;
	sub_82CA9F20(ctx, base);
	// 8326BE88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326BE8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326BE90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326BE94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326BE98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8326BE98 size=96
    let mut pc: u32 = 0x8326BE98;
    'dispatch: loop {
        match pc {
            0x8326BE98 => {
    //   block [0x8326BE98..0x8326BEF8)
	// 8326BE98: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 8326BE9C: 3921FFF0  addi r9, r1, -0x10
	ctx.r[9].s64 = ctx.r[1].s64 + -16;
	// 8326BEA0: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8326BEA4: 3901FFF4  addi r8, r1, -0xc
	ctx.r[8].s64 = ctx.r[1].s64 + -12;
	// 8326BEA8: 38E1FFF4  addi r7, r1, -0xc
	ctx.r[7].s64 = ctx.r[1].s64 + -12;
	// 8326BEAC: C1AA9484  lfs f13, -0x6b7c(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8326BEB0: 38C1FFF0  addi r6, r1, -0x10
	ctx.r[6].s64 = ctx.r[1].s64 + -16;
	// 8326BEB4: D1A1FFF0  stfs f13, -0x10(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 8326BEB8: 3CA0834A  lis r5, -0x7cb6
	ctx.r[5].s64 = -2092302336;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326BEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326BEF8 size=280
    let mut pc: u32 = 0x8326BEF8;
    'dispatch: loop {
        match pc {
            0x8326BEF8 => {
    //   block [0x8326BEF8..0x8326BF78)
	// 8326BEF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326BEFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326BF00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8326BF04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8326BF08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326BF0C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326BF10: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326BF14: 388B0500  addi r4, r11, 0x500
	ctx.r[4].s64 = ctx.r[11].s64 + 1280;
	// 8326BF18: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8326BF1C: 4AFC0FB5  bl 0x8222ced0
	ctx.lr = 0x8326BF20;
	sub_8222CED0(ctx, base);
	// 8326BF20: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8326BF24: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8326BF28: 388A02F4  addi r4, r10, 0x2f4
	ctx.r[4].s64 = ctx.r[10].s64 + 756;
	// 8326BF2C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326BF30: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8326BF34: 4AFC0F9D  bl 0x8222ced0
	ctx.lr = 0x8326BF38;
	sub_8222CED0(ctx, base);
	// 8326BF38: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8326BF3C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8326BF40: 4AF808B9  bl 0x821ec7f8
	ctx.lr = 0x8326BF44;
	sub_821EC7F8(ctx, base);
	// 8326BF44: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8326BF48: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8326BF4C: 4AF82BED  bl 0x821eeb38
	ctx.lr = 0x8326BF50;
	sub_821EEB38(ctx, base);
	// 8326BF50: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8326BF54: 4B99789D  bl 0x82c037f0
	ctx.lr = 0x8326BF58;
	sub_82C037F0(ctx, base);
	// 8326BF58: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8326BF5C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8326BF60: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8326BF64: 9169C34C  stw r11, -0x3cb4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-15540 as u32), ctx.r[11].u32 ) };
	// 8326BF68: 4AF5A801  bl 0x821c6768
	ctx.lr = 0x8326BF6C;
	sub_821C6768(ctx, base);
	// 8326BF6C: 3D008349  lis r8, -0x7cb7
	ctx.r[8].s64 = -2092367872;
	// 8326BF70: 3BC87088  addi r30, r8, 0x7088
	ctx.r[30].s64 = ctx.r[8].s64 + 28808;
	// 8326BF74: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	pc = 0x8326BF78; continue 'dispatch;
            }
            0x8326BF78 => {
    //   block [0x8326BF78..0x8326BFA8)
	// 8326BF78: 7CC000A6  mfmsr r6
	ctx.r[6].u64 = ctx.msr;
	// 8326BF7C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8326BF80: 7CE02828  lwarx r7, 0, r5
	// lwarx
	let ea = ctx.r[5].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[7].u64 = ctx.reserved.u32 as u64;
	// 8326BF84: 38E7FFFF  addi r7, r7, -1
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	// 8326BF88: 7CE0292D  stwcx. r7, 0, r5
	// stwcx.
	let addr = ctx.r[5].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[7].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8326BF8C: 7CC10164  mtmsrd r6, 1
	ctx.msr = (ctx.r[6].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8326BF90: 4082FFE8  bne 0x8326bf78
	if !ctx.cr[0].eq {
	pc = 0x8326BF78; continue 'dispatch;
	}
	// 8326BF94: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8326BF98: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8326BF9C: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 8326BFA0: 4AF5A7C9  bl 0x821c6768
	ctx.lr = 0x8326BFA4;
	sub_821C6768(ctx, base);
	// 8326BFA4: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	pc = 0x8326BFA8; continue 'dispatch;
            }
            0x8326BFA8 => {
    //   block [0x8326BFA8..0x8326BFCC)
	// 8326BFA8: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 8326BFAC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8326BFB0: 7C805828  lwarx r4, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 8326BFB4: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 8326BFB8: 7C80592D  stwcx. r4, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8326BFBC: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8326BFC0: 4082FFE8  bne 0x8326bfa8
	if !ctx.cr[0].eq {
	pc = 0x8326BFA8; continue 'dispatch;
	}
	// 8326BFC4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8326BFC8: 4AF5A7A1  bl 0x821c6768
	ctx.lr = 0x8326BFCC;
	sub_821C6768(ctx, base);
	pc = 0x8326BFCC; continue 'dispatch;
            }
            0x8326BFCC => {
    //   block [0x8326BFCC..0x8326C010)
	// 8326BFCC: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8326BFD0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8326BFD4: 7D40F028  lwarx r10, 0, r30
	// lwarx
	let ea = ctx.r[30].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8326BFD8: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8326BFDC: 7D40F12D  stwcx. r10, 0, r30
	// stwcx.
	let addr = ctx.r[30].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8326BFE0: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8326BFE4: 4082FFE8  bne 0x8326bfcc
	if !ctx.cr[0].eq {
	pc = 0x8326BFCC; continue 'dispatch;
	}
	// 8326BFE8: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 8326BFEC: 3D00832B  lis r8, -0x7cd5
	ctx.r[8].s64 = -2094333952;
	// 8326BFF0: 3868E900  addi r3, r8, -0x1700
	ctx.r[3].s64 = ctx.r[8].s64 + -5888;
	// 8326BFF4: 4BA3DF2D  bl 0x82ca9f20
	ctx.lr = 0x8326BFF8;
	sub_82CA9F20(ctx, base);
	// 8326BFF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8326BFFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C000: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C004: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8326C008: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8326C00C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326C010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C010 size=60
    let mut pc: u32 = 0x8326C010;
    'dispatch: loop {
        match pc {
            0x8326C010 => {
    //   block [0x8326C010..0x8326C04C)
	// 8326C010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C018: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C01C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8326C020: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C024: 388B0AF8  addi r4, r11, 0xaf8
	ctx.r[4].s64 = ctx.r[11].s64 + 2808;
	// 8326C028: 386AC360  addi r3, r10, -0x3ca0
	ctx.r[3].s64 = ctx.r[10].s64 + -15520;
	// 8326C02C: 4B06A3DD  bl 0x822d6408
	ctx.lr = 0x8326C030;
	sub_822D6408(ctx, base);
	// 8326C030: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326C034: 3869E908  addi r3, r9, -0x16f8
	ctx.r[3].s64 = ctx.r[9].s64 + -5880;
	// 8326C038: 4BA3DEE9  bl 0x82ca9f20
	ctx.lr = 0x8326C03C;
	sub_82CA9F20(ctx, base);
	// 8326C03C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C040: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C044: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C048: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326C050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C050 size=64
    let mut pc: u32 = 0x8326C050;
    'dispatch: loop {
        match pc {
            0x8326C050 => {
    //   block [0x8326C050..0x8326C090)
	// 8326C050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C058: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C05C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326C060: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C064: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8326C068: 386AC364  addi r3, r10, -0x3c9c
	ctx.r[3].s64 = ctx.r[10].s64 + -15516;
	// 8326C06C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326C070: 4AFC0E61  bl 0x8222ced0
	ctx.lr = 0x8326C074;
	sub_8222CED0(ctx, base);
	// 8326C074: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326C078: 3869E918  addi r3, r9, -0x16e8
	ctx.r[3].s64 = ctx.r[9].s64 + -5864;
	// 8326C07C: 4BA3DEA5  bl 0x82ca9f20
	ctx.lr = 0x8326C080;
	sub_82CA9F20(ctx, base);
	// 8326C080: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C084: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C088: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C08C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326C090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C090 size=64
    let mut pc: u32 = 0x8326C090;
    'dispatch: loop {
        match pc {
            0x8326C090 => {
    //   block [0x8326C090..0x8326C0D0)
	// 8326C090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C098: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C09C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326C0A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C0A4: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8326C0A8: 386AC368  addi r3, r10, -0x3c98
	ctx.r[3].s64 = ctx.r[10].s64 + -15512;
	// 8326C0AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326C0B0: 4AFC0E21  bl 0x8222ced0
	ctx.lr = 0x8326C0B4;
	sub_8222CED0(ctx, base);
	// 8326C0B4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326C0B8: 3869E928  addi r3, r9, -0x16d8
	ctx.r[3].s64 = ctx.r[9].s64 + -5848;
	// 8326C0BC: 4BA3DE65  bl 0x82ca9f20
	ctx.lr = 0x8326C0C0;
	sub_82CA9F20(ctx, base);
	// 8326C0C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C0C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C0C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C0CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326C0D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C0D0 size=64
    let mut pc: u32 = 0x8326C0D0;
    'dispatch: loop {
        match pc {
            0x8326C0D0 => {
    //   block [0x8326C0D0..0x8326C110)
	// 8326C0D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C0D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C0D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C0DC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326C0E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C0E4: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8326C0E8: 386AC36C  addi r3, r10, -0x3c94
	ctx.r[3].s64 = ctx.r[10].s64 + -15508;
	// 8326C0EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326C0F0: 4AFC0DE1  bl 0x8222ced0
	ctx.lr = 0x8326C0F4;
	sub_8222CED0(ctx, base);
	// 8326C0F4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326C0F8: 3869E938  addi r3, r9, -0x16c8
	ctx.r[3].s64 = ctx.r[9].s64 + -5832;
	// 8326C0FC: 4BA3DE25  bl 0x82ca9f20
	ctx.lr = 0x8326C100;
	sub_82CA9F20(ctx, base);
	// 8326C100: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C104: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C108: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C10C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326C110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C110 size=64
    let mut pc: u32 = 0x8326C110;
    'dispatch: loop {
        match pc {
            0x8326C110 => {
    //   block [0x8326C110..0x8326C150)
	// 8326C110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C118: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C11C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326C120: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C124: 388B0B80  addi r4, r11, 0xb80
	ctx.r[4].s64 = ctx.r[11].s64 + 2944;
	// 8326C128: 386AC370  addi r3, r10, -0x3c90
	ctx.r[3].s64 = ctx.r[10].s64 + -15504;
	// 8326C12C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326C130: 4AFC0DA1  bl 0x8222ced0
	ctx.lr = 0x8326C134;
	sub_8222CED0(ctx, base);
	// 8326C134: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326C138: 3869E948  addi r3, r9, -0x16b8
	ctx.r[3].s64 = ctx.r[9].s64 + -5816;
	// 8326C13C: 4BA3DDE5  bl 0x82ca9f20
	ctx.lr = 0x8326C140;
	sub_82CA9F20(ctx, base);
	// 8326C140: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C144: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C148: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C14C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326C150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C150 size=64
    let mut pc: u32 = 0x8326C150;
    'dispatch: loop {
        match pc {
            0x8326C150 => {
    //   block [0x8326C150..0x8326C190)
	// 8326C150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C158: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C15C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326C160: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C164: 388B0B8C  addi r4, r11, 0xb8c
	ctx.r[4].s64 = ctx.r[11].s64 + 2956;
	// 8326C168: 386AC374  addi r3, r10, -0x3c8c
	ctx.r[3].s64 = ctx.r[10].s64 + -15500;
	// 8326C16C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326C170: 4AFC0D61  bl 0x8222ced0
	ctx.lr = 0x8326C174;
	sub_8222CED0(ctx, base);
	// 8326C174: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326C178: 3869E958  addi r3, r9, -0x16a8
	ctx.r[3].s64 = ctx.r[9].s64 + -5800;
	// 8326C17C: 4BA3DDA5  bl 0x82ca9f20
	ctx.lr = 0x8326C180;
	sub_82CA9F20(ctx, base);
	// 8326C180: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C184: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C188: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C18C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326C190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C190 size=64
    let mut pc: u32 = 0x8326C190;
    'dispatch: loop {
        match pc {
            0x8326C190 => {
    //   block [0x8326C190..0x8326C1D0)
	// 8326C190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C198: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C19C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326C1A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C1A4: 388B0B98  addi r4, r11, 0xb98
	ctx.r[4].s64 = ctx.r[11].s64 + 2968;
	// 8326C1A8: 386AC378  addi r3, r10, -0x3c88
	ctx.r[3].s64 = ctx.r[10].s64 + -15496;
	// 8326C1AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326C1B0: 4AFC0D21  bl 0x8222ced0
	ctx.lr = 0x8326C1B4;
	sub_8222CED0(ctx, base);
	// 8326C1B4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326C1B8: 3869E968  addi r3, r9, -0x1698
	ctx.r[3].s64 = ctx.r[9].s64 + -5784;
	// 8326C1BC: 4BA3DD65  bl 0x82ca9f20
	ctx.lr = 0x8326C1C0;
	sub_82CA9F20(ctx, base);
	// 8326C1C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C1C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C1C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C1CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326C1D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C1D0 size=64
    let mut pc: u32 = 0x8326C1D0;
    'dispatch: loop {
        match pc {
            0x8326C1D0 => {
    //   block [0x8326C1D0..0x8326C210)
	// 8326C1D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C1D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C1D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C1DC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326C1E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C1E4: 388B0BA4  addi r4, r11, 0xba4
	ctx.r[4].s64 = ctx.r[11].s64 + 2980;
	// 8326C1E8: 386AC37C  addi r3, r10, -0x3c84
	ctx.r[3].s64 = ctx.r[10].s64 + -15492;
	// 8326C1EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326C1F0: 4AFC0CE1  bl 0x8222ced0
	ctx.lr = 0x8326C1F4;
	sub_8222CED0(ctx, base);
	// 8326C1F4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326C1F8: 3869E978  addi r3, r9, -0x1688
	ctx.r[3].s64 = ctx.r[9].s64 + -5768;
	// 8326C1FC: 4BA3DD25  bl 0x82ca9f20
	ctx.lr = 0x8326C200;
	sub_82CA9F20(ctx, base);
	// 8326C200: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C204: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C208: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C20C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326C210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C210 size=64
    let mut pc: u32 = 0x8326C210;
    'dispatch: loop {
        match pc {
            0x8326C210 => {
    //   block [0x8326C210..0x8326C250)
	// 8326C210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C218: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C21C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326C220: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C224: 388B0BB4  addi r4, r11, 0xbb4
	ctx.r[4].s64 = ctx.r[11].s64 + 2996;
	// 8326C228: 386AC380  addi r3, r10, -0x3c80
	ctx.r[3].s64 = ctx.r[10].s64 + -15488;
	// 8326C22C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326C230: 4AFC0CA1  bl 0x8222ced0
	ctx.lr = 0x8326C234;
	sub_8222CED0(ctx, base);
	// 8326C234: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326C238: 3869E988  addi r3, r9, -0x1678
	ctx.r[3].s64 = ctx.r[9].s64 + -5752;
	// 8326C23C: 4BA3DCE5  bl 0x82ca9f20
	ctx.lr = 0x8326C240;
	sub_82CA9F20(ctx, base);
	// 8326C240: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C244: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C248: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C24C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326C250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C250 size=64
    let mut pc: u32 = 0x8326C250;
    'dispatch: loop {
        match pc {
            0x8326C250 => {
    //   block [0x8326C250..0x8326C290)
	// 8326C250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C258: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C25C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326C260: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C264: 388B0B80  addi r4, r11, 0xb80
	ctx.r[4].s64 = ctx.r[11].s64 + 2944;
	// 8326C268: 386AC384  addi r3, r10, -0x3c7c
	ctx.r[3].s64 = ctx.r[10].s64 + -15484;
	// 8326C26C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326C270: 4AFC0C61  bl 0x8222ced0
	ctx.lr = 0x8326C274;
	sub_8222CED0(ctx, base);
	// 8326C274: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326C278: 3869E998  addi r3, r9, -0x1668
	ctx.r[3].s64 = ctx.r[9].s64 + -5736;
	// 8326C27C: 4BA3DCA5  bl 0x82ca9f20
	ctx.lr = 0x8326C280;
	sub_82CA9F20(ctx, base);
	// 8326C280: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C284: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C288: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C28C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326C290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C290 size=64
    let mut pc: u32 = 0x8326C290;
    'dispatch: loop {
        match pc {
            0x8326C290 => {
    //   block [0x8326C290..0x8326C2D0)
	// 8326C290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C298: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C29C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326C2A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C2A4: 388B0B98  addi r4, r11, 0xb98
	ctx.r[4].s64 = ctx.r[11].s64 + 2968;
	// 8326C2A8: 386AC388  addi r3, r10, -0x3c78
	ctx.r[3].s64 = ctx.r[10].s64 + -15480;
	// 8326C2AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326C2B0: 4AFC0C21  bl 0x8222ced0
	ctx.lr = 0x8326C2B4;
	sub_8222CED0(ctx, base);
	// 8326C2B4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326C2B8: 3869E9A8  addi r3, r9, -0x1658
	ctx.r[3].s64 = ctx.r[9].s64 + -5720;
	// 8326C2BC: 4BA3DC65  bl 0x82ca9f20
	ctx.lr = 0x8326C2C0;
	sub_82CA9F20(ctx, base);
	// 8326C2C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C2C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C2C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C2CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326C2D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C2D0 size=64
    let mut pc: u32 = 0x8326C2D0;
    'dispatch: loop {
        match pc {
            0x8326C2D0 => {
    //   block [0x8326C2D0..0x8326C310)
	// 8326C2D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C2D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C2D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C2DC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326C2E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C2E4: 388B0D04  addi r4, r11, 0xd04
	ctx.r[4].s64 = ctx.r[11].s64 + 3332;
	// 8326C2E8: 386AC38C  addi r3, r10, -0x3c74
	ctx.r[3].s64 = ctx.r[10].s64 + -15476;
	// 8326C2EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326C2F0: 4AFC0BE1  bl 0x8222ced0
	ctx.lr = 0x8326C2F4;
	sub_8222CED0(ctx, base);
	// 8326C2F4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326C2F8: 3869E9B8  addi r3, r9, -0x1648
	ctx.r[3].s64 = ctx.r[9].s64 + -5704;
	// 8326C2FC: 4BA3DC25  bl 0x82ca9f20
	ctx.lr = 0x8326C300;
	sub_82CA9F20(ctx, base);
	// 8326C300: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C304: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C308: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C30C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326C310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C310 size=96
    let mut pc: u32 = 0x8326C310;
    'dispatch: loop {
        match pc {
            0x8326C310 => {
    //   block [0x8326C310..0x8326C334)
	// 8326C310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C318: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C31C: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 8326C320: 4AFB2F39  bl 0x8221f258
	ctx.lr = 0x8326C324;
	sub_8221F258(ctx, base);
	// 8326C324: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8326C328: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8326C32C: 419A0008  beq cr6, 0x8326c334
	if ctx.cr[6].eq {
	pc = 0x8326C334; continue 'dispatch;
	}
	// 8326C330: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x8326C334; continue 'dispatch;
            }
            0x8326C334 => {
    //   block [0x8326C334..0x8326C340)
	// 8326C334: 354B0004  addic. r10, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8326C338: 41820008  beq 0x8326c340
	if ctx.cr[0].eq {
	pc = 0x8326C340; continue 'dispatch;
	}
	// 8326C33C: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x8326C340; continue 'dispatch;
            }
            0x8326C340 => {
    //   block [0x8326C340..0x8326C370)
	// 8326C340: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8326C344: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8326C348: 3909C390  addi r8, r9, -0x3c70
	ctx.r[8].s64 = ctx.r[9].s64 + -15472;
	// 8326C34C: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8326C350: 3867E9C8  addi r3, r7, -0x1638
	ctx.r[3].s64 = ctx.r[7].s64 + -5688;
	// 8326C354: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8326C358: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8326C35C: 4BA3DBC5  bl 0x82ca9f20
	ctx.lr = 0x8326C360;
	sub_82CA9F20(ctx, base);
	// 8326C360: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C364: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C368: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C36C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326C370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8326C370 size=12
    let mut pc: u32 = 0x8326C370;
    'dispatch: loop {
        match pc {
            0x8326C370 => {
    //   block [0x8326C370..0x8326C37C)
	// 8326C370: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8326C374: 386BEA50  addi r3, r11, -0x15b0
	ctx.r[3].s64 = ctx.r[11].s64 + -5552;
	// 8326C378: 4BA3DBA8  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326C380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C380 size=64
    let mut pc: u32 = 0x8326C380;
    'dispatch: loop {
        match pc {
            0x8326C380 => {
    //   block [0x8326C380..0x8326C3C0)
	// 8326C380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C388: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C38C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326C390: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C394: 388B0E00  addi r4, r11, 0xe00
	ctx.r[4].s64 = ctx.r[11].s64 + 3584;
	// 8326C398: 386AC3B0  addi r3, r10, -0x3c50
	ctx.r[3].s64 = ctx.r[10].s64 + -15440;
	// 8326C39C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326C3A0: 4AFC0B31  bl 0x8222ced0
	ctx.lr = 0x8326C3A4;
	sub_8222CED0(ctx, base);
	// 8326C3A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326C3A8: 3869EA60  addi r3, r9, -0x15a0
	ctx.r[3].s64 = ctx.r[9].s64 + -5536;
	// 8326C3AC: 4BA3DB75  bl 0x82ca9f20
	ctx.lr = 0x8326C3B0;
	sub_82CA9F20(ctx, base);
	// 8326C3B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C3B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C3B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C3BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326C3C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C3C0 size=64
    let mut pc: u32 = 0x8326C3C0;
    'dispatch: loop {
        match pc {
            0x8326C3C0 => {
    //   block [0x8326C3C0..0x8326C400)
	// 8326C3C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C3C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C3C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C3CC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326C3D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C3D4: 388B0E24  addi r4, r11, 0xe24
	ctx.r[4].s64 = ctx.r[11].s64 + 3620;
	// 8326C3D8: 386AC3B4  addi r3, r10, -0x3c4c
	ctx.r[3].s64 = ctx.r[10].s64 + -15436;
	// 8326C3DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326C3E0: 4AFC0AF1  bl 0x8222ced0
	ctx.lr = 0x8326C3E4;
	sub_8222CED0(ctx, base);
	// 8326C3E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326C3E8: 3869EA70  addi r3, r9, -0x1590
	ctx.r[3].s64 = ctx.r[9].s64 + -5520;
	// 8326C3EC: 4BA3DB35  bl 0x82ca9f20
	ctx.lr = 0x8326C3F0;
	sub_82CA9F20(ctx, base);
	// 8326C3F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C3F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C3F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C3FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326C400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C400 size=64
    let mut pc: u32 = 0x8326C400;
    'dispatch: loop {
        match pc {
            0x8326C400 => {
    //   block [0x8326C400..0x8326C440)
	// 8326C400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C408: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C40C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326C410: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C414: 388B1064  addi r4, r11, 0x1064
	ctx.r[4].s64 = ctx.r[11].s64 + 4196;
	// 8326C418: 386AC3B8  addi r3, r10, -0x3c48
	ctx.r[3].s64 = ctx.r[10].s64 + -15432;
	// 8326C41C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326C420: 4AFC0AB1  bl 0x8222ced0
	ctx.lr = 0x8326C424;
	sub_8222CED0(ctx, base);
	// 8326C424: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326C428: 3869EA90  addi r3, r9, -0x1570
	ctx.r[3].s64 = ctx.r[9].s64 + -5488;
	// 8326C42C: 4BA3DAF5  bl 0x82ca9f20
	ctx.lr = 0x8326C430;
	sub_82CA9F20(ctx, base);
	// 8326C430: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C434: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C438: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C43C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326C440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8326C440 size=12
    let mut pc: u32 = 0x8326C440;
    'dispatch: loop {
        match pc {
            0x8326C440 => {
    //   block [0x8326C440..0x8326C44C)
	// 8326C440: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8326C444: 386BC3C0  addi r3, r11, -0x3c40
	ctx.r[3].s64 = ctx.r[11].s64 + -15424;
	// 8326C448: 4B04C4F0  b 0x822b8938
	sub_822B8938(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326C450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8326C450 size=12
    let mut pc: u32 = 0x8326C450;
    'dispatch: loop {
        match pc {
            0x8326C450 => {
    //   block [0x8326C450..0x8326C45C)
	// 8326C450: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8326C454: 386BC420  addi r3, r11, -0x3be0
	ctx.r[3].s64 = ctx.r[11].s64 + -15328;
	// 8326C458: 4B04C4E0  b 0x822b8938
	sub_822B8938(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326C460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C460 size=56
    let mut pc: u32 = 0x8326C460;
    'dispatch: loop {
        match pc {
            0x8326C460 => {
    //   block [0x8326C460..0x8326C498)
	// 8326C460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C468: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C46C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326C470: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326C474: 386B2DF8  addi r3, r11, 0x2df8
	ctx.r[3].s64 = ctx.r[11].s64 + 11768;
	// 8326C478: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326C47C: 4AF878DD  bl 0x821f3d58
	ctx.lr = 0x8326C480;
	sub_821F3D58(ctx, base);
	// 8326C480: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C484: 906AC680  stw r3, -0x3980(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14720 as u32), ctx.r[3].u32 ) };
	// 8326C488: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C48C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C490: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C494: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326C498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C498 size=56
    let mut pc: u32 = 0x8326C498;
    'dispatch: loop {
        match pc {
            0x8326C498 => {
    //   block [0x8326C498..0x8326C4D0)
	// 8326C498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C49C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C4A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C4A4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8326C4A8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326C4AC: 386B0F7C  addi r3, r11, 0xf7c
	ctx.r[3].s64 = ctx.r[11].s64 + 3964;
	// 8326C4B0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326C4B4: 4AF878A5  bl 0x821f3d58
	ctx.lr = 0x8326C4B8;
	sub_821F3D58(ctx, base);
	// 8326C4B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C4BC: 906AC684  stw r3, -0x397c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14716 as u32), ctx.r[3].u32 ) };
	// 8326C4C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C4C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C4C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C4CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326C4D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C4D0 size=56
    let mut pc: u32 = 0x8326C4D0;
    'dispatch: loop {
        match pc {
            0x8326C4D0 => {
    //   block [0x8326C4D0..0x8326C508)
	// 8326C4D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C4D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C4D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C4DC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8326C4E0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326C4E4: 386B70CC  addi r3, r11, 0x70cc
	ctx.r[3].s64 = ctx.r[11].s64 + 28876;
	// 8326C4E8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326C4EC: 4AF8786D  bl 0x821f3d58
	ctx.lr = 0x8326C4F0;
	sub_821F3D58(ctx, base);
	// 8326C4F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C4F4: 906AC688  stw r3, -0x3978(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14712 as u32), ctx.r[3].u32 ) };
	// 8326C4F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C4FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C500: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C504: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326C508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C508 size=56
    let mut pc: u32 = 0x8326C508;
    'dispatch: loop {
        match pc {
            0x8326C508 => {
    //   block [0x8326C508..0x8326C540)
	// 8326C508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C50C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C510: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C514: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8326C518: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326C51C: 386B70E8  addi r3, r11, 0x70e8
	ctx.r[3].s64 = ctx.r[11].s64 + 28904;
	// 8326C520: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326C524: 4AF87835  bl 0x821f3d58
	ctx.lr = 0x8326C528;
	sub_821F3D58(ctx, base);
	// 8326C528: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C52C: 906AC68C  stw r3, -0x3974(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14708 as u32), ctx.r[3].u32 ) };
	// 8326C530: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C534: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C538: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C53C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326C540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C540 size=56
    let mut pc: u32 = 0x8326C540;
    'dispatch: loop {
        match pc {
            0x8326C540 => {
    //   block [0x8326C540..0x8326C578)
	// 8326C540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C548: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C54C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8326C550: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326C554: 386B70F4  addi r3, r11, 0x70f4
	ctx.r[3].s64 = ctx.r[11].s64 + 28916;
	// 8326C558: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326C55C: 4AF877FD  bl 0x821f3d58
	ctx.lr = 0x8326C560;
	sub_821F3D58(ctx, base);
	// 8326C560: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C564: 906AC690  stw r3, -0x3970(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14704 as u32), ctx.r[3].u32 ) };
	// 8326C568: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C56C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C570: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C574: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326C578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C578 size=56
    let mut pc: u32 = 0x8326C578;
    'dispatch: loop {
        match pc {
            0x8326C578 => {
    //   block [0x8326C578..0x8326C5B0)
	// 8326C578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C57C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C580: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C584: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8326C588: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326C58C: 386B7100  addi r3, r11, 0x7100
	ctx.r[3].s64 = ctx.r[11].s64 + 28928;
	// 8326C590: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326C594: 4AF877C5  bl 0x821f3d58
	ctx.lr = 0x8326C598;
	sub_821F3D58(ctx, base);
	// 8326C598: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C59C: 906AC694  stw r3, -0x396c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14700 as u32), ctx.r[3].u32 ) };
	// 8326C5A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C5A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C5A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C5AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326C5B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C5B0 size=56
    let mut pc: u32 = 0x8326C5B0;
    'dispatch: loop {
        match pc {
            0x8326C5B0 => {
    //   block [0x8326C5B0..0x8326C5E8)
	// 8326C5B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C5B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C5B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C5BC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8326C5C0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326C5C4: 386B710C  addi r3, r11, 0x710c
	ctx.r[3].s64 = ctx.r[11].s64 + 28940;
	// 8326C5C8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326C5CC: 4AF8778D  bl 0x821f3d58
	ctx.lr = 0x8326C5D0;
	sub_821F3D58(ctx, base);
	// 8326C5D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C5D4: 906AC698  stw r3, -0x3968(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14696 as u32), ctx.r[3].u32 ) };
	// 8326C5D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C5DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C5E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C5E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326C5E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C5E8 size=56
    let mut pc: u32 = 0x8326C5E8;
    'dispatch: loop {
        match pc {
            0x8326C5E8 => {
    //   block [0x8326C5E8..0x8326C620)
	// 8326C5E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C5EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C5F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C5F4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8326C5F8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326C5FC: 386B712C  addi r3, r11, 0x712c
	ctx.r[3].s64 = ctx.r[11].s64 + 28972;
	// 8326C600: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326C604: 4AF87755  bl 0x821f3d58
	ctx.lr = 0x8326C608;
	sub_821F3D58(ctx, base);
	// 8326C608: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C60C: 906AC69C  stw r3, -0x3964(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14692 as u32), ctx.r[3].u32 ) };
	// 8326C610: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C614: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C618: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C61C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326C620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C620 size=56
    let mut pc: u32 = 0x8326C620;
    'dispatch: loop {
        match pc {
            0x8326C620 => {
    //   block [0x8326C620..0x8326C658)
	// 8326C620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C628: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C62C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8326C630: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326C634: 386B7158  addi r3, r11, 0x7158
	ctx.r[3].s64 = ctx.r[11].s64 + 29016;
	// 8326C638: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326C63C: 4AF8771D  bl 0x821f3d58
	ctx.lr = 0x8326C640;
	sub_821F3D58(ctx, base);
	// 8326C640: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C644: 906AC6A0  stw r3, -0x3960(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14688 as u32), ctx.r[3].u32 ) };
	// 8326C648: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C64C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C650: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C654: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326C658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C658 size=56
    let mut pc: u32 = 0x8326C658;
    'dispatch: loop {
        match pc {
            0x8326C658 => {
    //   block [0x8326C658..0x8326C690)
	// 8326C658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C65C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C660: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C664: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8326C668: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326C66C: 386B7164  addi r3, r11, 0x7164
	ctx.r[3].s64 = ctx.r[11].s64 + 29028;
	// 8326C670: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326C674: 4AF876E5  bl 0x821f3d58
	ctx.lr = 0x8326C678;
	sub_821F3D58(ctx, base);
	// 8326C678: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C67C: 906AC6A4  stw r3, -0x395c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14684 as u32), ctx.r[3].u32 ) };
	// 8326C680: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C684: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C688: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C68C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326C690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C690 size=56
    let mut pc: u32 = 0x8326C690;
    'dispatch: loop {
        match pc {
            0x8326C690 => {
    //   block [0x8326C690..0x8326C6C8)
	// 8326C690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C698: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C69C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8326C6A0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326C6A4: 386B7174  addi r3, r11, 0x7174
	ctx.r[3].s64 = ctx.r[11].s64 + 29044;
	// 8326C6A8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326C6AC: 4AF876AD  bl 0x821f3d58
	ctx.lr = 0x8326C6B0;
	sub_821F3D58(ctx, base);
	// 8326C6B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C6B4: 906AC6A8  stw r3, -0x3958(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14680 as u32), ctx.r[3].u32 ) };
	// 8326C6B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C6BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C6C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C6C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326C6C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C6C8 size=56
    let mut pc: u32 = 0x8326C6C8;
    'dispatch: loop {
        match pc {
            0x8326C6C8 => {
    //   block [0x8326C6C8..0x8326C700)
	// 8326C6C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C6CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C6D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C6D4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8326C6D8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326C6DC: 386B7140  addi r3, r11, 0x7140
	ctx.r[3].s64 = ctx.r[11].s64 + 28992;
	// 8326C6E0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326C6E4: 4AF87675  bl 0x821f3d58
	ctx.lr = 0x8326C6E8;
	sub_821F3D58(ctx, base);
	// 8326C6E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C6EC: 906AC6AC  stw r3, -0x3954(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14676 as u32), ctx.r[3].u32 ) };
	// 8326C6F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C6F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C6F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C6FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326C700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C700 size=56
    let mut pc: u32 = 0x8326C700;
    'dispatch: loop {
        match pc {
            0x8326C700 => {
    //   block [0x8326C700..0x8326C738)
	// 8326C700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C708: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C70C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326C710: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326C714: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 8326C718: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326C71C: 4AF8763D  bl 0x821f3d58
	ctx.lr = 0x8326C720;
	sub_821F3D58(ctx, base);
	// 8326C720: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C724: 906AC6B0  stw r3, -0x3950(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14672 as u32), ctx.r[3].u32 ) };
	// 8326C728: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C72C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C730: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C734: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326C738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C738 size=56
    let mut pc: u32 = 0x8326C738;
    'dispatch: loop {
        match pc {
            0x8326C738 => {
    //   block [0x8326C738..0x8326C770)
	// 8326C738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C73C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C740: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C744: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326C748: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326C74C: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 8326C750: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326C754: 4AF87605  bl 0x821f3d58
	ctx.lr = 0x8326C758;
	sub_821F3D58(ctx, base);
	// 8326C758: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C75C: 906AC6B4  stw r3, -0x394c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14668 as u32), ctx.r[3].u32 ) };
	// 8326C760: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C764: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C768: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C76C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326C770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C770 size=56
    let mut pc: u32 = 0x8326C770;
    'dispatch: loop {
        match pc {
            0x8326C770 => {
    //   block [0x8326C770..0x8326C7A8)
	// 8326C770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C778: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C77C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326C780: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326C784: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 8326C788: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326C78C: 4AF875CD  bl 0x821f3d58
	ctx.lr = 0x8326C790;
	sub_821F3D58(ctx, base);
	// 8326C790: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C794: 906AC6B8  stw r3, -0x3948(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14664 as u32), ctx.r[3].u32 ) };
	// 8326C798: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C79C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C7A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C7A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326C7A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C7A8 size=56
    let mut pc: u32 = 0x8326C7A8;
    'dispatch: loop {
        match pc {
            0x8326C7A8 => {
    //   block [0x8326C7A8..0x8326C7E0)
	// 8326C7A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C7AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C7B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C7B4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326C7B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326C7BC: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 8326C7C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326C7C4: 4AF87595  bl 0x821f3d58
	ctx.lr = 0x8326C7C8;
	sub_821F3D58(ctx, base);
	// 8326C7C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C7CC: 906AC6BC  stw r3, -0x3944(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14660 as u32), ctx.r[3].u32 ) };
	// 8326C7D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C7D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C7D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C7DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326C7E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C7E0 size=56
    let mut pc: u32 = 0x8326C7E0;
    'dispatch: loop {
        match pc {
            0x8326C7E0 => {
    //   block [0x8326C7E0..0x8326C818)
	// 8326C7E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C7E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C7E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C7EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326C7F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326C7F4: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 8326C7F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326C7FC: 4AF8755D  bl 0x821f3d58
	ctx.lr = 0x8326C800;
	sub_821F3D58(ctx, base);
	// 8326C800: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C804: 906AC6C0  stw r3, -0x3940(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14656 as u32), ctx.r[3].u32 ) };
	// 8326C808: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C80C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C810: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C814: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326C818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C818 size=56
    let mut pc: u32 = 0x8326C818;
    'dispatch: loop {
        match pc {
            0x8326C818 => {
    //   block [0x8326C818..0x8326C850)
	// 8326C818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C81C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C820: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C824: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326C828: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326C82C: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 8326C830: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326C834: 4AF87525  bl 0x821f3d58
	ctx.lr = 0x8326C838;
	sub_821F3D58(ctx, base);
	// 8326C838: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C83C: 906AC6C4  stw r3, -0x393c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14652 as u32), ctx.r[3].u32 ) };
	// 8326C840: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C844: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C848: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C84C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326C850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C850 size=56
    let mut pc: u32 = 0x8326C850;
    'dispatch: loop {
        match pc {
            0x8326C850 => {
    //   block [0x8326C850..0x8326C888)
	// 8326C850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C858: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C85C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326C860: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326C864: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 8326C868: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326C86C: 4AF874ED  bl 0x821f3d58
	ctx.lr = 0x8326C870;
	sub_821F3D58(ctx, base);
	// 8326C870: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C874: 906AC6C8  stw r3, -0x3938(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14648 as u32), ctx.r[3].u32 ) };
	// 8326C878: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C87C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C880: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C884: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326C888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C888 size=56
    let mut pc: u32 = 0x8326C888;
    'dispatch: loop {
        match pc {
            0x8326C888 => {
    //   block [0x8326C888..0x8326C8C0)
	// 8326C888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C88C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C890: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C894: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326C898: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326C89C: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 8326C8A0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326C8A4: 4AF874B5  bl 0x821f3d58
	ctx.lr = 0x8326C8A8;
	sub_821F3D58(ctx, base);
	// 8326C8A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C8AC: 906AC6CC  stw r3, -0x3934(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14644 as u32), ctx.r[3].u32 ) };
	// 8326C8B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C8B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C8B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C8BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326C8C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C8C0 size=56
    let mut pc: u32 = 0x8326C8C0;
    'dispatch: loop {
        match pc {
            0x8326C8C0 => {
    //   block [0x8326C8C0..0x8326C8F8)
	// 8326C8C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C8C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C8C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C8CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326C8D0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326C8D4: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 8326C8D8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326C8DC: 4AF8747D  bl 0x821f3d58
	ctx.lr = 0x8326C8E0;
	sub_821F3D58(ctx, base);
	// 8326C8E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C8E4: 906AC6D0  stw r3, -0x3930(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14640 as u32), ctx.r[3].u32 ) };
	// 8326C8E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C8EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C8F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C8F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326C8F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C8F8 size=56
    let mut pc: u32 = 0x8326C8F8;
    'dispatch: loop {
        match pc {
            0x8326C8F8 => {
    //   block [0x8326C8F8..0x8326C930)
	// 8326C8F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C8FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C900: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C904: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326C908: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326C90C: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 8326C910: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326C914: 4AF87445  bl 0x821f3d58
	ctx.lr = 0x8326C918;
	sub_821F3D58(ctx, base);
	// 8326C918: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C91C: 906AC6D4  stw r3, -0x392c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14636 as u32), ctx.r[3].u32 ) };
	// 8326C920: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C924: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C928: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C92C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326C930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C930 size=56
    let mut pc: u32 = 0x8326C930;
    'dispatch: loop {
        match pc {
            0x8326C930 => {
    //   block [0x8326C930..0x8326C968)
	// 8326C930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C938: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C93C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326C940: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326C944: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 8326C948: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326C94C: 4AF8740D  bl 0x821f3d58
	ctx.lr = 0x8326C950;
	sub_821F3D58(ctx, base);
	// 8326C950: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C954: 906AC6D8  stw r3, -0x3928(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14632 as u32), ctx.r[3].u32 ) };
	// 8326C958: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C95C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C960: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C964: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326C968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C968 size=56
    let mut pc: u32 = 0x8326C968;
    'dispatch: loop {
        match pc {
            0x8326C968 => {
    //   block [0x8326C968..0x8326C9A0)
	// 8326C968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C96C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C970: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C974: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326C978: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326C97C: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 8326C980: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326C984: 4AF873D5  bl 0x821f3d58
	ctx.lr = 0x8326C988;
	sub_821F3D58(ctx, base);
	// 8326C988: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C98C: 906AC6DC  stw r3, -0x3924(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14628 as u32), ctx.r[3].u32 ) };
	// 8326C990: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C994: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C998: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C99C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326C9A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C9A0 size=56
    let mut pc: u32 = 0x8326C9A0;
    'dispatch: loop {
        match pc {
            0x8326C9A0 => {
    //   block [0x8326C9A0..0x8326C9D8)
	// 8326C9A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C9A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C9A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C9AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326C9B0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326C9B4: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 8326C9B8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326C9BC: 4AF8739D  bl 0x821f3d58
	ctx.lr = 0x8326C9C0;
	sub_821F3D58(ctx, base);
	// 8326C9C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C9C4: 906AC6E0  stw r3, -0x3920(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14624 as u32), ctx.r[3].u32 ) };
	// 8326C9C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C9CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C9D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C9D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326C9D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C9D8 size=56
    let mut pc: u32 = 0x8326C9D8;
    'dispatch: loop {
        match pc {
            0x8326C9D8 => {
    //   block [0x8326C9D8..0x8326CA10)
	// 8326C9D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C9DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C9E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C9E4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326C9E8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326C9EC: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 8326C9F0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326C9F4: 4AF87365  bl 0x821f3d58
	ctx.lr = 0x8326C9F8;
	sub_821F3D58(ctx, base);
	// 8326C9F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C9FC: 906AC6E4  stw r3, -0x391c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14620 as u32), ctx.r[3].u32 ) };
	// 8326CA00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326CA04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326CA08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326CA0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326CA10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326CA10 size=56
    let mut pc: u32 = 0x8326CA10;
    'dispatch: loop {
        match pc {
            0x8326CA10 => {
    //   block [0x8326CA10..0x8326CA48)
	// 8326CA10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326CA14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326CA18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326CA1C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326CA20: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326CA24: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 8326CA28: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326CA2C: 4AF8732D  bl 0x821f3d58
	ctx.lr = 0x8326CA30;
	sub_821F3D58(ctx, base);
	// 8326CA30: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326CA34: 906AC6E8  stw r3, -0x3918(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14616 as u32), ctx.r[3].u32 ) };
	// 8326CA38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326CA3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326CA40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326CA44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326CA48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326CA48 size=56
    let mut pc: u32 = 0x8326CA48;
    'dispatch: loop {
        match pc {
            0x8326CA48 => {
    //   block [0x8326CA48..0x8326CA80)
	// 8326CA48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326CA4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326CA50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326CA54: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326CA58: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326CA5C: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 8326CA60: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326CA64: 4AF872F5  bl 0x821f3d58
	ctx.lr = 0x8326CA68;
	sub_821F3D58(ctx, base);
	// 8326CA68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326CA6C: 906AC6EC  stw r3, -0x3914(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14612 as u32), ctx.r[3].u32 ) };
	// 8326CA70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326CA74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326CA78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326CA7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326CA80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326CA80 size=56
    let mut pc: u32 = 0x8326CA80;
    'dispatch: loop {
        match pc {
            0x8326CA80 => {
    //   block [0x8326CA80..0x8326CAB8)
	// 8326CA80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326CA84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326CA88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326CA8C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326CA90: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326CA94: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 8326CA98: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326CA9C: 4AF872BD  bl 0x821f3d58
	ctx.lr = 0x8326CAA0;
	sub_821F3D58(ctx, base);
	// 8326CAA0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326CAA4: 906AC6F0  stw r3, -0x3910(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14608 as u32), ctx.r[3].u32 ) };
	// 8326CAA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326CAAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326CAB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326CAB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326CAB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326CAB8 size=56
    let mut pc: u32 = 0x8326CAB8;
    'dispatch: loop {
        match pc {
            0x8326CAB8 => {
    //   block [0x8326CAB8..0x8326CAF0)
	// 8326CAB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326CABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326CAC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326CAC4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326CAC8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326CACC: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 8326CAD0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326CAD4: 4AF87285  bl 0x821f3d58
	ctx.lr = 0x8326CAD8;
	sub_821F3D58(ctx, base);
	// 8326CAD8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326CADC: 906AC6F4  stw r3, -0x390c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14604 as u32), ctx.r[3].u32 ) };
	// 8326CAE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326CAE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326CAE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326CAEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326CAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326CAF0 size=56
    let mut pc: u32 = 0x8326CAF0;
    'dispatch: loop {
        match pc {
            0x8326CAF0 => {
    //   block [0x8326CAF0..0x8326CB28)
	// 8326CAF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326CAF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326CAF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326CAFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326CB00: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326CB04: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 8326CB08: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326CB0C: 4AF8724D  bl 0x821f3d58
	ctx.lr = 0x8326CB10;
	sub_821F3D58(ctx, base);
	// 8326CB10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326CB14: 906AC6F8  stw r3, -0x3908(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14600 as u32), ctx.r[3].u32 ) };
	// 8326CB18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326CB1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326CB20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326CB24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326CB28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326CB28 size=56
    let mut pc: u32 = 0x8326CB28;
    'dispatch: loop {
        match pc {
            0x8326CB28 => {
    //   block [0x8326CB28..0x8326CB60)
	// 8326CB28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326CB2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326CB30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326CB34: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326CB38: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326CB3C: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 8326CB40: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326CB44: 4AF87215  bl 0x821f3d58
	ctx.lr = 0x8326CB48;
	sub_821F3D58(ctx, base);
	// 8326CB48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326CB4C: 906AC6FC  stw r3, -0x3904(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14596 as u32), ctx.r[3].u32 ) };
	// 8326CB50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326CB54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326CB58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326CB5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326CB60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326CB60 size=56
    let mut pc: u32 = 0x8326CB60;
    'dispatch: loop {
        match pc {
            0x8326CB60 => {
    //   block [0x8326CB60..0x8326CB98)
	// 8326CB60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326CB64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326CB68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326CB6C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326CB70: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326CB74: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 8326CB78: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326CB7C: 4AF871DD  bl 0x821f3d58
	ctx.lr = 0x8326CB80;
	sub_821F3D58(ctx, base);
	// 8326CB80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326CB84: 906AC700  stw r3, -0x3900(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14592 as u32), ctx.r[3].u32 ) };
	// 8326CB88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326CB8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326CB90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326CB94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326CB98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326CB98 size=1340
    let mut pc: u32 = 0x8326CB98;
    'dispatch: loop {
        match pc {
            0x8326CB98 => {
    //   block [0x8326CB98..0x8326D098)
	// 8326CB98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326CB9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326CBA0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8326CBA4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326CBA8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8326CBAC: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 8326CBB0: 3BEBC708  addi r31, r11, -0x38f8
	ctx.r[31].s64 = ctx.r[11].s64 + -14584;
	// 8326CBB4: 388AA530  addi r4, r10, -0x5ad0
	ctx.r[4].s64 = ctx.r[10].s64 + -23248;
	// 8326CBB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8326CBBC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CBC0: 4AFC0311  bl 0x8222ced0
	ctx.lr = 0x8326CBC4;
	sub_8222CED0(ctx, base);
	// 8326CBC4: 3D20820E  lis r9, -0x7df2
	ctx.r[9].s64 = -2113011712;
	// 8326CBC8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CBCC: 38892668  addi r4, r9, 0x2668
	ctx.r[4].s64 = ctx.r[9].s64 + 9832;
	// 8326CBD0: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8326CBD4: 4AFC02FD  bl 0x8222ced0
	ctx.lr = 0x8326CBD8;
	sub_8222CED0(ctx, base);
	// 8326CBD8: 3D00820E  lis r8, -0x7df2
	ctx.r[8].s64 = -2113011712;
	// 8326CBDC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CBE0: 38882660  addi r4, r8, 0x2660
	ctx.r[4].s64 = ctx.r[8].s64 + 9824;
	// 8326CBE4: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 8326CBE8: 4AFC02E9  bl 0x8222ced0
	ctx.lr = 0x8326CBEC;
	sub_8222CED0(ctx, base);
	// 8326CBEC: 3CE0820E  lis r7, -0x7df2
	ctx.r[7].s64 = -2113011712;
	// 8326CBF0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CBF4: 3887265C  addi r4, r7, 0x265c
	ctx.r[4].s64 = ctx.r[7].s64 + 9820;
	// 8326CBF8: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 8326CBFC: 4AFC02D5  bl 0x8222ced0
	ctx.lr = 0x8326CC00;
	sub_8222CED0(ctx, base);
	// 8326CC00: 3CC0820E  lis r6, -0x7df2
	ctx.r[6].s64 = -2113011712;
	// 8326CC04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CC08: 38862658  addi r4, r6, 0x2658
	ctx.r[4].s64 = ctx.r[6].s64 + 9816;
	// 8326CC0C: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 8326CC10: 4AFC02C1  bl 0x8222ced0
	ctx.lr = 0x8326CC14;
	sub_8222CED0(ctx, base);
	// 8326CC14: 3C80820E  lis r4, -0x7df2
	ctx.r[4].s64 = -2113011712;
	// 8326CC18: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CC1C: 3884264C  addi r4, r4, 0x264c
	ctx.r[4].s64 = ctx.r[4].s64 + 9804;
	// 8326CC20: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 8326CC24: 4AFC02AD  bl 0x8222ced0
	ctx.lr = 0x8326CC28;
	sub_8222CED0(ctx, base);
	// 8326CC28: 3C60820E  lis r3, -0x7df2
	ctx.r[3].s64 = -2113011712;
	// 8326CC2C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CC30: 38832640  addi r4, r3, 0x2640
	ctx.r[4].s64 = ctx.r[3].s64 + 9792;
	// 8326CC34: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 8326CC38: 4AFC0299  bl 0x8222ced0
	ctx.lr = 0x8326CC3C;
	sub_8222CED0(ctx, base);
	// 8326CC3C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326CC40: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CC44: 388B2638  addi r4, r11, 0x2638
	ctx.r[4].s64 = ctx.r[11].s64 + 9784;
	// 8326CC48: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 8326CC4C: 4AFC0285  bl 0x8222ced0
	ctx.lr = 0x8326CC50;
	sub_8222CED0(ctx, base);
	// 8326CC50: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 8326CC54: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CC58: 388A2634  addi r4, r10, 0x2634
	ctx.r[4].s64 = ctx.r[10].s64 + 9780;
	// 8326CC5C: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 8326CC60: 4AFC0271  bl 0x8222ced0
	ctx.lr = 0x8326CC64;
	sub_8222CED0(ctx, base);
	// 8326CC64: 3D20820E  lis r9, -0x7df2
	ctx.r[9].s64 = -2113011712;
	// 8326CC68: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CC6C: 38892624  addi r4, r9, 0x2624
	ctx.r[4].s64 = ctx.r[9].s64 + 9764;
	// 8326CC70: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 8326CC74: 4AFC025D  bl 0x8222ced0
	ctx.lr = 0x8326CC78;
	sub_8222CED0(ctx, base);
	// 8326CC78: 3D00820E  lis r8, -0x7df2
	ctx.r[8].s64 = -2113011712;
	// 8326CC7C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CC80: 38882618  addi r4, r8, 0x2618
	ctx.r[4].s64 = ctx.r[8].s64 + 9752;
	// 8326CC84: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 8326CC88: 4AFC0249  bl 0x8222ced0
	ctx.lr = 0x8326CC8C;
	sub_8222CED0(ctx, base);
	// 8326CC8C: 3CE0820E  lis r7, -0x7df2
	ctx.r[7].s64 = -2113011712;
	// 8326CC90: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CC94: 38872610  addi r4, r7, 0x2610
	ctx.r[4].s64 = ctx.r[7].s64 + 9744;
	// 8326CC98: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 8326CC9C: 4AFC0235  bl 0x8222ced0
	ctx.lr = 0x8326CCA0;
	sub_8222CED0(ctx, base);
	// 8326CCA0: 3CC0820E  lis r6, -0x7df2
	ctx.r[6].s64 = -2113011712;
	// 8326CCA4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CCA8: 38862608  addi r4, r6, 0x2608
	ctx.r[4].s64 = ctx.r[6].s64 + 9736;
	// 8326CCAC: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 8326CCB0: 4AFC0221  bl 0x8222ced0
	ctx.lr = 0x8326CCB4;
	sub_8222CED0(ctx, base);
	// 8326CCB4: 3C80820E  lis r4, -0x7df2
	ctx.r[4].s64 = -2113011712;
	// 8326CCB8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CCBC: 388425FC  addi r4, r4, 0x25fc
	ctx.r[4].s64 = ctx.r[4].s64 + 9724;
	// 8326CCC0: 387F0034  addi r3, r31, 0x34
	ctx.r[3].s64 = ctx.r[31].s64 + 52;
	// 8326CCC4: 4AFC020D  bl 0x8222ced0
	ctx.lr = 0x8326CCC8;
	sub_8222CED0(ctx, base);
	// 8326CCC8: 3C60820E  lis r3, -0x7df2
	ctx.r[3].s64 = -2113011712;
	// 8326CCCC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CCD0: 388325F4  addi r4, r3, 0x25f4
	ctx.r[4].s64 = ctx.r[3].s64 + 9716;
	// 8326CCD4: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 8326CCD8: 4AFC01F9  bl 0x8222ced0
	ctx.lr = 0x8326CCDC;
	sub_8222CED0(ctx, base);
	// 8326CCDC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8326CCE0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CCE4: 388B3AB0  addi r4, r11, 0x3ab0
	ctx.r[4].s64 = ctx.r[11].s64 + 15024;
	// 8326CCE8: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 8326CCEC: 4AFC01E5  bl 0x8222ced0
	ctx.lr = 0x8326CCF0;
	sub_8222CED0(ctx, base);
	// 8326CCF0: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 8326CCF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CCF8: 388A25E8  addi r4, r10, 0x25e8
	ctx.r[4].s64 = ctx.r[10].s64 + 9704;
	// 8326CCFC: 387F0040  addi r3, r31, 0x40
	ctx.r[3].s64 = ctx.r[31].s64 + 64;
	// 8326CD00: 4AFC01D1  bl 0x8222ced0
	ctx.lr = 0x8326CD04;
	sub_8222CED0(ctx, base);
	// 8326CD04: 3D20820E  lis r9, -0x7df2
	ctx.r[9].s64 = -2113011712;
	// 8326CD08: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CD0C: 388908B8  addi r4, r9, 0x8b8
	ctx.r[4].s64 = ctx.r[9].s64 + 2232;
	// 8326CD10: 387F0044  addi r3, r31, 0x44
	ctx.r[3].s64 = ctx.r[31].s64 + 68;
	// 8326CD14: 4AFC01BD  bl 0x8222ced0
	ctx.lr = 0x8326CD18;
	sub_8222CED0(ctx, base);
	// 8326CD18: 3D00820E  lis r8, -0x7df2
	ctx.r[8].s64 = -2113011712;
	// 8326CD1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CD20: 388825E0  addi r4, r8, 0x25e0
	ctx.r[4].s64 = ctx.r[8].s64 + 9696;
	// 8326CD24: 387F0048  addi r3, r31, 0x48
	ctx.r[3].s64 = ctx.r[31].s64 + 72;
	// 8326CD28: 4AFC01A9  bl 0x8222ced0
	ctx.lr = 0x8326CD2C;
	sub_8222CED0(ctx, base);
	// 8326CD2C: 3CE0820E  lis r7, -0x7df2
	ctx.r[7].s64 = -2113011712;
	// 8326CD30: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CD34: 388725D0  addi r4, r7, 0x25d0
	ctx.r[4].s64 = ctx.r[7].s64 + 9680;
	// 8326CD38: 387F004C  addi r3, r31, 0x4c
	ctx.r[3].s64 = ctx.r[31].s64 + 76;
	// 8326CD3C: 4AFC0195  bl 0x8222ced0
	ctx.lr = 0x8326CD40;
	sub_8222CED0(ctx, base);
	// 8326CD40: 3CC0820E  lis r6, -0x7df2
	ctx.r[6].s64 = -2113011712;
	// 8326CD44: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CD48: 388625C4  addi r4, r6, 0x25c4
	ctx.r[4].s64 = ctx.r[6].s64 + 9668;
	// 8326CD4C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8326CD50: 4AFC0181  bl 0x8222ced0
	ctx.lr = 0x8326CD54;
	sub_8222CED0(ctx, base);
	// 8326CD54: 3C80820E  lis r4, -0x7df2
	ctx.r[4].s64 = -2113011712;
	// 8326CD58: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CD5C: 388425B8  addi r4, r4, 0x25b8
	ctx.r[4].s64 = ctx.r[4].s64 + 9656;
	// 8326CD60: 387F0054  addi r3, r31, 0x54
	ctx.r[3].s64 = ctx.r[31].s64 + 84;
	// 8326CD64: 4AFC016D  bl 0x8222ced0
	ctx.lr = 0x8326CD68;
	sub_8222CED0(ctx, base);
	// 8326CD68: 3C60820E  lis r3, -0x7df2
	ctx.r[3].s64 = -2113011712;
	// 8326CD6C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CD70: 388325B0  addi r4, r3, 0x25b0
	ctx.r[4].s64 = ctx.r[3].s64 + 9648;
	// 8326CD74: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 8326CD78: 4AFC0159  bl 0x8222ced0
	ctx.lr = 0x8326CD7C;
	sub_8222CED0(ctx, base);
	// 8326CD7C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326CD80: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CD84: 388B25A8  addi r4, r11, 0x25a8
	ctx.r[4].s64 = ctx.r[11].s64 + 9640;
	// 8326CD88: 387F005C  addi r3, r31, 0x5c
	ctx.r[3].s64 = ctx.r[31].s64 + 92;
	// 8326CD8C: 4AFC0145  bl 0x8222ced0
	ctx.lr = 0x8326CD90;
	sub_8222CED0(ctx, base);
	// 8326CD90: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 8326CD94: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CD98: 388A25A0  addi r4, r10, 0x25a0
	ctx.r[4].s64 = ctx.r[10].s64 + 9632;
	// 8326CD9C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8326CDA0: 4AFC0131  bl 0x8222ced0
	ctx.lr = 0x8326CDA4;
	sub_8222CED0(ctx, base);
	// 8326CDA4: 3D20820E  lis r9, -0x7df2
	ctx.r[9].s64 = -2113011712;
	// 8326CDA8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CDAC: 38892590  addi r4, r9, 0x2590
	ctx.r[4].s64 = ctx.r[9].s64 + 9616;
	// 8326CDB0: 387F0064  addi r3, r31, 0x64
	ctx.r[3].s64 = ctx.r[31].s64 + 100;
	// 8326CDB4: 4AFC011D  bl 0x8222ced0
	ctx.lr = 0x8326CDB8;
	sub_8222CED0(ctx, base);
	// 8326CDB8: 3D00820E  lis r8, -0x7df2
	ctx.r[8].s64 = -2113011712;
	// 8326CDBC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CDC0: 3888257C  addi r4, r8, 0x257c
	ctx.r[4].s64 = ctx.r[8].s64 + 9596;
	// 8326CDC4: 387F0068  addi r3, r31, 0x68
	ctx.r[3].s64 = ctx.r[31].s64 + 104;
	// 8326CDC8: 4AFC0109  bl 0x8222ced0
	ctx.lr = 0x8326CDCC;
	sub_8222CED0(ctx, base);
	// 8326CDCC: 3CE0820E  lis r7, -0x7df2
	ctx.r[7].s64 = -2113011712;
	// 8326CDD0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CDD4: 3887256C  addi r4, r7, 0x256c
	ctx.r[4].s64 = ctx.r[7].s64 + 9580;
	// 8326CDD8: 387F006C  addi r3, r31, 0x6c
	ctx.r[3].s64 = ctx.r[31].s64 + 108;
	// 8326CDDC: 4AFC00F5  bl 0x8222ced0
	ctx.lr = 0x8326CDE0;
	sub_8222CED0(ctx, base);
	// 8326CDE0: 3CC0820E  lis r6, -0x7df2
	ctx.r[6].s64 = -2113011712;
	// 8326CDE4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CDE8: 38862560  addi r4, r6, 0x2560
	ctx.r[4].s64 = ctx.r[6].s64 + 9568;
	// 8326CDEC: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 8326CDF0: 4AFC00E1  bl 0x8222ced0
	ctx.lr = 0x8326CDF4;
	sub_8222CED0(ctx, base);
	// 8326CDF4: 3C80820E  lis r4, -0x7df2
	ctx.r[4].s64 = -2113011712;
	// 8326CDF8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CDFC: 3884254C  addi r4, r4, 0x254c
	ctx.r[4].s64 = ctx.r[4].s64 + 9548;
	// 8326CE00: 387F0074  addi r3, r31, 0x74
	ctx.r[3].s64 = ctx.r[31].s64 + 116;
	// 8326CE04: 4AFC00CD  bl 0x8222ced0
	ctx.lr = 0x8326CE08;
	sub_8222CED0(ctx, base);
	// 8326CE08: 3C60820E  lis r3, -0x7df2
	ctx.r[3].s64 = -2113011712;
	// 8326CE0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CE10: 38832544  addi r4, r3, 0x2544
	ctx.r[4].s64 = ctx.r[3].s64 + 9540;
	// 8326CE14: 387F0078  addi r3, r31, 0x78
	ctx.r[3].s64 = ctx.r[31].s64 + 120;
	// 8326CE18: 4AFC00B9  bl 0x8222ced0
	ctx.lr = 0x8326CE1C;
	sub_8222CED0(ctx, base);
	// 8326CE1C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326CE20: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CE24: 388B253C  addi r4, r11, 0x253c
	ctx.r[4].s64 = ctx.r[11].s64 + 9532;
	// 8326CE28: 387F007C  addi r3, r31, 0x7c
	ctx.r[3].s64 = ctx.r[31].s64 + 124;
	// 8326CE2C: 4AFC00A5  bl 0x8222ced0
	ctx.lr = 0x8326CE30;
	sub_8222CED0(ctx, base);
	// 8326CE30: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 8326CE34: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CE38: 388A2530  addi r4, r10, 0x2530
	ctx.r[4].s64 = ctx.r[10].s64 + 9520;
	// 8326CE3C: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 8326CE40: 4AFC0091  bl 0x8222ced0
	ctx.lr = 0x8326CE44;
	sub_8222CED0(ctx, base);
	// 8326CE44: 3D20820E  lis r9, -0x7df2
	ctx.r[9].s64 = -2113011712;
	// 8326CE48: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CE4C: 38892524  addi r4, r9, 0x2524
	ctx.r[4].s64 = ctx.r[9].s64 + 9508;
	// 8326CE50: 387F0084  addi r3, r31, 0x84
	ctx.r[3].s64 = ctx.r[31].s64 + 132;
	// 8326CE54: 4AFC007D  bl 0x8222ced0
	ctx.lr = 0x8326CE58;
	sub_8222CED0(ctx, base);
	// 8326CE58: 3D00820E  lis r8, -0x7df2
	ctx.r[8].s64 = -2113011712;
	// 8326CE5C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CE60: 38882520  addi r4, r8, 0x2520
	ctx.r[4].s64 = ctx.r[8].s64 + 9504;
	// 8326CE64: 387F0088  addi r3, r31, 0x88
	ctx.r[3].s64 = ctx.r[31].s64 + 136;
	// 8326CE68: 4AFC0069  bl 0x8222ced0
	ctx.lr = 0x8326CE6C;
	sub_8222CED0(ctx, base);
	// 8326CE6C: 3CE0820E  lis r7, -0x7df2
	ctx.r[7].s64 = -2113011712;
	// 8326CE70: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CE74: 38872514  addi r4, r7, 0x2514
	ctx.r[4].s64 = ctx.r[7].s64 + 9492;
	// 8326CE78: 387F008C  addi r3, r31, 0x8c
	ctx.r[3].s64 = ctx.r[31].s64 + 140;
	// 8326CE7C: 4AFC0055  bl 0x8222ced0
	ctx.lr = 0x8326CE80;
	sub_8222CED0(ctx, base);
	// 8326CE80: 3CC0820E  lis r6, -0x7df2
	ctx.r[6].s64 = -2113011712;
	// 8326CE84: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CE88: 38862508  addi r4, r6, 0x2508
	ctx.r[4].s64 = ctx.r[6].s64 + 9480;
	// 8326CE8C: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 8326CE90: 4AFC0041  bl 0x8222ced0
	ctx.lr = 0x8326CE94;
	sub_8222CED0(ctx, base);
	// 8326CE94: 3C80820E  lis r4, -0x7df2
	ctx.r[4].s64 = -2113011712;
	// 8326CE98: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CE9C: 38842500  addi r4, r4, 0x2500
	ctx.r[4].s64 = ctx.r[4].s64 + 9472;
	// 8326CEA0: 387F0094  addi r3, r31, 0x94
	ctx.r[3].s64 = ctx.r[31].s64 + 148;
	// 8326CEA4: 4AFC002D  bl 0x8222ced0
	ctx.lr = 0x8326CEA8;
	sub_8222CED0(ctx, base);
	// 8326CEA8: 3C60820E  lis r3, -0x7df2
	ctx.r[3].s64 = -2113011712;
	// 8326CEAC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CEB0: 388324F0  addi r4, r3, 0x24f0
	ctx.r[4].s64 = ctx.r[3].s64 + 9456;
	// 8326CEB4: 387F0098  addi r3, r31, 0x98
	ctx.r[3].s64 = ctx.r[31].s64 + 152;
	// 8326CEB8: 4AFC0019  bl 0x8222ced0
	ctx.lr = 0x8326CEBC;
	sub_8222CED0(ctx, base);
	// 8326CEBC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326CEC0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CEC4: 388B24E8  addi r4, r11, 0x24e8
	ctx.r[4].s64 = ctx.r[11].s64 + 9448;
	// 8326CEC8: 387F009C  addi r3, r31, 0x9c
	ctx.r[3].s64 = ctx.r[31].s64 + 156;
	// 8326CECC: 4AFC0005  bl 0x8222ced0
	ctx.lr = 0x8326CED0;
	sub_8222CED0(ctx, base);
	// 8326CED0: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 8326CED4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CED8: 388A24DC  addi r4, r10, 0x24dc
	ctx.r[4].s64 = ctx.r[10].s64 + 9436;
	// 8326CEDC: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 8326CEE0: 4AFBFFF1  bl 0x8222ced0
	ctx.lr = 0x8326CEE4;
	sub_8222CED0(ctx, base);
	// 8326CEE4: 3D20820E  lis r9, -0x7df2
	ctx.r[9].s64 = -2113011712;
	// 8326CEE8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CEEC: 388924D0  addi r4, r9, 0x24d0
	ctx.r[4].s64 = ctx.r[9].s64 + 9424;
	// 8326CEF0: 387F00A4  addi r3, r31, 0xa4
	ctx.r[3].s64 = ctx.r[31].s64 + 164;
	// 8326CEF4: 4AFBFFDD  bl 0x8222ced0
	ctx.lr = 0x8326CEF8;
	sub_8222CED0(ctx, base);
	// 8326CEF8: 3D00820E  lis r8, -0x7df2
	ctx.r[8].s64 = -2113011712;
	// 8326CEFC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CF00: 388824BC  addi r4, r8, 0x24bc
	ctx.r[4].s64 = ctx.r[8].s64 + 9404;
	// 8326CF04: 387F00A8  addi r3, r31, 0xa8
	ctx.r[3].s64 = ctx.r[31].s64 + 168;
	// 8326CF08: 4AFBFFC9  bl 0x8222ced0
	ctx.lr = 0x8326CF0C;
	sub_8222CED0(ctx, base);
	// 8326CF0C: 3CE0820E  lis r7, -0x7df2
	ctx.r[7].s64 = -2113011712;
	// 8326CF10: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CF14: 388724B4  addi r4, r7, 0x24b4
	ctx.r[4].s64 = ctx.r[7].s64 + 9396;
	// 8326CF18: 387F00AC  addi r3, r31, 0xac
	ctx.r[3].s64 = ctx.r[31].s64 + 172;
	// 8326CF1C: 4AFBFFB5  bl 0x8222ced0
	ctx.lr = 0x8326CF20;
	sub_8222CED0(ctx, base);
	// 8326CF20: 3CC0820E  lis r6, -0x7df2
	ctx.r[6].s64 = -2113011712;
	// 8326CF24: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CF28: 388624A0  addi r4, r6, 0x24a0
	ctx.r[4].s64 = ctx.r[6].s64 + 9376;
	// 8326CF2C: 387F00B0  addi r3, r31, 0xb0
	ctx.r[3].s64 = ctx.r[31].s64 + 176;
	// 8326CF30: 4AFBFFA1  bl 0x8222ced0
	ctx.lr = 0x8326CF34;
	sub_8222CED0(ctx, base);
	// 8326CF34: 3C80820E  lis r4, -0x7df2
	ctx.r[4].s64 = -2113011712;
	// 8326CF38: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CF3C: 38842494  addi r4, r4, 0x2494
	ctx.r[4].s64 = ctx.r[4].s64 + 9364;
	// 8326CF40: 387F00B4  addi r3, r31, 0xb4
	ctx.r[3].s64 = ctx.r[31].s64 + 180;
	// 8326CF44: 4AFBFF8D  bl 0x8222ced0
	ctx.lr = 0x8326CF48;
	sub_8222CED0(ctx, base);
	// 8326CF48: 3C60820E  lis r3, -0x7df2
	ctx.r[3].s64 = -2113011712;
	// 8326CF4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CF50: 3883248C  addi r4, r3, 0x248c
	ctx.r[4].s64 = ctx.r[3].s64 + 9356;
	// 8326CF54: 387F00B8  addi r3, r31, 0xb8
	ctx.r[3].s64 = ctx.r[31].s64 + 184;
	// 8326CF58: 4AFBFF79  bl 0x8222ced0
	ctx.lr = 0x8326CF5C;
	sub_8222CED0(ctx, base);
	// 8326CF5C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326CF60: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CF64: 388B2484  addi r4, r11, 0x2484
	ctx.r[4].s64 = ctx.r[11].s64 + 9348;
	// 8326CF68: 387F00BC  addi r3, r31, 0xbc
	ctx.r[3].s64 = ctx.r[31].s64 + 188;
	// 8326CF6C: 4AFBFF65  bl 0x8222ced0
	ctx.lr = 0x8326CF70;
	sub_8222CED0(ctx, base);
	// 8326CF70: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 8326CF74: 388A2480  addi r4, r10, 0x2480
	ctx.r[4].s64 = ctx.r[10].s64 + 9344;
	// 8326CF78: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CF7C: 387F00C0  addi r3, r31, 0xc0
	ctx.r[3].s64 = ctx.r[31].s64 + 192;
	// 8326CF80: 4AFBFF51  bl 0x8222ced0
	ctx.lr = 0x8326CF84;
	sub_8222CED0(ctx, base);
	// 8326CF84: 3D20820E  lis r9, -0x7df2
	ctx.r[9].s64 = -2113011712;
	// 8326CF88: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CF8C: 3889247C  addi r4, r9, 0x247c
	ctx.r[4].s64 = ctx.r[9].s64 + 9340;
	// 8326CF90: 387F00C4  addi r3, r31, 0xc4
	ctx.r[3].s64 = ctx.r[31].s64 + 196;
	// 8326CF94: 4AFBFF3D  bl 0x8222ced0
	ctx.lr = 0x8326CF98;
	sub_8222CED0(ctx, base);
	// 8326CF98: 3D00820E  lis r8, -0x7df2
	ctx.r[8].s64 = -2113011712;
	// 8326CF9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CFA0: 3888246C  addi r4, r8, 0x246c
	ctx.r[4].s64 = ctx.r[8].s64 + 9324;
	// 8326CFA4: 387F00C8  addi r3, r31, 0xc8
	ctx.r[3].s64 = ctx.r[31].s64 + 200;
	// 8326CFA8: 4AFBFF29  bl 0x8222ced0
	ctx.lr = 0x8326CFAC;
	sub_8222CED0(ctx, base);
	// 8326CFAC: 3CE0820E  lis r7, -0x7df2
	ctx.r[7].s64 = -2113011712;
	// 8326CFB0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CFB4: 38872460  addi r4, r7, 0x2460
	ctx.r[4].s64 = ctx.r[7].s64 + 9312;
	// 8326CFB8: 387F00CC  addi r3, r31, 0xcc
	ctx.r[3].s64 = ctx.r[31].s64 + 204;
	// 8326CFBC: 4AFBFF15  bl 0x8222ced0
	ctx.lr = 0x8326CFC0;
	sub_8222CED0(ctx, base);
	// 8326CFC0: 3CC0820E  lis r6, -0x7df2
	ctx.r[6].s64 = -2113011712;
	// 8326CFC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CFC8: 38862454  addi r4, r6, 0x2454
	ctx.r[4].s64 = ctx.r[6].s64 + 9300;
	// 8326CFCC: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 8326CFD0: 4AFBFF01  bl 0x8222ced0
	ctx.lr = 0x8326CFD4;
	sub_8222CED0(ctx, base);
	// 8326CFD4: 3C80820E  lis r4, -0x7df2
	ctx.r[4].s64 = -2113011712;
	// 8326CFD8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CFDC: 38842444  addi r4, r4, 0x2444
	ctx.r[4].s64 = ctx.r[4].s64 + 9284;
	// 8326CFE0: 387F00D4  addi r3, r31, 0xd4
	ctx.r[3].s64 = ctx.r[31].s64 + 212;
	// 8326CFE4: 4AFBFEED  bl 0x8222ced0
	ctx.lr = 0x8326CFE8;
	sub_8222CED0(ctx, base);
	// 8326CFE8: 3C60820E  lis r3, -0x7df2
	ctx.r[3].s64 = -2113011712;
	// 8326CFEC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CFF0: 3883243C  addi r4, r3, 0x243c
	ctx.r[4].s64 = ctx.r[3].s64 + 9276;
	// 8326CFF4: 387F00D8  addi r3, r31, 0xd8
	ctx.r[3].s64 = ctx.r[31].s64 + 216;
	// 8326CFF8: 4AFBFED9  bl 0x8222ced0
	ctx.lr = 0x8326CFFC;
	sub_8222CED0(ctx, base);
	// 8326CFFC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D000: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326D004: 388B2434  addi r4, r11, 0x2434
	ctx.r[4].s64 = ctx.r[11].s64 + 9268;
	// 8326D008: 387F00DC  addi r3, r31, 0xdc
	ctx.r[3].s64 = ctx.r[31].s64 + 220;
	// 8326D00C: 4AFBFEC5  bl 0x8222ced0
	ctx.lr = 0x8326D010;
	sub_8222CED0(ctx, base);
	// 8326D010: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 8326D014: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326D018: 388A2428  addi r4, r10, 0x2428
	ctx.r[4].s64 = ctx.r[10].s64 + 9256;
	// 8326D01C: 387F00E0  addi r3, r31, 0xe0
	ctx.r[3].s64 = ctx.r[31].s64 + 224;
	// 8326D020: 4AFBFEB1  bl 0x8222ced0
	ctx.lr = 0x8326D024;
	sub_8222CED0(ctx, base);
	// 8326D024: 3D20820E  lis r9, -0x7df2
	ctx.r[9].s64 = -2113011712;
	// 8326D028: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326D02C: 38892420  addi r4, r9, 0x2420
	ctx.r[4].s64 = ctx.r[9].s64 + 9248;
	// 8326D030: 387F00E4  addi r3, r31, 0xe4
	ctx.r[3].s64 = ctx.r[31].s64 + 228;
	// 8326D034: 4AFBFE9D  bl 0x8222ced0
	ctx.lr = 0x8326D038;
	sub_8222CED0(ctx, base);
	// 8326D038: 3D00820E  lis r8, -0x7df2
	ctx.r[8].s64 = -2113011712;
	// 8326D03C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326D040: 3888240C  addi r4, r8, 0x240c
	ctx.r[4].s64 = ctx.r[8].s64 + 9228;
	// 8326D044: 387F00E8  addi r3, r31, 0xe8
	ctx.r[3].s64 = ctx.r[31].s64 + 232;
	// 8326D048: 4AFBFE89  bl 0x8222ced0
	ctx.lr = 0x8326D04C;
	sub_8222CED0(ctx, base);
	// 8326D04C: 3CE0820E  lis r7, -0x7df2
	ctx.r[7].s64 = -2113011712;
	// 8326D050: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326D054: 38872400  addi r4, r7, 0x2400
	ctx.r[4].s64 = ctx.r[7].s64 + 9216;
	// 8326D058: 387F00EC  addi r3, r31, 0xec
	ctx.r[3].s64 = ctx.r[31].s64 + 236;
	// 8326D05C: 4AFBFE75  bl 0x8222ced0
	ctx.lr = 0x8326D060;
	sub_8222CED0(ctx, base);
	// 8326D060: 3CC0820E  lis r6, -0x7df2
	ctx.r[6].s64 = -2113011712;
	// 8326D064: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326D068: 388623F4  addi r4, r6, 0x23f4
	ctx.r[4].s64 = ctx.r[6].s64 + 9204;
	// 8326D06C: 387F00F0  addi r3, r31, 0xf0
	ctx.r[3].s64 = ctx.r[31].s64 + 240;
	// 8326D070: 4AFBFE61  bl 0x8222ced0
	ctx.lr = 0x8326D074;
	sub_8222CED0(ctx, base);
	// 8326D074: 3C80820E  lis r4, -0x7df2
	ctx.r[4].s64 = -2113011712;
	// 8326D078: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326D07C: 388423E8  addi r4, r4, 0x23e8
	ctx.r[4].s64 = ctx.r[4].s64 + 9192;
	// 8326D080: 387F00F4  addi r3, r31, 0xf4
	ctx.r[3].s64 = ctx.r[31].s64 + 244;
	// 8326D084: 4AFBFE4D  bl 0x8222ced0
	ctx.lr = 0x8326D088;
	sub_8222CED0(ctx, base);
	// 8326D088: 3C608349  lis r3, -0x7cb7
	ctx.r[3].s64 = -2092367872;
	// 8326D08C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8326D090: 39037088  addi r8, r3, 0x7088
	ctx.r[8].s64 = ctx.r[3].s64 + 28808;
	// 8326D094: 917F00F8  stw r11, 0xf8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(248 as u32), ctx.r[11].u32 ) };
	pc = 0x8326D098; continue 'dispatch;
            }
            0x8326D098 => {
    //   block [0x8326D098..0x8326D0D4)
	// 8326D098: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8326D09C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8326D0A0: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8326D0A4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8326D0A8: 7D40412D  stwcx. r10, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8326D0AC: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8326D0B0: 4082FFE8  bne 0x8326d098
	if !ctx.cr[0].eq {
	pc = 0x8326D098; continue 'dispatch;
	}
	// 8326D0B4: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8326D0B8: 3867EAA0  addi r3, r7, -0x1560
	ctx.r[3].s64 = ctx.r[7].s64 + -5472;
	// 8326D0BC: 4BA3CE65  bl 0x82ca9f20
	ctx.lr = 0x8326D0C0;
	sub_82CA9F20(ctx, base);
	// 8326D0C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D0C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D0C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D0CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8326D0D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D0D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D0D8 size=56
    let mut pc: u32 = 0x8326D0D8;
    'dispatch: loop {
        match pc {
            0x8326D0D8 => {
    //   block [0x8326D0D8..0x8326D110)
	// 8326D0D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D0DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D0E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D0E4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326D0E8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D0EC: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 8326D0F0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D0F4: 4AF86C65  bl 0x821f3d58
	ctx.lr = 0x8326D0F8;
	sub_821F3D58(ctx, base);
	// 8326D0F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D0FC: 906AC804  stw r3, -0x37fc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14332 as u32), ctx.r[3].u32 ) };
	// 8326D100: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D104: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D108: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D10C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D110 size=56
    let mut pc: u32 = 0x8326D110;
    'dispatch: loop {
        match pc {
            0x8326D110 => {
    //   block [0x8326D110..0x8326D148)
	// 8326D110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D118: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D11C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326D120: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D124: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 8326D128: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D12C: 4AF86C2D  bl 0x821f3d58
	ctx.lr = 0x8326D130;
	sub_821F3D58(ctx, base);
	// 8326D130: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D134: 906AC808  stw r3, -0x37f8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14328 as u32), ctx.r[3].u32 ) };
	// 8326D138: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D13C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D140: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D144: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D148 size=56
    let mut pc: u32 = 0x8326D148;
    'dispatch: loop {
        match pc {
            0x8326D148 => {
    //   block [0x8326D148..0x8326D180)
	// 8326D148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D14C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D150: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D154: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326D158: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D15C: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 8326D160: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D164: 4AF86BF5  bl 0x821f3d58
	ctx.lr = 0x8326D168;
	sub_821F3D58(ctx, base);
	// 8326D168: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D16C: 906AC80C  stw r3, -0x37f4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14324 as u32), ctx.r[3].u32 ) };
	// 8326D170: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D174: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D178: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D17C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D180 size=56
    let mut pc: u32 = 0x8326D180;
    'dispatch: loop {
        match pc {
            0x8326D180 => {
    //   block [0x8326D180..0x8326D1B8)
	// 8326D180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D188: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D18C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326D190: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D194: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 8326D198: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D19C: 4AF86BBD  bl 0x821f3d58
	ctx.lr = 0x8326D1A0;
	sub_821F3D58(ctx, base);
	// 8326D1A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D1A4: 906AC810  stw r3, -0x37f0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14320 as u32), ctx.r[3].u32 ) };
	// 8326D1A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D1AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D1B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D1B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D1B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D1B8 size=56
    let mut pc: u32 = 0x8326D1B8;
    'dispatch: loop {
        match pc {
            0x8326D1B8 => {
    //   block [0x8326D1B8..0x8326D1F0)
	// 8326D1B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D1BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D1C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D1C4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8326D1C8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D1CC: 386B40C8  addi r3, r11, 0x40c8
	ctx.r[3].s64 = ctx.r[11].s64 + 16584;
	// 8326D1D0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D1D4: 4AF86B85  bl 0x821f3d58
	ctx.lr = 0x8326D1D8;
	sub_821F3D58(ctx, base);
	// 8326D1D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D1DC: 906AC814  stw r3, -0x37ec(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14316 as u32), ctx.r[3].u32 ) };
	// 8326D1E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D1E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D1E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D1EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D1F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D1F0 size=56
    let mut pc: u32 = 0x8326D1F0;
    'dispatch: loop {
        match pc {
            0x8326D1F0 => {
    //   block [0x8326D1F0..0x8326D228)
	// 8326D1F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D1F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D1F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D1FC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D200: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D204: 386B2A90  addi r3, r11, 0x2a90
	ctx.r[3].s64 = ctx.r[11].s64 + 10896;
	// 8326D208: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D20C: 4AF86B4D  bl 0x821f3d58
	ctx.lr = 0x8326D210;
	sub_821F3D58(ctx, base);
	// 8326D210: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D214: 906AC818  stw r3, -0x37e8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14312 as u32), ctx.r[3].u32 ) };
	// 8326D218: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D21C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D220: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D224: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D228 size=56
    let mut pc: u32 = 0x8326D228;
    'dispatch: loop {
        match pc {
            0x8326D228 => {
    //   block [0x8326D228..0x8326D260)
	// 8326D228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D22C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D230: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D234: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D238: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D23C: 386B2B0C  addi r3, r11, 0x2b0c
	ctx.r[3].s64 = ctx.r[11].s64 + 11020;
	// 8326D240: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D244: 4AF86B15  bl 0x821f3d58
	ctx.lr = 0x8326D248;
	sub_821F3D58(ctx, base);
	// 8326D248: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D24C: 906AC81C  stw r3, -0x37e4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14308 as u32), ctx.r[3].u32 ) };
	// 8326D250: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D254: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D258: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D25C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D260 size=56
    let mut pc: u32 = 0x8326D260;
    'dispatch: loop {
        match pc {
            0x8326D260 => {
    //   block [0x8326D260..0x8326D298)
	// 8326D260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D268: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D26C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D270: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D274: 386B2B20  addi r3, r11, 0x2b20
	ctx.r[3].s64 = ctx.r[11].s64 + 11040;
	// 8326D278: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D27C: 4AF86ADD  bl 0x821f3d58
	ctx.lr = 0x8326D280;
	sub_821F3D58(ctx, base);
	// 8326D280: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D284: 906AC820  stw r3, -0x37e0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14304 as u32), ctx.r[3].u32 ) };
	// 8326D288: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D28C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D290: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D294: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D298 size=56
    let mut pc: u32 = 0x8326D298;
    'dispatch: loop {
        match pc {
            0x8326D298 => {
    //   block [0x8326D298..0x8326D2D0)
	// 8326D298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D29C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D2A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D2A4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326D2A8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D2AC: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 8326D2B0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D2B4: 4AF86AA5  bl 0x821f3d58
	ctx.lr = 0x8326D2B8;
	sub_821F3D58(ctx, base);
	// 8326D2B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D2BC: 906AC824  stw r3, -0x37dc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14300 as u32), ctx.r[3].u32 ) };
	// 8326D2C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D2C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D2C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D2CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D2D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D2D0 size=56
    let mut pc: u32 = 0x8326D2D0;
    'dispatch: loop {
        match pc {
            0x8326D2D0 => {
    //   block [0x8326D2D0..0x8326D308)
	// 8326D2D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D2D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D2D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D2DC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326D2E0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D2E4: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 8326D2E8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D2EC: 4AF86A6D  bl 0x821f3d58
	ctx.lr = 0x8326D2F0;
	sub_821F3D58(ctx, base);
	// 8326D2F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D2F4: 906AC828  stw r3, -0x37d8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14296 as u32), ctx.r[3].u32 ) };
	// 8326D2F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D2FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D300: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D304: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D308 size=56
    let mut pc: u32 = 0x8326D308;
    'dispatch: loop {
        match pc {
            0x8326D308 => {
    //   block [0x8326D308..0x8326D340)
	// 8326D308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D30C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D310: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D314: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326D318: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D31C: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 8326D320: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D324: 4AF86A35  bl 0x821f3d58
	ctx.lr = 0x8326D328;
	sub_821F3D58(ctx, base);
	// 8326D328: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D32C: 906AC82C  stw r3, -0x37d4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14292 as u32), ctx.r[3].u32 ) };
	// 8326D330: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D334: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D338: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D33C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D340 size=56
    let mut pc: u32 = 0x8326D340;
    'dispatch: loop {
        match pc {
            0x8326D340 => {
    //   block [0x8326D340..0x8326D378)
	// 8326D340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D348: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D34C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D350: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D354: 386B2B2C  addi r3, r11, 0x2b2c
	ctx.r[3].s64 = ctx.r[11].s64 + 11052;
	// 8326D358: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D35C: 4AF869FD  bl 0x821f3d58
	ctx.lr = 0x8326D360;
	sub_821F3D58(ctx, base);
	// 8326D360: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D364: 906AC830  stw r3, -0x37d0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14288 as u32), ctx.r[3].u32 ) };
	// 8326D368: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D36C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D370: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D374: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D378 size=56
    let mut pc: u32 = 0x8326D378;
    'dispatch: loop {
        match pc {
            0x8326D378 => {
    //   block [0x8326D378..0x8326D3B0)
	// 8326D378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D37C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D380: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D384: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D388: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D38C: 386B2B40  addi r3, r11, 0x2b40
	ctx.r[3].s64 = ctx.r[11].s64 + 11072;
	// 8326D390: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D394: 4AF869C5  bl 0x821f3d58
	ctx.lr = 0x8326D398;
	sub_821F3D58(ctx, base);
	// 8326D398: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D39C: 906AC834  stw r3, -0x37cc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14284 as u32), ctx.r[3].u32 ) };
	// 8326D3A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D3A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D3A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D3AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D3B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D3B0 size=56
    let mut pc: u32 = 0x8326D3B0;
    'dispatch: loop {
        match pc {
            0x8326D3B0 => {
    //   block [0x8326D3B0..0x8326D3E8)
	// 8326D3B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D3B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D3B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D3BC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D3C0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D3C4: 386B2B54  addi r3, r11, 0x2b54
	ctx.r[3].s64 = ctx.r[11].s64 + 11092;
	// 8326D3C8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D3CC: 4AF8698D  bl 0x821f3d58
	ctx.lr = 0x8326D3D0;
	sub_821F3D58(ctx, base);
	// 8326D3D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D3D4: 906AC838  stw r3, -0x37c8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14280 as u32), ctx.r[3].u32 ) };
	// 8326D3D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D3DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D3E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D3E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D3E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D3E8 size=56
    let mut pc: u32 = 0x8326D3E8;
    'dispatch: loop {
        match pc {
            0x8326D3E8 => {
    //   block [0x8326D3E8..0x8326D420)
	// 8326D3E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D3EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D3F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D3F4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D3F8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D3FC: 386B2B68  addi r3, r11, 0x2b68
	ctx.r[3].s64 = ctx.r[11].s64 + 11112;
	// 8326D400: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D404: 4AF86955  bl 0x821f3d58
	ctx.lr = 0x8326D408;
	sub_821F3D58(ctx, base);
	// 8326D408: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D40C: 906AC83C  stw r3, -0x37c4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14276 as u32), ctx.r[3].u32 ) };
	// 8326D410: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D414: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D418: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D41C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D420 size=56
    let mut pc: u32 = 0x8326D420;
    'dispatch: loop {
        match pc {
            0x8326D420 => {
    //   block [0x8326D420..0x8326D458)
	// 8326D420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D428: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D42C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D430: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D434: 386B2B7C  addi r3, r11, 0x2b7c
	ctx.r[3].s64 = ctx.r[11].s64 + 11132;
	// 8326D438: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D43C: 4AF8691D  bl 0x821f3d58
	ctx.lr = 0x8326D440;
	sub_821F3D58(ctx, base);
	// 8326D440: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D444: 906AC840  stw r3, -0x37c0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14272 as u32), ctx.r[3].u32 ) };
	// 8326D448: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D44C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D450: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D454: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D458 size=64
    let mut pc: u32 = 0x8326D458;
    'dispatch: loop {
        match pc {
            0x8326D458 => {
    //   block [0x8326D458..0x8326D498)
	// 8326D458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D45C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D460: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D464: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D468: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D46C: 388B30C8  addi r4, r11, 0x30c8
	ctx.r[4].s64 = ctx.r[11].s64 + 12488;
	// 8326D470: 386AC844  addi r3, r10, -0x37bc
	ctx.r[3].s64 = ctx.r[10].s64 + -14268;
	// 8326D474: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326D478: 4AFBFA59  bl 0x8222ced0
	ctx.lr = 0x8326D47C;
	sub_8222CED0(ctx, base);
	// 8326D47C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326D480: 3869EC50  addi r3, r9, -0x13b0
	ctx.r[3].s64 = ctx.r[9].s64 + -5040;
	// 8326D484: 4BA3CA9D  bl 0x82ca9f20
	ctx.lr = 0x8326D488;
	sub_82CA9F20(ctx, base);
	// 8326D488: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D48C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D490: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D494: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D498 size=64
    let mut pc: u32 = 0x8326D498;
    'dispatch: loop {
        match pc {
            0x8326D498 => {
    //   block [0x8326D498..0x8326D4D8)
	// 8326D498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D49C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D4A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D4A4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D4A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D4AC: 388B30D8  addi r4, r11, 0x30d8
	ctx.r[4].s64 = ctx.r[11].s64 + 12504;
	// 8326D4B0: 386AC848  addi r3, r10, -0x37b8
	ctx.r[3].s64 = ctx.r[10].s64 + -14264;
	// 8326D4B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326D4B8: 4AFBFA19  bl 0x8222ced0
	ctx.lr = 0x8326D4BC;
	sub_8222CED0(ctx, base);
	// 8326D4BC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326D4C0: 3869EC60  addi r3, r9, -0x13a0
	ctx.r[3].s64 = ctx.r[9].s64 + -5024;
	// 8326D4C4: 4BA3CA5D  bl 0x82ca9f20
	ctx.lr = 0x8326D4C8;
	sub_82CA9F20(ctx, base);
	// 8326D4C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D4CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D4D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D4D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D4D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D4D8 size=64
    let mut pc: u32 = 0x8326D4D8;
    'dispatch: loop {
        match pc {
            0x8326D4D8 => {
    //   block [0x8326D4D8..0x8326D518)
	// 8326D4D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D4DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D4E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D4E4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D4E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D4EC: 388B30F4  addi r4, r11, 0x30f4
	ctx.r[4].s64 = ctx.r[11].s64 + 12532;
	// 8326D4F0: 386AC84C  addi r3, r10, -0x37b4
	ctx.r[3].s64 = ctx.r[10].s64 + -14260;
	// 8326D4F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326D4F8: 4AFBF9D9  bl 0x8222ced0
	ctx.lr = 0x8326D4FC;
	sub_8222CED0(ctx, base);
	// 8326D4FC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326D500: 3869EC70  addi r3, r9, -0x1390
	ctx.r[3].s64 = ctx.r[9].s64 + -5008;
	// 8326D504: 4BA3CA1D  bl 0x82ca9f20
	ctx.lr = 0x8326D508;
	sub_82CA9F20(ctx, base);
	// 8326D508: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D50C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D510: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D514: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D518 size=64
    let mut pc: u32 = 0x8326D518;
    'dispatch: loop {
        match pc {
            0x8326D518 => {
    //   block [0x8326D518..0x8326D558)
	// 8326D518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D51C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D520: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D524: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D528: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D52C: 388B310C  addi r4, r11, 0x310c
	ctx.r[4].s64 = ctx.r[11].s64 + 12556;
	// 8326D530: 386AC850  addi r3, r10, -0x37b0
	ctx.r[3].s64 = ctx.r[10].s64 + -14256;
	// 8326D534: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326D538: 4AFBF999  bl 0x8222ced0
	ctx.lr = 0x8326D53C;
	sub_8222CED0(ctx, base);
	// 8326D53C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326D540: 3869EC80  addi r3, r9, -0x1380
	ctx.r[3].s64 = ctx.r[9].s64 + -4992;
	// 8326D544: 4BA3C9DD  bl 0x82ca9f20
	ctx.lr = 0x8326D548;
	sub_82CA9F20(ctx, base);
	// 8326D548: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D54C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D550: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D554: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D558 size=64
    let mut pc: u32 = 0x8326D558;
    'dispatch: loop {
        match pc {
            0x8326D558 => {
    //   block [0x8326D558..0x8326D598)
	// 8326D558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D55C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D560: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D564: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D568: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D56C: 388B3128  addi r4, r11, 0x3128
	ctx.r[4].s64 = ctx.r[11].s64 + 12584;
	// 8326D570: 386AC854  addi r3, r10, -0x37ac
	ctx.r[3].s64 = ctx.r[10].s64 + -14252;
	// 8326D574: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326D578: 4AFBF959  bl 0x8222ced0
	ctx.lr = 0x8326D57C;
	sub_8222CED0(ctx, base);
	// 8326D57C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326D580: 3869EC90  addi r3, r9, -0x1370
	ctx.r[3].s64 = ctx.r[9].s64 + -4976;
	// 8326D584: 4BA3C99D  bl 0x82ca9f20
	ctx.lr = 0x8326D588;
	sub_82CA9F20(ctx, base);
	// 8326D588: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D58C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D590: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D594: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D598 size=64
    let mut pc: u32 = 0x8326D598;
    'dispatch: loop {
        match pc {
            0x8326D598 => {
    //   block [0x8326D598..0x8326D5D8)
	// 8326D598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D59C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D5A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D5A4: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8326D5A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D5AC: 388BC484  addi r4, r11, -0x3b7c
	ctx.r[4].s64 = ctx.r[11].s64 + -15228;
	// 8326D5B0: 386AC858  addi r3, r10, -0x37a8
	ctx.r[3].s64 = ctx.r[10].s64 + -14248;
	// 8326D5B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326D5B8: 4AFBF919  bl 0x8222ced0
	ctx.lr = 0x8326D5BC;
	sub_8222CED0(ctx, base);
	// 8326D5BC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326D5C0: 3869ECA0  addi r3, r9, -0x1360
	ctx.r[3].s64 = ctx.r[9].s64 + -4960;
	// 8326D5C4: 4BA3C95D  bl 0x82ca9f20
	ctx.lr = 0x8326D5C8;
	sub_82CA9F20(ctx, base);
	// 8326D5C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D5CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D5D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D5D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D5D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D5D8 size=64
    let mut pc: u32 = 0x8326D5D8;
    'dispatch: loop {
        match pc {
            0x8326D5D8 => {
    //   block [0x8326D5D8..0x8326D618)
	// 8326D5D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D5DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D5E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D5E4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D5E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D5EC: 388B118C  addi r4, r11, 0x118c
	ctx.r[4].s64 = ctx.r[11].s64 + 4492;
	// 8326D5F0: 386AC85C  addi r3, r10, -0x37a4
	ctx.r[3].s64 = ctx.r[10].s64 + -14244;
	// 8326D5F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326D5F8: 4AFBF8D9  bl 0x8222ced0
	ctx.lr = 0x8326D5FC;
	sub_8222CED0(ctx, base);
	// 8326D5FC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326D600: 3869ECB0  addi r3, r9, -0x1350
	ctx.r[3].s64 = ctx.r[9].s64 + -4944;
	// 8326D604: 4BA3C91D  bl 0x82ca9f20
	ctx.lr = 0x8326D608;
	sub_82CA9F20(ctx, base);
	// 8326D608: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D60C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D610: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D614: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D618 size=64
    let mut pc: u32 = 0x8326D618;
    'dispatch: loop {
        match pc {
            0x8326D618 => {
    //   block [0x8326D618..0x8326D658)
	// 8326D618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D61C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D620: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D624: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8326D628: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D62C: 388BC4AC  addi r4, r11, -0x3b54
	ctx.r[4].s64 = ctx.r[11].s64 + -15188;
	// 8326D630: 386AC860  addi r3, r10, -0x37a0
	ctx.r[3].s64 = ctx.r[10].s64 + -14240;
	// 8326D634: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326D638: 4AFBF899  bl 0x8222ced0
	ctx.lr = 0x8326D63C;
	sub_8222CED0(ctx, base);
	// 8326D63C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326D640: 3869ECC0  addi r3, r9, -0x1340
	ctx.r[3].s64 = ctx.r[9].s64 + -4928;
	// 8326D644: 4BA3C8DD  bl 0x82ca9f20
	ctx.lr = 0x8326D648;
	sub_82CA9F20(ctx, base);
	// 8326D648: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D64C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D650: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D654: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D658 size=144
    let mut pc: u32 = 0x8326D658;
    'dispatch: loop {
        match pc {
            0x8326D658 => {
    //   block [0x8326D658..0x8326D67C)
	// 8326D658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D65C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D660: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D664: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 8326D668: 4AFB1BF1  bl 0x8221f258
	ctx.lr = 0x8326D66C;
	sub_8221F258(ctx, base);
	// 8326D66C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8326D670: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8326D674: 419A0008  beq cr6, 0x8326d67c
	if ctx.cr[6].eq {
	pc = 0x8326D67C; continue 'dispatch;
	}
	// 8326D678: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8326D67C; continue 'dispatch;
            }
            0x8326D67C => {
    //   block [0x8326D67C..0x8326D688)
	// 8326D67C: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8326D680: 41820008  beq 0x8326d688
	if ctx.cr[0].eq {
	pc = 0x8326D688; continue 'dispatch;
	}
	// 8326D684: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8326D688; continue 'dispatch;
            }
            0x8326D688 => {
    //   block [0x8326D688..0x8326D694)
	// 8326D688: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8326D68C: 41820008  beq 0x8326d694
	if ctx.cr[0].eq {
	pc = 0x8326D694; continue 'dispatch;
	}
	// 8326D690: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8326D694; continue 'dispatch;
            }
            0x8326D694 => {
    //   block [0x8326D694..0x8326D6E8)
	// 8326D694: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8326D698: 99430029  stb r10, 0x29(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(41 as u32), ctx.r[10].u8 ) };
	// 8326D69C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8326D6A0: 3909C864  addi r8, r9, -0x379c
	ctx.r[8].s64 = ctx.r[9].s64 + -14236;
	// 8326D6A4: 99630028  stb r11, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u8 ) };
	// 8326D6A8: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8326D6AC: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8326D6B0: 99630029  stb r11, 0x29(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(41 as u32), ctx.r[11].u8 ) };
	// 8326D6B4: 3867ECD0  addi r3, r7, -0x1330
	ctx.r[3].s64 = ctx.r[7].s64 + -4912;
	// 8326D6B8: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8326D6BC: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8326D6C0: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8326D6C4: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8326D6C8: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8326D6CC: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8326D6D0: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8326D6D4: 4BA3C84D  bl 0x82ca9f20
	ctx.lr = 0x8326D6D8;
	sub_82CA9F20(ctx, base);
	// 8326D6D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D6DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D6E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D6E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D6E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D6E8 size=144
    let mut pc: u32 = 0x8326D6E8;
    'dispatch: loop {
        match pc {
            0x8326D6E8 => {
    //   block [0x8326D6E8..0x8326D70C)
	// 8326D6E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D6EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D6F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D6F4: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 8326D6F8: 4AFB1B61  bl 0x8221f258
	ctx.lr = 0x8326D6FC;
	sub_8221F258(ctx, base);
	// 8326D6FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8326D700: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8326D704: 419A0008  beq cr6, 0x8326d70c
	if ctx.cr[6].eq {
	pc = 0x8326D70C; continue 'dispatch;
	}
	// 8326D708: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8326D70C; continue 'dispatch;
            }
            0x8326D70C => {
    //   block [0x8326D70C..0x8326D718)
	// 8326D70C: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8326D710: 41820008  beq 0x8326d718
	if ctx.cr[0].eq {
	pc = 0x8326D718; continue 'dispatch;
	}
	// 8326D714: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8326D718; continue 'dispatch;
            }
            0x8326D718 => {
    //   block [0x8326D718..0x8326D724)
	// 8326D718: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8326D71C: 41820008  beq 0x8326d724
	if ctx.cr[0].eq {
	pc = 0x8326D724; continue 'dispatch;
	}
	// 8326D720: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8326D724; continue 'dispatch;
            }
            0x8326D724 => {
    //   block [0x8326D724..0x8326D778)
	// 8326D724: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8326D728: 99430021  stb r10, 0x21(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(33 as u32), ctx.r[10].u8 ) };
	// 8326D72C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8326D730: 3909C870  addi r8, r9, -0x3790
	ctx.r[8].s64 = ctx.r[9].s64 + -14224;
	// 8326D734: 99630020  stb r11, 0x20(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u8 ) };
	// 8326D738: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8326D73C: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8326D740: 99630021  stb r11, 0x21(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(33 as u32), ctx.r[11].u8 ) };
	// 8326D744: 3867ECE0  addi r3, r7, -0x1320
	ctx.r[3].s64 = ctx.r[7].s64 + -4896;
	// 8326D748: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8326D74C: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8326D750: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8326D754: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8326D758: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8326D75C: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8326D760: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8326D764: 4BA3C7BD  bl 0x82ca9f20
	ctx.lr = 0x8326D768;
	sub_82CA9F20(ctx, base);
	// 8326D768: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D76C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D770: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D774: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8326D778 size=12
    let mut pc: u32 = 0x8326D778;
    'dispatch: loop {
        match pc {
            0x8326D778 => {
    //   block [0x8326D778..0x8326D784)
	// 8326D778: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8326D77C: 386BECF0  addi r3, r11, -0x1310
	ctx.r[3].s64 = ctx.r[11].s64 + -4880;
	// 8326D780: 4BA3C7A0  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D788 size=56
    let mut pc: u32 = 0x8326D788;
    'dispatch: loop {
        match pc {
            0x8326D788 => {
    //   block [0x8326D788..0x8326D7C0)
	// 8326D788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D78C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D790: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D794: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D798: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D79C: 386B4024  addi r3, r11, 0x4024
	ctx.r[3].s64 = ctx.r[11].s64 + 16420;
	// 8326D7A0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D7A4: 4AF865B5  bl 0x821f3d58
	ctx.lr = 0x8326D7A8;
	sub_821F3D58(ctx, base);
	// 8326D7A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D7AC: 906AC88C  stw r3, -0x3774(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14196 as u32), ctx.r[3].u32 ) };
	// 8326D7B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D7B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D7B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D7BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D7C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D7C0 size=56
    let mut pc: u32 = 0x8326D7C0;
    'dispatch: loop {
        match pc {
            0x8326D7C0 => {
    //   block [0x8326D7C0..0x8326D7F8)
	// 8326D7C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D7C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D7C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D7CC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D7D0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D7D4: 386B4034  addi r3, r11, 0x4034
	ctx.r[3].s64 = ctx.r[11].s64 + 16436;
	// 8326D7D8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D7DC: 4AF8657D  bl 0x821f3d58
	ctx.lr = 0x8326D7E0;
	sub_821F3D58(ctx, base);
	// 8326D7E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D7E4: 906AC890  stw r3, -0x3770(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14192 as u32), ctx.r[3].u32 ) };
	// 8326D7E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D7EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D7F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D7F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D7F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D7F8 size=56
    let mut pc: u32 = 0x8326D7F8;
    'dispatch: loop {
        match pc {
            0x8326D7F8 => {
    //   block [0x8326D7F8..0x8326D830)
	// 8326D7F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D7FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D800: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D804: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D808: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D80C: 386B4048  addi r3, r11, 0x4048
	ctx.r[3].s64 = ctx.r[11].s64 + 16456;
	// 8326D810: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D814: 4AF86545  bl 0x821f3d58
	ctx.lr = 0x8326D818;
	sub_821F3D58(ctx, base);
	// 8326D818: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D81C: 906AC894  stw r3, -0x376c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14188 as u32), ctx.r[3].u32 ) };
	// 8326D820: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D824: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D828: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D82C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D830 size=56
    let mut pc: u32 = 0x8326D830;
    'dispatch: loop {
        match pc {
            0x8326D830 => {
    //   block [0x8326D830..0x8326D868)
	// 8326D830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D838: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D83C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D840: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D844: 386B405C  addi r3, r11, 0x405c
	ctx.r[3].s64 = ctx.r[11].s64 + 16476;
	// 8326D848: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D84C: 4AF8650D  bl 0x821f3d58
	ctx.lr = 0x8326D850;
	sub_821F3D58(ctx, base);
	// 8326D850: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D854: 906AC898  stw r3, -0x3768(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14184 as u32), ctx.r[3].u32 ) };
	// 8326D858: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D85C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D860: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D864: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D868 size=56
    let mut pc: u32 = 0x8326D868;
    'dispatch: loop {
        match pc {
            0x8326D868 => {
    //   block [0x8326D868..0x8326D8A0)
	// 8326D868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D86C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D870: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D874: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D878: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D87C: 386B406C  addi r3, r11, 0x406c
	ctx.r[3].s64 = ctx.r[11].s64 + 16492;
	// 8326D880: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D884: 4AF864D5  bl 0x821f3d58
	ctx.lr = 0x8326D888;
	sub_821F3D58(ctx, base);
	// 8326D888: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D88C: 906AC89C  stw r3, -0x3764(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14180 as u32), ctx.r[3].u32 ) };
	// 8326D890: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D894: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D898: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D89C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D8A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D8A0 size=56
    let mut pc: u32 = 0x8326D8A0;
    'dispatch: loop {
        match pc {
            0x8326D8A0 => {
    //   block [0x8326D8A0..0x8326D8D8)
	// 8326D8A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D8A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D8A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D8AC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D8B0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D8B4: 386B407C  addi r3, r11, 0x407c
	ctx.r[3].s64 = ctx.r[11].s64 + 16508;
	// 8326D8B8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D8BC: 4AF8649D  bl 0x821f3d58
	ctx.lr = 0x8326D8C0;
	sub_821F3D58(ctx, base);
	// 8326D8C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D8C4: 906AC8A0  stw r3, -0x3760(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14176 as u32), ctx.r[3].u32 ) };
	// 8326D8C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D8CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D8D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D8D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D8D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D8D8 size=56
    let mut pc: u32 = 0x8326D8D8;
    'dispatch: loop {
        match pc {
            0x8326D8D8 => {
    //   block [0x8326D8D8..0x8326D910)
	// 8326D8D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D8DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D8E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D8E4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D8E8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D8EC: 386B408C  addi r3, r11, 0x408c
	ctx.r[3].s64 = ctx.r[11].s64 + 16524;
	// 8326D8F0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D8F4: 4AF86465  bl 0x821f3d58
	ctx.lr = 0x8326D8F8;
	sub_821F3D58(ctx, base);
	// 8326D8F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D8FC: 906AC8A4  stw r3, -0x375c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14172 as u32), ctx.r[3].u32 ) };
	// 8326D900: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D904: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D908: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D90C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D910 size=56
    let mut pc: u32 = 0x8326D910;
    'dispatch: loop {
        match pc {
            0x8326D910 => {
    //   block [0x8326D910..0x8326D948)
	// 8326D910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D918: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D91C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D920: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D924: 386B2B20  addi r3, r11, 0x2b20
	ctx.r[3].s64 = ctx.r[11].s64 + 11040;
	// 8326D928: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D92C: 4AF8642D  bl 0x821f3d58
	ctx.lr = 0x8326D930;
	sub_821F3D58(ctx, base);
	// 8326D930: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D934: 906AC8A8  stw r3, -0x3758(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14168 as u32), ctx.r[3].u32 ) };
	// 8326D938: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D93C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D940: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D944: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D948 size=56
    let mut pc: u32 = 0x8326D948;
    'dispatch: loop {
        match pc {
            0x8326D948 => {
    //   block [0x8326D948..0x8326D980)
	// 8326D948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D94C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D950: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D954: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D958: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D95C: 386B2B2C  addi r3, r11, 0x2b2c
	ctx.r[3].s64 = ctx.r[11].s64 + 11052;
	// 8326D960: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D964: 4AF863F5  bl 0x821f3d58
	ctx.lr = 0x8326D968;
	sub_821F3D58(ctx, base);
	// 8326D968: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D96C: 906AC8AC  stw r3, -0x3754(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14164 as u32), ctx.r[3].u32 ) };
	// 8326D970: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D974: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D978: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D97C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D980 size=56
    let mut pc: u32 = 0x8326D980;
    'dispatch: loop {
        match pc {
            0x8326D980 => {
    //   block [0x8326D980..0x8326D9B8)
	// 8326D980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D988: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D98C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D990: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D994: 386B40A0  addi r3, r11, 0x40a0
	ctx.r[3].s64 = ctx.r[11].s64 + 16544;
	// 8326D998: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D99C: 4AF863BD  bl 0x821f3d58
	ctx.lr = 0x8326D9A0;
	sub_821F3D58(ctx, base);
	// 8326D9A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D9A4: 906AC8B0  stw r3, -0x3750(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14160 as u32), ctx.r[3].u32 ) };
	// 8326D9A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D9AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D9B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D9B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D9B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D9B8 size=56
    let mut pc: u32 = 0x8326D9B8;
    'dispatch: loop {
        match pc {
            0x8326D9B8 => {
    //   block [0x8326D9B8..0x8326D9F0)
	// 8326D9B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D9BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D9C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D9C4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D9C8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D9CC: 386B40B8  addi r3, r11, 0x40b8
	ctx.r[3].s64 = ctx.r[11].s64 + 16568;
	// 8326D9D0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D9D4: 4AF86385  bl 0x821f3d58
	ctx.lr = 0x8326D9D8;
	sub_821F3D58(ctx, base);
	// 8326D9D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D9DC: 906AC8B4  stw r3, -0x374c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14156 as u32), ctx.r[3].u32 ) };
	// 8326D9E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D9E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D9E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D9EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D9F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D9F0 size=56
    let mut pc: u32 = 0x8326D9F0;
    'dispatch: loop {
        match pc {
            0x8326D9F0 => {
    //   block [0x8326D9F0..0x8326DA28)
	// 8326D9F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D9F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D9F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D9FC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326DA00: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326DA04: 386B40C8  addi r3, r11, 0x40c8
	ctx.r[3].s64 = ctx.r[11].s64 + 16584;
	// 8326DA08: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326DA0C: 4AF8634D  bl 0x821f3d58
	ctx.lr = 0x8326DA10;
	sub_821F3D58(ctx, base);
	// 8326DA10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326DA14: 906AC8B8  stw r3, -0x3748(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14152 as u32), ctx.r[3].u32 ) };
	// 8326DA18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326DA1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326DA20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326DA24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326DA28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326DA28 size=56
    let mut pc: u32 = 0x8326DA28;
    'dispatch: loop {
        match pc {
            0x8326DA28 => {
    //   block [0x8326DA28..0x8326DA60)
	// 8326DA28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326DA2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326DA30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326DA34: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326DA38: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326DA3C: 386B40DC  addi r3, r11, 0x40dc
	ctx.r[3].s64 = ctx.r[11].s64 + 16604;
	// 8326DA40: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326DA44: 4AF86315  bl 0x821f3d58
	ctx.lr = 0x8326DA48;
	sub_821F3D58(ctx, base);
	// 8326DA48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326DA4C: 906AC8BC  stw r3, -0x3744(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14148 as u32), ctx.r[3].u32 ) };
	// 8326DA50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326DA54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326DA58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326DA5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326DA60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326DA60 size=56
    let mut pc: u32 = 0x8326DA60;
    'dispatch: loop {
        match pc {
            0x8326DA60 => {
    //   block [0x8326DA60..0x8326DA98)
	// 8326DA60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326DA64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326DA68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326DA6C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326DA70: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326DA74: 386B40F4  addi r3, r11, 0x40f4
	ctx.r[3].s64 = ctx.r[11].s64 + 16628;
	// 8326DA78: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326DA7C: 4AF862DD  bl 0x821f3d58
	ctx.lr = 0x8326DA80;
	sub_821F3D58(ctx, base);
	// 8326DA80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326DA84: 906AC8C0  stw r3, -0x3740(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14144 as u32), ctx.r[3].u32 ) };
	// 8326DA88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326DA8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326DA90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326DA94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326DA98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326DA98 size=56
    let mut pc: u32 = 0x8326DA98;
    'dispatch: loop {
        match pc {
            0x8326DA98 => {
    //   block [0x8326DA98..0x8326DAD0)
	// 8326DA98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326DA9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326DAA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326DAA4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326DAA8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326DAAC: 386B410C  addi r3, r11, 0x410c
	ctx.r[3].s64 = ctx.r[11].s64 + 16652;
	// 8326DAB0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326DAB4: 4AF862A5  bl 0x821f3d58
	ctx.lr = 0x8326DAB8;
	sub_821F3D58(ctx, base);
	// 8326DAB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326DABC: 906AC8C4  stw r3, -0x373c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14140 as u32), ctx.r[3].u32 ) };
	// 8326DAC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326DAC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326DAC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326DACC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326DAD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326DAD0 size=56
    let mut pc: u32 = 0x8326DAD0;
    'dispatch: loop {
        match pc {
            0x8326DAD0 => {
    //   block [0x8326DAD0..0x8326DB08)
	// 8326DAD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326DAD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326DAD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326DADC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326DAE0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326DAE4: 386B4128  addi r3, r11, 0x4128
	ctx.r[3].s64 = ctx.r[11].s64 + 16680;
	// 8326DAE8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326DAEC: 4AF8626D  bl 0x821f3d58
	ctx.lr = 0x8326DAF0;
	sub_821F3D58(ctx, base);
	// 8326DAF0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326DAF4: 906AC8C8  stw r3, -0x3738(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14136 as u32), ctx.r[3].u32 ) };
	// 8326DAF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326DAFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326DB00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326DB04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326DB08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326DB08 size=56
    let mut pc: u32 = 0x8326DB08;
    'dispatch: loop {
        match pc {
            0x8326DB08 => {
    //   block [0x8326DB08..0x8326DB40)
	// 8326DB08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326DB0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326DB10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326DB14: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326DB18: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326DB1C: 386B4144  addi r3, r11, 0x4144
	ctx.r[3].s64 = ctx.r[11].s64 + 16708;
	// 8326DB20: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326DB24: 4AF86235  bl 0x821f3d58
	ctx.lr = 0x8326DB28;
	sub_821F3D58(ctx, base);
	// 8326DB28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326DB2C: 906AC8CC  stw r3, -0x3734(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14132 as u32), ctx.r[3].u32 ) };
	// 8326DB30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326DB34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326DB38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326DB3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326DB40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326DB40 size=56
    let mut pc: u32 = 0x8326DB40;
    'dispatch: loop {
        match pc {
            0x8326DB40 => {
    //   block [0x8326DB40..0x8326DB78)
	// 8326DB40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326DB44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326DB48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326DB4C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326DB50: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326DB54: 386B4154  addi r3, r11, 0x4154
	ctx.r[3].s64 = ctx.r[11].s64 + 16724;
	// 8326DB58: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326DB5C: 4AF861FD  bl 0x821f3d58
	ctx.lr = 0x8326DB60;
	sub_821F3D58(ctx, base);
	// 8326DB60: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326DB64: 906AC8D0  stw r3, -0x3730(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14128 as u32), ctx.r[3].u32 ) };
	// 8326DB68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326DB6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326DB70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326DB74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326DB78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8326DB78 size=76
    let mut pc: u32 = 0x8326DB78;
    'dispatch: loop {
        match pc {
            0x8326DB78 => {
    //   block [0x8326DB78..0x8326DBC4)
	// 8326DB78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326DB7C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8326DB80: 3D00834A  lis r8, -0x7cb6
	ctx.r[8].s64 = -2092302336;
	// 8326DB84: 3CE0834A  lis r7, -0x7cb6
	ctx.r[7].s64 = -2092302336;
	// 8326DB88: 3CC0834A  lis r6, -0x7cb6
	ctx.r[6].s64 = -2092302336;
	// 8326DB8C: 812AC8B4  lwz r9, -0x374c(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-14156 as u32) ) } as u64;
	// 8326DB90: 816BC89C  lwz r11, -0x3764(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-14180 as u32) ) } as u64;
	// 8326DB94: 38A6C8D4  addi r5, r6, -0x372c
	ctx.r[5].s64 = ctx.r[6].s64 + -14124;
	// 8326DB98: 8108C8B8  lwz r8, -0x3748(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-14152 as u32) ) } as u64;
	// 8326DB9C: 8147C8A4  lwz r10, -0x375c(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(-14172 as u32) ) } as u64;
	// 8326DBA0: 9166C8D4  stw r11, -0x372c(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(-14124 as u32), ctx.r[11].u32 ) };
	// 8326DBA4: 91250004  stw r9, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8326DBA8: 91650008  stw r11, 8(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8326DBAC: 9105000C  stw r8, 0xc(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 8326DBB0: 91450010  stw r10, 0x10(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8326DBB4: 91650014  stw r11, 0x14(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8326DBB8: 91450018  stw r10, 0x18(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 8326DBBC: 9165001C  stw r11, 0x1c(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 8326DBC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326DBC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326DBC8 size=164
    let mut pc: u32 = 0x8326DBC8;
    'dispatch: loop {
        match pc {
            0x8326DBC8 => {
    //   block [0x8326DBC8..0x8326DC6C)
	// 8326DBC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326DBCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326DBD0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8326DBD4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8326DBD8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326DBDC: 3D60811C  lis r11, -0x7ee4
	ctx.r[11].s64 = -2128871424;
	// 8326DBE0: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 8326DBE4: 617F9DC5  ori r31, r11, 0x9dc5
	ctx.r[31].u64 = ctx.r[11].u64 | 40389;
	// 8326DBE8: 386A418C  addi r3, r10, 0x418c
	ctx.r[3].s64 = ctx.r[10].s64 + 16780;
	// 8326DBEC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8326DBF0: 4AF86169  bl 0x821f3d58
	ctx.lr = 0x8326DBF4;
	sub_821F3D58(ctx, base);
	// 8326DBF4: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8326DBF8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8326DBFC: 3D00820E  lis r8, -0x7df2
	ctx.r[8].s64 = -2113011712;
	// 8326DC00: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8326DC04: 38684184  addi r3, r8, 0x4184
	ctx.r[3].s64 = ctx.r[8].s64 + 16772;
	// 8326DC08: 9169C8F4  stw r11, -0x370c(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-14092 as u32), ctx.r[11].u32 ) };
	// 8326DC0C: 3BC9C8F4  addi r30, r9, -0x370c
	ctx.r[30].s64 = ctx.r[9].s64 + -14092;
	// 8326DC10: 4AF86149  bl 0x821f3d58
	ctx.lr = 0x8326DC14;
	sub_821F3D58(ctx, base);
	// 8326DC14: 907E0004  stw r3, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8326DC18: 3CE0820E  lis r7, -0x7df2
	ctx.r[7].s64 = -2113011712;
	// 8326DC1C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8326DC20: 3867417C  addi r3, r7, 0x417c
	ctx.r[3].s64 = ctx.r[7].s64 + 16764;
	// 8326DC24: 4AF86135  bl 0x821f3d58
	ctx.lr = 0x8326DC28;
	sub_821F3D58(ctx, base);
	// 8326DC28: 907E0008  stw r3, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 8326DC2C: 3CC0820E  lis r6, -0x7df2
	ctx.r[6].s64 = -2113011712;
	// 8326DC30: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8326DC34: 38664174  addi r3, r6, 0x4174
	ctx.r[3].s64 = ctx.r[6].s64 + 16756;
	// 8326DC38: 4AF86121  bl 0x821f3d58
	ctx.lr = 0x8326DC3C;
	sub_821F3D58(ctx, base);
	// 8326DC3C: 907E000C  stw r3, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 8326DC40: 3CA0820E  lis r5, -0x7df2
	ctx.r[5].s64 = -2113011712;
	// 8326DC44: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8326DC48: 3865416C  addi r3, r5, 0x416c
	ctx.r[3].s64 = ctx.r[5].s64 + 16748;
	// 8326DC4C: 4AF8610D  bl 0x821f3d58
	ctx.lr = 0x8326DC50;
	sub_821F3D58(ctx, base);
	// 8326DC50: 907E0010  stw r3, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 8326DC54: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8326DC58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326DC5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326DC60: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8326DC64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8326DC68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326DC70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8326DC70 size=12
    let mut pc: u32 = 0x8326DC70;
    'dispatch: loop {
        match pc {
            0x8326DC70 => {
    //   block [0x8326DC70..0x8326DC7C)
	// 8326DC70: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8326DC74: 386BED58  addi r3, r11, -0x12a8
	ctx.r[3].s64 = ctx.r[11].s64 + -4776;
	// 8326DC78: 4BA3C2A8  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326DC80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326DC80 size=68
    let mut pc: u32 = 0x8326DC80;
    'dispatch: loop {
        match pc {
            0x8326DC80 => {
    //   block [0x8326DC80..0x8326DCC4)
	// 8326DC80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326DC84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326DC88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326DC8C: 3D608290  lis r11, -0x7d70
	ctx.r[11].s64 = -2104492032;
	// 8326DC90: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 8326DC94: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8326DC98: 388A4D9C  addi r4, r10, 0x4d9c
	ctx.r[4].s64 = ctx.r[10].s64 + 19868;
	// 8326DC9C: 3869C918  addi r3, r9, -0x36e8
	ctx.r[3].s64 = ctx.r[9].s64 + -14056;
	// 8326DCA0: 38ABCA90  addi r5, r11, -0x3570
	ctx.r[5].s64 = ctx.r[11].s64 + -13680;
	// 8326DCA4: 4BC2AB3D  bl 0x82e987e0
	ctx.lr = 0x8326DCA8;
	sub_82E987E0(ctx, base);
	// 8326DCA8: 3D00832B  lis r8, -0x7cd5
	ctx.r[8].s64 = -2094333952;
	// 8326DCAC: 3868ED68  addi r3, r8, -0x1298
	ctx.r[3].s64 = ctx.r[8].s64 + -4760;
	// 8326DCB0: 4BA3C271  bl 0x82ca9f20
	ctx.lr = 0x8326DCB4;
	sub_82CA9F20(ctx, base);
	// 8326DCB4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326DCB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326DCBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326DCC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326DCC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326DCC8 size=76
    let mut pc: u32 = 0x8326DCC8;
    'dispatch: loop {
        match pc {
            0x8326DCC8 => {
    //   block [0x8326DCC8..0x8326DD14)
	// 8326DCC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326DCCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326DCD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326DCD4: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8326DCD8: 3D408290  lis r10, -0x7d70
	ctx.r[10].s64 = -2104492032;
	// 8326DCDC: 3D20820E  lis r9, -0x7df2
	ctx.r[9].s64 = -2113011712;
	// 8326DCE0: 3D00834A  lis r8, -0x7cb6
	ctx.r[8].s64 = -2092302336;
	// 8326DCE4: 38CB581C  addi r6, r11, 0x581c
	ctx.r[6].s64 = ctx.r[11].s64 + 22556;
	// 8326DCE8: 38894DC8  addi r4, r9, 0x4dc8
	ctx.r[4].s64 = ctx.r[9].s64 + 19912;
	// 8326DCEC: 3868CA2C  addi r3, r8, -0x35d4
	ctx.r[3].s64 = ctx.r[8].s64 + -13780;
	// 8326DCF0: 38AAD860  addi r5, r10, -0x27a0
	ctx.r[5].s64 = ctx.r[10].s64 + -10144;
	// 8326DCF4: 4BC17F05  bl 0x82e85bf8
	ctx.lr = 0x8326DCF8;
	sub_82E85BF8(ctx, base);
	// 8326DCF8: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8326DCFC: 3867ED80  addi r3, r7, -0x1280
	ctx.r[3].s64 = ctx.r[7].s64 + -4736;
	// 8326DD00: 4BA3C221  bl 0x82ca9f20
	ctx.lr = 0x8326DD04;
	sub_82CA9F20(ctx, base);
	// 8326DD04: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326DD08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326DD0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326DD10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326DD18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8326DD18 size=24
    let mut pc: u32 = 0x8326DD18;
    'dispatch: loop {
        match pc {
            0x8326DD18 => {
    //   block [0x8326DD18..0x8326DD30)
	// 8326DD18: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 8326DD1C: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326DD20: C00B8EF0  lfs f0, -0x7110(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28944 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8326DD24: EC000032  fmuls f0, f0, f0
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[0].f64) as f32) as f64);
	// 8326DD28: D00ACB40  stfs f0, -0x34c0(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13504 as u32), tmp.u32 ) };
	// 8326DD2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326DD30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326DD30 size=64
    let mut pc: u32 = 0x8326DD30;
    'dispatch: loop {
        match pc {
            0x8326DD30 => {
    //   block [0x8326DD30..0x8326DD70)
	// 8326DD30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326DD34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326DD38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326DD3C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326DD40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326DD44: 388B5380  addi r4, r11, 0x5380
	ctx.r[4].s64 = ctx.r[11].s64 + 21376;
	// 8326DD48: 386ACB44  addi r3, r10, -0x34bc
	ctx.r[3].s64 = ctx.r[10].s64 + -13500;
	// 8326DD4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326DD50: 4AFBF181  bl 0x8222ced0
	ctx.lr = 0x8326DD54;
	sub_8222CED0(ctx, base);
	// 8326DD54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326DD58: 3869ED98  addi r3, r9, -0x1268
	ctx.r[3].s64 = ctx.r[9].s64 + -4712;
	// 8326DD5C: 4BA3C1C5  bl 0x82ca9f20
	ctx.lr = 0x8326DD60;
	sub_82CA9F20(ctx, base);
	// 8326DD60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326DD64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326DD68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326DD6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326DD70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8326DD70 size=12
    let mut pc: u32 = 0x8326DD70;
    'dispatch: loop {
        match pc {
            0x8326DD70 => {
    //   block [0x8326DD70..0x8326DD7C)
	// 8326DD70: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8326DD74: 386BEDA8  addi r3, r11, -0x1258
	ctx.r[3].s64 = ctx.r[11].s64 + -4696;
	// 8326DD78: 4BA3C1A8  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326DD80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326DD80 size=64
    let mut pc: u32 = 0x8326DD80;
    'dispatch: loop {
        match pc {
            0x8326DD80 => {
    //   block [0x8326DD80..0x8326DDC0)
	// 8326DD80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326DD84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326DD88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326DD8C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326DD90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326DD94: 388B53BC  addi r4, r11, 0x53bc
	ctx.r[4].s64 = ctx.r[11].s64 + 21436;
	// 8326DD98: 386ACB58  addi r3, r10, -0x34a8
	ctx.r[3].s64 = ctx.r[10].s64 + -13480;
	// 8326DD9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326DDA0: 4AFBF131  bl 0x8222ced0
	ctx.lr = 0x8326DDA4;
	sub_8222CED0(ctx, base);
	// 8326DDA4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326DDA8: 3869EE00  addi r3, r9, -0x1200
	ctx.r[3].s64 = ctx.r[9].s64 + -4608;
	// 8326DDAC: 4BA3C175  bl 0x82ca9f20
	ctx.lr = 0x8326DDB0;
	sub_82CA9F20(ctx, base);
	// 8326DDB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326DDB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326DDB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326DDBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326DDC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326DDC0 size=64
    let mut pc: u32 = 0x8326DDC0;
    'dispatch: loop {
        match pc {
            0x8326DDC0 => {
    //   block [0x8326DDC0..0x8326DE00)
	// 8326DDC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326DDC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326DDC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326DDCC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326DDD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326DDD4: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 8326DDD8: 386ACB5C  addi r3, r10, -0x34a4
	ctx.r[3].s64 = ctx.r[10].s64 + -13476;
	// 8326DDDC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326DDE0: 4AFBF0F1  bl 0x8222ced0
	ctx.lr = 0x8326DDE4;
	sub_8222CED0(ctx, base);
	// 8326DDE4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326DDE8: 3869EE10  addi r3, r9, -0x11f0
	ctx.r[3].s64 = ctx.r[9].s64 + -4592;
	// 8326DDEC: 4BA3C135  bl 0x82ca9f20
	ctx.lr = 0x8326DDF0;
	sub_82CA9F20(ctx, base);
	// 8326DDF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326DDF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326DDF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326DDFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326DE00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326DE00 size=64
    let mut pc: u32 = 0x8326DE00;
    'dispatch: loop {
        match pc {
            0x8326DE00 => {
    //   block [0x8326DE00..0x8326DE40)
	// 8326DE00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326DE04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326DE08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326DE0C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326DE10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326DE14: 388B9620  addi r4, r11, -0x69e0
	ctx.r[4].s64 = ctx.r[11].s64 + -27104;
	// 8326DE18: 386ACB60  addi r3, r10, -0x34a0
	ctx.r[3].s64 = ctx.r[10].s64 + -13472;
	// 8326DE1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326DE20: 4AFBF0B1  bl 0x8222ced0
	ctx.lr = 0x8326DE24;
	sub_8222CED0(ctx, base);
	// 8326DE24: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326DE28: 3869EE20  addi r3, r9, -0x11e0
	ctx.r[3].s64 = ctx.r[9].s64 + -4576;
	// 8326DE2C: 4BA3C0F5  bl 0x82ca9f20
	ctx.lr = 0x8326DE30;
	sub_82CA9F20(ctx, base);
	// 8326DE30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326DE34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326DE38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326DE3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326DE40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326DE40 size=56
    let mut pc: u32 = 0x8326DE40;
    'dispatch: loop {
        match pc {
            0x8326DE40 => {
    //   block [0x8326DE40..0x8326DE78)
	// 8326DE40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326DE44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326DE48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326DE4C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326DE50: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326DE54: 386B5CC8  addi r3, r11, 0x5cc8
	ctx.r[3].s64 = ctx.r[11].s64 + 23752;
	// 8326DE58: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326DE5C: 4AF85EFD  bl 0x821f3d58
	ctx.lr = 0x8326DE60;
	sub_821F3D58(ctx, base);
	// 8326DE60: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326DE64: 906ACB64  stw r3, -0x349c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13468 as u32), ctx.r[3].u32 ) };
	// 8326DE68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326DE6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326DE70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326DE74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326DE78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326DE78 size=56
    let mut pc: u32 = 0x8326DE78;
    'dispatch: loop {
        match pc {
            0x8326DE78 => {
    //   block [0x8326DE78..0x8326DEB0)
	// 8326DE78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326DE7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326DE80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326DE84: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326DE88: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326DE8C: 386B5CDC  addi r3, r11, 0x5cdc
	ctx.r[3].s64 = ctx.r[11].s64 + 23772;
	// 8326DE90: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326DE94: 4AF85EC5  bl 0x821f3d58
	ctx.lr = 0x8326DE98;
	sub_821F3D58(ctx, base);
	// 8326DE98: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326DE9C: 906ACB68  stw r3, -0x3498(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13464 as u32), ctx.r[3].u32 ) };
	// 8326DEA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326DEA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326DEA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326DEAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326DEB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326DEB0 size=56
    let mut pc: u32 = 0x8326DEB0;
    'dispatch: loop {
        match pc {
            0x8326DEB0 => {
    //   block [0x8326DEB0..0x8326DEE8)
	// 8326DEB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326DEB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326DEB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326DEBC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326DEC0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326DEC4: 386B5FA0  addi r3, r11, 0x5fa0
	ctx.r[3].s64 = ctx.r[11].s64 + 24480;
	// 8326DEC8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326DECC: 4AF85E8D  bl 0x821f3d58
	ctx.lr = 0x8326DED0;
	sub_821F3D58(ctx, base);
	// 8326DED0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326DED4: 906ACB6C  stw r3, -0x3494(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13460 as u32), ctx.r[3].u32 ) };
	// 8326DED8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326DEDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326DEE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326DEE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326DEE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326DEE8 size=56
    let mut pc: u32 = 0x8326DEE8;
    'dispatch: loop {
        match pc {
            0x8326DEE8 => {
    //   block [0x8326DEE8..0x8326DF20)
	// 8326DEE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326DEEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326DEF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326DEF4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326DEF8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326DEFC: 386B5FB8  addi r3, r11, 0x5fb8
	ctx.r[3].s64 = ctx.r[11].s64 + 24504;
	// 8326DF00: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326DF04: 4AF85E55  bl 0x821f3d58
	ctx.lr = 0x8326DF08;
	sub_821F3D58(ctx, base);
	// 8326DF08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326DF0C: 906ACB70  stw r3, -0x3490(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13456 as u32), ctx.r[3].u32 ) };
	// 8326DF10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326DF14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326DF18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326DF1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326DF20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326DF20 size=56
    let mut pc: u32 = 0x8326DF20;
    'dispatch: loop {
        match pc {
            0x8326DF20 => {
    //   block [0x8326DF20..0x8326DF58)
	// 8326DF20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326DF24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326DF28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326DF2C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326DF30: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326DF34: 386B6428  addi r3, r11, 0x6428
	ctx.r[3].s64 = ctx.r[11].s64 + 25640;
	// 8326DF38: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326DF3C: 4AF85E1D  bl 0x821f3d58
	ctx.lr = 0x8326DF40;
	sub_821F3D58(ctx, base);
	// 8326DF40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326DF44: 906ACB74  stw r3, -0x348c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13452 as u32), ctx.r[3].u32 ) };
	// 8326DF48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326DF4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326DF50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326DF54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326DF58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326DF58 size=64
    let mut pc: u32 = 0x8326DF58;
    'dispatch: loop {
        match pc {
            0x8326DF58 => {
    //   block [0x8326DF58..0x8326DF98)
	// 8326DF58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326DF5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326DF60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326DF64: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326DF68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326DF6C: 388B74A8  addi r4, r11, 0x74a8
	ctx.r[4].s64 = ctx.r[11].s64 + 29864;
	// 8326DF70: 386ACB78  addi r3, r10, -0x3488
	ctx.r[3].s64 = ctx.r[10].s64 + -13448;
	// 8326DF74: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326DF78: 4AFBEF59  bl 0x8222ced0
	ctx.lr = 0x8326DF7C;
	sub_8222CED0(ctx, base);
	// 8326DF7C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326DF80: 3869EE30  addi r3, r9, -0x11d0
	ctx.r[3].s64 = ctx.r[9].s64 + -4560;
	// 8326DF84: 4BA3BF9D  bl 0x82ca9f20
	ctx.lr = 0x8326DF88;
	sub_82CA9F20(ctx, base);
	// 8326DF88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326DF8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326DF90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326DF94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326DF98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326DF98 size=64
    let mut pc: u32 = 0x8326DF98;
    'dispatch: loop {
        match pc {
            0x8326DF98 => {
    //   block [0x8326DF98..0x8326DFD8)
	// 8326DF98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326DF9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326DFA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326DFA4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326DFA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326DFAC: 388B74B4  addi r4, r11, 0x74b4
	ctx.r[4].s64 = ctx.r[11].s64 + 29876;
	// 8326DFB0: 386ACB7C  addi r3, r10, -0x3484
	ctx.r[3].s64 = ctx.r[10].s64 + -13444;
	// 8326DFB4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326DFB8: 4AFBEF19  bl 0x8222ced0
	ctx.lr = 0x8326DFBC;
	sub_8222CED0(ctx, base);
	// 8326DFBC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326DFC0: 3869EE40  addi r3, r9, -0x11c0
	ctx.r[3].s64 = ctx.r[9].s64 + -4544;
	// 8326DFC4: 4BA3BF5D  bl 0x82ca9f20
	ctx.lr = 0x8326DFC8;
	sub_82CA9F20(ctx, base);
	// 8326DFC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326DFCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326DFD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326DFD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326DFD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326DFD8 size=64
    let mut pc: u32 = 0x8326DFD8;
    'dispatch: loop {
        match pc {
            0x8326DFD8 => {
    //   block [0x8326DFD8..0x8326E018)
	// 8326DFD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326DFDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326DFE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326DFE4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326DFE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326DFEC: 388B74C0  addi r4, r11, 0x74c0
	ctx.r[4].s64 = ctx.r[11].s64 + 29888;
	// 8326DFF0: 386ACB80  addi r3, r10, -0x3480
	ctx.r[3].s64 = ctx.r[10].s64 + -13440;
	// 8326DFF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326DFF8: 4AFBEED9  bl 0x8222ced0
	ctx.lr = 0x8326DFFC;
	sub_8222CED0(ctx, base);
	// 8326DFFC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326E000: 3869EE50  addi r3, r9, -0x11b0
	ctx.r[3].s64 = ctx.r[9].s64 + -4528;
	// 8326E004: 4BA3BF1D  bl 0x82ca9f20
	ctx.lr = 0x8326E008;
	sub_82CA9F20(ctx, base);
	// 8326E008: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E00C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E010: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E014: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E018 size=64
    let mut pc: u32 = 0x8326E018;
    'dispatch: loop {
        match pc {
            0x8326E018 => {
    //   block [0x8326E018..0x8326E058)
	// 8326E018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E01C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E020: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E024: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326E028: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E02C: 388B74CC  addi r4, r11, 0x74cc
	ctx.r[4].s64 = ctx.r[11].s64 + 29900;
	// 8326E030: 386ACB84  addi r3, r10, -0x347c
	ctx.r[3].s64 = ctx.r[10].s64 + -13436;
	// 8326E034: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326E038: 4AFBEE99  bl 0x8222ced0
	ctx.lr = 0x8326E03C;
	sub_8222CED0(ctx, base);
	// 8326E03C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326E040: 3869EE60  addi r3, r9, -0x11a0
	ctx.r[3].s64 = ctx.r[9].s64 + -4512;
	// 8326E044: 4BA3BEDD  bl 0x82ca9f20
	ctx.lr = 0x8326E048;
	sub_82CA9F20(ctx, base);
	// 8326E048: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E04C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E050: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E054: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E058 size=64
    let mut pc: u32 = 0x8326E058;
    'dispatch: loop {
        match pc {
            0x8326E058 => {
    //   block [0x8326E058..0x8326E098)
	// 8326E058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E05C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E060: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E064: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326E068: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E06C: 388B74D8  addi r4, r11, 0x74d8
	ctx.r[4].s64 = ctx.r[11].s64 + 29912;
	// 8326E070: 386ACB88  addi r3, r10, -0x3478
	ctx.r[3].s64 = ctx.r[10].s64 + -13432;
	// 8326E074: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326E078: 4AFBEE59  bl 0x8222ced0
	ctx.lr = 0x8326E07C;
	sub_8222CED0(ctx, base);
	// 8326E07C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326E080: 3869EE70  addi r3, r9, -0x1190
	ctx.r[3].s64 = ctx.r[9].s64 + -4496;
	// 8326E084: 4BA3BE9D  bl 0x82ca9f20
	ctx.lr = 0x8326E088;
	sub_82CA9F20(ctx, base);
	// 8326E088: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E08C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E090: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E094: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E098 size=64
    let mut pc: u32 = 0x8326E098;
    'dispatch: loop {
        match pc {
            0x8326E098 => {
    //   block [0x8326E098..0x8326E0D8)
	// 8326E098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E09C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E0A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E0A4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326E0A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E0AC: 388B74E4  addi r4, r11, 0x74e4
	ctx.r[4].s64 = ctx.r[11].s64 + 29924;
	// 8326E0B0: 386ACB8C  addi r3, r10, -0x3474
	ctx.r[3].s64 = ctx.r[10].s64 + -13428;
	// 8326E0B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326E0B8: 4AFBEE19  bl 0x8222ced0
	ctx.lr = 0x8326E0BC;
	sub_8222CED0(ctx, base);
	// 8326E0BC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326E0C0: 3869EE80  addi r3, r9, -0x1180
	ctx.r[3].s64 = ctx.r[9].s64 + -4480;
	// 8326E0C4: 4BA3BE5D  bl 0x82ca9f20
	ctx.lr = 0x8326E0C8;
	sub_82CA9F20(ctx, base);
	// 8326E0C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E0CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E0D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E0D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E0D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8326E0D8 size=32
    let mut pc: u32 = 0x8326E0D8;
    'dispatch: loop {
        match pc {
            0x8326E0D8 => {
    //   block [0x8326E0D8..0x8326E0F8)
	// 8326E0D8: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 8326E0DC: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E0E0: C80B9660  lfd f0, -0x69a0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-27040 as u32) ) };
	// 8326E0E4: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 8326E0E8: D801FFF0  stfd f0, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[0].u64 ) };
	// 8326E0EC: 8161FFF4  lwz r11, -0xc(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 8326E0F0: 916ACB90  stw r11, -0x3470(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13424 as u32), ctx.r[11].u32 ) };
	// 8326E0F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E0F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E0F8 size=64
    let mut pc: u32 = 0x8326E0F8;
    'dispatch: loop {
        match pc {
            0x8326E0F8 => {
    //   block [0x8326E0F8..0x8326E138)
	// 8326E0F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E0FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E100: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E104: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326E108: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E10C: 388B7BB8  addi r4, r11, 0x7bb8
	ctx.r[4].s64 = ctx.r[11].s64 + 31672;
	// 8326E110: 386ACB94  addi r3, r10, -0x346c
	ctx.r[3].s64 = ctx.r[10].s64 + -13420;
	// 8326E114: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326E118: 4AFBEDB9  bl 0x8222ced0
	ctx.lr = 0x8326E11C;
	sub_8222CED0(ctx, base);
	// 8326E11C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326E120: 3869EE90  addi r3, r9, -0x1170
	ctx.r[3].s64 = ctx.r[9].s64 + -4464;
	// 8326E124: 4BA3BDFD  bl 0x82ca9f20
	ctx.lr = 0x8326E128;
	sub_82CA9F20(ctx, base);
	// 8326E128: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E12C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E138 size=64
    let mut pc: u32 = 0x8326E138;
    'dispatch: loop {
        match pc {
            0x8326E138 => {
    //   block [0x8326E138..0x8326E178)
	// 8326E138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E13C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E140: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E144: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326E148: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E14C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8326E150: 386ACB98  addi r3, r10, -0x3468
	ctx.r[3].s64 = ctx.r[10].s64 + -13416;
	// 8326E154: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326E158: 4AFBED79  bl 0x8222ced0
	ctx.lr = 0x8326E15C;
	sub_8222CED0(ctx, base);
	// 8326E15C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326E160: 3869EEA0  addi r3, r9, -0x1160
	ctx.r[3].s64 = ctx.r[9].s64 + -4448;
	// 8326E164: 4BA3BDBD  bl 0x82ca9f20
	ctx.lr = 0x8326E168;
	sub_82CA9F20(ctx, base);
	// 8326E168: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E16C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E170: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E174: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E178 size=64
    let mut pc: u32 = 0x8326E178;
    'dispatch: loop {
        match pc {
            0x8326E178 => {
    //   block [0x8326E178..0x8326E1B8)
	// 8326E178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E17C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E180: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E184: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326E188: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E18C: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8326E190: 386ACB9C  addi r3, r10, -0x3464
	ctx.r[3].s64 = ctx.r[10].s64 + -13412;
	// 8326E194: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326E198: 4AFBED39  bl 0x8222ced0
	ctx.lr = 0x8326E19C;
	sub_8222CED0(ctx, base);
	// 8326E19C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326E1A0: 3869EEB0  addi r3, r9, -0x1150
	ctx.r[3].s64 = ctx.r[9].s64 + -4432;
	// 8326E1A4: 4BA3BD7D  bl 0x82ca9f20
	ctx.lr = 0x8326E1A8;
	sub_82CA9F20(ctx, base);
	// 8326E1A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E1AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E1B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E1B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E1B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E1B8 size=64
    let mut pc: u32 = 0x8326E1B8;
    'dispatch: loop {
        match pc {
            0x8326E1B8 => {
    //   block [0x8326E1B8..0x8326E1F8)
	// 8326E1B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E1BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E1C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E1C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326E1C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E1CC: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8326E1D0: 386ACBA0  addi r3, r10, -0x3460
	ctx.r[3].s64 = ctx.r[10].s64 + -13408;
	// 8326E1D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326E1D8: 4AFBECF9  bl 0x8222ced0
	ctx.lr = 0x8326E1DC;
	sub_8222CED0(ctx, base);
	// 8326E1DC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326E1E0: 3869EEC0  addi r3, r9, -0x1140
	ctx.r[3].s64 = ctx.r[9].s64 + -4416;
	// 8326E1E4: 4BA3BD3D  bl 0x82ca9f20
	ctx.lr = 0x8326E1E8;
	sub_82CA9F20(ctx, base);
	// 8326E1E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E1EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E1F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E1F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E1F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E1F8 size=64
    let mut pc: u32 = 0x8326E1F8;
    'dispatch: loop {
        match pc {
            0x8326E1F8 => {
    //   block [0x8326E1F8..0x8326E238)
	// 8326E1F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E1FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E200: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E204: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326E208: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E20C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8326E210: 386ACBA4  addi r3, r10, -0x345c
	ctx.r[3].s64 = ctx.r[10].s64 + -13404;
	// 8326E214: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326E218: 4AFBECB9  bl 0x8222ced0
	ctx.lr = 0x8326E21C;
	sub_8222CED0(ctx, base);
	// 8326E21C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326E220: 3869EED0  addi r3, r9, -0x1130
	ctx.r[3].s64 = ctx.r[9].s64 + -4400;
	// 8326E224: 4BA3BCFD  bl 0x82ca9f20
	ctx.lr = 0x8326E228;
	sub_82CA9F20(ctx, base);
	// 8326E228: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E22C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E230: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E234: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E238 size=64
    let mut pc: u32 = 0x8326E238;
    'dispatch: loop {
        match pc {
            0x8326E238 => {
    //   block [0x8326E238..0x8326E278)
	// 8326E238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E23C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E240: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E244: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326E248: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E24C: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8326E250: 386ACBA8  addi r3, r10, -0x3458
	ctx.r[3].s64 = ctx.r[10].s64 + -13400;
	// 8326E254: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326E258: 4AFBEC79  bl 0x8222ced0
	ctx.lr = 0x8326E25C;
	sub_8222CED0(ctx, base);
	// 8326E25C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326E260: 3869EEE0  addi r3, r9, -0x1120
	ctx.r[3].s64 = ctx.r[9].s64 + -4384;
	// 8326E264: 4BA3BCBD  bl 0x82ca9f20
	ctx.lr = 0x8326E268;
	sub_82CA9F20(ctx, base);
	// 8326E268: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E26C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E270: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E274: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E278 size=64
    let mut pc: u32 = 0x8326E278;
    'dispatch: loop {
        match pc {
            0x8326E278 => {
    //   block [0x8326E278..0x8326E2B8)
	// 8326E278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E27C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E280: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E284: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326E288: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E28C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8326E290: 386ACBAC  addi r3, r10, -0x3454
	ctx.r[3].s64 = ctx.r[10].s64 + -13396;
	// 8326E294: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326E298: 4AFBEC39  bl 0x8222ced0
	ctx.lr = 0x8326E29C;
	sub_8222CED0(ctx, base);
	// 8326E29C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326E2A0: 3869EEF0  addi r3, r9, -0x1110
	ctx.r[3].s64 = ctx.r[9].s64 + -4368;
	// 8326E2A4: 4BA3BC7D  bl 0x82ca9f20
	ctx.lr = 0x8326E2A8;
	sub_82CA9F20(ctx, base);
	// 8326E2A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E2AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E2B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E2B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E2B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E2B8 size=56
    let mut pc: u32 = 0x8326E2B8;
    'dispatch: loop {
        match pc {
            0x8326E2B8 => {
    //   block [0x8326E2B8..0x8326E2F0)
	// 8326E2B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E2BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E2C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E2C4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8326E2C8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E2CC: 386B123C  addi r3, r11, 0x123c
	ctx.r[3].s64 = ctx.r[11].s64 + 4668;
	// 8326E2D0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E2D4: 4AF85A85  bl 0x821f3d58
	ctx.lr = 0x8326E2D8;
	sub_821F3D58(ctx, base);
	// 8326E2D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E2DC: 906ACBB0  stw r3, -0x3450(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13392 as u32), ctx.r[3].u32 ) };
	// 8326E2E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E2E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E2E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E2EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E2F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E2F0 size=56
    let mut pc: u32 = 0x8326E2F0;
    'dispatch: loop {
        match pc {
            0x8326E2F0 => {
    //   block [0x8326E2F0..0x8326E328)
	// 8326E2F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E2F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E2F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E2FC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E300: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E304: 386B92EC  addi r3, r11, -0x6d14
	ctx.r[3].s64 = ctx.r[11].s64 + -27924;
	// 8326E308: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E30C: 4AF85A4D  bl 0x821f3d58
	ctx.lr = 0x8326E310;
	sub_821F3D58(ctx, base);
	// 8326E310: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E314: 906ACBB4  stw r3, -0x344c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13388 as u32), ctx.r[3].u32 ) };
	// 8326E318: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E31C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E320: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E324: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E328 size=56
    let mut pc: u32 = 0x8326E328;
    'dispatch: loop {
        match pc {
            0x8326E328 => {
    //   block [0x8326E328..0x8326E360)
	// 8326E328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E32C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E330: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E334: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E338: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E33C: 386B92F8  addi r3, r11, -0x6d08
	ctx.r[3].s64 = ctx.r[11].s64 + -27912;
	// 8326E340: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E344: 4AF85A15  bl 0x821f3d58
	ctx.lr = 0x8326E348;
	sub_821F3D58(ctx, base);
	// 8326E348: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E34C: 906ACBB8  stw r3, -0x3448(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13384 as u32), ctx.r[3].u32 ) };
	// 8326E350: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E354: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E358: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E35C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E360 size=56
    let mut pc: u32 = 0x8326E360;
    'dispatch: loop {
        match pc {
            0x8326E360 => {
    //   block [0x8326E360..0x8326E398)
	// 8326E360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E368: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E36C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E370: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E374: 386B9428  addi r3, r11, -0x6bd8
	ctx.r[3].s64 = ctx.r[11].s64 + -27608;
	// 8326E378: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E37C: 4AF859DD  bl 0x821f3d58
	ctx.lr = 0x8326E380;
	sub_821F3D58(ctx, base);
	// 8326E380: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E384: 906ACBBC  stw r3, -0x3444(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13380 as u32), ctx.r[3].u32 ) };
	// 8326E388: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E38C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E390: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E394: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E398 size=56
    let mut pc: u32 = 0x8326E398;
    'dispatch: loop {
        match pc {
            0x8326E398 => {
    //   block [0x8326E398..0x8326E3D0)
	// 8326E398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E39C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E3A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E3A4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E3A8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E3AC: 386B9430  addi r3, r11, -0x6bd0
	ctx.r[3].s64 = ctx.r[11].s64 + -27600;
	// 8326E3B0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E3B4: 4AF859A5  bl 0x821f3d58
	ctx.lr = 0x8326E3B8;
	sub_821F3D58(ctx, base);
	// 8326E3B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E3BC: 906ACBC0  stw r3, -0x3440(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13376 as u32), ctx.r[3].u32 ) };
	// 8326E3C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E3C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E3C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E3CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E3D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E3D0 size=56
    let mut pc: u32 = 0x8326E3D0;
    'dispatch: loop {
        match pc {
            0x8326E3D0 => {
    //   block [0x8326E3D0..0x8326E408)
	// 8326E3D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E3D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E3D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E3DC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E3E0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E3E4: 386B943C  addi r3, r11, -0x6bc4
	ctx.r[3].s64 = ctx.r[11].s64 + -27588;
	// 8326E3E8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E3EC: 4AF8596D  bl 0x821f3d58
	ctx.lr = 0x8326E3F0;
	sub_821F3D58(ctx, base);
	// 8326E3F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E3F4: 906ACBC4  stw r3, -0x343c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13372 as u32), ctx.r[3].u32 ) };
	// 8326E3F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E3FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E400: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E404: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E408 size=56
    let mut pc: u32 = 0x8326E408;
    'dispatch: loop {
        match pc {
            0x8326E408 => {
    //   block [0x8326E408..0x8326E440)
	// 8326E408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E40C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E410: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E414: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8326E418: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E41C: 386B123C  addi r3, r11, 0x123c
	ctx.r[3].s64 = ctx.r[11].s64 + 4668;
	// 8326E420: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E424: 4AF85935  bl 0x821f3d58
	ctx.lr = 0x8326E428;
	sub_821F3D58(ctx, base);
	// 8326E428: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E42C: 906ACBC8  stw r3, -0x3438(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13368 as u32), ctx.r[3].u32 ) };
	// 8326E430: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E434: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E438: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E43C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E440 size=56
    let mut pc: u32 = 0x8326E440;
    'dispatch: loop {
        match pc {
            0x8326E440 => {
    //   block [0x8326E440..0x8326E478)
	// 8326E440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E448: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E44C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E450: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E454: 386B944C  addi r3, r11, -0x6bb4
	ctx.r[3].s64 = ctx.r[11].s64 + -27572;
	// 8326E458: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E45C: 4AF858FD  bl 0x821f3d58
	ctx.lr = 0x8326E460;
	sub_821F3D58(ctx, base);
	// 8326E460: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E464: 906ACBCC  stw r3, -0x3434(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13364 as u32), ctx.r[3].u32 ) };
	// 8326E468: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E46C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E470: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E474: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E478 size=56
    let mut pc: u32 = 0x8326E478;
    'dispatch: loop {
        match pc {
            0x8326E478 => {
    //   block [0x8326E478..0x8326E4B0)
	// 8326E478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E47C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E480: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E484: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E488: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E48C: 386B9458  addi r3, r11, -0x6ba8
	ctx.r[3].s64 = ctx.r[11].s64 + -27560;
	// 8326E490: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E494: 4AF858C5  bl 0x821f3d58
	ctx.lr = 0x8326E498;
	sub_821F3D58(ctx, base);
	// 8326E498: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E49C: 906ACBD0  stw r3, -0x3430(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13360 as u32), ctx.r[3].u32 ) };
	// 8326E4A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E4A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E4A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E4AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E4B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E4B0 size=56
    let mut pc: u32 = 0x8326E4B0;
    'dispatch: loop {
        match pc {
            0x8326E4B0 => {
    //   block [0x8326E4B0..0x8326E4E8)
	// 8326E4B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E4B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E4B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E4BC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E4C0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E4C4: 386B9464  addi r3, r11, -0x6b9c
	ctx.r[3].s64 = ctx.r[11].s64 + -27548;
	// 8326E4C8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E4CC: 4AF8588D  bl 0x821f3d58
	ctx.lr = 0x8326E4D0;
	sub_821F3D58(ctx, base);
	// 8326E4D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E4D4: 906ACBD4  stw r3, -0x342c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13356 as u32), ctx.r[3].u32 ) };
	// 8326E4D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E4DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E4E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E4E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E4E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E4E8 size=56
    let mut pc: u32 = 0x8326E4E8;
    'dispatch: loop {
        match pc {
            0x8326E4E8 => {
    //   block [0x8326E4E8..0x8326E520)
	// 8326E4E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E4EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E4F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E4F4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E4F8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E4FC: 386B92EC  addi r3, r11, -0x6d14
	ctx.r[3].s64 = ctx.r[11].s64 + -27924;
	// 8326E500: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E504: 4AF85855  bl 0x821f3d58
	ctx.lr = 0x8326E508;
	sub_821F3D58(ctx, base);
	// 8326E508: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E50C: 906ACBD8  stw r3, -0x3428(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13352 as u32), ctx.r[3].u32 ) };
	// 8326E510: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E514: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E518: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E51C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E520 size=56
    let mut pc: u32 = 0x8326E520;
    'dispatch: loop {
        match pc {
            0x8326E520 => {
    //   block [0x8326E520..0x8326E558)
	// 8326E520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E528: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E52C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E530: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E534: 386B92F8  addi r3, r11, -0x6d08
	ctx.r[3].s64 = ctx.r[11].s64 + -27912;
	// 8326E538: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E53C: 4AF8581D  bl 0x821f3d58
	ctx.lr = 0x8326E540;
	sub_821F3D58(ctx, base);
	// 8326E540: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E544: 906ACBDC  stw r3, -0x3424(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13348 as u32), ctx.r[3].u32 ) };
	// 8326E548: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E54C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E550: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E554: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E558 size=56
    let mut pc: u32 = 0x8326E558;
    'dispatch: loop {
        match pc {
            0x8326E558 => {
    //   block [0x8326E558..0x8326E590)
	// 8326E558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E55C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E560: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E564: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E568: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E56C: 386B9474  addi r3, r11, -0x6b8c
	ctx.r[3].s64 = ctx.r[11].s64 + -27532;
	// 8326E570: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E574: 4AF857E5  bl 0x821f3d58
	ctx.lr = 0x8326E578;
	sub_821F3D58(ctx, base);
	// 8326E578: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E57C: 906ACBE0  stw r3, -0x3420(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13344 as u32), ctx.r[3].u32 ) };
	// 8326E580: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E584: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E588: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E58C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E590 size=56
    let mut pc: u32 = 0x8326E590;
    'dispatch: loop {
        match pc {
            0x8326E590 => {
    //   block [0x8326E590..0x8326E5C8)
	// 8326E590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E598: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E59C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E5A0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E5A4: 386B9480  addi r3, r11, -0x6b80
	ctx.r[3].s64 = ctx.r[11].s64 + -27520;
	// 8326E5A8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E5AC: 4AF857AD  bl 0x821f3d58
	ctx.lr = 0x8326E5B0;
	sub_821F3D58(ctx, base);
	// 8326E5B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E5B4: 906ACBE4  stw r3, -0x341c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13340 as u32), ctx.r[3].u32 ) };
	// 8326E5B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E5BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E5C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E5C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E5C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E5C8 size=56
    let mut pc: u32 = 0x8326E5C8;
    'dispatch: loop {
        match pc {
            0x8326E5C8 => {
    //   block [0x8326E5C8..0x8326E600)
	// 8326E5C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E5CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E5D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E5D4: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8326E5D8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E5DC: 386BA994  addi r3, r11, -0x566c
	ctx.r[3].s64 = ctx.r[11].s64 + -22124;
	// 8326E5E0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E5E4: 4AF85775  bl 0x821f3d58
	ctx.lr = 0x8326E5E8;
	sub_821F3D58(ctx, base);
	// 8326E5E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E5EC: 906ACBE8  stw r3, -0x3418(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13336 as u32), ctx.r[3].u32 ) };
	// 8326E5F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E5F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E5F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E5FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E600 size=56
    let mut pc: u32 = 0x8326E600;
    'dispatch: loop {
        match pc {
            0x8326E600 => {
    //   block [0x8326E600..0x8326E638)
	// 8326E600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E604: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E608: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E60C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E610: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E614: 386B9490  addi r3, r11, -0x6b70
	ctx.r[3].s64 = ctx.r[11].s64 + -27504;
	// 8326E618: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E61C: 4AF8573D  bl 0x821f3d58
	ctx.lr = 0x8326E620;
	sub_821F3D58(ctx, base);
	// 8326E620: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E624: 906ACBEC  stw r3, -0x3414(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13332 as u32), ctx.r[3].u32 ) };
	// 8326E628: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E62C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E630: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E634: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E638 size=56
    let mut pc: u32 = 0x8326E638;
    'dispatch: loop {
        match pc {
            0x8326E638 => {
    //   block [0x8326E638..0x8326E670)
	// 8326E638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E63C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E640: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E644: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8326E648: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E64C: 386BA9A0  addi r3, r11, -0x5660
	ctx.r[3].s64 = ctx.r[11].s64 + -22112;
	// 8326E650: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E654: 4AF85705  bl 0x821f3d58
	ctx.lr = 0x8326E658;
	sub_821F3D58(ctx, base);
	// 8326E658: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E65C: 906ACBF0  stw r3, -0x3410(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13328 as u32), ctx.r[3].u32 ) };
	// 8326E660: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E664: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E668: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E66C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E670 size=56
    let mut pc: u32 = 0x8326E670;
    'dispatch: loop {
        match pc {
            0x8326E670 => {
    //   block [0x8326E670..0x8326E6A8)
	// 8326E670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E678: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E67C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E680: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E684: 386B94A0  addi r3, r11, -0x6b60
	ctx.r[3].s64 = ctx.r[11].s64 + -27488;
	// 8326E688: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E68C: 4AF856CD  bl 0x821f3d58
	ctx.lr = 0x8326E690;
	sub_821F3D58(ctx, base);
	// 8326E690: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E694: 906ACBF4  stw r3, -0x340c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13324 as u32), ctx.r[3].u32 ) };
	// 8326E698: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E69C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E6A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E6A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E6A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E6A8 size=56
    let mut pc: u32 = 0x8326E6A8;
    'dispatch: loop {
        match pc {
            0x8326E6A8 => {
    //   block [0x8326E6A8..0x8326E6E0)
	// 8326E6A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E6AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E6B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E6B4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E6B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E6BC: 386B94B8  addi r3, r11, -0x6b48
	ctx.r[3].s64 = ctx.r[11].s64 + -27464;
	// 8326E6C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E6C4: 4AF85695  bl 0x821f3d58
	ctx.lr = 0x8326E6C8;
	sub_821F3D58(ctx, base);
	// 8326E6C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E6CC: 906ACBF8  stw r3, -0x3408(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13320 as u32), ctx.r[3].u32 ) };
	// 8326E6D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E6D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E6D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E6DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E6E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E6E0 size=56
    let mut pc: u32 = 0x8326E6E0;
    'dispatch: loop {
        match pc {
            0x8326E6E0 => {
    //   block [0x8326E6E0..0x8326E718)
	// 8326E6E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E6E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E6E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E6EC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E6F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E6F4: 386B94C8  addi r3, r11, -0x6b38
	ctx.r[3].s64 = ctx.r[11].s64 + -27448;
	// 8326E6F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E6FC: 4AF8565D  bl 0x821f3d58
	ctx.lr = 0x8326E700;
	sub_821F3D58(ctx, base);
	// 8326E700: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E704: 906ACBFC  stw r3, -0x3404(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13316 as u32), ctx.r[3].u32 ) };
	// 8326E708: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E70C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E710: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E714: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E718 size=56
    let mut pc: u32 = 0x8326E718;
    'dispatch: loop {
        match pc {
            0x8326E718 => {
    //   block [0x8326E718..0x8326E750)
	// 8326E718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E71C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E720: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E724: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E728: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E72C: 386B94DC  addi r3, r11, -0x6b24
	ctx.r[3].s64 = ctx.r[11].s64 + -27428;
	// 8326E730: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E734: 4AF85625  bl 0x821f3d58
	ctx.lr = 0x8326E738;
	sub_821F3D58(ctx, base);
	// 8326E738: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E73C: 906ACC00  stw r3, -0x3400(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13312 as u32), ctx.r[3].u32 ) };
	// 8326E740: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E744: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E748: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E74C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E750 size=56
    let mut pc: u32 = 0x8326E750;
    'dispatch: loop {
        match pc {
            0x8326E750 => {
    //   block [0x8326E750..0x8326E788)
	// 8326E750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E758: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E75C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8326E760: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E764: 386B1034  addi r3, r11, 0x1034
	ctx.r[3].s64 = ctx.r[11].s64 + 4148;
	// 8326E768: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E76C: 4AF855ED  bl 0x821f3d58
	ctx.lr = 0x8326E770;
	sub_821F3D58(ctx, base);
	// 8326E770: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E774: 906ACC04  stw r3, -0x33fc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13308 as u32), ctx.r[3].u32 ) };
	// 8326E778: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E77C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E780: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E784: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E788 size=56
    let mut pc: u32 = 0x8326E788;
    'dispatch: loop {
        match pc {
            0x8326E788 => {
    //   block [0x8326E788..0x8326E7C0)
	// 8326E788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E78C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E790: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E794: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E798: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E79C: 386B94E4  addi r3, r11, -0x6b1c
	ctx.r[3].s64 = ctx.r[11].s64 + -27420;
	// 8326E7A0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E7A4: 4AF855B5  bl 0x821f3d58
	ctx.lr = 0x8326E7A8;
	sub_821F3D58(ctx, base);
	// 8326E7A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E7AC: 906ACC08  stw r3, -0x33f8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13304 as u32), ctx.r[3].u32 ) };
	// 8326E7B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E7B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E7B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E7BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E7C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E7C0 size=56
    let mut pc: u32 = 0x8326E7C0;
    'dispatch: loop {
        match pc {
            0x8326E7C0 => {
    //   block [0x8326E7C0..0x8326E7F8)
	// 8326E7C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E7C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E7C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E7CC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E7D0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E7D4: 386B94F4  addi r3, r11, -0x6b0c
	ctx.r[3].s64 = ctx.r[11].s64 + -27404;
	// 8326E7D8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E7DC: 4AF8557D  bl 0x821f3d58
	ctx.lr = 0x8326E7E0;
	sub_821F3D58(ctx, base);
	// 8326E7E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E7E4: 906ACC0C  stw r3, -0x33f4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13300 as u32), ctx.r[3].u32 ) };
	// 8326E7E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E7EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E7F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E7F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E7F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E7F8 size=56
    let mut pc: u32 = 0x8326E7F8;
    'dispatch: loop {
        match pc {
            0x8326E7F8 => {
    //   block [0x8326E7F8..0x8326E830)
	// 8326E7F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E7FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E800: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E804: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E808: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E80C: 386B9508  addi r3, r11, -0x6af8
	ctx.r[3].s64 = ctx.r[11].s64 + -27384;
	// 8326E810: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E814: 4AF85545  bl 0x821f3d58
	ctx.lr = 0x8326E818;
	sub_821F3D58(ctx, base);
	// 8326E818: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E81C: 906ACC10  stw r3, -0x33f0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13296 as u32), ctx.r[3].u32 ) };
	// 8326E820: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E824: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E828: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E82C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E830 size=56
    let mut pc: u32 = 0x8326E830;
    'dispatch: loop {
        match pc {
            0x8326E830 => {
    //   block [0x8326E830..0x8326E868)
	// 8326E830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E838: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E83C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E840: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E844: 386B9514  addi r3, r11, -0x6aec
	ctx.r[3].s64 = ctx.r[11].s64 + -27372;
	// 8326E848: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E84C: 4AF8550D  bl 0x821f3d58
	ctx.lr = 0x8326E850;
	sub_821F3D58(ctx, base);
	// 8326E850: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E854: 906ACC14  stw r3, -0x33ec(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13292 as u32), ctx.r[3].u32 ) };
	// 8326E858: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E85C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E860: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E864: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E868 size=56
    let mut pc: u32 = 0x8326E868;
    'dispatch: loop {
        match pc {
            0x8326E868 => {
    //   block [0x8326E868..0x8326E8A0)
	// 8326E868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E86C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E870: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E874: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E878: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E87C: 386B9520  addi r3, r11, -0x6ae0
	ctx.r[3].s64 = ctx.r[11].s64 + -27360;
	// 8326E880: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E884: 4AF854D5  bl 0x821f3d58
	ctx.lr = 0x8326E888;
	sub_821F3D58(ctx, base);
	// 8326E888: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E88C: 906ACC18  stw r3, -0x33e8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13288 as u32), ctx.r[3].u32 ) };
	// 8326E890: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E894: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E898: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E89C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E8A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E8A0 size=56
    let mut pc: u32 = 0x8326E8A0;
    'dispatch: loop {
        match pc {
            0x8326E8A0 => {
    //   block [0x8326E8A0..0x8326E8D8)
	// 8326E8A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E8A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E8A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E8AC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E8B0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E8B4: 386B9528  addi r3, r11, -0x6ad8
	ctx.r[3].s64 = ctx.r[11].s64 + -27352;
	// 8326E8B8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E8BC: 4AF8549D  bl 0x821f3d58
	ctx.lr = 0x8326E8C0;
	sub_821F3D58(ctx, base);
	// 8326E8C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E8C4: 906ACC1C  stw r3, -0x33e4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13284 as u32), ctx.r[3].u32 ) };
	// 8326E8C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E8CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E8D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E8D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E8D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E8D8 size=56
    let mut pc: u32 = 0x8326E8D8;
    'dispatch: loop {
        match pc {
            0x8326E8D8 => {
    //   block [0x8326E8D8..0x8326E910)
	// 8326E8D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E8DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E8E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E8E4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8326E8E8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E8EC: 386B2C84  addi r3, r11, 0x2c84
	ctx.r[3].s64 = ctx.r[11].s64 + 11396;
	// 8326E8F0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E8F4: 4AF85465  bl 0x821f3d58
	ctx.lr = 0x8326E8F8;
	sub_821F3D58(ctx, base);
	// 8326E8F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E8FC: 906ACC20  stw r3, -0x33e0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13280 as u32), ctx.r[3].u32 ) };
	// 8326E900: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E904: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E908: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E90C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E910 size=56
    let mut pc: u32 = 0x8326E910;
    'dispatch: loop {
        match pc {
            0x8326E910 => {
    //   block [0x8326E910..0x8326E948)
	// 8326E910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E918: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E91C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E920: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E924: 386B9608  addi r3, r11, -0x69f8
	ctx.r[3].s64 = ctx.r[11].s64 + -27128;
	// 8326E928: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E92C: 4AF8542D  bl 0x821f3d58
	ctx.lr = 0x8326E930;
	sub_821F3D58(ctx, base);
	// 8326E930: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E934: 906ACC24  stw r3, -0x33dc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13276 as u32), ctx.r[3].u32 ) };
	// 8326E938: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E93C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E940: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E944: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E948 size=56
    let mut pc: u32 = 0x8326E948;
    'dispatch: loop {
        match pc {
            0x8326E948 => {
    //   block [0x8326E948..0x8326E980)
	// 8326E948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E94C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E950: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E954: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E958: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E95C: 386B94DC  addi r3, r11, -0x6b24
	ctx.r[3].s64 = ctx.r[11].s64 + -27428;
	// 8326E960: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E964: 4AF853F5  bl 0x821f3d58
	ctx.lr = 0x8326E968;
	sub_821F3D58(ctx, base);
	// 8326E968: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E96C: 906ACC28  stw r3, -0x33d8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13272 as u32), ctx.r[3].u32 ) };
	// 8326E970: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E974: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E978: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E97C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E980 size=56
    let mut pc: u32 = 0x8326E980;
    'dispatch: loop {
        match pc {
            0x8326E980 => {
    //   block [0x8326E980..0x8326E9B8)
	// 8326E980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E988: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E98C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8326E990: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E994: 386B1034  addi r3, r11, 0x1034
	ctx.r[3].s64 = ctx.r[11].s64 + 4148;
	// 8326E998: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E99C: 4AF853BD  bl 0x821f3d58
	ctx.lr = 0x8326E9A0;
	sub_821F3D58(ctx, base);
	// 8326E9A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E9A4: 906ACC2C  stw r3, -0x33d4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13268 as u32), ctx.r[3].u32 ) };
	// 8326E9A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E9AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E9B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E9B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


