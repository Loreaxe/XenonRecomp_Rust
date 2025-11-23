pub fn sub_8328E290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328E290 size=64
    let mut pc: u32 = 0x8328E290;
    'dispatch: loop {
        match pc {
            0x8328E290 => {
    //   block [0x8328E290..0x8328E2D0)
	// 8328E290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328E294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328E298: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328E29C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8328E2A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328E2A4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8328E2A8: 386A4380  addi r3, r10, 0x4380
	ctx.r[3].s64 = ctx.r[10].s64 + 17280;
	// 8328E2AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328E2B0: 4AF9EC21  bl 0x8222ced0
	ctx.lr = 0x8328E2B4;
	sub_8222CED0(ctx, base);
	// 8328E2B4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328E2B8: 38694A18  addi r3, r9, 0x4a18
	ctx.r[3].s64 = ctx.r[9].s64 + 18968;
	// 8328E2BC: 4BA1BC65  bl 0x82ca9f20
	ctx.lr = 0x8328E2C0;
	sub_82CA9F20(ctx, base);
	// 8328E2C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328E2C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328E2C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328E2CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328E2D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328E2D0 size=64
    let mut pc: u32 = 0x8328E2D0;
    'dispatch: loop {
        match pc {
            0x8328E2D0 => {
    //   block [0x8328E2D0..0x8328E310)
	// 8328E2D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328E2D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328E2D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328E2DC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8328E2E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328E2E4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8328E2E8: 386A4384  addi r3, r10, 0x4384
	ctx.r[3].s64 = ctx.r[10].s64 + 17284;
	// 8328E2EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328E2F0: 4AF9EBE1  bl 0x8222ced0
	ctx.lr = 0x8328E2F4;
	sub_8222CED0(ctx, base);
	// 8328E2F4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328E2F8: 38694A28  addi r3, r9, 0x4a28
	ctx.r[3].s64 = ctx.r[9].s64 + 18984;
	// 8328E2FC: 4BA1BC25  bl 0x82ca9f20
	ctx.lr = 0x8328E300;
	sub_82CA9F20(ctx, base);
	// 8328E300: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328E304: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328E308: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328E30C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328E310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328E310 size=12
    let mut pc: u32 = 0x8328E310;
    'dispatch: loop {
        match pc {
            0x8328E310 => {
    //   block [0x8328E310..0x8328E31C)
	// 8328E310: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8328E314: 386B4A38  addi r3, r11, 0x4a38
	ctx.r[3].s64 = ctx.r[11].s64 + 19000;
	// 8328E318: 4BA1BC08  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328E320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328E320 size=12
    let mut pc: u32 = 0x8328E320;
    'dispatch: loop {
        match pc {
            0x8328E320 => {
    //   block [0x8328E320..0x8328E32C)
	// 8328E320: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8328E324: 386B4A48  addi r3, r11, 0x4a48
	ctx.r[3].s64 = ctx.r[11].s64 + 19016;
	// 8328E328: 4BA1BBF8  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328E330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328E330 size=12
    let mut pc: u32 = 0x8328E330;
    'dispatch: loop {
        match pc {
            0x8328E330 => {
    //   block [0x8328E330..0x8328E33C)
	// 8328E330: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8328E334: 386B4AA8  addi r3, r11, 0x4aa8
	ctx.r[3].s64 = ctx.r[11].s64 + 19112;
	// 8328E338: 4BA1BBE8  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328E340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328E340 size=64
    let mut pc: u32 = 0x8328E340;
    'dispatch: loop {
        match pc {
            0x8328E340 => {
    //   block [0x8328E340..0x8328E380)
	// 8328E340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328E344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328E348: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328E34C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8328E350: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328E354: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8328E358: 386A4390  addi r3, r10, 0x4390
	ctx.r[3].s64 = ctx.r[10].s64 + 17296;
	// 8328E35C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328E360: 4AF9EB71  bl 0x8222ced0
	ctx.lr = 0x8328E364;
	sub_8222CED0(ctx, base);
	// 8328E364: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328E368: 38694B08  addi r3, r9, 0x4b08
	ctx.r[3].s64 = ctx.r[9].s64 + 19208;
	// 8328E36C: 4BA1BBB5  bl 0x82ca9f20
	ctx.lr = 0x8328E370;
	sub_82CA9F20(ctx, base);
	// 8328E370: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328E374: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328E378: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328E37C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328E380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328E380 size=64
    let mut pc: u32 = 0x8328E380;
    'dispatch: loop {
        match pc {
            0x8328E380 => {
    //   block [0x8328E380..0x8328E3C0)
	// 8328E380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328E384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328E388: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328E38C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8328E390: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328E394: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8328E398: 386A4394  addi r3, r10, 0x4394
	ctx.r[3].s64 = ctx.r[10].s64 + 17300;
	// 8328E39C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328E3A0: 4AF9EB31  bl 0x8222ced0
	ctx.lr = 0x8328E3A4;
	sub_8222CED0(ctx, base);
	// 8328E3A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328E3A8: 38694B18  addi r3, r9, 0x4b18
	ctx.r[3].s64 = ctx.r[9].s64 + 19224;
	// 8328E3AC: 4BA1BB75  bl 0x82ca9f20
	ctx.lr = 0x8328E3B0;
	sub_82CA9F20(ctx, base);
	// 8328E3B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328E3B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328E3B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328E3BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328E3C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328E3C0 size=64
    let mut pc: u32 = 0x8328E3C0;
    'dispatch: loop {
        match pc {
            0x8328E3C0 => {
    //   block [0x8328E3C0..0x8328E400)
	// 8328E3C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328E3C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328E3C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328E3CC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8328E3D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328E3D4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8328E3D8: 386A4398  addi r3, r10, 0x4398
	ctx.r[3].s64 = ctx.r[10].s64 + 17304;
	// 8328E3DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328E3E0: 4AF9EAF1  bl 0x8222ced0
	ctx.lr = 0x8328E3E4;
	sub_8222CED0(ctx, base);
	// 8328E3E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328E3E8: 38694B38  addi r3, r9, 0x4b38
	ctx.r[3].s64 = ctx.r[9].s64 + 19256;
	// 8328E3EC: 4BA1BB35  bl 0x82ca9f20
	ctx.lr = 0x8328E3F0;
	sub_82CA9F20(ctx, base);
	// 8328E3F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328E3F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328E3F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328E3FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328E400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328E400 size=64
    let mut pc: u32 = 0x8328E400;
    'dispatch: loop {
        match pc {
            0x8328E400 => {
    //   block [0x8328E400..0x8328E440)
	// 8328E400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328E404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328E408: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328E40C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8328E410: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328E414: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8328E418: 386A439C  addi r3, r10, 0x439c
	ctx.r[3].s64 = ctx.r[10].s64 + 17308;
	// 8328E41C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328E420: 4AF9EAB1  bl 0x8222ced0
	ctx.lr = 0x8328E424;
	sub_8222CED0(ctx, base);
	// 8328E424: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328E428: 38694B48  addi r3, r9, 0x4b48
	ctx.r[3].s64 = ctx.r[9].s64 + 19272;
	// 8328E42C: 4BA1BAF5  bl 0x82ca9f20
	ctx.lr = 0x8328E430;
	sub_82CA9F20(ctx, base);
	// 8328E430: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328E434: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328E438: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328E43C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328E440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328E440 size=64
    let mut pc: u32 = 0x8328E440;
    'dispatch: loop {
        match pc {
            0x8328E440 => {
    //   block [0x8328E440..0x8328E480)
	// 8328E440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328E444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328E448: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328E44C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8328E450: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328E454: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8328E458: 386A43A0  addi r3, r10, 0x43a0
	ctx.r[3].s64 = ctx.r[10].s64 + 17312;
	// 8328E45C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328E460: 4AF9EA71  bl 0x8222ced0
	ctx.lr = 0x8328E464;
	sub_8222CED0(ctx, base);
	// 8328E464: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328E468: 38694B58  addi r3, r9, 0x4b58
	ctx.r[3].s64 = ctx.r[9].s64 + 19288;
	// 8328E46C: 4BA1BAB5  bl 0x82ca9f20
	ctx.lr = 0x8328E470;
	sub_82CA9F20(ctx, base);
	// 8328E470: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328E474: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328E478: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328E47C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328E480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328E480 size=64
    let mut pc: u32 = 0x8328E480;
    'dispatch: loop {
        match pc {
            0x8328E480 => {
    //   block [0x8328E480..0x8328E4C0)
	// 8328E480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328E484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328E488: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328E48C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8328E490: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328E494: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8328E498: 386A43A4  addi r3, r10, 0x43a4
	ctx.r[3].s64 = ctx.r[10].s64 + 17316;
	// 8328E49C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328E4A0: 4AF9EA31  bl 0x8222ced0
	ctx.lr = 0x8328E4A4;
	sub_8222CED0(ctx, base);
	// 8328E4A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328E4A8: 38694B68  addi r3, r9, 0x4b68
	ctx.r[3].s64 = ctx.r[9].s64 + 19304;
	// 8328E4AC: 4BA1BA75  bl 0x82ca9f20
	ctx.lr = 0x8328E4B0;
	sub_82CA9F20(ctx, base);
	// 8328E4B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328E4B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328E4B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328E4BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328E4C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328E4C0 size=64
    let mut pc: u32 = 0x8328E4C0;
    'dispatch: loop {
        match pc {
            0x8328E4C0 => {
    //   block [0x8328E4C0..0x8328E500)
	// 8328E4C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328E4C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328E4C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328E4CC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8328E4D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328E4D4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8328E4D8: 386A43A8  addi r3, r10, 0x43a8
	ctx.r[3].s64 = ctx.r[10].s64 + 17320;
	// 8328E4DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328E4E0: 4AF9E9F1  bl 0x8222ced0
	ctx.lr = 0x8328E4E4;
	sub_8222CED0(ctx, base);
	// 8328E4E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328E4E8: 38694B78  addi r3, r9, 0x4b78
	ctx.r[3].s64 = ctx.r[9].s64 + 19320;
	// 8328E4EC: 4BA1BA35  bl 0x82ca9f20
	ctx.lr = 0x8328E4F0;
	sub_82CA9F20(ctx, base);
	// 8328E4F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328E4F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328E4F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328E4FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328E500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328E500 size=64
    let mut pc: u32 = 0x8328E500;
    'dispatch: loop {
        match pc {
            0x8328E500 => {
    //   block [0x8328E500..0x8328E540)
	// 8328E500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328E504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328E508: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328E50C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8328E510: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328E514: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8328E518: 386A43AC  addi r3, r10, 0x43ac
	ctx.r[3].s64 = ctx.r[10].s64 + 17324;
	// 8328E51C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328E520: 4AF9E9B1  bl 0x8222ced0
	ctx.lr = 0x8328E524;
	sub_8222CED0(ctx, base);
	// 8328E524: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328E528: 38694B88  addi r3, r9, 0x4b88
	ctx.r[3].s64 = ctx.r[9].s64 + 19336;
	// 8328E52C: 4BA1B9F5  bl 0x82ca9f20
	ctx.lr = 0x8328E530;
	sub_82CA9F20(ctx, base);
	// 8328E530: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328E534: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328E538: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328E53C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328E540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328E540 size=64
    let mut pc: u32 = 0x8328E540;
    'dispatch: loop {
        match pc {
            0x8328E540 => {
    //   block [0x8328E540..0x8328E580)
	// 8328E540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328E544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328E548: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328E54C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8328E550: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328E554: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8328E558: 386A43B0  addi r3, r10, 0x43b0
	ctx.r[3].s64 = ctx.r[10].s64 + 17328;
	// 8328E55C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328E560: 4AF9E971  bl 0x8222ced0
	ctx.lr = 0x8328E564;
	sub_8222CED0(ctx, base);
	// 8328E564: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328E568: 38694B98  addi r3, r9, 0x4b98
	ctx.r[3].s64 = ctx.r[9].s64 + 19352;
	// 8328E56C: 4BA1B9B5  bl 0x82ca9f20
	ctx.lr = 0x8328E570;
	sub_82CA9F20(ctx, base);
	// 8328E570: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328E574: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328E578: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328E57C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328E580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328E580 size=64
    let mut pc: u32 = 0x8328E580;
    'dispatch: loop {
        match pc {
            0x8328E580 => {
    //   block [0x8328E580..0x8328E5C0)
	// 8328E580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328E584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328E588: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328E58C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8328E590: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328E594: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8328E598: 386A43B4  addi r3, r10, 0x43b4
	ctx.r[3].s64 = ctx.r[10].s64 + 17332;
	// 8328E59C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328E5A0: 4AF9E931  bl 0x8222ced0
	ctx.lr = 0x8328E5A4;
	sub_8222CED0(ctx, base);
	// 8328E5A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328E5A8: 38694BA8  addi r3, r9, 0x4ba8
	ctx.r[3].s64 = ctx.r[9].s64 + 19368;
	// 8328E5AC: 4BA1B975  bl 0x82ca9f20
	ctx.lr = 0x8328E5B0;
	sub_82CA9F20(ctx, base);
	// 8328E5B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328E5B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328E5B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328E5BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328E5C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328E5C0 size=64
    let mut pc: u32 = 0x8328E5C0;
    'dispatch: loop {
        match pc {
            0x8328E5C0 => {
    //   block [0x8328E5C0..0x8328E600)
	// 8328E5C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328E5C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328E5C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328E5CC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8328E5D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328E5D4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8328E5D8: 386A43B8  addi r3, r10, 0x43b8
	ctx.r[3].s64 = ctx.r[10].s64 + 17336;
	// 8328E5DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328E5E0: 4AF9E8F1  bl 0x8222ced0
	ctx.lr = 0x8328E5E4;
	sub_8222CED0(ctx, base);
	// 8328E5E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328E5E8: 38694BB8  addi r3, r9, 0x4bb8
	ctx.r[3].s64 = ctx.r[9].s64 + 19384;
	// 8328E5EC: 4BA1B935  bl 0x82ca9f20
	ctx.lr = 0x8328E5F0;
	sub_82CA9F20(ctx, base);
	// 8328E5F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328E5F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328E5F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328E5FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328E600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328E600 size=64
    let mut pc: u32 = 0x8328E600;
    'dispatch: loop {
        match pc {
            0x8328E600 => {
    //   block [0x8328E600..0x8328E640)
	// 8328E600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328E604: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328E608: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328E60C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8328E610: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328E614: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8328E618: 386A43BC  addi r3, r10, 0x43bc
	ctx.r[3].s64 = ctx.r[10].s64 + 17340;
	// 8328E61C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328E620: 4AF9E8B1  bl 0x8222ced0
	ctx.lr = 0x8328E624;
	sub_8222CED0(ctx, base);
	// 8328E624: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328E628: 38694BC8  addi r3, r9, 0x4bc8
	ctx.r[3].s64 = ctx.r[9].s64 + 19400;
	// 8328E62C: 4BA1B8F5  bl 0x82ca9f20
	ctx.lr = 0x8328E630;
	sub_82CA9F20(ctx, base);
	// 8328E630: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328E634: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328E638: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328E63C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328E640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328E640 size=64
    let mut pc: u32 = 0x8328E640;
    'dispatch: loop {
        match pc {
            0x8328E640 => {
    //   block [0x8328E640..0x8328E680)
	// 8328E640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328E644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328E648: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328E64C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8328E650: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328E654: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8328E658: 386A43C0  addi r3, r10, 0x43c0
	ctx.r[3].s64 = ctx.r[10].s64 + 17344;
	// 8328E65C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328E660: 4AF9E871  bl 0x8222ced0
	ctx.lr = 0x8328E664;
	sub_8222CED0(ctx, base);
	// 8328E664: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328E668: 38694BD8  addi r3, r9, 0x4bd8
	ctx.r[3].s64 = ctx.r[9].s64 + 19416;
	// 8328E66C: 4BA1B8B5  bl 0x82ca9f20
	ctx.lr = 0x8328E670;
	sub_82CA9F20(ctx, base);
	// 8328E670: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328E674: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328E678: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328E67C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328E680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328E680 size=64
    let mut pc: u32 = 0x8328E680;
    'dispatch: loop {
        match pc {
            0x8328E680 => {
    //   block [0x8328E680..0x8328E6C0)
	// 8328E680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328E684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328E688: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328E68C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8328E690: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328E694: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8328E698: 386A43C4  addi r3, r10, 0x43c4
	ctx.r[3].s64 = ctx.r[10].s64 + 17348;
	// 8328E69C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328E6A0: 4AF9E831  bl 0x8222ced0
	ctx.lr = 0x8328E6A4;
	sub_8222CED0(ctx, base);
	// 8328E6A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328E6A8: 38694BE8  addi r3, r9, 0x4be8
	ctx.r[3].s64 = ctx.r[9].s64 + 19432;
	// 8328E6AC: 4BA1B875  bl 0x82ca9f20
	ctx.lr = 0x8328E6B0;
	sub_82CA9F20(ctx, base);
	// 8328E6B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328E6B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328E6B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328E6BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328E6C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328E6C0 size=64
    let mut pc: u32 = 0x8328E6C0;
    'dispatch: loop {
        match pc {
            0x8328E6C0 => {
    //   block [0x8328E6C0..0x8328E700)
	// 8328E6C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328E6C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328E6C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328E6CC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8328E6D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328E6D4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8328E6D8: 386A43C8  addi r3, r10, 0x43c8
	ctx.r[3].s64 = ctx.r[10].s64 + 17352;
	// 8328E6DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328E6E0: 4AF9E7F1  bl 0x8222ced0
	ctx.lr = 0x8328E6E4;
	sub_8222CED0(ctx, base);
	// 8328E6E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328E6E8: 38694BF8  addi r3, r9, 0x4bf8
	ctx.r[3].s64 = ctx.r[9].s64 + 19448;
	// 8328E6EC: 4BA1B835  bl 0x82ca9f20
	ctx.lr = 0x8328E6F0;
	sub_82CA9F20(ctx, base);
	// 8328E6F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328E6F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328E6F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328E6FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328E700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328E700 size=64
    let mut pc: u32 = 0x8328E700;
    'dispatch: loop {
        match pc {
            0x8328E700 => {
    //   block [0x8328E700..0x8328E740)
	// 8328E700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328E704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328E708: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328E70C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8328E710: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328E714: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8328E718: 386A43CC  addi r3, r10, 0x43cc
	ctx.r[3].s64 = ctx.r[10].s64 + 17356;
	// 8328E71C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328E720: 4AF9E7B1  bl 0x8222ced0
	ctx.lr = 0x8328E724;
	sub_8222CED0(ctx, base);
	// 8328E724: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328E728: 38694C08  addi r3, r9, 0x4c08
	ctx.r[3].s64 = ctx.r[9].s64 + 19464;
	// 8328E72C: 4BA1B7F5  bl 0x82ca9f20
	ctx.lr = 0x8328E730;
	sub_82CA9F20(ctx, base);
	// 8328E730: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328E734: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328E738: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328E73C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328E740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328E740 size=64
    let mut pc: u32 = 0x8328E740;
    'dispatch: loop {
        match pc {
            0x8328E740 => {
    //   block [0x8328E740..0x8328E780)
	// 8328E740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328E744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328E748: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328E74C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8328E750: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328E754: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8328E758: 386A43D0  addi r3, r10, 0x43d0
	ctx.r[3].s64 = ctx.r[10].s64 + 17360;
	// 8328E75C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328E760: 4AF9E771  bl 0x8222ced0
	ctx.lr = 0x8328E764;
	sub_8222CED0(ctx, base);
	// 8328E764: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328E768: 38694C18  addi r3, r9, 0x4c18
	ctx.r[3].s64 = ctx.r[9].s64 + 19480;
	// 8328E76C: 4BA1B7B5  bl 0x82ca9f20
	ctx.lr = 0x8328E770;
	sub_82CA9F20(ctx, base);
	// 8328E770: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328E774: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328E778: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328E77C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328E780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328E780 size=64
    let mut pc: u32 = 0x8328E780;
    'dispatch: loop {
        match pc {
            0x8328E780 => {
    //   block [0x8328E780..0x8328E7C0)
	// 8328E780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328E784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328E788: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328E78C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8328E790: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328E794: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8328E798: 386A43D4  addi r3, r10, 0x43d4
	ctx.r[3].s64 = ctx.r[10].s64 + 17364;
	// 8328E79C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328E7A0: 4AF9E731  bl 0x8222ced0
	ctx.lr = 0x8328E7A4;
	sub_8222CED0(ctx, base);
	// 8328E7A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328E7A8: 38694C28  addi r3, r9, 0x4c28
	ctx.r[3].s64 = ctx.r[9].s64 + 19496;
	// 8328E7AC: 4BA1B775  bl 0x82ca9f20
	ctx.lr = 0x8328E7B0;
	sub_82CA9F20(ctx, base);
	// 8328E7B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328E7B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328E7B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328E7BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328E7C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328E7C0 size=40
    let mut pc: u32 = 0x8328E7C0;
    'dispatch: loop {
        match pc {
            0x8328E7C0 => {
    //   block [0x8328E7C0..0x8328E7E8)
	// 8328E7C0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8328E7C4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8328E7C8: 396B43D8  addi r11, r11, 0x43d8
	ctx.r[11].s64 = ctx.r[11].s64 + 17368;
	// 8328E7CC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328E7D0: 38694C38  addi r3, r9, 0x4c38
	ctx.r[3].s64 = ctx.r[9].s64 + 19512;
	// 8328E7D4: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8328E7D8: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8328E7DC: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8328E7E0: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8328E7E4: 4BA1B73C  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328E7E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328E7E8 size=64
    let mut pc: u32 = 0x8328E7E8;
    'dispatch: loop {
        match pc {
            0x8328E7E8 => {
    //   block [0x8328E7E8..0x8328E828)
	// 8328E7E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328E7EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328E7F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328E7F4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8328E7F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328E7FC: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8328E800: 386A43EC  addi r3, r10, 0x43ec
	ctx.r[3].s64 = ctx.r[10].s64 + 17388;
	// 8328E804: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328E808: 4AF9E6C9  bl 0x8222ced0
	ctx.lr = 0x8328E80C;
	sub_8222CED0(ctx, base);
	// 8328E80C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328E810: 38694C50  addi r3, r9, 0x4c50
	ctx.r[3].s64 = ctx.r[9].s64 + 19536;
	// 8328E814: 4BA1B70D  bl 0x82ca9f20
	ctx.lr = 0x8328E818;
	sub_82CA9F20(ctx, base);
	// 8328E818: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328E81C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328E820: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328E824: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328E828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328E828 size=64
    let mut pc: u32 = 0x8328E828;
    'dispatch: loop {
        match pc {
            0x8328E828 => {
    //   block [0x8328E828..0x8328E868)
	// 8328E828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328E82C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328E830: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328E834: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8328E838: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328E83C: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8328E840: 386A43F0  addi r3, r10, 0x43f0
	ctx.r[3].s64 = ctx.r[10].s64 + 17392;
	// 8328E844: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328E848: 4AF9E689  bl 0x8222ced0
	ctx.lr = 0x8328E84C;
	sub_8222CED0(ctx, base);
	// 8328E84C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328E850: 38694C60  addi r3, r9, 0x4c60
	ctx.r[3].s64 = ctx.r[9].s64 + 19552;
	// 8328E854: 4BA1B6CD  bl 0x82ca9f20
	ctx.lr = 0x8328E858;
	sub_82CA9F20(ctx, base);
	// 8328E858: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328E85C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328E860: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328E864: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328E868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328E868 size=120
    let mut pc: u32 = 0x8328E868;
    'dispatch: loop {
        match pc {
            0x8328E868 => {
    //   block [0x8328E868..0x8328E8A4)
	// 8328E868: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8328E86C: 3CE0820F  lis r7, -0x7df1
	ctx.r[7].s64 = -2112946176;
	// 8328E870: 396B4440  addi r11, r11, 0x4440
	ctx.r[11].s64 = ctx.r[11].s64 + 17472;
	// 8328E874: 3CC0820F  lis r6, -0x7df1
	ctx.r[6].s64 = -2112946176;
	// 8328E878: 3D208210  lis r9, -0x7df0
	ctx.r[9].s64 = -2112880640;
	// 8328E87C: 3CA0820F  lis r5, -0x7df1
	ctx.r[5].s64 = -2112946176;
	// 8328E880: 3C80820F  lis r4, -0x7df1
	ctx.r[4].s64 = -2112946176;
	// 8328E884: 39000016  li r8, 0x16
	ctx.r[8].s64 = 22;
	// 8328E888: 396B001C  addi r11, r11, 0x1c
	ctx.r[11].s64 = ctx.r[11].s64 + 28;
	// 8328E88C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8328E890: 38E76B78  addi r7, r7, 0x6b78
	ctx.r[7].s64 = ctx.r[7].s64 + 27512;
	// 8328E894: 38C66B74  addi r6, r6, 0x6b74
	ctx.r[6].s64 = ctx.r[6].s64 + 27508;
	// 8328E898: 3929B00C  addi r9, r9, -0x4ff4
	ctx.r[9].s64 = ctx.r[9].s64 + -20468;
	// 8328E89C: 38A56B68  addi r5, r5, 0x6b68
	ctx.r[5].s64 = ctx.r[5].s64 + 27496;
	// 8328E8A0: 38846B58  addi r4, r4, 0x6b58
	ctx.r[4].s64 = ctx.r[4].s64 + 27480;
	pc = 0x8328E8A4; continue 'dispatch;
            }
            0x8328E8A4 => {
    //   block [0x8328E8A4..0x8328E8E0)
	// 8328E8A4: 914BFFE8  stw r10, -0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-24 as u32), ctx.r[10].u32 ) };
	// 8328E8A8: 3508FFFF  addic. r8, r8, -1
	ctx.xer.ca = (ctx.r[8].u32 > (!(-1 as u32)));
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 8328E8AC: 912BFFEC  stw r9, -0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-20 as u32), ctx.r[9].u32 ) };
	// 8328E8B0: 908BFFE4  stw r4, -0x1c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-28 as u32), ctx.r[4].u32 ) };
	// 8328E8B4: 90ABFFEC  stw r5, -0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-20 as u32), ctx.r[5].u32 ) };
	// 8328E8B8: 914BFFF8  stw r10, -8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-8 as u32), ctx.r[10].u32 ) };
	// 8328E8BC: 914BFFFC  stw r10, -4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[10].u32 ) };
	// 8328E8C0: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8328E8C4: 90CBFFF4  stw r6, -0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-12 as u32), ctx.r[6].u32 ) };
	// 8328E8C8: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 8328E8CC: 396B0060  addi r11, r11, 0x60
	ctx.r[11].s64 = ctx.r[11].s64 + 96;
	// 8328E8D0: 4080FFD4  bge 0x8328e8a4
	if !ctx.cr[0].lt {
	pc = 0x8328E8A4; continue 'dispatch;
	}
	// 8328E8D4: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8328E8D8: 386B4C70  addi r3, r11, 0x4c70
	ctx.r[3].s64 = ctx.r[11].s64 + 19568;
	// 8328E8DC: 4BA1B644  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328E8E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328E8E0 size=76
    let mut pc: u32 = 0x8328E8E0;
    'dispatch: loop {
        match pc {
            0x8328E8E0 => {
    //   block [0x8328E8E0..0x8328E92C)
	// 8328E8E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328E8E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328E8E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328E8EC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8328E8F0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328E8F4: 388B73E8  addi r4, r11, 0x73e8
	ctx.r[4].s64 = ctx.r[11].s64 + 29672;
	// 8328E8F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8328E8FC: 4AF9E5D5  bl 0x8222ced0
	ctx.lr = 0x8328E900;
	sub_8222CED0(ctx, base);
	// 8328E900: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328E904: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8328E908: 386A43F4  addi r3, r10, 0x43f4
	ctx.r[3].s64 = ctx.r[10].s64 + 17396;
	// 8328E90C: 4B01CBFD  bl 0x822ab508
	ctx.lr = 0x8328E910;
	sub_822AB508(ctx, base);
	// 8328E910: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328E914: 38694D20  addi r3, r9, 0x4d20
	ctx.r[3].s64 = ctx.r[9].s64 + 19744;
	// 8328E918: 4BA1B609  bl 0x82ca9f20
	ctx.lr = 0x8328E91C;
	sub_82CA9F20(ctx, base);
	// 8328E91C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328E920: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328E924: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328E928: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328E930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328E930 size=12
    let mut pc: u32 = 0x8328E930;
    'dispatch: loop {
        match pc {
            0x8328E930 => {
    //   block [0x8328E930..0x8328E93C)
	// 8328E930: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8328E934: 386B4D30  addi r3, r11, 0x4d30
	ctx.r[3].s64 = ctx.r[11].s64 + 19760;
	// 8328E938: 4BA1B5E8  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328E940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328E940 size=12
    let mut pc: u32 = 0x8328E940;
    'dispatch: loop {
        match pc {
            0x8328E940 => {
    //   block [0x8328E940..0x8328E94C)
	// 8328E940: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8328E944: 386B4DA8  addi r3, r11, 0x4da8
	ctx.r[3].s64 = ctx.r[11].s64 + 19880;
	// 8328E948: 4BA1B5D8  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328E950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328E950 size=64
    let mut pc: u32 = 0x8328E950;
    'dispatch: loop {
        match pc {
            0x8328E950 => {
    //   block [0x8328E950..0x8328E990)
	// 8328E950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328E954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328E958: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328E95C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8328E960: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328E964: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8328E968: 386A4D30  addi r3, r10, 0x4d30
	ctx.r[3].s64 = ctx.r[10].s64 + 19760;
	// 8328E96C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328E970: 4AF9E561  bl 0x8222ced0
	ctx.lr = 0x8328E974;
	sub_8222CED0(ctx, base);
	// 8328E974: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328E978: 38694DB8  addi r3, r9, 0x4db8
	ctx.r[3].s64 = ctx.r[9].s64 + 19896;
	// 8328E97C: 4BA1B5A5  bl 0x82ca9f20
	ctx.lr = 0x8328E980;
	sub_82CA9F20(ctx, base);
	// 8328E980: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328E984: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328E988: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328E98C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328E990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328E990 size=64
    let mut pc: u32 = 0x8328E990;
    'dispatch: loop {
        match pc {
            0x8328E990 => {
    //   block [0x8328E990..0x8328E9D0)
	// 8328E990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328E994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328E998: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328E99C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8328E9A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328E9A4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8328E9A8: 386A4D34  addi r3, r10, 0x4d34
	ctx.r[3].s64 = ctx.r[10].s64 + 19764;
	// 8328E9AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328E9B0: 4AF9E521  bl 0x8222ced0
	ctx.lr = 0x8328E9B4;
	sub_8222CED0(ctx, base);
	// 8328E9B4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328E9B8: 38694DC8  addi r3, r9, 0x4dc8
	ctx.r[3].s64 = ctx.r[9].s64 + 19912;
	// 8328E9BC: 4BA1B565  bl 0x82ca9f20
	ctx.lr = 0x8328E9C0;
	sub_82CA9F20(ctx, base);
	// 8328E9C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328E9C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328E9C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328E9CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328E9D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8328E9D0 size=88
    let mut pc: u32 = 0x8328E9D0;
    'dispatch: loop {
        match pc {
            0x8328E9D0 => {
    //   block [0x8328E9D0..0x8328EA28)
	// 8328E9D0: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8328E9D4: 3941FFF0  addi r10, r1, -0x10
	ctx.r[10].s64 = ctx.r[1].s64 + -16;
	// 8328E9D8: 3921FFF4  addi r9, r1, -0xc
	ctx.r[9].s64 = ctx.r[1].s64 + -12;
	// 8328E9DC: 3901FFF4  addi r8, r1, -0xc
	ctx.r[8].s64 = ctx.r[1].s64 + -12;
	// 8328E9E0: 38E1FFF0  addi r7, r1, -0x10
	ctx.r[7].s64 = ctx.r[1].s64 + -16;
	// 8328E9E4: C00B9490  lfs f0, -0x6b70(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8328E9E8: 3CC0834A  lis r6, -0x7cb6
	ctx.r[6].s64 = -2092302336;
	// 8328E9EC: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328EA28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8328EA28 size=96
    let mut pc: u32 = 0x8328EA28;
    'dispatch: loop {
        match pc {
            0x8328EA28 => {
    //   block [0x8328EA28..0x8328EA88)
	// 8328EA28: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8328EA2C: 3901FFF0  addi r8, r1, -0x10
	ctx.r[8].s64 = ctx.r[1].s64 + -16;
	// 8328EA30: 392B9484  addi r9, r11, -0x6b7c
	ctx.r[9].s64 = ctx.r[11].s64 + -27516;
	// 8328EA34: 3941FFF4  addi r10, r1, -0xc
	ctx.r[10].s64 = ctx.r[1].s64 + -12;
	// 8328EA38: 38E1FFF4  addi r7, r1, -0xc
	ctx.r[7].s64 = ctx.r[1].s64 + -12;
	// 8328EA3C: C00B9484  lfs f0, -0x6b7c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8328EA40: 38C1FFF0  addi r6, r1, -0x10
	ctx.r[6].s64 = ctx.r[1].s64 + -16;
	// 8328EA44: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 8328EA48: 3CA0834A  lis r5, -0x7cb6
	ctx.r[5].s64 = -2092302336;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328EA88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328EA88 size=12
    let mut pc: u32 = 0x8328EA88;
    'dispatch: loop {
        match pc {
            0x8328EA88 => {
    //   block [0x8328EA88..0x8328EA94)
	// 8328EA88: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8328EA8C: 386B4DD8  addi r3, r11, 0x4dd8
	ctx.r[3].s64 = ctx.r[11].s64 + 19928;
	// 8328EA90: 4BA1B490  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328EA98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328EA98 size=12
    let mut pc: u32 = 0x8328EA98;
    'dispatch: loop {
        match pc {
            0x8328EA98 => {
    //   block [0x8328EA98..0x8328EAA4)
	// 8328EA98: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8328EA9C: 386B4E38  addi r3, r11, 0x4e38
	ctx.r[3].s64 = ctx.r[11].s64 + 20024;
	// 8328EAA0: 4BA1B480  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328EAA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328EAA8 size=12
    let mut pc: u32 = 0x8328EAA8;
    'dispatch: loop {
        match pc {
            0x8328EAA8 => {
    //   block [0x8328EAA8..0x8328EAB4)
	// 8328EAA8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8328EAAC: 386B4E98  addi r3, r11, 0x4e98
	ctx.r[3].s64 = ctx.r[11].s64 + 20120;
	// 8328EAB0: 4BA1B470  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328EAB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328EAB8 size=64
    let mut pc: u32 = 0x8328EAB8;
    'dispatch: loop {
        match pc {
            0x8328EAB8 => {
    //   block [0x8328EAB8..0x8328EAF8)
	// 8328EAB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328EABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328EAC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328EAC4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8328EAC8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328EACC: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8328EAD0: 386A4DAC  addi r3, r10, 0x4dac
	ctx.r[3].s64 = ctx.r[10].s64 + 19884;
	// 8328EAD4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328EAD8: 4AF9E3F9  bl 0x8222ced0
	ctx.lr = 0x8328EADC;
	sub_8222CED0(ctx, base);
	// 8328EADC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328EAE0: 38694EA8  addi r3, r9, 0x4ea8
	ctx.r[3].s64 = ctx.r[9].s64 + 20136;
	// 8328EAE4: 4BA1B43D  bl 0x82ca9f20
	ctx.lr = 0x8328EAE8;
	sub_82CA9F20(ctx, base);
	// 8328EAE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328EAEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328EAF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328EAF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328EAF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328EAF8 size=64
    let mut pc: u32 = 0x8328EAF8;
    'dispatch: loop {
        match pc {
            0x8328EAF8 => {
    //   block [0x8328EAF8..0x8328EB38)
	// 8328EAF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328EAFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328EB00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328EB04: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8328EB08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328EB0C: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8328EB10: 386A4DB0  addi r3, r10, 0x4db0
	ctx.r[3].s64 = ctx.r[10].s64 + 19888;
	// 8328EB14: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328EB18: 4AF9E3B9  bl 0x8222ced0
	ctx.lr = 0x8328EB1C;
	sub_8222CED0(ctx, base);
	// 8328EB1C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328EB20: 38694EB8  addi r3, r9, 0x4eb8
	ctx.r[3].s64 = ctx.r[9].s64 + 20152;
	// 8328EB24: 4BA1B3FD  bl 0x82ca9f20
	ctx.lr = 0x8328EB28;
	sub_82CA9F20(ctx, base);
	// 8328EB28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328EB2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328EB30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328EB34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328EB38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328EB38 size=64
    let mut pc: u32 = 0x8328EB38;
    'dispatch: loop {
        match pc {
            0x8328EB38 => {
    //   block [0x8328EB38..0x8328EB78)
	// 8328EB38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328EB3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328EB40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328EB44: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8328EB48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328EB4C: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8328EB50: 386A4DB4  addi r3, r10, 0x4db4
	ctx.r[3].s64 = ctx.r[10].s64 + 19892;
	// 8328EB54: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328EB58: 4AF9E379  bl 0x8222ced0
	ctx.lr = 0x8328EB5C;
	sub_8222CED0(ctx, base);
	// 8328EB5C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328EB60: 38694EC8  addi r3, r9, 0x4ec8
	ctx.r[3].s64 = ctx.r[9].s64 + 20168;
	// 8328EB64: 4BA1B3BD  bl 0x82ca9f20
	ctx.lr = 0x8328EB68;
	sub_82CA9F20(ctx, base);
	// 8328EB68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328EB6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328EB70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328EB74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328EB78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328EB78 size=64
    let mut pc: u32 = 0x8328EB78;
    'dispatch: loop {
        match pc {
            0x8328EB78 => {
    //   block [0x8328EB78..0x8328EBB8)
	// 8328EB78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328EB7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328EB80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328EB84: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8328EB88: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328EB8C: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8328EB90: 386A4DB8  addi r3, r10, 0x4db8
	ctx.r[3].s64 = ctx.r[10].s64 + 19896;
	// 8328EB94: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328EB98: 4AF9E339  bl 0x8222ced0
	ctx.lr = 0x8328EB9C;
	sub_8222CED0(ctx, base);
	// 8328EB9C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328EBA0: 38694ED8  addi r3, r9, 0x4ed8
	ctx.r[3].s64 = ctx.r[9].s64 + 20184;
	// 8328EBA4: 4BA1B37D  bl 0x82ca9f20
	ctx.lr = 0x8328EBA8;
	sub_82CA9F20(ctx, base);
	// 8328EBA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328EBAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328EBB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328EBB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328EBB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328EBB8 size=40
    let mut pc: u32 = 0x8328EBB8;
    'dispatch: loop {
        match pc {
            0x8328EBB8 => {
    //   block [0x8328EBB8..0x8328EBE0)
	// 8328EBB8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8328EBBC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8328EBC0: 396B4DBC  addi r11, r11, 0x4dbc
	ctx.r[11].s64 = ctx.r[11].s64 + 19900;
	// 8328EBC4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328EBC8: 38694EE8  addi r3, r9, 0x4ee8
	ctx.r[3].s64 = ctx.r[9].s64 + 20200;
	// 8328EBCC: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8328EBD0: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8328EBD4: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8328EBD8: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8328EBDC: 4BA1B344  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328EBE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328EBE0 size=40
    let mut pc: u32 = 0x8328EBE0;
    'dispatch: loop {
        match pc {
            0x8328EBE0 => {
    //   block [0x8328EBE0..0x8328EC08)
	// 8328EBE0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8328EBE4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8328EBE8: 396B4DD0  addi r11, r11, 0x4dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 19920;
	// 8328EBEC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328EBF0: 38694F00  addi r3, r9, 0x4f00
	ctx.r[3].s64 = ctx.r[9].s64 + 20224;
	// 8328EBF4: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8328EBF8: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8328EBFC: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8328EC00: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8328EC04: 4BA1B31C  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328EC08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328EC08 size=12
    let mut pc: u32 = 0x8328EC08;
    'dispatch: loop {
        match pc {
            0x8328EC08 => {
    //   block [0x8328EC08..0x8328EC14)
	// 8328EC08: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8328EC0C: 386B4F18  addi r3, r11, 0x4f18
	ctx.r[3].s64 = ctx.r[11].s64 + 20248;
	// 8328EC10: 4BA1B310  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328EC18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328EC18 size=64
    let mut pc: u32 = 0x8328EC18;
    'dispatch: loop {
        match pc {
            0x8328EC18 => {
    //   block [0x8328EC18..0x8328EC58)
	// 8328EC18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328EC1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328EC20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328EC24: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8328EC28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328EC2C: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8328EC30: 386A4DF4  addi r3, r10, 0x4df4
	ctx.r[3].s64 = ctx.r[10].s64 + 19956;
	// 8328EC34: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328EC38: 4AF9E299  bl 0x8222ced0
	ctx.lr = 0x8328EC3C;
	sub_8222CED0(ctx, base);
	// 8328EC3C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328EC40: 38694F70  addi r3, r9, 0x4f70
	ctx.r[3].s64 = ctx.r[9].s64 + 20336;
	// 8328EC44: 4BA1B2DD  bl 0x82ca9f20
	ctx.lr = 0x8328EC48;
	sub_82CA9F20(ctx, base);
	// 8328EC48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328EC4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328EC50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328EC54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328EC58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328EC58 size=64
    let mut pc: u32 = 0x8328EC58;
    'dispatch: loop {
        match pc {
            0x8328EC58 => {
    //   block [0x8328EC58..0x8328EC98)
	// 8328EC58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328EC5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328EC60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328EC64: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8328EC68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328EC6C: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8328EC70: 386A4DF8  addi r3, r10, 0x4df8
	ctx.r[3].s64 = ctx.r[10].s64 + 19960;
	// 8328EC74: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328EC78: 4AF9E259  bl 0x8222ced0
	ctx.lr = 0x8328EC7C;
	sub_8222CED0(ctx, base);
	// 8328EC7C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328EC80: 38694F80  addi r3, r9, 0x4f80
	ctx.r[3].s64 = ctx.r[9].s64 + 20352;
	// 8328EC84: 4BA1B29D  bl 0x82ca9f20
	ctx.lr = 0x8328EC88;
	sub_82CA9F20(ctx, base);
	// 8328EC88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328EC8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328EC90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328EC94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328EC98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328EC98 size=64
    let mut pc: u32 = 0x8328EC98;
    'dispatch: loop {
        match pc {
            0x8328EC98 => {
    //   block [0x8328EC98..0x8328ECD8)
	// 8328EC98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328EC9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328ECA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328ECA4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8328ECA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328ECAC: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8328ECB0: 386A4DFC  addi r3, r10, 0x4dfc
	ctx.r[3].s64 = ctx.r[10].s64 + 19964;
	// 8328ECB4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328ECB8: 4AF9E219  bl 0x8222ced0
	ctx.lr = 0x8328ECBC;
	sub_8222CED0(ctx, base);
	// 8328ECBC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328ECC0: 38694F90  addi r3, r9, 0x4f90
	ctx.r[3].s64 = ctx.r[9].s64 + 20368;
	// 8328ECC4: 4BA1B25D  bl 0x82ca9f20
	ctx.lr = 0x8328ECC8;
	sub_82CA9F20(ctx, base);
	// 8328ECC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328ECCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328ECD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328ECD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328ECD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328ECD8 size=64
    let mut pc: u32 = 0x8328ECD8;
    'dispatch: loop {
        match pc {
            0x8328ECD8 => {
    //   block [0x8328ECD8..0x8328ED18)
	// 8328ECD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328ECDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328ECE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328ECE4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8328ECE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328ECEC: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8328ECF0: 386A4E00  addi r3, r10, 0x4e00
	ctx.r[3].s64 = ctx.r[10].s64 + 19968;
	// 8328ECF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328ECF8: 4AF9E1D9  bl 0x8222ced0
	ctx.lr = 0x8328ECFC;
	sub_8222CED0(ctx, base);
	// 8328ECFC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328ED00: 38694FA0  addi r3, r9, 0x4fa0
	ctx.r[3].s64 = ctx.r[9].s64 + 20384;
	// 8328ED04: 4BA1B21D  bl 0x82ca9f20
	ctx.lr = 0x8328ED08;
	sub_82CA9F20(ctx, base);
	// 8328ED08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328ED0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328ED10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328ED14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328ED18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328ED18 size=64
    let mut pc: u32 = 0x8328ED18;
    'dispatch: loop {
        match pc {
            0x8328ED18 => {
    //   block [0x8328ED18..0x8328ED58)
	// 8328ED18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328ED1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328ED20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328ED24: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8328ED28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328ED2C: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8328ED30: 386A4E04  addi r3, r10, 0x4e04
	ctx.r[3].s64 = ctx.r[10].s64 + 19972;
	// 8328ED34: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328ED38: 4AF9E199  bl 0x8222ced0
	ctx.lr = 0x8328ED3C;
	sub_8222CED0(ctx, base);
	// 8328ED3C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328ED40: 38694FB0  addi r3, r9, 0x4fb0
	ctx.r[3].s64 = ctx.r[9].s64 + 20400;
	// 8328ED44: 4BA1B1DD  bl 0x82ca9f20
	ctx.lr = 0x8328ED48;
	sub_82CA9F20(ctx, base);
	// 8328ED48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328ED4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328ED50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328ED54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328ED58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328ED58 size=64
    let mut pc: u32 = 0x8328ED58;
    'dispatch: loop {
        match pc {
            0x8328ED58 => {
    //   block [0x8328ED58..0x8328ED98)
	// 8328ED58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328ED5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328ED60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328ED64: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8328ED68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328ED6C: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8328ED70: 386A4E08  addi r3, r10, 0x4e08
	ctx.r[3].s64 = ctx.r[10].s64 + 19976;
	// 8328ED74: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328ED78: 4AF9E159  bl 0x8222ced0
	ctx.lr = 0x8328ED7C;
	sub_8222CED0(ctx, base);
	// 8328ED7C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328ED80: 38694FC0  addi r3, r9, 0x4fc0
	ctx.r[3].s64 = ctx.r[9].s64 + 20416;
	// 8328ED84: 4BA1B19D  bl 0x82ca9f20
	ctx.lr = 0x8328ED88;
	sub_82CA9F20(ctx, base);
	// 8328ED88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328ED8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328ED90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328ED94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328ED98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328ED98 size=52
    let mut pc: u32 = 0x8328ED98;
    'dispatch: loop {
        match pc {
            0x8328ED98 => {
    //   block [0x8328ED98..0x8328EDCC)
	// 8328ED98: 3CE0834A  lis r7, -0x7cb6
	ctx.r[7].s64 = -2092302336;
	// 8328ED9C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8328EDA0: 38C74E0C  addi r6, r7, 0x4e0c
	ctx.r[6].s64 = ctx.r[7].s64 + 19980;
	// 8328EDA4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8328EDA8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8328EDAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8328EDB0: 91674E0C  stw r11, 0x4e0c(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(19980 as u32), ctx.r[11].u32 ) };
	// 8328EDB4: 3CA0832B  lis r5, -0x7cd5
	ctx.r[5].s64 = -2094333952;
	// 8328EDB8: 91460004  stw r10, 4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8328EDBC: 91260008  stw r9, 8(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8328EDC0: 38654FD0  addi r3, r5, 0x4fd0
	ctx.r[3].s64 = ctx.r[5].s64 + 20432;
	// 8328EDC4: 9106000C  stw r8, 0xc(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 8328EDC8: 4BA1B158  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328EDD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328EDD0 size=52
    let mut pc: u32 = 0x8328EDD0;
    'dispatch: loop {
        match pc {
            0x8328EDD0 => {
    //   block [0x8328EDD0..0x8328EE04)
	// 8328EDD0: 3CE0834A  lis r7, -0x7cb6
	ctx.r[7].s64 = -2092302336;
	// 8328EDD4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8328EDD8: 38C74E1C  addi r6, r7, 0x4e1c
	ctx.r[6].s64 = ctx.r[7].s64 + 19996;
	// 8328EDDC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8328EDE0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8328EDE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8328EDE8: 91674E1C  stw r11, 0x4e1c(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(19996 as u32), ctx.r[11].u32 ) };
	// 8328EDEC: 3CA0832B  lis r5, -0x7cd5
	ctx.r[5].s64 = -2094333952;
	// 8328EDF0: 91460004  stw r10, 4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8328EDF4: 91260008  stw r9, 8(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8328EDF8: 38655020  addi r3, r5, 0x5020
	ctx.r[3].s64 = ctx.r[5].s64 + 20512;
	// 8328EDFC: 9106000C  stw r8, 0xc(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 8328EE00: 4BA1B120  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328EE08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328EE08 size=52
    let mut pc: u32 = 0x8328EE08;
    'dispatch: loop {
        match pc {
            0x8328EE08 => {
    //   block [0x8328EE08..0x8328EE3C)
	// 8328EE08: 3CE0834A  lis r7, -0x7cb6
	ctx.r[7].s64 = -2092302336;
	// 8328EE0C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8328EE10: 38C74E2C  addi r6, r7, 0x4e2c
	ctx.r[6].s64 = ctx.r[7].s64 + 20012;
	// 8328EE14: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8328EE18: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8328EE1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8328EE20: 91674E2C  stw r11, 0x4e2c(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(20012 as u32), ctx.r[11].u32 ) };
	// 8328EE24: 3CA0832B  lis r5, -0x7cd5
	ctx.r[5].s64 = -2094333952;
	// 8328EE28: 91460004  stw r10, 4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8328EE2C: 91260008  stw r9, 8(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8328EE30: 38655070  addi r3, r5, 0x5070
	ctx.r[3].s64 = ctx.r[5].s64 + 20592;
	// 8328EE34: 9106000C  stw r8, 0xc(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 8328EE38: 4BA1B0E8  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328EE40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328EE40 size=52
    let mut pc: u32 = 0x8328EE40;
    'dispatch: loop {
        match pc {
            0x8328EE40 => {
    //   block [0x8328EE40..0x8328EE74)
	// 8328EE40: 3CE0834A  lis r7, -0x7cb6
	ctx.r[7].s64 = -2092302336;
	// 8328EE44: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8328EE48: 38C74E3C  addi r6, r7, 0x4e3c
	ctx.r[6].s64 = ctx.r[7].s64 + 20028;
	// 8328EE4C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8328EE50: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8328EE54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8328EE58: 91674E3C  stw r11, 0x4e3c(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(20028 as u32), ctx.r[11].u32 ) };
	// 8328EE5C: 3CA0832B  lis r5, -0x7cd5
	ctx.r[5].s64 = -2094333952;
	// 8328EE60: 91460004  stw r10, 4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8328EE64: 91260008  stw r9, 8(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8328EE68: 386550C0  addi r3, r5, 0x50c0
	ctx.r[3].s64 = ctx.r[5].s64 + 20672;
	// 8328EE6C: 9106000C  stw r8, 0xc(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 8328EE70: 4BA1B0B0  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328EE78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328EE78 size=52
    let mut pc: u32 = 0x8328EE78;
    'dispatch: loop {
        match pc {
            0x8328EE78 => {
    //   block [0x8328EE78..0x8328EEAC)
	// 8328EE78: 3CE0834A  lis r7, -0x7cb6
	ctx.r[7].s64 = -2092302336;
	// 8328EE7C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8328EE80: 38C74E4C  addi r6, r7, 0x4e4c
	ctx.r[6].s64 = ctx.r[7].s64 + 20044;
	// 8328EE84: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8328EE88: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8328EE8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8328EE90: 91674E4C  stw r11, 0x4e4c(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(20044 as u32), ctx.r[11].u32 ) };
	// 8328EE94: 3CA0832B  lis r5, -0x7cd5
	ctx.r[5].s64 = -2094333952;
	// 8328EE98: 91460004  stw r10, 4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8328EE9C: 91260008  stw r9, 8(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8328EEA0: 38655110  addi r3, r5, 0x5110
	ctx.r[3].s64 = ctx.r[5].s64 + 20752;
	// 8328EEA4: 9106000C  stw r8, 0xc(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 8328EEA8: 4BA1B078  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328EEB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328EEB0 size=52
    let mut pc: u32 = 0x8328EEB0;
    'dispatch: loop {
        match pc {
            0x8328EEB0 => {
    //   block [0x8328EEB0..0x8328EEE4)
	// 8328EEB0: 3CE0834A  lis r7, -0x7cb6
	ctx.r[7].s64 = -2092302336;
	// 8328EEB4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8328EEB8: 38C74E5C  addi r6, r7, 0x4e5c
	ctx.r[6].s64 = ctx.r[7].s64 + 20060;
	// 8328EEBC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8328EEC0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8328EEC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8328EEC8: 91674E5C  stw r11, 0x4e5c(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(20060 as u32), ctx.r[11].u32 ) };
	// 8328EECC: 3CA0832B  lis r5, -0x7cd5
	ctx.r[5].s64 = -2094333952;
	// 8328EED0: 91460004  stw r10, 4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8328EED4: 91260008  stw r9, 8(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8328EED8: 38655160  addi r3, r5, 0x5160
	ctx.r[3].s64 = ctx.r[5].s64 + 20832;
	// 8328EEDC: 9106000C  stw r8, 0xc(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 8328EEE0: 4BA1B040  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328EEE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328EEE8 size=12
    let mut pc: u32 = 0x8328EEE8;
    'dispatch: loop {
        match pc {
            0x8328EEE8 => {
    //   block [0x8328EEE8..0x8328EEF4)
	// 8328EEE8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8328EEEC: 386B51B0  addi r3, r11, 0x51b0
	ctx.r[3].s64 = ctx.r[11].s64 + 20912;
	// 8328EEF0: 4BA1B030  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328EEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328EEF8 size=144
    let mut pc: u32 = 0x8328EEF8;
    'dispatch: loop {
        match pc {
            0x8328EEF8 => {
    //   block [0x8328EEF8..0x8328EF1C)
	// 8328EEF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328EEFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328EF00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328EF04: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8328EF08: 4AF90351  bl 0x8221f258
	ctx.lr = 0x8328EF0C;
	sub_8221F258(ctx, base);
	// 8328EF0C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8328EF10: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8328EF14: 419A0008  beq cr6, 0x8328ef1c
	if ctx.cr[6].eq {
	pc = 0x8328EF1C; continue 'dispatch;
	}
	// 8328EF18: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8328EF1C; continue 'dispatch;
            }
            0x8328EF1C => {
    //   block [0x8328EF1C..0x8328EF28)
	// 8328EF1C: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8328EF20: 41820008  beq 0x8328ef28
	if ctx.cr[0].eq {
	pc = 0x8328EF28; continue 'dispatch;
	}
	// 8328EF24: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8328EF28; continue 'dispatch;
            }
            0x8328EF28 => {
    //   block [0x8328EF28..0x8328EF34)
	// 8328EF28: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8328EF2C: 41820008  beq 0x8328ef34
	if ctx.cr[0].eq {
	pc = 0x8328EF34; continue 'dispatch;
	}
	// 8328EF30: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8328EF34; continue 'dispatch;
            }
            0x8328EF34 => {
    //   block [0x8328EF34..0x8328EF88)
	// 8328EF34: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8328EF38: 99430015  stb r10, 0x15(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(21 as u32), ctx.r[10].u8 ) };
	// 8328EF3C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8328EF40: 39094E6C  addi r8, r9, 0x4e6c
	ctx.r[8].s64 = ctx.r[9].s64 + 20076;
	// 8328EF44: 99630014  stb r11, 0x14(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 8328EF48: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8328EF4C: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8328EF50: 99630015  stb r11, 0x15(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(21 as u32), ctx.r[11].u8 ) };
	// 8328EF54: 38675210  addi r3, r7, 0x5210
	ctx.r[3].s64 = ctx.r[7].s64 + 21008;
	// 8328EF58: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8328EF5C: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8328EF60: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8328EF64: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8328EF68: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8328EF6C: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8328EF70: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8328EF74: 4BA1AFAD  bl 0x82ca9f20
	ctx.lr = 0x8328EF78;
	sub_82CA9F20(ctx, base);
	// 8328EF78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328EF7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328EF80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328EF84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328EF88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328EF88 size=64
    let mut pc: u32 = 0x8328EF88;
    'dispatch: loop {
        match pc {
            0x8328EF88 => {
    //   block [0x8328EF88..0x8328EFC8)
	// 8328EF88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328EF8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328EF90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328EF94: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8328EF98: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328EF9C: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8328EFA0: 386A4E78  addi r3, r10, 0x4e78
	ctx.r[3].s64 = ctx.r[10].s64 + 20088;
	// 8328EFA4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328EFA8: 4AF9DF29  bl 0x8222ced0
	ctx.lr = 0x8328EFAC;
	sub_8222CED0(ctx, base);
	// 8328EFAC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328EFB0: 38695230  addi r3, r9, 0x5230
	ctx.r[3].s64 = ctx.r[9].s64 + 21040;
	// 8328EFB4: 4BA1AF6D  bl 0x82ca9f20
	ctx.lr = 0x8328EFB8;
	sub_82CA9F20(ctx, base);
	// 8328EFB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328EFBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328EFC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328EFC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328EFC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328EFC8 size=64
    let mut pc: u32 = 0x8328EFC8;
    'dispatch: loop {
        match pc {
            0x8328EFC8 => {
    //   block [0x8328EFC8..0x8328F008)
	// 8328EFC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328EFCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328EFD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328EFD4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8328EFD8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328EFDC: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8328EFE0: 386A4E7C  addi r3, r10, 0x4e7c
	ctx.r[3].s64 = ctx.r[10].s64 + 20092;
	// 8328EFE4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328EFE8: 4AF9DEE9  bl 0x8222ced0
	ctx.lr = 0x8328EFEC;
	sub_8222CED0(ctx, base);
	// 8328EFEC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328EFF0: 38695240  addi r3, r9, 0x5240
	ctx.r[3].s64 = ctx.r[9].s64 + 21056;
	// 8328EFF4: 4BA1AF2D  bl 0x82ca9f20
	ctx.lr = 0x8328EFF8;
	sub_82CA9F20(ctx, base);
	// 8328EFF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328EFFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328F000: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328F004: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328F008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328F008 size=144
    let mut pc: u32 = 0x8328F008;
    'dispatch: loop {
        match pc {
            0x8328F008 => {
    //   block [0x8328F008..0x8328F02C)
	// 8328F008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328F00C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328F010: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328F014: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 8328F018: 4AF90241  bl 0x8221f258
	ctx.lr = 0x8328F01C;
	sub_8221F258(ctx, base);
	// 8328F01C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8328F020: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8328F024: 419A0008  beq cr6, 0x8328f02c
	if ctx.cr[6].eq {
	pc = 0x8328F02C; continue 'dispatch;
	}
	// 8328F028: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8328F02C; continue 'dispatch;
            }
            0x8328F02C => {
    //   block [0x8328F02C..0x8328F038)
	// 8328F02C: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8328F030: 41820008  beq 0x8328f038
	if ctx.cr[0].eq {
	pc = 0x8328F038; continue 'dispatch;
	}
	// 8328F034: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8328F038; continue 'dispatch;
            }
            0x8328F038 => {
    //   block [0x8328F038..0x8328F044)
	// 8328F038: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8328F03C: 41820008  beq 0x8328f044
	if ctx.cr[0].eq {
	pc = 0x8328F044; continue 'dispatch;
	}
	// 8328F040: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8328F044; continue 'dispatch;
            }
            0x8328F044 => {
    //   block [0x8328F044..0x8328F098)
	// 8328F044: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8328F048: 99430019  stb r10, 0x19(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(25 as u32), ctx.r[10].u8 ) };
	// 8328F04C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8328F050: 39094E80  addi r8, r9, 0x4e80
	ctx.r[8].s64 = ctx.r[9].s64 + 20096;
	// 8328F054: 99630018  stb r11, 0x18(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u8 ) };
	// 8328F058: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8328F05C: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8328F060: 99630019  stb r11, 0x19(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(25 as u32), ctx.r[11].u8 ) };
	// 8328F064: 38675250  addi r3, r7, 0x5250
	ctx.r[3].s64 = ctx.r[7].s64 + 21072;
	// 8328F068: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8328F06C: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8328F070: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8328F074: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8328F078: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8328F07C: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8328F080: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8328F084: 4BA1AE9D  bl 0x82ca9f20
	ctx.lr = 0x8328F088;
	sub_82CA9F20(ctx, base);
	// 8328F088: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328F08C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328F090: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328F094: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328F098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328F098 size=64
    let mut pc: u32 = 0x8328F098;
    'dispatch: loop {
        match pc {
            0x8328F098 => {
    //   block [0x8328F098..0x8328F0D8)
	// 8328F098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328F09C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328F0A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328F0A4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8328F0A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328F0AC: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8328F0B0: 386A4E8C  addi r3, r10, 0x4e8c
	ctx.r[3].s64 = ctx.r[10].s64 + 20108;
	// 8328F0B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328F0B8: 4AF9DE19  bl 0x8222ced0
	ctx.lr = 0x8328F0BC;
	sub_8222CED0(ctx, base);
	// 8328F0BC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328F0C0: 38695260  addi r3, r9, 0x5260
	ctx.r[3].s64 = ctx.r[9].s64 + 21088;
	// 8328F0C4: 4BA1AE5D  bl 0x82ca9f20
	ctx.lr = 0x8328F0C8;
	sub_82CA9F20(ctx, base);
	// 8328F0C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328F0CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328F0D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328F0D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328F0D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328F0D8 size=64
    let mut pc: u32 = 0x8328F0D8;
    'dispatch: loop {
        match pc {
            0x8328F0D8 => {
    //   block [0x8328F0D8..0x8328F118)
	// 8328F0D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328F0DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328F0E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328F0E4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8328F0E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328F0EC: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8328F0F0: 386A4E90  addi r3, r10, 0x4e90
	ctx.r[3].s64 = ctx.r[10].s64 + 20112;
	// 8328F0F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328F0F8: 4AF9DDD9  bl 0x8222ced0
	ctx.lr = 0x8328F0FC;
	sub_8222CED0(ctx, base);
	// 8328F0FC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328F100: 38695270  addi r3, r9, 0x5270
	ctx.r[3].s64 = ctx.r[9].s64 + 21104;
	// 8328F104: 4BA1AE1D  bl 0x82ca9f20
	ctx.lr = 0x8328F108;
	sub_82CA9F20(ctx, base);
	// 8328F108: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328F10C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328F110: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328F114: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328F118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328F118 size=12
    let mut pc: u32 = 0x8328F118;
    'dispatch: loop {
        match pc {
            0x8328F118 => {
    //   block [0x8328F118..0x8328F124)
	// 8328F118: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8328F11C: 386B5280  addi r3, r11, 0x5280
	ctx.r[3].s64 = ctx.r[11].s64 + 21120;
	// 8328F120: 4BA1AE00  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328F128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328F128 size=64
    let mut pc: u32 = 0x8328F128;
    'dispatch: loop {
        match pc {
            0x8328F128 => {
    //   block [0x8328F128..0x8328F168)
	// 8328F128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328F12C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328F130: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328F134: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8328F138: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328F13C: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8328F140: 386A4EA4  addi r3, r10, 0x4ea4
	ctx.r[3].s64 = ctx.r[10].s64 + 20132;
	// 8328F144: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328F148: 4AF9DD89  bl 0x8222ced0
	ctx.lr = 0x8328F14C;
	sub_8222CED0(ctx, base);
	// 8328F14C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328F150: 386952D8  addi r3, r9, 0x52d8
	ctx.r[3].s64 = ctx.r[9].s64 + 21208;
	// 8328F154: 4BA1ADCD  bl 0x82ca9f20
	ctx.lr = 0x8328F158;
	sub_82CA9F20(ctx, base);
	// 8328F158: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328F15C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328F160: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328F164: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328F168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328F168 size=64
    let mut pc: u32 = 0x8328F168;
    'dispatch: loop {
        match pc {
            0x8328F168 => {
    //   block [0x8328F168..0x8328F1A8)
	// 8328F168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328F16C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328F170: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328F174: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8328F178: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328F17C: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8328F180: 386A4EA8  addi r3, r10, 0x4ea8
	ctx.r[3].s64 = ctx.r[10].s64 + 20136;
	// 8328F184: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328F188: 4AF9DD49  bl 0x8222ced0
	ctx.lr = 0x8328F18C;
	sub_8222CED0(ctx, base);
	// 8328F18C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328F190: 386952E8  addi r3, r9, 0x52e8
	ctx.r[3].s64 = ctx.r[9].s64 + 21224;
	// 8328F194: 4BA1AD8D  bl 0x82ca9f20
	ctx.lr = 0x8328F198;
	sub_82CA9F20(ctx, base);
	// 8328F198: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328F19C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328F1A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328F1A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328F1A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328F1A8 size=60
    let mut pc: u32 = 0x8328F1A8;
    'dispatch: loop {
        match pc {
            0x8328F1A8 => {
    //   block [0x8328F1A8..0x8328F1B8)
	// 8328F1A8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8328F1AC: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8328F1B0: 396B4EC0  addi r11, r11, 0x4ec0
	ctx.r[11].s64 = ctx.r[11].s64 + 20160;
	// 8328F1B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	pc = 0x8328F1B8; continue 'dispatch;
            }
            0x8328F1B8 => {
    //   block [0x8328F1B8..0x8328F1E4)
	// 8328F1B8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8328F1BC: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8328F1C0: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8328F1C4: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8328F1C8: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8328F1CC: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8328F1D0: 396B0014  addi r11, r11, 0x14
	ctx.r[11].s64 = ctx.r[11].s64 + 20;
	// 8328F1D4: 4080FFE4  bge 0x8328f1b8
	if !ctx.cr[0].lt {
	pc = 0x8328F1B8; continue 'dispatch;
	}
	// 8328F1D8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8328F1DC: 386B52F8  addi r3, r11, 0x52f8
	ctx.r[3].s64 = ctx.r[11].s64 + 21240;
	// 8328F1E0: 4BA1AD40  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328F1E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328F1E8 size=40
    let mut pc: u32 = 0x8328F1E8;
    'dispatch: loop {
        match pc {
            0x8328F1E8 => {
    //   block [0x8328F1E8..0x8328F210)
	// 8328F1E8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8328F1EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8328F1F0: 396B4EAC  addi r11, r11, 0x4eac
	ctx.r[11].s64 = ctx.r[11].s64 + 20140;
	// 8328F1F4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328F1F8: 38695350  addi r3, r9, 0x5350
	ctx.r[3].s64 = ctx.r[9].s64 + 21328;
	// 8328F1FC: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8328F200: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8328F204: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8328F208: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8328F20C: 4BA1AD14  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328F210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328F210 size=40
    let mut pc: u32 = 0x8328F210;
    'dispatch: loop {
        match pc {
            0x8328F210 => {
    //   block [0x8328F210..0x8328F238)
	// 8328F210: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8328F214: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8328F218: 396B4F10  addi r11, r11, 0x4f10
	ctx.r[11].s64 = ctx.r[11].s64 + 20240;
	// 8328F21C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328F220: 38695368  addi r3, r9, 0x5368
	ctx.r[3].s64 = ctx.r[9].s64 + 21352;
	// 8328F224: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8328F228: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8328F22C: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8328F230: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8328F234: 4BA1ACEC  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328F238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328F238 size=64
    let mut pc: u32 = 0x8328F238;
    'dispatch: loop {
        match pc {
            0x8328F238 => {
    //   block [0x8328F238..0x8328F278)
	// 8328F238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328F23C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328F240: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328F244: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8328F248: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328F24C: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8328F250: 386A4F24  addi r3, r10, 0x4f24
	ctx.r[3].s64 = ctx.r[10].s64 + 20260;
	// 8328F254: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328F258: 4AF9DC79  bl 0x8222ced0
	ctx.lr = 0x8328F25C;
	sub_8222CED0(ctx, base);
	// 8328F25C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328F260: 38695380  addi r3, r9, 0x5380
	ctx.r[3].s64 = ctx.r[9].s64 + 21376;
	// 8328F264: 4BA1ACBD  bl 0x82ca9f20
	ctx.lr = 0x8328F268;
	sub_82CA9F20(ctx, base);
	// 8328F268: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328F26C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328F270: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328F274: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328F278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328F278 size=64
    let mut pc: u32 = 0x8328F278;
    'dispatch: loop {
        match pc {
            0x8328F278 => {
    //   block [0x8328F278..0x8328F2B8)
	// 8328F278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328F27C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328F280: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328F284: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8328F288: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328F28C: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8328F290: 386A4F28  addi r3, r10, 0x4f28
	ctx.r[3].s64 = ctx.r[10].s64 + 20264;
	// 8328F294: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328F298: 4AF9DC39  bl 0x8222ced0
	ctx.lr = 0x8328F29C;
	sub_8222CED0(ctx, base);
	// 8328F29C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328F2A0: 38695390  addi r3, r9, 0x5390
	ctx.r[3].s64 = ctx.r[9].s64 + 21392;
	// 8328F2A4: 4BA1AC7D  bl 0x82ca9f20
	ctx.lr = 0x8328F2A8;
	sub_82CA9F20(ctx, base);
	// 8328F2A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328F2AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328F2B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328F2B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328F2B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328F2B8 size=60
    let mut pc: u32 = 0x8328F2B8;
    'dispatch: loop {
        match pc {
            0x8328F2B8 => {
    //   block [0x8328F2B8..0x8328F2C8)
	// 8328F2B8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8328F2BC: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8328F2C0: 396B4F30  addi r11, r11, 0x4f30
	ctx.r[11].s64 = ctx.r[11].s64 + 20272;
	// 8328F2C4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	pc = 0x8328F2C8; continue 'dispatch;
            }
            0x8328F2C8 => {
    //   block [0x8328F2C8..0x8328F2F4)
	// 8328F2C8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8328F2CC: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8328F2D0: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8328F2D4: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8328F2D8: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8328F2DC: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8328F2E0: 396B0014  addi r11, r11, 0x14
	ctx.r[11].s64 = ctx.r[11].s64 + 20;
	// 8328F2E4: 4080FFE4  bge 0x8328f2c8
	if !ctx.cr[0].lt {
	pc = 0x8328F2C8; continue 'dispatch;
	}
	// 8328F2E8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8328F2EC: 386B53A0  addi r3, r11, 0x53a0
	ctx.r[3].s64 = ctx.r[11].s64 + 21408;
	// 8328F2F0: 4BA1AC30  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328F2F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328F2F8 size=12
    let mut pc: u32 = 0x8328F2F8;
    'dispatch: loop {
        match pc {
            0x8328F2F8 => {
    //   block [0x8328F2F8..0x8328F304)
	// 8328F2F8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8328F2FC: 386B53F8  addi r3, r11, 0x53f8
	ctx.r[3].s64 = ctx.r[11].s64 + 21496;
	// 8328F300: 4BA1AC20  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328F308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328F308 size=64
    let mut pc: u32 = 0x8328F308;
    'dispatch: loop {
        match pc {
            0x8328F308 => {
    //   block [0x8328F308..0x8328F348)
	// 8328F308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328F30C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328F310: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328F314: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8328F318: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328F31C: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8328F320: 386A4F84  addi r3, r10, 0x4f84
	ctx.r[3].s64 = ctx.r[10].s64 + 20356;
	// 8328F324: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328F328: 4AF9DBA9  bl 0x8222ced0
	ctx.lr = 0x8328F32C;
	sub_8222CED0(ctx, base);
	// 8328F32C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328F330: 38695470  addi r3, r9, 0x5470
	ctx.r[3].s64 = ctx.r[9].s64 + 21616;
	// 8328F334: 4BA1ABED  bl 0x82ca9f20
	ctx.lr = 0x8328F338;
	sub_82CA9F20(ctx, base);
	// 8328F338: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328F33C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328F340: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328F344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328F348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328F348 size=64
    let mut pc: u32 = 0x8328F348;
    'dispatch: loop {
        match pc {
            0x8328F348 => {
    //   block [0x8328F348..0x8328F388)
	// 8328F348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328F34C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328F350: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328F354: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8328F358: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328F35C: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8328F360: 386A4F88  addi r3, r10, 0x4f88
	ctx.r[3].s64 = ctx.r[10].s64 + 20360;
	// 8328F364: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328F368: 4AF9DB69  bl 0x8222ced0
	ctx.lr = 0x8328F36C;
	sub_8222CED0(ctx, base);
	// 8328F36C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328F370: 38695480  addi r3, r9, 0x5480
	ctx.r[3].s64 = ctx.r[9].s64 + 21632;
	// 8328F374: 4BA1ABAD  bl 0x82ca9f20
	ctx.lr = 0x8328F378;
	sub_82CA9F20(ctx, base);
	// 8328F378: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328F37C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328F380: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328F384: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328F388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328F388 size=40
    let mut pc: u32 = 0x8328F388;
    'dispatch: loop {
        match pc {
            0x8328F388 => {
    //   block [0x8328F388..0x8328F3B0)
	// 8328F388: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8328F38C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8328F390: 396B4F8C  addi r11, r11, 0x4f8c
	ctx.r[11].s64 = ctx.r[11].s64 + 20364;
	// 8328F394: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328F398: 38695490  addi r3, r9, 0x5490
	ctx.r[3].s64 = ctx.r[9].s64 + 21648;
	// 8328F39C: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8328F3A0: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8328F3A4: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8328F3A8: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8328F3AC: 4BA1AB74  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328F3B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328F3B0 size=40
    let mut pc: u32 = 0x8328F3B0;
    'dispatch: loop {
        match pc {
            0x8328F3B0 => {
    //   block [0x8328F3B0..0x8328F3D8)
	// 8328F3B0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8328F3B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8328F3B8: 396B4FA0  addi r11, r11, 0x4fa0
	ctx.r[11].s64 = ctx.r[11].s64 + 20384;
	// 8328F3BC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328F3C0: 386954A8  addi r3, r9, 0x54a8
	ctx.r[3].s64 = ctx.r[9].s64 + 21672;
	// 8328F3C4: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8328F3C8: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8328F3CC: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8328F3D0: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8328F3D4: 4BA1AB4C  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328F3D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328F3D8 size=40
    let mut pc: u32 = 0x8328F3D8;
    'dispatch: loop {
        match pc {
            0x8328F3D8 => {
    //   block [0x8328F3D8..0x8328F400)
	// 8328F3D8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8328F3DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8328F3E0: 396B4FB4  addi r11, r11, 0x4fb4
	ctx.r[11].s64 = ctx.r[11].s64 + 20404;
	// 8328F3E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328F3E8: 386954C0  addi r3, r9, 0x54c0
	ctx.r[3].s64 = ctx.r[9].s64 + 21696;
	// 8328F3EC: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8328F3F0: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8328F3F4: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8328F3F8: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8328F3FC: 4BA1AB24  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328F400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328F400 size=40
    let mut pc: u32 = 0x8328F400;
    'dispatch: loop {
        match pc {
            0x8328F400 => {
    //   block [0x8328F400..0x8328F428)
	// 8328F400: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8328F404: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8328F408: 396B4FC8  addi r11, r11, 0x4fc8
	ctx.r[11].s64 = ctx.r[11].s64 + 20424;
	// 8328F40C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328F410: 386954D8  addi r3, r9, 0x54d8
	ctx.r[3].s64 = ctx.r[9].s64 + 21720;
	// 8328F414: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8328F418: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8328F41C: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8328F420: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8328F424: 4BA1AAFC  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328F428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328F428 size=40
    let mut pc: u32 = 0x8328F428;
    'dispatch: loop {
        match pc {
            0x8328F428 => {
    //   block [0x8328F428..0x8328F450)
	// 8328F428: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8328F42C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8328F430: 396B4FDC  addi r11, r11, 0x4fdc
	ctx.r[11].s64 = ctx.r[11].s64 + 20444;
	// 8328F434: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328F438: 386954F0  addi r3, r9, 0x54f0
	ctx.r[3].s64 = ctx.r[9].s64 + 21744;
	// 8328F43C: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8328F440: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8328F444: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8328F448: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8328F44C: 4BA1AAD4  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328F450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328F450 size=64
    let mut pc: u32 = 0x8328F450;
    'dispatch: loop {
        match pc {
            0x8328F450 => {
    //   block [0x8328F450..0x8328F490)
	// 8328F450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328F454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328F458: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328F45C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8328F460: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328F464: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8328F468: 386A4FF0  addi r3, r10, 0x4ff0
	ctx.r[3].s64 = ctx.r[10].s64 + 20464;
	// 8328F46C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328F470: 4AF9DA61  bl 0x8222ced0
	ctx.lr = 0x8328F474;
	sub_8222CED0(ctx, base);
	// 8328F474: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328F478: 38695560  addi r3, r9, 0x5560
	ctx.r[3].s64 = ctx.r[9].s64 + 21856;
	// 8328F47C: 4BA1AAA5  bl 0x82ca9f20
	ctx.lr = 0x8328F480;
	sub_82CA9F20(ctx, base);
	// 8328F480: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328F484: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328F488: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328F48C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328F490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328F490 size=64
    let mut pc: u32 = 0x8328F490;
    'dispatch: loop {
        match pc {
            0x8328F490 => {
    //   block [0x8328F490..0x8328F4D0)
	// 8328F490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328F494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328F498: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328F49C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8328F4A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328F4A4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8328F4A8: 386A4FF4  addi r3, r10, 0x4ff4
	ctx.r[3].s64 = ctx.r[10].s64 + 20468;
	// 8328F4AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328F4B0: 4AF9DA21  bl 0x8222ced0
	ctx.lr = 0x8328F4B4;
	sub_8222CED0(ctx, base);
	// 8328F4B4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328F4B8: 38695570  addi r3, r9, 0x5570
	ctx.r[3].s64 = ctx.r[9].s64 + 21872;
	// 8328F4BC: 4BA1AA65  bl 0x82ca9f20
	ctx.lr = 0x8328F4C0;
	sub_82CA9F20(ctx, base);
	// 8328F4C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328F4C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328F4C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328F4CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328F4D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328F4D0 size=12
    let mut pc: u32 = 0x8328F4D0;
    'dispatch: loop {
        match pc {
            0x8328F4D0 => {
    //   block [0x8328F4D0..0x8328F4DC)
	// 8328F4D0: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8328F4D4: 386B5580  addi r3, r11, 0x5580
	ctx.r[3].s64 = ctx.r[11].s64 + 21888;
	// 8328F4D8: 4BA1AA48  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328F4E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328F4E0 size=12
    let mut pc: u32 = 0x8328F4E0;
    'dispatch: loop {
        match pc {
            0x8328F4E0 => {
    //   block [0x8328F4E0..0x8328F4EC)
	// 8328F4E0: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8328F4E4: 386B55D8  addi r3, r11, 0x55d8
	ctx.r[3].s64 = ctx.r[11].s64 + 21976;
	// 8328F4E8: 4BA1AA38  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328F4F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328F4F0 size=12
    let mut pc: u32 = 0x8328F4F0;
    'dispatch: loop {
        match pc {
            0x8328F4F0 => {
    //   block [0x8328F4F0..0x8328F4FC)
	// 8328F4F0: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8328F4F4: 386B5630  addi r3, r11, 0x5630
	ctx.r[3].s64 = ctx.r[11].s64 + 22064;
	// 8328F4F8: 4BA1AA28  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328F500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328F500 size=64
    let mut pc: u32 = 0x8328F500;
    'dispatch: loop {
        match pc {
            0x8328F500 => {
    //   block [0x8328F500..0x8328F540)
	// 8328F500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328F504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328F508: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328F50C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8328F510: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328F514: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8328F518: 386A5028  addi r3, r10, 0x5028
	ctx.r[3].s64 = ctx.r[10].s64 + 20520;
	// 8328F51C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328F520: 4AF9D9B1  bl 0x8222ced0
	ctx.lr = 0x8328F524;
	sub_8222CED0(ctx, base);
	// 8328F524: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328F528: 38695688  addi r3, r9, 0x5688
	ctx.r[3].s64 = ctx.r[9].s64 + 22152;
	// 8328F52C: 4BA1A9F5  bl 0x82ca9f20
	ctx.lr = 0x8328F530;
	sub_82CA9F20(ctx, base);
	// 8328F530: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328F534: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328F538: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328F53C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328F540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328F540 size=64
    let mut pc: u32 = 0x8328F540;
    'dispatch: loop {
        match pc {
            0x8328F540 => {
    //   block [0x8328F540..0x8328F580)
	// 8328F540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328F544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328F548: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328F54C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8328F550: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328F554: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8328F558: 386A502C  addi r3, r10, 0x502c
	ctx.r[3].s64 = ctx.r[10].s64 + 20524;
	// 8328F55C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328F560: 4AF9D971  bl 0x8222ced0
	ctx.lr = 0x8328F564;
	sub_8222CED0(ctx, base);
	// 8328F564: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328F568: 38695698  addi r3, r9, 0x5698
	ctx.r[3].s64 = ctx.r[9].s64 + 22168;
	// 8328F56C: 4BA1A9B5  bl 0x82ca9f20
	ctx.lr = 0x8328F570;
	sub_82CA9F20(ctx, base);
	// 8328F570: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328F574: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328F578: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328F57C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328F580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328F580 size=12
    let mut pc: u32 = 0x8328F580;
    'dispatch: loop {
        match pc {
            0x8328F580 => {
    //   block [0x8328F580..0x8328F58C)
	// 8328F580: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8328F584: 386B56A8  addi r3, r11, 0x56a8
	ctx.r[3].s64 = ctx.r[11].s64 + 22184;
	// 8328F588: 4BA1A998  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328F590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328F590 size=96
    let mut pc: u32 = 0x8328F590;
    'dispatch: loop {
        match pc {
            0x8328F590 => {
    //   block [0x8328F590..0x8328F5B4)
	// 8328F590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328F594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328F598: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328F59C: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 8328F5A0: 4AF8FCB9  bl 0x8221f258
	ctx.lr = 0x8328F5A4;
	sub_8221F258(ctx, base);
	// 8328F5A4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8328F5A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8328F5AC: 419A0008  beq cr6, 0x8328f5b4
	if ctx.cr[6].eq {
	pc = 0x8328F5B4; continue 'dispatch;
	}
	// 8328F5B0: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x8328F5B4; continue 'dispatch;
            }
            0x8328F5B4 => {
    //   block [0x8328F5B4..0x8328F5C0)
	// 8328F5B4: 354B0004  addic. r10, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8328F5B8: 41820008  beq 0x8328f5c0
	if ctx.cr[0].eq {
	pc = 0x8328F5C0; continue 'dispatch;
	}
	// 8328F5BC: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x8328F5C0; continue 'dispatch;
            }
            0x8328F5C0 => {
    //   block [0x8328F5C0..0x8328F5F0)
	// 8328F5C0: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8328F5C4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8328F5C8: 39095034  addi r8, r9, 0x5034
	ctx.r[8].s64 = ctx.r[9].s64 + 20532;
	// 8328F5CC: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8328F5D0: 386756E8  addi r3, r7, 0x56e8
	ctx.r[3].s64 = ctx.r[7].s64 + 22248;
	// 8328F5D4: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8328F5D8: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8328F5DC: 4BA1A945  bl 0x82ca9f20
	ctx.lr = 0x8328F5E0;
	sub_82CA9F20(ctx, base);
	// 8328F5E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328F5E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328F5E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328F5EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328F5F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328F5F0 size=96
    let mut pc: u32 = 0x8328F5F0;
    'dispatch: loop {
        match pc {
            0x8328F5F0 => {
    //   block [0x8328F5F0..0x8328F614)
	// 8328F5F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328F5F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328F5F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328F5FC: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 8328F600: 4AF8FC59  bl 0x8221f258
	ctx.lr = 0x8328F604;
	sub_8221F258(ctx, base);
	// 8328F604: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8328F608: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8328F60C: 419A0008  beq cr6, 0x8328f614
	if ctx.cr[6].eq {
	pc = 0x8328F614; continue 'dispatch;
	}
	// 8328F610: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x8328F614; continue 'dispatch;
            }
            0x8328F614 => {
    //   block [0x8328F614..0x8328F620)
	// 8328F614: 354B0004  addic. r10, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8328F618: 41820008  beq 0x8328f620
	if ctx.cr[0].eq {
	pc = 0x8328F620; continue 'dispatch;
	}
	// 8328F61C: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x8328F620; continue 'dispatch;
            }
            0x8328F620 => {
    //   block [0x8328F620..0x8328F650)
	// 8328F620: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8328F624: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8328F628: 39095040  addi r8, r9, 0x5040
	ctx.r[8].s64 = ctx.r[9].s64 + 20544;
	// 8328F62C: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8328F630: 38675730  addi r3, r7, 0x5730
	ctx.r[3].s64 = ctx.r[7].s64 + 22320;
	// 8328F634: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8328F638: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8328F63C: 4BA1A8E5  bl 0x82ca9f20
	ctx.lr = 0x8328F640;
	sub_82CA9F20(ctx, base);
	// 8328F640: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328F644: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328F648: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328F64C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328F650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328F650 size=96
    let mut pc: u32 = 0x8328F650;
    'dispatch: loop {
        match pc {
            0x8328F650 => {
    //   block [0x8328F650..0x8328F674)
	// 8328F650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328F654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328F658: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328F65C: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 8328F660: 4AF8FBF9  bl 0x8221f258
	ctx.lr = 0x8328F664;
	sub_8221F258(ctx, base);
	// 8328F664: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8328F668: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8328F66C: 419A0008  beq cr6, 0x8328f674
	if ctx.cr[6].eq {
	pc = 0x8328F674; continue 'dispatch;
	}
	// 8328F670: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x8328F674; continue 'dispatch;
            }
            0x8328F674 => {
    //   block [0x8328F674..0x8328F680)
	// 8328F674: 354B0004  addic. r10, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8328F678: 41820008  beq 0x8328f680
	if ctx.cr[0].eq {
	pc = 0x8328F680; continue 'dispatch;
	}
	// 8328F67C: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x8328F680; continue 'dispatch;
            }
            0x8328F680 => {
    //   block [0x8328F680..0x8328F6B0)
	// 8328F680: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8328F684: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8328F688: 3909504C  addi r8, r9, 0x504c
	ctx.r[8].s64 = ctx.r[9].s64 + 20556;
	// 8328F68C: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8328F690: 38675778  addi r3, r7, 0x5778
	ctx.r[3].s64 = ctx.r[7].s64 + 22392;
	// 8328F694: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8328F698: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8328F69C: 4BA1A885  bl 0x82ca9f20
	ctx.lr = 0x8328F6A0;
	sub_82CA9F20(ctx, base);
	// 8328F6A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328F6A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328F6A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328F6AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328F6B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328F6B0 size=96
    let mut pc: u32 = 0x8328F6B0;
    'dispatch: loop {
        match pc {
            0x8328F6B0 => {
    //   block [0x8328F6B0..0x8328F6D4)
	// 8328F6B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328F6B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328F6B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328F6BC: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 8328F6C0: 4AF8FB99  bl 0x8221f258
	ctx.lr = 0x8328F6C4;
	sub_8221F258(ctx, base);
	// 8328F6C4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8328F6C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8328F6CC: 419A0008  beq cr6, 0x8328f6d4
	if ctx.cr[6].eq {
	pc = 0x8328F6D4; continue 'dispatch;
	}
	// 8328F6D0: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x8328F6D4; continue 'dispatch;
            }
            0x8328F6D4 => {
    //   block [0x8328F6D4..0x8328F6E0)
	// 8328F6D4: 354B0004  addic. r10, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8328F6D8: 41820008  beq 0x8328f6e0
	if ctx.cr[0].eq {
	pc = 0x8328F6E0; continue 'dispatch;
	}
	// 8328F6DC: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x8328F6E0; continue 'dispatch;
            }
            0x8328F6E0 => {
    //   block [0x8328F6E0..0x8328F710)
	// 8328F6E0: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8328F6E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8328F6E8: 39095058  addi r8, r9, 0x5058
	ctx.r[8].s64 = ctx.r[9].s64 + 20568;
	// 8328F6EC: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8328F6F0: 38675800  addi r3, r7, 0x5800
	ctx.r[3].s64 = ctx.r[7].s64 + 22528;
	// 8328F6F4: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8328F6F8: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8328F6FC: 4BA1A825  bl 0x82ca9f20
	ctx.lr = 0x8328F700;
	sub_82CA9F20(ctx, base);
	// 8328F700: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328F704: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328F708: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328F70C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328F710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8328F710 size=100
    let mut pc: u32 = 0x8328F710;
    'dispatch: loop {
        match pc {
            0x8328F710 => {
    //   block [0x8328F710..0x8328F774)
	// 8328F710: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8328F714: 3921FFF0  addi r9, r1, -0x10
	ctx.r[9].s64 = ctx.r[1].s64 + -16;
	// 8328F718: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8328F71C: 3901FFF4  addi r8, r1, -0xc
	ctx.r[8].s64 = ctx.r[1].s64 + -12;
	// 8328F720: 38E1FFF4  addi r7, r1, -0xc
	ctx.r[7].s64 = ctx.r[1].s64 + -12;
	// 8328F724: C00B9484  lfs f0, -0x6b7c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8328F728: 38C1FFF0  addi r6, r1, -0x10
	ctx.r[6].s64 = ctx.r[1].s64 + -16;
	// 8328F72C: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 8328F730: 3CA08332  lis r5, -0x7cce
	ctx.r[5].s64 = -2093875200;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328F778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8328F778 size=100
    let mut pc: u32 = 0x8328F778;
    'dispatch: loop {
        match pc {
            0x8328F778 => {
    //   block [0x8328F778..0x8328F7DC)
	// 8328F778: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8328F77C: 3941FFF4  addi r10, r1, -0xc
	ctx.r[10].s64 = ctx.r[1].s64 + -12;
	// 8328F780: 392B9490  addi r9, r11, -0x6b70
	ctx.r[9].s64 = ctx.r[11].s64 + -27504;
	// 8328F784: 3901FFF8  addi r8, r1, -8
	ctx.r[8].s64 = ctx.r[1].s64 + -8;
	// 8328F788: 38E1FFF8  addi r7, r1, -8
	ctx.r[7].s64 = ctx.r[1].s64 + -8;
	// 8328F78C: C00B9490  lfs f0, -0x6b70(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8328F790: 38C1FFF0  addi r6, r1, -0x10
	ctx.r[6].s64 = ctx.r[1].s64 + -16;
	// 8328F794: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 8328F798: 3CA0834A  lis r5, -0x7cb6
	ctx.r[5].s64 = -2092302336;
	// 8328F79C: C009FFF4  lfs f0, -0xc(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8328F7A0: D001FFF4  stfs f0, -0xc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), tmp.u32 ) };
	// 8328F7A4: 38855070  addi r4, r5, 0x5070
	ctx.r[4].s64 = ctx.r[5].s64 + 20592;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328F7E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328F7E0 size=64
    let mut pc: u32 = 0x8328F7E0;
    'dispatch: loop {
        match pc {
            0x8328F7E0 => {
    //   block [0x8328F7E0..0x8328F820)
	// 8328F7E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328F7E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328F7E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328F7EC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8328F7F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328F7F4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8328F7F8: 386A5080  addi r3, r10, 0x5080
	ctx.r[3].s64 = ctx.r[10].s64 + 20608;
	// 8328F7FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328F800: 4AF9D6D1  bl 0x8222ced0
	ctx.lr = 0x8328F804;
	sub_8222CED0(ctx, base);
	// 8328F804: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328F808: 38695848  addi r3, r9, 0x5848
	ctx.r[3].s64 = ctx.r[9].s64 + 22600;
	// 8328F80C: 4BA1A715  bl 0x82ca9f20
	ctx.lr = 0x8328F810;
	sub_82CA9F20(ctx, base);
	// 8328F810: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328F814: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328F818: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328F81C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328F820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328F820 size=64
    let mut pc: u32 = 0x8328F820;
    'dispatch: loop {
        match pc {
            0x8328F820 => {
    //   block [0x8328F820..0x8328F860)
	// 8328F820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328F824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328F828: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328F82C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8328F830: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328F834: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8328F838: 386A5084  addi r3, r10, 0x5084
	ctx.r[3].s64 = ctx.r[10].s64 + 20612;
	// 8328F83C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328F840: 4AF9D691  bl 0x8222ced0
	ctx.lr = 0x8328F844;
	sub_8222CED0(ctx, base);
	// 8328F844: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328F848: 38695858  addi r3, r9, 0x5858
	ctx.r[3].s64 = ctx.r[9].s64 + 22616;
	// 8328F84C: 4BA1A6D5  bl 0x82ca9f20
	ctx.lr = 0x8328F850;
	sub_82CA9F20(ctx, base);
	// 8328F850: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328F854: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328F858: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328F85C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328F860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8328F860 size=88
    let mut pc: u32 = 0x8328F860;
    'dispatch: loop {
        match pc {
            0x8328F860 => {
    //   block [0x8328F860..0x8328F8B8)
	// 8328F860: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8328F864: 3941FFF0  addi r10, r1, -0x10
	ctx.r[10].s64 = ctx.r[1].s64 + -16;
	// 8328F868: 3921FFF4  addi r9, r1, -0xc
	ctx.r[9].s64 = ctx.r[1].s64 + -12;
	// 8328F86C: 3901FFF4  addi r8, r1, -0xc
	ctx.r[8].s64 = ctx.r[1].s64 + -12;
	// 8328F870: 38E1FFF0  addi r7, r1, -0x10
	ctx.r[7].s64 = ctx.r[1].s64 + -16;
	// 8328F874: C00B9490  lfs f0, -0x6b70(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8328F878: 3CC0834A  lis r6, -0x7cb6
	ctx.r[6].s64 = -2092302336;
	// 8328F87C: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328F8B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8328F8B8 size=88
    let mut pc: u32 = 0x8328F8B8;
    'dispatch: loop {
        match pc {
            0x8328F8B8 => {
    //   block [0x8328F8B8..0x8328F910)
	// 8328F8B8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8328F8BC: 3941FFF0  addi r10, r1, -0x10
	ctx.r[10].s64 = ctx.r[1].s64 + -16;
	// 8328F8C0: 3921FFF4  addi r9, r1, -0xc
	ctx.r[9].s64 = ctx.r[1].s64 + -12;
	// 8328F8C4: 3901FFF4  addi r8, r1, -0xc
	ctx.r[8].s64 = ctx.r[1].s64 + -12;
	// 8328F8C8: 38E1FFF0  addi r7, r1, -0x10
	ctx.r[7].s64 = ctx.r[1].s64 + -16;
	// 8328F8CC: C00B0C68  lfs f0, 0xc68(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3176 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8328F8D0: 3CC0834A  lis r6, -0x7cb6
	ctx.r[6].s64 = -2092302336;
	// 8328F8D4: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328F910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8328F910 size=88
    let mut pc: u32 = 0x8328F910;
    'dispatch: loop {
        match pc {
            0x8328F910 => {
    //   block [0x8328F910..0x8328F968)
	// 8328F910: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8328F914: 3941FFF0  addi r10, r1, -0x10
	ctx.r[10].s64 = ctx.r[1].s64 + -16;
	// 8328F918: 3921FFF4  addi r9, r1, -0xc
	ctx.r[9].s64 = ctx.r[1].s64 + -12;
	// 8328F91C: 3901FFF4  addi r8, r1, -0xc
	ctx.r[8].s64 = ctx.r[1].s64 + -12;
	// 8328F920: 38E1FFF0  addi r7, r1, -0x10
	ctx.r[7].s64 = ctx.r[1].s64 + -16;
	// 8328F924: C00BB7A4  lfs f0, -0x485c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18524 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8328F928: 3CC0834A  lis r6, -0x7cb6
	ctx.r[6].s64 = -2092302336;
	// 8328F92C: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328F968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8328F968 size=96
    let mut pc: u32 = 0x8328F968;
    'dispatch: loop {
        match pc {
            0x8328F968 => {
    //   block [0x8328F968..0x8328F9C8)
	// 8328F968: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8328F96C: 3901FFF0  addi r8, r1, -0x10
	ctx.r[8].s64 = ctx.r[1].s64 + -16;
	// 8328F970: 392B9484  addi r9, r11, -0x6b7c
	ctx.r[9].s64 = ctx.r[11].s64 + -27516;
	// 8328F974: 3941FFF4  addi r10, r1, -0xc
	ctx.r[10].s64 = ctx.r[1].s64 + -12;
	// 8328F978: 38E1FFF4  addi r7, r1, -0xc
	ctx.r[7].s64 = ctx.r[1].s64 + -12;
	// 8328F97C: C1AB9484  lfs f13, -0x6b7c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8328F980: 38C1FFF0  addi r6, r1, -0x10
	ctx.r[6].s64 = ctx.r[1].s64 + -16;
	// 8328F984: D1A1FFF0  stfs f13, -0x10(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 8328F988: 3CA0834A  lis r5, -0x7cb6
	ctx.r[5].s64 = -2092302336;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328F9C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8328F9C8 size=96
    let mut pc: u32 = 0x8328F9C8;
    'dispatch: loop {
        match pc {
            0x8328F9C8 => {
    //   block [0x8328F9C8..0x8328FA28)
	// 8328F9C8: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8328F9CC: 3941FFF4  addi r10, r1, -0xc
	ctx.r[10].s64 = ctx.r[1].s64 + -12;
	// 8328F9D0: 392B9484  addi r9, r11, -0x6b7c
	ctx.r[9].s64 = ctx.r[11].s64 + -27516;
	// 8328F9D4: 3901FFF8  addi r8, r1, -8
	ctx.r[8].s64 = ctx.r[1].s64 + -8;
	// 8328F9D8: 38E1FFF0  addi r7, r1, -0x10
	ctx.r[7].s64 = ctx.r[1].s64 + -16;
	// 8328F9DC: 38C1FFF8  addi r6, r1, -8
	ctx.r[6].s64 = ctx.r[1].s64 + -8;
	// 8328F9E0: C00B9484  lfs f0, -0x6b7c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8328F9E4: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 8328F9E8: 3CA0834A  lis r5, -0x7cb6
	ctx.r[5].s64 = -2092302336;
	// 8328F9EC: C1A9000C  lfs f13, 0xc(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8328F9F0: D1A1FFF4  stfs f13, -0xc(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), tmp.u32 ) };
	// 8328F9F4: 388550D0  addi r4, r5, 0x50d0
	ctx.r[4].s64 = ctx.r[5].s64 + 20688;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328FA28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328FA28 size=28
    let mut pc: u32 = 0x8328FA28;
    'dispatch: loop {
        match pc {
            0x8328FA28 => {
    //   block [0x8328FA28..0x8328FA44)
	// 8328FA28: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8328FA2C: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328FA30: 392B9360  addi r9, r11, -0x6ca0
	ctx.r[9].s64 = ctx.r[11].s64 + -27808;
	// 8328FA34: 390A50E0  addi r8, r10, 0x50e0
	ctx.r[8].s64 = ctx.r[10].s64 + 20704;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328FA48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328FA48 size=28
    let mut pc: u32 = 0x8328FA48;
    'dispatch: loop {
        match pc {
            0x8328FA48 => {
    //   block [0x8328FA48..0x8328FA64)
	// 8328FA48: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8328FA4C: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328FA50: 392B9370  addi r9, r11, -0x6c90
	ctx.r[9].s64 = ctx.r[11].s64 + -27792;
	// 8328FA54: 390A50F0  addi r8, r10, 0x50f0
	ctx.r[8].s64 = ctx.r[10].s64 + 20720;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328FA68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328FA68 size=28
    let mut pc: u32 = 0x8328FA68;
    'dispatch: loop {
        match pc {
            0x8328FA68 => {
    //   block [0x8328FA68..0x8328FA84)
	// 8328FA68: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8328FA6C: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328FA70: 392B9380  addi r9, r11, -0x6c80
	ctx.r[9].s64 = ctx.r[11].s64 + -27776;
	// 8328FA74: 390A5100  addi r8, r10, 0x5100
	ctx.r[8].s64 = ctx.r[10].s64 + 20736;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328FA88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328FA88 size=28
    let mut pc: u32 = 0x8328FA88;
    'dispatch: loop {
        match pc {
            0x8328FA88 => {
    //   block [0x8328FA88..0x8328FAA4)
	// 8328FA88: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8328FA8C: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328FA90: 392B9390  addi r9, r11, -0x6c70
	ctx.r[9].s64 = ctx.r[11].s64 + -27760;
	// 8328FA94: 390A5110  addi r8, r10, 0x5110
	ctx.r[8].s64 = ctx.r[10].s64 + 20752;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328FAA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328FAA8 size=12
    let mut pc: u32 = 0x8328FAA8;
    'dispatch: loop {
        match pc {
            0x8328FAA8 => {
    //   block [0x8328FAA8..0x8328FAB4)
	// 8328FAA8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8328FAAC: 386B5868  addi r3, r11, 0x5868
	ctx.r[3].s64 = ctx.r[11].s64 + 22632;
	// 8328FAB0: 4BA1A470  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328FAB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328FAB8 size=12
    let mut pc: u32 = 0x8328FAB8;
    'dispatch: loop {
        match pc {
            0x8328FAB8 => {
    //   block [0x8328FAB8..0x8328FAC4)
	// 8328FAB8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8328FABC: 386B58A8  addi r3, r11, 0x58a8
	ctx.r[3].s64 = ctx.r[11].s64 + 22696;
	// 8328FAC0: 4BA1A460  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328FAC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328FAC8 size=12
    let mut pc: u32 = 0x8328FAC8;
    'dispatch: loop {
        match pc {
            0x8328FAC8 => {
    //   block [0x8328FAC8..0x8328FAD4)
	// 8328FAC8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8328FACC: 386B58E8  addi r3, r11, 0x58e8
	ctx.r[3].s64 = ctx.r[11].s64 + 22760;
	// 8328FAD0: 4BA1A450  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328FAD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328FAD8 size=12
    let mut pc: u32 = 0x8328FAD8;
    'dispatch: loop {
        match pc {
            0x8328FAD8 => {
    //   block [0x8328FAD8..0x8328FAE4)
	// 8328FAD8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8328FADC: 386B5928  addi r3, r11, 0x5928
	ctx.r[3].s64 = ctx.r[11].s64 + 22824;
	// 8328FAE0: 4BA1A440  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328FAE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328FAE8 size=12
    let mut pc: u32 = 0x8328FAE8;
    'dispatch: loop {
        match pc {
            0x8328FAE8 => {
    //   block [0x8328FAE8..0x8328FAF4)
	// 8328FAE8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8328FAEC: 386B5968  addi r3, r11, 0x5968
	ctx.r[3].s64 = ctx.r[11].s64 + 22888;
	// 8328FAF0: 4BA1A430  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328FAF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328FAF8 size=40
    let mut pc: u32 = 0x8328FAF8;
    'dispatch: loop {
        match pc {
            0x8328FAF8 => {
    //   block [0x8328FAF8..0x8328FB20)
	// 8328FAF8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8328FAFC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8328FB00: 396B5144  addi r11, r11, 0x5144
	ctx.r[11].s64 = ctx.r[11].s64 + 20804;
	// 8328FB04: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328FB08: 386959A8  addi r3, r9, 0x59a8
	ctx.r[3].s64 = ctx.r[9].s64 + 22952;
	// 8328FB0C: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8328FB10: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8328FB14: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8328FB18: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8328FB1C: 4BA1A404  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328FB20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328FB20 size=12
    let mut pc: u32 = 0x8328FB20;
    'dispatch: loop {
        match pc {
            0x8328FB20 => {
    //   block [0x8328FB20..0x8328FB2C)
	// 8328FB20: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8328FB24: 386B59C0  addi r3, r11, 0x59c0
	ctx.r[3].s64 = ctx.r[11].s64 + 22976;
	// 8328FB28: 4BA1A3F8  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328FB30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328FB30 size=144
    let mut pc: u32 = 0x8328FB30;
    'dispatch: loop {
        match pc {
            0x8328FB30 => {
    //   block [0x8328FB30..0x8328FB54)
	// 8328FB30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328FB34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328FB38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328FB3C: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 8328FB40: 4AF8F719  bl 0x8221f258
	ctx.lr = 0x8328FB44;
	sub_8221F258(ctx, base);
	// 8328FB44: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8328FB48: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8328FB4C: 419A0008  beq cr6, 0x8328fb54
	if ctx.cr[6].eq {
	pc = 0x8328FB54; continue 'dispatch;
	}
	// 8328FB50: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8328FB54; continue 'dispatch;
            }
            0x8328FB54 => {
    //   block [0x8328FB54..0x8328FB60)
	// 8328FB54: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8328FB58: 41820008  beq 0x8328fb60
	if ctx.cr[0].eq {
	pc = 0x8328FB60; continue 'dispatch;
	}
	// 8328FB5C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8328FB60; continue 'dispatch;
            }
            0x8328FB60 => {
    //   block [0x8328FB60..0x8328FB6C)
	// 8328FB60: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8328FB64: 41820008  beq 0x8328fb6c
	if ctx.cr[0].eq {
	pc = 0x8328FB6C; continue 'dispatch;
	}
	// 8328FB68: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8328FB6C; continue 'dispatch;
            }
            0x8328FB6C => {
    //   block [0x8328FB6C..0x8328FBC0)
	// 8328FB6C: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8328FB70: 9943001D  stb r10, 0x1d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(29 as u32), ctx.r[10].u8 ) };
	// 8328FB74: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8328FB78: 39095158  addi r8, r9, 0x5158
	ctx.r[8].s64 = ctx.r[9].s64 + 20824;
	// 8328FB7C: 9963001C  stb r11, 0x1c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u8 ) };
	// 8328FB80: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8328FB84: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8328FB88: 9963001D  stb r11, 0x1d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(29 as u32), ctx.r[11].u8 ) };
	// 8328FB8C: 38675A08  addi r3, r7, 0x5a08
	ctx.r[3].s64 = ctx.r[7].s64 + 23048;
	// 8328FB90: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8328FB94: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8328FB98: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8328FB9C: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8328FBA0: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8328FBA4: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8328FBA8: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8328FBAC: 4BA1A375  bl 0x82ca9f20
	ctx.lr = 0x8328FBB0;
	sub_82CA9F20(ctx, base);
	// 8328FBB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328FBB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328FBB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328FBBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328FBC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328FBC0 size=144
    let mut pc: u32 = 0x8328FBC0;
    'dispatch: loop {
        match pc {
            0x8328FBC0 => {
    //   block [0x8328FBC0..0x8328FBE4)
	// 8328FBC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328FBC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328FBC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328FBCC: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 8328FBD0: 4AF8F689  bl 0x8221f258
	ctx.lr = 0x8328FBD4;
	sub_8221F258(ctx, base);
	// 8328FBD4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8328FBD8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8328FBDC: 419A0008  beq cr6, 0x8328fbe4
	if ctx.cr[6].eq {
	pc = 0x8328FBE4; continue 'dispatch;
	}
	// 8328FBE0: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8328FBE4; continue 'dispatch;
            }
            0x8328FBE4 => {
    //   block [0x8328FBE4..0x8328FBF0)
	// 8328FBE4: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8328FBE8: 41820008  beq 0x8328fbf0
	if ctx.cr[0].eq {
	pc = 0x8328FBF0; continue 'dispatch;
	}
	// 8328FBEC: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8328FBF0; continue 'dispatch;
            }
            0x8328FBF0 => {
    //   block [0x8328FBF0..0x8328FBFC)
	// 8328FBF0: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8328FBF4: 41820008  beq 0x8328fbfc
	if ctx.cr[0].eq {
	pc = 0x8328FBFC; continue 'dispatch;
	}
	// 8328FBF8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8328FBFC; continue 'dispatch;
            }
            0x8328FBFC => {
    //   block [0x8328FBFC..0x8328FC50)
	// 8328FBFC: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8328FC00: 9943001D  stb r10, 0x1d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(29 as u32), ctx.r[10].u8 ) };
	// 8328FC04: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8328FC08: 39095164  addi r8, r9, 0x5164
	ctx.r[8].s64 = ctx.r[9].s64 + 20836;
	// 8328FC0C: 9963001C  stb r11, 0x1c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u8 ) };
	// 8328FC10: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8328FC14: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8328FC18: 9963001D  stb r11, 0x1d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(29 as u32), ctx.r[11].u8 ) };
	// 8328FC1C: 38675A18  addi r3, r7, 0x5a18
	ctx.r[3].s64 = ctx.r[7].s64 + 23064;
	// 8328FC20: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8328FC24: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8328FC28: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8328FC2C: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8328FC30: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8328FC34: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8328FC38: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8328FC3C: 4BA1A2E5  bl 0x82ca9f20
	ctx.lr = 0x8328FC40;
	sub_82CA9F20(ctx, base);
	// 8328FC40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328FC44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328FC48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328FC4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328FC50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328FC50 size=144
    let mut pc: u32 = 0x8328FC50;
    'dispatch: loop {
        match pc {
            0x8328FC50 => {
    //   block [0x8328FC50..0x8328FC74)
	// 8328FC50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328FC54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328FC58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328FC5C: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 8328FC60: 4AF8F5F9  bl 0x8221f258
	ctx.lr = 0x8328FC64;
	sub_8221F258(ctx, base);
	// 8328FC64: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8328FC68: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8328FC6C: 419A0008  beq cr6, 0x8328fc74
	if ctx.cr[6].eq {
	pc = 0x8328FC74; continue 'dispatch;
	}
	// 8328FC70: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8328FC74; continue 'dispatch;
            }
            0x8328FC74 => {
    //   block [0x8328FC74..0x8328FC80)
	// 8328FC74: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8328FC78: 41820008  beq 0x8328fc80
	if ctx.cr[0].eq {
	pc = 0x8328FC80; continue 'dispatch;
	}
	// 8328FC7C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8328FC80; continue 'dispatch;
            }
            0x8328FC80 => {
    //   block [0x8328FC80..0x8328FC8C)
	// 8328FC80: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8328FC84: 41820008  beq 0x8328fc8c
	if ctx.cr[0].eq {
	pc = 0x8328FC8C; continue 'dispatch;
	}
	// 8328FC88: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8328FC8C; continue 'dispatch;
            }
            0x8328FC8C => {
    //   block [0x8328FC8C..0x8328FCE0)
	// 8328FC8C: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8328FC90: 9943001D  stb r10, 0x1d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(29 as u32), ctx.r[10].u8 ) };
	// 8328FC94: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8328FC98: 39095170  addi r8, r9, 0x5170
	ctx.r[8].s64 = ctx.r[9].s64 + 20848;
	// 8328FC9C: 9963001C  stb r11, 0x1c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u8 ) };
	// 8328FCA0: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8328FCA4: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8328FCA8: 9963001D  stb r11, 0x1d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(29 as u32), ctx.r[11].u8 ) };
	// 8328FCAC: 38675A28  addi r3, r7, 0x5a28
	ctx.r[3].s64 = ctx.r[7].s64 + 23080;
	// 8328FCB0: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8328FCB4: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8328FCB8: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8328FCBC: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8328FCC0: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8328FCC4: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8328FCC8: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8328FCCC: 4BA1A255  bl 0x82ca9f20
	ctx.lr = 0x8328FCD0;
	sub_82CA9F20(ctx, base);
	// 8328FCD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328FCD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328FCD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328FCDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328FCE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328FCE0 size=144
    let mut pc: u32 = 0x8328FCE0;
    'dispatch: loop {
        match pc {
            0x8328FCE0 => {
    //   block [0x8328FCE0..0x8328FD04)
	// 8328FCE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328FCE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328FCE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328FCEC: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 8328FCF0: 4AF8F569  bl 0x8221f258
	ctx.lr = 0x8328FCF4;
	sub_8221F258(ctx, base);
	// 8328FCF4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8328FCF8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8328FCFC: 419A0008  beq cr6, 0x8328fd04
	if ctx.cr[6].eq {
	pc = 0x8328FD04; continue 'dispatch;
	}
	// 8328FD00: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8328FD04; continue 'dispatch;
            }
            0x8328FD04 => {
    //   block [0x8328FD04..0x8328FD10)
	// 8328FD04: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8328FD08: 41820008  beq 0x8328fd10
	if ctx.cr[0].eq {
	pc = 0x8328FD10; continue 'dispatch;
	}
	// 8328FD0C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8328FD10; continue 'dispatch;
            }
            0x8328FD10 => {
    //   block [0x8328FD10..0x8328FD1C)
	// 8328FD10: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8328FD14: 41820008  beq 0x8328fd1c
	if ctx.cr[0].eq {
	pc = 0x8328FD1C; continue 'dispatch;
	}
	// 8328FD18: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8328FD1C; continue 'dispatch;
            }
            0x8328FD1C => {
    //   block [0x8328FD1C..0x8328FD70)
	// 8328FD1C: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8328FD20: 9943001D  stb r10, 0x1d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(29 as u32), ctx.r[10].u8 ) };
	// 8328FD24: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8328FD28: 3909517C  addi r8, r9, 0x517c
	ctx.r[8].s64 = ctx.r[9].s64 + 20860;
	// 8328FD2C: 9963001C  stb r11, 0x1c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u8 ) };
	// 8328FD30: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8328FD34: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8328FD38: 9963001D  stb r11, 0x1d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(29 as u32), ctx.r[11].u8 ) };
	// 8328FD3C: 38675A38  addi r3, r7, 0x5a38
	ctx.r[3].s64 = ctx.r[7].s64 + 23096;
	// 8328FD40: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8328FD44: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8328FD48: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8328FD4C: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8328FD50: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8328FD54: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8328FD58: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8328FD5C: 4BA1A1C5  bl 0x82ca9f20
	ctx.lr = 0x8328FD60;
	sub_82CA9F20(ctx, base);
	// 8328FD60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328FD64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328FD68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328FD6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328FD70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328FD70 size=144
    let mut pc: u32 = 0x8328FD70;
    'dispatch: loop {
        match pc {
            0x8328FD70 => {
    //   block [0x8328FD70..0x8328FD94)
	// 8328FD70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328FD74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328FD78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328FD7C: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 8328FD80: 4AF8F4D9  bl 0x8221f258
	ctx.lr = 0x8328FD84;
	sub_8221F258(ctx, base);
	// 8328FD84: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8328FD88: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8328FD8C: 419A0008  beq cr6, 0x8328fd94
	if ctx.cr[6].eq {
	pc = 0x8328FD94; continue 'dispatch;
	}
	// 8328FD90: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8328FD94; continue 'dispatch;
            }
            0x8328FD94 => {
    //   block [0x8328FD94..0x8328FDA0)
	// 8328FD94: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8328FD98: 41820008  beq 0x8328fda0
	if ctx.cr[0].eq {
	pc = 0x8328FDA0; continue 'dispatch;
	}
	// 8328FD9C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8328FDA0; continue 'dispatch;
            }
            0x8328FDA0 => {
    //   block [0x8328FDA0..0x8328FDAC)
	// 8328FDA0: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8328FDA4: 41820008  beq 0x8328fdac
	if ctx.cr[0].eq {
	pc = 0x8328FDAC; continue 'dispatch;
	}
	// 8328FDA8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8328FDAC; continue 'dispatch;
            }
            0x8328FDAC => {
    //   block [0x8328FDAC..0x8328FE00)
	// 8328FDAC: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8328FDB0: 9943001D  stb r10, 0x1d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(29 as u32), ctx.r[10].u8 ) };
	// 8328FDB4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8328FDB8: 39095188  addi r8, r9, 0x5188
	ctx.r[8].s64 = ctx.r[9].s64 + 20872;
	// 8328FDBC: 9963001C  stb r11, 0x1c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u8 ) };
	// 8328FDC0: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8328FDC4: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8328FDC8: 9963001D  stb r11, 0x1d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(29 as u32), ctx.r[11].u8 ) };
	// 8328FDCC: 38675A48  addi r3, r7, 0x5a48
	ctx.r[3].s64 = ctx.r[7].s64 + 23112;
	// 8328FDD0: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8328FDD4: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8328FDD8: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8328FDDC: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8328FDE0: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8328FDE4: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8328FDE8: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8328FDEC: 4BA1A135  bl 0x82ca9f20
	ctx.lr = 0x8328FDF0;
	sub_82CA9F20(ctx, base);
	// 8328FDF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328FDF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328FDF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328FDFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328FE00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328FE00 size=64
    let mut pc: u32 = 0x8328FE00;
    'dispatch: loop {
        match pc {
            0x8328FE00 => {
    //   block [0x8328FE00..0x8328FE40)
	// 8328FE00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328FE04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328FE08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328FE0C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8328FE10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328FE14: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8328FE18: 386A5194  addi r3, r10, 0x5194
	ctx.r[3].s64 = ctx.r[10].s64 + 20884;
	// 8328FE1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328FE20: 4AF9D0B1  bl 0x8222ced0
	ctx.lr = 0x8328FE24;
	sub_8222CED0(ctx, base);
	// 8328FE24: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328FE28: 38695A58  addi r3, r9, 0x5a58
	ctx.r[3].s64 = ctx.r[9].s64 + 23128;
	// 8328FE2C: 4BA1A0F5  bl 0x82ca9f20
	ctx.lr = 0x8328FE30;
	sub_82CA9F20(ctx, base);
	// 8328FE30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328FE34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328FE38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328FE3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328FE40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328FE40 size=64
    let mut pc: u32 = 0x8328FE40;
    'dispatch: loop {
        match pc {
            0x8328FE40 => {
    //   block [0x8328FE40..0x8328FE80)
	// 8328FE40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328FE44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328FE48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328FE4C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8328FE50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328FE54: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8328FE58: 386A5198  addi r3, r10, 0x5198
	ctx.r[3].s64 = ctx.r[10].s64 + 20888;
	// 8328FE5C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328FE60: 4AF9D071  bl 0x8222ced0
	ctx.lr = 0x8328FE64;
	sub_8222CED0(ctx, base);
	// 8328FE64: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328FE68: 38695A68  addi r3, r9, 0x5a68
	ctx.r[3].s64 = ctx.r[9].s64 + 23144;
	// 8328FE6C: 4BA1A0B5  bl 0x82ca9f20
	ctx.lr = 0x8328FE70;
	sub_82CA9F20(ctx, base);
	// 8328FE70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328FE74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328FE78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328FE7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328FE80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328FE80 size=444
    let mut pc: u32 = 0x8328FE80;
    'dispatch: loop {
        match pc {
            0x8328FE80 => {
    //   block [0x8328FE80..0x8329003C)
	// 8328FE80: 3D00834A  lis r8, -0x7cb6
	ctx.r[8].s64 = -2092302336;
	// 8328FE84: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8328FE88: 38C851A0  addi r6, r8, 0x51a0
	ctx.r[6].s64 = ctx.r[8].s64 + 20896;
	// 8328FE8C: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 8328FE90: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 8328FE94: 916851A0  stw r11, 0x51a0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(20896 as u32), ctx.r[11].u32 ) };
	// 8328FE98: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8328FE9C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8328FEA0: 91460004  stw r10, 4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8328FEA4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8328FEA8: 91260008  stw r9, 8(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8328FEAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8328FEB0: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8328FEB4: 91660010  stw r11, 0x10(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8328FEB8: 91460018  stw r10, 0x18(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 8328FEBC: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8328FEC0: 9926001C  stb r9, 0x1c(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(28 as u32), ctx.r[9].u8 ) };
	// 8328FEC4: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 8328FEC8: 9106000C  stw r8, 0xc(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 8328FECC: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8328FED0: 90E60014  stw r7, 0x14(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 8328FED4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8328FED8: 38E0FFFF  li r7, -1
	ctx.r[7].s64 = -1;
	// 8328FEDC: 91660024  stw r11, 0x24(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 8328FEE0: 9146002C  stw r10, 0x2c(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(44 as u32), ctx.r[10].u32 ) };
	// 8328FEE4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8328FEE8: 91260030  stw r9, 0x30(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(48 as u32), ctx.r[9].u32 ) };
	// 8328FEEC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8328FEF0: 91060020  stw r8, 0x20(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(32 as u32), ctx.r[8].u32 ) };
	// 8328FEF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8328FEF8: 90E60028  stw r7, 0x28(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(40 as u32), ctx.r[7].u32 ) };
	// 8328FEFC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8328FF00: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8328FF04: 91660038  stw r11, 0x38(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 8328FF08: 99460040  stb r10, 0x40(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(64 as u32), ctx.r[10].u8 ) };
	// 8328FF0C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8328FF10: 91260044  stw r9, 0x44(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(68 as u32), ctx.r[9].u32 ) };
	// 8328FF14: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8328FF18: 91060034  stw r8, 0x34(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(52 as u32), ctx.r[8].u32 ) };
	// 8328FF1C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8328FF20: 90E6003C  stw r7, 0x3c(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(60 as u32), ctx.r[7].u32 ) };
	// 8328FF24: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 8328FF28: 38E0FFFF  li r7, -1
	ctx.r[7].s64 = -1;
	// 8328FF2C: 9166004C  stw r11, 0x4c(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 8328FF30: 91460054  stw r10, 0x54(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 8328FF34: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8328FF38: 91260058  stw r9, 0x58(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 8328FF3C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8328FF40: 91060048  stw r8, 0x48(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(72 as u32), ctx.r[8].u32 ) };
	// 8328FF44: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 8328FF48: 90E60050  stw r7, 0x50(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(80 as u32), ctx.r[7].u32 ) };
	// 8328FF4C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8328FF50: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8328FF54: 91660060  stw r11, 0x60(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 8328FF58: 91460068  stw r10, 0x68(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 8328FF5C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8328FF60: 9126006C  stw r9, 0x6c(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8328FF64: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8328FF68: 9106005C  stw r8, 0x5c(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8328FF6C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8328FF70: 98E60064  stb r7, 0x64(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(100 as u32), ctx.r[7].u8 ) };
	// 8328FF74: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 8328FF78: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8328FF7C: 91660074  stw r11, 0x74(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8328FF80: 9146007C  stw r10, 0x7c(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(124 as u32), ctx.r[10].u32 ) };
	// 8328FF84: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8328FF88: 91260080  stw r9, 0x80(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(128 as u32), ctx.r[9].u32 ) };
	// 8328FF8C: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 8328FF90: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 8328FF94: 91060070  stw r8, 0x70(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(112 as u32), ctx.r[8].u32 ) };
	// 8328FF98: 90E60078  stw r7, 0x78(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(120 as u32), ctx.r[7].u32 ) };
	// 8328FF9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8328FFA0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8328FFA4: 99660088  stb r11, 0x88(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(136 as u32), ctx.r[11].u8 ) };
	// 8328FFA8: 91460090  stw r10, 0x90(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(144 as u32), ctx.r[10].u32 ) };
	// 8328FFAC: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8328FFB0: 91260094  stw r9, 0x94(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(148 as u32), ctx.r[9].u32 ) };
	// 8328FFB4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8328FFB8: 91060084  stw r8, 0x84(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(132 as u32), ctx.r[8].u32 ) };
	// 8328FFBC: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8328FFC0: 90E6008C  stw r7, 0x8c(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(140 as u32), ctx.r[7].u32 ) };
	// 8328FFC4: 91660098  stw r11, 0x98(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(152 as u32), ctx.r[11].u32 ) };
	// 8328FFC8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8328FFCC: 9146009C  stw r10, 0x9c(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(156 as u32), ctx.r[10].u32 ) };
	// 8328FFD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8328FFD4: 912600A0  stw r9, 0xa0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(160 as u32), ctx.r[9].u32 ) };
	// 8328FFD8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8328FFDC: 916600A4  stw r11, 0xa4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(164 as u32), ctx.r[11].u32 ) };
	// 8328FFE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8328FFE4: 914600A8  stw r10, 0xa8(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(168 as u32), ctx.r[10].u32 ) };
	// 8328FFE8: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 8328FFEC: 992600AC  stb r9, 0xac(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(172 as u32), ctx.r[9].u8 ) };
	// 8328FFF0: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 8328FFF4: 916600B0  stw r11, 0xb0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 8328FFF8: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8328FFFC: 914600B4  stw r10, 0xb4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(180 as u32), ctx.r[10].u32 ) };
	// 83290000: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83290004: 912600B8  stw r9, 0xb8(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(184 as u32), ctx.r[9].u32 ) };
	// 83290008: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8329000C: 916600BC  stw r11, 0xbc(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(188 as u32), ctx.r[11].u32 ) };
	// 83290010: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83290014: 914600C0  stw r10, 0xc0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(192 as u32), ctx.r[10].u32 ) };
	// 83290018: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329001C: 912600C4  stw r9, 0xc4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(196 as u32), ctx.r[9].u32 ) };
	// 83290020: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83290024: 916600C8  stw r11, 0xc8(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 83290028: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329002C: 914600CC  stw r10, 0xcc(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(204 as u32), ctx.r[10].u32 ) };
	// 83290030: 992600D0  stb r9, 0xd0(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(208 as u32), ctx.r[9].u8 ) };
	// 83290034: 916600D4  stw r11, 0xd4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(212 as u32), ctx.r[11].u32 ) };
	// 83290038: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83290040 size=64
    let mut pc: u32 = 0x83290040;
    'dispatch: loop {
        match pc {
            0x83290040 => {
    //   block [0x83290040..0x83290080)
	// 83290040: 3D00834A  lis r8, -0x7cb6
	ctx.r[8].s64 = -2092302336;
	// 83290044: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83290048: 38C85278  addi r6, r8, 0x5278
	ctx.r[6].s64 = ctx.r[8].s64 + 21112;
	// 8329004C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83290050: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83290054: 91685278  stw r11, 0x5278(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(21112 as u32), ctx.r[11].u32 ) };
	// 83290058: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329005C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83290060: 3CA0832B  lis r5, -0x7cd5
	ctx.r[5].s64 = -2094333952;
	// 83290064: 91460004  stw r10, 4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83290068: 91260008  stw r9, 8(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8329006C: 9106000C  stw r8, 0xc(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 83290070: 38655A78  addi r3, r5, 0x5a78
	ctx.r[3].s64 = ctx.r[5].s64 + 23160;
	// 83290074: 91660010  stw r11, 0x10(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83290078: 90E60014  stw r7, 0x14(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 8329007C: 4BA19EA4  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290080 size=64
    let mut pc: u32 = 0x83290080;
    'dispatch: loop {
        match pc {
            0x83290080 => {
    //   block [0x83290080..0x832900C0)
	// 83290080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83290084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290088: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329008C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83290090: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83290094: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83290098: 386A5290  addi r3, r10, 0x5290
	ctx.r[3].s64 = ctx.r[10].s64 + 21136;
	// 8329009C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832900A0: 4AF9CE31  bl 0x8222ced0
	ctx.lr = 0x832900A4;
	sub_8222CED0(ctx, base);
	// 832900A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832900A8: 38695A80  addi r3, r9, 0x5a80
	ctx.r[3].s64 = ctx.r[9].s64 + 23168;
	// 832900AC: 4BA19E75  bl 0x82ca9f20
	ctx.lr = 0x832900B0;
	sub_82CA9F20(ctx, base);
	// 832900B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832900B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832900B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832900BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832900C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832900C0 size=64
    let mut pc: u32 = 0x832900C0;
    'dispatch: loop {
        match pc {
            0x832900C0 => {
    //   block [0x832900C0..0x83290100)
	// 832900C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832900C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832900C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832900CC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832900D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832900D4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 832900D8: 386A5294  addi r3, r10, 0x5294
	ctx.r[3].s64 = ctx.r[10].s64 + 21140;
	// 832900DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832900E0: 4AF9CDF1  bl 0x8222ced0
	ctx.lr = 0x832900E4;
	sub_8222CED0(ctx, base);
	// 832900E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832900E8: 38695A90  addi r3, r9, 0x5a90
	ctx.r[3].s64 = ctx.r[9].s64 + 23184;
	// 832900EC: 4BA19E35  bl 0x82ca9f20
	ctx.lr = 0x832900F0;
	sub_82CA9F20(ctx, base);
	// 832900F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832900F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832900F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832900FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290100 size=52
    let mut pc: u32 = 0x83290100;
    'dispatch: loop {
        match pc {
            0x83290100 => {
    //   block [0x83290100..0x83290134)
	// 83290100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83290104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290108: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329010C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83290110: 386B5298  addi r3, r11, 0x5298
	ctx.r[3].s64 = ctx.r[11].s64 + 21144;
	// 83290114: 4AFFBC45  bl 0x8228bd58
	ctx.lr = 0x83290118;
	sub_8228BD58(ctx, base);
	// 83290118: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 8329011C: 386A5AA0  addi r3, r10, 0x5aa0
	ctx.r[3].s64 = ctx.r[10].s64 + 23200;
	// 83290120: 4BA19E01  bl 0x82ca9f20
	ctx.lr = 0x83290124;
	sub_82CA9F20(ctx, base);
	// 83290124: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83290128: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329012C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83290130: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290138 size=76
    let mut pc: u32 = 0x83290138;
    'dispatch: loop {
        match pc {
            0x83290138 => {
    //   block [0x83290138..0x83290184)
	// 83290138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329013C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290140: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83290144: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83290148: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8329014C: 388B7F00  addi r4, r11, 0x7f00
	ctx.r[4].s64 = ctx.r[11].s64 + 32512;
	// 83290150: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83290154: 4AF9CD7D  bl 0x8222ced0
	ctx.lr = 0x83290158;
	sub_8222CED0(ctx, base);
	// 83290158: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8329015C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83290160: 386A5654  addi r3, r10, 0x5654
	ctx.r[3].s64 = ctx.r[10].s64 + 22100;
	// 83290164: 4B01B3A5  bl 0x822ab508
	ctx.lr = 0x83290168;
	sub_822AB508(ctx, base);
	// 83290168: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8329016C: 38695AB0  addi r3, r9, 0x5ab0
	ctx.r[3].s64 = ctx.r[9].s64 + 23216;
	// 83290170: 4BA19DB1  bl 0x82ca9f20
	ctx.lr = 0x83290174;
	sub_82CA9F20(ctx, base);
	// 83290174: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83290178: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329017C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83290180: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290188 size=64
    let mut pc: u32 = 0x83290188;
    'dispatch: loop {
        match pc {
            0x83290188 => {
    //   block [0x83290188..0x832901C8)
	// 83290188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329018C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290190: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83290194: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83290198: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8329019C: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 832901A0: 386A56A0  addi r3, r10, 0x56a0
	ctx.r[3].s64 = ctx.r[10].s64 + 22176;
	// 832901A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832901A8: 4AF9CD29  bl 0x8222ced0
	ctx.lr = 0x832901AC;
	sub_8222CED0(ctx, base);
	// 832901AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832901B0: 38695AC0  addi r3, r9, 0x5ac0
	ctx.r[3].s64 = ctx.r[9].s64 + 23232;
	// 832901B4: 4BA19D6D  bl 0x82ca9f20
	ctx.lr = 0x832901B8;
	sub_82CA9F20(ctx, base);
	// 832901B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832901BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832901C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832901C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832901C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832901C8 size=64
    let mut pc: u32 = 0x832901C8;
    'dispatch: loop {
        match pc {
            0x832901C8 => {
    //   block [0x832901C8..0x83290208)
	// 832901C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832901CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832901D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832901D4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832901D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832901DC: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 832901E0: 386A56A4  addi r3, r10, 0x56a4
	ctx.r[3].s64 = ctx.r[10].s64 + 22180;
	// 832901E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832901E8: 4AF9CCE9  bl 0x8222ced0
	ctx.lr = 0x832901EC;
	sub_8222CED0(ctx, base);
	// 832901EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832901F0: 38695AD0  addi r3, r9, 0x5ad0
	ctx.r[3].s64 = ctx.r[9].s64 + 23248;
	// 832901F4: 4BA19D2D  bl 0x82ca9f20
	ctx.lr = 0x832901F8;
	sub_82CA9F20(ctx, base);
	// 832901F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832901FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83290200: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83290204: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290208 size=64
    let mut pc: u32 = 0x83290208;
    'dispatch: loop {
        match pc {
            0x83290208 => {
    //   block [0x83290208..0x83290248)
	// 83290208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329020C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290210: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83290214: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83290218: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8329021C: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83290220: 386A56A8  addi r3, r10, 0x56a8
	ctx.r[3].s64 = ctx.r[10].s64 + 22184;
	// 83290224: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83290228: 4AF9CCA9  bl 0x8222ced0
	ctx.lr = 0x8329022C;
	sub_8222CED0(ctx, base);
	// 8329022C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83290230: 38695AE0  addi r3, r9, 0x5ae0
	ctx.r[3].s64 = ctx.r[9].s64 + 23264;
	// 83290234: 4BA19CED  bl 0x82ca9f20
	ctx.lr = 0x83290238;
	sub_82CA9F20(ctx, base);
	// 83290238: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8329023C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83290240: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83290244: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290248 size=64
    let mut pc: u32 = 0x83290248;
    'dispatch: loop {
        match pc {
            0x83290248 => {
    //   block [0x83290248..0x83290288)
	// 83290248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329024C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290250: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83290254: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83290258: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8329025C: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83290260: 386A56AC  addi r3, r10, 0x56ac
	ctx.r[3].s64 = ctx.r[10].s64 + 22188;
	// 83290264: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83290268: 4AF9CC69  bl 0x8222ced0
	ctx.lr = 0x8329026C;
	sub_8222CED0(ctx, base);
	// 8329026C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83290270: 38695AF0  addi r3, r9, 0x5af0
	ctx.r[3].s64 = ctx.r[9].s64 + 23280;
	// 83290274: 4BA19CAD  bl 0x82ca9f20
	ctx.lr = 0x83290278;
	sub_82CA9F20(ctx, base);
	// 83290278: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8329027C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83290280: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83290284: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290288 size=64
    let mut pc: u32 = 0x83290288;
    'dispatch: loop {
        match pc {
            0x83290288 => {
    //   block [0x83290288..0x832902C8)
	// 83290288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329028C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290290: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83290294: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83290298: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8329029C: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 832902A0: 386A56B0  addi r3, r10, 0x56b0
	ctx.r[3].s64 = ctx.r[10].s64 + 22192;
	// 832902A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832902A8: 4AF9CC29  bl 0x8222ced0
	ctx.lr = 0x832902AC;
	sub_8222CED0(ctx, base);
	// 832902AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832902B0: 38695B00  addi r3, r9, 0x5b00
	ctx.r[3].s64 = ctx.r[9].s64 + 23296;
	// 832902B4: 4BA19C6D  bl 0x82ca9f20
	ctx.lr = 0x832902B8;
	sub_82CA9F20(ctx, base);
	// 832902B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832902BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832902C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832902C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832902C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832902C8 size=64
    let mut pc: u32 = 0x832902C8;
    'dispatch: loop {
        match pc {
            0x832902C8 => {
    //   block [0x832902C8..0x83290308)
	// 832902C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832902CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832902D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832902D4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832902D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832902DC: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 832902E0: 386A56B4  addi r3, r10, 0x56b4
	ctx.r[3].s64 = ctx.r[10].s64 + 22196;
	// 832902E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832902E8: 4AF9CBE9  bl 0x8222ced0
	ctx.lr = 0x832902EC;
	sub_8222CED0(ctx, base);
	// 832902EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832902F0: 38695B10  addi r3, r9, 0x5b10
	ctx.r[3].s64 = ctx.r[9].s64 + 23312;
	// 832902F4: 4BA19C2D  bl 0x82ca9f20
	ctx.lr = 0x832902F8;
	sub_82CA9F20(ctx, base);
	// 832902F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832902FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83290300: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83290304: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290308 size=64
    let mut pc: u32 = 0x83290308;
    'dispatch: loop {
        match pc {
            0x83290308 => {
    //   block [0x83290308..0x83290348)
	// 83290308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329030C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290310: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83290314: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83290318: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8329031C: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83290320: 386A56B8  addi r3, r10, 0x56b8
	ctx.r[3].s64 = ctx.r[10].s64 + 22200;
	// 83290324: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83290328: 4AF9CBA9  bl 0x8222ced0
	ctx.lr = 0x8329032C;
	sub_8222CED0(ctx, base);
	// 8329032C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83290330: 38695B20  addi r3, r9, 0x5b20
	ctx.r[3].s64 = ctx.r[9].s64 + 23328;
	// 83290334: 4BA19BED  bl 0x82ca9f20
	ctx.lr = 0x83290338;
	sub_82CA9F20(ctx, base);
	// 83290338: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8329033C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83290340: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83290344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290348 size=64
    let mut pc: u32 = 0x83290348;
    'dispatch: loop {
        match pc {
            0x83290348 => {
    //   block [0x83290348..0x83290388)
	// 83290348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329034C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290350: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83290354: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83290358: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8329035C: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83290360: 386A56BC  addi r3, r10, 0x56bc
	ctx.r[3].s64 = ctx.r[10].s64 + 22204;
	// 83290364: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83290368: 4AF9CB69  bl 0x8222ced0
	ctx.lr = 0x8329036C;
	sub_8222CED0(ctx, base);
	// 8329036C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83290370: 38695B30  addi r3, r9, 0x5b30
	ctx.r[3].s64 = ctx.r[9].s64 + 23344;
	// 83290374: 4BA19BAD  bl 0x82ca9f20
	ctx.lr = 0x83290378;
	sub_82CA9F20(ctx, base);
	// 83290378: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8329037C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83290380: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83290384: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290388 size=96
    let mut pc: u32 = 0x83290388;
    'dispatch: loop {
        match pc {
            0x83290388 => {
    //   block [0x83290388..0x832903AC)
	// 83290388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329038C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290390: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83290394: 38600070  li r3, 0x70
	ctx.r[3].s64 = 112;
	// 83290398: 4AF8EEC1  bl 0x8221f258
	ctx.lr = 0x8329039C;
	sub_8221F258(ctx, base);
	// 8329039C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 832903A0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 832903A4: 419A0008  beq cr6, 0x832903ac
	if ctx.cr[6].eq {
	pc = 0x832903AC; continue 'dispatch;
	}
	// 832903A8: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x832903AC; continue 'dispatch;
            }
            0x832903AC => {
    //   block [0x832903AC..0x832903B8)
	// 832903AC: 354B0004  addic. r10, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 832903B0: 41820008  beq 0x832903b8
	if ctx.cr[0].eq {
	pc = 0x832903B8; continue 'dispatch;
	}
	// 832903B4: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x832903B8; continue 'dispatch;
            }
            0x832903B8 => {
    //   block [0x832903B8..0x832903E8)
	// 832903B8: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 832903BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832903C0: 390956C0  addi r8, r9, 0x56c0
	ctx.r[8].s64 = ctx.r[9].s64 + 22208;
	// 832903C4: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 832903C8: 38675B40  addi r3, r7, 0x5b40
	ctx.r[3].s64 = ctx.r[7].s64 + 23360;
	// 832903CC: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832903D0: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832903D4: 4BA19B4D  bl 0x82ca9f20
	ctx.lr = 0x832903D8;
	sub_82CA9F20(ctx, base);
	// 832903D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832903DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832903E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832903E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832903E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832903E8 size=96
    let mut pc: u32 = 0x832903E8;
    'dispatch: loop {
        match pc {
            0x832903E8 => {
    //   block [0x832903E8..0x8329040C)
	// 832903E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832903EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832903F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832903F4: 38600070  li r3, 0x70
	ctx.r[3].s64 = 112;
	// 832903F8: 4AF8EE61  bl 0x8221f258
	ctx.lr = 0x832903FC;
	sub_8221F258(ctx, base);
	// 832903FC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83290400: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83290404: 419A0008  beq cr6, 0x8329040c
	if ctx.cr[6].eq {
	pc = 0x8329040C; continue 'dispatch;
	}
	// 83290408: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x8329040C; continue 'dispatch;
            }
            0x8329040C => {
    //   block [0x8329040C..0x83290418)
	// 8329040C: 354B0004  addic. r10, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83290410: 41820008  beq 0x83290418
	if ctx.cr[0].eq {
	pc = 0x83290418; continue 'dispatch;
	}
	// 83290414: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x83290418; continue 'dispatch;
            }
            0x83290418 => {
    //   block [0x83290418..0x83290448)
	// 83290418: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8329041C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83290420: 390956CC  addi r8, r9, 0x56cc
	ctx.r[8].s64 = ctx.r[9].s64 + 22220;
	// 83290424: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 83290428: 38675B88  addi r3, r7, 0x5b88
	ctx.r[3].s64 = ctx.r[7].s64 + 23432;
	// 8329042C: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83290430: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83290434: 4BA19AED  bl 0x82ca9f20
	ctx.lr = 0x83290438;
	sub_82CA9F20(ctx, base);
	// 83290438: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8329043C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83290440: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83290444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290448 size=96
    let mut pc: u32 = 0x83290448;
    'dispatch: loop {
        match pc {
            0x83290448 => {
    //   block [0x83290448..0x8329046C)
	// 83290448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329044C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290450: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83290454: 38600064  li r3, 0x64
	ctx.r[3].s64 = 100;
	// 83290458: 4AF8EE01  bl 0x8221f258
	ctx.lr = 0x8329045C;
	sub_8221F258(ctx, base);
	// 8329045C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83290460: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83290464: 419A0008  beq cr6, 0x8329046c
	if ctx.cr[6].eq {
	pc = 0x8329046C; continue 'dispatch;
	}
	// 83290468: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x8329046C; continue 'dispatch;
            }
            0x8329046C => {
    //   block [0x8329046C..0x83290478)
	// 8329046C: 354B0004  addic. r10, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83290470: 41820008  beq 0x83290478
	if ctx.cr[0].eq {
	pc = 0x83290478; continue 'dispatch;
	}
	// 83290474: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x83290478; continue 'dispatch;
            }
            0x83290478 => {
    //   block [0x83290478..0x832904A8)
	// 83290478: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8329047C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83290480: 390956D8  addi r8, r9, 0x56d8
	ctx.r[8].s64 = ctx.r[9].s64 + 22232;
	// 83290484: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 83290488: 38675BD0  addi r3, r7, 0x5bd0
	ctx.r[3].s64 = ctx.r[7].s64 + 23504;
	// 8329048C: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83290490: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83290494: 4BA19A8D  bl 0x82ca9f20
	ctx.lr = 0x83290498;
	sub_82CA9F20(ctx, base);
	// 83290498: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8329049C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832904A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832904A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832904A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832904A8 size=96
    let mut pc: u32 = 0x832904A8;
    'dispatch: loop {
        match pc {
            0x832904A8 => {
    //   block [0x832904A8..0x832904CC)
	// 832904A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832904AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832904B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832904B4: 3860006C  li r3, 0x6c
	ctx.r[3].s64 = 108;
	// 832904B8: 4AF8EDA1  bl 0x8221f258
	ctx.lr = 0x832904BC;
	sub_8221F258(ctx, base);
	// 832904BC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 832904C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 832904C4: 419A0008  beq cr6, 0x832904cc
	if ctx.cr[6].eq {
	pc = 0x832904CC; continue 'dispatch;
	}
	// 832904C8: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x832904CC; continue 'dispatch;
            }
            0x832904CC => {
    //   block [0x832904CC..0x832904D8)
	// 832904CC: 354B0004  addic. r10, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 832904D0: 41820008  beq 0x832904d8
	if ctx.cr[0].eq {
	pc = 0x832904D8; continue 'dispatch;
	}
	// 832904D4: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x832904D8; continue 'dispatch;
            }
            0x832904D8 => {
    //   block [0x832904D8..0x83290508)
	// 832904D8: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 832904DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832904E0: 390956E4  addi r8, r9, 0x56e4
	ctx.r[8].s64 = ctx.r[9].s64 + 22244;
	// 832904E4: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 832904E8: 38675C18  addi r3, r7, 0x5c18
	ctx.r[3].s64 = ctx.r[7].s64 + 23576;
	// 832904EC: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832904F0: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832904F4: 4BA19A2D  bl 0x82ca9f20
	ctx.lr = 0x832904F8;
	sub_82CA9F20(ctx, base);
	// 832904F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832904FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83290500: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83290504: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290508 size=96
    let mut pc: u32 = 0x83290508;
    'dispatch: loop {
        match pc {
            0x83290508 => {
    //   block [0x83290508..0x8329052C)
	// 83290508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329050C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290510: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83290514: 38600074  li r3, 0x74
	ctx.r[3].s64 = 116;
	// 83290518: 4AF8ED41  bl 0x8221f258
	ctx.lr = 0x8329051C;
	sub_8221F258(ctx, base);
	// 8329051C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83290520: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83290524: 419A0008  beq cr6, 0x8329052c
	if ctx.cr[6].eq {
	pc = 0x8329052C; continue 'dispatch;
	}
	// 83290528: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x8329052C; continue 'dispatch;
            }
            0x8329052C => {
    //   block [0x8329052C..0x83290538)
	// 8329052C: 354B0004  addic. r10, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83290530: 41820008  beq 0x83290538
	if ctx.cr[0].eq {
	pc = 0x83290538; continue 'dispatch;
	}
	// 83290534: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x83290538; continue 'dispatch;
            }
            0x83290538 => {
    //   block [0x83290538..0x83290568)
	// 83290538: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8329053C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83290540: 390956F0  addi r8, r9, 0x56f0
	ctx.r[8].s64 = ctx.r[9].s64 + 22256;
	// 83290544: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 83290548: 38675C60  addi r3, r7, 0x5c60
	ctx.r[3].s64 = ctx.r[7].s64 + 23648;
	// 8329054C: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83290550: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83290554: 4BA199CD  bl 0x82ca9f20
	ctx.lr = 0x83290558;
	sub_82CA9F20(ctx, base);
	// 83290558: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8329055C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83290560: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83290564: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290568 size=96
    let mut pc: u32 = 0x83290568;
    'dispatch: loop {
        match pc {
            0x83290568 => {
    //   block [0x83290568..0x8329058C)
	// 83290568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329056C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290570: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83290574: 38600058  li r3, 0x58
	ctx.r[3].s64 = 88;
	// 83290578: 4AF8ECE1  bl 0x8221f258
	ctx.lr = 0x8329057C;
	sub_8221F258(ctx, base);
	// 8329057C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83290580: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83290584: 419A0008  beq cr6, 0x8329058c
	if ctx.cr[6].eq {
	pc = 0x8329058C; continue 'dispatch;
	}
	// 83290588: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x8329058C; continue 'dispatch;
            }
            0x8329058C => {
    //   block [0x8329058C..0x83290598)
	// 8329058C: 354B0004  addic. r10, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83290590: 41820008  beq 0x83290598
	if ctx.cr[0].eq {
	pc = 0x83290598; continue 'dispatch;
	}
	// 83290594: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x83290598; continue 'dispatch;
            }
            0x83290598 => {
    //   block [0x83290598..0x832905C8)
	// 83290598: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8329059C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832905A0: 390956FC  addi r8, r9, 0x56fc
	ctx.r[8].s64 = ctx.r[9].s64 + 22268;
	// 832905A4: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 832905A8: 38675CA8  addi r3, r7, 0x5ca8
	ctx.r[3].s64 = ctx.r[7].s64 + 23720;
	// 832905AC: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832905B0: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832905B4: 4BA1996D  bl 0x82ca9f20
	ctx.lr = 0x832905B8;
	sub_82CA9F20(ctx, base);
	// 832905B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832905BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832905C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832905C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832905C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832905C8 size=12
    let mut pc: u32 = 0x832905C8;
    'dispatch: loop {
        match pc {
            0x832905C8 => {
    //   block [0x832905C8..0x832905D4)
	// 832905C8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832905CC: 386B5CF0  addi r3, r11, 0x5cf0
	ctx.r[3].s64 = ctx.r[11].s64 + 23792;
	// 832905D0: 4BA19950  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832905D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832905D8 size=12
    let mut pc: u32 = 0x832905D8;
    'dispatch: loop {
        match pc {
            0x832905D8 => {
    //   block [0x832905D8..0x832905E4)
	// 832905D8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832905DC: 386B5D48  addi r3, r11, 0x5d48
	ctx.r[3].s64 = ctx.r[11].s64 + 23880;
	// 832905E0: 4BA19940  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832905E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832905E8 size=12
    let mut pc: u32 = 0x832905E8;
    'dispatch: loop {
        match pc {
            0x832905E8 => {
    //   block [0x832905E8..0x832905F4)
	// 832905E8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832905EC: 386B5D58  addi r3, r11, 0x5d58
	ctx.r[3].s64 = ctx.r[11].s64 + 23896;
	// 832905F0: 4BA19930  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832905F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832905F8 size=64
    let mut pc: u32 = 0x832905F8;
    'dispatch: loop {
        match pc {
            0x832905F8 => {
    //   block [0x832905F8..0x83290638)
	// 832905F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832905FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290600: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83290604: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83290608: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8329060C: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83290610: 386A5714  addi r3, r10, 0x5714
	ctx.r[3].s64 = ctx.r[10].s64 + 22292;
	// 83290614: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83290618: 4AF9C8B9  bl 0x8222ced0
	ctx.lr = 0x8329061C;
	sub_8222CED0(ctx, base);
	// 8329061C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83290620: 38695D68  addi r3, r9, 0x5d68
	ctx.r[3].s64 = ctx.r[9].s64 + 23912;
	// 83290624: 4BA198FD  bl 0x82ca9f20
	ctx.lr = 0x83290628;
	sub_82CA9F20(ctx, base);
	// 83290628: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8329062C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83290630: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83290634: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290638 size=64
    let mut pc: u32 = 0x83290638;
    'dispatch: loop {
        match pc {
            0x83290638 => {
    //   block [0x83290638..0x83290678)
	// 83290638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329063C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290640: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83290644: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83290648: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8329064C: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83290650: 386A5718  addi r3, r10, 0x5718
	ctx.r[3].s64 = ctx.r[10].s64 + 22296;
	// 83290654: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83290658: 4AF9C879  bl 0x8222ced0
	ctx.lr = 0x8329065C;
	sub_8222CED0(ctx, base);
	// 8329065C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83290660: 38695D78  addi r3, r9, 0x5d78
	ctx.r[3].s64 = ctx.r[9].s64 + 23928;
	// 83290664: 4BA198BD  bl 0x82ca9f20
	ctx.lr = 0x83290668;
	sub_82CA9F20(ctx, base);
	// 83290668: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8329066C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83290670: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83290674: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83290678 size=12
    let mut pc: u32 = 0x83290678;
    'dispatch: loop {
        match pc {
            0x83290678 => {
    //   block [0x83290678..0x83290684)
	// 83290678: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8329067C: 386B5D88  addi r3, r11, 0x5d88
	ctx.r[3].s64 = ctx.r[11].s64 + 23944;
	// 83290680: 4BA198A0  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83290688 size=12
    let mut pc: u32 = 0x83290688;
    'dispatch: loop {
        match pc {
            0x83290688 => {
    //   block [0x83290688..0x83290694)
	// 83290688: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8329068C: 386B5D98  addi r3, r11, 0x5d98
	ctx.r[3].s64 = ctx.r[11].s64 + 23960;
	// 83290690: 4BA19890  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83290698 size=12
    let mut pc: u32 = 0x83290698;
    'dispatch: loop {
        match pc {
            0x83290698 => {
    //   block [0x83290698..0x832906A4)
	// 83290698: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8329069C: 386B5DA8  addi r3, r11, 0x5da8
	ctx.r[3].s64 = ctx.r[11].s64 + 23976;
	// 832906A0: 4BA19880  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832906A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832906A8 size=52
    let mut pc: u32 = 0x832906A8;
    'dispatch: loop {
        match pc {
            0x832906A8 => {
    //   block [0x832906A8..0x832906DC)
	// 832906A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832906AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832906B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832906B4: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832906B8: 386B573C  addi r3, r11, 0x573c
	ctx.r[3].s64 = ctx.r[11].s64 + 22332;
	// 832906BC: 480295C9  bl 0x832b9c84
	ctx.lr = 0x832906C0;
	// extern call 0x832B9C84 → crate::xboxkrnl::RtlInitializeCriticalSection
	crate::xboxkrnl::RtlInitializeCriticalSection(ctx, base);
	// 832906C0: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 832906C4: 386A5DB8  addi r3, r10, 0x5db8
	ctx.r[3].s64 = ctx.r[10].s64 + 23992;
	// 832906C8: 4BA19859  bl 0x82ca9f20
	ctx.lr = 0x832906CC;
	sub_82CA9F20(ctx, base);
	// 832906CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832906D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832906D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832906D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832906E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832906E0 size=64
    let mut pc: u32 = 0x832906E0;
    'dispatch: loop {
        match pc {
            0x832906E0 => {
    //   block [0x832906E0..0x83290720)
	// 832906E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832906E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832906E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832906EC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832906F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832906F4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 832906F8: 386A5758  addi r3, r10, 0x5758
	ctx.r[3].s64 = ctx.r[10].s64 + 22360;
	// 832906FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83290700: 4AF9C7D1  bl 0x8222ced0
	ctx.lr = 0x83290704;
	sub_8222CED0(ctx, base);
	// 83290704: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83290708: 38695DC0  addi r3, r9, 0x5dc0
	ctx.r[3].s64 = ctx.r[9].s64 + 24000;
	// 8329070C: 4BA19815  bl 0x82ca9f20
	ctx.lr = 0x83290710;
	sub_82CA9F20(ctx, base);
	// 83290710: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83290714: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83290718: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329071C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290720 size=64
    let mut pc: u32 = 0x83290720;
    'dispatch: loop {
        match pc {
            0x83290720 => {
    //   block [0x83290720..0x83290760)
	// 83290720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83290724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290728: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329072C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83290730: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83290734: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83290738: 386A575C  addi r3, r10, 0x575c
	ctx.r[3].s64 = ctx.r[10].s64 + 22364;
	// 8329073C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83290740: 4AF9C791  bl 0x8222ced0
	ctx.lr = 0x83290744;
	sub_8222CED0(ctx, base);
	// 83290744: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83290748: 38695DD0  addi r3, r9, 0x5dd0
	ctx.r[3].s64 = ctx.r[9].s64 + 24016;
	// 8329074C: 4BA197D5  bl 0x82ca9f20
	ctx.lr = 0x83290750;
	sub_82CA9F20(ctx, base);
	// 83290750: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83290754: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83290758: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329075C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83290760 size=12
    let mut pc: u32 = 0x83290760;
    'dispatch: loop {
        match pc {
            0x83290760 => {
    //   block [0x83290760..0x8329076C)
	// 83290760: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83290764: 386B5DE0  addi r3, r11, 0x5de0
	ctx.r[3].s64 = ctx.r[11].s64 + 24032;
	// 83290768: 4BA197B8  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290770 size=64
    let mut pc: u32 = 0x83290770;
    'dispatch: loop {
        match pc {
            0x83290770 => {
    //   block [0x83290770..0x832907B0)
	// 83290770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83290774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290778: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329077C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83290780: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83290784: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83290788: 386A5764  addi r3, r10, 0x5764
	ctx.r[3].s64 = ctx.r[10].s64 + 22372;
	// 8329078C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83290790: 4AF9C741  bl 0x8222ced0
	ctx.lr = 0x83290794;
	sub_8222CED0(ctx, base);
	// 83290794: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83290798: 38695E30  addi r3, r9, 0x5e30
	ctx.r[3].s64 = ctx.r[9].s64 + 24112;
	// 8329079C: 4BA19785  bl 0x82ca9f20
	ctx.lr = 0x832907A0;
	sub_82CA9F20(ctx, base);
	// 832907A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832907A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832907A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832907AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832907B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832907B0 size=64
    let mut pc: u32 = 0x832907B0;
    'dispatch: loop {
        match pc {
            0x832907B0 => {
    //   block [0x832907B0..0x832907F0)
	// 832907B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832907B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832907B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832907BC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832907C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832907C4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 832907C8: 386A5768  addi r3, r10, 0x5768
	ctx.r[3].s64 = ctx.r[10].s64 + 22376;
	// 832907CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832907D0: 4AF9C701  bl 0x8222ced0
	ctx.lr = 0x832907D4;
	sub_8222CED0(ctx, base);
	// 832907D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832907D8: 38695E40  addi r3, r9, 0x5e40
	ctx.r[3].s64 = ctx.r[9].s64 + 24128;
	// 832907DC: 4BA19745  bl 0x82ca9f20
	ctx.lr = 0x832907E0;
	sub_82CA9F20(ctx, base);
	// 832907E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832907E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832907E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832907EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832907F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832907F0 size=64
    let mut pc: u32 = 0x832907F0;
    'dispatch: loop {
        match pc {
            0x832907F0 => {
    //   block [0x832907F0..0x83290830)
	// 832907F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832907F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832907F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832907FC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83290800: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83290804: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83290808: 386A576C  addi r3, r10, 0x576c
	ctx.r[3].s64 = ctx.r[10].s64 + 22380;
	// 8329080C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83290810: 4AF9C6C1  bl 0x8222ced0
	ctx.lr = 0x83290814;
	sub_8222CED0(ctx, base);
	// 83290814: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83290818: 38695E50  addi r3, r9, 0x5e50
	ctx.r[3].s64 = ctx.r[9].s64 + 24144;
	// 8329081C: 4BA19705  bl 0x82ca9f20
	ctx.lr = 0x83290820;
	sub_82CA9F20(ctx, base);
	// 83290820: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83290824: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83290828: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329082C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290830 size=64
    let mut pc: u32 = 0x83290830;
    'dispatch: loop {
        match pc {
            0x83290830 => {
    //   block [0x83290830..0x83290870)
	// 83290830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83290834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290838: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329083C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83290840: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83290844: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83290848: 386A5770  addi r3, r10, 0x5770
	ctx.r[3].s64 = ctx.r[10].s64 + 22384;
	// 8329084C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83290850: 4AF9C681  bl 0x8222ced0
	ctx.lr = 0x83290854;
	sub_8222CED0(ctx, base);
	// 83290854: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83290858: 38695E60  addi r3, r9, 0x5e60
	ctx.r[3].s64 = ctx.r[9].s64 + 24160;
	// 8329085C: 4BA196C5  bl 0x82ca9f20
	ctx.lr = 0x83290860;
	sub_82CA9F20(ctx, base);
	// 83290860: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83290864: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83290868: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329086C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83290870 size=12
    let mut pc: u32 = 0x83290870;
    'dispatch: loop {
        match pc {
            0x83290870 => {
    //   block [0x83290870..0x8329087C)
	// 83290870: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83290874: 386B5E70  addi r3, r11, 0x5e70
	ctx.r[3].s64 = ctx.r[11].s64 + 24176;
	// 83290878: 4BA196A8  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83290880 size=12
    let mut pc: u32 = 0x83290880;
    'dispatch: loop {
        match pc {
            0x83290880 => {
    //   block [0x83290880..0x8329088C)
	// 83290880: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83290884: 386B5EC8  addi r3, r11, 0x5ec8
	ctx.r[3].s64 = ctx.r[11].s64 + 24264;
	// 83290888: 4BA19698  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290890 size=64
    let mut pc: u32 = 0x83290890;
    'dispatch: loop {
        match pc {
            0x83290890 => {
    //   block [0x83290890..0x832908D0)
	// 83290890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83290894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290898: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329089C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832908A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832908A4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 832908A8: 386A5794  addi r3, r10, 0x5794
	ctx.r[3].s64 = ctx.r[10].s64 + 22420;
	// 832908AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832908B0: 4AF9C621  bl 0x8222ced0
	ctx.lr = 0x832908B4;
	sub_8222CED0(ctx, base);
	// 832908B4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832908B8: 38695F88  addi r3, r9, 0x5f88
	ctx.r[3].s64 = ctx.r[9].s64 + 24456;
	// 832908BC: 4BA19665  bl 0x82ca9f20
	ctx.lr = 0x832908C0;
	sub_82CA9F20(ctx, base);
	// 832908C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832908C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832908C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832908CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832908D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832908D0 size=64
    let mut pc: u32 = 0x832908D0;
    'dispatch: loop {
        match pc {
            0x832908D0 => {
    //   block [0x832908D0..0x83290910)
	// 832908D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832908D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832908D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832908DC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832908E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832908E4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 832908E8: 386A5798  addi r3, r10, 0x5798
	ctx.r[3].s64 = ctx.r[10].s64 + 22424;
	// 832908EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832908F0: 4AF9C5E1  bl 0x8222ced0
	ctx.lr = 0x832908F4;
	sub_8222CED0(ctx, base);
	// 832908F4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832908F8: 38695F98  addi r3, r9, 0x5f98
	ctx.r[3].s64 = ctx.r[9].s64 + 24472;
	// 832908FC: 4BA19625  bl 0x82ca9f20
	ctx.lr = 0x83290900;
	sub_82CA9F20(ctx, base);
	// 83290900: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83290904: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83290908: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329090C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290910 size=64
    let mut pc: u32 = 0x83290910;
    'dispatch: loop {
        match pc {
            0x83290910 => {
    //   block [0x83290910..0x83290950)
	// 83290910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83290914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290918: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329091C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83290920: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83290924: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83290928: 386A579C  addi r3, r10, 0x579c
	ctx.r[3].s64 = ctx.r[10].s64 + 22428;
	// 8329092C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83290930: 4AF9C5A1  bl 0x8222ced0
	ctx.lr = 0x83290934;
	sub_8222CED0(ctx, base);
	// 83290934: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83290938: 38696000  addi r3, r9, 0x6000
	ctx.r[3].s64 = ctx.r[9].s64 + 24576;
	// 8329093C: 4BA195E5  bl 0x82ca9f20
	ctx.lr = 0x83290940;
	sub_82CA9F20(ctx, base);
	// 83290940: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83290944: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83290948: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329094C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290950 size=64
    let mut pc: u32 = 0x83290950;
    'dispatch: loop {
        match pc {
            0x83290950 => {
    //   block [0x83290950..0x83290990)
	// 83290950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83290954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290958: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329095C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83290960: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83290964: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83290968: 386A57A0  addi r3, r10, 0x57a0
	ctx.r[3].s64 = ctx.r[10].s64 + 22432;
	// 8329096C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83290970: 4AF9C561  bl 0x8222ced0
	ctx.lr = 0x83290974;
	sub_8222CED0(ctx, base);
	// 83290974: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83290978: 38696010  addi r3, r9, 0x6010
	ctx.r[3].s64 = ctx.r[9].s64 + 24592;
	// 8329097C: 4BA195A5  bl 0x82ca9f20
	ctx.lr = 0x83290980;
	sub_82CA9F20(ctx, base);
	// 83290980: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83290984: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83290988: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329098C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290990 size=64
    let mut pc: u32 = 0x83290990;
    'dispatch: loop {
        match pc {
            0x83290990 => {
    //   block [0x83290990..0x832909D0)
	// 83290990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83290994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290998: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329099C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832909A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832909A4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 832909A8: 386A57A4  addi r3, r10, 0x57a4
	ctx.r[3].s64 = ctx.r[10].s64 + 22436;
	// 832909AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832909B0: 4AF9C521  bl 0x8222ced0
	ctx.lr = 0x832909B4;
	sub_8222CED0(ctx, base);
	// 832909B4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832909B8: 38696020  addi r3, r9, 0x6020
	ctx.r[3].s64 = ctx.r[9].s64 + 24608;
	// 832909BC: 4BA19565  bl 0x82ca9f20
	ctx.lr = 0x832909C0;
	sub_82CA9F20(ctx, base);
	// 832909C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832909C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832909C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832909CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832909D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832909D0 size=64
    let mut pc: u32 = 0x832909D0;
    'dispatch: loop {
        match pc {
            0x832909D0 => {
    //   block [0x832909D0..0x83290A10)
	// 832909D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832909D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832909D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832909DC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832909E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832909E4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 832909E8: 386A57A8  addi r3, r10, 0x57a8
	ctx.r[3].s64 = ctx.r[10].s64 + 22440;
	// 832909EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832909F0: 4AF9C4E1  bl 0x8222ced0
	ctx.lr = 0x832909F4;
	sub_8222CED0(ctx, base);
	// 832909F4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832909F8: 38696030  addi r3, r9, 0x6030
	ctx.r[3].s64 = ctx.r[9].s64 + 24624;
	// 832909FC: 4BA19525  bl 0x82ca9f20
	ctx.lr = 0x83290A00;
	sub_82CA9F20(ctx, base);
	// 83290A00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83290A04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83290A08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83290A0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83290A10 size=12
    let mut pc: u32 = 0x83290A10;
    'dispatch: loop {
        match pc {
            0x83290A10 => {
    //   block [0x83290A10..0x83290A1C)
	// 83290A10: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83290A14: 386B6040  addi r3, r11, 0x6040
	ctx.r[3].s64 = ctx.r[11].s64 + 24640;
	// 83290A18: 4BA19508  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290A20 size=52
    let mut pc: u32 = 0x83290A20;
    'dispatch: loop {
        match pc {
            0x83290A20 => {
    //   block [0x83290A20..0x83290A54)
	// 83290A20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83290A24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290A28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83290A2C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83290A30: 386B57C0  addi r3, r11, 0x57c0
	ctx.r[3].s64 = ctx.r[11].s64 + 22464;
	// 83290A34: 4B82ADF5  bl 0x82abb828
	ctx.lr = 0x83290A38;
	sub_82ABB828(ctx, base);
	// 83290A38: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83290A3C: 386A6098  addi r3, r10, 0x6098
	ctx.r[3].s64 = ctx.r[10].s64 + 24728;
	// 83290A40: 4BA194E1  bl 0x82ca9f20
	ctx.lr = 0x83290A44;
	sub_82CA9F20(ctx, base);
	// 83290A44: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83290A48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83290A4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83290A50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83290A58 size=12
    let mut pc: u32 = 0x83290A58;
    'dispatch: loop {
        match pc {
            0x83290A58 => {
    //   block [0x83290A58..0x83290A64)
	// 83290A58: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83290A5C: 386B60A8  addi r3, r11, 0x60a8
	ctx.r[3].s64 = ctx.r[11].s64 + 24744;
	// 83290A60: 4BA194C0  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83290A68 size=12
    let mut pc: u32 = 0x83290A68;
    'dispatch: loop {
        match pc {
            0x83290A68 => {
    //   block [0x83290A68..0x83290A74)
	// 83290A68: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83290A6C: 386B60B8  addi r3, r11, 0x60b8
	ctx.r[3].s64 = ctx.r[11].s64 + 24760;
	// 83290A70: 4BA194B0  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83290A78 size=12
    let mut pc: u32 = 0x83290A78;
    'dispatch: loop {
        match pc {
            0x83290A78 => {
    //   block [0x83290A78..0x83290A84)
	// 83290A78: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83290A7C: 386B60C8  addi r3, r11, 0x60c8
	ctx.r[3].s64 = ctx.r[11].s64 + 24776;
	// 83290A80: 4BA194A0  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83290A88 size=12
    let mut pc: u32 = 0x83290A88;
    'dispatch: loop {
        match pc {
            0x83290A88 => {
    //   block [0x83290A88..0x83290A94)
	// 83290A88: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83290A8C: 386B60D8  addi r3, r11, 0x60d8
	ctx.r[3].s64 = ctx.r[11].s64 + 24792;
	// 83290A90: 4BA19490  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83290A98 size=12
    let mut pc: u32 = 0x83290A98;
    'dispatch: loop {
        match pc {
            0x83290A98 => {
    //   block [0x83290A98..0x83290AA4)
	// 83290A98: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83290A9C: 386B60E8  addi r3, r11, 0x60e8
	ctx.r[3].s64 = ctx.r[11].s64 + 24808;
	// 83290AA0: 4BA19480  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83290AA8 size=12
    let mut pc: u32 = 0x83290AA8;
    'dispatch: loop {
        match pc {
            0x83290AA8 => {
    //   block [0x83290AA8..0x83290AB4)
	// 83290AA8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83290AAC: 386B60F8  addi r3, r11, 0x60f8
	ctx.r[3].s64 = ctx.r[11].s64 + 24824;
	// 83290AB0: 4BA19470  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83290AB8 size=12
    let mut pc: u32 = 0x83290AB8;
    'dispatch: loop {
        match pc {
            0x83290AB8 => {
    //   block [0x83290AB8..0x83290AC4)
	// 83290AB8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83290ABC: 386B6108  addi r3, r11, 0x6108
	ctx.r[3].s64 = ctx.r[11].s64 + 24840;
	// 83290AC0: 4BA19460  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83290AC8 size=12
    let mut pc: u32 = 0x83290AC8;
    'dispatch: loop {
        match pc {
            0x83290AC8 => {
    //   block [0x83290AC8..0x83290AD4)
	// 83290AC8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83290ACC: 386B6118  addi r3, r11, 0x6118
	ctx.r[3].s64 = ctx.r[11].s64 + 24856;
	// 83290AD0: 4BA19450  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290AD8 size=64
    let mut pc: u32 = 0x83290AD8;
    'dispatch: loop {
        match pc {
            0x83290AD8 => {
    //   block [0x83290AD8..0x83290B18)
	// 83290AD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83290ADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290AE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83290AE4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83290AE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83290AEC: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83290AF0: 386A58D0  addi r3, r10, 0x58d0
	ctx.r[3].s64 = ctx.r[10].s64 + 22736;
	// 83290AF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83290AF8: 4AF9C3D9  bl 0x8222ced0
	ctx.lr = 0x83290AFC;
	sub_8222CED0(ctx, base);
	// 83290AFC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83290B00: 38696128  addi r3, r9, 0x6128
	ctx.r[3].s64 = ctx.r[9].s64 + 24872;
	// 83290B04: 4BA1941D  bl 0x82ca9f20
	ctx.lr = 0x83290B08;
	sub_82CA9F20(ctx, base);
	// 83290B08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83290B0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83290B10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83290B14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290B18 size=64
    let mut pc: u32 = 0x83290B18;
    'dispatch: loop {
        match pc {
            0x83290B18 => {
    //   block [0x83290B18..0x83290B58)
	// 83290B18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83290B1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290B20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83290B24: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83290B28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83290B2C: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83290B30: 386A58D4  addi r3, r10, 0x58d4
	ctx.r[3].s64 = ctx.r[10].s64 + 22740;
	// 83290B34: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83290B38: 4AF9C399  bl 0x8222ced0
	ctx.lr = 0x83290B3C;
	sub_8222CED0(ctx, base);
	// 83290B3C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83290B40: 38696138  addi r3, r9, 0x6138
	ctx.r[3].s64 = ctx.r[9].s64 + 24888;
	// 83290B44: 4BA193DD  bl 0x82ca9f20
	ctx.lr = 0x83290B48;
	sub_82CA9F20(ctx, base);
	// 83290B48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83290B4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83290B50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83290B54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290B58 size=64
    let mut pc: u32 = 0x83290B58;
    'dispatch: loop {
        match pc {
            0x83290B58 => {
    //   block [0x83290B58..0x83290B98)
	// 83290B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83290B5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290B60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83290B64: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83290B68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83290B6C: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83290B70: 386A58D8  addi r3, r10, 0x58d8
	ctx.r[3].s64 = ctx.r[10].s64 + 22744;
	// 83290B74: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83290B78: 4AF9C359  bl 0x8222ced0
	ctx.lr = 0x83290B7C;
	sub_8222CED0(ctx, base);
	// 83290B7C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83290B80: 38696148  addi r3, r9, 0x6148
	ctx.r[3].s64 = ctx.r[9].s64 + 24904;
	// 83290B84: 4BA1939D  bl 0x82ca9f20
	ctx.lr = 0x83290B88;
	sub_82CA9F20(ctx, base);
	// 83290B88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83290B8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83290B90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83290B94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290B98 size=64
    let mut pc: u32 = 0x83290B98;
    'dispatch: loop {
        match pc {
            0x83290B98 => {
    //   block [0x83290B98..0x83290BD8)
	// 83290B98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83290B9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290BA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83290BA4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83290BA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83290BAC: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83290BB0: 386A58DC  addi r3, r10, 0x58dc
	ctx.r[3].s64 = ctx.r[10].s64 + 22748;
	// 83290BB4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83290BB8: 4AF9C319  bl 0x8222ced0
	ctx.lr = 0x83290BBC;
	sub_8222CED0(ctx, base);
	// 83290BBC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83290BC0: 38696158  addi r3, r9, 0x6158
	ctx.r[3].s64 = ctx.r[9].s64 + 24920;
	// 83290BC4: 4BA1935D  bl 0x82ca9f20
	ctx.lr = 0x83290BC8;
	sub_82CA9F20(ctx, base);
	// 83290BC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83290BCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83290BD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83290BD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290BD8 size=64
    let mut pc: u32 = 0x83290BD8;
    'dispatch: loop {
        match pc {
            0x83290BD8 => {
    //   block [0x83290BD8..0x83290C18)
	// 83290BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83290BDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290BE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83290BE4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83290BE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83290BEC: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83290BF0: 386A58E0  addi r3, r10, 0x58e0
	ctx.r[3].s64 = ctx.r[10].s64 + 22752;
	// 83290BF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83290BF8: 4AF9C2D9  bl 0x8222ced0
	ctx.lr = 0x83290BFC;
	sub_8222CED0(ctx, base);
	// 83290BFC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83290C00: 38696168  addi r3, r9, 0x6168
	ctx.r[3].s64 = ctx.r[9].s64 + 24936;
	// 83290C04: 4BA1931D  bl 0x82ca9f20
	ctx.lr = 0x83290C08;
	sub_82CA9F20(ctx, base);
	// 83290C08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83290C0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83290C10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83290C14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290C18 size=64
    let mut pc: u32 = 0x83290C18;
    'dispatch: loop {
        match pc {
            0x83290C18 => {
    //   block [0x83290C18..0x83290C58)
	// 83290C18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83290C1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290C20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83290C24: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83290C28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83290C2C: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83290C30: 386A58E4  addi r3, r10, 0x58e4
	ctx.r[3].s64 = ctx.r[10].s64 + 22756;
	// 83290C34: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83290C38: 4AF9C299  bl 0x8222ced0
	ctx.lr = 0x83290C3C;
	sub_8222CED0(ctx, base);
	// 83290C3C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83290C40: 38696178  addi r3, r9, 0x6178
	ctx.r[3].s64 = ctx.r[9].s64 + 24952;
	// 83290C44: 4BA192DD  bl 0x82ca9f20
	ctx.lr = 0x83290C48;
	sub_82CA9F20(ctx, base);
	// 83290C48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83290C4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83290C50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83290C54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83290C58 size=12
    let mut pc: u32 = 0x83290C58;
    'dispatch: loop {
        match pc {
            0x83290C58 => {
    //   block [0x83290C58..0x83290C64)
	// 83290C58: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83290C5C: 386B58F0  addi r3, r11, 0x58f0
	ctx.r[3].s64 = ctx.r[11].s64 + 22768;
	// 83290C60: 4AFAB150  b 0x8223bdb0
	sub_8223BDB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83290C68 size=28
    let mut pc: u32 = 0x83290C68;
    'dispatch: loop {
        match pc {
            0x83290C68 => {
    //   block [0x83290C68..0x83290C84)
	// 83290C68: 1001038C  vspltisw v0, 1
	for i in 0..4 {
		ctx.v[0].u32[i] = 1;
	}
	// 83290C6C: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 83290C70: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 83290C74: 392BAD50  addi r9, r11, -0x52b0
	ctx.r[9].s64 = ctx.r[11].s64 + -21168;
	// 83290C78: 1000030A  vcfux v0, v0, 0
	// vcfux/vcuxwfp128: ctx.v[0].f32[i] = ( ctx.v[0].u32[i] as f32 ) * (2.0f32).powi(0);
	for i in 0..4 { ctx.v[0].f32[i] = (ctx.v[0].u32[i] as f32) * (2.0f32).powi(0); }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83290C88 size=28
    let mut pc: u32 = 0x83290C88;
    'dispatch: loop {
        match pc {
            0x83290C88 => {
    //   block [0x83290C88..0x83290CA4)
	// 83290C88: 1001038C  vspltisw v0, 1
	for i in 0..4 {
		ctx.v[0].u32[i] = 1;
	}
	// 83290C8C: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 83290C90: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 83290C94: 392BAD70  addi r9, r11, -0x5290
	ctx.r[9].s64 = ctx.r[11].s64 + -21136;
	// 83290C98: 1000030A  vcfux v0, v0, 0
	// vcfux/vcuxwfp128: ctx.v[0].f32[i] = ( ctx.v[0].u32[i] as f32 ) * (2.0f32).powi(0);
	for i in 0..4 { ctx.v[0].f32[i] = (ctx.v[0].u32[i] as f32) * (2.0f32).powi(0); }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83290CA8 size=24
    let mut pc: u32 = 0x83290CA8;
    'dispatch: loop {
        match pc {
            0x83290CA8 => {
    //   block [0x83290CA8..0x83290CC0)
	// 83290CA8: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 83290CAC: 1000038C  vspltisw v0, 0
	for i in 0..4 {
		ctx.v[0].u32[i] = 0;
	}
	// 83290CB0: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 83290CB4: 392BAD90  addi r9, r11, -0x5270
	ctx.r[9].s64 = ctx.r[11].s64 + -21104;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290CC0 size=64
    let mut pc: u32 = 0x83290CC0;
    'dispatch: loop {
        match pc {
            0x83290CC0 => {
    //   block [0x83290CC0..0x83290D00)
	// 83290CC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83290CC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290CC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83290CCC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83290CD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83290CD4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83290CD8: 386A5930  addi r3, r10, 0x5930
	ctx.r[3].s64 = ctx.r[10].s64 + 22832;
	// 83290CDC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83290CE0: 4AF9C1F1  bl 0x8222ced0
	ctx.lr = 0x83290CE4;
	sub_8222CED0(ctx, base);
	// 83290CE4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83290CE8: 38696188  addi r3, r9, 0x6188
	ctx.r[3].s64 = ctx.r[9].s64 + 24968;
	// 83290CEC: 4BA19235  bl 0x82ca9f20
	ctx.lr = 0x83290CF0;
	sub_82CA9F20(ctx, base);
	// 83290CF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83290CF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83290CF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83290CFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290D00 size=64
    let mut pc: u32 = 0x83290D00;
    'dispatch: loop {
        match pc {
            0x83290D00 => {
    //   block [0x83290D00..0x83290D40)
	// 83290D00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83290D04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290D08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83290D0C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83290D10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83290D14: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83290D18: 386A5934  addi r3, r10, 0x5934
	ctx.r[3].s64 = ctx.r[10].s64 + 22836;
	// 83290D1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83290D20: 4AF9C1B1  bl 0x8222ced0
	ctx.lr = 0x83290D24;
	sub_8222CED0(ctx, base);
	// 83290D24: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83290D28: 38696198  addi r3, r9, 0x6198
	ctx.r[3].s64 = ctx.r[9].s64 + 24984;
	// 83290D2C: 4BA191F5  bl 0x82ca9f20
	ctx.lr = 0x83290D30;
	sub_82CA9F20(ctx, base);
	// 83290D30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83290D34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83290D38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83290D3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290D40 size=64
    let mut pc: u32 = 0x83290D40;
    'dispatch: loop {
        match pc {
            0x83290D40 => {
    //   block [0x83290D40..0x83290D80)
	// 83290D40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83290D44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290D48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83290D4C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83290D50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83290D54: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83290D58: 386A5938  addi r3, r10, 0x5938
	ctx.r[3].s64 = ctx.r[10].s64 + 22840;
	// 83290D5C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83290D60: 4AF9C171  bl 0x8222ced0
	ctx.lr = 0x83290D64;
	sub_8222CED0(ctx, base);
	// 83290D64: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83290D68: 386961A8  addi r3, r9, 0x61a8
	ctx.r[3].s64 = ctx.r[9].s64 + 25000;
	// 83290D6C: 4BA191B5  bl 0x82ca9f20
	ctx.lr = 0x83290D70;
	sub_82CA9F20(ctx, base);
	// 83290D70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83290D74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83290D78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83290D7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290D80 size=64
    let mut pc: u32 = 0x83290D80;
    'dispatch: loop {
        match pc {
            0x83290D80 => {
    //   block [0x83290D80..0x83290DC0)
	// 83290D80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83290D84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290D88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83290D8C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83290D90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83290D94: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83290D98: 386A593C  addi r3, r10, 0x593c
	ctx.r[3].s64 = ctx.r[10].s64 + 22844;
	// 83290D9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83290DA0: 4AF9C131  bl 0x8222ced0
	ctx.lr = 0x83290DA4;
	sub_8222CED0(ctx, base);
	// 83290DA4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83290DA8: 386961B8  addi r3, r9, 0x61b8
	ctx.r[3].s64 = ctx.r[9].s64 + 25016;
	// 83290DAC: 4BA19175  bl 0x82ca9f20
	ctx.lr = 0x83290DB0;
	sub_82CA9F20(ctx, base);
	// 83290DB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83290DB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83290DB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83290DBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83290DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83290DC0 size=992
    let mut pc: u32 = 0x83290DC0;
    'dispatch: loop {
        match pc {
            0x83290DC0 => {
    //   block [0x83290DC0..0x832911A0)
	// 83290DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83290DC4: 4BA18621  bl 0x82ca93e4
	ctx.lr = 0x83290DC8;
	sub_82CA93D0(ctx, base);
	// 83290DC8: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83290DCC: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 83290DD0: 392BD5DC  addi r9, r11, -0x2a24
	ctx.r[9].s64 = ctx.r[11].s64 + -10788;
	// 83290DD4: 3901FF34  addi r8, r1, -0xcc
	ctx.r[8].s64 = ctx.r[1].s64 + -204;
	// 83290DD8: 38E1FF38  addi r7, r1, -0xc8
	ctx.r[7].s64 = ctx.r[1].s64 + -200;
	// 83290DDC: 38C1FF3C  addi r6, r1, -0xc4
	ctx.r[6].s64 = ctx.r[1].s64 + -196;
	// 83290DE0: C1AA0A9C  lfs f13, 0xa9c(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2716 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 83290DE4: 38A1FF30  addi r5, r1, -0xd0
	ctx.r[5].s64 = ctx.r[1].s64 + -208;
	// 83290DE8: C009BEA8  lfs f0, -0x4158(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-16728 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83290DEC: 3881FF40  addi r4, r1, -0xc0
	ctx.r[4].s64 = ctx.r[1].s64 + -192;
	// 83290DF0: D001FF34  stfs f0, -0xcc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-204 as u32), tmp.u32 ) };
	// 83290DF4: 3861FF44  addi r3, r1, -0xbc
	ctx.r[3].s64 = ctx.r[1].s64 + -188;
	// 83290DF8: D001FF38  stfs f0, -0xc8(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-200 as u32), tmp.u32 ) };
	// 83290DFC: 3BC1FF50  addi r30, r1, -0xb0
	ctx.r[30].s64 = ctx.r[1].s64 + -176;
	// 83290E00: C189BEB4  lfs f12, -0x414c(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-16716 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 83290E04: 3BA1FF54  addi r29, r1, -0xac
	ctx.r[29].s64 = ctx.r[1].s64 + -172;
	// 83290E08: D181FF30  stfs f12, -0xd0(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-208 as u32), tmp.u32 ) };
	// 83290E0C: 3B81FF58  addi r28, r1, -0xa8
	ctx.r[28].s64 = ctx.r[1].s64 + -168;
	// 83290E10: C189BCF8  lfs f12, -0x4308(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-17160 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 83290E14: 3B61FF5C  addi r27, r1, -0xa4
	ctx.r[27].s64 = ctx.r[1].s64 + -164;
	// 83290E18: D001FF3C  stfs f0, -0xc4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-196 as u32), tmp.u32 ) };
	// 83290E1C: 3941FF48  addi r10, r1, -0xb8
	ctx.r[10].s64 = ctx.r[1].s64 + -184;
	// 83290E20: D1A1FF40  stfs f13, -0xc0(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-192 as u32), tmp.u32 ) };
	// 83290E24: 3BE1FF4C  addi r31, r1, -0xb4
	ctx.r[31].s64 = ctx.r[1].s64 + -180;
	// 83290E28: D181FF44  stfs f12, -0xbc(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-188 as u32), tmp.u32 ) };
	// 83290E2C: 3B41FF60  addi r26, r1, -0xa0
	ctx.r[26].s64 = ctx.r[1].s64 + -160;
	// 83290E30: D001FF50  stfs f0, -0xb0(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-176 as u32), tmp.u32 ) };
	// 83290E34: 3B21FF64  addi r25, r1, -0x9c
	ctx.r[25].s64 = ctx.r[1].s64 + -156;
	// 83290E38: D001FF54  stfs f0, -0xac(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-172 as u32), tmp.u32 ) };
	// 83290E3C: 3B01FF68  addi r24, r1, -0x98
	ctx.r[24].s64 = ctx.r[1].s64 + -152;
	// 83290E40: D001FF58  stfs f0, -0xa8(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-168 as u32), tmp.u32 ) };
	// 83290E44: 3AE1FF6C  addi r23, r1, -0x94
	ctx.r[23].s64 = ctx.r[1].s64 + -148;
	// 83290E48: D001FF5C  stfs f0, -0xa4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-164 as u32), tmp.u32 ) };
	// 83290E4C: 3AA1FF74  addi r21, r1, -0x8c
	ctx.r[21].s64 = ctx.r[1].s64 + -140;
	// 83290E50: C16911D0  lfs f11, 0x11d0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4560 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 83290E54: 3AC1FF70  addi r22, r1, -0x90
	ctx.r[22].s64 = ctx.r[1].s64 + -144;
	// 83290E58: D001FF6C  stfs f0, -0x94(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-148 as u32), tmp.u32 ) };
	// 83290E5C: 3A81FF78  addi r20, r1, -0x88
	ctx.r[20].s64 = ctx.r[1].s64 + -136;
	// 83290E60: D1A1FF48  stfs f13, -0xb8(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-184 as u32), tmp.u32 ) };
	// 83290E64: 3A61FF74  addi r19, r1, -0x8c
	ctx.r[19].s64 = ctx.r[1].s64 + -140;
	// 83290E68: D161FF4C  stfs f11, -0xb4(r1)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-180 as u32), tmp.u32 ) };
	// 83290E6C: D001FF60  stfs f0, -0xa0(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), tmp.u32 ) };
	// 83290E70: D001FF64  stfs f0, -0x9c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-156 as u32), tmp.u32 ) };
	// 83290E74: D001FF68  stfs f0, -0x98(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-152 as u32), tmp.u32 ) };
	// 83290E78: D001FF74  stfs f0, -0x8c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-140 as u32), tmp.u32 ) };
	// 83290E7C: D001FF70  stfs f0, -0x90(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-144 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832911A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832911A0 size=64
    let mut pc: u32 = 0x832911A0;
    'dispatch: loop {
        match pc {
            0x832911A0 => {
    //   block [0x832911A0..0x832911E0)
	// 832911A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832911A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832911A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832911AC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832911B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832911B4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 832911B8: 386A5A10  addi r3, r10, 0x5a10
	ctx.r[3].s64 = ctx.r[10].s64 + 23056;
	// 832911BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832911C0: 4AF9BD11  bl 0x8222ced0
	ctx.lr = 0x832911C4;
	sub_8222CED0(ctx, base);
	// 832911C4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832911C8: 386961E0  addi r3, r9, 0x61e0
	ctx.r[3].s64 = ctx.r[9].s64 + 25056;
	// 832911CC: 4BA18D55  bl 0x82ca9f20
	ctx.lr = 0x832911D0;
	sub_82CA9F20(ctx, base);
	// 832911D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832911D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832911D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832911DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832911E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832911E0 size=64
    let mut pc: u32 = 0x832911E0;
    'dispatch: loop {
        match pc {
            0x832911E0 => {
    //   block [0x832911E0..0x83291220)
	// 832911E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832911E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832911E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832911EC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832911F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832911F4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 832911F8: 386A5A14  addi r3, r10, 0x5a14
	ctx.r[3].s64 = ctx.r[10].s64 + 23060;
	// 832911FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291200: 4AF9BCD1  bl 0x8222ced0
	ctx.lr = 0x83291204;
	sub_8222CED0(ctx, base);
	// 83291204: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291208: 386961F0  addi r3, r9, 0x61f0
	ctx.r[3].s64 = ctx.r[9].s64 + 25072;
	// 8329120C: 4BA18D15  bl 0x82ca9f20
	ctx.lr = 0x83291210;
	sub_82CA9F20(ctx, base);
	// 83291210: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291214: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291218: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329121C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291220 size=64
    let mut pc: u32 = 0x83291220;
    'dispatch: loop {
        match pc {
            0x83291220 => {
    //   block [0x83291220..0x83291260)
	// 83291220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291228: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329122C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291230: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291234: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291238: 386A5A18  addi r3, r10, 0x5a18
	ctx.r[3].s64 = ctx.r[10].s64 + 23064;
	// 8329123C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291240: 4AF9BC91  bl 0x8222ced0
	ctx.lr = 0x83291244;
	sub_8222CED0(ctx, base);
	// 83291244: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291248: 38696200  addi r3, r9, 0x6200
	ctx.r[3].s64 = ctx.r[9].s64 + 25088;
	// 8329124C: 4BA18CD5  bl 0x82ca9f20
	ctx.lr = 0x83291250;
	sub_82CA9F20(ctx, base);
	// 83291250: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291254: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291258: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329125C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291260 size=64
    let mut pc: u32 = 0x83291260;
    'dispatch: loop {
        match pc {
            0x83291260 => {
    //   block [0x83291260..0x832912A0)
	// 83291260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291268: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329126C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291270: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291274: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291278: 386A5A1C  addi r3, r10, 0x5a1c
	ctx.r[3].s64 = ctx.r[10].s64 + 23068;
	// 8329127C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291280: 4AF9BC51  bl 0x8222ced0
	ctx.lr = 0x83291284;
	sub_8222CED0(ctx, base);
	// 83291284: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291288: 38696210  addi r3, r9, 0x6210
	ctx.r[3].s64 = ctx.r[9].s64 + 25104;
	// 8329128C: 4BA18C95  bl 0x82ca9f20
	ctx.lr = 0x83291290;
	sub_82CA9F20(ctx, base);
	// 83291290: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291294: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291298: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329129C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832912A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832912A0 size=64
    let mut pc: u32 = 0x832912A0;
    'dispatch: loop {
        match pc {
            0x832912A0 => {
    //   block [0x832912A0..0x832912E0)
	// 832912A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832912A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832912A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832912AC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832912B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832912B4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 832912B8: 386A5A20  addi r3, r10, 0x5a20
	ctx.r[3].s64 = ctx.r[10].s64 + 23072;
	// 832912BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832912C0: 4AF9BC11  bl 0x8222ced0
	ctx.lr = 0x832912C4;
	sub_8222CED0(ctx, base);
	// 832912C4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832912C8: 38696220  addi r3, r9, 0x6220
	ctx.r[3].s64 = ctx.r[9].s64 + 25120;
	// 832912CC: 4BA18C55  bl 0x82ca9f20
	ctx.lr = 0x832912D0;
	sub_82CA9F20(ctx, base);
	// 832912D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832912D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832912D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832912DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832912E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832912E0 size=64
    let mut pc: u32 = 0x832912E0;
    'dispatch: loop {
        match pc {
            0x832912E0 => {
    //   block [0x832912E0..0x83291320)
	// 832912E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832912E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832912E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832912EC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832912F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832912F4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 832912F8: 386A5A24  addi r3, r10, 0x5a24
	ctx.r[3].s64 = ctx.r[10].s64 + 23076;
	// 832912FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291300: 4AF9BBD1  bl 0x8222ced0
	ctx.lr = 0x83291304;
	sub_8222CED0(ctx, base);
	// 83291304: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291308: 38696230  addi r3, r9, 0x6230
	ctx.r[3].s64 = ctx.r[9].s64 + 25136;
	// 8329130C: 4BA18C15  bl 0x82ca9f20
	ctx.lr = 0x83291310;
	sub_82CA9F20(ctx, base);
	// 83291310: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291314: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291318: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329131C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291320 size=64
    let mut pc: u32 = 0x83291320;
    'dispatch: loop {
        match pc {
            0x83291320 => {
    //   block [0x83291320..0x83291360)
	// 83291320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291328: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329132C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291330: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291334: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291338: 386A5A28  addi r3, r10, 0x5a28
	ctx.r[3].s64 = ctx.r[10].s64 + 23080;
	// 8329133C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291340: 4AF9BB91  bl 0x8222ced0
	ctx.lr = 0x83291344;
	sub_8222CED0(ctx, base);
	// 83291344: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291348: 38696240  addi r3, r9, 0x6240
	ctx.r[3].s64 = ctx.r[9].s64 + 25152;
	// 8329134C: 4BA18BD5  bl 0x82ca9f20
	ctx.lr = 0x83291350;
	sub_82CA9F20(ctx, base);
	// 83291350: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291354: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291358: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329135C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291360 size=64
    let mut pc: u32 = 0x83291360;
    'dispatch: loop {
        match pc {
            0x83291360 => {
    //   block [0x83291360..0x832913A0)
	// 83291360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291368: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329136C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291370: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291374: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291378: 386A5A2C  addi r3, r10, 0x5a2c
	ctx.r[3].s64 = ctx.r[10].s64 + 23084;
	// 8329137C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291380: 4AF9BB51  bl 0x8222ced0
	ctx.lr = 0x83291384;
	sub_8222CED0(ctx, base);
	// 83291384: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291388: 38696250  addi r3, r9, 0x6250
	ctx.r[3].s64 = ctx.r[9].s64 + 25168;
	// 8329138C: 4BA18B95  bl 0x82ca9f20
	ctx.lr = 0x83291390;
	sub_82CA9F20(ctx, base);
	// 83291390: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291394: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291398: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329139C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832913A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832913A0 size=64
    let mut pc: u32 = 0x832913A0;
    'dispatch: loop {
        match pc {
            0x832913A0 => {
    //   block [0x832913A0..0x832913E0)
	// 832913A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832913A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832913A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832913AC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832913B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832913B4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 832913B8: 386A5A30  addi r3, r10, 0x5a30
	ctx.r[3].s64 = ctx.r[10].s64 + 23088;
	// 832913BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832913C0: 4AF9BB11  bl 0x8222ced0
	ctx.lr = 0x832913C4;
	sub_8222CED0(ctx, base);
	// 832913C4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832913C8: 38696260  addi r3, r9, 0x6260
	ctx.r[3].s64 = ctx.r[9].s64 + 25184;
	// 832913CC: 4BA18B55  bl 0x82ca9f20
	ctx.lr = 0x832913D0;
	sub_82CA9F20(ctx, base);
	// 832913D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832913D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832913D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832913DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832913E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832913E0 size=64
    let mut pc: u32 = 0x832913E0;
    'dispatch: loop {
        match pc {
            0x832913E0 => {
    //   block [0x832913E0..0x83291420)
	// 832913E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832913E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832913E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832913EC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832913F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832913F4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 832913F8: 386A5A34  addi r3, r10, 0x5a34
	ctx.r[3].s64 = ctx.r[10].s64 + 23092;
	// 832913FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291400: 4AF9BAD1  bl 0x8222ced0
	ctx.lr = 0x83291404;
	sub_8222CED0(ctx, base);
	// 83291404: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291408: 38696270  addi r3, r9, 0x6270
	ctx.r[3].s64 = ctx.r[9].s64 + 25200;
	// 8329140C: 4BA18B15  bl 0x82ca9f20
	ctx.lr = 0x83291410;
	sub_82CA9F20(ctx, base);
	// 83291410: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291414: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291418: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329141C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291420 size=64
    let mut pc: u32 = 0x83291420;
    'dispatch: loop {
        match pc {
            0x83291420 => {
    //   block [0x83291420..0x83291460)
	// 83291420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291428: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329142C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291430: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291434: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291438: 386A5A38  addi r3, r10, 0x5a38
	ctx.r[3].s64 = ctx.r[10].s64 + 23096;
	// 8329143C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291440: 4AF9BA91  bl 0x8222ced0
	ctx.lr = 0x83291444;
	sub_8222CED0(ctx, base);
	// 83291444: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291448: 38696280  addi r3, r9, 0x6280
	ctx.r[3].s64 = ctx.r[9].s64 + 25216;
	// 8329144C: 4BA18AD5  bl 0x82ca9f20
	ctx.lr = 0x83291450;
	sub_82CA9F20(ctx, base);
	// 83291450: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291454: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291458: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329145C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291460 size=64
    let mut pc: u32 = 0x83291460;
    'dispatch: loop {
        match pc {
            0x83291460 => {
    //   block [0x83291460..0x832914A0)
	// 83291460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291468: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329146C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291470: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291474: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291478: 386A5A3C  addi r3, r10, 0x5a3c
	ctx.r[3].s64 = ctx.r[10].s64 + 23100;
	// 8329147C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291480: 4AF9BA51  bl 0x8222ced0
	ctx.lr = 0x83291484;
	sub_8222CED0(ctx, base);
	// 83291484: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291488: 38696290  addi r3, r9, 0x6290
	ctx.r[3].s64 = ctx.r[9].s64 + 25232;
	// 8329148C: 4BA18A95  bl 0x82ca9f20
	ctx.lr = 0x83291490;
	sub_82CA9F20(ctx, base);
	// 83291490: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291494: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291498: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329149C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832914A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832914A0 size=64
    let mut pc: u32 = 0x832914A0;
    'dispatch: loop {
        match pc {
            0x832914A0 => {
    //   block [0x832914A0..0x832914E0)
	// 832914A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832914A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832914A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832914AC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832914B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832914B4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 832914B8: 386A5A40  addi r3, r10, 0x5a40
	ctx.r[3].s64 = ctx.r[10].s64 + 23104;
	// 832914BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832914C0: 4AF9BA11  bl 0x8222ced0
	ctx.lr = 0x832914C4;
	sub_8222CED0(ctx, base);
	// 832914C4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832914C8: 386962A0  addi r3, r9, 0x62a0
	ctx.r[3].s64 = ctx.r[9].s64 + 25248;
	// 832914CC: 4BA18A55  bl 0x82ca9f20
	ctx.lr = 0x832914D0;
	sub_82CA9F20(ctx, base);
	// 832914D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832914D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832914D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832914DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832914E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832914E0 size=64
    let mut pc: u32 = 0x832914E0;
    'dispatch: loop {
        match pc {
            0x832914E0 => {
    //   block [0x832914E0..0x83291520)
	// 832914E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832914E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832914E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832914EC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832914F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832914F4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 832914F8: 386A5A44  addi r3, r10, 0x5a44
	ctx.r[3].s64 = ctx.r[10].s64 + 23108;
	// 832914FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291500: 4AF9B9D1  bl 0x8222ced0
	ctx.lr = 0x83291504;
	sub_8222CED0(ctx, base);
	// 83291504: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291508: 386962B0  addi r3, r9, 0x62b0
	ctx.r[3].s64 = ctx.r[9].s64 + 25264;
	// 8329150C: 4BA18A15  bl 0x82ca9f20
	ctx.lr = 0x83291510;
	sub_82CA9F20(ctx, base);
	// 83291510: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291514: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291518: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329151C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291520 size=64
    let mut pc: u32 = 0x83291520;
    'dispatch: loop {
        match pc {
            0x83291520 => {
    //   block [0x83291520..0x83291560)
	// 83291520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291528: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329152C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291530: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291534: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291538: 386A5A48  addi r3, r10, 0x5a48
	ctx.r[3].s64 = ctx.r[10].s64 + 23112;
	// 8329153C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291540: 4AF9B991  bl 0x8222ced0
	ctx.lr = 0x83291544;
	sub_8222CED0(ctx, base);
	// 83291544: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291548: 386962C0  addi r3, r9, 0x62c0
	ctx.r[3].s64 = ctx.r[9].s64 + 25280;
	// 8329154C: 4BA189D5  bl 0x82ca9f20
	ctx.lr = 0x83291550;
	sub_82CA9F20(ctx, base);
	// 83291550: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291554: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291558: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329155C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291560 size=64
    let mut pc: u32 = 0x83291560;
    'dispatch: loop {
        match pc {
            0x83291560 => {
    //   block [0x83291560..0x832915A0)
	// 83291560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291568: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329156C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291570: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291574: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291578: 386A5A4C  addi r3, r10, 0x5a4c
	ctx.r[3].s64 = ctx.r[10].s64 + 23116;
	// 8329157C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291580: 4AF9B951  bl 0x8222ced0
	ctx.lr = 0x83291584;
	sub_8222CED0(ctx, base);
	// 83291584: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291588: 386962D0  addi r3, r9, 0x62d0
	ctx.r[3].s64 = ctx.r[9].s64 + 25296;
	// 8329158C: 4BA18995  bl 0x82ca9f20
	ctx.lr = 0x83291590;
	sub_82CA9F20(ctx, base);
	// 83291590: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291594: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291598: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329159C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832915A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832915A0 size=64
    let mut pc: u32 = 0x832915A0;
    'dispatch: loop {
        match pc {
            0x832915A0 => {
    //   block [0x832915A0..0x832915E0)
	// 832915A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832915A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832915A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832915AC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832915B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832915B4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 832915B8: 386A5A50  addi r3, r10, 0x5a50
	ctx.r[3].s64 = ctx.r[10].s64 + 23120;
	// 832915BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832915C0: 4AF9B911  bl 0x8222ced0
	ctx.lr = 0x832915C4;
	sub_8222CED0(ctx, base);
	// 832915C4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832915C8: 386962E0  addi r3, r9, 0x62e0
	ctx.r[3].s64 = ctx.r[9].s64 + 25312;
	// 832915CC: 4BA18955  bl 0x82ca9f20
	ctx.lr = 0x832915D0;
	sub_82CA9F20(ctx, base);
	// 832915D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832915D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832915D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832915DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832915E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832915E0 size=64
    let mut pc: u32 = 0x832915E0;
    'dispatch: loop {
        match pc {
            0x832915E0 => {
    //   block [0x832915E0..0x83291620)
	// 832915E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832915E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832915E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832915EC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832915F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832915F4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 832915F8: 386A5A54  addi r3, r10, 0x5a54
	ctx.r[3].s64 = ctx.r[10].s64 + 23124;
	// 832915FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291600: 4AF9B8D1  bl 0x8222ced0
	ctx.lr = 0x83291604;
	sub_8222CED0(ctx, base);
	// 83291604: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291608: 386962F0  addi r3, r9, 0x62f0
	ctx.r[3].s64 = ctx.r[9].s64 + 25328;
	// 8329160C: 4BA18915  bl 0x82ca9f20
	ctx.lr = 0x83291610;
	sub_82CA9F20(ctx, base);
	// 83291610: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291614: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291618: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329161C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83291620 size=88
    let mut pc: u32 = 0x83291620;
    'dispatch: loop {
        match pc {
            0x83291620 => {
    //   block [0x83291620..0x83291678)
	// 83291620: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83291624: 3941FFF0  addi r10, r1, -0x10
	ctx.r[10].s64 = ctx.r[1].s64 + -16;
	// 83291628: 3921FFF4  addi r9, r1, -0xc
	ctx.r[9].s64 = ctx.r[1].s64 + -12;
	// 8329162C: 3901FFF4  addi r8, r1, -0xc
	ctx.r[8].s64 = ctx.r[1].s64 + -12;
	// 83291630: 38E1FFF0  addi r7, r1, -0x10
	ctx.r[7].s64 = ctx.r[1].s64 + -16;
	// 83291634: C00B9490  lfs f0, -0x6b70(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83291638: 3CC0834A  lis r6, -0x7cb6
	ctx.r[6].s64 = -2092302336;
	// 8329163C: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83291678 size=132
    let mut pc: u32 = 0x83291678;
    'dispatch: loop {
        match pc {
            0x83291678 => {
    //   block [0x83291678..0x832916FC)
	// 83291678: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8329167C: 3941FFF0  addi r10, r1, -0x10
	ctx.r[10].s64 = ctx.r[1].s64 + -16;
	// 83291680: 3921FFF0  addi r9, r1, -0x10
	ctx.r[9].s64 = ctx.r[1].s64 + -16;
	// 83291684: 3901FFF0  addi r8, r1, -0x10
	ctx.r[8].s64 = ctx.r[1].s64 + -16;
	// 83291688: 3CE0820A  lis r7, -0x7df6
	ctx.r[7].s64 = -2113273856;
	// 8329168C: C00B9490  lfs f0, -0x6b70(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83291690: 3CC0820A  lis r6, -0x7df6
	ctx.r[6].s64 = -2113273856;
	// 83291694: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 83291698: 3CA0820A  lis r5, -0x7df6
	ctx.r[5].s64 = -2113273856;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291700 size=64
    let mut pc: u32 = 0x83291700;
    'dispatch: loop {
        match pc {
            0x83291700 => {
    //   block [0x83291700..0x83291740)
	// 83291700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291708: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329170C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291710: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291714: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291718: 386A5AA0  addi r3, r10, 0x5aa0
	ctx.r[3].s64 = ctx.r[10].s64 + 23200;
	// 8329171C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291720: 4AF9B7B1  bl 0x8222ced0
	ctx.lr = 0x83291724;
	sub_8222CED0(ctx, base);
	// 83291724: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291728: 38696300  addi r3, r9, 0x6300
	ctx.r[3].s64 = ctx.r[9].s64 + 25344;
	// 8329172C: 4BA187F5  bl 0x82ca9f20
	ctx.lr = 0x83291730;
	sub_82CA9F20(ctx, base);
	// 83291730: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291734: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291738: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329173C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291740 size=64
    let mut pc: u32 = 0x83291740;
    'dispatch: loop {
        match pc {
            0x83291740 => {
    //   block [0x83291740..0x83291780)
	// 83291740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291748: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329174C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291750: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291754: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291758: 386A5AA4  addi r3, r10, 0x5aa4
	ctx.r[3].s64 = ctx.r[10].s64 + 23204;
	// 8329175C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291760: 4AF9B771  bl 0x8222ced0
	ctx.lr = 0x83291764;
	sub_8222CED0(ctx, base);
	// 83291764: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291768: 38696310  addi r3, r9, 0x6310
	ctx.r[3].s64 = ctx.r[9].s64 + 25360;
	// 8329176C: 4BA187B5  bl 0x82ca9f20
	ctx.lr = 0x83291770;
	sub_82CA9F20(ctx, base);
	// 83291770: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291774: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291778: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329177C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291780 size=64
    let mut pc: u32 = 0x83291780;
    'dispatch: loop {
        match pc {
            0x83291780 => {
    //   block [0x83291780..0x832917C0)
	// 83291780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291788: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329178C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291790: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291794: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291798: 386A5AA8  addi r3, r10, 0x5aa8
	ctx.r[3].s64 = ctx.r[10].s64 + 23208;
	// 8329179C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832917A0: 4AF9B731  bl 0x8222ced0
	ctx.lr = 0x832917A4;
	sub_8222CED0(ctx, base);
	// 832917A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832917A8: 38696320  addi r3, r9, 0x6320
	ctx.r[3].s64 = ctx.r[9].s64 + 25376;
	// 832917AC: 4BA18775  bl 0x82ca9f20
	ctx.lr = 0x832917B0;
	sub_82CA9F20(ctx, base);
	// 832917B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832917B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832917B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832917BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832917C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832917C0 size=64
    let mut pc: u32 = 0x832917C0;
    'dispatch: loop {
        match pc {
            0x832917C0 => {
    //   block [0x832917C0..0x83291800)
	// 832917C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832917C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832917C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832917CC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832917D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832917D4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 832917D8: 386A5AAC  addi r3, r10, 0x5aac
	ctx.r[3].s64 = ctx.r[10].s64 + 23212;
	// 832917DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832917E0: 4AF9B6F1  bl 0x8222ced0
	ctx.lr = 0x832917E4;
	sub_8222CED0(ctx, base);
	// 832917E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832917E8: 38696330  addi r3, r9, 0x6330
	ctx.r[3].s64 = ctx.r[9].s64 + 25392;
	// 832917EC: 4BA18735  bl 0x82ca9f20
	ctx.lr = 0x832917F0;
	sub_82CA9F20(ctx, base);
	// 832917F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832917F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832917F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832917FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291800 size=64
    let mut pc: u32 = 0x83291800;
    'dispatch: loop {
        match pc {
            0x83291800 => {
    //   block [0x83291800..0x83291840)
	// 83291800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291808: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329180C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291810: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291814: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291818: 386A5AB0  addi r3, r10, 0x5ab0
	ctx.r[3].s64 = ctx.r[10].s64 + 23216;
	// 8329181C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291820: 4AF9B6B1  bl 0x8222ced0
	ctx.lr = 0x83291824;
	sub_8222CED0(ctx, base);
	// 83291824: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291828: 38696340  addi r3, r9, 0x6340
	ctx.r[3].s64 = ctx.r[9].s64 + 25408;
	// 8329182C: 4BA186F5  bl 0x82ca9f20
	ctx.lr = 0x83291830;
	sub_82CA9F20(ctx, base);
	// 83291830: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291834: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291838: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329183C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291840 size=64
    let mut pc: u32 = 0x83291840;
    'dispatch: loop {
        match pc {
            0x83291840 => {
    //   block [0x83291840..0x83291880)
	// 83291840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291848: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329184C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291850: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291854: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291858: 386A5AB4  addi r3, r10, 0x5ab4
	ctx.r[3].s64 = ctx.r[10].s64 + 23220;
	// 8329185C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291860: 4AF9B671  bl 0x8222ced0
	ctx.lr = 0x83291864;
	sub_8222CED0(ctx, base);
	// 83291864: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291868: 38696350  addi r3, r9, 0x6350
	ctx.r[3].s64 = ctx.r[9].s64 + 25424;
	// 8329186C: 4BA186B5  bl 0x82ca9f20
	ctx.lr = 0x83291870;
	sub_82CA9F20(ctx, base);
	// 83291870: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291874: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291878: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329187C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291880 size=64
    let mut pc: u32 = 0x83291880;
    'dispatch: loop {
        match pc {
            0x83291880 => {
    //   block [0x83291880..0x832918C0)
	// 83291880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291888: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329188C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291890: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291894: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291898: 386A5AB8  addi r3, r10, 0x5ab8
	ctx.r[3].s64 = ctx.r[10].s64 + 23224;
	// 8329189C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832918A0: 4AF9B631  bl 0x8222ced0
	ctx.lr = 0x832918A4;
	sub_8222CED0(ctx, base);
	// 832918A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832918A8: 38696360  addi r3, r9, 0x6360
	ctx.r[3].s64 = ctx.r[9].s64 + 25440;
	// 832918AC: 4BA18675  bl 0x82ca9f20
	ctx.lr = 0x832918B0;
	sub_82CA9F20(ctx, base);
	// 832918B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832918B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832918B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832918BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832918C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832918C0 size=64
    let mut pc: u32 = 0x832918C0;
    'dispatch: loop {
        match pc {
            0x832918C0 => {
    //   block [0x832918C0..0x83291900)
	// 832918C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832918C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832918C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832918CC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832918D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832918D4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 832918D8: 386A5ABC  addi r3, r10, 0x5abc
	ctx.r[3].s64 = ctx.r[10].s64 + 23228;
	// 832918DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832918E0: 4AF9B5F1  bl 0x8222ced0
	ctx.lr = 0x832918E4;
	sub_8222CED0(ctx, base);
	// 832918E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832918E8: 38696370  addi r3, r9, 0x6370
	ctx.r[3].s64 = ctx.r[9].s64 + 25456;
	// 832918EC: 4BA18635  bl 0x82ca9f20
	ctx.lr = 0x832918F0;
	sub_82CA9F20(ctx, base);
	// 832918F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832918F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832918F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832918FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291900 size=64
    let mut pc: u32 = 0x83291900;
    'dispatch: loop {
        match pc {
            0x83291900 => {
    //   block [0x83291900..0x83291940)
	// 83291900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291908: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329190C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291910: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291914: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291918: 386A5AC0  addi r3, r10, 0x5ac0
	ctx.r[3].s64 = ctx.r[10].s64 + 23232;
	// 8329191C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291920: 4AF9B5B1  bl 0x8222ced0
	ctx.lr = 0x83291924;
	sub_8222CED0(ctx, base);
	// 83291924: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291928: 38696380  addi r3, r9, 0x6380
	ctx.r[3].s64 = ctx.r[9].s64 + 25472;
	// 8329192C: 4BA185F5  bl 0x82ca9f20
	ctx.lr = 0x83291930;
	sub_82CA9F20(ctx, base);
	// 83291930: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291934: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291938: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329193C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291940 size=64
    let mut pc: u32 = 0x83291940;
    'dispatch: loop {
        match pc {
            0x83291940 => {
    //   block [0x83291940..0x83291980)
	// 83291940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291948: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329194C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291950: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291954: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291958: 386A5AC4  addi r3, r10, 0x5ac4
	ctx.r[3].s64 = ctx.r[10].s64 + 23236;
	// 8329195C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291960: 4AF9B571  bl 0x8222ced0
	ctx.lr = 0x83291964;
	sub_8222CED0(ctx, base);
	// 83291964: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291968: 38696390  addi r3, r9, 0x6390
	ctx.r[3].s64 = ctx.r[9].s64 + 25488;
	// 8329196C: 4BA185B5  bl 0x82ca9f20
	ctx.lr = 0x83291970;
	sub_82CA9F20(ctx, base);
	// 83291970: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291974: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291978: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329197C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291980 size=64
    let mut pc: u32 = 0x83291980;
    'dispatch: loop {
        match pc {
            0x83291980 => {
    //   block [0x83291980..0x832919C0)
	// 83291980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291988: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329198C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291990: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291994: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291998: 386A5AC8  addi r3, r10, 0x5ac8
	ctx.r[3].s64 = ctx.r[10].s64 + 23240;
	// 8329199C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832919A0: 4AF9B531  bl 0x8222ced0
	ctx.lr = 0x832919A4;
	sub_8222CED0(ctx, base);
	// 832919A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832919A8: 386963A0  addi r3, r9, 0x63a0
	ctx.r[3].s64 = ctx.r[9].s64 + 25504;
	// 832919AC: 4BA18575  bl 0x82ca9f20
	ctx.lr = 0x832919B0;
	sub_82CA9F20(ctx, base);
	// 832919B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832919B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832919B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832919BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832919C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832919C0 size=64
    let mut pc: u32 = 0x832919C0;
    'dispatch: loop {
        match pc {
            0x832919C0 => {
    //   block [0x832919C0..0x83291A00)
	// 832919C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832919C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832919C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832919CC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832919D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832919D4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 832919D8: 386A5ACC  addi r3, r10, 0x5acc
	ctx.r[3].s64 = ctx.r[10].s64 + 23244;
	// 832919DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832919E0: 4AF9B4F1  bl 0x8222ced0
	ctx.lr = 0x832919E4;
	sub_8222CED0(ctx, base);
	// 832919E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832919E8: 386963B0  addi r3, r9, 0x63b0
	ctx.r[3].s64 = ctx.r[9].s64 + 25520;
	// 832919EC: 4BA18535  bl 0x82ca9f20
	ctx.lr = 0x832919F0;
	sub_82CA9F20(ctx, base);
	// 832919F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832919F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832919F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832919FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291A00 size=64
    let mut pc: u32 = 0x83291A00;
    'dispatch: loop {
        match pc {
            0x83291A00 => {
    //   block [0x83291A00..0x83291A40)
	// 83291A00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291A04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291A08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83291A0C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291A10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291A14: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291A18: 386A5AD0  addi r3, r10, 0x5ad0
	ctx.r[3].s64 = ctx.r[10].s64 + 23248;
	// 83291A1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291A20: 4AF9B4B1  bl 0x8222ced0
	ctx.lr = 0x83291A24;
	sub_8222CED0(ctx, base);
	// 83291A24: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291A28: 386963C0  addi r3, r9, 0x63c0
	ctx.r[3].s64 = ctx.r[9].s64 + 25536;
	// 83291A2C: 4BA184F5  bl 0x82ca9f20
	ctx.lr = 0x83291A30;
	sub_82CA9F20(ctx, base);
	// 83291A30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291A34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291A38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83291A3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291A40 size=64
    let mut pc: u32 = 0x83291A40;
    'dispatch: loop {
        match pc {
            0x83291A40 => {
    //   block [0x83291A40..0x83291A80)
	// 83291A40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291A44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291A48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83291A4C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291A50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291A54: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291A58: 386A5AD4  addi r3, r10, 0x5ad4
	ctx.r[3].s64 = ctx.r[10].s64 + 23252;
	// 83291A5C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291A60: 4AF9B471  bl 0x8222ced0
	ctx.lr = 0x83291A64;
	sub_8222CED0(ctx, base);
	// 83291A64: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291A68: 386963D0  addi r3, r9, 0x63d0
	ctx.r[3].s64 = ctx.r[9].s64 + 25552;
	// 83291A6C: 4BA184B5  bl 0x82ca9f20
	ctx.lr = 0x83291A70;
	sub_82CA9F20(ctx, base);
	// 83291A70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291A74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291A78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83291A7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291A80 size=64
    let mut pc: u32 = 0x83291A80;
    'dispatch: loop {
        match pc {
            0x83291A80 => {
    //   block [0x83291A80..0x83291AC0)
	// 83291A80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291A84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291A88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83291A8C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291A90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291A94: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291A98: 386A5AD8  addi r3, r10, 0x5ad8
	ctx.r[3].s64 = ctx.r[10].s64 + 23256;
	// 83291A9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291AA0: 4AF9B431  bl 0x8222ced0
	ctx.lr = 0x83291AA4;
	sub_8222CED0(ctx, base);
	// 83291AA4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291AA8: 386963E0  addi r3, r9, 0x63e0
	ctx.r[3].s64 = ctx.r[9].s64 + 25568;
	// 83291AAC: 4BA18475  bl 0x82ca9f20
	ctx.lr = 0x83291AB0;
	sub_82CA9F20(ctx, base);
	// 83291AB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291AB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291AB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83291ABC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291AC0 size=64
    let mut pc: u32 = 0x83291AC0;
    'dispatch: loop {
        match pc {
            0x83291AC0 => {
    //   block [0x83291AC0..0x83291B00)
	// 83291AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291AC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291AC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83291ACC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291AD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291AD4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291AD8: 386A5ADC  addi r3, r10, 0x5adc
	ctx.r[3].s64 = ctx.r[10].s64 + 23260;
	// 83291ADC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291AE0: 4AF9B3F1  bl 0x8222ced0
	ctx.lr = 0x83291AE4;
	sub_8222CED0(ctx, base);
	// 83291AE4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291AE8: 386963F0  addi r3, r9, 0x63f0
	ctx.r[3].s64 = ctx.r[9].s64 + 25584;
	// 83291AEC: 4BA18435  bl 0x82ca9f20
	ctx.lr = 0x83291AF0;
	sub_82CA9F20(ctx, base);
	// 83291AF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291AF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291AF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83291AFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291B00 size=64
    let mut pc: u32 = 0x83291B00;
    'dispatch: loop {
        match pc {
            0x83291B00 => {
    //   block [0x83291B00..0x83291B40)
	// 83291B00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291B04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291B08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83291B0C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291B10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291B14: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291B18: 386A5AE0  addi r3, r10, 0x5ae0
	ctx.r[3].s64 = ctx.r[10].s64 + 23264;
	// 83291B1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291B20: 4AF9B3B1  bl 0x8222ced0
	ctx.lr = 0x83291B24;
	sub_8222CED0(ctx, base);
	// 83291B24: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291B28: 38696400  addi r3, r9, 0x6400
	ctx.r[3].s64 = ctx.r[9].s64 + 25600;
	// 83291B2C: 4BA183F5  bl 0x82ca9f20
	ctx.lr = 0x83291B30;
	sub_82CA9F20(ctx, base);
	// 83291B30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291B34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291B38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83291B3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291B40 size=64
    let mut pc: u32 = 0x83291B40;
    'dispatch: loop {
        match pc {
            0x83291B40 => {
    //   block [0x83291B40..0x83291B80)
	// 83291B40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291B44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291B48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83291B4C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291B50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291B54: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291B58: 386A5AE4  addi r3, r10, 0x5ae4
	ctx.r[3].s64 = ctx.r[10].s64 + 23268;
	// 83291B5C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291B60: 4AF9B371  bl 0x8222ced0
	ctx.lr = 0x83291B64;
	sub_8222CED0(ctx, base);
	// 83291B64: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291B68: 38696410  addi r3, r9, 0x6410
	ctx.r[3].s64 = ctx.r[9].s64 + 25616;
	// 83291B6C: 4BA183B5  bl 0x82ca9f20
	ctx.lr = 0x83291B70;
	sub_82CA9F20(ctx, base);
	// 83291B70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291B74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291B78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83291B7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83291B80 size=12
    let mut pc: u32 = 0x83291B80;
    'dispatch: loop {
        match pc {
            0x83291B80 => {
    //   block [0x83291B80..0x83291B8C)
	// 83291B80: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83291B84: 386B5AE8  addi r3, r11, 0x5ae8
	ctx.r[3].s64 = ctx.r[11].s64 + 23272;
	// 83291B88: 4B829AF8  b 0x82abb680
	sub_82ABB680(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291B90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291B90 size=64
    let mut pc: u32 = 0x83291B90;
    'dispatch: loop {
        match pc {
            0x83291B90 => {
    //   block [0x83291B90..0x83291BD0)
	// 83291B90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291B94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291B98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83291B9C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291BA0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291BA4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291BA8: 386A5D2C  addi r3, r10, 0x5d2c
	ctx.r[3].s64 = ctx.r[10].s64 + 23852;
	// 83291BAC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291BB0: 4AF9B321  bl 0x8222ced0
	ctx.lr = 0x83291BB4;
	sub_8222CED0(ctx, base);
	// 83291BB4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291BB8: 38696420  addi r3, r9, 0x6420
	ctx.r[3].s64 = ctx.r[9].s64 + 25632;
	// 83291BBC: 4BA18365  bl 0x82ca9f20
	ctx.lr = 0x83291BC0;
	sub_82CA9F20(ctx, base);
	// 83291BC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291BC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291BC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83291BCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291BD0 size=64
    let mut pc: u32 = 0x83291BD0;
    'dispatch: loop {
        match pc {
            0x83291BD0 => {
    //   block [0x83291BD0..0x83291C10)
	// 83291BD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291BD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291BD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83291BDC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291BE0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291BE4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291BE8: 386A5D30  addi r3, r10, 0x5d30
	ctx.r[3].s64 = ctx.r[10].s64 + 23856;
	// 83291BEC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291BF0: 4AF9B2E1  bl 0x8222ced0
	ctx.lr = 0x83291BF4;
	sub_8222CED0(ctx, base);
	// 83291BF4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291BF8: 38696430  addi r3, r9, 0x6430
	ctx.r[3].s64 = ctx.r[9].s64 + 25648;
	// 83291BFC: 4BA18325  bl 0x82ca9f20
	ctx.lr = 0x83291C00;
	sub_82CA9F20(ctx, base);
	// 83291C00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291C04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291C08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83291C0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291C10 size=64
    let mut pc: u32 = 0x83291C10;
    'dispatch: loop {
        match pc {
            0x83291C10 => {
    //   block [0x83291C10..0x83291C50)
	// 83291C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291C14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291C18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83291C1C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291C20: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291C24: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291C28: 386A5D34  addi r3, r10, 0x5d34
	ctx.r[3].s64 = ctx.r[10].s64 + 23860;
	// 83291C2C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291C30: 4AF9B2A1  bl 0x8222ced0
	ctx.lr = 0x83291C34;
	sub_8222CED0(ctx, base);
	// 83291C34: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291C38: 38696440  addi r3, r9, 0x6440
	ctx.r[3].s64 = ctx.r[9].s64 + 25664;
	// 83291C3C: 4BA182E5  bl 0x82ca9f20
	ctx.lr = 0x83291C40;
	sub_82CA9F20(ctx, base);
	// 83291C40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291C44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291C48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83291C4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291C50 size=64
    let mut pc: u32 = 0x83291C50;
    'dispatch: loop {
        match pc {
            0x83291C50 => {
    //   block [0x83291C50..0x83291C90)
	// 83291C50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291C54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291C58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83291C5C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291C60: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291C64: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291C68: 386A5D38  addi r3, r10, 0x5d38
	ctx.r[3].s64 = ctx.r[10].s64 + 23864;
	// 83291C6C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291C70: 4AF9B261  bl 0x8222ced0
	ctx.lr = 0x83291C74;
	sub_8222CED0(ctx, base);
	// 83291C74: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291C78: 38696450  addi r3, r9, 0x6450
	ctx.r[3].s64 = ctx.r[9].s64 + 25680;
	// 83291C7C: 4BA182A5  bl 0x82ca9f20
	ctx.lr = 0x83291C80;
	sub_82CA9F20(ctx, base);
	// 83291C80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291C84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291C88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83291C8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291C90 size=64
    let mut pc: u32 = 0x83291C90;
    'dispatch: loop {
        match pc {
            0x83291C90 => {
    //   block [0x83291C90..0x83291CD0)
	// 83291C90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291C94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291C98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83291C9C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291CA0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291CA4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291CA8: 386A5D3C  addi r3, r10, 0x5d3c
	ctx.r[3].s64 = ctx.r[10].s64 + 23868;
	// 83291CAC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291CB0: 4AF9B221  bl 0x8222ced0
	ctx.lr = 0x83291CB4;
	sub_8222CED0(ctx, base);
	// 83291CB4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291CB8: 38696460  addi r3, r9, 0x6460
	ctx.r[3].s64 = ctx.r[9].s64 + 25696;
	// 83291CBC: 4BA18265  bl 0x82ca9f20
	ctx.lr = 0x83291CC0;
	sub_82CA9F20(ctx, base);
	// 83291CC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291CC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291CC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83291CCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291CD0 size=64
    let mut pc: u32 = 0x83291CD0;
    'dispatch: loop {
        match pc {
            0x83291CD0 => {
    //   block [0x83291CD0..0x83291D10)
	// 83291CD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291CD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291CD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83291CDC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291CE0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291CE4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291CE8: 386A5D40  addi r3, r10, 0x5d40
	ctx.r[3].s64 = ctx.r[10].s64 + 23872;
	// 83291CEC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291CF0: 4AF9B1E1  bl 0x8222ced0
	ctx.lr = 0x83291CF4;
	sub_8222CED0(ctx, base);
	// 83291CF4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291CF8: 38696470  addi r3, r9, 0x6470
	ctx.r[3].s64 = ctx.r[9].s64 + 25712;
	// 83291CFC: 4BA18225  bl 0x82ca9f20
	ctx.lr = 0x83291D00;
	sub_82CA9F20(ctx, base);
	// 83291D00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291D04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291D08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83291D0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291D10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291D10 size=64
    let mut pc: u32 = 0x83291D10;
    'dispatch: loop {
        match pc {
            0x83291D10 => {
    //   block [0x83291D10..0x83291D50)
	// 83291D10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291D14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291D18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83291D1C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291D20: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291D24: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291D28: 386A5D44  addi r3, r10, 0x5d44
	ctx.r[3].s64 = ctx.r[10].s64 + 23876;
	// 83291D2C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291D30: 4AF9B1A1  bl 0x8222ced0
	ctx.lr = 0x83291D34;
	sub_8222CED0(ctx, base);
	// 83291D34: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291D38: 38696480  addi r3, r9, 0x6480
	ctx.r[3].s64 = ctx.r[9].s64 + 25728;
	// 83291D3C: 4BA181E5  bl 0x82ca9f20
	ctx.lr = 0x83291D40;
	sub_82CA9F20(ctx, base);
	// 83291D40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291D44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291D48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83291D4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291D50 size=64
    let mut pc: u32 = 0x83291D50;
    'dispatch: loop {
        match pc {
            0x83291D50 => {
    //   block [0x83291D50..0x83291D90)
	// 83291D50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291D54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291D58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83291D5C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291D60: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291D64: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291D68: 386A5D48  addi r3, r10, 0x5d48
	ctx.r[3].s64 = ctx.r[10].s64 + 23880;
	// 83291D6C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291D70: 4AF9B161  bl 0x8222ced0
	ctx.lr = 0x83291D74;
	sub_8222CED0(ctx, base);
	// 83291D74: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291D78: 38696490  addi r3, r9, 0x6490
	ctx.r[3].s64 = ctx.r[9].s64 + 25744;
	// 83291D7C: 4BA181A5  bl 0x82ca9f20
	ctx.lr = 0x83291D80;
	sub_82CA9F20(ctx, base);
	// 83291D80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291D84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291D88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83291D8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83291D90 size=12
    let mut pc: u32 = 0x83291D90;
    'dispatch: loop {
        match pc {
            0x83291D90 => {
    //   block [0x83291D90..0x83291D9C)
	// 83291D90: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83291D94: 386B64A0  addi r3, r11, 0x64a0
	ctx.r[3].s64 = ctx.r[11].s64 + 25760;
	// 83291D98: 4BA18188  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291DA0 size=64
    let mut pc: u32 = 0x83291DA0;
    'dispatch: loop {
        match pc {
            0x83291DA0 => {
    //   block [0x83291DA0..0x83291DE0)
	// 83291DA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291DA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291DA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83291DAC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291DB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291DB4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291DB8: 386A5D5C  addi r3, r10, 0x5d5c
	ctx.r[3].s64 = ctx.r[10].s64 + 23900;
	// 83291DBC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291DC0: 4AF9B111  bl 0x8222ced0
	ctx.lr = 0x83291DC4;
	sub_8222CED0(ctx, base);
	// 83291DC4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291DC8: 386964F8  addi r3, r9, 0x64f8
	ctx.r[3].s64 = ctx.r[9].s64 + 25848;
	// 83291DCC: 4BA18155  bl 0x82ca9f20
	ctx.lr = 0x83291DD0;
	sub_82CA9F20(ctx, base);
	// 83291DD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291DD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291DD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83291DDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291DE0 size=64
    let mut pc: u32 = 0x83291DE0;
    'dispatch: loop {
        match pc {
            0x83291DE0 => {
    //   block [0x83291DE0..0x83291E20)
	// 83291DE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291DE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291DE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83291DEC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83291DF0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83291DF4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83291DF8: 386A5D60  addi r3, r10, 0x5d60
	ctx.r[3].s64 = ctx.r[10].s64 + 23904;
	// 83291DFC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83291E00: 4AF9B0D1  bl 0x8222ced0
	ctx.lr = 0x83291E04;
	sub_8222CED0(ctx, base);
	// 83291E04: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83291E08: 38696508  addi r3, r9, 0x6508
	ctx.r[3].s64 = ctx.r[9].s64 + 25864;
	// 83291E0C: 4BA18115  bl 0x82ca9f20
	ctx.lr = 0x83291E10;
	sub_82CA9F20(ctx, base);
	// 83291E10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83291E14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83291E18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83291E1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83291E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83291E20 size=800
    let mut pc: u32 = 0x83291E20;
    'dispatch: loop {
        match pc {
            0x83291E20 => {
    //   block [0x83291E20..0x83292140)
	// 83291E20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291E24: 4BA175C1  bl 0x82ca93e4
	ctx.lr = 0x83291E28;
	sub_82CA93D0(ctx, base);
	// 83291E28: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83291E2C: 3941FF40  addi r10, r1, -0xc0
	ctx.r[10].s64 = ctx.r[1].s64 + -192;
	// 83291E30: 392B92D4  addi r9, r11, -0x6d2c
	ctx.r[9].s64 = ctx.r[11].s64 + -27948;
	// 83291E34: 3901FF44  addi r8, r1, -0xbc
	ctx.r[8].s64 = ctx.r[1].s64 + -188;
	// 83291E38: 38E1FF48  addi r7, r1, -0xb8
	ctx.r[7].s64 = ctx.r[1].s64 + -184;
	// 83291E3C: 38C1FF4C  addi r6, r1, -0xb4
	ctx.r[6].s64 = ctx.r[1].s64 + -180;
	// 83291E40: C18B92D4  lfs f12, -0x6d2c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27948 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 83291E44: 38A1FF50  addi r5, r1, -0xb0
	ctx.r[5].s64 = ctx.r[1].s64 + -176;
	// 83291E48: C00901B0  lfs f0, 0x1b0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(432 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83291E4C: 3881FF54  addi r4, r1, -0xac
	ctx.r[4].s64 = ctx.r[1].s64 + -172;
	// 83291E50: C1A901BC  lfs f13, 0x1bc(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(444 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 83291E54: 3861FF58  addi r3, r1, -0xa8
	ctx.r[3].s64 = ctx.r[1].s64 + -168;
	// 83291E58: D001FF40  stfs f0, -0xc0(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-192 as u32), tmp.u32 ) };
	// 83291E5C: 3921FF5C  addi r9, r1, -0xa4
	ctx.r[9].s64 = ctx.r[1].s64 + -164;
	// 83291E60: D1A1FF44  stfs f13, -0xbc(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-188 as u32), tmp.u32 ) };
	// 83291E64: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 83291E68: D1A1FF48  stfs f13, -0xb8(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-184 as u32), tmp.u32 ) };
	// 83291E6C: 3BC1FF64  addi r30, r1, -0x9c
	ctx.r[30].s64 = ctx.r[1].s64 + -156;
	// 83291E70: D001FF4C  stfs f0, -0xb4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-180 as u32), tmp.u32 ) };
	// 83291E74: 3BA1FF68  addi r29, r1, -0x98
	ctx.r[29].s64 = ctx.r[1].s64 + -152;
	// 83291E78: D001FF50  stfs f0, -0xb0(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-176 as u32), tmp.u32 ) };
	// 83291E7C: 3B81FF6C  addi r28, r1, -0x94
	ctx.r[28].s64 = ctx.r[1].s64 + -148;
	// 83291E80: D001FF54  stfs f0, -0xac(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-172 as u32), tmp.u32 ) };
	// 83291E84: 3B61FF70  addi r27, r1, -0x90
	ctx.r[27].s64 = ctx.r[1].s64 + -144;
	// 83291E88: D1A1FF58  stfs f13, -0xa8(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-168 as u32), tmp.u32 ) };
	// 83291E8C: 3B41FF74  addi r26, r1, -0x8c
	ctx.r[26].s64 = ctx.r[1].s64 + -140;
	// 83291E90: D1A1FF5C  stfs f13, -0xa4(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-164 as u32), tmp.u32 ) };
	// 83291E94: 3B21FF78  addi r25, r1, -0x88
	ctx.r[25].s64 = ctx.r[1].s64 + -136;
	// 83291E98: D001FF60  stfs f0, -0xa0(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), tmp.u32 ) };
	// 83291E9C: 3B01FF7C  addi r24, r1, -0x84
	ctx.r[24].s64 = ctx.r[1].s64 + -132;
	// 83291EA0: D001FF64  stfs f0, -0x9c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-156 as u32), tmp.u32 ) };
	// 83291EA4: 3AE1FF80  addi r23, r1, -0x80
	ctx.r[23].s64 = ctx.r[1].s64 + -128;
	// 83291EA8: D001FF68  stfs f0, -0x98(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-152 as u32), tmp.u32 ) };
	// 83291EAC: 3AC1FF84  addi r22, r1, -0x7c
	ctx.r[22].s64 = ctx.r[1].s64 + -124;
	// 83291EB0: D001FF6C  stfs f0, -0x94(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-148 as u32), tmp.u32 ) };
	// 83291EB4: 3AA1FF88  addi r21, r1, -0x78
	ctx.r[21].s64 = ctx.r[1].s64 + -120;
	// 83291EB8: D001FF70  stfs f0, -0x90(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-144 as u32), tmp.u32 ) };
	// 83291EBC: 3A81FF8C  addi r20, r1, -0x74
	ctx.r[20].s64 = ctx.r[1].s64 + -116;
	// 83291EC0: D1A1FF74  stfs f13, -0x8c(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-140 as u32), tmp.u32 ) };
	// 83291EC4: 3E608332  lis r19, -0x7cce
	ctx.r[19].s64 = -2093875200;
	// 83291EC8: D001FF78  stfs f0, -0x88(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-136 as u32), tmp.u32 ) };
	// 83291ECC: D001FF7C  stfs f0, -0x84(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-132 as u32), tmp.u32 ) };
	// 83291ED0: 3A73AE50  addi r19, r19, -0x51b0
	ctx.r[19].s64 = ctx.r[19].s64 + -20912;
	// 83291ED4: D001FF80  stfs f0, -0x80(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-128 as u32), tmp.u32 ) };
	// 83291ED8: D1A1FF84  stfs f13, -0x7c(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-124 as u32), tmp.u32 ) };
	// 83291EDC: D001FF88  stfs f0, -0x78(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-120 as u32), tmp.u32 ) };
	// 83291EE0: D1A1FF8C  stfs f13, -0x74(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-116 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83292140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83292140 size=456
    let mut pc: u32 = 0x83292140;
    'dispatch: loop {
        match pc {
            0x83292140 => {
    //   block [0x83292140..0x83292308)
	// 83292140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83292144: 4BA172A5  bl 0x82ca93e8
	ctx.lr = 0x83292148;
	sub_82CA93D0(ctx, base);
	// 83292148: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8329214C: 3901FF44  addi r8, r1, -0xbc
	ctx.r[8].s64 = ctx.r[1].s64 + -188;
	// 83292150: 392B9490  addi r9, r11, -0x6b70
	ctx.r[9].s64 = ctx.r[11].s64 + -27504;
	// 83292154: 3941FF40  addi r10, r1, -0xc0
	ctx.r[10].s64 = ctx.r[1].s64 + -192;
	// 83292158: 38E1FF48  addi r7, r1, -0xb8
	ctx.r[7].s64 = ctx.r[1].s64 + -184;
	// 8329215C: C1AB9490  lfs f13, -0x6b70(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 83292160: 38C1FF4C  addi r6, r1, -0xb4
	ctx.r[6].s64 = ctx.r[1].s64 + -180;
	// 83292164: D1A1FF40  stfs f13, -0xc0(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-192 as u32), tmp.u32 ) };
	// 83292168: 38A1FF50  addi r5, r1, -0xb0
	ctx.r[5].s64 = ctx.r[1].s64 + -176;
	// 8329216C: C009FFF4  lfs f0, -0xc(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83292170: 3881FF54  addi r4, r1, -0xac
	ctx.r[4].s64 = ctx.r[1].s64 + -172;
	// 83292174: D001FF44  stfs f0, -0xbc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-188 as u32), tmp.u32 ) };
	// 83292178: 3861FF58  addi r3, r1, -0xa8
	ctx.r[3].s64 = ctx.r[1].s64 + -168;
	// 8329217C: D001FF48  stfs f0, -0xb8(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-184 as u32), tmp.u32 ) };
	// 83292180: 3961FF5C  addi r11, r1, -0xa4
	ctx.r[11].s64 = ctx.r[1].s64 + -164;
	// 83292184: D001FF4C  stfs f0, -0xb4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-180 as u32), tmp.u32 ) };
	// 83292188: 3921FF60  addi r9, r1, -0xa0
	ctx.r[9].s64 = ctx.r[1].s64 + -160;
	// 8329218C: D1A1FF50  stfs f13, -0xb0(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-176 as u32), tmp.u32 ) };
	// 83292190: 3BE1FF64  addi r31, r1, -0x9c
	ctx.r[31].s64 = ctx.r[1].s64 + -156;
	// 83292194: D1A1FF54  stfs f13, -0xac(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-172 as u32), tmp.u32 ) };
	// 83292198: 3BC1FF68  addi r30, r1, -0x98
	ctx.r[30].s64 = ctx.r[1].s64 + -152;
	// 8329219C: D001FF58  stfs f0, -0xa8(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-168 as u32), tmp.u32 ) };
	// 832921A0: 3BA1FF6C  addi r29, r1, -0x94
	ctx.r[29].s64 = ctx.r[1].s64 + -148;
	// 832921A4: D001FF5C  stfs f0, -0xa4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-164 as u32), tmp.u32 ) };
	// 832921A8: 3B81FF70  addi r28, r1, -0x90
	ctx.r[28].s64 = ctx.r[1].s64 + -144;
	// 832921AC: D001FF60  stfs f0, -0xa0(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), tmp.u32 ) };
	// 832921B0: 3B61FF74  addi r27, r1, -0x8c
	ctx.r[27].s64 = ctx.r[1].s64 + -140;
	// 832921B4: D001FF64  stfs f0, -0x9c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-156 as u32), tmp.u32 ) };
	// 832921B8: 3B41FF78  addi r26, r1, -0x88
	ctx.r[26].s64 = ctx.r[1].s64 + -136;
	// 832921BC: D001FF68  stfs f0, -0x98(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-152 as u32), tmp.u32 ) };
	// 832921C0: 3B21FF7C  addi r25, r1, -0x84
	ctx.r[25].s64 = ctx.r[1].s64 + -132;
	// 832921C4: D1A1FF6C  stfs f13, -0x94(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-148 as u32), tmp.u32 ) };
	// 832921C8: 3B01FF80  addi r24, r1, -0x80
	ctx.r[24].s64 = ctx.r[1].s64 + -128;
	// 832921CC: D001FF70  stfs f0, -0x90(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-144 as u32), tmp.u32 ) };
	// 832921D0: 3AE1FF84  addi r23, r1, -0x7c
	ctx.r[23].s64 = ctx.r[1].s64 + -124;
	// 832921D4: D001FF74  stfs f0, -0x8c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-140 as u32), tmp.u32 ) };
	// 832921D8: 3AC1FF88  addi r22, r1, -0x78
	ctx.r[22].s64 = ctx.r[1].s64 + -120;
	// 832921DC: D001FF78  stfs f0, -0x88(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-136 as u32), tmp.u32 ) };
	// 832921E0: 3AA1FF8C  addi r21, r1, -0x74
	ctx.r[21].s64 = ctx.r[1].s64 + -116;
	// 832921E4: D001FF7C  stfs f0, -0x84(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-132 as u32), tmp.u32 ) };
	// 832921E8: 3E808332  lis r20, -0x7cce
	ctx.r[20].s64 = -2093875200;
	// 832921EC: D001FF80  stfs f0, -0x80(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-128 as u32), tmp.u32 ) };
	// 832921F0: D1A1FF84  stfs f13, -0x7c(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-124 as u32), tmp.u32 ) };
	// 832921F4: 3A94AFB0  addi r20, r20, -0x5050
	ctx.r[20].s64 = ctx.r[20].s64 + -20560;
	// 832921F8: D001FF88  stfs f0, -0x78(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-120 as u32), tmp.u32 ) };
	// 832921FC: D1A1FF8C  stfs f13, -0x74(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-116 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83292308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83292308 size=64
    let mut pc: u32 = 0x83292308;
    'dispatch: loop {
        match pc {
            0x83292308 => {
    //   block [0x83292308..0x83292348)
	// 83292308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329230C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83292310: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83292314: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83292318: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8329231C: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83292320: 386A5D64  addi r3, r10, 0x5d64
	ctx.r[3].s64 = ctx.r[10].s64 + 23908;
	// 83292324: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83292328: 4AF9ABA9  bl 0x8222ced0
	ctx.lr = 0x8329232C;
	sub_8222CED0(ctx, base);
	// 8329232C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83292330: 38696518  addi r3, r9, 0x6518
	ctx.r[3].s64 = ctx.r[9].s64 + 25880;
	// 83292334: 4BA17BED  bl 0x82ca9f20
	ctx.lr = 0x83292338;
	sub_82CA9F20(ctx, base);
	// 83292338: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8329233C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83292340: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83292344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83292348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83292348 size=64
    let mut pc: u32 = 0x83292348;
    'dispatch: loop {
        match pc {
            0x83292348 => {
    //   block [0x83292348..0x83292388)
	// 83292348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329234C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83292350: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83292354: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83292358: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8329235C: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83292360: 386A5D68  addi r3, r10, 0x5d68
	ctx.r[3].s64 = ctx.r[10].s64 + 23912;
	// 83292364: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83292368: 4AF9AB69  bl 0x8222ced0
	ctx.lr = 0x8329236C;
	sub_8222CED0(ctx, base);
	// 8329236C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83292370: 38696528  addi r3, r9, 0x6528
	ctx.r[3].s64 = ctx.r[9].s64 + 25896;
	// 83292374: 4BA17BAD  bl 0x82ca9f20
	ctx.lr = 0x83292378;
	sub_82CA9F20(ctx, base);
	// 83292378: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8329237C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83292380: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83292384: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83292388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83292388 size=64
    let mut pc: u32 = 0x83292388;
    'dispatch: loop {
        match pc {
            0x83292388 => {
    //   block [0x83292388..0x832923C8)
	// 83292388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329238C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83292390: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83292394: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83292398: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8329239C: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 832923A0: 386A5D6C  addi r3, r10, 0x5d6c
	ctx.r[3].s64 = ctx.r[10].s64 + 23916;
	// 832923A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832923A8: 4AF9AB29  bl 0x8222ced0
	ctx.lr = 0x832923AC;
	sub_8222CED0(ctx, base);
	// 832923AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832923B0: 38696538  addi r3, r9, 0x6538
	ctx.r[3].s64 = ctx.r[9].s64 + 25912;
	// 832923B4: 4BA17B6D  bl 0x82ca9f20
	ctx.lr = 0x832923B8;
	sub_82CA9F20(ctx, base);
	// 832923B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832923BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832923C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832923C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832923C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832923C8 size=64
    let mut pc: u32 = 0x832923C8;
    'dispatch: loop {
        match pc {
            0x832923C8 => {
    //   block [0x832923C8..0x83292408)
	// 832923C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832923CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832923D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832923D4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832923D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832923DC: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 832923E0: 386A5D70  addi r3, r10, 0x5d70
	ctx.r[3].s64 = ctx.r[10].s64 + 23920;
	// 832923E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832923E8: 4AF9AAE9  bl 0x8222ced0
	ctx.lr = 0x832923EC;
	sub_8222CED0(ctx, base);
	// 832923EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832923F0: 38696548  addi r3, r9, 0x6548
	ctx.r[3].s64 = ctx.r[9].s64 + 25928;
	// 832923F4: 4BA17B2D  bl 0x82ca9f20
	ctx.lr = 0x832923F8;
	sub_82CA9F20(ctx, base);
	// 832923F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832923FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83292400: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83292404: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83292408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83292408 size=64
    let mut pc: u32 = 0x83292408;
    'dispatch: loop {
        match pc {
            0x83292408 => {
    //   block [0x83292408..0x83292448)
	// 83292408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329240C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83292410: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83292414: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83292418: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8329241C: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83292420: 386A5D74  addi r3, r10, 0x5d74
	ctx.r[3].s64 = ctx.r[10].s64 + 23924;
	// 83292424: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83292428: 4AF9AAA9  bl 0x8222ced0
	ctx.lr = 0x8329242C;
	sub_8222CED0(ctx, base);
	// 8329242C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83292430: 38696558  addi r3, r9, 0x6558
	ctx.r[3].s64 = ctx.r[9].s64 + 25944;
	// 83292434: 4BA17AED  bl 0x82ca9f20
	ctx.lr = 0x83292438;
	sub_82CA9F20(ctx, base);
	// 83292438: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8329243C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83292440: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83292444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83292448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83292448 size=64
    let mut pc: u32 = 0x83292448;
    'dispatch: loop {
        match pc {
            0x83292448 => {
    //   block [0x83292448..0x83292488)
	// 83292448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329244C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83292450: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83292454: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83292458: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8329245C: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83292460: 386A5D78  addi r3, r10, 0x5d78
	ctx.r[3].s64 = ctx.r[10].s64 + 23928;
	// 83292464: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83292468: 4AF9AA69  bl 0x8222ced0
	ctx.lr = 0x8329246C;
	sub_8222CED0(ctx, base);
	// 8329246C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83292470: 38696568  addi r3, r9, 0x6568
	ctx.r[3].s64 = ctx.r[9].s64 + 25960;
	// 83292474: 4BA17AAD  bl 0x82ca9f20
	ctx.lr = 0x83292478;
	sub_82CA9F20(ctx, base);
	// 83292478: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8329247C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83292480: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83292484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83292488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83292488 size=64
    let mut pc: u32 = 0x83292488;
    'dispatch: loop {
        match pc {
            0x83292488 => {
    //   block [0x83292488..0x832924C8)
	// 83292488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329248C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83292490: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83292494: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83292498: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8329249C: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 832924A0: 386A5D7C  addi r3, r10, 0x5d7c
	ctx.r[3].s64 = ctx.r[10].s64 + 23932;
	// 832924A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832924A8: 4AF9AA29  bl 0x8222ced0
	ctx.lr = 0x832924AC;
	sub_8222CED0(ctx, base);
	// 832924AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832924B0: 38696578  addi r3, r9, 0x6578
	ctx.r[3].s64 = ctx.r[9].s64 + 25976;
	// 832924B4: 4BA17A6D  bl 0x82ca9f20
	ctx.lr = 0x832924B8;
	sub_82CA9F20(ctx, base);
	// 832924B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832924BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832924C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832924C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832924C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832924C8 size=64
    let mut pc: u32 = 0x832924C8;
    'dispatch: loop {
        match pc {
            0x832924C8 => {
    //   block [0x832924C8..0x83292508)
	// 832924C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832924CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832924D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832924D4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832924D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832924DC: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 832924E0: 386A5D80  addi r3, r10, 0x5d80
	ctx.r[3].s64 = ctx.r[10].s64 + 23936;
	// 832924E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832924E8: 4AF9A9E9  bl 0x8222ced0
	ctx.lr = 0x832924EC;
	sub_8222CED0(ctx, base);
	// 832924EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832924F0: 38696588  addi r3, r9, 0x6588
	ctx.r[3].s64 = ctx.r[9].s64 + 25992;
	// 832924F4: 4BA17A2D  bl 0x82ca9f20
	ctx.lr = 0x832924F8;
	sub_82CA9F20(ctx, base);
	// 832924F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832924FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83292500: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83292504: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83292508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83292508 size=64
    let mut pc: u32 = 0x83292508;
    'dispatch: loop {
        match pc {
            0x83292508 => {
    //   block [0x83292508..0x83292548)
	// 83292508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329250C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83292510: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83292514: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83292518: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8329251C: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83292520: 386A5D84  addi r3, r10, 0x5d84
	ctx.r[3].s64 = ctx.r[10].s64 + 23940;
	// 83292524: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83292528: 4AF9A9A9  bl 0x8222ced0
	ctx.lr = 0x8329252C;
	sub_8222CED0(ctx, base);
	// 8329252C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83292530: 38696598  addi r3, r9, 0x6598
	ctx.r[3].s64 = ctx.r[9].s64 + 26008;
	// 83292534: 4BA179ED  bl 0x82ca9f20
	ctx.lr = 0x83292538;
	sub_82CA9F20(ctx, base);
	// 83292538: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8329253C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83292540: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83292544: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83292548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83292548 size=64
    let mut pc: u32 = 0x83292548;
    'dispatch: loop {
        match pc {
            0x83292548 => {
    //   block [0x83292548..0x83292588)
	// 83292548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329254C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83292550: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83292554: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83292558: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8329255C: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83292560: 386A5D88  addi r3, r10, 0x5d88
	ctx.r[3].s64 = ctx.r[10].s64 + 23944;
	// 83292564: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83292568: 4AF9A969  bl 0x8222ced0
	ctx.lr = 0x8329256C;
	sub_8222CED0(ctx, base);
	// 8329256C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83292570: 386965A8  addi r3, r9, 0x65a8
	ctx.r[3].s64 = ctx.r[9].s64 + 26024;
	// 83292574: 4BA179AD  bl 0x82ca9f20
	ctx.lr = 0x83292578;
	sub_82CA9F20(ctx, base);
	// 83292578: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8329257C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83292580: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83292584: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


