pub fn sub_826F3818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F3818 size=116
    let mut pc: u32 = 0x826F3818;
    'dispatch: loop {
        match pc {
            0x826F3818 => {
    //   block [0x826F3818..0x826F388C)
	// 826F3818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F381C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F3820: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F3824: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F3828: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F382C: 390BD1AC  addi r8, r11, -0x2e54
	ctx.r[8].s64 = ctx.r[11].s64 + -11860;
	// 826F3830: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F3834: 392A9F3C  addi r9, r10, -0x60c4
	ctx.r[9].s64 = ctx.r[10].s64 + -24772;
	// 826F3838: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F383C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826F3840: 38AA9364  addi r5, r10, -0x6c9c
	ctx.r[5].s64 = ctx.r[10].s64 + -27804;
	// 826F3844: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F3848: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F384C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F3850: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F3854: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F3858: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F385C: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 826F3860: 388A3BE8  addi r4, r10, 0x3be8
	ctx.r[4].s64 = ctx.r[10].s64 + 15336;
	// 826F3864: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F3868: 386B8D04  addi r3, r11, -0x72fc
	ctx.r[3].s64 = ctx.r[11].s64 + -29436;
	// 826F386C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F3870: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F3874: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F3878: 4BD735A9  bl 0x82466e20
	ctx.lr = 0x826F387C;
	sub_82466E20(ctx, base);
	// 826F387C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F3880: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F3884: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F3888: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F3890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F3890 size=112
    let mut pc: u32 = 0x826F3890;
    'dispatch: loop {
        match pc {
            0x826F3890 => {
    //   block [0x826F3890..0x826F3900)
	// 826F3890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F3894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F3898: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F389C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F38A0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F38A4: 38AA8D04  addi r5, r10, -0x72fc
	ctx.r[5].s64 = ctx.r[10].s64 + -29436;
	// 826F38A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F38AC: 390BD1C8  addi r8, r11, -0x2e38
	ctx.r[8].s64 = ctx.r[11].s64 + -11832;
	// 826F38B0: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826F38B4: 388A3BF8  addi r4, r10, 0x3bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 15352;
	// 826F38B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F38BC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F38C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F38C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F38C8: 386A8D34  addi r3, r10, -0x72cc
	ctx.r[3].s64 = ctx.r[10].s64 + -29388;
	// 826F38CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F38D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F38D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F38D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F38DC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F38E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F38E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F38E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F38EC: 4BD73535  bl 0x82466e20
	ctx.lr = 0x826F38F0;
	sub_82466E20(ctx, base);
	// 826F38F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F38F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F38F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F38FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F3900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F3900 size=112
    let mut pc: u32 = 0x826F3900;
    'dispatch: loop {
        match pc {
            0x826F3900 => {
    //   block [0x826F3900..0x826F3970)
	// 826F3900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F3904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F3908: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F390C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3910: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F3914: 38AA8CD4  addi r5, r10, -0x732c
	ctx.r[5].s64 = ctx.r[10].s64 + -29484;
	// 826F3918: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F391C: 390BD240  addi r8, r11, -0x2dc0
	ctx.r[8].s64 = ctx.r[11].s64 + -11712;
	// 826F3920: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826F3924: 388A3C0C  addi r4, r10, 0x3c0c
	ctx.r[4].s64 = ctx.r[10].s64 + 15372;
	// 826F3928: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F392C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3930: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F3934: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F3938: 386A8D64  addi r3, r10, -0x729c
	ctx.r[3].s64 = ctx.r[10].s64 + -29340;
	// 826F393C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F3940: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F3944: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F3948: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F394C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F3950: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F3954: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F3958: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F395C: 4BD734C5  bl 0x82466e20
	ctx.lr = 0x826F3960;
	sub_82466E20(ctx, base);
	// 826F3960: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F3964: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F3968: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F396C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F3970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F3970 size=112
    let mut pc: u32 = 0x826F3970;
    'dispatch: loop {
        match pc {
            0x826F3970 => {
    //   block [0x826F3970..0x826F39E0)
	// 826F3970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F3974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F3978: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F397C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3980: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F3984: 38AA8D04  addi r5, r10, -0x72fc
	ctx.r[5].s64 = ctx.r[10].s64 + -29436;
	// 826F3988: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F398C: 390BD288  addi r8, r11, -0x2d78
	ctx.r[8].s64 = ctx.r[11].s64 + -11640;
	// 826F3990: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826F3994: 388A3C24  addi r4, r10, 0x3c24
	ctx.r[4].s64 = ctx.r[10].s64 + 15396;
	// 826F3998: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F399C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F39A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F39A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F39A8: 386A8D94  addi r3, r10, -0x726c
	ctx.r[3].s64 = ctx.r[10].s64 + -29292;
	// 826F39AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F39B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F39B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F39B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F39BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F39C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F39C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F39C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F39CC: 4BD73455  bl 0x82466e20
	ctx.lr = 0x826F39D0;
	sub_82466E20(ctx, base);
	// 826F39D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F39D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F39D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F39DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F39E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F39E0 size=112
    let mut pc: u32 = 0x826F39E0;
    'dispatch: loop {
        match pc {
            0x826F39E0 => {
    //   block [0x826F39E0..0x826F3A50)
	// 826F39E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F39E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F39E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F39EC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F39F0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F39F4: 38AA8D04  addi r5, r10, -0x72fc
	ctx.r[5].s64 = ctx.r[10].s64 + -29436;
	// 826F39F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F39FC: 390BD2D0  addi r8, r11, -0x2d30
	ctx.r[8].s64 = ctx.r[11].s64 + -11568;
	// 826F3A00: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826F3A04: 388A3C3C  addi r4, r10, 0x3c3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15420;
	// 826F3A08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F3A0C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3A10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F3A14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F3A18: 386A8DC4  addi r3, r10, -0x723c
	ctx.r[3].s64 = ctx.r[10].s64 + -29244;
	// 826F3A1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F3A20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F3A24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F3A28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F3A2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F3A30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F3A34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F3A38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F3A3C: 4BD733E5  bl 0x82466e20
	ctx.lr = 0x826F3A40;
	sub_82466E20(ctx, base);
	// 826F3A40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F3A44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F3A48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F3A4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F3A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F3A50 size=108
    let mut pc: u32 = 0x826F3A50;
    'dispatch: loop {
        match pc {
            0x826F3A50 => {
    //   block [0x826F3A50..0x826F3ABC)
	// 826F3A50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F3A54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F3A58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F3A5C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F3A60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F3A64: 38EBD318  addi r7, r11, -0x2ce8
	ctx.r[7].s64 = ctx.r[11].s64 + -11496;
	// 826F3A68: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826F3A6C: 388A3C54  addi r4, r10, 0x3c54
	ctx.r[4].s64 = ctx.r[10].s64 + 15444;
	// 826F3A70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F3A74: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3A78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F3A7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F3A80: 386A8DF4  addi r3, r10, -0x720c
	ctx.r[3].s64 = ctx.r[10].s64 + -29196;
	// 826F3A84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F3A88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F3A8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F3A90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F3A94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F3A98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F3A9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F3AA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F3AA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F3AA8: 4BD73379  bl 0x82466e20
	ctx.lr = 0x826F3AAC;
	sub_82466E20(ctx, base);
	// 826F3AAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F3AB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F3AB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F3AB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F3AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F3AC0 size=112
    let mut pc: u32 = 0x826F3AC0;
    'dispatch: loop {
        match pc {
            0x826F3AC0 => {
    //   block [0x826F3AC0..0x826F3B30)
	// 826F3AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F3AC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F3AC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F3ACC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3AD0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F3AD4: 38AA8D04  addi r5, r10, -0x72fc
	ctx.r[5].s64 = ctx.r[10].s64 + -29436;
	// 826F3AD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F3ADC: 390BD360  addi r8, r11, -0x2ca0
	ctx.r[8].s64 = ctx.r[11].s64 + -11424;
	// 826F3AE0: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826F3AE4: 388A3C78  addi r4, r10, 0x3c78
	ctx.r[4].s64 = ctx.r[10].s64 + 15480;
	// 826F3AE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F3AEC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3AF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F3AF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F3AF8: 386A8E24  addi r3, r10, -0x71dc
	ctx.r[3].s64 = ctx.r[10].s64 + -29148;
	// 826F3AFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F3B00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F3B04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F3B08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F3B0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F3B10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F3B14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F3B18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F3B1C: 4BD73305  bl 0x82466e20
	ctx.lr = 0x826F3B20;
	sub_82466E20(ctx, base);
	// 826F3B20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F3B24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F3B28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F3B2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F3B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F3B30 size=116
    let mut pc: u32 = 0x826F3B30;
    'dispatch: loop {
        match pc {
            0x826F3B30 => {
    //   block [0x826F3B30..0x826F3BA4)
	// 826F3B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F3B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F3B38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F3B3C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826F3B40: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3B44: 392B9F78  addi r9, r11, -0x6088
	ctx.r[9].s64 = ctx.r[11].s64 + -24712;
	// 826F3B48: 38AA8D04  addi r5, r10, -0x72fc
	ctx.r[5].s64 = ctx.r[10].s64 + -29436;
	// 826F3B4C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F3B50: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826F3B54: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 826F3B58: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F3B5C: 388A3C90  addi r4, r10, 0x3c90
	ctx.r[4].s64 = ctx.r[10].s64 + 15504;
	// 826F3B60: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F3B64: 396BD3D8  addi r11, r11, -0x2c28
	ctx.r[11].s64 = ctx.r[11].s64 + -11304;
	// 826F3B68: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826F3B6C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3B70: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826F3B74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F3B78: 386A8E54  addi r3, r10, -0x71ac
	ctx.r[3].s64 = ctx.r[10].s64 + -29100;
	// 826F3B7C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F3B80: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826F3B84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F3B88: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826F3B8C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F3B90: 4BD73291  bl 0x82466e20
	ctx.lr = 0x826F3B94;
	sub_82466E20(ctx, base);
	// 826F3B94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F3B98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F3B9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F3BA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F3BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826F3BA8 size=36
    let mut pc: u32 = 0x826F3BA8;
    'dispatch: loop {
        match pc {
            0x826F3BA8 => {
    //   block [0x826F3BA8..0x826F3BCC)
	// 826F3BA8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F3BAC: 814BD470  lwz r10, -0x2b90(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-11152 as u32) ) } as u64;
	// 826F3BB0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F3BB4: 396B1BE0  addi r11, r11, 0x1be0
	ctx.r[11].s64 = ctx.r[11].s64 + 7136;
	// 826F3BB8: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 826F3BBC: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F3BC0: 814AD468  lwz r10, -0x2b98(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-11160 as u32) ) } as u64;
	// 826F3BC4: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 826F3BC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F3BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F3BD0 size=108
    let mut pc: u32 = 0x826F3BD0;
    'dispatch: loop {
        match pc {
            0x826F3BD0 => {
    //   block [0x826F3BD0..0x826F3C3C)
	// 826F3BD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F3BD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F3BD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F3BDC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F3BE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F3BE4: 38EB1BE0  addi r7, r11, 0x1be0
	ctx.r[7].s64 = ctx.r[11].s64 + 7136;
	// 826F3BE8: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826F3BEC: 388A3D1C  addi r4, r10, 0x3d1c
	ctx.r[4].s64 = ctx.r[10].s64 + 15644;
	// 826F3BF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F3BF4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3BF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F3BFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F3C00: 386A8E84  addi r3, r10, -0x717c
	ctx.r[3].s64 = ctx.r[10].s64 + -29052;
	// 826F3C04: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F3C08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F3C0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F3C10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F3C14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F3C18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F3C1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F3C20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F3C24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F3C28: 4BD731F9  bl 0x82466e20
	ctx.lr = 0x826F3C2C;
	sub_82466E20(ctx, base);
	// 826F3C2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F3C30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F3C34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F3C38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F3C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826F3C40 size=24
    let mut pc: u32 = 0x826F3C40;
    'dispatch: loop {
        match pc {
            0x826F3C40 => {
    //   block [0x826F3C40..0x826F3C58)
	// 826F3C40: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F3C44: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F3C48: 394A1C88  addi r10, r10, 0x1c88
	ctx.r[10].s64 = ctx.r[10].s64 + 7304;
	// 826F3C4C: 816BD468  lwz r11, -0x2b98(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-11160 as u32) ) } as u64;
	// 826F3C50: 916A00B0  stw r11, 0xb0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 826F3C54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F3C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F3C58 size=116
    let mut pc: u32 = 0x826F3C58;
    'dispatch: loop {
        match pc {
            0x826F3C58 => {
    //   block [0x826F3C58..0x826F3CCC)
	// 826F3C58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F3C5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F3C60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F3C64: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F3C68: 38E0000A  li r7, 0xa
	ctx.r[7].s64 = 10;
	// 826F3C6C: 390A1C88  addi r8, r10, 0x1c88
	ctx.r[8].s64 = ctx.r[10].s64 + 7304;
	// 826F3C70: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3C74: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826F3C78: 38AA8E84  addi r5, r10, -0x717c
	ctx.r[5].s64 = ctx.r[10].s64 + -29052;
	// 826F3C7C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F3C80: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F3C84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F3C88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F3C8C: 388A3D58  addi r4, r10, 0x3d58
	ctx.r[4].s64 = ctx.r[10].s64 + 15704;
	// 826F3C90: 396BA034  addi r11, r11, -0x5fcc
	ctx.r[11].s64 = ctx.r[11].s64 + -24524;
	// 826F3C94: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3C98: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F3C9C: 386A8EB4  addi r3, r10, -0x714c
	ctx.r[3].s64 = ctx.r[10].s64 + -29004;
	// 826F3CA0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826F3CA4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F3CA8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826F3CAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F3CB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F3CB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F3CB8: 4BD73169  bl 0x82466e20
	ctx.lr = 0x826F3CBC;
	sub_82466E20(ctx, base);
	// 826F3CBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F3CC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F3CC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F3CC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F3CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F3CD0 size=112
    let mut pc: u32 = 0x826F3CD0;
    'dispatch: loop {
        match pc {
            0x826F3CD0 => {
    //   block [0x826F3CD0..0x826F3D40)
	// 826F3CD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F3CD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F3CD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F3CDC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3CE0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F3CE4: 38AA8E84  addi r5, r10, -0x717c
	ctx.r[5].s64 = ctx.r[10].s64 + -29052;
	// 826F3CE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F3CEC: 390BD478  addi r8, r11, -0x2b88
	ctx.r[8].s64 = ctx.r[11].s64 + -11144;
	// 826F3CF0: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826F3CF4: 388A3D7C  addi r4, r10, 0x3d7c
	ctx.r[4].s64 = ctx.r[10].s64 + 15740;
	// 826F3CF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F3CFC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3D00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F3D04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F3D08: 386A8EE4  addi r3, r10, -0x711c
	ctx.r[3].s64 = ctx.r[10].s64 + -28956;
	// 826F3D0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F3D10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F3D14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F3D18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F3D1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F3D20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F3D24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F3D28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F3D2C: 4BD730F5  bl 0x82466e20
	ctx.lr = 0x826F3D30;
	sub_82466E20(ctx, base);
	// 826F3D30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F3D34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F3D38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F3D3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F3D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826F3D40 size=24
    let mut pc: u32 = 0x826F3D40;
    'dispatch: loop {
        match pc {
            0x826F3D40 => {
    //   block [0x826F3D40..0x826F3D58)
	// 826F3D40: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F3D44: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F3D48: 394A1D78  addi r10, r10, 0x1d78
	ctx.r[10].s64 = ctx.r[10].s64 + 7544;
	// 826F3D4C: 816BD5CC  lwz r11, -0x2a34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10804 as u32) ) } as u64;
	// 826F3D50: 916A00B0  stw r11, 0xb0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 826F3D54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F3D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F3D58 size=116
    let mut pc: u32 = 0x826F3D58;
    'dispatch: loop {
        match pc {
            0x826F3D58 => {
    //   block [0x826F3D58..0x826F3DCC)
	// 826F3D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F3D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F3D60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F3D64: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826F3D68: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3D6C: 392B9FF8  addi r9, r11, -0x6008
	ctx.r[9].s64 = ctx.r[11].s64 + -24584;
	// 826F3D70: 38AA8CD4  addi r5, r10, -0x732c
	ctx.r[5].s64 = ctx.r[10].s64 + -29484;
	// 826F3D74: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F3D78: 38E90068  addi r7, r9, 0x68
	ctx.r[7].s64 = ctx.r[9].s64 + 104;
	// 826F3D7C: 38C00009  li r6, 9
	ctx.r[6].s64 = 9;
	// 826F3D80: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F3D84: 388A3DA0  addi r4, r10, 0x3da0
	ctx.r[4].s64 = ctx.r[10].s64 + 15776;
	// 826F3D88: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F3D8C: 396B1D78  addi r11, r11, 0x1d78
	ctx.r[11].s64 = ctx.r[11].s64 + 7544;
	// 826F3D90: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826F3D94: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3D98: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826F3D9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F3DA0: 386A8F14  addi r3, r10, -0x70ec
	ctx.r[3].s64 = ctx.r[10].s64 + -28908;
	// 826F3DA4: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 826F3DA8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826F3DAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F3DB0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826F3DB4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F3DB8: 4BD73069  bl 0x82466e20
	ctx.lr = 0x826F3DBC;
	sub_82466E20(ctx, base);
	// 826F3DBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F3DC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F3DC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F3DC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F3DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F3DD0 size=100
    let mut pc: u32 = 0x826F3DD0;
    'dispatch: loop {
        match pc {
            0x826F3DD0 => {
    //   block [0x826F3DD0..0x826F3E34)
	// 826F3DD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F3DD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F3DD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F3DDC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3DE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F3DE4: 38AA9064  addi r5, r10, -0x6f9c
	ctx.r[5].s64 = ctx.r[10].s64 + -28572;
	// 826F3DE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F3DEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F3DF0: 388A3DB4  addi r4, r10, 0x3db4
	ctx.r[4].s64 = ctx.r[10].s64 + 15796;
	// 826F3DF4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3DF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F3DFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F3E00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F3E04: 386A8F44  addi r3, r10, -0x70bc
	ctx.r[3].s64 = ctx.r[10].s64 + -28860;
	// 826F3E08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F3E0C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F3E10: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F3E14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F3E18: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F3E1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F3E20: 4BD73001  bl 0x82466e20
	ctx.lr = 0x826F3E24;
	sub_82466E20(ctx, base);
	// 826F3E24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F3E28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F3E2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F3E30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F3E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F3E38 size=100
    let mut pc: u32 = 0x826F3E38;
    'dispatch: loop {
        match pc {
            0x826F3E38 => {
    //   block [0x826F3E38..0x826F3E9C)
	// 826F3E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F3E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F3E40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F3E44: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3E48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F3E4C: 38AA8B54  addi r5, r10, -0x74ac
	ctx.r[5].s64 = ctx.r[10].s64 + -29868;
	// 826F3E50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F3E54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F3E58: 388A3DC4  addi r4, r10, 0x3dc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15812;
	// 826F3E5C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3E60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F3E64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F3E68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F3E6C: 386A8F74  addi r3, r10, -0x708c
	ctx.r[3].s64 = ctx.r[10].s64 + -28812;
	// 826F3E70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F3E74: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F3E78: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F3E7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F3E80: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F3E84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F3E88: 4BD72F99  bl 0x82466e20
	ctx.lr = 0x826F3E8C;
	sub_82466E20(ctx, base);
	// 826F3E8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F3E90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F3E94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F3E98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F3EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F3EA0 size=108
    let mut pc: u32 = 0x826F3EA0;
    'dispatch: loop {
        match pc {
            0x826F3EA0 => {
    //   block [0x826F3EA0..0x826F3F0C)
	// 826F3EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F3EA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F3EA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F3EAC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F3EB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F3EB4: 38EBD4F0  addi r7, r11, -0x2b10
	ctx.r[7].s64 = ctx.r[11].s64 + -11024;
	// 826F3EB8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826F3EBC: 388A3DD8  addi r4, r10, 0x3dd8
	ctx.r[4].s64 = ctx.r[10].s64 + 15832;
	// 826F3EC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F3EC4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3EC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F3ECC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F3ED0: 386A8FA4  addi r3, r10, -0x705c
	ctx.r[3].s64 = ctx.r[10].s64 + -28764;
	// 826F3ED4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F3ED8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F3EDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F3EE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F3EE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F3EE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F3EEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F3EF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F3EF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F3EF8: 4BD72F29  bl 0x82466e20
	ctx.lr = 0x826F3EFC;
	sub_82466E20(ctx, base);
	// 826F3EFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F3F00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F3F04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F3F08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F3F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F3F10 size=112
    let mut pc: u32 = 0x826F3F10;
    'dispatch: loop {
        match pc {
            0x826F3F10 => {
    //   block [0x826F3F10..0x826F3F80)
	// 826F3F10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F3F14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F3F18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F3F1C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3F20: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F3F24: 38AA8CD4  addi r5, r10, -0x732c
	ctx.r[5].s64 = ctx.r[10].s64 + -29484;
	// 826F3F28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F3F2C: 390BD550  addi r8, r11, -0x2ab0
	ctx.r[8].s64 = ctx.r[11].s64 + -10928;
	// 826F3F30: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826F3F34: 388A3DF0  addi r4, r10, 0x3df0
	ctx.r[4].s64 = ctx.r[10].s64 + 15856;
	// 826F3F38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F3F3C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3F40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F3F44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F3F48: 386A8FD4  addi r3, r10, -0x702c
	ctx.r[3].s64 = ctx.r[10].s64 + -28716;
	// 826F3F4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F3F50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F3F54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F3F58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F3F5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F3F60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F3F64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F3F68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F3F6C: 4BD72EB5  bl 0x82466e20
	ctx.lr = 0x826F3F70;
	sub_82466E20(ctx, base);
	// 826F3F70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F3F74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F3F78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F3F7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F3F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F3F80 size=108
    let mut pc: u32 = 0x826F3F80;
    'dispatch: loop {
        match pc {
            0x826F3F80 => {
    //   block [0x826F3F80..0x826F3FEC)
	// 826F3F80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F3F84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F3F88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F3F8C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F3F90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F3F94: 38EBD5B0  addi r7, r11, -0x2a50
	ctx.r[7].s64 = ctx.r[11].s64 + -10832;
	// 826F3F98: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F3F9C: 388A3DFC  addi r4, r10, 0x3dfc
	ctx.r[4].s64 = ctx.r[10].s64 + 15868;
	// 826F3FA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F3FA4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3FA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F3FAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F3FB0: 386A9004  addi r3, r10, -0x6ffc
	ctx.r[3].s64 = ctx.r[10].s64 + -28668;
	// 826F3FB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F3FB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F3FBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F3FC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F3FC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F3FC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F3FCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F3FD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F3FD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F3FD8: 4BD72E49  bl 0x82466e20
	ctx.lr = 0x826F3FDC;
	sub_82466E20(ctx, base);
	// 826F3FDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F3FE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F3FE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F3FE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F3FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826F3FF0 size=28
    let mut pc: u32 = 0x826F3FF0;
    'dispatch: loop {
        match pc {
            0x826F3FF0 => {
    //   block [0x826F3FF0..0x826F400C)
	// 826F3FF0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F3FF4: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F3FF8: 394A1E50  addi r10, r10, 0x1e50
	ctx.r[10].s64 = ctx.r[10].s64 + 7760;
	// 826F3FFC: 816BD474  lwz r11, -0x2b8c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-11148 as u32) ) } as u64;
	// 826F4000: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826F4004: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 826F4008: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F4010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F4010 size=112
    let mut pc: u32 = 0x826F4010;
    'dispatch: loop {
        match pc {
            0x826F4010 => {
    //   block [0x826F4010..0x826F4080)
	// 826F4010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F4014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F4018: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F401C: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F4020: 3900000F  li r8, 0xf
	ctx.r[8].s64 = 15;
	// 826F4024: 38EA1E50  addi r7, r10, 0x1e50
	ctx.r[7].s64 = ctx.r[10].s64 + 7760;
	// 826F4028: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F402C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826F4030: 388A3E1C  addi r4, r10, 0x3e1c
	ctx.r[4].s64 = ctx.r[10].s64 + 15900;
	// 826F4034: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F4038: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F403C: 396BA0F0  addi r11, r11, -0x5f10
	ctx.r[11].s64 = ctx.r[11].s64 + -24336;
	// 826F4040: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F4044: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4048: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F404C: 386A9034  addi r3, r10, -0x6fcc
	ctx.r[3].s64 = ctx.r[10].s64 + -28620;
	// 826F4050: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F4054: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826F4058: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F405C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826F4060: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F4064: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F4068: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F406C: 4BD72DB5  bl 0x82466e20
	ctx.lr = 0x826F4070;
	sub_82466E20(ctx, base);
	// 826F4070: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F4074: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F4078: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F407C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F4080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826F4080 size=24
    let mut pc: u32 = 0x826F4080;
    'dispatch: loop {
        match pc {
            0x826F4080 => {
    //   block [0x826F4080..0x826F4098)
	// 826F4080: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F4084: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F4088: 394A1FB8  addi r10, r10, 0x1fb8
	ctx.r[10].s64 = ctx.r[10].s64 + 8120;
	// 826F408C: 816BD5CC  lwz r11, -0x2a34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10804 as u32) ) } as u64;
	// 826F4090: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826F4094: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F4098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F4098 size=116
    let mut pc: u32 = 0x826F4098;
    'dispatch: loop {
        match pc {
            0x826F4098 => {
    //   block [0x826F4098..0x826F410C)
	// 826F4098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F409C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F40A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F40A4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826F40A8: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F40AC: 392BA0C8  addi r9, r11, -0x5f38
	ctx.r[9].s64 = ctx.r[11].s64 + -24376;
	// 826F40B0: 38AA8CD4  addi r5, r10, -0x732c
	ctx.r[5].s64 = ctx.r[10].s64 + -29484;
	// 826F40B4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F40B8: 38E90068  addi r7, r9, 0x68
	ctx.r[7].s64 = ctx.r[9].s64 + 104;
	// 826F40BC: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 826F40C0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F40C4: 388A3E30  addi r4, r10, 0x3e30
	ctx.r[4].s64 = ctx.r[10].s64 + 15920;
	// 826F40C8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F40CC: 396B1FB8  addi r11, r11, 0x1fb8
	ctx.r[11].s64 = ctx.r[11].s64 + 8120;
	// 826F40D0: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826F40D4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F40D8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826F40DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F40E0: 386A9064  addi r3, r10, -0x6f9c
	ctx.r[3].s64 = ctx.r[10].s64 + -28572;
	// 826F40E4: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826F40E8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826F40EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F40F0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826F40F4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F40F8: 4BD72D29  bl 0x82466e20
	ctx.lr = 0x826F40FC;
	sub_82466E20(ctx, base);
	// 826F40FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F4100: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F4104: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F4108: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F4110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F4110 size=112
    let mut pc: u32 = 0x826F4110;
    'dispatch: loop {
        match pc {
            0x826F4110 => {
    //   block [0x826F4110..0x826F4180)
	// 826F4110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F4114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F4118: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F411C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4120: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F4124: 38AA8C74  addi r5, r10, -0x738c
	ctx.r[5].s64 = ctx.r[10].s64 + -29580;
	// 826F4128: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F412C: 390BD5D0  addi r8, r11, -0x2a30
	ctx.r[8].s64 = ctx.r[11].s64 + -10800;
	// 826F4130: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826F4134: 388A3E3C  addi r4, r10, 0x3e3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15932;
	// 826F4138: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F413C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4140: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F4144: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F4148: 386A9094  addi r3, r10, -0x6f6c
	ctx.r[3].s64 = ctx.r[10].s64 + -28524;
	// 826F414C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F4150: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F4154: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F4158: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F415C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F4160: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F4164: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F4168: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F416C: 4BD72CB5  bl 0x82466e20
	ctx.lr = 0x826F4170;
	sub_82466E20(ctx, base);
	// 826F4170: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F4174: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F4178: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F417C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F4180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826F4180 size=24
    let mut pc: u32 = 0x826F4180;
    'dispatch: loop {
        match pc {
            0x826F4180 => {
    //   block [0x826F4180..0x826F4198)
	// 826F4180: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F4184: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F4188: 394A2060  addi r10, r10, 0x2060
	ctx.r[10].s64 = ctx.r[10].s64 + 8288;
	// 826F418C: 816BD5CC  lwz r11, -0x2a34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10804 as u32) ) } as u64;
	// 826F4190: 916A00C8  stw r11, 0xc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 826F4194: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F4198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F4198 size=116
    let mut pc: u32 = 0x826F4198;
    'dispatch: loop {
        match pc {
            0x826F4198 => {
    //   block [0x826F4198..0x826F420C)
	// 826F4198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F419C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F41A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F41A4: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F41A8: 38E0000D  li r7, 0xd
	ctx.r[7].s64 = 13;
	// 826F41AC: 390A2060  addi r8, r10, 0x2060
	ctx.r[8].s64 = ctx.r[10].s64 + 8288;
	// 826F41B0: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F41B4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826F41B8: 38AA8C74  addi r5, r10, -0x738c
	ctx.r[5].s64 = ctx.r[10].s64 + -29580;
	// 826F41BC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F41C0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F41C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F41C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F41CC: 388A3E64  addi r4, r10, 0x3e64
	ctx.r[4].s64 = ctx.r[10].s64 + 15972;
	// 826F41D0: 396BA150  addi r11, r11, -0x5eb0
	ctx.r[11].s64 = ctx.r[11].s64 + -24240;
	// 826F41D4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F41D8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F41DC: 386A90C4  addi r3, r10, -0x6f3c
	ctx.r[3].s64 = ctx.r[10].s64 + -28476;
	// 826F41E0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826F41E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F41E8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826F41EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F41F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F41F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F41F8: 4BD72C29  bl 0x82466e20
	ctx.lr = 0x826F41FC;
	sub_82466E20(ctx, base);
	// 826F41FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F4200: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F4204: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F4208: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F4210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F4210 size=112
    let mut pc: u32 = 0x826F4210;
    'dispatch: loop {
        match pc {
            0x826F4210 => {
    //   block [0x826F4210..0x826F4280)
	// 826F4210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F4214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F4218: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F421C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F4220: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F4224: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F4228: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F422C: 390BD660  addi r8, r11, -0x29a0
	ctx.r[8].s64 = ctx.r[11].s64 + -10656;
	// 826F4230: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826F4234: 388A3E78  addi r4, r10, 0x3e78
	ctx.r[4].s64 = ctx.r[10].s64 + 15992;
	// 826F4238: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F423C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4240: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F4244: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F4248: 386A90F4  addi r3, r10, -0x6f0c
	ctx.r[3].s64 = ctx.r[10].s64 + -28428;
	// 826F424C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F4250: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F4254: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F4258: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F425C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F4260: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F4264: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F4268: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F426C: 4BD72BB5  bl 0x82466e20
	ctx.lr = 0x826F4270;
	sub_82466E20(ctx, base);
	// 826F4270: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F4274: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F4278: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F427C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F4280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F4280 size=108
    let mut pc: u32 = 0x826F4280;
    'dispatch: loop {
        match pc {
            0x826F4280 => {
    //   block [0x826F4280..0x826F42EC)
	// 826F4280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F4284: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F4288: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F428C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F4290: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F4294: 38EBD690  addi r7, r11, -0x2970
	ctx.r[7].s64 = ctx.r[11].s64 + -10608;
	// 826F4298: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F429C: 388A3E98  addi r4, r10, 0x3e98
	ctx.r[4].s64 = ctx.r[10].s64 + 16024;
	// 826F42A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F42A4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F42A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F42AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F42B0: 386A9124  addi r3, r10, -0x6edc
	ctx.r[3].s64 = ctx.r[10].s64 + -28380;
	// 826F42B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F42B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F42BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F42C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F42C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F42C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F42CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F42D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F42D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F42D8: 4BD72B49  bl 0x82466e20
	ctx.lr = 0x826F42DC;
	sub_82466E20(ctx, base);
	// 826F42DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F42E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F42E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F42E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F42F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F42F0 size=112
    let mut pc: u32 = 0x826F42F0;
    'dispatch: loop {
        match pc {
            0x826F42F0 => {
    //   block [0x826F42F0..0x826F4360)
	// 826F42F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F42F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F42F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F42FC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4300: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F4304: 38AA8B54  addi r5, r10, -0x74ac
	ctx.r[5].s64 = ctx.r[10].s64 + -29868;
	// 826F4308: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F430C: 390BD6C0  addi r8, r11, -0x2940
	ctx.r[8].s64 = ctx.r[11].s64 + -10560;
	// 826F4310: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826F4314: 388A3EAC  addi r4, r10, 0x3eac
	ctx.r[4].s64 = ctx.r[10].s64 + 16044;
	// 826F4318: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F431C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4320: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F4324: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F4328: 386A9154  addi r3, r10, -0x6eac
	ctx.r[3].s64 = ctx.r[10].s64 + -28332;
	// 826F432C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F4330: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F4334: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F4338: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F433C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F4340: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F4344: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F4348: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F434C: 4BD72AD5  bl 0x82466e20
	ctx.lr = 0x826F4350;
	sub_82466E20(ctx, base);
	// 826F4350: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F4354: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F4358: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F435C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F4360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F4360 size=112
    let mut pc: u32 = 0x826F4360;
    'dispatch: loop {
        match pc {
            0x826F4360 => {
    //   block [0x826F4360..0x826F43D0)
	// 826F4360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F4364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F4368: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F436C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4370: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F4374: 38AA9364  addi r5, r10, -0x6c9c
	ctx.r[5].s64 = ctx.r[10].s64 + -27804;
	// 826F4378: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F437C: 390BD6F0  addi r8, r11, -0x2910
	ctx.r[8].s64 = ctx.r[11].s64 + -10512;
	// 826F4380: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826F4384: 388A3EBC  addi r4, r10, 0x3ebc
	ctx.r[4].s64 = ctx.r[10].s64 + 16060;
	// 826F4388: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F438C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4390: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F4394: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F4398: 386A9184  addi r3, r10, -0x6e7c
	ctx.r[3].s64 = ctx.r[10].s64 + -28284;
	// 826F439C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F43A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F43A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F43A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F43AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F43B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F43B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F43B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F43BC: 4BD72A65  bl 0x82466e20
	ctx.lr = 0x826F43C0;
	sub_82466E20(ctx, base);
	// 826F43C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F43C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F43C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F43CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F43D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F43D0 size=108
    let mut pc: u32 = 0x826F43D0;
    'dispatch: loop {
        match pc {
            0x826F43D0 => {
    //   block [0x826F43D0..0x826F443C)
	// 826F43D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F43D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F43D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F43DC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F43E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F43E4: 38EBD720  addi r7, r11, -0x28e0
	ctx.r[7].s64 = ctx.r[11].s64 + -10464;
	// 826F43E8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826F43EC: 388A3ED0  addi r4, r10, 0x3ed0
	ctx.r[4].s64 = ctx.r[10].s64 + 16080;
	// 826F43F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F43F4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F43F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F43FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F4400: 386A91B4  addi r3, r10, -0x6e4c
	ctx.r[3].s64 = ctx.r[10].s64 + -28236;
	// 826F4404: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F4408: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F440C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F4410: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F4414: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F4418: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F441C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F4420: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F4424: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F4428: 4BD729F9  bl 0x82466e20
	ctx.lr = 0x826F442C;
	sub_82466E20(ctx, base);
	// 826F442C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F4430: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F4434: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F4438: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F4440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F4440 size=108
    let mut pc: u32 = 0x826F4440;
    'dispatch: loop {
        match pc {
            0x826F4440 => {
    //   block [0x826F4440..0x826F44AC)
	// 826F4440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F4444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F4448: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F444C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F4450: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826F4454: 38EBD768  addi r7, r11, -0x2898
	ctx.r[7].s64 = ctx.r[11].s64 + -10392;
	// 826F4458: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826F445C: 388A0DDC  addi r4, r10, 0xddc
	ctx.r[4].s64 = ctx.r[10].s64 + 3548;
	// 826F4460: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F4464: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4468: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F446C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F4470: 386A91E4  addi r3, r10, -0x6e1c
	ctx.r[3].s64 = ctx.r[10].s64 + -28188;
	// 826F4474: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F4478: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F447C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F4480: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F4484: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F4488: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F448C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F4490: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F4494: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F4498: 4BD72989  bl 0x82466e20
	ctx.lr = 0x826F449C;
	sub_82466E20(ctx, base);
	// 826F449C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F44A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F44A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F44A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F44B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F44B0 size=112
    let mut pc: u32 = 0x826F44B0;
    'dispatch: loop {
        match pc {
            0x826F44B0 => {
    //   block [0x826F44B0..0x826F4520)
	// 826F44B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F44B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F44B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F44BC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F44C0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F44C4: 38AA8D04  addi r5, r10, -0x72fc
	ctx.r[5].s64 = ctx.r[10].s64 + -29436;
	// 826F44C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F44CC: 390BD7C8  addi r8, r11, -0x2838
	ctx.r[8].s64 = ctx.r[11].s64 + -10296;
	// 826F44D0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826F44D4: 388A3EF8  addi r4, r10, 0x3ef8
	ctx.r[4].s64 = ctx.r[10].s64 + 16120;
	// 826F44D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F44DC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F44E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F44E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F44E8: 386A9214  addi r3, r10, -0x6dec
	ctx.r[3].s64 = ctx.r[10].s64 + -28140;
	// 826F44EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F44F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F44F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F44F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F44FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F4500: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F4504: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F4508: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F450C: 4BD72915  bl 0x82466e20
	ctx.lr = 0x826F4510;
	sub_82466E20(ctx, base);
	// 826F4510: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F4514: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F4518: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F451C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F4520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F4520 size=100
    let mut pc: u32 = 0x826F4520;
    'dispatch: loop {
        match pc {
            0x826F4520 => {
    //   block [0x826F4520..0x826F4584)
	// 826F4520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F4524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F4528: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F452C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4530: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F4534: 38AA8B54  addi r5, r10, -0x74ac
	ctx.r[5].s64 = ctx.r[10].s64 + -29868;
	// 826F4538: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F453C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F4540: 388A3F14  addi r4, r10, 0x3f14
	ctx.r[4].s64 = ctx.r[10].s64 + 16148;
	// 826F4544: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4548: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F454C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F4550: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F4554: 386A9244  addi r3, r10, -0x6dbc
	ctx.r[3].s64 = ctx.r[10].s64 + -28092;
	// 826F4558: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F455C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F4560: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F4564: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F4568: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F456C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F4570: 4BD728B1  bl 0x82466e20
	ctx.lr = 0x826F4574;
	sub_82466E20(ctx, base);
	// 826F4574: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F4578: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F457C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F4580: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F4588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F4588 size=112
    let mut pc: u32 = 0x826F4588;
    'dispatch: loop {
        match pc {
            0x826F4588 => {
    //   block [0x826F4588..0x826F45F8)
	// 826F4588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F458C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F4590: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F4594: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4598: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F459C: 38AA8F74  addi r5, r10, -0x708c
	ctx.r[5].s64 = ctx.r[10].s64 + -28812;
	// 826F45A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F45A4: 390BD828  addi r8, r11, -0x27d8
	ctx.r[8].s64 = ctx.r[11].s64 + -10200;
	// 826F45A8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826F45AC: 388A3F2C  addi r4, r10, 0x3f2c
	ctx.r[4].s64 = ctx.r[10].s64 + 16172;
	// 826F45B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F45B4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F45B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F45BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F45C0: 386A9274  addi r3, r10, -0x6d8c
	ctx.r[3].s64 = ctx.r[10].s64 + -28044;
	// 826F45C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F45C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F45CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F45D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F45D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F45D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F45DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F45E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F45E4: 4BD7283D  bl 0x82466e20
	ctx.lr = 0x826F45E8;
	sub_82466E20(ctx, base);
	// 826F45E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F45EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F45F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F45F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F45F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F45F8 size=112
    let mut pc: u32 = 0x826F45F8;
    'dispatch: loop {
        match pc {
            0x826F45F8 => {
    //   block [0x826F45F8..0x826F4668)
	// 826F45F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F45FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F4600: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F4604: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4608: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F460C: 38AA8F74  addi r5, r10, -0x708c
	ctx.r[5].s64 = ctx.r[10].s64 + -28812;
	// 826F4610: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F4614: 390BD870  addi r8, r11, -0x2790
	ctx.r[8].s64 = ctx.r[11].s64 + -10128;
	// 826F4618: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826F461C: 388A3F3C  addi r4, r10, 0x3f3c
	ctx.r[4].s64 = ctx.r[10].s64 + 16188;
	// 826F4620: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F4624: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4628: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F462C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F4630: 386A92A4  addi r3, r10, -0x6d5c
	ctx.r[3].s64 = ctx.r[10].s64 + -27996;
	// 826F4634: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F4638: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F463C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F4640: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F4644: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F4648: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F464C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F4650: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F4654: 4BD727CD  bl 0x82466e20
	ctx.lr = 0x826F4658;
	sub_82466E20(ctx, base);
	// 826F4658: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F465C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F4660: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F4664: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F4668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F4668 size=108
    let mut pc: u32 = 0x826F4668;
    'dispatch: loop {
        match pc {
            0x826F4668 => {
    //   block [0x826F4668..0x826F46D4)
	// 826F4668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F466C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F4670: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F4674: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F4678: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F467C: 38EBD918  addi r7, r11, -0x26e8
	ctx.r[7].s64 = ctx.r[11].s64 + -9960;
	// 826F4680: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826F4684: 388A3F58  addi r4, r10, 0x3f58
	ctx.r[4].s64 = ctx.r[10].s64 + 16216;
	// 826F4688: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F468C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4690: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F4694: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F4698: 386A92D4  addi r3, r10, -0x6d2c
	ctx.r[3].s64 = ctx.r[10].s64 + -27948;
	// 826F469C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F46A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F46A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F46A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F46AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F46B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F46B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F46B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F46BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F46C0: 4BD72761  bl 0x82466e20
	ctx.lr = 0x826F46C4;
	sub_82466E20(ctx, base);
	// 826F46C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F46C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F46CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F46D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F46D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826F46D8 size=24
    let mut pc: u32 = 0x826F46D8;
    'dispatch: loop {
        match pc {
            0x826F46D8 => {
    //   block [0x826F46D8..0x826F46F0)
	// 826F46D8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F46DC: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F46E0: 394A2198  addi r10, r10, 0x2198
	ctx.r[10].s64 = ctx.r[10].s64 + 8600;
	// 826F46E4: 816BD5CC  lwz r11, -0x2a34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10804 as u32) ) } as u64;
	// 826F46E8: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826F46EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F46F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F46F0 size=116
    let mut pc: u32 = 0x826F46F0;
    'dispatch: loop {
        match pc {
            0x826F46F0 => {
    //   block [0x826F46F0..0x826F4764)
	// 826F46F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F46F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F46F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F46FC: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F4700: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826F4704: 390A2198  addi r8, r10, 0x2198
	ctx.r[8].s64 = ctx.r[10].s64 + 8600;
	// 826F4708: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F470C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826F4710: 38AA8CD4  addi r5, r10, -0x732c
	ctx.r[5].s64 = ctx.r[10].s64 + -29484;
	// 826F4714: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F4718: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F471C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F4720: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F4724: 388A3F74  addi r4, r10, 0x3f74
	ctx.r[4].s64 = ctx.r[10].s64 + 16244;
	// 826F4728: 396BA188  addi r11, r11, -0x5e78
	ctx.r[11].s64 = ctx.r[11].s64 + -24184;
	// 826F472C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4730: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F4734: 386A9304  addi r3, r10, -0x6cfc
	ctx.r[3].s64 = ctx.r[10].s64 + -27900;
	// 826F4738: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826F473C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F4740: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826F4744: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F4748: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F474C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F4750: 4BD726D1  bl 0x82466e20
	ctx.lr = 0x826F4754;
	sub_82466E20(ctx, base);
	// 826F4754: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F4758: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F475C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F4760: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F4768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F4768 size=100
    let mut pc: u32 = 0x826F4768;
    'dispatch: loop {
        match pc {
            0x826F4768 => {
    //   block [0x826F4768..0x826F47CC)
	// 826F4768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F476C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F4770: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F4774: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4778: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F477C: 38AA8D04  addi r5, r10, -0x72fc
	ctx.r[5].s64 = ctx.r[10].s64 + -29436;
	// 826F4780: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F4784: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F4788: 388A3F88  addi r4, r10, 0x3f88
	ctx.r[4].s64 = ctx.r[10].s64 + 16264;
	// 826F478C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4790: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F4794: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F4798: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F479C: 386A9334  addi r3, r10, -0x6ccc
	ctx.r[3].s64 = ctx.r[10].s64 + -27852;
	// 826F47A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F47A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F47A8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F47AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F47B0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F47B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F47B8: 4BD72669  bl 0x82466e20
	ctx.lr = 0x826F47BC;
	sub_82466E20(ctx, base);
	// 826F47BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F47C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F47C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F47C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F47D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F47D0 size=100
    let mut pc: u32 = 0x826F47D0;
    'dispatch: loop {
        match pc {
            0x826F47D0 => {
    //   block [0x826F47D0..0x826F4834)
	// 826F47D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F47D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F47D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F47DC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F47E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F47E4: 38AA8B54  addi r5, r10, -0x74ac
	ctx.r[5].s64 = ctx.r[10].s64 + -29868;
	// 826F47E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F47EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F47F0: 388A3F98  addi r4, r10, 0x3f98
	ctx.r[4].s64 = ctx.r[10].s64 + 16280;
	// 826F47F4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F47F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F47FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F4800: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F4804: 386A9364  addi r3, r10, -0x6c9c
	ctx.r[3].s64 = ctx.r[10].s64 + -27804;
	// 826F4808: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F480C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F4810: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F4814: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F4818: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F481C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F4820: 4BD72601  bl 0x82466e20
	ctx.lr = 0x826F4824;
	sub_82466E20(ctx, base);
	// 826F4824: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F4828: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F482C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F4830: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F4838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F4838 size=112
    let mut pc: u32 = 0x826F4838;
    'dispatch: loop {
        match pc {
            0x826F4838 => {
    //   block [0x826F4838..0x826F48A8)
	// 826F4838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F483C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F4840: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F4844: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F4848: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F484C: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F4850: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F4854: 390BD978  addi r8, r11, -0x2688
	ctx.r[8].s64 = ctx.r[11].s64 + -9864;
	// 826F4858: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826F485C: 388A3FAC  addi r4, r10, 0x3fac
	ctx.r[4].s64 = ctx.r[10].s64 + 16300;
	// 826F4860: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F4864: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4868: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F486C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F4870: 386A9394  addi r3, r10, -0x6c6c
	ctx.r[3].s64 = ctx.r[10].s64 + -27756;
	// 826F4874: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F4878: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F487C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F4880: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F4884: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F4888: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F488C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F4890: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F4894: 4BD7258D  bl 0x82466e20
	ctx.lr = 0x826F4898;
	sub_82466E20(ctx, base);
	// 826F4898: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F489C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F48A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F48A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F48A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F48A8 size=112
    let mut pc: u32 = 0x826F48A8;
    'dispatch: loop {
        match pc {
            0x826F48A8 => {
    //   block [0x826F48A8..0x826F4918)
	// 826F48A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F48AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F48B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F48B4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F48B8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F48BC: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F48C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F48C4: 390BDA08  addi r8, r11, -0x25f8
	ctx.r[8].s64 = ctx.r[11].s64 + -9720;
	// 826F48C8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826F48CC: 388A3FDC  addi r4, r10, 0x3fdc
	ctx.r[4].s64 = ctx.r[10].s64 + 16348;
	// 826F48D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F48D4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F48D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F48DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F48E0: 386A93C4  addi r3, r10, -0x6c3c
	ctx.r[3].s64 = ctx.r[10].s64 + -27708;
	// 826F48E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F48E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F48EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F48F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F48F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F48F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F48FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F4900: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F4904: 4BD7251D  bl 0x82466e20
	ctx.lr = 0x826F4908;
	sub_82466E20(ctx, base);
	// 826F4908: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F490C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F4910: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F4914: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F4918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F4918 size=112
    let mut pc: u32 = 0x826F4918;
    'dispatch: loop {
        match pc {
            0x826F4918 => {
    //   block [0x826F4918..0x826F4988)
	// 826F4918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F491C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F4920: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F4924: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4928: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F492C: 38AA8F14  addi r5, r10, -0x70ec
	ctx.r[5].s64 = ctx.r[10].s64 + -28908;
	// 826F4930: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F4934: 390BDA68  addi r8, r11, -0x2598
	ctx.r[8].s64 = ctx.r[11].s64 + -9624;
	// 826F4938: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826F493C: 388A400C  addi r4, r10, 0x400c
	ctx.r[4].s64 = ctx.r[10].s64 + 16396;
	// 826F4940: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F4944: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4948: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F494C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F4950: 386A93F4  addi r3, r10, -0x6c0c
	ctx.r[3].s64 = ctx.r[10].s64 + -27660;
	// 826F4954: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F4958: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F495C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F4960: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F4964: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F4968: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F496C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F4970: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F4974: 4BD724AD  bl 0x82466e20
	ctx.lr = 0x826F4978;
	sub_82466E20(ctx, base);
	// 826F4978: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F497C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F4980: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F4984: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F4988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F4988 size=112
    let mut pc: u32 = 0x826F4988;
    'dispatch: loop {
        match pc {
            0x826F4988 => {
    //   block [0x826F4988..0x826F49F8)
	// 826F4988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F498C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F4990: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F4994: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F4998: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F499C: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F49A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F49A4: 390BDA98  addi r8, r11, -0x2568
	ctx.r[8].s64 = ctx.r[11].s64 + -9576;
	// 826F49A8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826F49AC: 388A4028  addi r4, r10, 0x4028
	ctx.r[4].s64 = ctx.r[10].s64 + 16424;
	// 826F49B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F49B4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F49B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F49BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F49C0: 386A9424  addi r3, r10, -0x6bdc
	ctx.r[3].s64 = ctx.r[10].s64 + -27612;
	// 826F49C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F49C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F49CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F49D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F49D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F49D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F49DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F49E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F49E4: 4BD7243D  bl 0x82466e20
	ctx.lr = 0x826F49E8;
	sub_82466E20(ctx, base);
	// 826F49E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F49EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F49F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F49F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F49F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F49F8 size=112
    let mut pc: u32 = 0x826F49F8;
    'dispatch: loop {
        match pc {
            0x826F49F8 => {
    //   block [0x826F49F8..0x826F4A68)
	// 826F49F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F49FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F4A00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F4A04: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4A08: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F4A0C: 38AA9064  addi r5, r10, -0x6f9c
	ctx.r[5].s64 = ctx.r[10].s64 + -28572;
	// 826F4A10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F4A14: 390BDB28  addi r8, r11, -0x24d8
	ctx.r[8].s64 = ctx.r[11].s64 + -9432;
	// 826F4A18: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F4A1C: 388A404C  addi r4, r10, 0x404c
	ctx.r[4].s64 = ctx.r[10].s64 + 16460;
	// 826F4A20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F4A24: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4A28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F4A2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F4A30: 386A9454  addi r3, r10, -0x6bac
	ctx.r[3].s64 = ctx.r[10].s64 + -27564;
	// 826F4A34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F4A38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F4A3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F4A40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F4A44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F4A48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F4A4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F4A50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F4A54: 4BD723CD  bl 0x82466e20
	ctx.lr = 0x826F4A58;
	sub_82466E20(ctx, base);
	// 826F4A58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F4A5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F4A60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F4A64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F4A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F4A68 size=112
    let mut pc: u32 = 0x826F4A68;
    'dispatch: loop {
        match pc {
            0x826F4A68 => {
    //   block [0x826F4A68..0x826F4AD8)
	// 826F4A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F4A6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F4A70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F4A74: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4A78: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F4A7C: 38AA92A4  addi r5, r10, -0x6d5c
	ctx.r[5].s64 = ctx.r[10].s64 + -27996;
	// 826F4A80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F4A84: 390BDB40  addi r8, r11, -0x24c0
	ctx.r[8].s64 = ctx.r[11].s64 + -9408;
	// 826F4A88: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826F4A8C: 388A4060  addi r4, r10, 0x4060
	ctx.r[4].s64 = ctx.r[10].s64 + 16480;
	// 826F4A90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F4A94: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4A98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F4A9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F4AA0: 386A9484  addi r3, r10, -0x6b7c
	ctx.r[3].s64 = ctx.r[10].s64 + -27516;
	// 826F4AA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F4AA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F4AAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F4AB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F4AB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F4AB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F4ABC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F4AC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F4AC4: 4BD7235D  bl 0x82466e20
	ctx.lr = 0x826F4AC8;
	sub_82466E20(ctx, base);
	// 826F4AC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F4ACC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F4AD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F4AD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F4AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F4AD8 size=112
    let mut pc: u32 = 0x826F4AD8;
    'dispatch: loop {
        match pc {
            0x826F4AD8 => {
    //   block [0x826F4AD8..0x826F4B48)
	// 826F4AD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F4ADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F4AE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F4AE4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4AE8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F4AEC: 38AA8B54  addi r5, r10, -0x74ac
	ctx.r[5].s64 = ctx.r[10].s64 + -29868;
	// 826F4AF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F4AF4: 390BDB70  addi r8, r11, -0x2490
	ctx.r[8].s64 = ctx.r[11].s64 + -9360;
	// 826F4AF8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826F4AFC: 388A4084  addi r4, r10, 0x4084
	ctx.r[4].s64 = ctx.r[10].s64 + 16516;
	// 826F4B00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F4B04: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4B08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F4B0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F4B10: 386A94B4  addi r3, r10, -0x6b4c
	ctx.r[3].s64 = ctx.r[10].s64 + -27468;
	// 826F4B14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F4B18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F4B1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F4B20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F4B24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F4B28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F4B2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F4B30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F4B34: 4BD722ED  bl 0x82466e20
	ctx.lr = 0x826F4B38;
	sub_82466E20(ctx, base);
	// 826F4B38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F4B3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F4B40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F4B44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F4B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826F4B48 size=24
    let mut pc: u32 = 0x826F4B48;
    'dispatch: loop {
        match pc {
            0x826F4B48 => {
    //   block [0x826F4B48..0x826F4B60)
	// 826F4B48: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F4B4C: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F4B50: 394A2210  addi r10, r10, 0x2210
	ctx.r[10].s64 = ctx.r[10].s64 + 8720;
	// 826F4B54: 816BD5CC  lwz r11, -0x2a34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10804 as u32) ) } as u64;
	// 826F4B58: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826F4B5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F4B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F4B60 size=116
    let mut pc: u32 = 0x826F4B60;
    'dispatch: loop {
        match pc {
            0x826F4B60 => {
    //   block [0x826F4B60..0x826F4BD4)
	// 826F4B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F4B64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F4B68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F4B6C: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F4B70: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826F4B74: 390A2210  addi r8, r10, 0x2210
	ctx.r[8].s64 = ctx.r[10].s64 + 8720;
	// 826F4B78: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4B7C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826F4B80: 38AA8D04  addi r5, r10, -0x72fc
	ctx.r[5].s64 = ctx.r[10].s64 + -29436;
	// 826F4B84: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F4B88: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F4B8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F4B90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F4B94: 388A40B8  addi r4, r10, 0x40b8
	ctx.r[4].s64 = ctx.r[10].s64 + 16568;
	// 826F4B98: 396BA1A0  addi r11, r11, -0x5e60
	ctx.r[11].s64 = ctx.r[11].s64 + -24160;
	// 826F4B9C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4BA0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F4BA4: 386A94E4  addi r3, r10, -0x6b1c
	ctx.r[3].s64 = ctx.r[10].s64 + -27420;
	// 826F4BA8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826F4BAC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F4BB0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826F4BB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F4BB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F4BBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F4BC0: 4BD72261  bl 0x82466e20
	ctx.lr = 0x826F4BC4;
	sub_82466E20(ctx, base);
	// 826F4BC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F4BC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F4BCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F4BD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F4BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F4BD8 size=112
    let mut pc: u32 = 0x826F4BD8;
    'dispatch: loop {
        match pc {
            0x826F4BD8 => {
    //   block [0x826F4BD8..0x826F4C48)
	// 826F4BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F4BDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F4BE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F4BE4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4BE8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F4BEC: 38AA8C74  addi r5, r10, -0x738c
	ctx.r[5].s64 = ctx.r[10].s64 + -29580;
	// 826F4BF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F4BF4: 390BDBB8  addi r8, r11, -0x2448
	ctx.r[8].s64 = ctx.r[11].s64 + -9288;
	// 826F4BF8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826F4BFC: 388A40C8  addi r4, r10, 0x40c8
	ctx.r[4].s64 = ctx.r[10].s64 + 16584;
	// 826F4C00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F4C04: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4C08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F4C0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F4C10: 386A9514  addi r3, r10, -0x6aec
	ctx.r[3].s64 = ctx.r[10].s64 + -27372;
	// 826F4C14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F4C18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F4C1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F4C20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F4C24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F4C28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F4C2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F4C30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F4C34: 4BD721ED  bl 0x82466e20
	ctx.lr = 0x826F4C38;
	sub_82466E20(ctx, base);
	// 826F4C38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F4C3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F4C40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F4C44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F4C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F4C48 size=112
    let mut pc: u32 = 0x826F4C48;
    'dispatch: loop {
        match pc {
            0x826F4C48 => {
    //   block [0x826F4C48..0x826F4CB8)
	// 826F4C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F4C4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F4C50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F4C54: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4C58: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F4C5C: 38AA8CD4  addi r5, r10, -0x732c
	ctx.r[5].s64 = ctx.r[10].s64 + -29484;
	// 826F4C60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F4C64: 390BDBE8  addi r8, r11, -0x2418
	ctx.r[8].s64 = ctx.r[11].s64 + -9240;
	// 826F4C68: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826F4C6C: 388A40EC  addi r4, r10, 0x40ec
	ctx.r[4].s64 = ctx.r[10].s64 + 16620;
	// 826F4C70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F4C74: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4C78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F4C7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F4C80: 386A9544  addi r3, r10, -0x6abc
	ctx.r[3].s64 = ctx.r[10].s64 + -27324;
	// 826F4C84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F4C88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F4C8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F4C90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F4C94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F4C98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F4C9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F4CA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F4CA4: 4BD7217D  bl 0x82466e20
	ctx.lr = 0x826F4CA8;
	sub_82466E20(ctx, base);
	// 826F4CA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F4CAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F4CB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F4CB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F4CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F4CB8 size=100
    let mut pc: u32 = 0x826F4CB8;
    'dispatch: loop {
        match pc {
            0x826F4CB8 => {
    //   block [0x826F4CB8..0x826F4D1C)
	// 826F4CB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F4CBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F4CC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F4CC4: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F4CC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F4CCC: 392AA210  addi r9, r10, -0x5df0
	ctx.r[9].s64 = ctx.r[10].s64 + -24048;
	// 826F4CD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F4CD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F4CD8: 388A4110  addi r4, r10, 0x4110
	ctx.r[4].s64 = ctx.r[10].s64 + 16656;
	// 826F4CDC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4CE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F4CE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F4CE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F4CEC: 386A9574  addi r3, r10, -0x6a8c
	ctx.r[3].s64 = ctx.r[10].s64 + -27276;
	// 826F4CF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F4CF4: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 826F4CF8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F4CFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F4D00: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F4D04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F4D08: 4BD72119  bl 0x82466e20
	ctx.lr = 0x826F4D0C;
	sub_82466E20(ctx, base);
	// 826F4D0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F4D10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F4D14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F4D18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F4D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826F4D20 size=24
    let mut pc: u32 = 0x826F4D20;
    'dispatch: loop {
        match pc {
            0x826F4D20 => {
    //   block [0x826F4D20..0x826F4D38)
	// 826F4D20: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F4D24: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F4D28: 394A22B8  addi r10, r10, 0x22b8
	ctx.r[10].s64 = ctx.r[10].s64 + 8888;
	// 826F4D2C: 816BDC20  lwz r11, -0x23e0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-9184 as u32) ) } as u64;
	// 826F4D30: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826F4D34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F4D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F4D38 size=112
    let mut pc: u32 = 0x826F4D38;
    'dispatch: loop {
        match pc {
            0x826F4D38 => {
    //   block [0x826F4D38..0x826F4DA8)
	// 826F4D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F4D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F4D40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F4D44: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F4D48: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F4D4C: 392AA350  addi r9, r10, -0x5cb0
	ctx.r[9].s64 = ctx.r[10].s64 + -23728;
	// 826F4D50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F4D54: 390B22B8  addi r8, r11, 0x22b8
	ctx.r[8].s64 = ctx.r[11].s64 + 8888;
	// 826F4D58: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826F4D5C: 388A4124  addi r4, r10, 0x4124
	ctx.r[4].s64 = ctx.r[10].s64 + 16676;
	// 826F4D60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F4D64: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4D68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F4D6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F4D70: 386A95A4  addi r3, r10, -0x6a5c
	ctx.r[3].s64 = ctx.r[10].s64 + -27228;
	// 826F4D74: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F4D78: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826F4D7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F4D80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F4D84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F4D88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F4D8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F4D90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F4D94: 4BD7208D  bl 0x82466e20
	ctx.lr = 0x826F4D98;
	sub_82466E20(ctx, base);
	// 826F4D98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F4D9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F4DA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F4DA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F4DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F4DA8 size=112
    let mut pc: u32 = 0x826F4DA8;
    'dispatch: loop {
        match pc {
            0x826F4DA8 => {
    //   block [0x826F4DA8..0x826F4E18)
	// 826F4DA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F4DAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F4DB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F4DB4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4DB8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F4DBC: 38AA95A4  addi r5, r10, -0x6a5c
	ctx.r[5].s64 = ctx.r[10].s64 + -27228;
	// 826F4DC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F4DC4: 390BDC28  addi r8, r11, -0x23d8
	ctx.r[8].s64 = ctx.r[11].s64 + -9176;
	// 826F4DC8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826F4DCC: 388A4138  addi r4, r10, 0x4138
	ctx.r[4].s64 = ctx.r[10].s64 + 16696;
	// 826F4DD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F4DD4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4DD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F4DDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F4DE0: 386A95D4  addi r3, r10, -0x6a2c
	ctx.r[3].s64 = ctx.r[10].s64 + -27180;
	// 826F4DE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F4DE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F4DEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F4DF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F4DF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F4DF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F4DFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F4E00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F4E04: 4BD7201D  bl 0x82466e20
	ctx.lr = 0x826F4E08;
	sub_82466E20(ctx, base);
	// 826F4E08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F4E0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F4E10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F4E14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F4E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F4E18 size=108
    let mut pc: u32 = 0x826F4E18;
    'dispatch: loop {
        match pc {
            0x826F4E18 => {
    //   block [0x826F4E18..0x826F4E84)
	// 826F4E18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F4E1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F4E20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F4E24: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F4E28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F4E2C: 38EBDC58  addi r7, r11, -0x23a8
	ctx.r[7].s64 = ctx.r[11].s64 + -9128;
	// 826F4E30: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F4E34: 388A4150  addi r4, r10, 0x4150
	ctx.r[4].s64 = ctx.r[10].s64 + 16720;
	// 826F4E38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F4E3C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4E40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F4E44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F4E48: 386A9604  addi r3, r10, -0x69fc
	ctx.r[3].s64 = ctx.r[10].s64 + -27132;
	// 826F4E4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F4E50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F4E54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F4E58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F4E5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F4E60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F4E64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F4E68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F4E6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F4E70: 4BD71FB1  bl 0x82466e20
	ctx.lr = 0x826F4E74;
	sub_82466E20(ctx, base);
	// 826F4E74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F4E78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F4E7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F4E80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F4E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F4E88 size=112
    let mut pc: u32 = 0x826F4E88;
    'dispatch: loop {
        match pc {
            0x826F4E88 => {
    //   block [0x826F4E88..0x826F4EF8)
	// 826F4E88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F4E8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F4E90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F4E94: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4E98: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F4E9C: 38AA95A4  addi r5, r10, -0x6a5c
	ctx.r[5].s64 = ctx.r[10].s64 + -27228;
	// 826F4EA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826F4EA4: 390BDC70  addi r8, r11, -0x2390
	ctx.r[8].s64 = ctx.r[11].s64 + -9104;
	// 826F4EA8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826F4EAC: 388A0E04  addi r4, r10, 0xe04
	ctx.r[4].s64 = ctx.r[10].s64 + 3588;
	// 826F4EB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F4EB4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4EB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F4EBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F4EC0: 386A9634  addi r3, r10, -0x69cc
	ctx.r[3].s64 = ctx.r[10].s64 + -27084;
	// 826F4EC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F4EC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F4ECC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F4ED0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F4ED4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F4ED8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F4EDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F4EE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F4EE4: 4BD71F3D  bl 0x82466e20
	ctx.lr = 0x826F4EE8;
	sub_82466E20(ctx, base);
	// 826F4EE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F4EEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F4EF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F4EF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F4EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F4EF8 size=100
    let mut pc: u32 = 0x826F4EF8;
    'dispatch: loop {
        match pc {
            0x826F4EF8 => {
    //   block [0x826F4EF8..0x826F4F5C)
	// 826F4EF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F4EFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F4F00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F4F04: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4F08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F4F0C: 38AA95A4  addi r5, r10, -0x6a5c
	ctx.r[5].s64 = ctx.r[10].s64 + -27228;
	// 826F4F10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F4F14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F4F18: 388A4160  addi r4, r10, 0x4160
	ctx.r[4].s64 = ctx.r[10].s64 + 16736;
	// 826F4F1C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4F20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F4F24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F4F28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F4F2C: 386A9664  addi r3, r10, -0x699c
	ctx.r[3].s64 = ctx.r[10].s64 + -27036;
	// 826F4F30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F4F34: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F4F38: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F4F3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F4F40: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F4F44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F4F48: 4BD71ED9  bl 0x82466e20
	ctx.lr = 0x826F4F4C;
	sub_82466E20(ctx, base);
	// 826F4F4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F4F50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F4F54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F4F58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F4F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F4F60 size=112
    let mut pc: u32 = 0x826F4F60;
    'dispatch: loop {
        match pc {
            0x826F4F60 => {
    //   block [0x826F4F60..0x826F4FD0)
	// 826F4F60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F4F64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F4F68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F4F6C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4F70: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F4F74: 38AA95A4  addi r5, r10, -0x6a5c
	ctx.r[5].s64 = ctx.r[10].s64 + -27228;
	// 826F4F78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F4F7C: 390BDCD0  addi r8, r11, -0x2330
	ctx.r[8].s64 = ctx.r[11].s64 + -9008;
	// 826F4F80: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F4F84: 388A417C  addi r4, r10, 0x417c
	ctx.r[4].s64 = ctx.r[10].s64 + 16764;
	// 826F4F88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F4F8C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4F90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F4F94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F4F98: 386A9694  addi r3, r10, -0x696c
	ctx.r[3].s64 = ctx.r[10].s64 + -26988;
	// 826F4F9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F4FA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F4FA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F4FA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F4FAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F4FB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F4FB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F4FB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F4FBC: 4BD71E65  bl 0x82466e20
	ctx.lr = 0x826F4FC0;
	sub_82466E20(ctx, base);
	// 826F4FC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F4FC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F4FC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F4FCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F4FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F4FD0 size=112
    let mut pc: u32 = 0x826F4FD0;
    'dispatch: loop {
        match pc {
            0x826F4FD0 => {
    //   block [0x826F4FD0..0x826F5040)
	// 826F4FD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F4FD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F4FD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F4FDC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4FE0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F4FE4: 38AA95A4  addi r5, r10, -0x6a5c
	ctx.r[5].s64 = ctx.r[10].s64 + -27228;
	// 826F4FE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F4FEC: 390BDCE8  addi r8, r11, -0x2318
	ctx.r[8].s64 = ctx.r[11].s64 + -8984;
	// 826F4FF0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826F4FF4: 388A4198  addi r4, r10, 0x4198
	ctx.r[4].s64 = ctx.r[10].s64 + 16792;
	// 826F4FF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F4FFC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5000: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F5004: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F5008: 386A96C4  addi r3, r10, -0x693c
	ctx.r[3].s64 = ctx.r[10].s64 + -26940;
	// 826F500C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F5010: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F5014: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F5018: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F501C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F5020: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F5024: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F5028: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F502C: 4BD71DF5  bl 0x82466e20
	ctx.lr = 0x826F5030;
	sub_82466E20(ctx, base);
	// 826F5030: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F5034: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F5038: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F503C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F5040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F5040 size=112
    let mut pc: u32 = 0x826F5040;
    'dispatch: loop {
        match pc {
            0x826F5040 => {
    //   block [0x826F5040..0x826F50B0)
	// 826F5040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F5044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F5048: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F504C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5050: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F5054: 38AA95A4  addi r5, r10, -0x6a5c
	ctx.r[5].s64 = ctx.r[10].s64 + -27228;
	// 826F5058: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F505C: 390BDD18  addi r8, r11, -0x22e8
	ctx.r[8].s64 = ctx.r[11].s64 + -8936;
	// 826F5060: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826F5064: 388A41BC  addi r4, r10, 0x41bc
	ctx.r[4].s64 = ctx.r[10].s64 + 16828;
	// 826F5068: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F506C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5070: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F5074: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F5078: 386A96F4  addi r3, r10, -0x690c
	ctx.r[3].s64 = ctx.r[10].s64 + -26892;
	// 826F507C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F5080: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F5084: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F5088: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F508C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F5090: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F5094: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F5098: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F509C: 4BD71D85  bl 0x82466e20
	ctx.lr = 0x826F50A0;
	sub_82466E20(ctx, base);
	// 826F50A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F50A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F50A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F50AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F50B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F50B0 size=112
    let mut pc: u32 = 0x826F50B0;
    'dispatch: loop {
        match pc {
            0x826F50B0 => {
    //   block [0x826F50B0..0x826F5120)
	// 826F50B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F50B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F50B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F50BC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F50C0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F50C4: 38AA95A4  addi r5, r10, -0x6a5c
	ctx.r[5].s64 = ctx.r[10].s64 + -27228;
	// 826F50C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F50CC: 390BDD48  addi r8, r11, -0x22b8
	ctx.r[8].s64 = ctx.r[11].s64 + -8888;
	// 826F50D0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826F50D4: 388A41E4  addi r4, r10, 0x41e4
	ctx.r[4].s64 = ctx.r[10].s64 + 16868;
	// 826F50D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F50DC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F50E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F50E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F50E8: 386A9724  addi r3, r10, -0x68dc
	ctx.r[3].s64 = ctx.r[10].s64 + -26844;
	// 826F50EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F50F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F50F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F50F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F50FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F5100: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F5104: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F5108: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F510C: 4BD71D15  bl 0x82466e20
	ctx.lr = 0x826F5110;
	sub_82466E20(ctx, base);
	// 826F5110: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F5114: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F5118: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F511C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F5120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F5120 size=112
    let mut pc: u32 = 0x826F5120;
    'dispatch: loop {
        match pc {
            0x826F5120 => {
    //   block [0x826F5120..0x826F5190)
	// 826F5120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F5124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F5128: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F512C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5130: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F5134: 38AA95A4  addi r5, r10, -0x6a5c
	ctx.r[5].s64 = ctx.r[10].s64 + -27228;
	// 826F5138: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F513C: 390BDD78  addi r8, r11, -0x2288
	ctx.r[8].s64 = ctx.r[11].s64 + -8840;
	// 826F5140: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F5144: 388A4208  addi r4, r10, 0x4208
	ctx.r[4].s64 = ctx.r[10].s64 + 16904;
	// 826F5148: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F514C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5150: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F5154: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F5158: 386A9754  addi r3, r10, -0x68ac
	ctx.r[3].s64 = ctx.r[10].s64 + -26796;
	// 826F515C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F5160: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F5164: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F5168: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F516C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F5170: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F5174: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F5178: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F517C: 4BD71CA5  bl 0x82466e20
	ctx.lr = 0x826F5180;
	sub_82466E20(ctx, base);
	// 826F5180: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F5184: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F5188: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F518C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F5190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F5190 size=112
    let mut pc: u32 = 0x826F5190;
    'dispatch: loop {
        match pc {
            0x826F5190 => {
    //   block [0x826F5190..0x826F5200)
	// 826F5190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F5194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F5198: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F519C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F51A0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F51A4: 38AA95A4  addi r5, r10, -0x6a5c
	ctx.r[5].s64 = ctx.r[10].s64 + -27228;
	// 826F51A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F51AC: 390BDD90  addi r8, r11, -0x2270
	ctx.r[8].s64 = ctx.r[11].s64 + -8816;
	// 826F51B0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F51B4: 388A4228  addi r4, r10, 0x4228
	ctx.r[4].s64 = ctx.r[10].s64 + 16936;
	// 826F51B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F51BC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F51C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F51C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F51C8: 386A9784  addi r3, r10, -0x687c
	ctx.r[3].s64 = ctx.r[10].s64 + -26748;
	// 826F51CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F51D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F51D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F51D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F51DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F51E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F51E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F51E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F51EC: 4BD71C35  bl 0x82466e20
	ctx.lr = 0x826F51F0;
	sub_82466E20(ctx, base);
	// 826F51F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F51F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F51F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F51FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F5200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F5200 size=112
    let mut pc: u32 = 0x826F5200;
    'dispatch: loop {
        match pc {
            0x826F5200 => {
    //   block [0x826F5200..0x826F5270)
	// 826F5200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F5204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F5208: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F520C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5210: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F5214: 38AA95A4  addi r5, r10, -0x6a5c
	ctx.r[5].s64 = ctx.r[10].s64 + -27228;
	// 826F5218: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F521C: 390BDDA8  addi r8, r11, -0x2258
	ctx.r[8].s64 = ctx.r[11].s64 + -8792;
	// 826F5220: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826F5224: 388A423C  addi r4, r10, 0x423c
	ctx.r[4].s64 = ctx.r[10].s64 + 16956;
	// 826F5228: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F522C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5230: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F5234: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F5238: 386A97B4  addi r3, r10, -0x684c
	ctx.r[3].s64 = ctx.r[10].s64 + -26700;
	// 826F523C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F5240: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F5244: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F5248: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F524C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F5250: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F5254: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F5258: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F525C: 4BD71BC5  bl 0x82466e20
	ctx.lr = 0x826F5260;
	sub_82466E20(ctx, base);
	// 826F5260: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F5264: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F5268: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F526C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F5270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F5270 size=112
    let mut pc: u32 = 0x826F5270;
    'dispatch: loop {
        match pc {
            0x826F5270 => {
    //   block [0x826F5270..0x826F52E0)
	// 826F5270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F5274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F5278: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F527C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5280: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F5284: 38AA95A4  addi r5, r10, -0x6a5c
	ctx.r[5].s64 = ctx.r[10].s64 + -27228;
	// 826F5288: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F528C: 390BDDF0  addi r8, r11, -0x2210
	ctx.r[8].s64 = ctx.r[11].s64 + -8720;
	// 826F5290: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826F5294: 388A4254  addi r4, r10, 0x4254
	ctx.r[4].s64 = ctx.r[10].s64 + 16980;
	// 826F5298: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F529C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F52A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F52A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F52A8: 386A97E4  addi r3, r10, -0x681c
	ctx.r[3].s64 = ctx.r[10].s64 + -26652;
	// 826F52AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F52B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F52B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F52B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F52BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F52C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F52C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F52C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F52CC: 4BD71B55  bl 0x82466e20
	ctx.lr = 0x826F52D0;
	sub_82466E20(ctx, base);
	// 826F52D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F52D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F52D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F52DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F52E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F52E0 size=112
    let mut pc: u32 = 0x826F52E0;
    'dispatch: loop {
        match pc {
            0x826F52E0 => {
    //   block [0x826F52E0..0x826F5350)
	// 826F52E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F52E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F52E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F52EC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F52F0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F52F4: 38AA95A4  addi r5, r10, -0x6a5c
	ctx.r[5].s64 = ctx.r[10].s64 + -27228;
	// 826F52F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F52FC: 390BDE38  addi r8, r11, -0x21c8
	ctx.r[8].s64 = ctx.r[11].s64 + -8648;
	// 826F5300: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F5304: 388A4270  addi r4, r10, 0x4270
	ctx.r[4].s64 = ctx.r[10].s64 + 17008;
	// 826F5308: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F530C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5310: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F5314: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F5318: 386A9814  addi r3, r10, -0x67ec
	ctx.r[3].s64 = ctx.r[10].s64 + -26604;
	// 826F531C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F5320: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F5324: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F5328: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F532C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F5330: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F5334: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F5338: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F533C: 4BD71AE5  bl 0x82466e20
	ctx.lr = 0x826F5340;
	sub_82466E20(ctx, base);
	// 826F5340: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F5344: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F5348: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F534C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F5350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F5350 size=112
    let mut pc: u32 = 0x826F5350;
    'dispatch: loop {
        match pc {
            0x826F5350 => {
    //   block [0x826F5350..0x826F53C0)
	// 826F5350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F5354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F5358: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F535C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5360: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F5364: 38AA95A4  addi r5, r10, -0x6a5c
	ctx.r[5].s64 = ctx.r[10].s64 + -27228;
	// 826F5368: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F536C: 390BDE50  addi r8, r11, -0x21b0
	ctx.r[8].s64 = ctx.r[11].s64 + -8624;
	// 826F5370: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826F5374: 388A4288  addi r4, r10, 0x4288
	ctx.r[4].s64 = ctx.r[10].s64 + 17032;
	// 826F5378: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F537C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5380: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F5384: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F5388: 386A9844  addi r3, r10, -0x67bc
	ctx.r[3].s64 = ctx.r[10].s64 + -26556;
	// 826F538C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F5390: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F5394: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F5398: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F539C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F53A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F53A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F53A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F53AC: 4BD71A75  bl 0x82466e20
	ctx.lr = 0x826F53B0;
	sub_82466E20(ctx, base);
	// 826F53B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F53B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F53B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F53BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F53C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F53C0 size=116
    let mut pc: u32 = 0x826F53C0;
    'dispatch: loop {
        match pc {
            0x826F53C0 => {
    //   block [0x826F53C0..0x826F5434)
	// 826F53C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F53C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F53C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F53CC: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F53D0: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826F53D4: 390ADE80  addi r8, r10, -0x2180
	ctx.r[8].s64 = ctx.r[10].s64 + -8576;
	// 826F53D8: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F53DC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826F53E0: 38AA95A4  addi r5, r10, -0x6a5c
	ctx.r[5].s64 = ctx.r[10].s64 + -27228;
	// 826F53E4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F53E8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F53EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F53F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F53F4: 388A429C  addi r4, r10, 0x429c
	ctx.r[4].s64 = ctx.r[10].s64 + 17052;
	// 826F53F8: 396BA378  addi r11, r11, -0x5c88
	ctx.r[11].s64 = ctx.r[11].s64 + -23688;
	// 826F53FC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5400: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F5404: 386A9874  addi r3, r10, -0x678c
	ctx.r[3].s64 = ctx.r[10].s64 + -26508;
	// 826F5408: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826F540C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F5410: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826F5414: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F5418: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F541C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F5420: 4BD71A01  bl 0x82466e20
	ctx.lr = 0x826F5424;
	sub_82466E20(ctx, base);
	// 826F5424: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F5428: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F542C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F5430: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F5438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F5438 size=116
    let mut pc: u32 = 0x826F5438;
    'dispatch: loop {
        match pc {
            0x826F5438 => {
    //   block [0x826F5438..0x826F54AC)
	// 826F5438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F543C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F5440: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F5444: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F5448: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826F544C: 390ADEF8  addi r8, r10, -0x2108
	ctx.r[8].s64 = ctx.r[10].s64 + -8456;
	// 826F5450: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5454: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826F5458: 38AA95A4  addi r5, r10, -0x6a5c
	ctx.r[5].s64 = ctx.r[10].s64 + -27228;
	// 826F545C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F5460: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F5464: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F5468: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F546C: 388A42B8  addi r4, r10, 0x42b8
	ctx.r[4].s64 = ctx.r[10].s64 + 17080;
	// 826F5470: 396BA390  addi r11, r11, -0x5c70
	ctx.r[11].s64 = ctx.r[11].s64 + -23664;
	// 826F5474: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5478: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F547C: 386A98A4  addi r3, r10, -0x675c
	ctx.r[3].s64 = ctx.r[10].s64 + -26460;
	// 826F5480: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826F5484: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F5488: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826F548C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F5490: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F5494: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F5498: 4BD71989  bl 0x82466e20
	ctx.lr = 0x826F549C;
	sub_82466E20(ctx, base);
	// 826F549C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F54A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F54A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F54A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F54B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826F54B0 size=24
    let mut pc: u32 = 0x826F54B0;
    'dispatch: loop {
        match pc {
            0x826F54B0 => {
    //   block [0x826F54B0..0x826F54C8)
	// 826F54B0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F54B4: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F54B8: 394A22D0  addi r10, r10, 0x22d0
	ctx.r[10].s64 = ctx.r[10].s64 + 8912;
	// 826F54BC: 816BDF88  lwz r11, -0x2078(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8312 as u32) ) } as u64;
	// 826F54C0: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826F54C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F54C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F54C8 size=116
    let mut pc: u32 = 0x826F54C8;
    'dispatch: loop {
        match pc {
            0x826F54C8 => {
    //   block [0x826F54C8..0x826F553C)
	// 826F54C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F54CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F54D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F54D4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826F54D8: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F54DC: 392BA3BC  addi r9, r11, -0x5c44
	ctx.r[9].s64 = ctx.r[11].s64 + -23620;
	// 826F54E0: 38AA95A4  addi r5, r10, -0x6a5c
	ctx.r[5].s64 = ctx.r[10].s64 + -27228;
	// 826F54E4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F54E8: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826F54EC: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 826F54F0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F54F4: 388A42D4  addi r4, r10, 0x42d4
	ctx.r[4].s64 = ctx.r[10].s64 + 17108;
	// 826F54F8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F54FC: 396B22D0  addi r11, r11, 0x22d0
	ctx.r[11].s64 = ctx.r[11].s64 + 8912;
	// 826F5500: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826F5504: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5508: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826F550C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F5510: 386A98D4  addi r3, r10, -0x672c
	ctx.r[3].s64 = ctx.r[10].s64 + -26412;
	// 826F5514: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F5518: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826F551C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F5520: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826F5524: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F5528: 4BD718F9  bl 0x82466e20
	ctx.lr = 0x826F552C;
	sub_82466E20(ctx, base);
	// 826F552C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F5530: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F5534: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F5538: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F5540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F5540 size=112
    let mut pc: u32 = 0x826F5540;
    'dispatch: loop {
        match pc {
            0x826F5540 => {
    //   block [0x826F5540..0x826F55B0)
	// 826F5540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F5544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F5548: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F554C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5550: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F5554: 38AA95A4  addi r5, r10, -0x6a5c
	ctx.r[5].s64 = ctx.r[10].s64 + -27228;
	// 826F5558: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F555C: 390BDF90  addi r8, r11, -0x2070
	ctx.r[8].s64 = ctx.r[11].s64 + -8304;
	// 826F5560: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826F5564: 388A42F0  addi r4, r10, 0x42f0
	ctx.r[4].s64 = ctx.r[10].s64 + 17136;
	// 826F5568: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F556C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5570: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F5574: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F5578: 386A9904  addi r3, r10, -0x66fc
	ctx.r[3].s64 = ctx.r[10].s64 + -26364;
	// 826F557C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F5580: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F5584: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F5588: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F558C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F5590: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F5594: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F5598: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F559C: 4BD71885  bl 0x82466e20
	ctx.lr = 0x826F55A0;
	sub_82466E20(ctx, base);
	// 826F55A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F55A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F55A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F55AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F55B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F55B0 size=112
    let mut pc: u32 = 0x826F55B0;
    'dispatch: loop {
        match pc {
            0x826F55B0 => {
    //   block [0x826F55B0..0x826F5620)
	// 826F55B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F55B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F55B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F55BC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F55C0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F55C4: 38AA95A4  addi r5, r10, -0x6a5c
	ctx.r[5].s64 = ctx.r[10].s64 + -27228;
	// 826F55C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F55CC: 390BDFF0  addi r8, r11, -0x2010
	ctx.r[8].s64 = ctx.r[11].s64 + -8208;
	// 826F55D0: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826F55D4: 388A430C  addi r4, r10, 0x430c
	ctx.r[4].s64 = ctx.r[10].s64 + 17164;
	// 826F55D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F55DC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F55E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F55E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F55E8: 386A9934  addi r3, r10, -0x66cc
	ctx.r[3].s64 = ctx.r[10].s64 + -26316;
	// 826F55EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F55F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F55F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F55F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F55FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F5600: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F5604: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F5608: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F560C: 4BD71815  bl 0x82466e20
	ctx.lr = 0x826F5610;
	sub_82466E20(ctx, base);
	// 826F5610: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F5614: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F5618: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F561C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F5620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F5620 size=112
    let mut pc: u32 = 0x826F5620;
    'dispatch: loop {
        match pc {
            0x826F5620 => {
    //   block [0x826F5620..0x826F5690)
	// 826F5620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F5624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F5628: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F562C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5630: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F5634: 38AA95A4  addi r5, r10, -0x6a5c
	ctx.r[5].s64 = ctx.r[10].s64 + -27228;
	// 826F5638: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F563C: 390BE098  addi r8, r11, -0x1f68
	ctx.r[8].s64 = ctx.r[11].s64 + -8040;
	// 826F5640: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826F5644: 388A4328  addi r4, r10, 0x4328
	ctx.r[4].s64 = ctx.r[10].s64 + 17192;
	// 826F5648: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F564C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5650: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F5654: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F5658: 386A9964  addi r3, r10, -0x669c
	ctx.r[3].s64 = ctx.r[10].s64 + -26268;
	// 826F565C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F5660: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F5664: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F5668: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F566C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F5670: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F5674: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F5678: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F567C: 4BD717A5  bl 0x82466e20
	ctx.lr = 0x826F5680;
	sub_82466E20(ctx, base);
	// 826F5680: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F5684: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F5688: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F568C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F5690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F5690 size=112
    let mut pc: u32 = 0x826F5690;
    'dispatch: loop {
        match pc {
            0x826F5690 => {
    //   block [0x826F5690..0x826F5700)
	// 826F5690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F5694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F5698: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F569C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F56A0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F56A4: 38AA95A4  addi r5, r10, -0x6a5c
	ctx.r[5].s64 = ctx.r[10].s64 + -27228;
	// 826F56A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F56AC: 390BE110  addi r8, r11, -0x1ef0
	ctx.r[8].s64 = ctx.r[11].s64 + -7920;
	// 826F56B0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826F56B4: 388A4348  addi r4, r10, 0x4348
	ctx.r[4].s64 = ctx.r[10].s64 + 17224;
	// 826F56B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F56BC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F56C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F56C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F56C8: 386A9994  addi r3, r10, -0x666c
	ctx.r[3].s64 = ctx.r[10].s64 + -26220;
	// 826F56CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F56D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F56D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F56D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F56DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F56E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F56E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F56E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F56EC: 4BD71735  bl 0x82466e20
	ctx.lr = 0x826F56F0;
	sub_82466E20(ctx, base);
	// 826F56F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F56F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F56F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F56FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F5700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F5700 size=112
    let mut pc: u32 = 0x826F5700;
    'dispatch: loop {
        match pc {
            0x826F5700 => {
    //   block [0x826F5700..0x826F5770)
	// 826F5700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F5704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F5708: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F570C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5710: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F5714: 38AA95A4  addi r5, r10, -0x6a5c
	ctx.r[5].s64 = ctx.r[10].s64 + -27228;
	// 826F5718: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F571C: 390BE158  addi r8, r11, -0x1ea8
	ctx.r[8].s64 = ctx.r[11].s64 + -7848;
	// 826F5720: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826F5724: 388A4364  addi r4, r10, 0x4364
	ctx.r[4].s64 = ctx.r[10].s64 + 17252;
	// 826F5728: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F572C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5730: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F5734: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F5738: 386A99C4  addi r3, r10, -0x663c
	ctx.r[3].s64 = ctx.r[10].s64 + -26172;
	// 826F573C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F5740: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F5744: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F5748: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F574C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F5750: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F5754: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F5758: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F575C: 4BD716C5  bl 0x82466e20
	ctx.lr = 0x826F5760;
	sub_82466E20(ctx, base);
	// 826F5760: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F5764: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F5768: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F576C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F5770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F5770 size=112
    let mut pc: u32 = 0x826F5770;
    'dispatch: loop {
        match pc {
            0x826F5770 => {
    //   block [0x826F5770..0x826F57E0)
	// 826F5770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F5774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F5778: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F577C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5780: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F5784: 38AA95A4  addi r5, r10, -0x6a5c
	ctx.r[5].s64 = ctx.r[10].s64 + -27228;
	// 826F5788: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F578C: 390BE1E8  addi r8, r11, -0x1e18
	ctx.r[8].s64 = ctx.r[11].s64 + -7704;
	// 826F5790: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826F5794: 388A4380  addi r4, r10, 0x4380
	ctx.r[4].s64 = ctx.r[10].s64 + 17280;
	// 826F5798: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F579C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F57A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F57A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F57A8: 386A99F4  addi r3, r10, -0x660c
	ctx.r[3].s64 = ctx.r[10].s64 + -26124;
	// 826F57AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F57B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F57B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F57B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F57BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F57C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F57C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F57C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F57CC: 4BD71655  bl 0x82466e20
	ctx.lr = 0x826F57D0;
	sub_82466E20(ctx, base);
	// 826F57D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F57D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F57D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F57DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F57E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F57E0 size=112
    let mut pc: u32 = 0x826F57E0;
    'dispatch: loop {
        match pc {
            0x826F57E0 => {
    //   block [0x826F57E0..0x826F5850)
	// 826F57E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F57E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F57E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F57EC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F57F0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F57F4: 38AA95A4  addi r5, r10, -0x6a5c
	ctx.r[5].s64 = ctx.r[10].s64 + -27228;
	// 826F57F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F57FC: 390BE248  addi r8, r11, -0x1db8
	ctx.r[8].s64 = ctx.r[11].s64 + -7608;
	// 826F5800: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826F5804: 388A4398  addi r4, r10, 0x4398
	ctx.r[4].s64 = ctx.r[10].s64 + 17304;
	// 826F5808: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F580C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5810: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F5814: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F5818: 386A9A24  addi r3, r10, -0x65dc
	ctx.r[3].s64 = ctx.r[10].s64 + -26076;
	// 826F581C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F5820: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F5824: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F5828: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F582C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F5830: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F5834: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F5838: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F583C: 4BD715E5  bl 0x82466e20
	ctx.lr = 0x826F5840;
	sub_82466E20(ctx, base);
	// 826F5840: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F5844: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F5848: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F584C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F5850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F5850 size=112
    let mut pc: u32 = 0x826F5850;
    'dispatch: loop {
        match pc {
            0x826F5850 => {
    //   block [0x826F5850..0x826F58C0)
	// 826F5850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F5854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F5858: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F585C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5860: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F5864: 38AA9A24  addi r5, r10, -0x65dc
	ctx.r[5].s64 = ctx.r[10].s64 + -26076;
	// 826F5868: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F586C: 390BE2A8  addi r8, r11, -0x1d58
	ctx.r[8].s64 = ctx.r[11].s64 + -7512;
	// 826F5870: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826F5874: 388A43B4  addi r4, r10, 0x43b4
	ctx.r[4].s64 = ctx.r[10].s64 + 17332;
	// 826F5878: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F587C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5880: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F5884: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F5888: 386A9A54  addi r3, r10, -0x65ac
	ctx.r[3].s64 = ctx.r[10].s64 + -26028;
	// 826F588C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F5890: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F5894: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F5898: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F589C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F58A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F58A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F58A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F58AC: 4BD71575  bl 0x82466e20
	ctx.lr = 0x826F58B0;
	sub_82466E20(ctx, base);
	// 826F58B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F58B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F58B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F58BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F58C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F58C0 size=112
    let mut pc: u32 = 0x826F58C0;
    'dispatch: loop {
        match pc {
            0x826F58C0 => {
    //   block [0x826F58C0..0x826F5930)
	// 826F58C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F58C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F58C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F58CC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F58D0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F58D4: 38AA9A24  addi r5, r10, -0x65dc
	ctx.r[5].s64 = ctx.r[10].s64 + -26076;
	// 826F58D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F58DC: 390BE2D8  addi r8, r11, -0x1d28
	ctx.r[8].s64 = ctx.r[11].s64 + -7464;
	// 826F58E0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826F58E4: 388A43D8  addi r4, r10, 0x43d8
	ctx.r[4].s64 = ctx.r[10].s64 + 17368;
	// 826F58E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F58EC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F58F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F58F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F58F8: 386A9A84  addi r3, r10, -0x657c
	ctx.r[3].s64 = ctx.r[10].s64 + -25980;
	// 826F58FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F5900: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F5904: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F5908: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F590C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F5910: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F5914: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F5918: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F591C: 4BD71505  bl 0x82466e20
	ctx.lr = 0x826F5920;
	sub_82466E20(ctx, base);
	// 826F5920: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F5924: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F5928: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F592C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F5930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F5930 size=100
    let mut pc: u32 = 0x826F5930;
    'dispatch: loop {
        match pc {
            0x826F5930 => {
    //   block [0x826F5930..0x826F5994)
	// 826F5930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F5934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F5938: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F593C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5940: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F5944: 38AA9A24  addi r5, r10, -0x65dc
	ctx.r[5].s64 = ctx.r[10].s64 + -26076;
	// 826F5948: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F594C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F5950: 388A43FC  addi r4, r10, 0x43fc
	ctx.r[4].s64 = ctx.r[10].s64 + 17404;
	// 826F5954: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5958: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F595C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F5960: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F5964: 386A9AB4  addi r3, r10, -0x654c
	ctx.r[3].s64 = ctx.r[10].s64 + -25932;
	// 826F5968: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F596C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F5970: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F5974: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F5978: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F597C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F5980: 4BD714A1  bl 0x82466e20
	ctx.lr = 0x826F5984;
	sub_82466E20(ctx, base);
	// 826F5984: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F5988: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F598C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F5990: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F5998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F5998 size=112
    let mut pc: u32 = 0x826F5998;
    'dispatch: loop {
        match pc {
            0x826F5998 => {
    //   block [0x826F5998..0x826F5A08)
	// 826F5998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F599C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F59A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F59A4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F59A8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F59AC: 38AA9A24  addi r5, r10, -0x65dc
	ctx.r[5].s64 = ctx.r[10].s64 + -26076;
	// 826F59B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F59B4: 390BE308  addi r8, r11, -0x1cf8
	ctx.r[8].s64 = ctx.r[11].s64 + -7416;
	// 826F59B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F59BC: 388A4424  addi r4, r10, 0x4424
	ctx.r[4].s64 = ctx.r[10].s64 + 17444;
	// 826F59C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F59C4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F59C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F59CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F59D0: 386A9AE4  addi r3, r10, -0x651c
	ctx.r[3].s64 = ctx.r[10].s64 + -25884;
	// 826F59D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F59D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F59DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F59E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F59E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F59E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F59EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F59F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F59F4: 4BD7142D  bl 0x82466e20
	ctx.lr = 0x826F59F8;
	sub_82466E20(ctx, base);
	// 826F59F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F59FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F5A00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F5A04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F5A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F5A08 size=108
    let mut pc: u32 = 0x826F5A08;
    'dispatch: loop {
        match pc {
            0x826F5A08 => {
    //   block [0x826F5A08..0x826F5A74)
	// 826F5A08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F5A0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F5A10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F5A14: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F5A18: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826F5A1C: 38EBE320  addi r7, r11, -0x1ce0
	ctx.r[7].s64 = ctx.r[11].s64 + -7392;
	// 826F5A20: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826F5A24: 388A0E24  addi r4, r10, 0xe24
	ctx.r[4].s64 = ctx.r[10].s64 + 3620;
	// 826F5A28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F5A2C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5A30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F5A34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F5A38: 386A9B14  addi r3, r10, -0x64ec
	ctx.r[3].s64 = ctx.r[10].s64 + -25836;
	// 826F5A3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F5A40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F5A44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F5A48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F5A4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F5A50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F5A54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F5A58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F5A5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F5A60: 4BD713C1  bl 0x82466e20
	ctx.lr = 0x826F5A64;
	sub_82466E20(ctx, base);
	// 826F5A64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F5A68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F5A6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F5A70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F5A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F5A78 size=112
    let mut pc: u32 = 0x826F5A78;
    'dispatch: loop {
        match pc {
            0x826F5A78 => {
    //   block [0x826F5A78..0x826F5AE8)
	// 826F5A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F5A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F5A80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F5A84: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F5A88: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F5A8C: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F5A90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F5A94: 390BE368  addi r8, r11, -0x1c98
	ctx.r[8].s64 = ctx.r[11].s64 + -7320;
	// 826F5A98: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826F5A9C: 388A444C  addi r4, r10, 0x444c
	ctx.r[4].s64 = ctx.r[10].s64 + 17484;
	// 826F5AA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F5AA4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5AA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F5AAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F5AB0: 386A9B44  addi r3, r10, -0x64bc
	ctx.r[3].s64 = ctx.r[10].s64 + -25788;
	// 826F5AB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F5AB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F5ABC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F5AC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F5AC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F5AC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F5ACC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F5AD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F5AD4: 4BD7134D  bl 0x82466e20
	ctx.lr = 0x826F5AD8;
	sub_82466E20(ctx, base);
	// 826F5AD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F5ADC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F5AE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F5AE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F5AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F5AE8 size=112
    let mut pc: u32 = 0x826F5AE8;
    'dispatch: loop {
        match pc {
            0x826F5AE8 => {
    //   block [0x826F5AE8..0x826F5B58)
	// 826F5AE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F5AEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F5AF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F5AF4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5AF8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F5AFC: 38AA9B44  addi r5, r10, -0x64bc
	ctx.r[5].s64 = ctx.r[10].s64 + -25788;
	// 826F5B00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F5B04: 390BE3C8  addi r8, r11, -0x1c38
	ctx.r[8].s64 = ctx.r[11].s64 + -7224;
	// 826F5B08: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F5B0C: 388A4458  addi r4, r10, 0x4458
	ctx.r[4].s64 = ctx.r[10].s64 + 17496;
	// 826F5B10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F5B14: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5B18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F5B1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F5B20: 386A9B74  addi r3, r10, -0x648c
	ctx.r[3].s64 = ctx.r[10].s64 + -25740;
	// 826F5B24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F5B28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F5B2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F5B30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F5B34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F5B38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F5B3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F5B40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F5B44: 4BD712DD  bl 0x82466e20
	ctx.lr = 0x826F5B48;
	sub_82466E20(ctx, base);
	// 826F5B48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F5B4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F5B50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F5B54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F5B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F5B58 size=112
    let mut pc: u32 = 0x826F5B58;
    'dispatch: loop {
        match pc {
            0x826F5B58 => {
    //   block [0x826F5B58..0x826F5BC8)
	// 826F5B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F5B5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F5B60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F5B64: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5B68: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F5B6C: 38AA9B44  addi r5, r10, -0x64bc
	ctx.r[5].s64 = ctx.r[10].s64 + -25788;
	// 826F5B70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F5B74: 390BE3E0  addi r8, r11, -0x1c20
	ctx.r[8].s64 = ctx.r[11].s64 + -7200;
	// 826F5B78: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826F5B7C: 388A4468  addi r4, r10, 0x4468
	ctx.r[4].s64 = ctx.r[10].s64 + 17512;
	// 826F5B80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F5B84: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5B88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F5B8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F5B90: 386A9BA4  addi r3, r10, -0x645c
	ctx.r[3].s64 = ctx.r[10].s64 + -25692;
	// 826F5B94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F5B98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F5B9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F5BA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F5BA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F5BA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F5BAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F5BB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F5BB4: 4BD7126D  bl 0x82466e20
	ctx.lr = 0x826F5BB8;
	sub_82466E20(ctx, base);
	// 826F5BB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F5BBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F5BC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F5BC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F5BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F5BC8 size=112
    let mut pc: u32 = 0x826F5BC8;
    'dispatch: loop {
        match pc {
            0x826F5BC8 => {
    //   block [0x826F5BC8..0x826F5C38)
	// 826F5BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F5BCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F5BD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F5BD4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5BD8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F5BDC: 38AA9B44  addi r5, r10, -0x64bc
	ctx.r[5].s64 = ctx.r[10].s64 + -25788;
	// 826F5BE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F5BE4: 390BE410  addi r8, r11, -0x1bf0
	ctx.r[8].s64 = ctx.r[11].s64 + -7152;
	// 826F5BE8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F5BEC: 388A4478  addi r4, r10, 0x4478
	ctx.r[4].s64 = ctx.r[10].s64 + 17528;
	// 826F5BF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F5BF4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5BF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F5BFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F5C00: 386A9BD4  addi r3, r10, -0x642c
	ctx.r[3].s64 = ctx.r[10].s64 + -25644;
	// 826F5C04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F5C08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F5C0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F5C10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F5C14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F5C18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F5C1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F5C20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F5C24: 4BD711FD  bl 0x82466e20
	ctx.lr = 0x826F5C28;
	sub_82466E20(ctx, base);
	// 826F5C28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F5C2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F5C30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F5C34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F5C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826F5C38 size=24
    let mut pc: u32 = 0x826F5C38;
    'dispatch: loop {
        match pc {
            0x826F5C38 => {
    //   block [0x826F5C38..0x826F5C50)
	// 826F5C38: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F5C3C: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F5C40: 394A2378  addi r10, r10, 0x2378
	ctx.r[10].s64 = ctx.r[10].s64 + 9080;
	// 826F5C44: 816BDF8C  lwz r11, -0x2074(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8308 as u32) ) } as u64;
	// 826F5C48: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826F5C4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F5C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F5C50 size=112
    let mut pc: u32 = 0x826F5C50;
    'dispatch: loop {
        match pc {
            0x826F5C50 => {
    //   block [0x826F5C50..0x826F5CC0)
	// 826F5C50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F5C54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F5C58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F5C5C: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F5C60: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F5C64: 392AA418  addi r9, r10, -0x5be8
	ctx.r[9].s64 = ctx.r[10].s64 + -23528;
	// 826F5C68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F5C6C: 390B2378  addi r8, r11, 0x2378
	ctx.r[8].s64 = ctx.r[11].s64 + 9080;
	// 826F5C70: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826F5C74: 388A4488  addi r4, r10, 0x4488
	ctx.r[4].s64 = ctx.r[10].s64 + 17544;
	// 826F5C78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F5C7C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5C80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F5C84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F5C88: 386A9C04  addi r3, r10, -0x63fc
	ctx.r[3].s64 = ctx.r[10].s64 + -25596;
	// 826F5C8C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F5C90: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F5C94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F5C98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F5C9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F5CA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F5CA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F5CA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F5CAC: 4BD71175  bl 0x82466e20
	ctx.lr = 0x826F5CB0;
	sub_82466E20(ctx, base);
	// 826F5CB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F5CB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F5CB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F5CBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F5CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F5CC0 size=108
    let mut pc: u32 = 0x826F5CC0;
    'dispatch: loop {
        match pc {
            0x826F5CC0 => {
    //   block [0x826F5CC0..0x826F5D2C)
	// 826F5CC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F5CC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F5CC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F5CCC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F5CD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F5CD4: 38EBE428  addi r7, r11, -0x1bd8
	ctx.r[7].s64 = ctx.r[11].s64 + -7128;
	// 826F5CD8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F5CDC: 388A4494  addi r4, r10, 0x4494
	ctx.r[4].s64 = ctx.r[10].s64 + 17556;
	// 826F5CE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F5CE4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5CE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F5CEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F5CF0: 386A9C34  addi r3, r10, -0x63cc
	ctx.r[3].s64 = ctx.r[10].s64 + -25548;
	// 826F5CF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F5CF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F5CFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F5D00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F5D04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F5D08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F5D0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F5D10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F5D14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F5D18: 4BD71109  bl 0x82466e20
	ctx.lr = 0x826F5D1C;
	sub_82466E20(ctx, base);
	// 826F5D1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F5D20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F5D24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F5D28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F5D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F5D30 size=108
    let mut pc: u32 = 0x826F5D30;
    'dispatch: loop {
        match pc {
            0x826F5D30 => {
    //   block [0x826F5D30..0x826F5D9C)
	// 826F5D30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F5D34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F5D38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F5D3C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F5D40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F5D44: 38EBE440  addi r7, r11, -0x1bc0
	ctx.r[7].s64 = ctx.r[11].s64 + -7104;
	// 826F5D48: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826F5D4C: 388A44A4  addi r4, r10, 0x44a4
	ctx.r[4].s64 = ctx.r[10].s64 + 17572;
	// 826F5D50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F5D54: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5D58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F5D5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F5D60: 386A9C64  addi r3, r10, -0x639c
	ctx.r[3].s64 = ctx.r[10].s64 + -25500;
	// 826F5D64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F5D68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F5D6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F5D70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F5D74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F5D78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F5D7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F5D80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F5D84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F5D88: 4BD71099  bl 0x82466e20
	ctx.lr = 0x826F5D8C;
	sub_82466E20(ctx, base);
	// 826F5D8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F5D90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F5D94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F5D98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F5DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F5DA0 size=116
    let mut pc: u32 = 0x826F5DA0;
    'dispatch: loop {
        match pc {
            0x826F5DA0 => {
    //   block [0x826F5DA0..0x826F5E14)
	// 826F5DA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F5DA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F5DA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F5DAC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F5DB0: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F5DB4: 390BE48C  addi r8, r11, -0x1b74
	ctx.r[8].s64 = ctx.r[11].s64 + -7028;
	// 826F5DB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F5DBC: 392AA4D0  addi r9, r10, -0x5b30
	ctx.r[9].s64 = ctx.r[10].s64 + -23344;
	// 826F5DC0: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F5DC4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826F5DC8: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F5DCC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F5DD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F5DD4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F5DD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F5DDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F5DE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F5DE4: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 826F5DE8: 388A44B0  addi r4, r10, 0x44b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17584;
	// 826F5DEC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F5DF0: 386B9C94  addi r3, r11, -0x636c
	ctx.r[3].s64 = ctx.r[11].s64 + -25452;
	// 826F5DF4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F5DF8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F5DFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F5E00: 4BD71021  bl 0x82466e20
	ctx.lr = 0x826F5E04;
	sub_82466E20(ctx, base);
	// 826F5E04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F5E08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F5E0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F5E10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F5E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826F5E18 size=24
    let mut pc: u32 = 0x826F5E18;
    'dispatch: loop {
        match pc {
            0x826F5E18 => {
    //   block [0x826F5E18..0x826F5E30)
	// 826F5E18: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F5E1C: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F5E20: 394A23C0  addi r10, r10, 0x23c0
	ctx.r[10].s64 = ctx.r[10].s64 + 9152;
	// 826F5E24: 816BE4A4  lwz r11, -0x1b5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7004 as u32) ) } as u64;
	// 826F5E28: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826F5E2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F5E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F5E30 size=116
    let mut pc: u32 = 0x826F5E30;
    'dispatch: loop {
        match pc {
            0x826F5E30 => {
    //   block [0x826F5E30..0x826F5EA4)
	// 826F5E30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F5E34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F5E38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F5E3C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F5E40: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F5E44: 390B23C0  addi r8, r11, 0x23c0
	ctx.r[8].s64 = ctx.r[11].s64 + 9152;
	// 826F5E48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F5E4C: 392AA540  addi r9, r10, -0x5ac0
	ctx.r[9].s64 = ctx.r[10].s64 + -23232;
	// 826F5E50: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F5E54: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 826F5E58: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F5E5C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F5E60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F5E64: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F5E68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F5E6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F5E70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F5E74: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 826F5E78: 388A44DC  addi r4, r10, 0x44dc
	ctx.r[4].s64 = ctx.r[10].s64 + 17628;
	// 826F5E7C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F5E80: 386B9CC4  addi r3, r11, -0x633c
	ctx.r[3].s64 = ctx.r[11].s64 + -25404;
	// 826F5E84: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 826F5E88: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F5E8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F5E90: 4BD70F91  bl 0x82466e20
	ctx.lr = 0x826F5E94;
	sub_82466E20(ctx, base);
	// 826F5E94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F5E98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F5E9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F5EA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F5EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F5EA8 size=108
    let mut pc: u32 = 0x826F5EA8;
    'dispatch: loop {
        match pc {
            0x826F5EA8 => {
    //   block [0x826F5EA8..0x826F5F14)
	// 826F5EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F5EAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F5EB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F5EB4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F5EB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F5EBC: 38EBE4B4  addi r7, r11, -0x1b4c
	ctx.r[7].s64 = ctx.r[11].s64 + -6988;
	// 826F5EC0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F5EC4: 388A44F4  addi r4, r10, 0x44f4
	ctx.r[4].s64 = ctx.r[10].s64 + 17652;
	// 826F5EC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F5ECC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5ED0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F5ED4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F5ED8: 386A9CF4  addi r3, r10, -0x630c
	ctx.r[3].s64 = ctx.r[10].s64 + -25356;
	// 826F5EDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F5EE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F5EE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F5EE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F5EEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F5EF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F5EF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F5EF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F5EFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F5F00: 4BD70F21  bl 0x82466e20
	ctx.lr = 0x826F5F04;
	sub_82466E20(ctx, base);
	// 826F5F04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F5F08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F5F0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F5F10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F5F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F5F18 size=112
    let mut pc: u32 = 0x826F5F18;
    'dispatch: loop {
        match pc {
            0x826F5F18 => {
    //   block [0x826F5F18..0x826F5F88)
	// 826F5F18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F5F1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F5F20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F5F24: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5F28: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F5F2C: 38AA9C94  addi r5, r10, -0x636c
	ctx.r[5].s64 = ctx.r[10].s64 + -25452;
	// 826F5F30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F5F34: 390BE4E4  addi r8, r11, -0x1b1c
	ctx.r[8].s64 = ctx.r[11].s64 + -6940;
	// 826F5F38: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F5F3C: 388A4518  addi r4, r10, 0x4518
	ctx.r[4].s64 = ctx.r[10].s64 + 17688;
	// 826F5F40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F5F44: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5F48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F5F4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F5F50: 386A9D24  addi r3, r10, -0x62dc
	ctx.r[3].s64 = ctx.r[10].s64 + -25308;
	// 826F5F54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F5F58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F5F5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F5F60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F5F64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F5F68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F5F6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F5F70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F5F74: 4BD70EAD  bl 0x82466e20
	ctx.lr = 0x826F5F78;
	sub_82466E20(ctx, base);
	// 826F5F78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F5F7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F5F80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F5F84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F5F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F5F88 size=112
    let mut pc: u32 = 0x826F5F88;
    'dispatch: loop {
        match pc {
            0x826F5F88 => {
    //   block [0x826F5F88..0x826F5FF8)
	// 826F5F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F5F8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F5F90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F5F94: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F5F98: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F5F9C: 392AA598  addi r9, r10, -0x5a68
	ctx.r[9].s64 = ctx.r[10].s64 + -23144;
	// 826F5FA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F5FA4: 390BE500  addi r8, r11, -0x1b00
	ctx.r[8].s64 = ctx.r[11].s64 + -6912;
	// 826F5FA8: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826F5FAC: 388A4538  addi r4, r10, 0x4538
	ctx.r[4].s64 = ctx.r[10].s64 + 17720;
	// 826F5FB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F5FB4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5FB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F5FBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F5FC0: 386A9D54  addi r3, r10, -0x62ac
	ctx.r[3].s64 = ctx.r[10].s64 + -25260;
	// 826F5FC4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F5FC8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F5FCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F5FD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F5FD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F5FD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F5FDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F5FE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F5FE4: 4BD70E3D  bl 0x82466e20
	ctx.lr = 0x826F5FE8;
	sub_82466E20(ctx, base);
	// 826F5FE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F5FEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F5FF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F5FF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F5FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F5FF8 size=112
    let mut pc: u32 = 0x826F5FF8;
    'dispatch: loop {
        match pc {
            0x826F5FF8 => {
    //   block [0x826F5FF8..0x826F6068)
	// 826F5FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F5FFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F6000: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F6004: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6008: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F600C: 38AA9C94  addi r5, r10, -0x636c
	ctx.r[5].s64 = ctx.r[10].s64 + -25452;
	// 826F6010: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F6014: 390BE548  addi r8, r11, -0x1ab8
	ctx.r[8].s64 = ctx.r[11].s64 + -6840;
	// 826F6018: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F601C: 388A4554  addi r4, r10, 0x4554
	ctx.r[4].s64 = ctx.r[10].s64 + 17748;
	// 826F6020: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F6024: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6028: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F602C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F6030: 386A9D84  addi r3, r10, -0x627c
	ctx.r[3].s64 = ctx.r[10].s64 + -25212;
	// 826F6034: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F6038: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F603C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F6040: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F6044: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F6048: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F604C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F6050: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F6054: 4BD70DCD  bl 0x82466e20
	ctx.lr = 0x826F6058;
	sub_82466E20(ctx, base);
	// 826F6058: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F605C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F6060: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F6064: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F6068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F6068 size=112
    let mut pc: u32 = 0x826F6068;
    'dispatch: loop {
        match pc {
            0x826F6068 => {
    //   block [0x826F6068..0x826F60D8)
	// 826F6068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F606C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F6070: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F6074: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F6078: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F607C: 392AA5C4  addi r9, r10, -0x5a3c
	ctx.r[9].s64 = ctx.r[10].s64 + -23100;
	// 826F6080: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F6084: 390BE568  addi r8, r11, -0x1a98
	ctx.r[8].s64 = ctx.r[11].s64 + -6808;
	// 826F6088: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826F608C: 388A456C  addi r4, r10, 0x456c
	ctx.r[4].s64 = ctx.r[10].s64 + 17772;
	// 826F6090: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F6094: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6098: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F609C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F60A0: 386A9DB4  addi r3, r10, -0x624c
	ctx.r[3].s64 = ctx.r[10].s64 + -25164;
	// 826F60A4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F60A8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F60AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F60B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F60B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F60B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F60BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F60C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F60C4: 4BD70D5D  bl 0x82466e20
	ctx.lr = 0x826F60C8;
	sub_82466E20(ctx, base);
	// 826F60C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F60CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F60D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F60D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F60D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F60D8 size=112
    let mut pc: u32 = 0x826F60D8;
    'dispatch: loop {
        match pc {
            0x826F60D8 => {
    //   block [0x826F60D8..0x826F6148)
	// 826F60D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F60DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F60E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F60E4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F60E8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F60EC: 38AA9C94  addi r5, r10, -0x636c
	ctx.r[5].s64 = ctx.r[10].s64 + -25452;
	// 826F60F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F60F4: 390BE5F8  addi r8, r11, -0x1a08
	ctx.r[8].s64 = ctx.r[11].s64 + -6664;
	// 826F60F8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F60FC: 388A4590  addi r4, r10, 0x4590
	ctx.r[4].s64 = ctx.r[10].s64 + 17808;
	// 826F6100: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F6104: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6108: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F610C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F6110: 386A9DE4  addi r3, r10, -0x621c
	ctx.r[3].s64 = ctx.r[10].s64 + -25116;
	// 826F6114: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F6118: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F611C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F6120: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F6124: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F6128: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F612C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F6130: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F6134: 4BD70CED  bl 0x82466e20
	ctx.lr = 0x826F6138;
	sub_82466E20(ctx, base);
	// 826F6138: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F613C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F6140: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F6144: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F6148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F6148 size=112
    let mut pc: u32 = 0x826F6148;
    'dispatch: loop {
        match pc {
            0x826F6148 => {
    //   block [0x826F6148..0x826F61B8)
	// 826F6148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F614C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F6150: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F6154: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6158: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F615C: 38AA9E44  addi r5, r10, -0x61bc
	ctx.r[5].s64 = ctx.r[10].s64 + -25020;
	// 826F6160: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F6164: 390BE610  addi r8, r11, -0x19f0
	ctx.r[8].s64 = ctx.r[11].s64 + -6640;
	// 826F6168: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826F616C: 388A45B0  addi r4, r10, 0x45b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17840;
	// 826F6170: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F6174: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6178: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F617C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F6180: 386A9E14  addi r3, r10, -0x61ec
	ctx.r[3].s64 = ctx.r[10].s64 + -25068;
	// 826F6184: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F6188: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F618C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F6190: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F6194: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F6198: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F619C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F61A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F61A4: 4BD70C7D  bl 0x82466e20
	ctx.lr = 0x826F61A8;
	sub_82466E20(ctx, base);
	// 826F61A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F61AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F61B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F61B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F61B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F61B8 size=100
    let mut pc: u32 = 0x826F61B8;
    'dispatch: loop {
        match pc {
            0x826F61B8 => {
    //   block [0x826F61B8..0x826F621C)
	// 826F61B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F61BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F61C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F61C4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F61C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F61CC: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F61D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F61D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F61D8: 388A45C8  addi r4, r10, 0x45c8
	ctx.r[4].s64 = ctx.r[10].s64 + 17864;
	// 826F61DC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F61E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F61E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F61E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F61EC: 386A9E44  addi r3, r10, -0x61bc
	ctx.r[3].s64 = ctx.r[10].s64 + -25020;
	// 826F61F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F61F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F61F8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F61FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F6200: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F6204: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F6208: 4BD70C19  bl 0x82466e20
	ctx.lr = 0x826F620C;
	sub_82466E20(ctx, base);
	// 826F620C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F6210: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F6214: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F6218: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F6220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826F6220 size=24
    let mut pc: u32 = 0x826F6220;
    'dispatch: loop {
        match pc {
            0x826F6220 => {
    //   block [0x826F6220..0x826F6238)
	// 826F6220: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F6224: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F6228: 394A2498  addi r10, r10, 0x2498
	ctx.r[10].s64 = ctx.r[10].s64 + 9368;
	// 826F622C: 816BE564  lwz r11, -0x1a9c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-6812 as u32) ) } as u64;
	// 826F6230: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826F6234: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F6238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F6238 size=116
    let mut pc: u32 = 0x826F6238;
    'dispatch: loop {
        match pc {
            0x826F6238 => {
    //   block [0x826F6238..0x826F62AC)
	// 826F6238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F623C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F6240: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F6244: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F6248: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F624C: 390B2498  addi r8, r11, 0x2498
	ctx.r[8].s64 = ctx.r[11].s64 + 9368;
	// 826F6250: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F6254: 392AA600  addi r9, r10, -0x5a00
	ctx.r[9].s64 = ctx.r[10].s64 + -23040;
	// 826F6258: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F625C: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826F6260: 38AA9C94  addi r5, r10, -0x636c
	ctx.r[5].s64 = ctx.r[10].s64 + -25452;
	// 826F6264: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F6268: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F626C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F6270: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F6274: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F6278: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F627C: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 826F6280: 388A45EC  addi r4, r10, 0x45ec
	ctx.r[4].s64 = ctx.r[10].s64 + 17900;
	// 826F6284: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F6288: 386B9E74  addi r3, r11, -0x618c
	ctx.r[3].s64 = ctx.r[11].s64 + -24972;
	// 826F628C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F6290: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F6294: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F6298: 4BD70B89  bl 0x82466e20
	ctx.lr = 0x826F629C;
	sub_82466E20(ctx, base);
	// 826F629C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F62A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F62A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F62A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F62B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F62B0 size=108
    let mut pc: u32 = 0x826F62B0;
    'dispatch: loop {
        match pc {
            0x826F62B0 => {
    //   block [0x826F62B0..0x826F631C)
	// 826F62B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F62B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F62B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F62BC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F62C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F62C4: 38EBE688  addi r7, r11, -0x1978
	ctx.r[7].s64 = ctx.r[11].s64 + -6520;
	// 826F62C8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F62CC: 388A4608  addi r4, r10, 0x4608
	ctx.r[4].s64 = ctx.r[10].s64 + 17928;
	// 826F62D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F62D4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F62D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F62DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F62E0: 386A9EA4  addi r3, r10, -0x615c
	ctx.r[3].s64 = ctx.r[10].s64 + -24924;
	// 826F62E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F62E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F62EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F62F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F62F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F62F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F62FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F6300: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F6304: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F6308: 4BD70B19  bl 0x82466e20
	ctx.lr = 0x826F630C;
	sub_82466E20(ctx, base);
	// 826F630C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F6310: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F6314: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F6318: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F6320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F6320 size=112
    let mut pc: u32 = 0x826F6320;
    'dispatch: loop {
        match pc {
            0x826F6320 => {
    //   block [0x826F6320..0x826F6390)
	// 826F6320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F6324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F6328: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F632C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6330: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F6334: 38AA9C94  addi r5, r10, -0x636c
	ctx.r[5].s64 = ctx.r[10].s64 + -25452;
	// 826F6338: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F633C: 390BE6B8  addi r8, r11, -0x1948
	ctx.r[8].s64 = ctx.r[11].s64 + -6472;
	// 826F6340: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F6344: 388A462C  addi r4, r10, 0x462c
	ctx.r[4].s64 = ctx.r[10].s64 + 17964;
	// 826F6348: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F634C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6350: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F6354: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F6358: 386A9ED4  addi r3, r10, -0x612c
	ctx.r[3].s64 = ctx.r[10].s64 + -24876;
	// 826F635C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F6360: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F6364: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F6368: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F636C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F6370: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F6374: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F6378: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F637C: 4BD70AA5  bl 0x82466e20
	ctx.lr = 0x826F6380;
	sub_82466E20(ctx, base);
	// 826F6380: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F6384: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F6388: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F638C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F6390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F6390 size=112
    let mut pc: u32 = 0x826F6390;
    'dispatch: loop {
        match pc {
            0x826F6390 => {
    //   block [0x826F6390..0x826F6400)
	// 826F6390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F6394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F6398: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F639C: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F63A0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F63A4: 392AA624  addi r9, r10, -0x59dc
	ctx.r[9].s64 = ctx.r[10].s64 + -23004;
	// 826F63A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F63AC: 390BE6D8  addi r8, r11, -0x1928
	ctx.r[8].s64 = ctx.r[11].s64 + -6440;
	// 826F63B0: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826F63B4: 388A464C  addi r4, r10, 0x464c
	ctx.r[4].s64 = ctx.r[10].s64 + 17996;
	// 826F63B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F63BC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F63C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F63C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F63C8: 386A9F04  addi r3, r10, -0x60fc
	ctx.r[3].s64 = ctx.r[10].s64 + -24828;
	// 826F63CC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F63D0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F63D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F63D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F63DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F63E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F63E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F63E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F63EC: 4BD70A35  bl 0x82466e20
	ctx.lr = 0x826F63F0;
	sub_82466E20(ctx, base);
	// 826F63F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F63F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F63F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F63FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F6400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F6400 size=112
    let mut pc: u32 = 0x826F6400;
    'dispatch: loop {
        match pc {
            0x826F6400 => {
    //   block [0x826F6400..0x826F6470)
	// 826F6400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F6404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F6408: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F640C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6410: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F6414: 38AA9C94  addi r5, r10, -0x636c
	ctx.r[5].s64 = ctx.r[10].s64 + -25452;
	// 826F6418: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F641C: 390BE780  addi r8, r11, -0x1880
	ctx.r[8].s64 = ctx.r[11].s64 + -6272;
	// 826F6420: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F6424: 388A466C  addi r4, r10, 0x466c
	ctx.r[4].s64 = ctx.r[10].s64 + 18028;
	// 826F6428: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F642C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6430: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F6434: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F6438: 386A9F34  addi r3, r10, -0x60cc
	ctx.r[3].s64 = ctx.r[10].s64 + -24780;
	// 826F643C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F6440: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F6444: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F6448: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F644C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F6450: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F6454: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F6458: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F645C: 4BD709C5  bl 0x82466e20
	ctx.lr = 0x826F6460;
	sub_82466E20(ctx, base);
	// 826F6460: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F6464: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F6468: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F646C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F6470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F6470 size=108
    let mut pc: u32 = 0x826F6470;
    'dispatch: loop {
        match pc {
            0x826F6470 => {
    //   block [0x826F6470..0x826F64DC)
	// 826F6470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F6474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F6478: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F647C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F6480: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F6484: 38EBE798  addi r7, r11, -0x1868
	ctx.r[7].s64 = ctx.r[11].s64 + -6248;
	// 826F6488: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F648C: 388A4688  addi r4, r10, 0x4688
	ctx.r[4].s64 = ctx.r[10].s64 + 18056;
	// 826F6490: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F6494: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6498: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F649C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F64A0: 386A9F64  addi r3, r10, -0x609c
	ctx.r[3].s64 = ctx.r[10].s64 + -24732;
	// 826F64A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F64A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F64AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F64B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F64B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F64B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F64BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F64C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F64C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F64C8: 4BD70959  bl 0x82466e20
	ctx.lr = 0x826F64CC;
	sub_82466E20(ctx, base);
	// 826F64CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F64D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F64D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F64D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F64E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F64E0 size=112
    let mut pc: u32 = 0x826F64E0;
    'dispatch: loop {
        match pc {
            0x826F64E0 => {
    //   block [0x826F64E0..0x826F6550)
	// 826F64E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F64E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F64E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F64EC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F64F0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F64F4: 38AA9C94  addi r5, r10, -0x636c
	ctx.r[5].s64 = ctx.r[10].s64 + -25452;
	// 826F64F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F64FC: 390BE7C8  addi r8, r11, -0x1838
	ctx.r[8].s64 = ctx.r[11].s64 + -6200;
	// 826F6500: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F6504: 388A46AC  addi r4, r10, 0x46ac
	ctx.r[4].s64 = ctx.r[10].s64 + 18092;
	// 826F6508: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F650C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6510: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F6514: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F6518: 386A9F94  addi r3, r10, -0x606c
	ctx.r[3].s64 = ctx.r[10].s64 + -24684;
	// 826F651C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F6520: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F6524: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F6528: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F652C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F6530: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F6534: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F6538: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F653C: 4BD708E5  bl 0x82466e20
	ctx.lr = 0x826F6540;
	sub_82466E20(ctx, base);
	// 826F6540: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F6544: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F6548: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F654C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F6550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F6550 size=112
    let mut pc: u32 = 0x826F6550;
    'dispatch: loop {
        match pc {
            0x826F6550 => {
    //   block [0x826F6550..0x826F65C0)
	// 826F6550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F6554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F6558: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F655C: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F6560: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F6564: 392AA658  addi r9, r10, -0x59a8
	ctx.r[9].s64 = ctx.r[10].s64 + -22952;
	// 826F6568: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F656C: 390BE7E0  addi r8, r11, -0x1820
	ctx.r[8].s64 = ctx.r[11].s64 + -6176;
	// 826F6570: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826F6574: 388A46C8  addi r4, r10, 0x46c8
	ctx.r[4].s64 = ctx.r[10].s64 + 18120;
	// 826F6578: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F657C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6580: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F6584: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F6588: 386A9FC4  addi r3, r10, -0x603c
	ctx.r[3].s64 = ctx.r[10].s64 + -24636;
	// 826F658C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F6590: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F6594: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F6598: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F659C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F65A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F65A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F65A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F65AC: 4BD70875  bl 0x82466e20
	ctx.lr = 0x826F65B0;
	sub_82466E20(ctx, base);
	// 826F65B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F65B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F65B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F65BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F65C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F65C0 size=112
    let mut pc: u32 = 0x826F65C0;
    'dispatch: loop {
        match pc {
            0x826F65C0 => {
    //   block [0x826F65C0..0x826F6630)
	// 826F65C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F65C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F65C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F65CC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F65D0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F65D4: 38AA9C94  addi r5, r10, -0x636c
	ctx.r[5].s64 = ctx.r[10].s64 + -25452;
	// 826F65D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F65DC: 390BE888  addi r8, r11, -0x1778
	ctx.r[8].s64 = ctx.r[11].s64 + -6008;
	// 826F65E0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826F65E4: 388A46E4  addi r4, r10, 0x46e4
	ctx.r[4].s64 = ctx.r[10].s64 + 18148;
	// 826F65E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F65EC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F65F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F65F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F65F8: 386A9FF4  addi r3, r10, -0x600c
	ctx.r[3].s64 = ctx.r[10].s64 + -24588;
	// 826F65FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F6600: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F6604: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F6608: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F660C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F6610: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F6614: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F6618: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F661C: 4BD70805  bl 0x82466e20
	ctx.lr = 0x826F6620;
	sub_82466E20(ctx, base);
	// 826F6620: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F6624: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F6628: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F662C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F6630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F6630 size=112
    let mut pc: u32 = 0x826F6630;
    'dispatch: loop {
        match pc {
            0x826F6630 => {
    //   block [0x826F6630..0x826F66A0)
	// 826F6630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F6634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F6638: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F663C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6640: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F6644: 38AA9C94  addi r5, r10, -0x636c
	ctx.r[5].s64 = ctx.r[10].s64 + -25452;
	// 826F6648: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F664C: 390BE8D0  addi r8, r11, -0x1730
	ctx.r[8].s64 = ctx.r[11].s64 + -5936;
	// 826F6650: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 826F6654: 388A46FC  addi r4, r10, 0x46fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18172;
	// 826F6658: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F665C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6660: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F6664: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F6668: 386AA024  addi r3, r10, -0x5fdc
	ctx.r[3].s64 = ctx.r[10].s64 + -24540;
	// 826F666C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F6670: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F6674: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F6678: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F667C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F6680: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F6684: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F6688: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F668C: 4BD70795  bl 0x82466e20
	ctx.lr = 0x826F6690;
	sub_82466E20(ctx, base);
	// 826F6690: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F6694: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F6698: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F669C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F66A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F66A0 size=100
    let mut pc: u32 = 0x826F66A0;
    'dispatch: loop {
        match pc {
            0x826F66A0 => {
    //   block [0x826F66A0..0x826F6704)
	// 826F66A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F66A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F66A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F66AC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F66B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F66B4: 38AA9C94  addi r5, r10, -0x636c
	ctx.r[5].s64 = ctx.r[10].s64 + -25452;
	// 826F66B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F66BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F66C0: 388A4718  addi r4, r10, 0x4718
	ctx.r[4].s64 = ctx.r[10].s64 + 18200;
	// 826F66C4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F66C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F66CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F66D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F66D4: 386AA054  addi r3, r10, -0x5fac
	ctx.r[3].s64 = ctx.r[10].s64 + -24492;
	// 826F66D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F66DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F66E0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F66E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F66E8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F66EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F66F0: 4BD70731  bl 0x82466e20
	ctx.lr = 0x826F66F4;
	sub_82466E20(ctx, base);
	// 826F66F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F66F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F66FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F6700: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F6708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F6708 size=112
    let mut pc: u32 = 0x826F6708;
    'dispatch: loop {
        match pc {
            0x826F6708 => {
    //   block [0x826F6708..0x826F6778)
	// 826F6708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F670C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F6710: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F6714: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6718: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F671C: 38AA9CC4  addi r5, r10, -0x633c
	ctx.r[5].s64 = ctx.r[10].s64 + -25404;
	// 826F6720: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F6724: 390BE990  addi r8, r11, -0x1670
	ctx.r[8].s64 = ctx.r[11].s64 + -5744;
	// 826F6728: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826F672C: 388A4730  addi r4, r10, 0x4730
	ctx.r[4].s64 = ctx.r[10].s64 + 18224;
	// 826F6730: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F6734: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6738: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F673C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F6740: 386AA084  addi r3, r10, -0x5f7c
	ctx.r[3].s64 = ctx.r[10].s64 + -24444;
	// 826F6744: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F6748: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F674C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F6750: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F6754: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F6758: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F675C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F6760: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F6764: 4BD706BD  bl 0x82466e20
	ctx.lr = 0x826F6768;
	sub_82466E20(ctx, base);
	// 826F6768: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F676C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F6770: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F6774: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F6778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F6778 size=112
    let mut pc: u32 = 0x826F6778;
    'dispatch: loop {
        match pc {
            0x826F6778 => {
    //   block [0x826F6778..0x826F67E8)
	// 826F6778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F677C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F6780: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F6784: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6788: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F678C: 38AA9B44  addi r5, r10, -0x64bc
	ctx.r[5].s64 = ctx.r[10].s64 + -25788;
	// 826F6790: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F6794: 390BE9C0  addi r8, r11, -0x1640
	ctx.r[8].s64 = ctx.r[11].s64 + -5696;
	// 826F6798: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F679C: 388A474C  addi r4, r10, 0x474c
	ctx.r[4].s64 = ctx.r[10].s64 + 18252;
	// 826F67A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F67A4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F67A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F67AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F67B0: 386AA0B4  addi r3, r10, -0x5f4c
	ctx.r[3].s64 = ctx.r[10].s64 + -24396;
	// 826F67B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F67B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F67BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F67C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F67C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F67C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F67CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F67D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F67D4: 4BD7064D  bl 0x82466e20
	ctx.lr = 0x826F67D8;
	sub_82466E20(ctx, base);
	// 826F67D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F67DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F67E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F67E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F67E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F67E8 size=108
    let mut pc: u32 = 0x826F67E8;
    'dispatch: loop {
        match pc {
            0x826F67E8 => {
    //   block [0x826F67E8..0x826F6854)
	// 826F67E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F67EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F67F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F67F4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F67F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F67FC: 38EBE9D8  addi r7, r11, -0x1628
	ctx.r[7].s64 = ctx.r[11].s64 + -5672;
	// 826F6800: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F6804: 388A476C  addi r4, r10, 0x476c
	ctx.r[4].s64 = ctx.r[10].s64 + 18284;
	// 826F6808: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F680C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6810: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F6814: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F6818: 386AA0E4  addi r3, r10, -0x5f1c
	ctx.r[3].s64 = ctx.r[10].s64 + -24348;
	// 826F681C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F6820: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F6824: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F6828: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F682C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F6830: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F6834: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F6838: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F683C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F6840: 4BD705E1  bl 0x82466e20
	ctx.lr = 0x826F6844;
	sub_82466E20(ctx, base);
	// 826F6844: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F6848: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F684C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F6850: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F6858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F6858 size=112
    let mut pc: u32 = 0x826F6858;
    'dispatch: loop {
        match pc {
            0x826F6858 => {
    //   block [0x826F6858..0x826F68C8)
	// 826F6858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F685C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F6860: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F6864: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6868: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F686C: 38AAA054  addi r5, r10, -0x5fac
	ctx.r[5].s64 = ctx.r[10].s64 + -24492;
	// 826F6870: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F6874: 390BEA08  addi r8, r11, -0x15f8
	ctx.r[8].s64 = ctx.r[11].s64 + -5624;
	// 826F6878: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826F687C: 388A4790  addi r4, r10, 0x4790
	ctx.r[4].s64 = ctx.r[10].s64 + 18320;
	// 826F6880: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F6884: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6888: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F688C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F6890: 386AA114  addi r3, r10, -0x5eec
	ctx.r[3].s64 = ctx.r[10].s64 + -24300;
	// 826F6894: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F6898: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F689C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F68A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F68A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F68A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F68AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F68B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F68B4: 4BD7056D  bl 0x82466e20
	ctx.lr = 0x826F68B8;
	sub_82466E20(ctx, base);
	// 826F68B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F68BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F68C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F68C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F68C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F68C8 size=112
    let mut pc: u32 = 0x826F68C8;
    'dispatch: loop {
        match pc {
            0x826F68C8 => {
    //   block [0x826F68C8..0x826F6938)
	// 826F68C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F68CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F68D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F68D4: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F68D8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F68DC: 392AA684  addi r9, r10, -0x597c
	ctx.r[9].s64 = ctx.r[10].s64 + -22908;
	// 826F68E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F68E4: 390BEAA0  addi r8, r11, -0x1560
	ctx.r[8].s64 = ctx.r[11].s64 + -5472;
	// 826F68E8: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826F68EC: 388A47A8  addi r4, r10, 0x47a8
	ctx.r[4].s64 = ctx.r[10].s64 + 18344;
	// 826F68F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F68F4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F68F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F68FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F6900: 386AA144  addi r3, r10, -0x5ebc
	ctx.r[3].s64 = ctx.r[10].s64 + -24252;
	// 826F6904: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F6908: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F690C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F6910: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F6914: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F6918: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F691C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F6920: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F6924: 4BD704FD  bl 0x82466e20
	ctx.lr = 0x826F6928;
	sub_82466E20(ctx, base);
	// 826F6928: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F692C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F6930: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F6934: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F6938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F6938 size=112
    let mut pc: u32 = 0x826F6938;
    'dispatch: loop {
        match pc {
            0x826F6938 => {
    //   block [0x826F6938..0x826F69A8)
	// 826F6938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F693C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F6940: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F6944: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6948: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F694C: 38AA9C94  addi r5, r10, -0x636c
	ctx.r[5].s64 = ctx.r[10].s64 + -25452;
	// 826F6950: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F6954: 390BEAE8  addi r8, r11, -0x1518
	ctx.r[8].s64 = ctx.r[11].s64 + -5400;
	// 826F6958: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F695C: 388A47C0  addi r4, r10, 0x47c0
	ctx.r[4].s64 = ctx.r[10].s64 + 18368;
	// 826F6960: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F6964: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6968: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F696C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F6970: 386AA174  addi r3, r10, -0x5e8c
	ctx.r[3].s64 = ctx.r[10].s64 + -24204;
	// 826F6974: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F6978: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F697C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F6980: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F6984: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F6988: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F698C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F6990: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F6994: 4BD7048D  bl 0x82466e20
	ctx.lr = 0x826F6998;
	sub_82466E20(ctx, base);
	// 826F6998: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F699C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F69A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F69A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F69A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F69A8 size=108
    let mut pc: u32 = 0x826F69A8;
    'dispatch: loop {
        match pc {
            0x826F69A8 => {
    //   block [0x826F69A8..0x826F6A14)
	// 826F69A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F69AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F69B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F69B4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F69B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F69BC: 38EBEB00  addi r7, r11, -0x1500
	ctx.r[7].s64 = ctx.r[11].s64 + -5376;
	// 826F69C0: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826F69C4: 388A47D4  addi r4, r10, 0x47d4
	ctx.r[4].s64 = ctx.r[10].s64 + 18388;
	// 826F69C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F69CC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F69D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F69D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F69D8: 386AA1A4  addi r3, r10, -0x5e5c
	ctx.r[3].s64 = ctx.r[10].s64 + -24156;
	// 826F69DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F69E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F69E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F69E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F69EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F69F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F69F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F69F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F69FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F6A00: 4BD70421  bl 0x82466e20
	ctx.lr = 0x826F6A04;
	sub_82466E20(ctx, base);
	// 826F6A04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F6A08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F6A0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F6A10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F6A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F6A18 size=116
    let mut pc: u32 = 0x826F6A18;
    'dispatch: loop {
        match pc {
            0x826F6A18 => {
    //   block [0x826F6A18..0x826F6A8C)
	// 826F6A18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F6A1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F6A20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F6A24: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F6A28: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 826F6A2C: 390AEB90  addi r8, r10, -0x1470
	ctx.r[8].s64 = ctx.r[10].s64 + -5232;
	// 826F6A30: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6A34: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826F6A38: 38AAA054  addi r5, r10, -0x5fac
	ctx.r[5].s64 = ctx.r[10].s64 + -24492;
	// 826F6A3C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F6A40: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F6A44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F6A48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F6A4C: 388A47F8  addi r4, r10, 0x47f8
	ctx.r[4].s64 = ctx.r[10].s64 + 18424;
	// 826F6A50: 396BA698  addi r11, r11, -0x5968
	ctx.r[11].s64 = ctx.r[11].s64 + -22888;
	// 826F6A54: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6A58: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F6A5C: 386AA1D4  addi r3, r10, -0x5e2c
	ctx.r[3].s64 = ctx.r[10].s64 + -24108;
	// 826F6A60: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826F6A64: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F6A68: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826F6A6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F6A70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F6A74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F6A78: 4BD703A9  bl 0x82466e20
	ctx.lr = 0x826F6A7C;
	sub_82466E20(ctx, base);
	// 826F6A7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F6A80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F6A84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F6A88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F6A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F6A90 size=108
    let mut pc: u32 = 0x826F6A90;
    'dispatch: loop {
        match pc {
            0x826F6A90 => {
    //   block [0x826F6A90..0x826F6AFC)
	// 826F6A90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F6A94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F6A98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F6A9C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F6AA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F6AA4: 38EBEC68  addi r7, r11, -0x1398
	ctx.r[7].s64 = ctx.r[11].s64 + -5016;
	// 826F6AA8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826F6AAC: 388A480C  addi r4, r10, 0x480c
	ctx.r[4].s64 = ctx.r[10].s64 + 18444;
	// 826F6AB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F6AB4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6AB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F6ABC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F6AC0: 386AA204  addi r3, r10, -0x5dfc
	ctx.r[3].s64 = ctx.r[10].s64 + -24060;
	// 826F6AC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F6AC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F6ACC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F6AD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F6AD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F6AD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F6ADC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F6AE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F6AE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F6AE8: 4BD70339  bl 0x82466e20
	ctx.lr = 0x826F6AEC;
	sub_82466E20(ctx, base);
	// 826F6AEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F6AF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F6AF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F6AF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F6B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F6B00 size=112
    let mut pc: u32 = 0x826F6B00;
    'dispatch: loop {
        match pc {
            0x826F6B00 => {
    //   block [0x826F6B00..0x826F6B70)
	// 826F6B00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F6B04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F6B08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F6B0C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6B10: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F6B14: 38AAA054  addi r5, r10, -0x5fac
	ctx.r[5].s64 = ctx.r[10].s64 + -24492;
	// 826F6B18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F6B1C: 390BECB0  addi r8, r11, -0x1350
	ctx.r[8].s64 = ctx.r[11].s64 + -4944;
	// 826F6B20: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826F6B24: 388A4834  addi r4, r10, 0x4834
	ctx.r[4].s64 = ctx.r[10].s64 + 18484;
	// 826F6B28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F6B2C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6B30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F6B34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F6B38: 386AA234  addi r3, r10, -0x5dcc
	ctx.r[3].s64 = ctx.r[10].s64 + -24012;
	// 826F6B3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F6B40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F6B44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F6B48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F6B4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F6B50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F6B54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F6B58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F6B5C: 4BD702C5  bl 0x82466e20
	ctx.lr = 0x826F6B60;
	sub_82466E20(ctx, base);
	// 826F6B60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F6B64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F6B68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F6B6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F6B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F6B70 size=112
    let mut pc: u32 = 0x826F6B70;
    'dispatch: loop {
        match pc {
            0x826F6B70 => {
    //   block [0x826F6B70..0x826F6BE0)
	// 826F6B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F6B74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F6B78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F6B7C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6B80: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F6B84: 38AA9C94  addi r5, r10, -0x636c
	ctx.r[5].s64 = ctx.r[10].s64 + -25452;
	// 826F6B88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F6B8C: 390BED28  addi r8, r11, -0x12d8
	ctx.r[8].s64 = ctx.r[11].s64 + -4824;
	// 826F6B90: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826F6B94: 388A484C  addi r4, r10, 0x484c
	ctx.r[4].s64 = ctx.r[10].s64 + 18508;
	// 826F6B98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F6B9C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6BA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F6BA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F6BA8: 386AA264  addi r3, r10, -0x5d9c
	ctx.r[3].s64 = ctx.r[10].s64 + -23964;
	// 826F6BAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F6BB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F6BB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F6BB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F6BBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F6BC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F6BC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F6BC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F6BCC: 4BD70255  bl 0x82466e20
	ctx.lr = 0x826F6BD0;
	sub_82466E20(ctx, base);
	// 826F6BD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F6BD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F6BD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F6BDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F6BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F6BE0 size=108
    let mut pc: u32 = 0x826F6BE0;
    'dispatch: loop {
        match pc {
            0x826F6BE0 => {
    //   block [0x826F6BE0..0x826F6C4C)
	// 826F6BE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F6BE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F6BE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F6BEC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F6BF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F6BF4: 38EBED58  addi r7, r11, -0x12a8
	ctx.r[7].s64 = ctx.r[11].s64 + -4776;
	// 826F6BF8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826F6BFC: 388A4864  addi r4, r10, 0x4864
	ctx.r[4].s64 = ctx.r[10].s64 + 18532;
	// 826F6C00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F6C04: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6C08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F6C0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F6C10: 386AA294  addi r3, r10, -0x5d6c
	ctx.r[3].s64 = ctx.r[10].s64 + -23916;
	// 826F6C14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F6C18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F6C1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F6C20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F6C24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F6C28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F6C2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F6C30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F6C34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F6C38: 4BD701E9  bl 0x82466e20
	ctx.lr = 0x826F6C3C;
	sub_82466E20(ctx, base);
	// 826F6C3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F6C40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F6C44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F6C48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F6C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F6C50 size=108
    let mut pc: u32 = 0x826F6C50;
    'dispatch: loop {
        match pc {
            0x826F6C50 => {
    //   block [0x826F6C50..0x826F6CBC)
	// 826F6C50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F6C54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F6C58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F6C5C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F6C60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F6C64: 38EBEDB8  addi r7, r11, -0x1248
	ctx.r[7].s64 = ctx.r[11].s64 + -4680;
	// 826F6C68: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826F6C6C: 388A4890  addi r4, r10, 0x4890
	ctx.r[4].s64 = ctx.r[10].s64 + 18576;
	// 826F6C70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F6C74: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6C78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F6C7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F6C80: 386AA2C4  addi r3, r10, -0x5d3c
	ctx.r[3].s64 = ctx.r[10].s64 + -23868;
	// 826F6C84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F6C88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F6C8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F6C90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F6C94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F6C98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F6C9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F6CA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F6CA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F6CA8: 4BD70179  bl 0x82466e20
	ctx.lr = 0x826F6CAC;
	sub_82466E20(ctx, base);
	// 826F6CAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F6CB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F6CB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F6CB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F6CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F6CC0 size=112
    let mut pc: u32 = 0x826F6CC0;
    'dispatch: loop {
        match pc {
            0x826F6CC0 => {
    //   block [0x826F6CC0..0x826F6D30)
	// 826F6CC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F6CC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F6CC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F6CCC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6CD0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F6CD4: 38AA9C94  addi r5, r10, -0x636c
	ctx.r[5].s64 = ctx.r[10].s64 + -25452;
	// 826F6CD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F6CDC: 390BEE30  addi r8, r11, -0x11d0
	ctx.r[8].s64 = ctx.r[11].s64 + -4560;
	// 826F6CE0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826F6CE4: 388A48B0  addi r4, r10, 0x48b0
	ctx.r[4].s64 = ctx.r[10].s64 + 18608;
	// 826F6CE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F6CEC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6CF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F6CF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F6CF8: 386AA2F4  addi r3, r10, -0x5d0c
	ctx.r[3].s64 = ctx.r[10].s64 + -23820;
	// 826F6CFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F6D00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F6D04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F6D08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F6D0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F6D10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F6D14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F6D18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F6D1C: 4BD70105  bl 0x82466e20
	ctx.lr = 0x826F6D20;
	sub_82466E20(ctx, base);
	// 826F6D20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F6D24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F6D28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F6D2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F6D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826F6D30 size=24
    let mut pc: u32 = 0x826F6D30;
    'dispatch: loop {
        match pc {
            0x826F6D30 => {
    //   block [0x826F6D30..0x826F6D48)
	// 826F6D30: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F6D34: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F6D38: 394A2510  addi r10, r10, 0x2510
	ctx.r[10].s64 = ctx.r[10].s64 + 9488;
	// 826F6D3C: 816BEA9C  lwz r11, -0x1564(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-5476 as u32) ) } as u64;
	// 826F6D40: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826F6D44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F6D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F6D48 size=116
    let mut pc: u32 = 0x826F6D48;
    'dispatch: loop {
        match pc {
            0x826F6D48 => {
    //   block [0x826F6D48..0x826F6DBC)
	// 826F6D48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F6D4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F6D50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F6D54: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F6D58: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F6D5C: 390B2510  addi r8, r11, 0x2510
	ctx.r[8].s64 = ctx.r[11].s64 + 9488;
	// 826F6D60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F6D64: 392AA6FC  addi r9, r10, -0x5904
	ctx.r[9].s64 = ctx.r[10].s64 + -22788;
	// 826F6D68: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F6D6C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826F6D70: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F6D74: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F6D78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F6D7C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F6D80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F6D84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F6D88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F6D8C: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 826F6D90: 388A48CC  addi r4, r10, 0x48cc
	ctx.r[4].s64 = ctx.r[10].s64 + 18636;
	// 826F6D94: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F6D98: 386BA324  addi r3, r11, -0x5cdc
	ctx.r[3].s64 = ctx.r[11].s64 + -23772;
	// 826F6D9C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F6DA0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F6DA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F6DA8: 4BD70079  bl 0x82466e20
	ctx.lr = 0x826F6DAC;
	sub_82466E20(ctx, base);
	// 826F6DAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F6DB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F6DB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F6DB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F6DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F6DC0 size=112
    let mut pc: u32 = 0x826F6DC0;
    'dispatch: loop {
        match pc {
            0x826F6DC0 => {
    //   block [0x826F6DC0..0x826F6E30)
	// 826F6DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F6DC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F6DC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F6DCC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6DD0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F6DD4: 38AAA324  addi r5, r10, -0x5cdc
	ctx.r[5].s64 = ctx.r[10].s64 + -23772;
	// 826F6DD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F6DDC: 390BEE78  addi r8, r11, -0x1188
	ctx.r[8].s64 = ctx.r[11].s64 + -4488;
	// 826F6DE0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826F6DE4: 388A48E0  addi r4, r10, 0x48e0
	ctx.r[4].s64 = ctx.r[10].s64 + 18656;
	// 826F6DE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F6DEC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6DF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F6DF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F6DF8: 386AA354  addi r3, r10, -0x5cac
	ctx.r[3].s64 = ctx.r[10].s64 + -23724;
	// 826F6DFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F6E00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F6E04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F6E08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F6E0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F6E10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F6E14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F6E18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F6E1C: 4BD70005  bl 0x82466e20
	ctx.lr = 0x826F6E20;
	sub_82466E20(ctx, base);
	// 826F6E20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F6E24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F6E28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F6E2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F6E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826F6E30 size=24
    let mut pc: u32 = 0x826F6E30;
    'dispatch: loop {
        match pc {
            0x826F6E30 => {
    //   block [0x826F6E30..0x826F6E48)
	// 826F6E30: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F6E34: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F6E38: 394A2528  addi r10, r10, 0x2528
	ctx.r[10].s64 = ctx.r[10].s64 + 9512;
	// 826F6E3C: 816BEEA8  lwz r11, -0x1158(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4440 as u32) ) } as u64;
	// 826F6E40: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826F6E44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F6E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F6E48 size=116
    let mut pc: u32 = 0x826F6E48;
    'dispatch: loop {
        match pc {
            0x826F6E48 => {
    //   block [0x826F6E48..0x826F6EBC)
	// 826F6E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F6E4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F6E50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F6E54: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F6E58: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F6E5C: 390B2528  addi r8, r11, 0x2528
	ctx.r[8].s64 = ctx.r[11].s64 + 9512;
	// 826F6E60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F6E64: 392AA738  addi r9, r10, -0x58c8
	ctx.r[9].s64 = ctx.r[10].s64 + -22728;
	// 826F6E68: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6E6C: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826F6E70: 38AAA354  addi r5, r10, -0x5cac
	ctx.r[5].s64 = ctx.r[10].s64 + -23724;
	// 826F6E74: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F6E78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F6E7C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F6E80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F6E84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F6E88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F6E8C: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 826F6E90: 388A490C  addi r4, r10, 0x490c
	ctx.r[4].s64 = ctx.r[10].s64 + 18700;
	// 826F6E94: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F6E98: 386BA384  addi r3, r11, -0x5c7c
	ctx.r[3].s64 = ctx.r[11].s64 + -23676;
	// 826F6E9C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F6EA0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F6EA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F6EA8: 4BD6FF79  bl 0x82466e20
	ctx.lr = 0x826F6EAC;
	sub_82466E20(ctx, base);
	// 826F6EAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F6EB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F6EB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F6EB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F6EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F6EC0 size=112
    let mut pc: u32 = 0x826F6EC0;
    'dispatch: loop {
        match pc {
            0x826F6EC0 => {
    //   block [0x826F6EC0..0x826F6F30)
	// 826F6EC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F6EC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F6EC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F6ECC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6ED0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F6ED4: 38AAA354  addi r5, r10, -0x5cac
	ctx.r[5].s64 = ctx.r[10].s64 + -23724;
	// 826F6ED8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F6EDC: 390BEEB0  addi r8, r11, -0x1150
	ctx.r[8].s64 = ctx.r[11].s64 + -4432;
	// 826F6EE0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826F6EE4: 388A4928  addi r4, r10, 0x4928
	ctx.r[4].s64 = ctx.r[10].s64 + 18728;
	// 826F6EE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F6EEC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6EF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F6EF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F6EF8: 386AA3B4  addi r3, r10, -0x5c4c
	ctx.r[3].s64 = ctx.r[10].s64 + -23628;
	// 826F6EFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F6F00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F6F04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F6F08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F6F0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F6F10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F6F14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F6F18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F6F1C: 4BD6FF05  bl 0x82466e20
	ctx.lr = 0x826F6F20;
	sub_82466E20(ctx, base);
	// 826F6F20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F6F24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F6F28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F6F2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F6F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F6F30 size=112
    let mut pc: u32 = 0x826F6F30;
    'dispatch: loop {
        match pc {
            0x826F6F30 => {
    //   block [0x826F6F30..0x826F6FA0)
	// 826F6F30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F6F34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F6F38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F6F3C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6F40: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F6F44: 38AAA354  addi r5, r10, -0x5cac
	ctx.r[5].s64 = ctx.r[10].s64 + -23724;
	// 826F6F48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F6F4C: 390BEF10  addi r8, r11, -0x10f0
	ctx.r[8].s64 = ctx.r[11].s64 + -4336;
	// 826F6F50: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826F6F54: 388A4944  addi r4, r10, 0x4944
	ctx.r[4].s64 = ctx.r[10].s64 + 18756;
	// 826F6F58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F6F5C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6F60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F6F64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F6F68: 386AA3E4  addi r3, r10, -0x5c1c
	ctx.r[3].s64 = ctx.r[10].s64 + -23580;
	// 826F6F6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F6F70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F6F74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F6F78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F6F7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F6F80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F6F84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F6F88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F6F8C: 4BD6FE95  bl 0x82466e20
	ctx.lr = 0x826F6F90;
	sub_82466E20(ctx, base);
	// 826F6F90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F6F94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F6F98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F6F9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F6FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F6FA0 size=112
    let mut pc: u32 = 0x826F6FA0;
    'dispatch: loop {
        match pc {
            0x826F6FA0 => {
    //   block [0x826F6FA0..0x826F7010)
	// 826F6FA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F6FA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F6FA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F6FAC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6FB0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F6FB4: 38AAA354  addi r5, r10, -0x5cac
	ctx.r[5].s64 = ctx.r[10].s64 + -23724;
	// 826F6FB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F6FBC: 390BEF40  addi r8, r11, -0x10c0
	ctx.r[8].s64 = ctx.r[11].s64 + -4288;
	// 826F6FC0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826F6FC4: 388A4964  addi r4, r10, 0x4964
	ctx.r[4].s64 = ctx.r[10].s64 + 18788;
	// 826F6FC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F6FCC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6FD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F6FD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F6FD8: 386AA414  addi r3, r10, -0x5bec
	ctx.r[3].s64 = ctx.r[10].s64 + -23532;
	// 826F6FDC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F6FE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F6FE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F6FE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F6FEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F6FF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F6FF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F6FF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F6FFC: 4BD6FE25  bl 0x82466e20
	ctx.lr = 0x826F7000;
	sub_82466E20(ctx, base);
	// 826F7000: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F7004: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F7008: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F700C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F7010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F7010 size=108
    let mut pc: u32 = 0x826F7010;
    'dispatch: loop {
        match pc {
            0x826F7010 => {
    //   block [0x826F7010..0x826F707C)
	// 826F7010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F7014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F7018: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F701C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F7020: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F7024: 38EBEF88  addi r7, r11, -0x1078
	ctx.r[7].s64 = ctx.r[11].s64 + -4216;
	// 826F7028: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F702C: 388A4980  addi r4, r10, 0x4980
	ctx.r[4].s64 = ctx.r[10].s64 + 18816;
	// 826F7030: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F7034: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7038: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F703C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F7040: 386AA444  addi r3, r10, -0x5bbc
	ctx.r[3].s64 = ctx.r[10].s64 + -23484;
	// 826F7044: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F7048: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F704C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F7050: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F7054: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F7058: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F705C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F7060: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F7064: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F7068: 4BD6FDB9  bl 0x82466e20
	ctx.lr = 0x826F706C;
	sub_82466E20(ctx, base);
	// 826F706C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F7070: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F7074: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F7078: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F7080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F7080 size=112
    let mut pc: u32 = 0x826F7080;
    'dispatch: loop {
        match pc {
            0x826F7080 => {
    //   block [0x826F7080..0x826F70F0)
	// 826F7080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F7084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F7088: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F708C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7090: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F7094: 38AA9C94  addi r5, r10, -0x636c
	ctx.r[5].s64 = ctx.r[10].s64 + -25452;
	// 826F7098: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F709C: 390BEFB8  addi r8, r11, -0x1048
	ctx.r[8].s64 = ctx.r[11].s64 + -4168;
	// 826F70A0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F70A4: 388A499C  addi r4, r10, 0x499c
	ctx.r[4].s64 = ctx.r[10].s64 + 18844;
	// 826F70A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F70AC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F70B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F70B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F70B8: 386AA474  addi r3, r10, -0x5b8c
	ctx.r[3].s64 = ctx.r[10].s64 + -23436;
	// 826F70BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F70C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F70C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F70C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F70CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F70D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F70D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F70D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F70DC: 4BD6FD45  bl 0x82466e20
	ctx.lr = 0x826F70E0;
	sub_82466E20(ctx, base);
	// 826F70E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F70E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F70E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F70EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F70F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F70F0 size=108
    let mut pc: u32 = 0x826F70F0;
    'dispatch: loop {
        match pc {
            0x826F70F0 => {
    //   block [0x826F70F0..0x826F715C)
	// 826F70F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F70F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F70F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F70FC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F7100: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F7104: 38EBEFD0  addi r7, r11, -0x1030
	ctx.r[7].s64 = ctx.r[11].s64 + -4144;
	// 826F7108: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826F710C: 388A49B4  addi r4, r10, 0x49b4
	ctx.r[4].s64 = ctx.r[10].s64 + 18868;
	// 826F7110: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F7114: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7118: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F711C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F7120: 386AA4A4  addi r3, r10, -0x5b5c
	ctx.r[3].s64 = ctx.r[10].s64 + -23388;
	// 826F7124: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F7128: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F712C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F7130: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F7134: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F7138: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F713C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F7140: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F7144: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F7148: 4BD6FCD9  bl 0x82466e20
	ctx.lr = 0x826F714C;
	sub_82466E20(ctx, base);
	// 826F714C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F7150: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F7154: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F7158: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F7160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F7160 size=108
    let mut pc: u32 = 0x826F7160;
    'dispatch: loop {
        match pc {
            0x826F7160 => {
    //   block [0x826F7160..0x826F71CC)
	// 826F7160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F7164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F7168: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F716C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F7170: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F7174: 38EBF018  addi r7, r11, -0xfe8
	ctx.r[7].s64 = ctx.r[11].s64 + -4072;
	// 826F7178: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826F717C: 388A49DC  addi r4, r10, 0x49dc
	ctx.r[4].s64 = ctx.r[10].s64 + 18908;
	// 826F7180: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F7184: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7188: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F718C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F7190: 386AA4D4  addi r3, r10, -0x5b2c
	ctx.r[3].s64 = ctx.r[10].s64 + -23340;
	// 826F7194: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F7198: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F719C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F71A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F71A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F71A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F71AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F71B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F71B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F71B8: 4BD6FC69  bl 0x82466e20
	ctx.lr = 0x826F71BC;
	sub_82466E20(ctx, base);
	// 826F71BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F71C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F71C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F71C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F71D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F71D0 size=116
    let mut pc: u32 = 0x826F71D0;
    'dispatch: loop {
        match pc {
            0x826F71D0 => {
    //   block [0x826F71D0..0x826F7244)
	// 826F71D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F71D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F71D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F71DC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826F71E0: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F71E4: 392BA76C  addi r9, r11, -0x5894
	ctx.r[9].s64 = ctx.r[11].s64 + -22676;
	// 826F71E8: 38AAA954  addi r5, r10, -0x56ac
	ctx.r[5].s64 = ctx.r[10].s64 + -22188;
	// 826F71EC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F71F0: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826F71F4: 38C00012  li r6, 0x12
	ctx.r[6].s64 = 18;
	// 826F71F8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F71FC: 388A49FC  addi r4, r10, 0x49fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18940;
	// 826F7200: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F7204: 396BF078  addi r11, r11, -0xf88
	ctx.r[11].s64 = ctx.r[11].s64 + -3976;
	// 826F7208: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826F720C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7210: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826F7214: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F7218: 386AA504  addi r3, r10, -0x5afc
	ctx.r[3].s64 = ctx.r[10].s64 + -23292;
	// 826F721C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F7220: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826F7224: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F7228: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826F722C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F7230: 4BD6FBF1  bl 0x82466e20
	ctx.lr = 0x826F7234;
	sub_82466E20(ctx, base);
	// 826F7234: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F7238: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F723C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F7240: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F7248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F7248 size=100
    let mut pc: u32 = 0x826F7248;
    'dispatch: loop {
        match pc {
            0x826F7248 => {
    //   block [0x826F7248..0x826F72AC)
	// 826F7248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F724C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F7250: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F7254: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F7258: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F725C: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F7260: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F7264: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F7268: 388A4A08  addi r4, r10, 0x4a08
	ctx.r[4].s64 = ctx.r[10].s64 + 18952;
	// 826F726C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7270: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F7274: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F7278: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F727C: 386AA534  addi r3, r10, -0x5acc
	ctx.r[3].s64 = ctx.r[10].s64 + -23244;
	// 826F7280: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F7284: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F7288: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F728C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F7290: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F7294: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F7298: 4BD6FB89  bl 0x82466e20
	ctx.lr = 0x826F729C;
	sub_82466E20(ctx, base);
	// 826F729C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F72A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F72A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F72A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F72B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F72B0 size=100
    let mut pc: u32 = 0x826F72B0;
    'dispatch: loop {
        match pc {
            0x826F72B0 => {
    //   block [0x826F72B0..0x826F7314)
	// 826F72B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F72B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F72B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F72BC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F72C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F72C4: 38AAA5C4  addi r5, r10, -0x5a3c
	ctx.r[5].s64 = ctx.r[10].s64 + -23100;
	// 826F72C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F72CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F72D0: 388A4A1C  addi r4, r10, 0x4a1c
	ctx.r[4].s64 = ctx.r[10].s64 + 18972;
	// 826F72D4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F72D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F72DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F72E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F72E4: 386AA564  addi r3, r10, -0x5a9c
	ctx.r[3].s64 = ctx.r[10].s64 + -23196;
	// 826F72E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F72EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F72F0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F72F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F72F8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F72FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F7300: 4BD6FB21  bl 0x82466e20
	ctx.lr = 0x826F7304;
	sub_82466E20(ctx, base);
	// 826F7304: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F7308: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F730C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F7310: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F7318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F7318 size=100
    let mut pc: u32 = 0x826F7318;
    'dispatch: loop {
        match pc {
            0x826F7318 => {
    //   block [0x826F7318..0x826F737C)
	// 826F7318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F731C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F7320: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F7324: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7328: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F732C: 38AAA504  addi r5, r10, -0x5afc
	ctx.r[5].s64 = ctx.r[10].s64 + -23292;
	// 826F7330: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F7334: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F7338: 388A4A38  addi r4, r10, 0x4a38
	ctx.r[4].s64 = ctx.r[10].s64 + 19000;
	// 826F733C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7340: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F7344: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F7348: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F734C: 386AA594  addi r3, r10, -0x5a6c
	ctx.r[3].s64 = ctx.r[10].s64 + -23148;
	// 826F7350: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F7354: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F7358: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F735C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F7360: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F7364: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F7368: 4BD6FAB9  bl 0x82466e20
	ctx.lr = 0x826F736C;
	sub_82466E20(ctx, base);
	// 826F736C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F7370: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F7374: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F7378: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F7380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F7380 size=104
    let mut pc: u32 = 0x826F7380;
    'dispatch: loop {
        match pc {
            0x826F7380 => {
    //   block [0x826F7380..0x826F73E8)
	// 826F7380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F7384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F7388: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F738C: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F7390: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F7394: 392AA7EC  addi r9, r10, -0x5814
	ctx.r[9].s64 = ctx.r[10].s64 + -22548;
	// 826F7398: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F739C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F73A0: 38AAA534  addi r5, r10, -0x5acc
	ctx.r[5].s64 = ctx.r[10].s64 + -23244;
	// 826F73A4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F73A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F73AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F73B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F73B4: 388A4A44  addi r4, r10, 0x4a44
	ctx.r[4].s64 = ctx.r[10].s64 + 19012;
	// 826F73B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F73BC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F73C0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F73C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F73C8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F73CC: 386AA5C4  addi r3, r10, -0x5a3c
	ctx.r[3].s64 = ctx.r[10].s64 + -23100;
	// 826F73D0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F73D4: 4BD6FA4D  bl 0x82466e20
	ctx.lr = 0x826F73D8;
	sub_82466E20(ctx, base);
	// 826F73D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F73DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F73E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F73E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F73E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F73E8 size=108
    let mut pc: u32 = 0x826F73E8;
    'dispatch: loop {
        match pc {
            0x826F73E8 => {
    //   block [0x826F73E8..0x826F7454)
	// 826F73E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F73EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F73F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F73F4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F73F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F73FC: 38EBF22C  addi r7, r11, -0xdd4
	ctx.r[7].s64 = ctx.r[11].s64 + -3540;
	// 826F7400: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F7404: 388A4A5C  addi r4, r10, 0x4a5c
	ctx.r[4].s64 = ctx.r[10].s64 + 19036;
	// 826F7408: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F740C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7410: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F7414: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F7418: 386AA5F4  addi r3, r10, -0x5a0c
	ctx.r[3].s64 = ctx.r[10].s64 + -23052;
	// 826F741C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F7420: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F7424: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F7428: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F742C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F7430: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F7434: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F7438: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F743C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F7440: 4BD6F9E1  bl 0x82466e20
	ctx.lr = 0x826F7444;
	sub_82466E20(ctx, base);
	// 826F7444: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F7448: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F744C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F7450: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F7458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F7458 size=112
    let mut pc: u32 = 0x826F7458;
    'dispatch: loop {
        match pc {
            0x826F7458 => {
    //   block [0x826F7458..0x826F74C8)
	// 826F7458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F745C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F7460: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F7464: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7468: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F746C: 38AAA5C4  addi r5, r10, -0x5a3c
	ctx.r[5].s64 = ctx.r[10].s64 + -23100;
	// 826F7470: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F7474: 390BF260  addi r8, r11, -0xda0
	ctx.r[8].s64 = ctx.r[11].s64 + -3488;
	// 826F7478: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826F747C: 388A4A80  addi r4, r10, 0x4a80
	ctx.r[4].s64 = ctx.r[10].s64 + 19072;
	// 826F7480: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F7484: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7488: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F748C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F7490: 386AA624  addi r3, r10, -0x59dc
	ctx.r[3].s64 = ctx.r[10].s64 + -23004;
	// 826F7494: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F7498: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F749C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F74A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F74A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F74A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F74AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F74B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F74B4: 4BD6F96D  bl 0x82466e20
	ctx.lr = 0x826F74B8;
	sub_82466E20(ctx, base);
	// 826F74B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F74BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F74C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F74C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F74C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826F74C8 size=24
    let mut pc: u32 = 0x826F74C8;
    'dispatch: loop {
        match pc {
            0x826F74C8 => {
    //   block [0x826F74C8..0x826F74E0)
	// 826F74C8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F74CC: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F74D0: 394A25A0  addi r10, r10, 0x25a0
	ctx.r[10].s64 = ctx.r[10].s64 + 9632;
	// 826F74D4: 816BF25C  lwz r11, -0xda4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-3492 as u32) ) } as u64;
	// 826F74D8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826F74DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F74E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F74E0 size=116
    let mut pc: u32 = 0x826F74E0;
    'dispatch: loop {
        match pc {
            0x826F74E0 => {
    //   block [0x826F74E0..0x826F7554)
	// 826F74E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F74E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F74E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F74EC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F74F0: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F74F4: 390B25A0  addi r8, r11, 0x25a0
	ctx.r[8].s64 = ctx.r[11].s64 + 9632;
	// 826F74F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F74FC: 392AA850  addi r9, r10, -0x57b0
	ctx.r[9].s64 = ctx.r[10].s64 + -22448;
	// 826F7500: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F7504: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826F7508: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F750C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F7510: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F7514: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F7518: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F751C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F7520: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F7524: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 826F7528: 388A4B0C  addi r4, r10, 0x4b0c
	ctx.r[4].s64 = ctx.r[10].s64 + 19212;
	// 826F752C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F7530: 386BA654  addi r3, r11, -0x59ac
	ctx.r[3].s64 = ctx.r[11].s64 + -22956;
	// 826F7534: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F7538: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F753C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F7540: 4BD6F8E1  bl 0x82466e20
	ctx.lr = 0x826F7544;
	sub_82466E20(ctx, base);
	// 826F7544: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F7548: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F754C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F7550: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F7558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F7558 size=100
    let mut pc: u32 = 0x826F7558;
    'dispatch: loop {
        match pc {
            0x826F7558 => {
    //   block [0x826F7558..0x826F75BC)
	// 826F7558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F755C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F7560: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F7564: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7568: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F756C: 38AAA654  addi r5, r10, -0x59ac
	ctx.r[5].s64 = ctx.r[10].s64 + -22956;
	// 826F7570: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F7574: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F7578: 388A4B18  addi r4, r10, 0x4b18
	ctx.r[4].s64 = ctx.r[10].s64 + 19224;
	// 826F757C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7580: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F7584: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F7588: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F758C: 386AA684  addi r3, r10, -0x597c
	ctx.r[3].s64 = ctx.r[10].s64 + -22908;
	// 826F7590: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F7594: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F7598: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F759C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F75A0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F75A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F75A8: 4BD6F879  bl 0x82466e20
	ctx.lr = 0x826F75AC;
	sub_82466E20(ctx, base);
	// 826F75AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F75B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F75B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F75B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F75C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F75C0 size=100
    let mut pc: u32 = 0x826F75C0;
    'dispatch: loop {
        match pc {
            0x826F75C0 => {
    //   block [0x826F75C0..0x826F7624)
	// 826F75C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F75C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F75C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F75CC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F75D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F75D4: 38AAA6E4  addi r5, r10, -0x591c
	ctx.r[5].s64 = ctx.r[10].s64 + -22812;
	// 826F75D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F75DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F75E0: 388A4B24  addi r4, r10, 0x4b24
	ctx.r[4].s64 = ctx.r[10].s64 + 19236;
	// 826F75E4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F75E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F75EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F75F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F75F4: 386AA6B4  addi r3, r10, -0x594c
	ctx.r[3].s64 = ctx.r[10].s64 + -22860;
	// 826F75F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F75FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F7600: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F7604: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F7608: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F760C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F7610: 4BD6F811  bl 0x82466e20
	ctx.lr = 0x826F7614;
	sub_82466E20(ctx, base);
	// 826F7614: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F7618: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F761C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F7620: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F7628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F7628 size=112
    let mut pc: u32 = 0x826F7628;
    'dispatch: loop {
        match pc {
            0x826F7628 => {
    //   block [0x826F7628..0x826F7698)
	// 826F7628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F762C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F7630: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F7634: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7638: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F763C: 38AAA654  addi r5, r10, -0x59ac
	ctx.r[5].s64 = ctx.r[10].s64 + -22956;
	// 826F7640: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F7644: 390BF308  addi r8, r11, -0xcf8
	ctx.r[8].s64 = ctx.r[11].s64 + -3320;
	// 826F7648: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826F764C: 388A4B38  addi r4, r10, 0x4b38
	ctx.r[4].s64 = ctx.r[10].s64 + 19256;
	// 826F7650: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F7654: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7658: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F765C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F7660: 386AA6E4  addi r3, r10, -0x591c
	ctx.r[3].s64 = ctx.r[10].s64 + -22812;
	// 826F7664: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F7668: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F766C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F7670: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F7674: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F7678: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F767C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F7680: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F7684: 4BD6F79D  bl 0x82466e20
	ctx.lr = 0x826F7688;
	sub_82466E20(ctx, base);
	// 826F7688: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F768C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F7690: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F7694: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F7698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F7698 size=100
    let mut pc: u32 = 0x826F7698;
    'dispatch: loop {
        match pc {
            0x826F7698 => {
    //   block [0x826F7698..0x826F76FC)
	// 826F7698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F769C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F76A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F76A4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F76A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F76AC: 38AAA6E4  addi r5, r10, -0x591c
	ctx.r[5].s64 = ctx.r[10].s64 + -22812;
	// 826F76B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F76B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F76B8: 388A4B50  addi r4, r10, 0x4b50
	ctx.r[4].s64 = ctx.r[10].s64 + 19280;
	// 826F76BC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F76C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F76C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F76C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F76CC: 386AA714  addi r3, r10, -0x58ec
	ctx.r[3].s64 = ctx.r[10].s64 + -22764;
	// 826F76D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F76D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F76D8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F76DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F76E0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F76E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F76E8: 4BD6F739  bl 0x82466e20
	ctx.lr = 0x826F76EC;
	sub_82466E20(ctx, base);
	// 826F76EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F76F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F76F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F76F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F7700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F7700 size=100
    let mut pc: u32 = 0x826F7700;
    'dispatch: loop {
        match pc {
            0x826F7700 => {
    //   block [0x826F7700..0x826F7764)
	// 826F7700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F7704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F7708: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F770C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7710: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F7714: 38AAA654  addi r5, r10, -0x59ac
	ctx.r[5].s64 = ctx.r[10].s64 + -22956;
	// 826F7718: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F771C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F7720: 388A4B60  addi r4, r10, 0x4b60
	ctx.r[4].s64 = ctx.r[10].s64 + 19296;
	// 826F7724: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7728: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F772C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F7730: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F7734: 386AA744  addi r3, r10, -0x58bc
	ctx.r[3].s64 = ctx.r[10].s64 + -22716;
	// 826F7738: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F773C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F7740: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F7744: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F7748: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F774C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F7750: 4BD6F6D1  bl 0x82466e20
	ctx.lr = 0x826F7754;
	sub_82466E20(ctx, base);
	// 826F7754: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F7758: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F775C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F7760: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F7768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F7768 size=100
    let mut pc: u32 = 0x826F7768;
    'dispatch: loop {
        match pc {
            0x826F7768 => {
    //   block [0x826F7768..0x826F77CC)
	// 826F7768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F776C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F7770: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F7774: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7778: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F777C: 38AAA684  addi r5, r10, -0x597c
	ctx.r[5].s64 = ctx.r[10].s64 + -22908;
	// 826F7780: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F7784: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F7788: 388A4B70  addi r4, r10, 0x4b70
	ctx.r[4].s64 = ctx.r[10].s64 + 19312;
	// 826F778C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7790: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F7794: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F7798: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F779C: 386AA774  addi r3, r10, -0x588c
	ctx.r[3].s64 = ctx.r[10].s64 + -22668;
	// 826F77A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F77A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F77A8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F77AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F77B0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F77B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F77B8: 4BD6F669  bl 0x82466e20
	ctx.lr = 0x826F77BC;
	sub_82466E20(ctx, base);
	// 826F77BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F77C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F77C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F77C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F77D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F77D0 size=100
    let mut pc: u32 = 0x826F77D0;
    'dispatch: loop {
        match pc {
            0x826F77D0 => {
    //   block [0x826F77D0..0x826F7834)
	// 826F77D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F77D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F77D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F77DC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F77E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F77E4: 38AAA744  addi r5, r10, -0x58bc
	ctx.r[5].s64 = ctx.r[10].s64 + -22716;
	// 826F77E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F77EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F77F0: 388A4B88  addi r4, r10, 0x4b88
	ctx.r[4].s64 = ctx.r[10].s64 + 19336;
	// 826F77F4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F77F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F77FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F7800: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F7804: 386AA7A4  addi r3, r10, -0x585c
	ctx.r[3].s64 = ctx.r[10].s64 + -22620;
	// 826F7808: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F780C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F7810: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F7814: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F7818: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F781C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F7820: 4BD6F601  bl 0x82466e20
	ctx.lr = 0x826F7824;
	sub_82466E20(ctx, base);
	// 826F7824: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F7828: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F782C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F7830: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F7838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F7838 size=100
    let mut pc: u32 = 0x826F7838;
    'dispatch: loop {
        match pc {
            0x826F7838 => {
    //   block [0x826F7838..0x826F789C)
	// 826F7838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F783C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F7840: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F7844: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7848: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F784C: 38AAA684  addi r5, r10, -0x597c
	ctx.r[5].s64 = ctx.r[10].s64 + -22908;
	// 826F7850: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F7854: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F7858: 388A4BA4  addi r4, r10, 0x4ba4
	ctx.r[4].s64 = ctx.r[10].s64 + 19364;
	// 826F785C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7860: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F7864: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F7868: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F786C: 386AA7D4  addi r3, r10, -0x582c
	ctx.r[3].s64 = ctx.r[10].s64 + -22572;
	// 826F7870: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F7874: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F7878: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F787C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F7880: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F7884: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F7888: 4BD6F599  bl 0x82466e20
	ctx.lr = 0x826F788C;
	sub_82466E20(ctx, base);
	// 826F788C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F7890: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F7894: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F7898: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F78A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F78A0 size=112
    let mut pc: u32 = 0x826F78A0;
    'dispatch: loop {
        match pc {
            0x826F78A0 => {
    //   block [0x826F78A0..0x826F7910)
	// 826F78A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F78A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F78A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F78AC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F78B0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F78B4: 38AAA864  addi r5, r10, -0x579c
	ctx.r[5].s64 = ctx.r[10].s64 + -22428;
	// 826F78B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F78BC: 390BF338  addi r8, r11, -0xcc8
	ctx.r[8].s64 = ctx.r[11].s64 + -3272;
	// 826F78C0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826F78C4: 388A4BB4  addi r4, r10, 0x4bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 19380;
	// 826F78C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F78CC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F78D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F78D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F78D8: 386AA804  addi r3, r10, -0x57fc
	ctx.r[3].s64 = ctx.r[10].s64 + -22524;
	// 826F78DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F78E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F78E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F78E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F78EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F78F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F78F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F78F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F78FC: 4BD6F525  bl 0x82466e20
	ctx.lr = 0x826F7900;
	sub_82466E20(ctx, base);
	// 826F7900: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F7904: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F7908: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F790C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F7910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F7910 size=112
    let mut pc: u32 = 0x826F7910;
    'dispatch: loop {
        match pc {
            0x826F7910 => {
    //   block [0x826F7910..0x826F7980)
	// 826F7910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F7914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F7918: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F791C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7920: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F7924: 38AAA894  addi r5, r10, -0x576c
	ctx.r[5].s64 = ctx.r[10].s64 + -22380;
	// 826F7928: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F792C: 390BF368  addi r8, r11, -0xc98
	ctx.r[8].s64 = ctx.r[11].s64 + -3224;
	// 826F7930: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F7934: 388A4BC4  addi r4, r10, 0x4bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 19396;
	// 826F7938: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F793C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7940: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F7944: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F7948: 386AA834  addi r3, r10, -0x57cc
	ctx.r[3].s64 = ctx.r[10].s64 + -22476;
	// 826F794C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F7950: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F7954: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F7958: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F795C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F7960: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F7964: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F7968: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F796C: 4BD6F4B5  bl 0x82466e20
	ctx.lr = 0x826F7970;
	sub_82466E20(ctx, base);
	// 826F7970: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F7974: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F7978: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F797C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F7980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F7980 size=112
    let mut pc: u32 = 0x826F7980;
    'dispatch: loop {
        match pc {
            0x826F7980 => {
    //   block [0x826F7980..0x826F79F0)
	// 826F7980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F7984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F7988: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F798C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7990: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F7994: 38AAA954  addi r5, r10, -0x56ac
	ctx.r[5].s64 = ctx.r[10].s64 + -22188;
	// 826F7998: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F799C: 390BF380  addi r8, r11, -0xc80
	ctx.r[8].s64 = ctx.r[11].s64 + -3200;
	// 826F79A0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826F79A4: 388A4BDC  addi r4, r10, 0x4bdc
	ctx.r[4].s64 = ctx.r[10].s64 + 19420;
	// 826F79A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F79AC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F79B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F79B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F79B8: 386AA864  addi r3, r10, -0x579c
	ctx.r[3].s64 = ctx.r[10].s64 + -22428;
	// 826F79BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F79C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F79C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F79C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F79CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F79D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F79D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F79D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F79DC: 4BD6F445  bl 0x82466e20
	ctx.lr = 0x826F79E0;
	sub_82466E20(ctx, base);
	// 826F79E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F79E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F79E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F79EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F79F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F79F0 size=112
    let mut pc: u32 = 0x826F79F0;
    'dispatch: loop {
        match pc {
            0x826F79F0 => {
    //   block [0x826F79F0..0x826F7A60)
	// 826F79F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F79F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F79F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F79FC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7A00: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F7A04: 38AAA864  addi r5, r10, -0x579c
	ctx.r[5].s64 = ctx.r[10].s64 + -22428;
	// 826F7A08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F7A0C: 390BF3B0  addi r8, r11, -0xc50
	ctx.r[8].s64 = ctx.r[11].s64 + -3152;
	// 826F7A10: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F7A14: 388A4BE8  addi r4, r10, 0x4be8
	ctx.r[4].s64 = ctx.r[10].s64 + 19432;
	// 826F7A18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F7A1C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7A20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F7A24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F7A28: 386AA894  addi r3, r10, -0x576c
	ctx.r[3].s64 = ctx.r[10].s64 + -22380;
	// 826F7A2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F7A30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F7A34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F7A38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F7A3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F7A40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F7A44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F7A48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F7A4C: 4BD6F3D5  bl 0x82466e20
	ctx.lr = 0x826F7A50;
	sub_82466E20(ctx, base);
	// 826F7A50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F7A54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F7A58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F7A5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F7A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F7A60 size=112
    let mut pc: u32 = 0x826F7A60;
    'dispatch: loop {
        match pc {
            0x826F7A60 => {
    //   block [0x826F7A60..0x826F7AD0)
	// 826F7A60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F7A64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F7A68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F7A6C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7A70: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F7A74: 38AAA894  addi r5, r10, -0x576c
	ctx.r[5].s64 = ctx.r[10].s64 + -22380;
	// 826F7A78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F7A7C: 390BF3C8  addi r8, r11, -0xc38
	ctx.r[8].s64 = ctx.r[11].s64 + -3128;
	// 826F7A80: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F7A84: 388A4BF8  addi r4, r10, 0x4bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 19448;
	// 826F7A88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F7A8C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7A90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F7A94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F7A98: 386AA8C4  addi r3, r10, -0x573c
	ctx.r[3].s64 = ctx.r[10].s64 + -22332;
	// 826F7A9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F7AA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F7AA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F7AA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F7AAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F7AB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F7AB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F7AB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F7ABC: 4BD6F365  bl 0x82466e20
	ctx.lr = 0x826F7AC0;
	sub_82466E20(ctx, base);
	// 826F7AC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F7AC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F7AC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F7ACC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F7AD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F7AD0 size=116
    let mut pc: u32 = 0x826F7AD0;
    'dispatch: loop {
        match pc {
            0x826F7AD0 => {
    //   block [0x826F7AD0..0x826F7B44)
	// 826F7AD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F7AD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F7AD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F7ADC: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F7AE0: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826F7AE4: 390AF3E0  addi r8, r10, -0xc20
	ctx.r[8].s64 = ctx.r[10].s64 + -3104;
	// 826F7AE8: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F7AEC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826F7AF0: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F7AF4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F7AF8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F7AFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F7B00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F7B04: 388A4C10  addi r4, r10, 0x4c10
	ctx.r[4].s64 = ctx.r[10].s64 + 19472;
	// 826F7B08: 396BA864  addi r11, r11, -0x579c
	ctx.r[11].s64 = ctx.r[11].s64 + -22428;
	// 826F7B0C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7B10: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F7B14: 386AA8F4  addi r3, r10, -0x570c
	ctx.r[3].s64 = ctx.r[10].s64 + -22284;
	// 826F7B18: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826F7B1C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F7B20: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826F7B24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F7B28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F7B2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F7B30: 4BD6F2F1  bl 0x82466e20
	ctx.lr = 0x826F7B34;
	sub_82466E20(ctx, base);
	// 826F7B34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F7B38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F7B3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F7B40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F7B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826F7B48 size=48
    let mut pc: u32 = 0x826F7B48;
    'dispatch: loop {
        match pc {
            0x826F7B48 => {
    //   block [0x826F7B48..0x826F7B78)
	// 826F7B48: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F7B4C: 814BF494  lwz r10, -0xb6c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-2924 as u32) ) } as u64;
	// 826F7B50: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F7B54: 396B2660  addi r11, r11, 0x2660
	ctx.r[11].s64 = ctx.r[11].s64 + 9824;
	// 826F7B58: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 826F7B5C: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F7B60: 814AF490  lwz r10, -0xb70(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-2928 as u32) ) } as u64;
	// 826F7B64: 914B0140  stw r10, 0x140(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(320 as u32), ctx.r[10].u32 ) };
	// 826F7B68: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F7B6C: 814AF48C  lwz r10, -0xb74(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-2932 as u32) ) } as u64;
	// 826F7B70: 914B0350  stw r10, 0x350(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(848 as u32), ctx.r[10].u32 ) };
	// 826F7B74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F7B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F7B78 size=116
    let mut pc: u32 = 0x826F7B78;
    'dispatch: loop {
        match pc {
            0x826F7B78 => {
    //   block [0x826F7B78..0x826F7BEC)
	// 826F7B78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F7B7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F7B80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F7B84: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826F7B88: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F7B8C: 392BA938  addi r9, r11, -0x56c8
	ctx.r[9].s64 = ctx.r[11].s64 + -22216;
	// 826F7B90: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F7B94: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F7B98: 38E90050  addi r7, r9, 0x50
	ctx.r[7].s64 = ctx.r[9].s64 + 80;
	// 826F7B9C: 38C00028  li r6, 0x28
	ctx.r[6].s64 = 40;
	// 826F7BA0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F7BA4: 388A4FC8  addi r4, r10, 0x4fc8
	ctx.r[4].s64 = ctx.r[10].s64 + 20424;
	// 826F7BA8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F7BAC: 396B2660  addi r11, r11, 0x2660
	ctx.r[11].s64 = ctx.r[11].s64 + 9824;
	// 826F7BB0: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826F7BB4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7BB8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826F7BBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F7BC0: 386AA924  addi r3, r10, -0x56dc
	ctx.r[3].s64 = ctx.r[10].s64 + -22236;
	// 826F7BC4: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 826F7BC8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826F7BCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F7BD0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826F7BD4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F7BD8: 4BD6F249  bl 0x82466e20
	ctx.lr = 0x826F7BDC;
	sub_82466E20(ctx, base);
	// 826F7BDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F7BE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F7BE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F7BE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F7BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F7BF0 size=116
    let mut pc: u32 = 0x826F7BF0;
    'dispatch: loop {
        match pc {
            0x826F7BF0 => {
    //   block [0x826F7BF0..0x826F7C64)
	// 826F7BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F7BF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F7BF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F7BFC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F7C00: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F7C04: 390BF4A0  addi r8, r11, -0xb60
	ctx.r[8].s64 = ctx.r[11].s64 + -2912;
	// 826F7C08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F7C0C: 392AAAD8  addi r9, r10, -0x5528
	ctx.r[9].s64 = ctx.r[10].s64 + -21800;
	// 826F7C10: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F7C14: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826F7C18: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F7C1C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F7C20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F7C24: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F7C28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F7C2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F7C30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F7C34: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 826F7C38: 388A4FD8  addi r4, r10, 0x4fd8
	ctx.r[4].s64 = ctx.r[10].s64 + 20440;
	// 826F7C3C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F7C40: 386BA954  addi r3, r11, -0x56ac
	ctx.r[3].s64 = ctx.r[11].s64 + -22188;
	// 826F7C44: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F7C48: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F7C4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F7C50: 4BD6F1D1  bl 0x82466e20
	ctx.lr = 0x826F7C54;
	sub_82466E20(ctx, base);
	// 826F7C54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F7C58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F7C5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F7C60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F7C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F7C68 size=112
    let mut pc: u32 = 0x826F7C68;
    'dispatch: loop {
        match pc {
            0x826F7C68 => {
    //   block [0x826F7C68..0x826F7CD8)
	// 826F7C68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F7C6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F7C70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F7C74: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F7C78: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F7C7C: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F7C80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F7C84: 390BF530  addi r8, r11, -0xad0
	ctx.r[8].s64 = ctx.r[11].s64 + -2768;
	// 826F7C88: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F7C8C: 388A4FE8  addi r4, r10, 0x4fe8
	ctx.r[4].s64 = ctx.r[10].s64 + 20456;
	// 826F7C90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F7C94: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7C98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F7C9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F7CA0: 386AA984  addi r3, r10, -0x567c
	ctx.r[3].s64 = ctx.r[10].s64 + -22140;
	// 826F7CA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F7CA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F7CAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F7CB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F7CB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F7CB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F7CBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F7CC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F7CC4: 4BD6F15D  bl 0x82466e20
	ctx.lr = 0x826F7CC8;
	sub_82466E20(ctx, base);
	// 826F7CC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F7CCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F7CD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F7CD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F7CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F7CD8 size=108
    let mut pc: u32 = 0x826F7CD8;
    'dispatch: loop {
        match pc {
            0x826F7CD8 => {
    //   block [0x826F7CD8..0x826F7D44)
	// 826F7CD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F7CDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F7CE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F7CE4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F7CE8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826F7CEC: 38EBF548  addi r7, r11, -0xab8
	ctx.r[7].s64 = ctx.r[11].s64 + -2744;
	// 826F7CF0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826F7CF4: 388A0E48  addi r4, r10, 0xe48
	ctx.r[4].s64 = ctx.r[10].s64 + 3656;
	// 826F7CF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F7CFC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7D00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F7D04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F7D08: 386AA9B4  addi r3, r10, -0x564c
	ctx.r[3].s64 = ctx.r[10].s64 + -22092;
	// 826F7D0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F7D10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F7D14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F7D18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F7D1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F7D20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F7D24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F7D28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F7D2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F7D30: 4BD6F0F1  bl 0x82466e20
	ctx.lr = 0x826F7D34;
	sub_82466E20(ctx, base);
	// 826F7D34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F7D38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F7D3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F7D40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F7D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F7D48 size=112
    let mut pc: u32 = 0x826F7D48;
    'dispatch: loop {
        match pc {
            0x826F7D48 => {
    //   block [0x826F7D48..0x826F7DB8)
	// 826F7D48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F7D4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F7D50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F7D54: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7D58: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F7D5C: 38AA8914  addi r5, r10, -0x76ec
	ctx.r[5].s64 = ctx.r[10].s64 + -30444;
	// 826F7D60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F7D64: 390BF5C0  addi r8, r11, -0xa40
	ctx.r[8].s64 = ctx.r[11].s64 + -2624;
	// 826F7D68: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F7D6C: 388A5000  addi r4, r10, 0x5000
	ctx.r[4].s64 = ctx.r[10].s64 + 20480;
	// 826F7D70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F7D74: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7D78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F7D7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F7D80: 386AA9E4  addi r3, r10, -0x561c
	ctx.r[3].s64 = ctx.r[10].s64 + -22044;
	// 826F7D84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F7D88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F7D8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F7D90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F7D94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F7D98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F7D9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F7DA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F7DA4: 4BD6F07D  bl 0x82466e20
	ctx.lr = 0x826F7DA8;
	sub_82466E20(ctx, base);
	// 826F7DA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F7DAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F7DB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F7DB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F7DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F7DB8 size=108
    let mut pc: u32 = 0x826F7DB8;
    'dispatch: loop {
        match pc {
            0x826F7DB8 => {
    //   block [0x826F7DB8..0x826F7E24)
	// 826F7DB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F7DBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F7DC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F7DC4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F7DC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F7DCC: 38EBF5D8  addi r7, r11, -0xa28
	ctx.r[7].s64 = ctx.r[11].s64 + -2600;
	// 826F7DD0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F7DD4: 388A5014  addi r4, r10, 0x5014
	ctx.r[4].s64 = ctx.r[10].s64 + 20500;
	// 826F7DD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F7DDC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7DE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F7DE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F7DE8: 386AAA14  addi r3, r10, -0x55ec
	ctx.r[3].s64 = ctx.r[10].s64 + -21996;
	// 826F7DEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F7DF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F7DF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F7DF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F7DFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F7E00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F7E04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F7E08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F7E0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F7E10: 4BD6F011  bl 0x82466e20
	ctx.lr = 0x826F7E14;
	sub_82466E20(ctx, base);
	// 826F7E14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F7E18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F7E1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F7E20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F7E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F7E28 size=112
    let mut pc: u32 = 0x826F7E28;
    'dispatch: loop {
        match pc {
            0x826F7E28 => {
    //   block [0x826F7E28..0x826F7E98)
	// 826F7E28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F7E2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F7E30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F7E34: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F7E38: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F7E3C: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F7E40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F7E44: 390BF5F0  addi r8, r11, -0xa10
	ctx.r[8].s64 = ctx.r[11].s64 + -2576;
	// 826F7E48: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826F7E4C: 388A5028  addi r4, r10, 0x5028
	ctx.r[4].s64 = ctx.r[10].s64 + 20520;
	// 826F7E50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F7E54: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7E58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F7E5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F7E60: 386AAA44  addi r3, r10, -0x55bc
	ctx.r[3].s64 = ctx.r[10].s64 + -21948;
	// 826F7E64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F7E68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F7E6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F7E70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F7E74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F7E78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F7E7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F7E80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F7E84: 4BD6EF9D  bl 0x82466e20
	ctx.lr = 0x826F7E88;
	sub_82466E20(ctx, base);
	// 826F7E88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F7E8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F7E90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F7E94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F7E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F7E98 size=108
    let mut pc: u32 = 0x826F7E98;
    'dispatch: loop {
        match pc {
            0x826F7E98 => {
    //   block [0x826F7E98..0x826F7F04)
	// 826F7E98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F7E9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F7EA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F7EA4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F7EA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F7EAC: 38EBF638  addi r7, r11, -0x9c8
	ctx.r[7].s64 = ctx.r[11].s64 + -2504;
	// 826F7EB0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F7EB4: 388A5040  addi r4, r10, 0x5040
	ctx.r[4].s64 = ctx.r[10].s64 + 20544;
	// 826F7EB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F7EBC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7EC0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F7EC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F7EC8: 386AAA74  addi r3, r10, -0x558c
	ctx.r[3].s64 = ctx.r[10].s64 + -21900;
	// 826F7ECC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F7ED0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F7ED4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F7ED8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F7EDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F7EE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F7EE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F7EE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F7EEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F7EF0: 4BD6EF31  bl 0x82466e20
	ctx.lr = 0x826F7EF4;
	sub_82466E20(ctx, base);
	// 826F7EF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F7EF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F7EFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F7F00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F7F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F7F08 size=108
    let mut pc: u32 = 0x826F7F08;
    'dispatch: loop {
        match pc {
            0x826F7F08 => {
    //   block [0x826F7F08..0x826F7F74)
	// 826F7F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F7F0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F7F10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F7F14: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F7F18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F7F1C: 38EBF668  addi r7, r11, -0x998
	ctx.r[7].s64 = ctx.r[11].s64 + -2456;
	// 826F7F20: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F7F24: 388A505C  addi r4, r10, 0x505c
	ctx.r[4].s64 = ctx.r[10].s64 + 20572;
	// 826F7F28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F7F2C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7F30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F7F34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F7F38: 386AAAA4  addi r3, r10, -0x555c
	ctx.r[3].s64 = ctx.r[10].s64 + -21852;
	// 826F7F3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F7F40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F7F44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F7F48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F7F4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F7F50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F7F54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F7F58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F7F5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F7F60: 4BD6EEC1  bl 0x82466e20
	ctx.lr = 0x826F7F64;
	sub_82466E20(ctx, base);
	// 826F7F64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F7F68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F7F6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F7F70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F7F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F7F78 size=112
    let mut pc: u32 = 0x826F7F78;
    'dispatch: loop {
        match pc {
            0x826F7F78 => {
    //   block [0x826F7F78..0x826F7FE8)
	// 826F7F78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F7F7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F7F80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F7F84: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F7F88: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F7F8C: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F7F90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F7F94: 390BF680  addi r8, r11, -0x980
	ctx.r[8].s64 = ctx.r[11].s64 + -2432;
	// 826F7F98: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826F7F9C: 388A5070  addi r4, r10, 0x5070
	ctx.r[4].s64 = ctx.r[10].s64 + 20592;
	// 826F7FA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F7FA4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7FA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F7FAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F7FB0: 386AAAD4  addi r3, r10, -0x552c
	ctx.r[3].s64 = ctx.r[10].s64 + -21804;
	// 826F7FB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F7FB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F7FBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F7FC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F7FC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F7FC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F7FCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F7FD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F7FD4: 4BD6EE4D  bl 0x82466e20
	ctx.lr = 0x826F7FD8;
	sub_82466E20(ctx, base);
	// 826F7FD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F7FDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F7FE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F7FE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F7FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F7FE8 size=112
    let mut pc: u32 = 0x826F7FE8;
    'dispatch: loop {
        match pc {
            0x826F7FE8 => {
    //   block [0x826F7FE8..0x826F8058)
	// 826F7FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F7FEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F7FF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F7FF4: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F7FF8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F7FFC: 392AAB30  addi r9, r10, -0x54d0
	ctx.r[9].s64 = ctx.r[10].s64 + -21712;
	// 826F8000: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F8004: 390BF6B8  addi r8, r11, -0x948
	ctx.r[8].s64 = ctx.r[11].s64 + -2376;
	// 826F8008: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826F800C: 388A508C  addi r4, r10, 0x508c
	ctx.r[4].s64 = ctx.r[10].s64 + 20620;
	// 826F8010: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F8014: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8018: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F801C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F8020: 386AAB04  addi r3, r10, -0x54fc
	ctx.r[3].s64 = ctx.r[10].s64 + -21756;
	// 826F8024: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F8028: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F802C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F8030: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F8034: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F8038: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F803C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F8040: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F8044: 4BD6EDDD  bl 0x82466e20
	ctx.lr = 0x826F8048;
	sub_82466E20(ctx, base);
	// 826F8048: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F804C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F8050: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F8054: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F8058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F8058 size=116
    let mut pc: u32 = 0x826F8058;
    'dispatch: loop {
        match pc {
            0x826F8058 => {
    //   block [0x826F8058..0x826F80CC)
	// 826F8058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F805C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F8060: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F8064: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F8068: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F806C: 390BF760  addi r8, r11, -0x8a0
	ctx.r[8].s64 = ctx.r[11].s64 + -2208;
	// 826F8070: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F8074: 392AAB04  addi r9, r10, -0x54fc
	ctx.r[9].s64 = ctx.r[10].s64 + -21756;
	// 826F8078: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F807C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826F8080: 38AA9C94  addi r5, r10, -0x636c
	ctx.r[5].s64 = ctx.r[10].s64 + -25452;
	// 826F8084: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F8088: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F808C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F8090: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F8094: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F8098: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F809C: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 826F80A0: 388A50AC  addi r4, r10, 0x50ac
	ctx.r[4].s64 = ctx.r[10].s64 + 20652;
	// 826F80A4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F80A8: 386BAB34  addi r3, r11, -0x54cc
	ctx.r[3].s64 = ctx.r[11].s64 + -21708;
	// 826F80AC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F80B0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F80B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F80B8: 4BD6ED69  bl 0x82466e20
	ctx.lr = 0x826F80BC;
	sub_82466E20(ctx, base);
	// 826F80BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F80C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F80C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F80C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F80D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F80D0 size=112
    let mut pc: u32 = 0x826F80D0;
    'dispatch: loop {
        match pc {
            0x826F80D0 => {
    //   block [0x826F80D0..0x826F8140)
	// 826F80D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F80D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F80D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F80DC: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F80E0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F80E4: 392AAB5C  addi r9, r10, -0x54a4
	ctx.r[9].s64 = ctx.r[10].s64 + -21668;
	// 826F80E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F80EC: 390BF780  addi r8, r11, -0x880
	ctx.r[8].s64 = ctx.r[11].s64 + -2176;
	// 826F80F0: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 826F80F4: 388A50C4  addi r4, r10, 0x50c4
	ctx.r[4].s64 = ctx.r[10].s64 + 20676;
	// 826F80F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F80FC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8100: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F8104: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F8108: 386AAB64  addi r3, r10, -0x549c
	ctx.r[3].s64 = ctx.r[10].s64 + -21660;
	// 826F810C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F8110: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F8114: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F8118: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F811C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F8120: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F8124: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F8128: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F812C: 4BD6ECF5  bl 0x82466e20
	ctx.lr = 0x826F8130;
	sub_82466E20(ctx, base);
	// 826F8130: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F8134: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F8138: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F813C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F8140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F8140 size=112
    let mut pc: u32 = 0x826F8140;
    'dispatch: loop {
        match pc {
            0x826F8140 => {
    //   block [0x826F8140..0x826F81B0)
	// 826F8140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F8144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F8148: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F814C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8150: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F8154: 38AA9C94  addi r5, r10, -0x636c
	ctx.r[5].s64 = ctx.r[10].s64 + -25452;
	// 826F8158: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F815C: 390BF7E0  addi r8, r11, -0x820
	ctx.r[8].s64 = ctx.r[11].s64 + -2080;
	// 826F8160: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F8164: 388A50E0  addi r4, r10, 0x50e0
	ctx.r[4].s64 = ctx.r[10].s64 + 20704;
	// 826F8168: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F816C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8170: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F8174: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F8178: 386AAB94  addi r3, r10, -0x546c
	ctx.r[3].s64 = ctx.r[10].s64 + -21612;
	// 826F817C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F8180: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F8184: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F8188: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F818C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F8190: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F8194: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F8198: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F819C: 4BD6EC85  bl 0x82466e20
	ctx.lr = 0x826F81A0;
	sub_82466E20(ctx, base);
	// 826F81A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F81A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F81A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F81AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F81B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F81B0 size=112
    let mut pc: u32 = 0x826F81B0;
    'dispatch: loop {
        match pc {
            0x826F81B0 => {
    //   block [0x826F81B0..0x826F8220)
	// 826F81B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F81B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F81B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F81BC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F81C0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F81C4: 38AA9BA4  addi r5, r10, -0x645c
	ctx.r[5].s64 = ctx.r[10].s64 + -25692;
	// 826F81C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F81CC: 390BF7F8  addi r8, r11, -0x808
	ctx.r[8].s64 = ctx.r[11].s64 + -2056;
	// 826F81D0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826F81D4: 388A50F4  addi r4, r10, 0x50f4
	ctx.r[4].s64 = ctx.r[10].s64 + 20724;
	// 826F81D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F81DC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F81E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F81E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F81E8: 386AABC4  addi r3, r10, -0x543c
	ctx.r[3].s64 = ctx.r[10].s64 + -21564;
	// 826F81EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F81F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F81F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F81F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F81FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F8200: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F8204: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F8208: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F820C: 4BD6EC15  bl 0x82466e20
	ctx.lr = 0x826F8210;
	sub_82466E20(ctx, base);
	// 826F8210: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F8214: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F8218: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F821C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F8220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F8220 size=112
    let mut pc: u32 = 0x826F8220;
    'dispatch: loop {
        match pc {
            0x826F8220 => {
    //   block [0x826F8220..0x826F8290)
	// 826F8220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F8224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F8228: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F822C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8230: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F8234: 38AA9BA4  addi r5, r10, -0x645c
	ctx.r[5].s64 = ctx.r[10].s64 + -25692;
	// 826F8238: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F823C: 390BF840  addi r8, r11, -0x7c0
	ctx.r[8].s64 = ctx.r[11].s64 + -1984;
	// 826F8240: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826F8244: 388A510C  addi r4, r10, 0x510c
	ctx.r[4].s64 = ctx.r[10].s64 + 20748;
	// 826F8248: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F824C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8250: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F8254: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F8258: 386AABF4  addi r3, r10, -0x540c
	ctx.r[3].s64 = ctx.r[10].s64 + -21516;
	// 826F825C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F8260: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F8264: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F8268: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F826C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F8270: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F8274: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F8278: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F827C: 4BD6EBA5  bl 0x82466e20
	ctx.lr = 0x826F8280;
	sub_82466E20(ctx, base);
	// 826F8280: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F8284: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F8288: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F828C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F8290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F8290 size=112
    let mut pc: u32 = 0x826F8290;
    'dispatch: loop {
        match pc {
            0x826F8290 => {
    //   block [0x826F8290..0x826F8300)
	// 826F8290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F8294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F8298: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F829C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F82A0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F82A4: 38AA9BD4  addi r5, r10, -0x642c
	ctx.r[5].s64 = ctx.r[10].s64 + -25644;
	// 826F82A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F82AC: 390BF8A0  addi r8, r11, -0x760
	ctx.r[8].s64 = ctx.r[11].s64 + -1888;
	// 826F82B0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826F82B4: 388A511C  addi r4, r10, 0x511c
	ctx.r[4].s64 = ctx.r[10].s64 + 20764;
	// 826F82B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F82BC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F82C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F82C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F82C8: 386AAC24  addi r3, r10, -0x53dc
	ctx.r[3].s64 = ctx.r[10].s64 + -21468;
	// 826F82CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F82D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F82D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F82D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F82DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F82E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F82E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F82E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F82EC: 4BD6EB35  bl 0x82466e20
	ctx.lr = 0x826F82F0;
	sub_82466E20(ctx, base);
	// 826F82F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F82F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F82F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F82FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F8300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F8300 size=112
    let mut pc: u32 = 0x826F8300;
    'dispatch: loop {
        match pc {
            0x826F8300 => {
    //   block [0x826F8300..0x826F8370)
	// 826F8300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F8304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F8308: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F830C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8310: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F8314: 38AA9BD4  addi r5, r10, -0x642c
	ctx.r[5].s64 = ctx.r[10].s64 + -25644;
	// 826F8318: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F831C: 390BF900  addi r8, r11, -0x700
	ctx.r[8].s64 = ctx.r[11].s64 + -1792;
	// 826F8320: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826F8324: 388A512C  addi r4, r10, 0x512c
	ctx.r[4].s64 = ctx.r[10].s64 + 20780;
	// 826F8328: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F832C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8330: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F8334: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F8338: 386AAC54  addi r3, r10, -0x53ac
	ctx.r[3].s64 = ctx.r[10].s64 + -21420;
	// 826F833C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F8340: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F8344: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F8348: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F834C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F8350: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F8354: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F8358: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F835C: 4BD6EAC5  bl 0x82466e20
	ctx.lr = 0x826F8360;
	sub_82466E20(ctx, base);
	// 826F8360: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F8364: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F8368: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F836C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F8370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F8370 size=112
    let mut pc: u32 = 0x826F8370;
    'dispatch: loop {
        match pc {
            0x826F8370 => {
    //   block [0x826F8370..0x826F83E0)
	// 826F8370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F8374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F8378: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F837C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8380: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F8384: 38AA9BA4  addi r5, r10, -0x645c
	ctx.r[5].s64 = ctx.r[10].s64 + -25692;
	// 826F8388: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F838C: 390BF960  addi r8, r11, -0x6a0
	ctx.r[8].s64 = ctx.r[11].s64 + -1696;
	// 826F8390: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 826F8394: 388A5140  addi r4, r10, 0x5140
	ctx.r[4].s64 = ctx.r[10].s64 + 20800;
	// 826F8398: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F839C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F83A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F83A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F83A8: 386AAC84  addi r3, r10, -0x537c
	ctx.r[3].s64 = ctx.r[10].s64 + -21372;
	// 826F83AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F83B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F83B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F83B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F83BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F83C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F83C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F83C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F83CC: 4BD6EA55  bl 0x82466e20
	ctx.lr = 0x826F83D0;
	sub_82466E20(ctx, base);
	// 826F83D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F83D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F83D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F83DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F83E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F83E0 size=112
    let mut pc: u32 = 0x826F83E0;
    'dispatch: loop {
        match pc {
            0x826F83E0 => {
    //   block [0x826F83E0..0x826F8450)
	// 826F83E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F83E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F83E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F83EC: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F83F0: 39000013  li r8, 0x13
	ctx.r[8].s64 = 19;
	// 826F83F4: 38EAFA20  addi r7, r10, -0x5e0
	ctx.r[7].s64 = ctx.r[10].s64 + -1504;
	// 826F83F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F83FC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826F8400: 388A5150  addi r4, r10, 0x5150
	ctx.r[4].s64 = ctx.r[10].s64 + 20816;
	// 826F8404: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F8408: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F840C: 396BAB70  addi r11, r11, -0x5490
	ctx.r[11].s64 = ctx.r[11].s64 + -21648;
	// 826F8410: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F8414: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8418: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F841C: 386AACB4  addi r3, r10, -0x534c
	ctx.r[3].s64 = ctx.r[10].s64 + -21324;
	// 826F8420: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F8424: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826F8428: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F842C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826F8430: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F8434: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F8438: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F843C: 4BD6E9E5  bl 0x82466e20
	ctx.lr = 0x826F8440;
	sub_82466E20(ctx, base);
	// 826F8440: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F8444: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F8448: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F844C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F8450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F8450 size=112
    let mut pc: u32 = 0x826F8450;
    'dispatch: loop {
        match pc {
            0x826F8450 => {
    //   block [0x826F8450..0x826F84C0)
	// 826F8450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F8454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F8458: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F845C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8460: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F8464: 38AA89D4  addi r5, r10, -0x762c
	ctx.r[5].s64 = ctx.r[10].s64 + -30252;
	// 826F8468: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F846C: 390BFBE8  addi r8, r11, -0x418
	ctx.r[8].s64 = ctx.r[11].s64 + -1048;
	// 826F8470: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F8474: 388A5168  addi r4, r10, 0x5168
	ctx.r[4].s64 = ctx.r[10].s64 + 20840;
	// 826F8478: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F847C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8480: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F8484: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F8488: 386AACE4  addi r3, r10, -0x531c
	ctx.r[3].s64 = ctx.r[10].s64 + -21276;
	// 826F848C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F8490: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F8494: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F8498: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F849C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F84A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F84A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F84A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F84AC: 4BD6E975  bl 0x82466e20
	ctx.lr = 0x826F84B0;
	sub_82466E20(ctx, base);
	// 826F84B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F84B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F84B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F84BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F84C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F84C0 size=112
    let mut pc: u32 = 0x826F84C0;
    'dispatch: loop {
        match pc {
            0x826F84C0 => {
    //   block [0x826F84C0..0x826F8530)
	// 826F84C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F84C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F84C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F84CC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F84D0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F84D4: 38AA89D4  addi r5, r10, -0x762c
	ctx.r[5].s64 = ctx.r[10].s64 + -30252;
	// 826F84D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F84DC: 390BFC00  addi r8, r11, -0x400
	ctx.r[8].s64 = ctx.r[11].s64 + -1024;
	// 826F84E0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F84E4: 388A5184  addi r4, r10, 0x5184
	ctx.r[4].s64 = ctx.r[10].s64 + 20868;
	// 826F84E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F84EC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F84F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F84F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F84F8: 386AAD14  addi r3, r10, -0x52ec
	ctx.r[3].s64 = ctx.r[10].s64 + -21228;
	// 826F84FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F8500: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F8504: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F8508: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F850C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F8510: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F8514: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F8518: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F851C: 4BD6E905  bl 0x82466e20
	ctx.lr = 0x826F8520;
	sub_82466E20(ctx, base);
	// 826F8520: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F8524: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F8528: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F852C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F8530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F8530 size=112
    let mut pc: u32 = 0x826F8530;
    'dispatch: loop {
        match pc {
            0x826F8530 => {
    //   block [0x826F8530..0x826F85A0)
	// 826F8530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F8534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F8538: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F853C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8540: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F8544: 38AA89D4  addi r5, r10, -0x762c
	ctx.r[5].s64 = ctx.r[10].s64 + -30252;
	// 826F8548: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F854C: 390BFC18  addi r8, r11, -0x3e8
	ctx.r[8].s64 = ctx.r[11].s64 + -1000;
	// 826F8550: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826F8554: 388A51A4  addi r4, r10, 0x51a4
	ctx.r[4].s64 = ctx.r[10].s64 + 20900;
	// 826F8558: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F855C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8560: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F8564: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F8568: 386AAD44  addi r3, r10, -0x52bc
	ctx.r[3].s64 = ctx.r[10].s64 + -21180;
	// 826F856C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F8570: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F8574: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F8578: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F857C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F8580: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F8584: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F8588: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F858C: 4BD6E895  bl 0x82466e20
	ctx.lr = 0x826F8590;
	sub_82466E20(ctx, base);
	// 826F8590: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F8594: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F8598: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F859C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F85A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F85A0 size=108
    let mut pc: u32 = 0x826F85A0;
    'dispatch: loop {
        match pc {
            0x826F85A0 => {
    //   block [0x826F85A0..0x826F860C)
	// 826F85A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F85A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F85A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F85AC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F85B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F85B4: 38EBFC48  addi r7, r11, -0x3b8
	ctx.r[7].s64 = ctx.r[11].s64 + -952;
	// 826F85B8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F85BC: 388A51BC  addi r4, r10, 0x51bc
	ctx.r[4].s64 = ctx.r[10].s64 + 20924;
	// 826F85C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F85C4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F85C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F85CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F85D0: 386AAD74  addi r3, r10, -0x528c
	ctx.r[3].s64 = ctx.r[10].s64 + -21132;
	// 826F85D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F85D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F85DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F85E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F85E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F85E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F85EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F85F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F85F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F85F8: 4BD6E829  bl 0x82466e20
	ctx.lr = 0x826F85FC;
	sub_82466E20(ctx, base);
	// 826F85FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F8600: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F8604: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F8608: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F8610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F8610 size=112
    let mut pc: u32 = 0x826F8610;
    'dispatch: loop {
        match pc {
            0x826F8610 => {
    //   block [0x826F8610..0x826F8680)
	// 826F8610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F8614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F8618: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F861C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8620: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F8624: 38AA89D4  addi r5, r10, -0x762c
	ctx.r[5].s64 = ctx.r[10].s64 + -30252;
	// 826F8628: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F862C: 390BFC78  addi r8, r11, -0x388
	ctx.r[8].s64 = ctx.r[11].s64 + -904;
	// 826F8630: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F8634: 388A51E4  addi r4, r10, 0x51e4
	ctx.r[4].s64 = ctx.r[10].s64 + 20964;
	// 826F8638: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F863C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8640: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F8644: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F8648: 386AADA4  addi r3, r10, -0x525c
	ctx.r[3].s64 = ctx.r[10].s64 + -21084;
	// 826F864C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F8650: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F8654: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F8658: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F865C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F8660: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F8664: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F8668: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F866C: 4BD6E7B5  bl 0x82466e20
	ctx.lr = 0x826F8670;
	sub_82466E20(ctx, base);
	// 826F8670: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F8674: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F8678: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F867C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F8680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F8680 size=108
    let mut pc: u32 = 0x826F8680;
    'dispatch: loop {
        match pc {
            0x826F8680 => {
    //   block [0x826F8680..0x826F86EC)
	// 826F8680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F8684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F8688: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F868C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F8690: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F8694: 38EBFC90  addi r7, r11, -0x370
	ctx.r[7].s64 = ctx.r[11].s64 + -880;
	// 826F8698: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F869C: 388A5200  addi r4, r10, 0x5200
	ctx.r[4].s64 = ctx.r[10].s64 + 20992;
	// 826F86A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F86A4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F86A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F86AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F86B0: 386AADD4  addi r3, r10, -0x522c
	ctx.r[3].s64 = ctx.r[10].s64 + -21036;
	// 826F86B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F86B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F86BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F86C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F86C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F86C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F86CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F86D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F86D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F86D8: 4BD6E749  bl 0x82466e20
	ctx.lr = 0x826F86DC;
	sub_82466E20(ctx, base);
	// 826F86DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F86E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F86E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F86E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F86F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F86F0 size=108
    let mut pc: u32 = 0x826F86F0;
    'dispatch: loop {
        match pc {
            0x826F86F0 => {
    //   block [0x826F86F0..0x826F875C)
	// 826F86F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F86F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F86F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F86FC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F8700: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F8704: 38EBFCC0  addi r7, r11, -0x340
	ctx.r[7].s64 = ctx.r[11].s64 + -832;
	// 826F8708: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826F870C: 388A521C  addi r4, r10, 0x521c
	ctx.r[4].s64 = ctx.r[10].s64 + 21020;
	// 826F8710: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F8714: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8718: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F871C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F8720: 386AAE04  addi r3, r10, -0x51fc
	ctx.r[3].s64 = ctx.r[10].s64 + -20988;
	// 826F8724: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F8728: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F872C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F8730: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F8734: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F8738: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F873C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F8740: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F8744: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F8748: 4BD6E6D9  bl 0x82466e20
	ctx.lr = 0x826F874C;
	sub_82466E20(ctx, base);
	// 826F874C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F8750: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F8754: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F8758: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F8760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F8760 size=112
    let mut pc: u32 = 0x826F8760;
    'dispatch: loop {
        match pc {
            0x826F8760 => {
    //   block [0x826F8760..0x826F87D0)
	// 826F8760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F8764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F8768: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F876C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F8770: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F8774: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F8778: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F877C: 390BFD08  addi r8, r11, -0x2f8
	ctx.r[8].s64 = ctx.r[11].s64 + -760;
	// 826F8780: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826F8784: 388A523C  addi r4, r10, 0x523c
	ctx.r[4].s64 = ctx.r[10].s64 + 21052;
	// 826F8788: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F878C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8790: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F8794: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F8798: 386AAE34  addi r3, r10, -0x51cc
	ctx.r[3].s64 = ctx.r[10].s64 + -20940;
	// 826F879C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F87A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F87A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F87A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F87AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F87B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F87B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F87B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F87BC: 4BD6E665  bl 0x82466e20
	ctx.lr = 0x826F87C0;
	sub_82466E20(ctx, base);
	// 826F87C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F87C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F87C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F87CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F87D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F87D0 size=112
    let mut pc: u32 = 0x826F87D0;
    'dispatch: loop {
        match pc {
            0x826F87D0 => {
    //   block [0x826F87D0..0x826F8840)
	// 826F87D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F87D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F87D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F87DC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F87E0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F87E4: 38AAA8F4  addi r5, r10, -0x570c
	ctx.r[5].s64 = ctx.r[10].s64 + -22284;
	// 826F87E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826F87EC: 390BFD50  addi r8, r11, -0x2b0
	ctx.r[8].s64 = ctx.r[11].s64 + -688;
	// 826F87F0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F87F4: 388A0E58  addi r4, r10, 0xe58
	ctx.r[4].s64 = ctx.r[10].s64 + 3672;
	// 826F87F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F87FC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8800: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F8804: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F8808: 386AAE64  addi r3, r10, -0x519c
	ctx.r[3].s64 = ctx.r[10].s64 + -20892;
	// 826F880C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F8810: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F8814: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F8818: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F881C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F8820: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F8824: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F8828: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F882C: 4BD6E5F5  bl 0x82466e20
	ctx.lr = 0x826F8830;
	sub_82466E20(ctx, base);
	// 826F8830: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F8834: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F8838: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F883C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F8840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F8840 size=108
    let mut pc: u32 = 0x826F8840;
    'dispatch: loop {
        match pc {
            0x826F8840 => {
    //   block [0x826F8840..0x826F88AC)
	// 826F8840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F8844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F8848: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F884C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F8850: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826F8854: 38EBFD68  addi r7, r11, -0x298
	ctx.r[7].s64 = ctx.r[11].s64 + -664;
	// 826F8858: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F885C: 388A0E74  addi r4, r10, 0xe74
	ctx.r[4].s64 = ctx.r[10].s64 + 3700;
	// 826F8860: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F8864: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8868: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F886C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F8870: 386AAE94  addi r3, r10, -0x516c
	ctx.r[3].s64 = ctx.r[10].s64 + -20844;
	// 826F8874: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F8878: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F887C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F8880: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F8884: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F8888: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F888C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F8890: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F8894: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F8898: 4BD6E589  bl 0x82466e20
	ctx.lr = 0x826F889C;
	sub_82466E20(ctx, base);
	// 826F889C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F88A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F88A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F88A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F88B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F88B0 size=108
    let mut pc: u32 = 0x826F88B0;
    'dispatch: loop {
        match pc {
            0x826F88B0 => {
    //   block [0x826F88B0..0x826F891C)
	// 826F88B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F88B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F88B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F88BC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F88C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826F88C4: 38EBFD80  addi r7, r11, -0x280
	ctx.r[7].s64 = ctx.r[11].s64 + -640;
	// 826F88C8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F88CC: 388A0E9C  addi r4, r10, 0xe9c
	ctx.r[4].s64 = ctx.r[10].s64 + 3740;
	// 826F88D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F88D4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F88D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F88DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F88E0: 386AAEC4  addi r3, r10, -0x513c
	ctx.r[3].s64 = ctx.r[10].s64 + -20796;
	// 826F88E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F88E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F88EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F88F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F88F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F88F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F88FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F8900: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F8904: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F8908: 4BD6E519  bl 0x82466e20
	ctx.lr = 0x826F890C;
	sub_82466E20(ctx, base);
	// 826F890C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F8910: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F8914: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F8918: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F8920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F8920 size=112
    let mut pc: u32 = 0x826F8920;
    'dispatch: loop {
        match pc {
            0x826F8920 => {
    //   block [0x826F8920..0x826F8990)
	// 826F8920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F8924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F8928: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F892C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8930: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F8934: 38AAAEC4  addi r5, r10, -0x513c
	ctx.r[5].s64 = ctx.r[10].s64 + -20796;
	// 826F8938: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826F893C: 390BFDB0  addi r8, r11, -0x250
	ctx.r[8].s64 = ctx.r[11].s64 + -592;
	// 826F8940: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826F8944: 388A0EB4  addi r4, r10, 0xeb4
	ctx.r[4].s64 = ctx.r[10].s64 + 3764;
	// 826F8948: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F894C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8950: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F8954: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F8958: 386AAEF4  addi r3, r10, -0x510c
	ctx.r[3].s64 = ctx.r[10].s64 + -20748;
	// 826F895C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F8960: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F8964: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F8968: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F896C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F8970: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F8974: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F8978: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F897C: 4BD6E4A5  bl 0x82466e20
	ctx.lr = 0x826F8980;
	sub_82466E20(ctx, base);
	// 826F8980: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F8984: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F8988: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F898C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F8990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826F8990 size=24
    let mut pc: u32 = 0x826F8990;
    'dispatch: loop {
        match pc {
            0x826F8990 => {
    //   block [0x826F8990..0x826F89A8)
	// 826F8990: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F8994: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F8998: 394A2A20  addi r10, r10, 0x2a20
	ctx.r[10].s64 = ctx.r[10].s64 + 10784;
	// 826F899C: 816BF77C  lwz r11, -0x884(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-2180 as u32) ) } as u64;
	// 826F89A0: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 826F89A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F89A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F89A8 size=116
    let mut pc: u32 = 0x826F89A8;
    'dispatch: loop {
        match pc {
            0x826F89A8 => {
    //   block [0x826F89A8..0x826F8A1C)
	// 826F89A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F89AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F89B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F89B4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F89B8: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F89BC: 390B2A20  addi r8, r11, 0x2a20
	ctx.r[8].s64 = ctx.r[11].s64 + 10784;
	// 826F89C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F89C4: 392AAC08  addi r9, r10, -0x53f8
	ctx.r[9].s64 = ctx.r[10].s64 + -21496;
	// 826F89C8: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F89CC: 38E0000D  li r7, 0xd
	ctx.r[7].s64 = 13;
	// 826F89D0: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F89D4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F89D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F89DC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826F89E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F89E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F89E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F89EC: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 826F89F0: 388A0EDC  addi r4, r10, 0xedc
	ctx.r[4].s64 = ctx.r[10].s64 + 3804;
	// 826F89F4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F89F8: 386BAF24  addi r3, r11, -0x50dc
	ctx.r[3].s64 = ctx.r[11].s64 + -20700;
	// 826F89FC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F8A00: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F8A04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F8A08: 4BD6E419  bl 0x82466e20
	ctx.lr = 0x826F8A0C;
	sub_82466E20(ctx, base);
	// 826F8A0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F8A10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F8A14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F8A18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F8A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F8A20 size=112
    let mut pc: u32 = 0x826F8A20;
    'dispatch: loop {
        match pc {
            0x826F8A20 => {
    //   block [0x826F8A20..0x826F8A90)
	// 826F8A20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F8A24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F8A28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F8A2C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8A30: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F8A34: 38AA9BD4  addi r5, r10, -0x642c
	ctx.r[5].s64 = ctx.r[10].s64 + -25644;
	// 826F8A38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F8A3C: 390BFDE0  addi r8, r11, -0x220
	ctx.r[8].s64 = ctx.r[11].s64 + -544;
	// 826F8A40: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826F8A44: 388A5254  addi r4, r10, 0x5254
	ctx.r[4].s64 = ctx.r[10].s64 + 21076;
	// 826F8A48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F8A4C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8A50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F8A54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F8A58: 386AAF54  addi r3, r10, -0x50ac
	ctx.r[3].s64 = ctx.r[10].s64 + -20652;
	// 826F8A5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F8A60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F8A64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F8A68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F8A6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F8A70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F8A74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F8A78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F8A7C: 4BD6E3A5  bl 0x82466e20
	ctx.lr = 0x826F8A80;
	sub_82466E20(ctx, base);
	// 826F8A80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F8A84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F8A88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F8A8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F8A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F8A90 size=108
    let mut pc: u32 = 0x826F8A90;
    'dispatch: loop {
        match pc {
            0x826F8A90 => {
    //   block [0x826F8A90..0x826F8AFC)
	// 826F8A90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F8A94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F8A98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F8A9C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F8AA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F8AA4: 38EBFE70  addi r7, r11, -0x190
	ctx.r[7].s64 = ctx.r[11].s64 + -400;
	// 826F8AA8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826F8AAC: 388A5268  addi r4, r10, 0x5268
	ctx.r[4].s64 = ctx.r[10].s64 + 21096;
	// 826F8AB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F8AB4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8AB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F8ABC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F8AC0: 386AAF84  addi r3, r10, -0x507c
	ctx.r[3].s64 = ctx.r[10].s64 + -20604;
	// 826F8AC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F8AC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F8ACC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F8AD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F8AD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F8AD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F8ADC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F8AE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F8AE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F8AE8: 4BD6E339  bl 0x82466e20
	ctx.lr = 0x826F8AEC;
	sub_82466E20(ctx, base);
	// 826F8AEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F8AF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F8AF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F8AF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F8B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F8B00 size=108
    let mut pc: u32 = 0x826F8B00;
    'dispatch: loop {
        match pc {
            0x826F8B00 => {
    //   block [0x826F8B00..0x826F8B6C)
	// 826F8B00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F8B04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F8B08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F8B0C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F8B10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F8B14: 38EBFEB8  addi r7, r11, -0x148
	ctx.r[7].s64 = ctx.r[11].s64 + -328;
	// 826F8B18: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F8B1C: 388A5284  addi r4, r10, 0x5284
	ctx.r[4].s64 = ctx.r[10].s64 + 21124;
	// 826F8B20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F8B24: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8B28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F8B2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F8B30: 386AAFB4  addi r3, r10, -0x504c
	ctx.r[3].s64 = ctx.r[10].s64 + -20556;
	// 826F8B34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F8B38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F8B3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F8B40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F8B44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F8B48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F8B4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F8B50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F8B54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F8B58: 4BD6E2C9  bl 0x82466e20
	ctx.lr = 0x826F8B5C;
	sub_82466E20(ctx, base);
	// 826F8B5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F8B60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F8B64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F8B68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F8B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F8B70 size=108
    let mut pc: u32 = 0x826F8B70;
    'dispatch: loop {
        match pc {
            0x826F8B70 => {
    //   block [0x826F8B70..0x826F8BDC)
	// 826F8B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F8B74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F8B78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F8B7C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F8B80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F8B84: 38EBFEE8  addi r7, r11, -0x118
	ctx.r[7].s64 = ctx.r[11].s64 + -280;
	// 826F8B88: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F8B8C: 388A52A4  addi r4, r10, 0x52a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21156;
	// 826F8B90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F8B94: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8B98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F8B9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F8BA0: 386AAFE4  addi r3, r10, -0x501c
	ctx.r[3].s64 = ctx.r[10].s64 + -20508;
	// 826F8BA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F8BA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F8BAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F8BB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F8BB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F8BB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F8BBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F8BC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F8BC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F8BC8: 4BD6E259  bl 0x82466e20
	ctx.lr = 0x826F8BCC;
	sub_82466E20(ctx, base);
	// 826F8BCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F8BD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F8BD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F8BD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F8BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F8BE0 size=112
    let mut pc: u32 = 0x826F8BE0;
    'dispatch: loop {
        match pc {
            0x826F8BE0 => {
    //   block [0x826F8BE0..0x826F8C50)
	// 826F8BE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F8BE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F8BE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F8BEC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F8BF0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F8BF4: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F8BF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F8BFC: 390BFF18  addi r8, r11, -0xe8
	ctx.r[8].s64 = ctx.r[11].s64 + -232;
	// 826F8C00: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826F8C04: 388A52BC  addi r4, r10, 0x52bc
	ctx.r[4].s64 = ctx.r[10].s64 + 21180;
	// 826F8C08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F8C0C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8C10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F8C14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F8C18: 386AB014  addi r3, r10, -0x4fec
	ctx.r[3].s64 = ctx.r[10].s64 + -20460;
	// 826F8C1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F8C20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F8C24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F8C28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F8C2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F8C30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F8C34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F8C38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F8C3C: 4BD6E1E5  bl 0x82466e20
	ctx.lr = 0x826F8C40;
	sub_82466E20(ctx, base);
	// 826F8C40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F8C44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F8C48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F8C4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F8C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F8C50 size=112
    let mut pc: u32 = 0x826F8C50;
    'dispatch: loop {
        match pc {
            0x826F8C50 => {
    //   block [0x826F8C50..0x826F8CC0)
	// 826F8C50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F8C54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F8C58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F8C5C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F8C60: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F8C64: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F8C68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F8C6C: 390BFF48  addi r8, r11, -0xb8
	ctx.r[8].s64 = ctx.r[11].s64 + -184;
	// 826F8C70: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F8C74: 388A52CC  addi r4, r10, 0x52cc
	ctx.r[4].s64 = ctx.r[10].s64 + 21196;
	// 826F8C78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F8C7C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8C80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F8C84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F8C88: 386AB044  addi r3, r10, -0x4fbc
	ctx.r[3].s64 = ctx.r[10].s64 + -20412;
	// 826F8C8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F8C90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F8C94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F8C98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F8C9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F8CA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F8CA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F8CA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F8CAC: 4BD6E175  bl 0x82466e20
	ctx.lr = 0x826F8CB0;
	sub_82466E20(ctx, base);
	// 826F8CB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F8CB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F8CB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F8CBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F8CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F8CC0 size=112
    let mut pc: u32 = 0x826F8CC0;
    'dispatch: loop {
        match pc {
            0x826F8CC0 => {
    //   block [0x826F8CC0..0x826F8D30)
	// 826F8CC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F8CC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F8CC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F8CCC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F8CD0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F8CD4: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F8CD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F8CDC: 390BFF60  addi r8, r11, -0xa0
	ctx.r[8].s64 = ctx.r[11].s64 + -160;
	// 826F8CE0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F8CE4: 388A52E8  addi r4, r10, 0x52e8
	ctx.r[4].s64 = ctx.r[10].s64 + 21224;
	// 826F8CE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F8CEC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8CF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F8CF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F8CF8: 386AB074  addi r3, r10, -0x4f8c
	ctx.r[3].s64 = ctx.r[10].s64 + -20364;
	// 826F8CFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F8D00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F8D04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F8D08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F8D0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F8D10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F8D14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F8D18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F8D1C: 4BD6E105  bl 0x82466e20
	ctx.lr = 0x826F8D20;
	sub_82466E20(ctx, base);
	// 826F8D20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F8D24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F8D28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F8D2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F8D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F8D30 size=108
    let mut pc: u32 = 0x826F8D30;
    'dispatch: loop {
        match pc {
            0x826F8D30 => {
    //   block [0x826F8D30..0x826F8D9C)
	// 826F8D30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F8D34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F8D38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F8D3C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F8D40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F8D44: 38EBFF78  addi r7, r11, -0x88
	ctx.r[7].s64 = ctx.r[11].s64 + -136;
	// 826F8D48: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F8D4C: 388A5308  addi r4, r10, 0x5308
	ctx.r[4].s64 = ctx.r[10].s64 + 21256;
	// 826F8D50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F8D54: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8D58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F8D5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F8D60: 386AB0A4  addi r3, r10, -0x4f5c
	ctx.r[3].s64 = ctx.r[10].s64 + -20316;
	// 826F8D64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F8D68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F8D6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F8D70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F8D74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F8D78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F8D7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F8D80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F8D84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F8D88: 4BD6E099  bl 0x82466e20
	ctx.lr = 0x826F8D8C;
	sub_82466E20(ctx, base);
	// 826F8D8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F8D90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F8D94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F8D98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F8DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F8DA0 size=112
    let mut pc: u32 = 0x826F8DA0;
    'dispatch: loop {
        match pc {
            0x826F8DA0 => {
    //   block [0x826F8DA0..0x826F8E10)
	// 826F8DA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F8DA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F8DA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F8DAC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F8DB0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F8DB4: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F8DB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F8DBC: 390BFFA8  addi r8, r11, -0x58
	ctx.r[8].s64 = ctx.r[11].s64 + -88;
	// 826F8DC0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F8DC4: 388A533C  addi r4, r10, 0x533c
	ctx.r[4].s64 = ctx.r[10].s64 + 21308;
	// 826F8DC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F8DCC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8DD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F8DD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F8DD8: 386AB0D4  addi r3, r10, -0x4f2c
	ctx.r[3].s64 = ctx.r[10].s64 + -20268;
	// 826F8DDC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F8DE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F8DE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F8DE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F8DEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F8DF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F8DF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F8DF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F8DFC: 4BD6E025  bl 0x82466e20
	ctx.lr = 0x826F8E00;
	sub_82466E20(ctx, base);
	// 826F8E00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F8E04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F8E08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F8E0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F8E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F8E10 size=108
    let mut pc: u32 = 0x826F8E10;
    'dispatch: loop {
        match pc {
            0x826F8E10 => {
    //   block [0x826F8E10..0x826F8E7C)
	// 826F8E10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F8E14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F8E18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F8E1C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F8E20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F8E24: 38EBFFC0  addi r7, r11, -0x40
	ctx.r[7].s64 = ctx.r[11].s64 + -64;
	// 826F8E28: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 826F8E2C: 388A535C  addi r4, r10, 0x535c
	ctx.r[4].s64 = ctx.r[10].s64 + 21340;
	// 826F8E30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F8E34: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8E38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F8E3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F8E40: 386AB104  addi r3, r10, -0x4efc
	ctx.r[3].s64 = ctx.r[10].s64 + -20220;
	// 826F8E44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F8E48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F8E4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F8E50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F8E54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F8E58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F8E5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F8E60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F8E64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F8E68: 4BD6DFB9  bl 0x82466e20
	ctx.lr = 0x826F8E6C;
	sub_82466E20(ctx, base);
	// 826F8E6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F8E70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F8E74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F8E78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F8E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F8E80 size=116
    let mut pc: u32 = 0x826F8E80;
    'dispatch: loop {
        match pc {
            0x826F8E80 => {
    //   block [0x826F8E80..0x826F8EF4)
	// 826F8E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F8E84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F8E88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F8E8C: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F8E90: 38E00013  li r7, 0x13
	ctx.r[7].s64 = 19;
	// 826F8E94: 390A00B0  addi r8, r10, 0xb0
	ctx.r[8].s64 = ctx.r[10].s64 + 176;
	// 826F8E98: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F8E9C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826F8EA0: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F8EA4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F8EA8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F8EAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F8EB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F8EB4: 388A5380  addi r4, r10, 0x5380
	ctx.r[4].s64 = ctx.r[10].s64 + 21376;
	// 826F8EB8: 396BAC20  addi r11, r11, -0x53e0
	ctx.r[11].s64 = ctx.r[11].s64 + -21472;
	// 826F8EBC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8EC0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F8EC4: 386AB134  addi r3, r10, -0x4ecc
	ctx.r[3].s64 = ctx.r[10].s64 + -20172;
	// 826F8EC8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826F8ECC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F8ED0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826F8ED4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F8ED8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F8EDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F8EE0: 4BD6DF41  bl 0x82466e20
	ctx.lr = 0x826F8EE4;
	sub_82466E20(ctx, base);
	// 826F8EE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F8EE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F8EEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F8EF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F8EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F8EF8 size=108
    let mut pc: u32 = 0x826F8EF8;
    'dispatch: loop {
        match pc {
            0x826F8EF8 => {
    //   block [0x826F8EF8..0x826F8F64)
	// 826F8EF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F8EFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F8F00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F8F04: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F8F08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F8F0C: 38EB0278  addi r7, r11, 0x278
	ctx.r[7].s64 = ctx.r[11].s64 + 632;
	// 826F8F10: 39000011  li r8, 0x11
	ctx.r[8].s64 = 17;
	// 826F8F14: 388A5390  addi r4, r10, 0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + 21392;
	// 826F8F18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F8F1C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8F20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F8F24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F8F28: 386AB164  addi r3, r10, -0x4e9c
	ctx.r[3].s64 = ctx.r[10].s64 + -20124;
	// 826F8F2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F8F30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F8F34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F8F38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F8F3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F8F40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F8F44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F8F48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F8F4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F8F50: 4BD6DED1  bl 0x82466e20
	ctx.lr = 0x826F8F54;
	sub_82466E20(ctx, base);
	// 826F8F54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F8F58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F8F5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F8F60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F8F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F8F68 size=112
    let mut pc: u32 = 0x826F8F68;
    'dispatch: loop {
        match pc {
            0x826F8F68 => {
    //   block [0x826F8F68..0x826F8FD8)
	// 826F8F68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F8F6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F8F70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F8F74: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8F78: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F8F7C: 38AA9BD4  addi r5, r10, -0x642c
	ctx.r[5].s64 = ctx.r[10].s64 + -25644;
	// 826F8F80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F8F84: 390B0410  addi r8, r11, 0x410
	ctx.r[8].s64 = ctx.r[11].s64 + 1040;
	// 826F8F88: 39200019  li r9, 0x19
	ctx.r[9].s64 = 25;
	// 826F8F8C: 388A53AC  addi r4, r10, 0x53ac
	ctx.r[4].s64 = ctx.r[10].s64 + 21420;
	// 826F8F90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F8F94: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F8F98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F8F9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F8FA0: 386AB194  addi r3, r10, -0x4e6c
	ctx.r[3].s64 = ctx.r[10].s64 + -20076;
	// 826F8FA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F8FA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F8FAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F8FB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F8FB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F8FB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F8FBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F8FC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F8FC4: 4BD6DE5D  bl 0x82466e20
	ctx.lr = 0x826F8FC8;
	sub_82466E20(ctx, base);
	// 826F8FC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F8FCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F8FD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F8FD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F8FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F8FD8 size=100
    let mut pc: u32 = 0x826F8FD8;
    'dispatch: loop {
        match pc {
            0x826F8FD8 => {
    //   block [0x826F8FD8..0x826F903C)
	// 826F8FD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F8FDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F8FE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F8FE4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F8FE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F8FEC: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F8FF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F8FF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F8FF8: 388A53C0  addi r4, r10, 0x53c0
	ctx.r[4].s64 = ctx.r[10].s64 + 21440;
	// 826F8FFC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9000: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F9004: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F9008: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F900C: 386AB1C4  addi r3, r10, -0x4e3c
	ctx.r[3].s64 = ctx.r[10].s64 + -20028;
	// 826F9010: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F9014: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F9018: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F901C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F9020: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F9024: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F9028: 4BD6DDF9  bl 0x82466e20
	ctx.lr = 0x826F902C;
	sub_82466E20(ctx, base);
	// 826F902C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F9030: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F9034: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F9038: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F9040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F9040 size=112
    let mut pc: u32 = 0x826F9040;
    'dispatch: loop {
        match pc {
            0x826F9040 => {
    //   block [0x826F9040..0x826F90B0)
	// 826F9040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F9044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F9048: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F904C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9050: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F9054: 38AAB1C4  addi r5, r10, -0x4e3c
	ctx.r[5].s64 = ctx.r[10].s64 + -20028;
	// 826F9058: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F905C: 390B0668  addi r8, r11, 0x668
	ctx.r[8].s64 = ctx.r[11].s64 + 1640;
	// 826F9060: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826F9064: 388A53D8  addi r4, r10, 0x53d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21464;
	// 826F9068: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F906C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9070: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F9074: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F9078: 386AB1F4  addi r3, r10, -0x4e0c
	ctx.r[3].s64 = ctx.r[10].s64 + -19980;
	// 826F907C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F9080: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F9084: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F9088: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F908C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F9090: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F9094: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F9098: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F909C: 4BD6DD85  bl 0x82466e20
	ctx.lr = 0x826F90A0;
	sub_82466E20(ctx, base);
	// 826F90A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F90A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F90A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F90AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F90B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F90B0 size=100
    let mut pc: u32 = 0x826F90B0;
    'dispatch: loop {
        match pc {
            0x826F90B0 => {
    //   block [0x826F90B0..0x826F9114)
	// 826F90B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F90B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F90B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F90BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F90C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F90C4: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F90C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F90CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F90D0: 388A53F8  addi r4, r10, 0x53f8
	ctx.r[4].s64 = ctx.r[10].s64 + 21496;
	// 826F90D4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F90D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F90DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F90E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F90E4: 386AB224  addi r3, r10, -0x4ddc
	ctx.r[3].s64 = ctx.r[10].s64 + -19932;
	// 826F90E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F90EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F90F0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F90F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F90F8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F90FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F9100: 4BD6DD21  bl 0x82466e20
	ctx.lr = 0x826F9104;
	sub_82466E20(ctx, base);
	// 826F9104: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F9108: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F910C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F9110: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F9118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F9118 size=108
    let mut pc: u32 = 0x826F9118;
    'dispatch: loop {
        match pc {
            0x826F9118 => {
    //   block [0x826F9118..0x826F9184)
	// 826F9118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F911C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F9120: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F9124: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F9128: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F912C: 38EB06E0  addi r7, r11, 0x6e0
	ctx.r[7].s64 = ctx.r[11].s64 + 1760;
	// 826F9130: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826F9134: 388A5408  addi r4, r10, 0x5408
	ctx.r[4].s64 = ctx.r[10].s64 + 21512;
	// 826F9138: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F913C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9140: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F9144: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F9148: 386AB254  addi r3, r10, -0x4dac
	ctx.r[3].s64 = ctx.r[10].s64 + -19884;
	// 826F914C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F9150: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F9154: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F9158: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F915C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F9160: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F9164: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F9168: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F916C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F9170: 4BD6DCB1  bl 0x82466e20
	ctx.lr = 0x826F9174;
	sub_82466E20(ctx, base);
	// 826F9174: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F9178: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F917C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F9180: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F9188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F9188 size=112
    let mut pc: u32 = 0x826F9188;
    'dispatch: loop {
        match pc {
            0x826F9188 => {
    //   block [0x826F9188..0x826F91F8)
	// 826F9188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F918C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F9190: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F9194: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9198: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F919C: 38AAB224  addi r5, r10, -0x4ddc
	ctx.r[5].s64 = ctx.r[10].s64 + -19932;
	// 826F91A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F91A4: 390B0728  addi r8, r11, 0x728
	ctx.r[8].s64 = ctx.r[11].s64 + 1832;
	// 826F91A8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826F91AC: 388A5434  addi r4, r10, 0x5434
	ctx.r[4].s64 = ctx.r[10].s64 + 21556;
	// 826F91B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F91B4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F91B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F91BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F91C0: 386AB284  addi r3, r10, -0x4d7c
	ctx.r[3].s64 = ctx.r[10].s64 + -19836;
	// 826F91C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F91C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F91CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F91D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F91D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F91D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F91DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F91E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F91E4: 4BD6DC3D  bl 0x82466e20
	ctx.lr = 0x826F91E8;
	sub_82466E20(ctx, base);
	// 826F91E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F91EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F91F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F91F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F91F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F91F8 size=100
    let mut pc: u32 = 0x826F91F8;
    'dispatch: loop {
        match pc {
            0x826F91F8 => {
    //   block [0x826F91F8..0x826F925C)
	// 826F91F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F91FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F9200: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F9204: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F9208: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F920C: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F9210: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F9214: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F9218: 388A544C  addi r4, r10, 0x544c
	ctx.r[4].s64 = ctx.r[10].s64 + 21580;
	// 826F921C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9220: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F9224: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F9228: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F922C: 386AB2B4  addi r3, r10, -0x4d4c
	ctx.r[3].s64 = ctx.r[10].s64 + -19788;
	// 826F9230: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F9234: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F9238: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F923C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F9240: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F9244: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F9248: 4BD6DBD9  bl 0x82466e20
	ctx.lr = 0x826F924C;
	sub_82466E20(ctx, base);
	// 826F924C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F9250: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F9254: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F9258: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F9260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F9260 size=100
    let mut pc: u32 = 0x826F9260;
    'dispatch: loop {
        match pc {
            0x826F9260 => {
    //   block [0x826F9260..0x826F92C4)
	// 826F9260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F9264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F9268: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F926C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F9270: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F9274: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F9278: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F927C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F9280: 388A5468  addi r4, r10, 0x5468
	ctx.r[4].s64 = ctx.r[10].s64 + 21608;
	// 826F9284: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9288: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F928C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F9290: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F9294: 386AB2E4  addi r3, r10, -0x4d1c
	ctx.r[3].s64 = ctx.r[10].s64 + -19740;
	// 826F9298: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F929C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F92A0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F92A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F92A8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F92AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F92B0: 4BD6DB71  bl 0x82466e20
	ctx.lr = 0x826F92B4;
	sub_82466E20(ctx, base);
	// 826F92B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F92B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F92BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F92C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F92C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F92C8 size=112
    let mut pc: u32 = 0x826F92C8;
    'dispatch: loop {
        match pc {
            0x826F92C8 => {
    //   block [0x826F92C8..0x826F9338)
	// 826F92C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F92CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F92D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F92D4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F92D8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F92DC: 38AAB2B4  addi r5, r10, -0x4d4c
	ctx.r[5].s64 = ctx.r[10].s64 + -19788;
	// 826F92E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F92E4: 390B0758  addi r8, r11, 0x758
	ctx.r[8].s64 = ctx.r[11].s64 + 1880;
	// 826F92E8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826F92EC: 388A5480  addi r4, r10, 0x5480
	ctx.r[4].s64 = ctx.r[10].s64 + 21632;
	// 826F92F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F92F4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F92F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F92FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F9300: 386AB314  addi r3, r10, -0x4cec
	ctx.r[3].s64 = ctx.r[10].s64 + -19692;
	// 826F9304: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F9308: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F930C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F9310: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F9314: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F9318: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F931C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F9320: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F9324: 4BD6DAFD  bl 0x82466e20
	ctx.lr = 0x826F9328;
	sub_82466E20(ctx, base);
	// 826F9328: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F932C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F9330: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F9334: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F9338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F9338 size=112
    let mut pc: u32 = 0x826F9338;
    'dispatch: loop {
        match pc {
            0x826F9338 => {
    //   block [0x826F9338..0x826F93A8)
	// 826F9338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F933C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F9340: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F9344: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9348: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F934C: 38AAB2E4  addi r5, r10, -0x4d1c
	ctx.r[5].s64 = ctx.r[10].s64 + -19740;
	// 826F9350: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F9354: 390B07B8  addi r8, r11, 0x7b8
	ctx.r[8].s64 = ctx.r[11].s64 + 1976;
	// 826F9358: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826F935C: 388A54A4  addi r4, r10, 0x54a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21668;
	// 826F9360: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F9364: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9368: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F936C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F9370: 386AB344  addi r3, r10, -0x4cbc
	ctx.r[3].s64 = ctx.r[10].s64 + -19644;
	// 826F9374: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F9378: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F937C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F9380: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F9384: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F9388: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F938C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F9390: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F9394: 4BD6DA8D  bl 0x82466e20
	ctx.lr = 0x826F9398;
	sub_82466E20(ctx, base);
	// 826F9398: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F939C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F93A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F93A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F93A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F93A8 size=100
    let mut pc: u32 = 0x826F93A8;
    'dispatch: loop {
        match pc {
            0x826F93A8 => {
    //   block [0x826F93A8..0x826F940C)
	// 826F93A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F93AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F93B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F93B4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F93B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F93BC: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F93C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F93C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F93C8: 388A54C8  addi r4, r10, 0x54c8
	ctx.r[4].s64 = ctx.r[10].s64 + 21704;
	// 826F93CC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F93D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F93D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F93D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F93DC: 386AB374  addi r3, r10, -0x4c8c
	ctx.r[3].s64 = ctx.r[10].s64 + -19596;
	// 826F93E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F93E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F93E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F93EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F93F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F93F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F93F8: 4BD6DA29  bl 0x82466e20
	ctx.lr = 0x826F93FC;
	sub_82466E20(ctx, base);
	// 826F93FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F9400: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F9404: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F9408: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F9410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F9410 size=112
    let mut pc: u32 = 0x826F9410;
    'dispatch: loop {
        match pc {
            0x826F9410 => {
    //   block [0x826F9410..0x826F9480)
	// 826F9410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F9414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F9418: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F941C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9420: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F9424: 38AAB374  addi r5, r10, -0x4c8c
	ctx.r[5].s64 = ctx.r[10].s64 + -19596;
	// 826F9428: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F942C: 390B0818  addi r8, r11, 0x818
	ctx.r[8].s64 = ctx.r[11].s64 + 2072;
	// 826F9430: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 826F9434: 388A54D8  addi r4, r10, 0x54d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21720;
	// 826F9438: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F943C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9440: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F9444: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F9448: 386AB3A4  addi r3, r10, -0x4c5c
	ctx.r[3].s64 = ctx.r[10].s64 + -19548;
	// 826F944C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F9450: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F9454: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F9458: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F945C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F9460: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F9464: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F9468: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F946C: 4BD6D9B5  bl 0x82466e20
	ctx.lr = 0x826F9470;
	sub_82466E20(ctx, base);
	// 826F9470: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F9474: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F9478: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F947C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F9480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F9480 size=108
    let mut pc: u32 = 0x826F9480;
    'dispatch: loop {
        match pc {
            0x826F9480 => {
    //   block [0x826F9480..0x826F94EC)
	// 826F9480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F9484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F9488: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F948C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F9490: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F9494: 38EB0908  addi r7, r11, 0x908
	ctx.r[7].s64 = ctx.r[11].s64 + 2312;
	// 826F9498: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 826F949C: 388A54F0  addi r4, r10, 0x54f0
	ctx.r[4].s64 = ctx.r[10].s64 + 21744;
	// 826F94A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F94A4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F94A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F94AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F94B0: 386AB3D4  addi r3, r10, -0x4c2c
	ctx.r[3].s64 = ctx.r[10].s64 + -19500;
	// 826F94B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F94B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F94BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F94C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F94C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F94C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F94CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F94D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F94D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F94D8: 4BD6D949  bl 0x82466e20
	ctx.lr = 0x826F94DC;
	sub_82466E20(ctx, base);
	// 826F94DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F94E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F94E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F94E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F94F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F94F0 size=108
    let mut pc: u32 = 0x826F94F0;
    'dispatch: loop {
        match pc {
            0x826F94F0 => {
    //   block [0x826F94F0..0x826F955C)
	// 826F94F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F94F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F94F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F94FC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F9500: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F9504: 38EB09F8  addi r7, r11, 0x9f8
	ctx.r[7].s64 = ctx.r[11].s64 + 2552;
	// 826F9508: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826F950C: 388A551C  addi r4, r10, 0x551c
	ctx.r[4].s64 = ctx.r[10].s64 + 21788;
	// 826F9510: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F9514: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9518: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F951C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F9520: 386AB404  addi r3, r10, -0x4bfc
	ctx.r[3].s64 = ctx.r[10].s64 + -19452;
	// 826F9524: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F9528: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F952C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F9530: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F9534: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F9538: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F953C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F9540: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F9544: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F9548: 4BD6D8D9  bl 0x82466e20
	ctx.lr = 0x826F954C;
	sub_82466E20(ctx, base);
	// 826F954C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F9550: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F9554: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F9558: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F9560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F9560 size=108
    let mut pc: u32 = 0x826F9560;
    'dispatch: loop {
        match pc {
            0x826F9560 => {
    //   block [0x826F9560..0x826F95CC)
	// 826F9560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F9564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F9568: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F956C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F9570: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F9574: 38EB0A40  addi r7, r11, 0xa40
	ctx.r[7].s64 = ctx.r[11].s64 + 2624;
	// 826F9578: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826F957C: 388A553C  addi r4, r10, 0x553c
	ctx.r[4].s64 = ctx.r[10].s64 + 21820;
	// 826F9580: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F9584: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9588: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F958C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F9590: 386AB434  addi r3, r10, -0x4bcc
	ctx.r[3].s64 = ctx.r[10].s64 + -19404;
	// 826F9594: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F9598: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F959C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F95A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F95A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F95A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F95AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F95B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F95B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F95B8: 4BD6D869  bl 0x82466e20
	ctx.lr = 0x826F95BC;
	sub_82466E20(ctx, base);
	// 826F95BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F95C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F95C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F95C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F95D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F95D0 size=108
    let mut pc: u32 = 0x826F95D0;
    'dispatch: loop {
        match pc {
            0x826F95D0 => {
    //   block [0x826F95D0..0x826F963C)
	// 826F95D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F95D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F95D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F95DC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F95E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F95E4: 38EB0B18  addi r7, r11, 0xb18
	ctx.r[7].s64 = ctx.r[11].s64 + 2840;
	// 826F95E8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F95EC: 388A5560  addi r4, r10, 0x5560
	ctx.r[4].s64 = ctx.r[10].s64 + 21856;
	// 826F95F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F95F4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F95F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F95FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F9600: 386AB464  addi r3, r10, -0x4b9c
	ctx.r[3].s64 = ctx.r[10].s64 + -19356;
	// 826F9604: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F9608: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F960C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F9610: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F9614: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F9618: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F961C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F9620: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F9624: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F9628: 4BD6D7F9  bl 0x82466e20
	ctx.lr = 0x826F962C;
	sub_82466E20(ctx, base);
	// 826F962C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F9630: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F9634: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F9638: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F9640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F9640 size=100
    let mut pc: u32 = 0x826F9640;
    'dispatch: loop {
        match pc {
            0x826F9640 => {
    //   block [0x826F9640..0x826F96A4)
	// 826F9640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F9644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F9648: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F964C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F9650: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F9654: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F9658: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F965C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F9660: 388A5578  addi r4, r10, 0x5578
	ctx.r[4].s64 = ctx.r[10].s64 + 21880;
	// 826F9664: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9668: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F966C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F9670: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F9674: 386AB494  addi r3, r10, -0x4b6c
	ctx.r[3].s64 = ctx.r[10].s64 + -19308;
	// 826F9678: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F967C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F9680: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F9684: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F9688: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F968C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F9690: 4BD6D791  bl 0x82466e20
	ctx.lr = 0x826F9694;
	sub_82466E20(ctx, base);
	// 826F9694: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F9698: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F969C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F96A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F96A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F96A8 size=112
    let mut pc: u32 = 0x826F96A8;
    'dispatch: loop {
        match pc {
            0x826F96A8 => {
    //   block [0x826F96A8..0x826F9718)
	// 826F96A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F96AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F96B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F96B4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F96B8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F96BC: 38AAB494  addi r5, r10, -0x4b6c
	ctx.r[5].s64 = ctx.r[10].s64 + -19308;
	// 826F96C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F96C4: 390B0B30  addi r8, r11, 0xb30
	ctx.r[8].s64 = ctx.r[11].s64 + 2864;
	// 826F96C8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826F96CC: 388A558C  addi r4, r10, 0x558c
	ctx.r[4].s64 = ctx.r[10].s64 + 21900;
	// 826F96D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F96D4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F96D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F96DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F96E0: 386AB4C4  addi r3, r10, -0x4b3c
	ctx.r[3].s64 = ctx.r[10].s64 + -19260;
	// 826F96E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F96E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F96EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F96F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F96F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F96F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F96FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F9700: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F9704: 4BD6D71D  bl 0x82466e20
	ctx.lr = 0x826F9708;
	sub_82466E20(ctx, base);
	// 826F9708: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F970C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F9710: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F9714: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F9718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F9718 size=108
    let mut pc: u32 = 0x826F9718;
    'dispatch: loop {
        match pc {
            0x826F9718 => {
    //   block [0x826F9718..0x826F9784)
	// 826F9718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F971C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F9720: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F9724: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F9728: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F972C: 38EB0B78  addi r7, r11, 0xb78
	ctx.r[7].s64 = ctx.r[11].s64 + 2936;
	// 826F9730: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826F9734: 388A55A8  addi r4, r10, 0x55a8
	ctx.r[4].s64 = ctx.r[10].s64 + 21928;
	// 826F9738: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F973C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9740: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F9744: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F9748: 386AB4F4  addi r3, r10, -0x4b0c
	ctx.r[3].s64 = ctx.r[10].s64 + -19212;
	// 826F974C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F9750: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F9754: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F9758: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F975C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F9760: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F9764: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F9768: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F976C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F9770: 4BD6D6B1  bl 0x82466e20
	ctx.lr = 0x826F9774;
	sub_82466E20(ctx, base);
	// 826F9774: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F9778: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F977C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F9780: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F9788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F9788 size=112
    let mut pc: u32 = 0x826F9788;
    'dispatch: loop {
        match pc {
            0x826F9788 => {
    //   block [0x826F9788..0x826F97F8)
	// 826F9788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F978C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F9790: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F9794: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F9798: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F979C: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F97A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F97A4: 390B0BC0  addi r8, r11, 0xbc0
	ctx.r[8].s64 = ctx.r[11].s64 + 3008;
	// 826F97A8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F97AC: 388A55D8  addi r4, r10, 0x55d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21976;
	// 826F97B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F97B4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F97B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F97BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F97C0: 386AB524  addi r3, r10, -0x4adc
	ctx.r[3].s64 = ctx.r[10].s64 + -19164;
	// 826F97C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F97C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F97CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F97D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F97D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F97D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F97DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F97E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F97E4: 4BD6D63D  bl 0x82466e20
	ctx.lr = 0x826F97E8;
	sub_82466E20(ctx, base);
	// 826F97E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F97EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F97F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F97F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F97F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F97F8 size=108
    let mut pc: u32 = 0x826F97F8;
    'dispatch: loop {
        match pc {
            0x826F97F8 => {
    //   block [0x826F97F8..0x826F9864)
	// 826F97F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F97FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F9800: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F9804: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F9808: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F980C: 38EB0BD8  addi r7, r11, 0xbd8
	ctx.r[7].s64 = ctx.r[11].s64 + 3032;
	// 826F9810: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826F9814: 388A55EC  addi r4, r10, 0x55ec
	ctx.r[4].s64 = ctx.r[10].s64 + 21996;
	// 826F9818: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F981C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9820: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F9824: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F9828: 386AB554  addi r3, r10, -0x4aac
	ctx.r[3].s64 = ctx.r[10].s64 + -19116;
	// 826F982C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F9830: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F9834: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F9838: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F983C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F9840: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F9844: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F9848: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F984C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F9850: 4BD6D5D1  bl 0x82466e20
	ctx.lr = 0x826F9854;
	sub_82466E20(ctx, base);
	// 826F9854: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F9858: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F985C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F9860: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F9868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F9868 size=112
    let mut pc: u32 = 0x826F9868;
    'dispatch: loop {
        match pc {
            0x826F9868 => {
    //   block [0x826F9868..0x826F98D8)
	// 826F9868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F986C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F9870: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F9874: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9878: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F987C: 38AAB524  addi r5, r10, -0x4adc
	ctx.r[5].s64 = ctx.r[10].s64 + -19164;
	// 826F9880: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F9884: 390B0C20  addi r8, r11, 0xc20
	ctx.r[8].s64 = ctx.r[11].s64 + 3104;
	// 826F9888: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F988C: 388A5628  addi r4, r10, 0x5628
	ctx.r[4].s64 = ctx.r[10].s64 + 22056;
	// 826F9890: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F9894: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9898: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F989C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F98A0: 386AB584  addi r3, r10, -0x4a7c
	ctx.r[3].s64 = ctx.r[10].s64 + -19068;
	// 826F98A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F98A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F98AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F98B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F98B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F98B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F98BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F98C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F98C4: 4BD6D55D  bl 0x82466e20
	ctx.lr = 0x826F98C8;
	sub_82466E20(ctx, base);
	// 826F98C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F98CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F98D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F98D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F98D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F98D8 size=100
    let mut pc: u32 = 0x826F98D8;
    'dispatch: loop {
        match pc {
            0x826F98D8 => {
    //   block [0x826F98D8..0x826F993C)
	// 826F98D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F98DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F98E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F98E4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F98E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F98EC: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F98F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F98F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F98F8: 388A5644  addi r4, r10, 0x5644
	ctx.r[4].s64 = ctx.r[10].s64 + 22084;
	// 826F98FC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9900: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F9904: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F9908: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F990C: 386AB5B4  addi r3, r10, -0x4a4c
	ctx.r[3].s64 = ctx.r[10].s64 + -19020;
	// 826F9910: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F9914: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F9918: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F991C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F9920: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F9924: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F9928: 4BD6D4F9  bl 0x82466e20
	ctx.lr = 0x826F992C;
	sub_82466E20(ctx, base);
	// 826F992C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F9930: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F9934: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F9938: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F9940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F9940 size=112
    let mut pc: u32 = 0x826F9940;
    'dispatch: loop {
        match pc {
            0x826F9940 => {
    //   block [0x826F9940..0x826F99B0)
	// 826F9940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F9944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F9948: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F994C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9950: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F9954: 38AAB5B4  addi r5, r10, -0x4a4c
	ctx.r[5].s64 = ctx.r[10].s64 + -19020;
	// 826F9958: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F995C: 390B0C38  addi r8, r11, 0xc38
	ctx.r[8].s64 = ctx.r[11].s64 + 3128;
	// 826F9960: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826F9964: 388A565C  addi r4, r10, 0x565c
	ctx.r[4].s64 = ctx.r[10].s64 + 22108;
	// 826F9968: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F996C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9970: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F9974: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F9978: 386AB5E4  addi r3, r10, -0x4a1c
	ctx.r[3].s64 = ctx.r[10].s64 + -18972;
	// 826F997C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F9980: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F9984: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F9988: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F998C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F9990: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F9994: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F9998: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F999C: 4BD6D485  bl 0x82466e20
	ctx.lr = 0x826F99A0;
	sub_82466E20(ctx, base);
	// 826F99A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F99A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F99A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F99AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F99B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F99B0 size=108
    let mut pc: u32 = 0x826F99B0;
    'dispatch: loop {
        match pc {
            0x826F99B0 => {
    //   block [0x826F99B0..0x826F9A1C)
	// 826F99B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F99B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F99B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F99BC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F99C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F99C4: 38EB0CE0  addi r7, r11, 0xce0
	ctx.r[7].s64 = ctx.r[11].s64 + 3296;
	// 826F99C8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F99CC: 388A567C  addi r4, r10, 0x567c
	ctx.r[4].s64 = ctx.r[10].s64 + 22140;
	// 826F99D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F99D4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F99D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F99DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F99E0: 386AB614  addi r3, r10, -0x49ec
	ctx.r[3].s64 = ctx.r[10].s64 + -18924;
	// 826F99E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F99E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F99EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F99F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F99F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F99F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F99FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F9A00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F9A04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F9A08: 4BD6D419  bl 0x82466e20
	ctx.lr = 0x826F9A0C;
	sub_82466E20(ctx, base);
	// 826F9A0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F9A10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F9A14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F9A18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F9A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F9A20 size=112
    let mut pc: u32 = 0x826F9A20;
    'dispatch: loop {
        match pc {
            0x826F9A20 => {
    //   block [0x826F9A20..0x826F9A90)
	// 826F9A20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F9A24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F9A28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F9A2C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F9A30: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F9A34: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F9A38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F9A3C: 390B0D10  addi r8, r11, 0xd10
	ctx.r[8].s64 = ctx.r[11].s64 + 3344;
	// 826F9A40: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826F9A44: 388A568C  addi r4, r10, 0x568c
	ctx.r[4].s64 = ctx.r[10].s64 + 22156;
	// 826F9A48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F9A4C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9A50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F9A54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F9A58: 386AB644  addi r3, r10, -0x49bc
	ctx.r[3].s64 = ctx.r[10].s64 + -18876;
	// 826F9A5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F9A60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F9A64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F9A68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F9A6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F9A70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F9A74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F9A78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F9A7C: 4BD6D3A5  bl 0x82466e20
	ctx.lr = 0x826F9A80;
	sub_82466E20(ctx, base);
	// 826F9A80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F9A84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F9A88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F9A8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F9A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F9A90 size=112
    let mut pc: u32 = 0x826F9A90;
    'dispatch: loop {
        match pc {
            0x826F9A90 => {
    //   block [0x826F9A90..0x826F9B00)
	// 826F9A90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F9A94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F9A98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F9A9C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F9AA0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F9AA4: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F9AA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F9AAC: 390B0D58  addi r8, r11, 0xd58
	ctx.r[8].s64 = ctx.r[11].s64 + 3416;
	// 826F9AB0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826F9AB4: 388A56A0  addi r4, r10, 0x56a0
	ctx.r[4].s64 = ctx.r[10].s64 + 22176;
	// 826F9AB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F9ABC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9AC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F9AC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F9AC8: 386AB674  addi r3, r10, -0x498c
	ctx.r[3].s64 = ctx.r[10].s64 + -18828;
	// 826F9ACC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F9AD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F9AD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F9AD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F9ADC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F9AE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F9AE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F9AE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F9AEC: 4BD6D335  bl 0x82466e20
	ctx.lr = 0x826F9AF0;
	sub_82466E20(ctx, base);
	// 826F9AF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F9AF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F9AF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F9AFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F9B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F9B00 size=100
    let mut pc: u32 = 0x826F9B00;
    'dispatch: loop {
        match pc {
            0x826F9B00 => {
    //   block [0x826F9B00..0x826F9B64)
	// 826F9B00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F9B04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F9B08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F9B0C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F9B10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F9B14: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F9B18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F9B1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F9B20: 388A56B0  addi r4, r10, 0x56b0
	ctx.r[4].s64 = ctx.r[10].s64 + 22192;
	// 826F9B24: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9B28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F9B2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F9B30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F9B34: 386AB6A4  addi r3, r10, -0x495c
	ctx.r[3].s64 = ctx.r[10].s64 + -18780;
	// 826F9B38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F9B3C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F9B40: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F9B44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F9B48: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F9B4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F9B50: 4BD6D2D1  bl 0x82466e20
	ctx.lr = 0x826F9B54;
	sub_82466E20(ctx, base);
	// 826F9B54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F9B58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F9B5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F9B60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F9B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F9B68 size=112
    let mut pc: u32 = 0x826F9B68;
    'dispatch: loop {
        match pc {
            0x826F9B68 => {
    //   block [0x826F9B68..0x826F9BD8)
	// 826F9B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F9B6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F9B70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F9B74: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9B78: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F9B7C: 38AAB6A4  addi r5, r10, -0x495c
	ctx.r[5].s64 = ctx.r[10].s64 + -18780;
	// 826F9B80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F9B84: 390B0DA0  addi r8, r11, 0xda0
	ctx.r[8].s64 = ctx.r[11].s64 + 3488;
	// 826F9B88: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826F9B8C: 388A56C8  addi r4, r10, 0x56c8
	ctx.r[4].s64 = ctx.r[10].s64 + 22216;
	// 826F9B90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F9B94: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9B98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F9B9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F9BA0: 386AB6D4  addi r3, r10, -0x492c
	ctx.r[3].s64 = ctx.r[10].s64 + -18732;
	// 826F9BA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F9BA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F9BAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F9BB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F9BB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F9BB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F9BBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F9BC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F9BC4: 4BD6D25D  bl 0x82466e20
	ctx.lr = 0x826F9BC8;
	sub_82466E20(ctx, base);
	// 826F9BC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F9BCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F9BD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F9BD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F9BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F9BD8 size=112
    let mut pc: u32 = 0x826F9BD8;
    'dispatch: loop {
        match pc {
            0x826F9BD8 => {
    //   block [0x826F9BD8..0x826F9C48)
	// 826F9BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F9BDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F9BE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F9BE4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F9BE8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F9BEC: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F9BF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F9BF4: 390B0DE8  addi r8, r11, 0xde8
	ctx.r[8].s64 = ctx.r[11].s64 + 3560;
	// 826F9BF8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F9BFC: 388A56E8  addi r4, r10, 0x56e8
	ctx.r[4].s64 = ctx.r[10].s64 + 22248;
	// 826F9C00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F9C04: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9C08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F9C0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F9C10: 386AB704  addi r3, r10, -0x48fc
	ctx.r[3].s64 = ctx.r[10].s64 + -18684;
	// 826F9C14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F9C18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F9C1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F9C20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F9C24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F9C28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F9C2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F9C30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F9C34: 4BD6D1ED  bl 0x82466e20
	ctx.lr = 0x826F9C38;
	sub_82466E20(ctx, base);
	// 826F9C38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F9C3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F9C40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F9C44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F9C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F9C48 size=112
    let mut pc: u32 = 0x826F9C48;
    'dispatch: loop {
        match pc {
            0x826F9C48 => {
    //   block [0x826F9C48..0x826F9CB8)
	// 826F9C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F9C4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F9C50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F9C54: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F9C58: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F9C5C: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F9C60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F9C64: 390B0E00  addi r8, r11, 0xe00
	ctx.r[8].s64 = ctx.r[11].s64 + 3584;
	// 826F9C68: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F9C6C: 388A5700  addi r4, r10, 0x5700
	ctx.r[4].s64 = ctx.r[10].s64 + 22272;
	// 826F9C70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F9C74: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9C78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F9C7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F9C80: 386AB734  addi r3, r10, -0x48cc
	ctx.r[3].s64 = ctx.r[10].s64 + -18636;
	// 826F9C84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F9C88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F9C8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F9C90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F9C94: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F9C98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F9C9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F9CA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F9CA4: 4BD6D17D  bl 0x82466e20
	ctx.lr = 0x826F9CA8;
	sub_82466E20(ctx, base);
	// 826F9CA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F9CAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F9CB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F9CB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F9CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F9CB8 size=112
    let mut pc: u32 = 0x826F9CB8;
    'dispatch: loop {
        match pc {
            0x826F9CB8 => {
    //   block [0x826F9CB8..0x826F9D28)
	// 826F9CB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F9CBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F9CC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F9CC4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9CC8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F9CCC: 38AAB704  addi r5, r10, -0x48fc
	ctx.r[5].s64 = ctx.r[10].s64 + -18684;
	// 826F9CD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F9CD4: 390B0E18  addi r8, r11, 0xe18
	ctx.r[8].s64 = ctx.r[11].s64 + 3608;
	// 826F9CD8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826F9CDC: 388A571C  addi r4, r10, 0x571c
	ctx.r[4].s64 = ctx.r[10].s64 + 22300;
	// 826F9CE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F9CE4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9CE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F9CEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F9CF0: 386AB764  addi r3, r10, -0x489c
	ctx.r[3].s64 = ctx.r[10].s64 + -18588;
	// 826F9CF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F9CF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F9CFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F9D00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F9D04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F9D08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F9D0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F9D10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F9D14: 4BD6D10D  bl 0x82466e20
	ctx.lr = 0x826F9D18;
	sub_82466E20(ctx, base);
	// 826F9D18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F9D1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F9D20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F9D24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F9D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F9D28 size=72
    let mut pc: u32 = 0x826F9D28;
    'dispatch: loop {
        match pc {
            0x826F9D28 => {
    //   block [0x826F9D28..0x826F9D70)
	// 826F9D28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F9D2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F9D30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F9D34: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826F9D38: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 826F9D3C: 38CB93F0  addi r6, r11, -0x6c10
	ctx.r[6].s64 = ctx.r[11].s64 + -27664;
	// 826F9D40: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826F9D44: 388BAC70  addi r4, r11, -0x5390
	ctx.r[4].s64 = ctx.r[11].s64 + -21392;
	// 826F9D48: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 826F9D4C: 386BB794  addi r3, r11, -0x486c
	ctx.r[3].s64 = ctx.r[11].s64 + -18540;
	// 826F9D50: 4BD81D39  bl 0x8247ba88
	ctx.lr = 0x826F9D54;
	sub_8247BA88(ctx, base);
	// 826F9D54: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 826F9D58: 386BCF48  addi r3, r11, -0x30b8
	ctx.r[3].s64 = ctx.r[11].s64 + -12472;
	// 826F9D5C: 4BE38DDD  bl 0x82532b38
	ctx.lr = 0x826F9D60;
	sub_82532B38(ctx, base);
	// 826F9D60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826F9D64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F9D68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F9D6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F9D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F9D70 size=108
    let mut pc: u32 = 0x826F9D70;
    'dispatch: loop {
        match pc {
            0x826F9D70 => {
    //   block [0x826F9D70..0x826F9DDC)
	// 826F9D70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F9D74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F9D78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F9D7C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F9D80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F9D84: 38EB2B58  addi r7, r11, 0x2b58
	ctx.r[7].s64 = ctx.r[11].s64 + 11096;
	// 826F9D88: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826F9D8C: 388A2B24  addi r4, r10, 0x2b24
	ctx.r[4].s64 = ctx.r[10].s64 + 11044;
	// 826F9D90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F9D94: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9D98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F9D9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F9DA0: 386AB7AC  addi r3, r10, -0x4854
	ctx.r[3].s64 = ctx.r[10].s64 + -18516;
	// 826F9DA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F9DA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F9DAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F9DB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F9DB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F9DB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F9DBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F9DC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F9DC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F9DC8: 4BD6D059  bl 0x82466e20
	ctx.lr = 0x826F9DCC;
	sub_82466E20(ctx, base);
	// 826F9DCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F9DD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F9DD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F9DD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F9DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826F9DE0 size=24
    let mut pc: u32 = 0x826F9DE0;
    'dispatch: loop {
        match pc {
            0x826F9DE0 => {
    //   block [0x826F9DE0..0x826F9DF8)
	// 826F9DE0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F9DE4: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 826F9DE8: 394A8CF8  addi r10, r10, -0x7308
	ctx.r[10].s64 = ctx.r[10].s64 + -29448;
	// 826F9DEC: 816B2BD0  lwz r11, 0x2bd0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(11216 as u32) ) } as u64;
	// 826F9DF0: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 826F9DF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F9DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F9DF8 size=112
    let mut pc: u32 = 0x826F9DF8;
    'dispatch: loop {
        match pc {
            0x826F9DF8 => {
    //   block [0x826F9DF8..0x826F9E68)
	// 826F9DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F9DFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F9E00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F9E04: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826F9E08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F9E0C: 392BB384  addi r9, r11, -0x4c7c
	ctx.r[9].s64 = ctx.r[11].s64 + -19580;
	// 826F9E10: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 826F9E14: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826F9E18: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 826F9E1C: 388A2B3C  addi r4, r10, 0x2b3c
	ctx.r[4].s64 = ctx.r[10].s64 + 11068;
	// 826F9E20: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F9E24: 396B8CF8  addi r11, r11, -0x7308
	ctx.r[11].s64 = ctx.r[11].s64 + -29448;
	// 826F9E28: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826F9E2C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9E30: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826F9E34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F9E38: 386AB7DC  addi r3, r10, -0x4824
	ctx.r[3].s64 = ctx.r[10].s64 + -18468;
	// 826F9E3C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F9E40: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826F9E44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F9E48: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826F9E4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F9E50: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F9E54: 4BD6CFCD  bl 0x82466e20
	ctx.lr = 0x826F9E58;
	sub_82466E20(ctx, base);
	// 826F9E58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F9E5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F9E60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F9E64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F9E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F9E68 size=108
    let mut pc: u32 = 0x826F9E68;
    'dispatch: loop {
        match pc {
            0x826F9E68 => {
    //   block [0x826F9E68..0x826F9ED4)
	// 826F9E68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F9E6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F9E70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F9E74: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F9E78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F9E7C: 38EB2BD4  addi r7, r11, 0x2bd4
	ctx.r[7].s64 = ctx.r[11].s64 + 11220;
	// 826F9E80: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F9E84: 388A2B50  addi r4, r10, 0x2b50
	ctx.r[4].s64 = ctx.r[10].s64 + 11088;
	// 826F9E88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F9E8C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F9E90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F9E94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F9E98: 386AB80C  addi r3, r10, -0x47f4
	ctx.r[3].s64 = ctx.r[10].s64 + -18420;
	// 826F9E9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F9EA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F9EA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F9EA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F9EAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F9EB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F9EB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F9EB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F9EBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F9EC0: 4BD6CF61  bl 0x82466e20
	ctx.lr = 0x826F9EC4;
	sub_82466E20(ctx, base);
	// 826F9EC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F9EC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F9ECC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F9ED0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


