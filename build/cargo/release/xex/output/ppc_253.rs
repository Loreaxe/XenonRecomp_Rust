pub fn sub_8327B3A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B3A0 size=56
    let mut pc: u32 = 0x8327B3A0;
    'dispatch: loop {
        match pc {
            0x8327B3A0 => {
    //   block [0x8327B3A0..0x8327B3D8)
	// 8327B3A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B3A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B3A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B3AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327B3B0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327B3B4: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 8327B3B8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327B3BC: 4AF7899D  bl 0x821f3d58
	ctx.lr = 0x8327B3C0;
	sub_821F3D58(ctx, base);
	// 8327B3C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B3C4: 906AD984  stw r3, -0x267c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9852 as u32), ctx.r[3].u32 ) };
	// 8327B3C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B3CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B3D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B3D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B3D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B3D8 size=64
    let mut pc: u32 = 0x8327B3D8;
    'dispatch: loop {
        match pc {
            0x8327B3D8 => {
    //   block [0x8327B3D8..0x8327B418)
	// 8327B3D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B3DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B3E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B3E4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327B3E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B3EC: 388BA6C4  addi r4, r11, -0x593c
	ctx.r[4].s64 = ctx.r[11].s64 + -22844;
	// 8327B3F0: 386AD988  addi r3, r10, -0x2678
	ctx.r[3].s64 = ctx.r[10].s64 + -9848;
	// 8327B3F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327B3F8: 4AFB1AD9  bl 0x8222ced0
	ctx.lr = 0x8327B3FC;
	sub_8222CED0(ctx, base);
	// 8327B3FC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327B400: 38690248  addi r3, r9, 0x248
	ctx.r[3].s64 = ctx.r[9].s64 + 584;
	// 8327B404: 4BA2EB1D  bl 0x82ca9f20
	ctx.lr = 0x8327B408;
	sub_82CA9F20(ctx, base);
	// 8327B408: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B40C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B410: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B414: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B418 size=64
    let mut pc: u32 = 0x8327B418;
    'dispatch: loop {
        match pc {
            0x8327B418 => {
    //   block [0x8327B418..0x8327B458)
	// 8327B418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B41C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B420: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B424: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327B428: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B42C: 388BA694  addi r4, r11, -0x596c
	ctx.r[4].s64 = ctx.r[11].s64 + -22892;
	// 8327B430: 386AD98C  addi r3, r10, -0x2674
	ctx.r[3].s64 = ctx.r[10].s64 + -9844;
	// 8327B434: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327B438: 4AFB1A99  bl 0x8222ced0
	ctx.lr = 0x8327B43C;
	sub_8222CED0(ctx, base);
	// 8327B43C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327B440: 38690258  addi r3, r9, 0x258
	ctx.r[3].s64 = ctx.r[9].s64 + 600;
	// 8327B444: 4BA2EADD  bl 0x82ca9f20
	ctx.lr = 0x8327B448;
	sub_82CA9F20(ctx, base);
	// 8327B448: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B44C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B450: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B454: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B458 size=64
    let mut pc: u32 = 0x8327B458;
    'dispatch: loop {
        match pc {
            0x8327B458 => {
    //   block [0x8327B458..0x8327B498)
	// 8327B458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B45C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B460: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B464: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327B468: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B46C: 388BE0E0  addi r4, r11, -0x1f20
	ctx.r[4].s64 = ctx.r[11].s64 + -7968;
	// 8327B470: 386AD990  addi r3, r10, -0x2670
	ctx.r[3].s64 = ctx.r[10].s64 + -9840;
	// 8327B474: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327B478: 4AFB1A59  bl 0x8222ced0
	ctx.lr = 0x8327B47C;
	sub_8222CED0(ctx, base);
	// 8327B47C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327B480: 38690268  addi r3, r9, 0x268
	ctx.r[3].s64 = ctx.r[9].s64 + 616;
	// 8327B484: 4BA2EA9D  bl 0x82ca9f20
	ctx.lr = 0x8327B488;
	sub_82CA9F20(ctx, base);
	// 8327B488: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B48C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B490: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B494: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B498 size=64
    let mut pc: u32 = 0x8327B498;
    'dispatch: loop {
        match pc {
            0x8327B498 => {
    //   block [0x8327B498..0x8327B4D8)
	// 8327B498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B49C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B4A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B4A4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327B4A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B4AC: 388BA8BC  addi r4, r11, -0x5744
	ctx.r[4].s64 = ctx.r[11].s64 + -22340;
	// 8327B4B0: 386AD994  addi r3, r10, -0x266c
	ctx.r[3].s64 = ctx.r[10].s64 + -9836;
	// 8327B4B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327B4B8: 4AFB1A19  bl 0x8222ced0
	ctx.lr = 0x8327B4BC;
	sub_8222CED0(ctx, base);
	// 8327B4BC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327B4C0: 38690278  addi r3, r9, 0x278
	ctx.r[3].s64 = ctx.r[9].s64 + 632;
	// 8327B4C4: 4BA2EA5D  bl 0x82ca9f20
	ctx.lr = 0x8327B4C8;
	sub_82CA9F20(ctx, base);
	// 8327B4C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B4CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B4D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B4D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B4D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B4D8 size=64
    let mut pc: u32 = 0x8327B4D8;
    'dispatch: loop {
        match pc {
            0x8327B4D8 => {
    //   block [0x8327B4D8..0x8327B518)
	// 8327B4D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B4DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B4E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B4E4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327B4E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B4EC: 388BA8EC  addi r4, r11, -0x5714
	ctx.r[4].s64 = ctx.r[11].s64 + -22292;
	// 8327B4F0: 386AD998  addi r3, r10, -0x2668
	ctx.r[3].s64 = ctx.r[10].s64 + -9832;
	// 8327B4F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327B4F8: 4AFB19D9  bl 0x8222ced0
	ctx.lr = 0x8327B4FC;
	sub_8222CED0(ctx, base);
	// 8327B4FC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327B500: 38690288  addi r3, r9, 0x288
	ctx.r[3].s64 = ctx.r[9].s64 + 648;
	// 8327B504: 4BA2EA1D  bl 0x82ca9f20
	ctx.lr = 0x8327B508;
	sub_82CA9F20(ctx, base);
	// 8327B508: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B50C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B510: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B514: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B518 size=64
    let mut pc: u32 = 0x8327B518;
    'dispatch: loop {
        match pc {
            0x8327B518 => {
    //   block [0x8327B518..0x8327B558)
	// 8327B518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B51C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B520: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B524: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327B528: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B52C: 388BE10C  addi r4, r11, -0x1ef4
	ctx.r[4].s64 = ctx.r[11].s64 + -7924;
	// 8327B530: 386AD99C  addi r3, r10, -0x2664
	ctx.r[3].s64 = ctx.r[10].s64 + -9828;
	// 8327B534: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327B538: 4AFB1999  bl 0x8222ced0
	ctx.lr = 0x8327B53C;
	sub_8222CED0(ctx, base);
	// 8327B53C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327B540: 38690298  addi r3, r9, 0x298
	ctx.r[3].s64 = ctx.r[9].s64 + 664;
	// 8327B544: 4BA2E9DD  bl 0x82ca9f20
	ctx.lr = 0x8327B548;
	sub_82CA9F20(ctx, base);
	// 8327B548: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B54C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B550: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B554: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B558 size=64
    let mut pc: u32 = 0x8327B558;
    'dispatch: loop {
        match pc {
            0x8327B558 => {
    //   block [0x8327B558..0x8327B598)
	// 8327B558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B55C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B560: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B564: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8327B568: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B56C: 388BFD7C  addi r4, r11, -0x284
	ctx.r[4].s64 = ctx.r[11].s64 + -644;
	// 8327B570: 386AD9A0  addi r3, r10, -0x2660
	ctx.r[3].s64 = ctx.r[10].s64 + -9824;
	// 8327B574: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327B578: 4AFB1959  bl 0x8222ced0
	ctx.lr = 0x8327B57C;
	sub_8222CED0(ctx, base);
	// 8327B57C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327B580: 386902A8  addi r3, r9, 0x2a8
	ctx.r[3].s64 = ctx.r[9].s64 + 680;
	// 8327B584: 4BA2E99D  bl 0x82ca9f20
	ctx.lr = 0x8327B588;
	sub_82CA9F20(ctx, base);
	// 8327B588: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B58C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B590: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B594: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B598 size=64
    let mut pc: u32 = 0x8327B598;
    'dispatch: loop {
        match pc {
            0x8327B598 => {
    //   block [0x8327B598..0x8327B5D8)
	// 8327B598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B59C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B5A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B5A4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8327B5A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B5AC: 388BFD38  addi r4, r11, -0x2c8
	ctx.r[4].s64 = ctx.r[11].s64 + -712;
	// 8327B5B0: 386AD9A4  addi r3, r10, -0x265c
	ctx.r[3].s64 = ctx.r[10].s64 + -9820;
	// 8327B5B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327B5B8: 4AFB1919  bl 0x8222ced0
	ctx.lr = 0x8327B5BC;
	sub_8222CED0(ctx, base);
	// 8327B5BC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327B5C0: 386902B8  addi r3, r9, 0x2b8
	ctx.r[3].s64 = ctx.r[9].s64 + 696;
	// 8327B5C4: 4BA2E95D  bl 0x82ca9f20
	ctx.lr = 0x8327B5C8;
	sub_82CA9F20(ctx, base);
	// 8327B5C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B5CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B5D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B5D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B5D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B5D8 size=64
    let mut pc: u32 = 0x8327B5D8;
    'dispatch: loop {
        match pc {
            0x8327B5D8 => {
    //   block [0x8327B5D8..0x8327B618)
	// 8327B5D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B5DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B5E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B5E4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327B5E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B5EC: 388BE144  addi r4, r11, -0x1ebc
	ctx.r[4].s64 = ctx.r[11].s64 + -7868;
	// 8327B5F0: 386AD9A8  addi r3, r10, -0x2658
	ctx.r[3].s64 = ctx.r[10].s64 + -9816;
	// 8327B5F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327B5F8: 4AFB18D9  bl 0x8222ced0
	ctx.lr = 0x8327B5FC;
	sub_8222CED0(ctx, base);
	// 8327B5FC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327B600: 386902C8  addi r3, r9, 0x2c8
	ctx.r[3].s64 = ctx.r[9].s64 + 712;
	// 8327B604: 4BA2E91D  bl 0x82ca9f20
	ctx.lr = 0x8327B608;
	sub_82CA9F20(ctx, base);
	// 8327B608: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B60C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B610: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B614: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B618 size=64
    let mut pc: u32 = 0x8327B618;
    'dispatch: loop {
        match pc {
            0x8327B618 => {
    //   block [0x8327B618..0x8327B658)
	// 8327B618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B61C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B620: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B624: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327B628: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B62C: 388BE180  addi r4, r11, -0x1e80
	ctx.r[4].s64 = ctx.r[11].s64 + -7808;
	// 8327B630: 386AD9AC  addi r3, r10, -0x2654
	ctx.r[3].s64 = ctx.r[10].s64 + -9812;
	// 8327B634: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327B638: 4AFB1899  bl 0x8222ced0
	ctx.lr = 0x8327B63C;
	sub_8222CED0(ctx, base);
	// 8327B63C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327B640: 386902D8  addi r3, r9, 0x2d8
	ctx.r[3].s64 = ctx.r[9].s64 + 728;
	// 8327B644: 4BA2E8DD  bl 0x82ca9f20
	ctx.lr = 0x8327B648;
	sub_82CA9F20(ctx, base);
	// 8327B648: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B64C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B650: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B654: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B658 size=64
    let mut pc: u32 = 0x8327B658;
    'dispatch: loop {
        match pc {
            0x8327B658 => {
    //   block [0x8327B658..0x8327B698)
	// 8327B658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B65C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B660: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B664: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327B668: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B66C: 388BA878  addi r4, r11, -0x5788
	ctx.r[4].s64 = ctx.r[11].s64 + -22408;
	// 8327B670: 386AD9B0  addi r3, r10, -0x2650
	ctx.r[3].s64 = ctx.r[10].s64 + -9808;
	// 8327B674: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327B678: 4AFB1859  bl 0x8222ced0
	ctx.lr = 0x8327B67C;
	sub_8222CED0(ctx, base);
	// 8327B67C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327B680: 386902E8  addi r3, r9, 0x2e8
	ctx.r[3].s64 = ctx.r[9].s64 + 744;
	// 8327B684: 4BA2E89D  bl 0x82ca9f20
	ctx.lr = 0x8327B688;
	sub_82CA9F20(ctx, base);
	// 8327B688: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B68C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B690: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B694: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B698 size=64
    let mut pc: u32 = 0x8327B698;
    'dispatch: loop {
        match pc {
            0x8327B698 => {
    //   block [0x8327B698..0x8327B6D8)
	// 8327B698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B69C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B6A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B6A4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327B6A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B6AC: 388BA924  addi r4, r11, -0x56dc
	ctx.r[4].s64 = ctx.r[11].s64 + -22236;
	// 8327B6B0: 386AD9B4  addi r3, r10, -0x264c
	ctx.r[3].s64 = ctx.r[10].s64 + -9804;
	// 8327B6B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327B6B8: 4AFB1819  bl 0x8222ced0
	ctx.lr = 0x8327B6BC;
	sub_8222CED0(ctx, base);
	// 8327B6BC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327B6C0: 386902F8  addi r3, r9, 0x2f8
	ctx.r[3].s64 = ctx.r[9].s64 + 760;
	// 8327B6C4: 4BA2E85D  bl 0x82ca9f20
	ctx.lr = 0x8327B6C8;
	sub_82CA9F20(ctx, base);
	// 8327B6C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B6CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B6D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B6D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B6D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B6D8 size=64
    let mut pc: u32 = 0x8327B6D8;
    'dispatch: loop {
        match pc {
            0x8327B6D8 => {
    //   block [0x8327B6D8..0x8327B718)
	// 8327B6D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B6DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B6E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B6E4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327B6E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B6EC: 388BE268  addi r4, r11, -0x1d98
	ctx.r[4].s64 = ctx.r[11].s64 + -7576;
	// 8327B6F0: 386AD9B8  addi r3, r10, -0x2648
	ctx.r[3].s64 = ctx.r[10].s64 + -9800;
	// 8327B6F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327B6F8: 4AFB17D9  bl 0x8222ced0
	ctx.lr = 0x8327B6FC;
	sub_8222CED0(ctx, base);
	// 8327B6FC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327B700: 38690308  addi r3, r9, 0x308
	ctx.r[3].s64 = ctx.r[9].s64 + 776;
	// 8327B704: 4BA2E81D  bl 0x82ca9f20
	ctx.lr = 0x8327B708;
	sub_82CA9F20(ctx, base);
	// 8327B708: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B70C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B710: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B714: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B718 size=64
    let mut pc: u32 = 0x8327B718;
    'dispatch: loop {
        match pc {
            0x8327B718 => {
    //   block [0x8327B718..0x8327B758)
	// 8327B718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B71C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B720: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B724: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327B728: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B72C: 388BE29C  addi r4, r11, -0x1d64
	ctx.r[4].s64 = ctx.r[11].s64 + -7524;
	// 8327B730: 386AD9BC  addi r3, r10, -0x2644
	ctx.r[3].s64 = ctx.r[10].s64 + -9796;
	// 8327B734: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327B738: 4AFB1799  bl 0x8222ced0
	ctx.lr = 0x8327B73C;
	sub_8222CED0(ctx, base);
	// 8327B73C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327B740: 38690318  addi r3, r9, 0x318
	ctx.r[3].s64 = ctx.r[9].s64 + 792;
	// 8327B744: 4BA2E7DD  bl 0x82ca9f20
	ctx.lr = 0x8327B748;
	sub_82CA9F20(ctx, base);
	// 8327B748: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B74C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B750: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B754: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B758 size=64
    let mut pc: u32 = 0x8327B758;
    'dispatch: loop {
        match pc {
            0x8327B758 => {
    //   block [0x8327B758..0x8327B798)
	// 8327B758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B75C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B760: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B764: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327B768: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B76C: 388BE2D8  addi r4, r11, -0x1d28
	ctx.r[4].s64 = ctx.r[11].s64 + -7464;
	// 8327B770: 386AD9C0  addi r3, r10, -0x2640
	ctx.r[3].s64 = ctx.r[10].s64 + -9792;
	// 8327B774: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327B778: 4AFB1759  bl 0x8222ced0
	ctx.lr = 0x8327B77C;
	sub_8222CED0(ctx, base);
	// 8327B77C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327B780: 38690328  addi r3, r9, 0x328
	ctx.r[3].s64 = ctx.r[9].s64 + 808;
	// 8327B784: 4BA2E79D  bl 0x82ca9f20
	ctx.lr = 0x8327B788;
	sub_82CA9F20(ctx, base);
	// 8327B788: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B78C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B790: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B794: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B798 size=64
    let mut pc: u32 = 0x8327B798;
    'dispatch: loop {
        match pc {
            0x8327B798 => {
    //   block [0x8327B798..0x8327B7D8)
	// 8327B798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B79C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B7A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B7A4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8327B7A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B7AC: 388BFAF0  addi r4, r11, -0x510
	ctx.r[4].s64 = ctx.r[11].s64 + -1296;
	// 8327B7B0: 386AD9C4  addi r3, r10, -0x263c
	ctx.r[3].s64 = ctx.r[10].s64 + -9788;
	// 8327B7B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327B7B8: 4AFB1719  bl 0x8222ced0
	ctx.lr = 0x8327B7BC;
	sub_8222CED0(ctx, base);
	// 8327B7BC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327B7C0: 38690338  addi r3, r9, 0x338
	ctx.r[3].s64 = ctx.r[9].s64 + 824;
	// 8327B7C4: 4BA2E75D  bl 0x82ca9f20
	ctx.lr = 0x8327B7C8;
	sub_82CA9F20(ctx, base);
	// 8327B7C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B7CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B7D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B7D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B7D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B7D8 size=64
    let mut pc: u32 = 0x8327B7D8;
    'dispatch: loop {
        match pc {
            0x8327B7D8 => {
    //   block [0x8327B7D8..0x8327B818)
	// 8327B7D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B7DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B7E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B7E4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327B7E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B7EC: 388BE144  addi r4, r11, -0x1ebc
	ctx.r[4].s64 = ctx.r[11].s64 + -7868;
	// 8327B7F0: 386AD9C8  addi r3, r10, -0x2638
	ctx.r[3].s64 = ctx.r[10].s64 + -9784;
	// 8327B7F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327B7F8: 4AFB16D9  bl 0x8222ced0
	ctx.lr = 0x8327B7FC;
	sub_8222CED0(ctx, base);
	// 8327B7FC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327B800: 38690348  addi r3, r9, 0x348
	ctx.r[3].s64 = ctx.r[9].s64 + 840;
	// 8327B804: 4BA2E71D  bl 0x82ca9f20
	ctx.lr = 0x8327B808;
	sub_82CA9F20(ctx, base);
	// 8327B808: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B80C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B810: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B814: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B818 size=64
    let mut pc: u32 = 0x8327B818;
    'dispatch: loop {
        match pc {
            0x8327B818 => {
    //   block [0x8327B818..0x8327B858)
	// 8327B818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B81C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B820: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B824: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327B828: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B82C: 388BE180  addi r4, r11, -0x1e80
	ctx.r[4].s64 = ctx.r[11].s64 + -7808;
	// 8327B830: 386AD9CC  addi r3, r10, -0x2634
	ctx.r[3].s64 = ctx.r[10].s64 + -9780;
	// 8327B834: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327B838: 4AFB1699  bl 0x8222ced0
	ctx.lr = 0x8327B83C;
	sub_8222CED0(ctx, base);
	// 8327B83C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327B840: 38690358  addi r3, r9, 0x358
	ctx.r[3].s64 = ctx.r[9].s64 + 856;
	// 8327B844: 4BA2E6DD  bl 0x82ca9f20
	ctx.lr = 0x8327B848;
	sub_82CA9F20(ctx, base);
	// 8327B848: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B84C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B850: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B854: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B858 size=64
    let mut pc: u32 = 0x8327B858;
    'dispatch: loop {
        match pc {
            0x8327B858 => {
    //   block [0x8327B858..0x8327B898)
	// 8327B858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B85C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B860: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B864: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327B868: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B86C: 388BE310  addi r4, r11, -0x1cf0
	ctx.r[4].s64 = ctx.r[11].s64 + -7408;
	// 8327B870: 386AD9D0  addi r3, r10, -0x2630
	ctx.r[3].s64 = ctx.r[10].s64 + -9776;
	// 8327B874: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327B878: 4AFB1659  bl 0x8222ced0
	ctx.lr = 0x8327B87C;
	sub_8222CED0(ctx, base);
	// 8327B87C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327B880: 38690368  addi r3, r9, 0x368
	ctx.r[3].s64 = ctx.r[9].s64 + 872;
	// 8327B884: 4BA2E69D  bl 0x82ca9f20
	ctx.lr = 0x8327B888;
	sub_82CA9F20(ctx, base);
	// 8327B888: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B88C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B890: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B894: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B898 size=64
    let mut pc: u32 = 0x8327B898;
    'dispatch: loop {
        match pc {
            0x8327B898 => {
    //   block [0x8327B898..0x8327B8D8)
	// 8327B898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B89C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B8A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B8A4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327B8A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B8AC: 388BE348  addi r4, r11, -0x1cb8
	ctx.r[4].s64 = ctx.r[11].s64 + -7352;
	// 8327B8B0: 386AD9D4  addi r3, r10, -0x262c
	ctx.r[3].s64 = ctx.r[10].s64 + -9772;
	// 8327B8B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327B8B8: 4AFB1619  bl 0x8222ced0
	ctx.lr = 0x8327B8BC;
	sub_8222CED0(ctx, base);
	// 8327B8BC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327B8C0: 38690378  addi r3, r9, 0x378
	ctx.r[3].s64 = ctx.r[9].s64 + 888;
	// 8327B8C4: 4BA2E65D  bl 0x82ca9f20
	ctx.lr = 0x8327B8C8;
	sub_82CA9F20(ctx, base);
	// 8327B8C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B8CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B8D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B8D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B8D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B8D8 size=64
    let mut pc: u32 = 0x8327B8D8;
    'dispatch: loop {
        match pc {
            0x8327B8D8 => {
    //   block [0x8327B8D8..0x8327B918)
	// 8327B8D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B8DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B8E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B8E4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327B8E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B8EC: 388BA8BC  addi r4, r11, -0x5744
	ctx.r[4].s64 = ctx.r[11].s64 + -22340;
	// 8327B8F0: 386AD9D8  addi r3, r10, -0x2628
	ctx.r[3].s64 = ctx.r[10].s64 + -9768;
	// 8327B8F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327B8F8: 4AFB15D9  bl 0x8222ced0
	ctx.lr = 0x8327B8FC;
	sub_8222CED0(ctx, base);
	// 8327B8FC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327B900: 38690388  addi r3, r9, 0x388
	ctx.r[3].s64 = ctx.r[9].s64 + 904;
	// 8327B904: 4BA2E61D  bl 0x82ca9f20
	ctx.lr = 0x8327B908;
	sub_82CA9F20(ctx, base);
	// 8327B908: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B90C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B910: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B914: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B918 size=64
    let mut pc: u32 = 0x8327B918;
    'dispatch: loop {
        match pc {
            0x8327B918 => {
    //   block [0x8327B918..0x8327B958)
	// 8327B918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B91C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B920: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B924: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327B928: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B92C: 388BA8EC  addi r4, r11, -0x5714
	ctx.r[4].s64 = ctx.r[11].s64 + -22292;
	// 8327B930: 386AD9DC  addi r3, r10, -0x2624
	ctx.r[3].s64 = ctx.r[10].s64 + -9764;
	// 8327B934: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327B938: 4AFB1599  bl 0x8222ced0
	ctx.lr = 0x8327B93C;
	sub_8222CED0(ctx, base);
	// 8327B93C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327B940: 38690398  addi r3, r9, 0x398
	ctx.r[3].s64 = ctx.r[9].s64 + 920;
	// 8327B944: 4BA2E5DD  bl 0x82ca9f20
	ctx.lr = 0x8327B948;
	sub_82CA9F20(ctx, base);
	// 8327B948: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B94C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B950: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B954: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B958 size=64
    let mut pc: u32 = 0x8327B958;
    'dispatch: loop {
        match pc {
            0x8327B958 => {
    //   block [0x8327B958..0x8327B998)
	// 8327B958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B95C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B960: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B964: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327B968: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B96C: 388BE38C  addi r4, r11, -0x1c74
	ctx.r[4].s64 = ctx.r[11].s64 + -7284;
	// 8327B970: 386AD9E0  addi r3, r10, -0x2620
	ctx.r[3].s64 = ctx.r[10].s64 + -9760;
	// 8327B974: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327B978: 4AFB1559  bl 0x8222ced0
	ctx.lr = 0x8327B97C;
	sub_8222CED0(ctx, base);
	// 8327B97C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327B980: 386903A8  addi r3, r9, 0x3a8
	ctx.r[3].s64 = ctx.r[9].s64 + 936;
	// 8327B984: 4BA2E59D  bl 0x82ca9f20
	ctx.lr = 0x8327B988;
	sub_82CA9F20(ctx, base);
	// 8327B988: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B98C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B990: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B994: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B998 size=56
    let mut pc: u32 = 0x8327B998;
    'dispatch: loop {
        match pc {
            0x8327B998 => {
    //   block [0x8327B998..0x8327B9D0)
	// 8327B998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B99C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B9A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B9A4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327B9A8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327B9AC: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 8327B9B0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327B9B4: 4AF783A5  bl 0x821f3d58
	ctx.lr = 0x8327B9B8;
	sub_821F3D58(ctx, base);
	// 8327B9B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B9BC: 906AD9E4  stw r3, -0x261c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9756 as u32), ctx.r[3].u32 ) };
	// 8327B9C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B9C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B9C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B9CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B9D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B9D0 size=56
    let mut pc: u32 = 0x8327B9D0;
    'dispatch: loop {
        match pc {
            0x8327B9D0 => {
    //   block [0x8327B9D0..0x8327BA08)
	// 8327B9D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B9D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B9D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B9DC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327B9E0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327B9E4: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 8327B9E8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327B9EC: 4AF7836D  bl 0x821f3d58
	ctx.lr = 0x8327B9F0;
	sub_821F3D58(ctx, base);
	// 8327B9F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B9F4: 906AD9E8  stw r3, -0x2618(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9752 as u32), ctx.r[3].u32 ) };
	// 8327B9F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B9FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327BA00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327BA04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327BA08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327BA08 size=56
    let mut pc: u32 = 0x8327BA08;
    'dispatch: loop {
        match pc {
            0x8327BA08 => {
    //   block [0x8327BA08..0x8327BA40)
	// 8327BA08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327BA0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327BA10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327BA14: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327BA18: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327BA1C: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 8327BA20: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327BA24: 4AF78335  bl 0x821f3d58
	ctx.lr = 0x8327BA28;
	sub_821F3D58(ctx, base);
	// 8327BA28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327BA2C: 906AD9EC  stw r3, -0x2614(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9748 as u32), ctx.r[3].u32 ) };
	// 8327BA30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327BA34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327BA38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327BA3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327BA40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327BA40 size=56
    let mut pc: u32 = 0x8327BA40;
    'dispatch: loop {
        match pc {
            0x8327BA40 => {
    //   block [0x8327BA40..0x8327BA78)
	// 8327BA40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327BA44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327BA48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327BA4C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327BA50: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327BA54: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 8327BA58: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327BA5C: 4AF782FD  bl 0x821f3d58
	ctx.lr = 0x8327BA60;
	sub_821F3D58(ctx, base);
	// 8327BA60: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327BA64: 906AD9F0  stw r3, -0x2610(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9744 as u32), ctx.r[3].u32 ) };
	// 8327BA68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327BA6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327BA70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327BA74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327BA78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327BA78 size=56
    let mut pc: u32 = 0x8327BA78;
    'dispatch: loop {
        match pc {
            0x8327BA78 => {
    //   block [0x8327BA78..0x8327BAB0)
	// 8327BA78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327BA7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327BA80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327BA84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327BA88: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327BA8C: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 8327BA90: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327BA94: 4AF782C5  bl 0x821f3d58
	ctx.lr = 0x8327BA98;
	sub_821F3D58(ctx, base);
	// 8327BA98: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327BA9C: 906AD9F4  stw r3, -0x260c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9740 as u32), ctx.r[3].u32 ) };
	// 8327BAA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327BAA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327BAA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327BAAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327BAB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327BAB0 size=56
    let mut pc: u32 = 0x8327BAB0;
    'dispatch: loop {
        match pc {
            0x8327BAB0 => {
    //   block [0x8327BAB0..0x8327BAE8)
	// 8327BAB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327BAB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327BAB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327BABC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327BAC0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327BAC4: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 8327BAC8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327BACC: 4AF7828D  bl 0x821f3d58
	ctx.lr = 0x8327BAD0;
	sub_821F3D58(ctx, base);
	// 8327BAD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327BAD4: 906AD9F8  stw r3, -0x2608(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9736 as u32), ctx.r[3].u32 ) };
	// 8327BAD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327BADC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327BAE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327BAE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327BAE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327BAE8 size=56
    let mut pc: u32 = 0x8327BAE8;
    'dispatch: loop {
        match pc {
            0x8327BAE8 => {
    //   block [0x8327BAE8..0x8327BB20)
	// 8327BAE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327BAEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327BAF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327BAF4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327BAF8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327BAFC: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 8327BB00: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327BB04: 4AF78255  bl 0x821f3d58
	ctx.lr = 0x8327BB08;
	sub_821F3D58(ctx, base);
	// 8327BB08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327BB0C: 906AD9FC  stw r3, -0x2604(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9732 as u32), ctx.r[3].u32 ) };
	// 8327BB10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327BB14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327BB18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327BB1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327BB20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327BB20 size=56
    let mut pc: u32 = 0x8327BB20;
    'dispatch: loop {
        match pc {
            0x8327BB20 => {
    //   block [0x8327BB20..0x8327BB58)
	// 8327BB20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327BB24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327BB28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327BB2C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327BB30: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327BB34: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 8327BB38: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327BB3C: 4AF7821D  bl 0x821f3d58
	ctx.lr = 0x8327BB40;
	sub_821F3D58(ctx, base);
	// 8327BB40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327BB44: 906ADA00  stw r3, -0x2600(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9728 as u32), ctx.r[3].u32 ) };
	// 8327BB48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327BB4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327BB50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327BB54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327BB58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327BB58 size=56
    let mut pc: u32 = 0x8327BB58;
    'dispatch: loop {
        match pc {
            0x8327BB58 => {
    //   block [0x8327BB58..0x8327BB90)
	// 8327BB58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327BB5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327BB60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327BB64: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327BB68: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327BB6C: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 8327BB70: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327BB74: 4AF781E5  bl 0x821f3d58
	ctx.lr = 0x8327BB78;
	sub_821F3D58(ctx, base);
	// 8327BB78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327BB7C: 906ADA04  stw r3, -0x25fc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9724 as u32), ctx.r[3].u32 ) };
	// 8327BB80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327BB84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327BB88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327BB8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327BB90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327BB90 size=56
    let mut pc: u32 = 0x8327BB90;
    'dispatch: loop {
        match pc {
            0x8327BB90 => {
    //   block [0x8327BB90..0x8327BBC8)
	// 8327BB90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327BB94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327BB98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327BB9C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327BBA0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327BBA4: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 8327BBA8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327BBAC: 4AF781AD  bl 0x821f3d58
	ctx.lr = 0x8327BBB0;
	sub_821F3D58(ctx, base);
	// 8327BBB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327BBB4: 906ADA08  stw r3, -0x25f8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9720 as u32), ctx.r[3].u32 ) };
	// 8327BBB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327BBBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327BBC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327BBC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327BBC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327BBC8 size=56
    let mut pc: u32 = 0x8327BBC8;
    'dispatch: loop {
        match pc {
            0x8327BBC8 => {
    //   block [0x8327BBC8..0x8327BC00)
	// 8327BBC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327BBCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327BBD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327BBD4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327BBD8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327BBDC: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 8327BBE0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327BBE4: 4AF78175  bl 0x821f3d58
	ctx.lr = 0x8327BBE8;
	sub_821F3D58(ctx, base);
	// 8327BBE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327BBEC: 906ADA0C  stw r3, -0x25f4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9716 as u32), ctx.r[3].u32 ) };
	// 8327BBF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327BBF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327BBF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327BBFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327BC00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327BC00 size=56
    let mut pc: u32 = 0x8327BC00;
    'dispatch: loop {
        match pc {
            0x8327BC00 => {
    //   block [0x8327BC00..0x8327BC38)
	// 8327BC00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327BC04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327BC08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327BC0C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327BC10: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327BC14: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 8327BC18: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327BC1C: 4AF7813D  bl 0x821f3d58
	ctx.lr = 0x8327BC20;
	sub_821F3D58(ctx, base);
	// 8327BC20: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327BC24: 906ADA10  stw r3, -0x25f0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9712 as u32), ctx.r[3].u32 ) };
	// 8327BC28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327BC2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327BC30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327BC34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327BC38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327BC38 size=56
    let mut pc: u32 = 0x8327BC38;
    'dispatch: loop {
        match pc {
            0x8327BC38 => {
    //   block [0x8327BC38..0x8327BC70)
	// 8327BC38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327BC3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327BC40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327BC44: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327BC48: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327BC4C: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 8327BC50: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327BC54: 4AF78105  bl 0x821f3d58
	ctx.lr = 0x8327BC58;
	sub_821F3D58(ctx, base);
	// 8327BC58: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327BC5C: 906ADA14  stw r3, -0x25ec(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9708 as u32), ctx.r[3].u32 ) };
	// 8327BC60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327BC64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327BC68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327BC6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327BC70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327BC70 size=56
    let mut pc: u32 = 0x8327BC70;
    'dispatch: loop {
        match pc {
            0x8327BC70 => {
    //   block [0x8327BC70..0x8327BCA8)
	// 8327BC70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327BC74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327BC78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327BC7C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327BC80: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327BC84: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 8327BC88: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327BC8C: 4AF780CD  bl 0x821f3d58
	ctx.lr = 0x8327BC90;
	sub_821F3D58(ctx, base);
	// 8327BC90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327BC94: 906ADA18  stw r3, -0x25e8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9704 as u32), ctx.r[3].u32 ) };
	// 8327BC98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327BC9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327BCA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327BCA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327BCA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327BCA8 size=56
    let mut pc: u32 = 0x8327BCA8;
    'dispatch: loop {
        match pc {
            0x8327BCA8 => {
    //   block [0x8327BCA8..0x8327BCE0)
	// 8327BCA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327BCAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327BCB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327BCB4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327BCB8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327BCBC: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 8327BCC0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327BCC4: 4AF78095  bl 0x821f3d58
	ctx.lr = 0x8327BCC8;
	sub_821F3D58(ctx, base);
	// 8327BCC8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327BCCC: 906ADA1C  stw r3, -0x25e4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9700 as u32), ctx.r[3].u32 ) };
	// 8327BCD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327BCD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327BCD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327BCDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327BCE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327BCE0 size=56
    let mut pc: u32 = 0x8327BCE0;
    'dispatch: loop {
        match pc {
            0x8327BCE0 => {
    //   block [0x8327BCE0..0x8327BD18)
	// 8327BCE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327BCE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327BCE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327BCEC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327BCF0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327BCF4: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 8327BCF8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327BCFC: 4AF7805D  bl 0x821f3d58
	ctx.lr = 0x8327BD00;
	sub_821F3D58(ctx, base);
	// 8327BD00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327BD04: 906ADA20  stw r3, -0x25e0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9696 as u32), ctx.r[3].u32 ) };
	// 8327BD08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327BD0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327BD10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327BD14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327BD18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327BD18 size=56
    let mut pc: u32 = 0x8327BD18;
    'dispatch: loop {
        match pc {
            0x8327BD18 => {
    //   block [0x8327BD18..0x8327BD50)
	// 8327BD18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327BD1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327BD20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327BD24: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327BD28: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327BD2C: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 8327BD30: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327BD34: 4AF78025  bl 0x821f3d58
	ctx.lr = 0x8327BD38;
	sub_821F3D58(ctx, base);
	// 8327BD38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327BD3C: 906ADA24  stw r3, -0x25dc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9692 as u32), ctx.r[3].u32 ) };
	// 8327BD40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327BD44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327BD48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327BD4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327BD50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327BD50 size=56
    let mut pc: u32 = 0x8327BD50;
    'dispatch: loop {
        match pc {
            0x8327BD50 => {
    //   block [0x8327BD50..0x8327BD88)
	// 8327BD50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327BD54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327BD58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327BD5C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327BD60: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327BD64: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 8327BD68: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327BD6C: 4AF77FED  bl 0x821f3d58
	ctx.lr = 0x8327BD70;
	sub_821F3D58(ctx, base);
	// 8327BD70: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327BD74: 906ADA28  stw r3, -0x25d8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9688 as u32), ctx.r[3].u32 ) };
	// 8327BD78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327BD7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327BD80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327BD84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327BD88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327BD88 size=56
    let mut pc: u32 = 0x8327BD88;
    'dispatch: loop {
        match pc {
            0x8327BD88 => {
    //   block [0x8327BD88..0x8327BDC0)
	// 8327BD88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327BD8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327BD90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327BD94: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327BD98: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327BD9C: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 8327BDA0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327BDA4: 4AF77FB5  bl 0x821f3d58
	ctx.lr = 0x8327BDA8;
	sub_821F3D58(ctx, base);
	// 8327BDA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327BDAC: 906ADA2C  stw r3, -0x25d4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9684 as u32), ctx.r[3].u32 ) };
	// 8327BDB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327BDB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327BDB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327BDBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327BDC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327BDC0 size=56
    let mut pc: u32 = 0x8327BDC0;
    'dispatch: loop {
        match pc {
            0x8327BDC0 => {
    //   block [0x8327BDC0..0x8327BDF8)
	// 8327BDC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327BDC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327BDC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327BDCC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327BDD0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327BDD4: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 8327BDD8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327BDDC: 4AF77F7D  bl 0x821f3d58
	ctx.lr = 0x8327BDE0;
	sub_821F3D58(ctx, base);
	// 8327BDE0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327BDE4: 906ADA30  stw r3, -0x25d0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9680 as u32), ctx.r[3].u32 ) };
	// 8327BDE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327BDEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327BDF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327BDF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327BDF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327BDF8 size=56
    let mut pc: u32 = 0x8327BDF8;
    'dispatch: loop {
        match pc {
            0x8327BDF8 => {
    //   block [0x8327BDF8..0x8327BE30)
	// 8327BDF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327BDFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327BE00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327BE04: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327BE08: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327BE0C: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 8327BE10: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327BE14: 4AF77F45  bl 0x821f3d58
	ctx.lr = 0x8327BE18;
	sub_821F3D58(ctx, base);
	// 8327BE18: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327BE1C: 906ADA34  stw r3, -0x25cc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9676 as u32), ctx.r[3].u32 ) };
	// 8327BE20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327BE24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327BE28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327BE2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327BE30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327BE30 size=64
    let mut pc: u32 = 0x8327BE30;
    'dispatch: loop {
        match pc {
            0x8327BE30 => {
    //   block [0x8327BE30..0x8327BE70)
	// 8327BE30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327BE34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327BE38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327BE3C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327BE40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327BE44: 388BA9A4  addi r4, r11, -0x565c
	ctx.r[4].s64 = ctx.r[11].s64 + -22108;
	// 8327BE48: 386ADA38  addi r3, r10, -0x25c8
	ctx.r[3].s64 = ctx.r[10].s64 + -9672;
	// 8327BE4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327BE50: 4AFB1081  bl 0x8222ced0
	ctx.lr = 0x8327BE54;
	sub_8222CED0(ctx, base);
	// 8327BE54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327BE58: 386903B8  addi r3, r9, 0x3b8
	ctx.r[3].s64 = ctx.r[9].s64 + 952;
	// 8327BE5C: 4BA2E0C5  bl 0x82ca9f20
	ctx.lr = 0x8327BE60;
	sub_82CA9F20(ctx, base);
	// 8327BE60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327BE64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327BE68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327BE6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327BE70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327BE70 size=64
    let mut pc: u32 = 0x8327BE70;
    'dispatch: loop {
        match pc {
            0x8327BE70 => {
    //   block [0x8327BE70..0x8327BEB0)
	// 8327BE70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327BE74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327BE78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327BE7C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327BE80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327BE84: 388BA9D4  addi r4, r11, -0x562c
	ctx.r[4].s64 = ctx.r[11].s64 + -22060;
	// 8327BE88: 386ADA3C  addi r3, r10, -0x25c4
	ctx.r[3].s64 = ctx.r[10].s64 + -9668;
	// 8327BE8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327BE90: 4AFB1041  bl 0x8222ced0
	ctx.lr = 0x8327BE94;
	sub_8222CED0(ctx, base);
	// 8327BE94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327BE98: 386903C8  addi r3, r9, 0x3c8
	ctx.r[3].s64 = ctx.r[9].s64 + 968;
	// 8327BE9C: 4BA2E085  bl 0x82ca9f20
	ctx.lr = 0x8327BEA0;
	sub_82CA9F20(ctx, base);
	// 8327BEA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327BEA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327BEA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327BEAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327BEB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327BEB0 size=64
    let mut pc: u32 = 0x8327BEB0;
    'dispatch: loop {
        match pc {
            0x8327BEB0 => {
    //   block [0x8327BEB0..0x8327BEF0)
	// 8327BEB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327BEB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327BEB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327BEBC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327BEC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327BEC4: 388BBDF4  addi r4, r11, -0x420c
	ctx.r[4].s64 = ctx.r[11].s64 + -16908;
	// 8327BEC8: 386ADA40  addi r3, r10, -0x25c0
	ctx.r[3].s64 = ctx.r[10].s64 + -9664;
	// 8327BECC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327BED0: 4AFB1001  bl 0x8222ced0
	ctx.lr = 0x8327BED4;
	sub_8222CED0(ctx, base);
	// 8327BED4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327BED8: 386903D8  addi r3, r9, 0x3d8
	ctx.r[3].s64 = ctx.r[9].s64 + 984;
	// 8327BEDC: 4BA2E045  bl 0x82ca9f20
	ctx.lr = 0x8327BEE0;
	sub_82CA9F20(ctx, base);
	// 8327BEE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327BEE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327BEE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327BEEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327BEF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327BEF0 size=64
    let mut pc: u32 = 0x8327BEF0;
    'dispatch: loop {
        match pc {
            0x8327BEF0 => {
    //   block [0x8327BEF0..0x8327BF30)
	// 8327BEF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327BEF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327BEF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327BEFC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327BF00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327BF04: 388BBDC4  addi r4, r11, -0x423c
	ctx.r[4].s64 = ctx.r[11].s64 + -16956;
	// 8327BF08: 386ADA44  addi r3, r10, -0x25bc
	ctx.r[3].s64 = ctx.r[10].s64 + -9660;
	// 8327BF0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327BF10: 4AFB0FC1  bl 0x8222ced0
	ctx.lr = 0x8327BF14;
	sub_8222CED0(ctx, base);
	// 8327BF14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327BF18: 386903E8  addi r3, r9, 0x3e8
	ctx.r[3].s64 = ctx.r[9].s64 + 1000;
	// 8327BF1C: 4BA2E005  bl 0x82ca9f20
	ctx.lr = 0x8327BF20;
	sub_82CA9F20(ctx, base);
	// 8327BF20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327BF24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327BF28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327BF2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327BF30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327BF30 size=64
    let mut pc: u32 = 0x8327BF30;
    'dispatch: loop {
        match pc {
            0x8327BF30 => {
    //   block [0x8327BF30..0x8327BF70)
	// 8327BF30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327BF34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327BF38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327BF3C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327BF40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327BF44: 388BBBB8  addi r4, r11, -0x4448
	ctx.r[4].s64 = ctx.r[11].s64 + -17480;
	// 8327BF48: 386ADA48  addi r3, r10, -0x25b8
	ctx.r[3].s64 = ctx.r[10].s64 + -9656;
	// 8327BF4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327BF50: 4AFB0F81  bl 0x8222ced0
	ctx.lr = 0x8327BF54;
	sub_8222CED0(ctx, base);
	// 8327BF54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327BF58: 386903F8  addi r3, r9, 0x3f8
	ctx.r[3].s64 = ctx.r[9].s64 + 1016;
	// 8327BF5C: 4BA2DFC5  bl 0x82ca9f20
	ctx.lr = 0x8327BF60;
	sub_82CA9F20(ctx, base);
	// 8327BF60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327BF64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327BF68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327BF6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327BF70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327BF70 size=64
    let mut pc: u32 = 0x8327BF70;
    'dispatch: loop {
        match pc {
            0x8327BF70 => {
    //   block [0x8327BF70..0x8327BFB0)
	// 8327BF70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327BF74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327BF78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327BF7C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327BF80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327BF84: 388B9F80  addi r4, r11, -0x6080
	ctx.r[4].s64 = ctx.r[11].s64 + -24704;
	// 8327BF88: 386ADA4C  addi r3, r10, -0x25b4
	ctx.r[3].s64 = ctx.r[10].s64 + -9652;
	// 8327BF8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327BF90: 4AFB0F41  bl 0x8222ced0
	ctx.lr = 0x8327BF94;
	sub_8222CED0(ctx, base);
	// 8327BF94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327BF98: 38690408  addi r3, r9, 0x408
	ctx.r[3].s64 = ctx.r[9].s64 + 1032;
	// 8327BF9C: 4BA2DF85  bl 0x82ca9f20
	ctx.lr = 0x8327BFA0;
	sub_82CA9F20(ctx, base);
	// 8327BFA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327BFA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327BFA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327BFAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327BFB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327BFB0 size=64
    let mut pc: u32 = 0x8327BFB0;
    'dispatch: loop {
        match pc {
            0x8327BFB0 => {
    //   block [0x8327BFB0..0x8327BFF0)
	// 8327BFB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327BFB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327BFB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327BFBC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327BFC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327BFC4: 388BAD48  addi r4, r11, -0x52b8
	ctx.r[4].s64 = ctx.r[11].s64 + -21176;
	// 8327BFC8: 386ADA50  addi r3, r10, -0x25b0
	ctx.r[3].s64 = ctx.r[10].s64 + -9648;
	// 8327BFCC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327BFD0: 4AFB0F01  bl 0x8222ced0
	ctx.lr = 0x8327BFD4;
	sub_8222CED0(ctx, base);
	// 8327BFD4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327BFD8: 38690418  addi r3, r9, 0x418
	ctx.r[3].s64 = ctx.r[9].s64 + 1048;
	// 8327BFDC: 4BA2DF45  bl 0x82ca9f20
	ctx.lr = 0x8327BFE0;
	sub_82CA9F20(ctx, base);
	// 8327BFE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327BFE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327BFE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327BFEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327BFF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327BFF0 size=64
    let mut pc: u32 = 0x8327BFF0;
    'dispatch: loop {
        match pc {
            0x8327BFF0 => {
    //   block [0x8327BFF0..0x8327C030)
	// 8327BFF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327BFF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327BFF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327BFFC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327C000: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327C004: 388BBC28  addi r4, r11, -0x43d8
	ctx.r[4].s64 = ctx.r[11].s64 + -17368;
	// 8327C008: 386ADA54  addi r3, r10, -0x25ac
	ctx.r[3].s64 = ctx.r[10].s64 + -9644;
	// 8327C00C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C010: 4AFB0EC1  bl 0x8222ced0
	ctx.lr = 0x8327C014;
	sub_8222CED0(ctx, base);
	// 8327C014: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327C018: 38690428  addi r3, r9, 0x428
	ctx.r[3].s64 = ctx.r[9].s64 + 1064;
	// 8327C01C: 4BA2DF05  bl 0x82ca9f20
	ctx.lr = 0x8327C020;
	sub_82CA9F20(ctx, base);
	// 8327C020: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C024: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C028: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C02C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C030 size=64
    let mut pc: u32 = 0x8327C030;
    'dispatch: loop {
        match pc {
            0x8327C030 => {
    //   block [0x8327C030..0x8327C070)
	// 8327C030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327C038: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327C03C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327C040: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327C044: 388BAD48  addi r4, r11, -0x52b8
	ctx.r[4].s64 = ctx.r[11].s64 + -21176;
	// 8327C048: 386ADA58  addi r3, r10, -0x25a8
	ctx.r[3].s64 = ctx.r[10].s64 + -9640;
	// 8327C04C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C050: 4AFB0E81  bl 0x8222ced0
	ctx.lr = 0x8327C054;
	sub_8222CED0(ctx, base);
	// 8327C054: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327C058: 38690438  addi r3, r9, 0x438
	ctx.r[3].s64 = ctx.r[9].s64 + 1080;
	// 8327C05C: 4BA2DEC5  bl 0x82ca9f20
	ctx.lr = 0x8327C060;
	sub_82CA9F20(ctx, base);
	// 8327C060: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C064: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C068: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C06C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C070 size=64
    let mut pc: u32 = 0x8327C070;
    'dispatch: loop {
        match pc {
            0x8327C070 => {
    //   block [0x8327C070..0x8327C0B0)
	// 8327C070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327C078: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327C07C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327C080: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327C084: 388BBC28  addi r4, r11, -0x43d8
	ctx.r[4].s64 = ctx.r[11].s64 + -17368;
	// 8327C088: 386ADA5C  addi r3, r10, -0x25a4
	ctx.r[3].s64 = ctx.r[10].s64 + -9636;
	// 8327C08C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C090: 4AFB0E41  bl 0x8222ced0
	ctx.lr = 0x8327C094;
	sub_8222CED0(ctx, base);
	// 8327C094: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327C098: 38690448  addi r3, r9, 0x448
	ctx.r[3].s64 = ctx.r[9].s64 + 1096;
	// 8327C09C: 4BA2DE85  bl 0x82ca9f20
	ctx.lr = 0x8327C0A0;
	sub_82CA9F20(ctx, base);
	// 8327C0A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C0A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C0A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C0AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C0B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C0B0 size=56
    let mut pc: u32 = 0x8327C0B0;
    'dispatch: loop {
        match pc {
            0x8327C0B0 => {
    //   block [0x8327C0B0..0x8327C0E8)
	// 8327C0B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C0B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327C0B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327C0BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327C0C0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327C0C4: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 8327C0C8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327C0CC: 4AF77C8D  bl 0x821f3d58
	ctx.lr = 0x8327C0D0;
	sub_821F3D58(ctx, base);
	// 8327C0D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327C0D4: 906ADA60  stw r3, -0x25a0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9632 as u32), ctx.r[3].u32 ) };
	// 8327C0D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C0DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C0E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C0E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C0E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C0E8 size=56
    let mut pc: u32 = 0x8327C0E8;
    'dispatch: loop {
        match pc {
            0x8327C0E8 => {
    //   block [0x8327C0E8..0x8327C120)
	// 8327C0E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C0EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327C0F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327C0F4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327C0F8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327C0FC: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 8327C100: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327C104: 4AF77C55  bl 0x821f3d58
	ctx.lr = 0x8327C108;
	sub_821F3D58(ctx, base);
	// 8327C108: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327C10C: 906ADA64  stw r3, -0x259c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9628 as u32), ctx.r[3].u32 ) };
	// 8327C110: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C114: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C118: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C11C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C120 size=56
    let mut pc: u32 = 0x8327C120;
    'dispatch: loop {
        match pc {
            0x8327C120 => {
    //   block [0x8327C120..0x8327C158)
	// 8327C120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327C128: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327C12C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327C130: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327C134: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 8327C138: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327C13C: 4AF77C1D  bl 0x821f3d58
	ctx.lr = 0x8327C140;
	sub_821F3D58(ctx, base);
	// 8327C140: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327C144: 906ADA68  stw r3, -0x2598(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9624 as u32), ctx.r[3].u32 ) };
	// 8327C148: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C14C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C150: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C154: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C158 size=56
    let mut pc: u32 = 0x8327C158;
    'dispatch: loop {
        match pc {
            0x8327C158 => {
    //   block [0x8327C158..0x8327C190)
	// 8327C158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C15C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327C160: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327C164: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327C168: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327C16C: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 8327C170: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327C174: 4AF77BE5  bl 0x821f3d58
	ctx.lr = 0x8327C178;
	sub_821F3D58(ctx, base);
	// 8327C178: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327C17C: 906ADA6C  stw r3, -0x2594(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9620 as u32), ctx.r[3].u32 ) };
	// 8327C180: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C184: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C188: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C18C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C190 size=56
    let mut pc: u32 = 0x8327C190;
    'dispatch: loop {
        match pc {
            0x8327C190 => {
    //   block [0x8327C190..0x8327C1C8)
	// 8327C190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327C198: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327C19C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327C1A0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327C1A4: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 8327C1A8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327C1AC: 4AF77BAD  bl 0x821f3d58
	ctx.lr = 0x8327C1B0;
	sub_821F3D58(ctx, base);
	// 8327C1B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327C1B4: 906ADA70  stw r3, -0x2590(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9616 as u32), ctx.r[3].u32 ) };
	// 8327C1B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C1BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C1C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C1C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C1C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C1C8 size=56
    let mut pc: u32 = 0x8327C1C8;
    'dispatch: loop {
        match pc {
            0x8327C1C8 => {
    //   block [0x8327C1C8..0x8327C200)
	// 8327C1C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C1CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327C1D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327C1D4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327C1D8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327C1DC: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 8327C1E0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327C1E4: 4AF77B75  bl 0x821f3d58
	ctx.lr = 0x8327C1E8;
	sub_821F3D58(ctx, base);
	// 8327C1E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327C1EC: 906ADA74  stw r3, -0x258c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9612 as u32), ctx.r[3].u32 ) };
	// 8327C1F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C1F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C1F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C1FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C200 size=56
    let mut pc: u32 = 0x8327C200;
    'dispatch: loop {
        match pc {
            0x8327C200 => {
    //   block [0x8327C200..0x8327C238)
	// 8327C200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327C208: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327C20C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327C210: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327C214: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 8327C218: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327C21C: 4AF77B3D  bl 0x821f3d58
	ctx.lr = 0x8327C220;
	sub_821F3D58(ctx, base);
	// 8327C220: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327C224: 906ADA78  stw r3, -0x2588(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9608 as u32), ctx.r[3].u32 ) };
	// 8327C228: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C22C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C230: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C234: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C238 size=56
    let mut pc: u32 = 0x8327C238;
    'dispatch: loop {
        match pc {
            0x8327C238 => {
    //   block [0x8327C238..0x8327C270)
	// 8327C238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C23C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327C240: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327C244: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327C248: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327C24C: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 8327C250: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327C254: 4AF77B05  bl 0x821f3d58
	ctx.lr = 0x8327C258;
	sub_821F3D58(ctx, base);
	// 8327C258: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327C25C: 906ADA7C  stw r3, -0x2584(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9604 as u32), ctx.r[3].u32 ) };
	// 8327C260: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C264: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C268: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C26C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C270 size=56
    let mut pc: u32 = 0x8327C270;
    'dispatch: loop {
        match pc {
            0x8327C270 => {
    //   block [0x8327C270..0x8327C2A8)
	// 8327C270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327C278: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327C27C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327C280: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327C284: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 8327C288: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327C28C: 4AF77ACD  bl 0x821f3d58
	ctx.lr = 0x8327C290;
	sub_821F3D58(ctx, base);
	// 8327C290: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327C294: 906ADA80  stw r3, -0x2580(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9600 as u32), ctx.r[3].u32 ) };
	// 8327C298: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C29C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C2A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C2A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C2A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C2A8 size=56
    let mut pc: u32 = 0x8327C2A8;
    'dispatch: loop {
        match pc {
            0x8327C2A8 => {
    //   block [0x8327C2A8..0x8327C2E0)
	// 8327C2A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C2AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327C2B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327C2B4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327C2B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327C2BC: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 8327C2C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327C2C4: 4AF77A95  bl 0x821f3d58
	ctx.lr = 0x8327C2C8;
	sub_821F3D58(ctx, base);
	// 8327C2C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327C2CC: 906ADA84  stw r3, -0x257c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9596 as u32), ctx.r[3].u32 ) };
	// 8327C2D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C2D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C2D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C2DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C2E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C2E0 size=56
    let mut pc: u32 = 0x8327C2E0;
    'dispatch: loop {
        match pc {
            0x8327C2E0 => {
    //   block [0x8327C2E0..0x8327C318)
	// 8327C2E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C2E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327C2E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327C2EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327C2F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327C2F4: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 8327C2F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327C2FC: 4AF77A5D  bl 0x821f3d58
	ctx.lr = 0x8327C300;
	sub_821F3D58(ctx, base);
	// 8327C300: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327C304: 906ADA88  stw r3, -0x2578(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9592 as u32), ctx.r[3].u32 ) };
	// 8327C308: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C30C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C310: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C314: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C318 size=56
    let mut pc: u32 = 0x8327C318;
    'dispatch: loop {
        match pc {
            0x8327C318 => {
    //   block [0x8327C318..0x8327C350)
	// 8327C318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C31C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327C320: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327C324: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327C328: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327C32C: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 8327C330: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327C334: 4AF77A25  bl 0x821f3d58
	ctx.lr = 0x8327C338;
	sub_821F3D58(ctx, base);
	// 8327C338: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327C33C: 906ADA8C  stw r3, -0x2574(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9588 as u32), ctx.r[3].u32 ) };
	// 8327C340: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C344: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C348: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C34C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C350 size=56
    let mut pc: u32 = 0x8327C350;
    'dispatch: loop {
        match pc {
            0x8327C350 => {
    //   block [0x8327C350..0x8327C388)
	// 8327C350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327C358: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327C35C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327C360: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327C364: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 8327C368: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327C36C: 4AF779ED  bl 0x821f3d58
	ctx.lr = 0x8327C370;
	sub_821F3D58(ctx, base);
	// 8327C370: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327C374: 906ADA90  stw r3, -0x2570(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9584 as u32), ctx.r[3].u32 ) };
	// 8327C378: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C37C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C380: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C384: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C388 size=56
    let mut pc: u32 = 0x8327C388;
    'dispatch: loop {
        match pc {
            0x8327C388 => {
    //   block [0x8327C388..0x8327C3C0)
	// 8327C388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C38C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327C390: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327C394: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327C398: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327C39C: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 8327C3A0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327C3A4: 4AF779B5  bl 0x821f3d58
	ctx.lr = 0x8327C3A8;
	sub_821F3D58(ctx, base);
	// 8327C3A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327C3AC: 906ADA94  stw r3, -0x256c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9580 as u32), ctx.r[3].u32 ) };
	// 8327C3B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C3B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C3B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C3BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C3C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C3C0 size=56
    let mut pc: u32 = 0x8327C3C0;
    'dispatch: loop {
        match pc {
            0x8327C3C0 => {
    //   block [0x8327C3C0..0x8327C3F8)
	// 8327C3C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C3C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327C3C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327C3CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327C3D0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327C3D4: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 8327C3D8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327C3DC: 4AF7797D  bl 0x821f3d58
	ctx.lr = 0x8327C3E0;
	sub_821F3D58(ctx, base);
	// 8327C3E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327C3E4: 906ADA98  stw r3, -0x2568(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9576 as u32), ctx.r[3].u32 ) };
	// 8327C3E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C3EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C3F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C3F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C3F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C3F8 size=56
    let mut pc: u32 = 0x8327C3F8;
    'dispatch: loop {
        match pc {
            0x8327C3F8 => {
    //   block [0x8327C3F8..0x8327C430)
	// 8327C3F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C3FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327C400: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327C404: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327C408: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327C40C: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 8327C410: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327C414: 4AF77945  bl 0x821f3d58
	ctx.lr = 0x8327C418;
	sub_821F3D58(ctx, base);
	// 8327C418: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327C41C: 906ADA9C  stw r3, -0x2564(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9572 as u32), ctx.r[3].u32 ) };
	// 8327C420: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C424: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C428: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C42C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C430 size=56
    let mut pc: u32 = 0x8327C430;
    'dispatch: loop {
        match pc {
            0x8327C430 => {
    //   block [0x8327C430..0x8327C468)
	// 8327C430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327C438: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327C43C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327C440: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327C444: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 8327C448: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327C44C: 4AF7790D  bl 0x821f3d58
	ctx.lr = 0x8327C450;
	sub_821F3D58(ctx, base);
	// 8327C450: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327C454: 906ADAA0  stw r3, -0x2560(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9568 as u32), ctx.r[3].u32 ) };
	// 8327C458: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C45C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C460: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C464: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C468 size=56
    let mut pc: u32 = 0x8327C468;
    'dispatch: loop {
        match pc {
            0x8327C468 => {
    //   block [0x8327C468..0x8327C4A0)
	// 8327C468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C46C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327C470: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327C474: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327C478: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327C47C: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 8327C480: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327C484: 4AF778D5  bl 0x821f3d58
	ctx.lr = 0x8327C488;
	sub_821F3D58(ctx, base);
	// 8327C488: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327C48C: 906ADAA4  stw r3, -0x255c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9564 as u32), ctx.r[3].u32 ) };
	// 8327C490: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C494: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C498: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C49C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C4A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C4A0 size=56
    let mut pc: u32 = 0x8327C4A0;
    'dispatch: loop {
        match pc {
            0x8327C4A0 => {
    //   block [0x8327C4A0..0x8327C4D8)
	// 8327C4A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C4A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327C4A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327C4AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327C4B0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327C4B4: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 8327C4B8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327C4BC: 4AF7789D  bl 0x821f3d58
	ctx.lr = 0x8327C4C0;
	sub_821F3D58(ctx, base);
	// 8327C4C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327C4C4: 906ADAA8  stw r3, -0x2558(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9560 as u32), ctx.r[3].u32 ) };
	// 8327C4C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C4CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C4D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C4D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C4D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C4D8 size=56
    let mut pc: u32 = 0x8327C4D8;
    'dispatch: loop {
        match pc {
            0x8327C4D8 => {
    //   block [0x8327C4D8..0x8327C510)
	// 8327C4D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C4DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327C4E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327C4E4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327C4E8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327C4EC: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 8327C4F0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327C4F4: 4AF77865  bl 0x821f3d58
	ctx.lr = 0x8327C4F8;
	sub_821F3D58(ctx, base);
	// 8327C4F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327C4FC: 906ADAAC  stw r3, -0x2554(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9556 as u32), ctx.r[3].u32 ) };
	// 8327C500: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C504: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C508: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C50C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C510 size=56
    let mut pc: u32 = 0x8327C510;
    'dispatch: loop {
        match pc {
            0x8327C510 => {
    //   block [0x8327C510..0x8327C548)
	// 8327C510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327C518: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327C51C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327C520: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327C524: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 8327C528: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327C52C: 4AF7782D  bl 0x821f3d58
	ctx.lr = 0x8327C530;
	sub_821F3D58(ctx, base);
	// 8327C530: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327C534: 906ADAB0  stw r3, -0x2550(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9552 as u32), ctx.r[3].u32 ) };
	// 8327C538: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C53C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C540: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C544: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C548 size=376
    let mut pc: u32 = 0x8327C548;
    'dispatch: loop {
        match pc {
            0x8327C548 => {
    //   block [0x8327C548..0x8327C6C0)
	// 8327C548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C54C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327C550: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8327C554: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327C558: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8327C55C: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 8327C560: 3BEBDAB8  addi r31, r11, -0x2548
	ctx.r[31].s64 = ctx.r[11].s64 + -9544;
	// 8327C564: 388A8A50  addi r4, r10, -0x75b0
	ctx.r[4].s64 = ctx.r[10].s64 + -30128;
	// 8327C568: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8327C56C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C570: 4AFB0961  bl 0x8222ced0
	ctx.lr = 0x8327C574;
	sub_8222CED0(ctx, base);
	// 8327C574: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 8327C578: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C57C: 38898A30  addi r4, r9, -0x75d0
	ctx.r[4].s64 = ctx.r[9].s64 + -30160;
	// 8327C580: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8327C584: 4AFB094D  bl 0x8222ced0
	ctx.lr = 0x8327C588;
	sub_8222CED0(ctx, base);
	// 8327C588: 3D00820B  lis r8, -0x7df5
	ctx.r[8].s64 = -2113208320;
	// 8327C58C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C590: 38888A10  addi r4, r8, -0x75f0
	ctx.r[4].s64 = ctx.r[8].s64 + -30192;
	// 8327C594: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 8327C598: 4AFB0939  bl 0x8222ced0
	ctx.lr = 0x8327C59C;
	sub_8222CED0(ctx, base);
	// 8327C59C: 3CE0820B  lis r7, -0x7df5
	ctx.r[7].s64 = -2113208320;
	// 8327C5A0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C5A4: 388789F0  addi r4, r7, -0x7610
	ctx.r[4].s64 = ctx.r[7].s64 + -30224;
	// 8327C5A8: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 8327C5AC: 4AFB0925  bl 0x8222ced0
	ctx.lr = 0x8327C5B0;
	sub_8222CED0(ctx, base);
	// 8327C5B0: 3CC0820B  lis r6, -0x7df5
	ctx.r[6].s64 = -2113208320;
	// 8327C5B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C5B8: 388689D0  addi r4, r6, -0x7630
	ctx.r[4].s64 = ctx.r[6].s64 + -30256;
	// 8327C5BC: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 8327C5C0: 4AFB0911  bl 0x8222ced0
	ctx.lr = 0x8327C5C4;
	sub_8222CED0(ctx, base);
	// 8327C5C4: 3C80820B  lis r4, -0x7df5
	ctx.r[4].s64 = -2113208320;
	// 8327C5C8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C5CC: 388489B0  addi r4, r4, -0x7650
	ctx.r[4].s64 = ctx.r[4].s64 + -30288;
	// 8327C5D0: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 8327C5D4: 4AFB08FD  bl 0x8222ced0
	ctx.lr = 0x8327C5D8;
	sub_8222CED0(ctx, base);
	// 8327C5D8: 3C60820B  lis r3, -0x7df5
	ctx.r[3].s64 = -2113208320;
	// 8327C5DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C5E0: 38838990  addi r4, r3, -0x7670
	ctx.r[4].s64 = ctx.r[3].s64 + -30320;
	// 8327C5E4: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 8327C5E8: 4AFB08E9  bl 0x8222ced0
	ctx.lr = 0x8327C5EC;
	sub_8222CED0(ctx, base);
	// 8327C5EC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8327C5F0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C5F4: 388B8970  addi r4, r11, -0x7690
	ctx.r[4].s64 = ctx.r[11].s64 + -30352;
	// 8327C5F8: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 8327C5FC: 4AFB08D5  bl 0x8222ced0
	ctx.lr = 0x8327C600;
	sub_8222CED0(ctx, base);
	// 8327C600: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 8327C604: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C608: 388A8950  addi r4, r10, -0x76b0
	ctx.r[4].s64 = ctx.r[10].s64 + -30384;
	// 8327C60C: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 8327C610: 4AFB08C1  bl 0x8222ced0
	ctx.lr = 0x8327C614;
	sub_8222CED0(ctx, base);
	// 8327C614: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 8327C618: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C61C: 38898930  addi r4, r9, -0x76d0
	ctx.r[4].s64 = ctx.r[9].s64 + -30416;
	// 8327C620: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 8327C624: 4AFB08AD  bl 0x8222ced0
	ctx.lr = 0x8327C628;
	sub_8222CED0(ctx, base);
	// 8327C628: 3D00820B  lis r8, -0x7df5
	ctx.r[8].s64 = -2113208320;
	// 8327C62C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C630: 38888914  addi r4, r8, -0x76ec
	ctx.r[4].s64 = ctx.r[8].s64 + -30444;
	// 8327C634: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 8327C638: 4AFB0899  bl 0x8222ced0
	ctx.lr = 0x8327C63C;
	sub_8222CED0(ctx, base);
	// 8327C63C: 3CE0820B  lis r7, -0x7df5
	ctx.r[7].s64 = -2113208320;
	// 8327C640: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C644: 388788F0  addi r4, r7, -0x7710
	ctx.r[4].s64 = ctx.r[7].s64 + -30480;
	// 8327C648: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 8327C64C: 4AFB0885  bl 0x8222ced0
	ctx.lr = 0x8327C650;
	sub_8222CED0(ctx, base);
	// 8327C650: 3CC0820B  lis r6, -0x7df5
	ctx.r[6].s64 = -2113208320;
	// 8327C654: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C658: 388688CC  addi r4, r6, -0x7734
	ctx.r[4].s64 = ctx.r[6].s64 + -30516;
	// 8327C65C: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 8327C660: 4AFB0871  bl 0x8222ced0
	ctx.lr = 0x8327C664;
	sub_8222CED0(ctx, base);
	// 8327C664: 3C80820B  lis r4, -0x7df5
	ctx.r[4].s64 = -2113208320;
	// 8327C668: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C66C: 388488AC  addi r4, r4, -0x7754
	ctx.r[4].s64 = ctx.r[4].s64 + -30548;
	// 8327C670: 387F0034  addi r3, r31, 0x34
	ctx.r[3].s64 = ctx.r[31].s64 + 52;
	// 8327C674: 4AFB085D  bl 0x8222ced0
	ctx.lr = 0x8327C678;
	sub_8222CED0(ctx, base);
	// 8327C678: 3C60820B  lis r3, -0x7df5
	ctx.r[3].s64 = -2113208320;
	// 8327C67C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C680: 3883888C  addi r4, r3, -0x7774
	ctx.r[4].s64 = ctx.r[3].s64 + -30580;
	// 8327C684: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 8327C688: 4AFB0849  bl 0x8222ced0
	ctx.lr = 0x8327C68C;
	sub_8222CED0(ctx, base);
	// 8327C68C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8327C690: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C694: 388B8868  addi r4, r11, -0x7798
	ctx.r[4].s64 = ctx.r[11].s64 + -30616;
	// 8327C698: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 8327C69C: 4AFB0835  bl 0x8222ced0
	ctx.lr = 0x8327C6A0;
	sub_8222CED0(ctx, base);
	// 8327C6A0: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 8327C6A4: 386A0458  addi r3, r10, 0x458
	ctx.r[3].s64 = ctx.r[10].s64 + 1112;
	// 8327C6A8: 4BA2D879  bl 0x82ca9f20
	ctx.lr = 0x8327C6AC;
	sub_82CA9F20(ctx, base);
	// 8327C6AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C6B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C6B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C6B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8327C6BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C6C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C6C0 size=376
    let mut pc: u32 = 0x8327C6C0;
    'dispatch: loop {
        match pc {
            0x8327C6C0 => {
    //   block [0x8327C6C0..0x8327C838)
	// 8327C6C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C6C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327C6C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8327C6CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327C6D0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8327C6D4: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 8327C6D8: 3BEBDAF8  addi r31, r11, -0x2508
	ctx.r[31].s64 = ctx.r[11].s64 + -9480;
	// 8327C6DC: 388A8C58  addi r4, r10, -0x73a8
	ctx.r[4].s64 = ctx.r[10].s64 + -29608;
	// 8327C6E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8327C6E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C6E8: 4AFB07E9  bl 0x8222ced0
	ctx.lr = 0x8327C6EC;
	sub_8222CED0(ctx, base);
	// 8327C6EC: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 8327C6F0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C6F4: 38898C38  addi r4, r9, -0x73c8
	ctx.r[4].s64 = ctx.r[9].s64 + -29640;
	// 8327C6F8: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8327C6FC: 4AFB07D5  bl 0x8222ced0
	ctx.lr = 0x8327C700;
	sub_8222CED0(ctx, base);
	// 8327C700: 3D00820B  lis r8, -0x7df5
	ctx.r[8].s64 = -2113208320;
	// 8327C704: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C708: 38888C18  addi r4, r8, -0x73e8
	ctx.r[4].s64 = ctx.r[8].s64 + -29672;
	// 8327C70C: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 8327C710: 4AFB07C1  bl 0x8222ced0
	ctx.lr = 0x8327C714;
	sub_8222CED0(ctx, base);
	// 8327C714: 3CE0820B  lis r7, -0x7df5
	ctx.r[7].s64 = -2113208320;
	// 8327C718: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C71C: 38878BF8  addi r4, r7, -0x7408
	ctx.r[4].s64 = ctx.r[7].s64 + -29704;
	// 8327C720: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 8327C724: 4AFB07AD  bl 0x8222ced0
	ctx.lr = 0x8327C728;
	sub_8222CED0(ctx, base);
	// 8327C728: 3CC0820B  lis r6, -0x7df5
	ctx.r[6].s64 = -2113208320;
	// 8327C72C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C730: 38868BD8  addi r4, r6, -0x7428
	ctx.r[4].s64 = ctx.r[6].s64 + -29736;
	// 8327C734: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 8327C738: 4AFB0799  bl 0x8222ced0
	ctx.lr = 0x8327C73C;
	sub_8222CED0(ctx, base);
	// 8327C73C: 3C80820B  lis r4, -0x7df5
	ctx.r[4].s64 = -2113208320;
	// 8327C740: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C744: 38848BB8  addi r4, r4, -0x7448
	ctx.r[4].s64 = ctx.r[4].s64 + -29768;
	// 8327C748: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 8327C74C: 4AFB0785  bl 0x8222ced0
	ctx.lr = 0x8327C750;
	sub_8222CED0(ctx, base);
	// 8327C750: 3C60820B  lis r3, -0x7df5
	ctx.r[3].s64 = -2113208320;
	// 8327C754: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C758: 38838B98  addi r4, r3, -0x7468
	ctx.r[4].s64 = ctx.r[3].s64 + -29800;
	// 8327C75C: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 8327C760: 4AFB0771  bl 0x8222ced0
	ctx.lr = 0x8327C764;
	sub_8222CED0(ctx, base);
	// 8327C764: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8327C768: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C76C: 388B8B78  addi r4, r11, -0x7488
	ctx.r[4].s64 = ctx.r[11].s64 + -29832;
	// 8327C770: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 8327C774: 4AFB075D  bl 0x8222ced0
	ctx.lr = 0x8327C778;
	sub_8222CED0(ctx, base);
	// 8327C778: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 8327C77C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C780: 388A8B58  addi r4, r10, -0x74a8
	ctx.r[4].s64 = ctx.r[10].s64 + -29864;
	// 8327C784: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 8327C788: 4AFB0749  bl 0x8222ced0
	ctx.lr = 0x8327C78C;
	sub_8222CED0(ctx, base);
	// 8327C78C: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 8327C790: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C794: 38898B38  addi r4, r9, -0x74c8
	ctx.r[4].s64 = ctx.r[9].s64 + -29896;
	// 8327C798: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 8327C79C: 4AFB0735  bl 0x8222ced0
	ctx.lr = 0x8327C7A0;
	sub_8222CED0(ctx, base);
	// 8327C7A0: 3D00820B  lis r8, -0x7df5
	ctx.r[8].s64 = -2113208320;
	// 8327C7A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C7A8: 38888B1C  addi r4, r8, -0x74e4
	ctx.r[4].s64 = ctx.r[8].s64 + -29924;
	// 8327C7AC: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 8327C7B0: 4AFB0721  bl 0x8222ced0
	ctx.lr = 0x8327C7B4;
	sub_8222CED0(ctx, base);
	// 8327C7B4: 3CE0820B  lis r7, -0x7df5
	ctx.r[7].s64 = -2113208320;
	// 8327C7B8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C7BC: 38878AF8  addi r4, r7, -0x7508
	ctx.r[4].s64 = ctx.r[7].s64 + -29960;
	// 8327C7C0: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 8327C7C4: 4AFB070D  bl 0x8222ced0
	ctx.lr = 0x8327C7C8;
	sub_8222CED0(ctx, base);
	// 8327C7C8: 3CC0820B  lis r6, -0x7df5
	ctx.r[6].s64 = -2113208320;
	// 8327C7CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C7D0: 38868AD4  addi r4, r6, -0x752c
	ctx.r[4].s64 = ctx.r[6].s64 + -29996;
	// 8327C7D4: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 8327C7D8: 4AFB06F9  bl 0x8222ced0
	ctx.lr = 0x8327C7DC;
	sub_8222CED0(ctx, base);
	// 8327C7DC: 3C80820B  lis r4, -0x7df5
	ctx.r[4].s64 = -2113208320;
	// 8327C7E0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C7E4: 38848AB4  addi r4, r4, -0x754c
	ctx.r[4].s64 = ctx.r[4].s64 + -30028;
	// 8327C7E8: 387F0034  addi r3, r31, 0x34
	ctx.r[3].s64 = ctx.r[31].s64 + 52;
	// 8327C7EC: 4AFB06E5  bl 0x8222ced0
	ctx.lr = 0x8327C7F0;
	sub_8222CED0(ctx, base);
	// 8327C7F0: 3C60820B  lis r3, -0x7df5
	ctx.r[3].s64 = -2113208320;
	// 8327C7F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C7F8: 38838A94  addi r4, r3, -0x756c
	ctx.r[4].s64 = ctx.r[3].s64 + -30060;
	// 8327C7FC: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 8327C800: 4AFB06D1  bl 0x8222ced0
	ctx.lr = 0x8327C804;
	sub_8222CED0(ctx, base);
	// 8327C804: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8327C808: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C80C: 388B8A70  addi r4, r11, -0x7590
	ctx.r[4].s64 = ctx.r[11].s64 + -30096;
	// 8327C810: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 8327C814: 4AFB06BD  bl 0x8222ced0
	ctx.lr = 0x8327C818;
	sub_8222CED0(ctx, base);
	// 8327C818: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 8327C81C: 386A04C0  addi r3, r10, 0x4c0
	ctx.r[3].s64 = ctx.r[10].s64 + 1216;
	// 8327C820: 4BA2D701  bl 0x82ca9f20
	ctx.lr = 0x8327C824;
	sub_82CA9F20(ctx, base);
	// 8327C824: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C828: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C82C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C830: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8327C834: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C838 size=64
    let mut pc: u32 = 0x8327C838;
    'dispatch: loop {
        match pc {
            0x8327C838 => {
    //   block [0x8327C838..0x8327C878)
	// 8327C838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C83C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327C840: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327C844: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8327C848: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327C84C: 388B0F58  addi r4, r11, 0xf58
	ctx.r[4].s64 = ctx.r[11].s64 + 3928;
	// 8327C850: 386ADAB4  addi r3, r10, -0x254c
	ctx.r[3].s64 = ctx.r[10].s64 + -9548;
	// 8327C854: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C858: 4AFB0679  bl 0x8222ced0
	ctx.lr = 0x8327C85C;
	sub_8222CED0(ctx, base);
	// 8327C85C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327C860: 38690528  addi r3, r9, 0x528
	ctx.r[3].s64 = ctx.r[9].s64 + 1320;
	// 8327C864: 4BA2D6BD  bl 0x82ca9f20
	ctx.lr = 0x8327C868;
	sub_82CA9F20(ctx, base);
	// 8327C868: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C86C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C870: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C874: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C878 size=64
    let mut pc: u32 = 0x8327C878;
    'dispatch: loop {
        match pc {
            0x8327C878 => {
    //   block [0x8327C878..0x8327C8B8)
	// 8327C878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C87C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327C880: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327C884: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327C888: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327C88C: 388BE58C  addi r4, r11, -0x1a74
	ctx.r[4].s64 = ctx.r[11].s64 + -6772;
	// 8327C890: 386ADB38  addi r3, r10, -0x24c8
	ctx.r[3].s64 = ctx.r[10].s64 + -9416;
	// 8327C894: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C898: 4AFB0639  bl 0x8222ced0
	ctx.lr = 0x8327C89C;
	sub_8222CED0(ctx, base);
	// 8327C89C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327C8A0: 38690538  addi r3, r9, 0x538
	ctx.r[3].s64 = ctx.r[9].s64 + 1336;
	// 8327C8A4: 4BA2D67D  bl 0x82ca9f20
	ctx.lr = 0x8327C8A8;
	sub_82CA9F20(ctx, base);
	// 8327C8A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C8AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C8B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C8B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C8B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C8B8 size=64
    let mut pc: u32 = 0x8327C8B8;
    'dispatch: loop {
        match pc {
            0x8327C8B8 => {
    //   block [0x8327C8B8..0x8327C8F8)
	// 8327C8B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C8BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327C8C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327C8C4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8327C8C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327C8CC: 388BF970  addi r4, r11, -0x690
	ctx.r[4].s64 = ctx.r[11].s64 + -1680;
	// 8327C8D0: 386ADB3C  addi r3, r10, -0x24c4
	ctx.r[3].s64 = ctx.r[10].s64 + -9412;
	// 8327C8D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C8D8: 4AFB05F9  bl 0x8222ced0
	ctx.lr = 0x8327C8DC;
	sub_8222CED0(ctx, base);
	// 8327C8DC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327C8E0: 38690548  addi r3, r9, 0x548
	ctx.r[3].s64 = ctx.r[9].s64 + 1352;
	// 8327C8E4: 4BA2D63D  bl 0x82ca9f20
	ctx.lr = 0x8327C8E8;
	sub_82CA9F20(ctx, base);
	// 8327C8E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C8EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C8F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C8F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C8F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C8F8 size=64
    let mut pc: u32 = 0x8327C8F8;
    'dispatch: loop {
        match pc {
            0x8327C8F8 => {
    //   block [0x8327C8F8..0x8327C938)
	// 8327C8F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C8FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327C900: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327C904: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8327C908: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327C90C: 388BE32C  addi r4, r11, -0x1cd4
	ctx.r[4].s64 = ctx.r[11].s64 + -7380;
	// 8327C910: 386ADB40  addi r3, r10, -0x24c0
	ctx.r[3].s64 = ctx.r[10].s64 + -9408;
	// 8327C914: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C918: 4AFB05B9  bl 0x8222ced0
	ctx.lr = 0x8327C91C;
	sub_8222CED0(ctx, base);
	// 8327C91C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327C920: 38690558  addi r3, r9, 0x558
	ctx.r[3].s64 = ctx.r[9].s64 + 1368;
	// 8327C924: 4BA2D5FD  bl 0x82ca9f20
	ctx.lr = 0x8327C928;
	sub_82CA9F20(ctx, base);
	// 8327C928: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C92C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C930: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C934: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C938 size=64
    let mut pc: u32 = 0x8327C938;
    'dispatch: loop {
        match pc {
            0x8327C938 => {
    //   block [0x8327C938..0x8327C978)
	// 8327C938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C93C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327C940: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327C944: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8327C948: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327C94C: 388BE2FC  addi r4, r11, -0x1d04
	ctx.r[4].s64 = ctx.r[11].s64 + -7428;
	// 8327C950: 386ADB44  addi r3, r10, -0x24bc
	ctx.r[3].s64 = ctx.r[10].s64 + -9404;
	// 8327C954: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C958: 4AFB0579  bl 0x8222ced0
	ctx.lr = 0x8327C95C;
	sub_8222CED0(ctx, base);
	// 8327C95C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327C960: 38690568  addi r3, r9, 0x568
	ctx.r[3].s64 = ctx.r[9].s64 + 1384;
	// 8327C964: 4BA2D5BD  bl 0x82ca9f20
	ctx.lr = 0x8327C968;
	sub_82CA9F20(ctx, base);
	// 8327C968: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C96C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C970: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C974: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C978 size=64
    let mut pc: u32 = 0x8327C978;
    'dispatch: loop {
        match pc {
            0x8327C978 => {
    //   block [0x8327C978..0x8327C9B8)
	// 8327C978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C97C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327C980: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327C984: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8327C988: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327C98C: 388BE2A0  addi r4, r11, -0x1d60
	ctx.r[4].s64 = ctx.r[11].s64 + -7520;
	// 8327C990: 386ADB48  addi r3, r10, -0x24b8
	ctx.r[3].s64 = ctx.r[10].s64 + -9400;
	// 8327C994: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C998: 4AFB0539  bl 0x8222ced0
	ctx.lr = 0x8327C99C;
	sub_8222CED0(ctx, base);
	// 8327C99C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327C9A0: 38690578  addi r3, r9, 0x578
	ctx.r[3].s64 = ctx.r[9].s64 + 1400;
	// 8327C9A4: 4BA2D57D  bl 0x82ca9f20
	ctx.lr = 0x8327C9A8;
	sub_82CA9F20(ctx, base);
	// 8327C9A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C9AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C9B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C9B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C9B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C9B8 size=64
    let mut pc: u32 = 0x8327C9B8;
    'dispatch: loop {
        match pc {
            0x8327C9B8 => {
    //   block [0x8327C9B8..0x8327C9F8)
	// 8327C9B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C9BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327C9C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327C9C4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8327C9C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327C9CC: 388BE3A8  addi r4, r11, -0x1c58
	ctx.r[4].s64 = ctx.r[11].s64 + -7256;
	// 8327C9D0: 386ADB4C  addi r3, r10, -0x24b4
	ctx.r[3].s64 = ctx.r[10].s64 + -9396;
	// 8327C9D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C9D8: 4AFB04F9  bl 0x8222ced0
	ctx.lr = 0x8327C9DC;
	sub_8222CED0(ctx, base);
	// 8327C9DC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327C9E0: 38690588  addi r3, r9, 0x588
	ctx.r[3].s64 = ctx.r[9].s64 + 1416;
	// 8327C9E4: 4BA2D53D  bl 0x82ca9f20
	ctx.lr = 0x8327C9E8;
	sub_82CA9F20(ctx, base);
	// 8327C9E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C9EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C9F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C9F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C9F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C9F8 size=64
    let mut pc: u32 = 0x8327C9F8;
    'dispatch: loop {
        match pc {
            0x8327C9F8 => {
    //   block [0x8327C9F8..0x8327CA38)
	// 8327C9F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C9FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327CA00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327CA04: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8327CA08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327CA0C: 388BE36C  addi r4, r11, -0x1c94
	ctx.r[4].s64 = ctx.r[11].s64 + -7316;
	// 8327CA10: 386ADB50  addi r3, r10, -0x24b0
	ctx.r[3].s64 = ctx.r[10].s64 + -9392;
	// 8327CA14: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327CA18: 4AFB04B9  bl 0x8222ced0
	ctx.lr = 0x8327CA1C;
	sub_8222CED0(ctx, base);
	// 8327CA1C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327CA20: 38690598  addi r3, r9, 0x598
	ctx.r[3].s64 = ctx.r[9].s64 + 1432;
	// 8327CA24: 4BA2D4FD  bl 0x82ca9f20
	ctx.lr = 0x8327CA28;
	sub_82CA9F20(ctx, base);
	// 8327CA28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327CA2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327CA30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327CA34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327CA38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327CA38 size=64
    let mut pc: u32 = 0x8327CA38;
    'dispatch: loop {
        match pc {
            0x8327CA38 => {
    //   block [0x8327CA38..0x8327CA78)
	// 8327CA38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327CA3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327CA40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327CA44: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327CA48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327CA4C: 388BE598  addi r4, r11, -0x1a68
	ctx.r[4].s64 = ctx.r[11].s64 + -6760;
	// 8327CA50: 386ADB54  addi r3, r10, -0x24ac
	ctx.r[3].s64 = ctx.r[10].s64 + -9388;
	// 8327CA54: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327CA58: 4AFB0479  bl 0x8222ced0
	ctx.lr = 0x8327CA5C;
	sub_8222CED0(ctx, base);
	// 8327CA5C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327CA60: 386905A8  addi r3, r9, 0x5a8
	ctx.r[3].s64 = ctx.r[9].s64 + 1448;
	// 8327CA64: 4BA2D4BD  bl 0x82ca9f20
	ctx.lr = 0x8327CA68;
	sub_82CA9F20(ctx, base);
	// 8327CA68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327CA6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327CA70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327CA74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327CA78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327CA78 size=64
    let mut pc: u32 = 0x8327CA78;
    'dispatch: loop {
        match pc {
            0x8327CA78 => {
    //   block [0x8327CA78..0x8327CAB8)
	// 8327CA78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327CA7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327CA80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327CA84: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8327CA88: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327CA8C: 388BF970  addi r4, r11, -0x690
	ctx.r[4].s64 = ctx.r[11].s64 + -1680;
	// 8327CA90: 386ADB58  addi r3, r10, -0x24a8
	ctx.r[3].s64 = ctx.r[10].s64 + -9384;
	// 8327CA94: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327CA98: 4AFB0439  bl 0x8222ced0
	ctx.lr = 0x8327CA9C;
	sub_8222CED0(ctx, base);
	// 8327CA9C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327CAA0: 386905B8  addi r3, r9, 0x5b8
	ctx.r[3].s64 = ctx.r[9].s64 + 1464;
	// 8327CAA4: 4BA2D47D  bl 0x82ca9f20
	ctx.lr = 0x8327CAA8;
	sub_82CA9F20(ctx, base);
	// 8327CAA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327CAAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327CAB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327CAB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327CAB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327CAB8 size=64
    let mut pc: u32 = 0x8327CAB8;
    'dispatch: loop {
        match pc {
            0x8327CAB8 => {
    //   block [0x8327CAB8..0x8327CAF8)
	// 8327CAB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327CABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327CAC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327CAC4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327CAC8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327CACC: 388BE5A8  addi r4, r11, -0x1a58
	ctx.r[4].s64 = ctx.r[11].s64 + -6744;
	// 8327CAD0: 386ADB5C  addi r3, r10, -0x24a4
	ctx.r[3].s64 = ctx.r[10].s64 + -9380;
	// 8327CAD4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327CAD8: 4AFB03F9  bl 0x8222ced0
	ctx.lr = 0x8327CADC;
	sub_8222CED0(ctx, base);
	// 8327CADC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327CAE0: 386905C8  addi r3, r9, 0x5c8
	ctx.r[3].s64 = ctx.r[9].s64 + 1480;
	// 8327CAE4: 4BA2D43D  bl 0x82ca9f20
	ctx.lr = 0x8327CAE8;
	sub_82CA9F20(ctx, base);
	// 8327CAE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327CAEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327CAF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327CAF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327CAF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327CAF8 size=64
    let mut pc: u32 = 0x8327CAF8;
    'dispatch: loop {
        match pc {
            0x8327CAF8 => {
    //   block [0x8327CAF8..0x8327CB38)
	// 8327CAF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327CAFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327CB00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327CB04: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327CB08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327CB0C: 388BE5B4  addi r4, r11, -0x1a4c
	ctx.r[4].s64 = ctx.r[11].s64 + -6732;
	// 8327CB10: 386ADB60  addi r3, r10, -0x24a0
	ctx.r[3].s64 = ctx.r[10].s64 + -9376;
	// 8327CB14: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327CB18: 4AFB03B9  bl 0x8222ced0
	ctx.lr = 0x8327CB1C;
	sub_8222CED0(ctx, base);
	// 8327CB1C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327CB20: 386905D8  addi r3, r9, 0x5d8
	ctx.r[3].s64 = ctx.r[9].s64 + 1496;
	// 8327CB24: 4BA2D3FD  bl 0x82ca9f20
	ctx.lr = 0x8327CB28;
	sub_82CA9F20(ctx, base);
	// 8327CB28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327CB2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327CB30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327CB34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327CB38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327CB38 size=64
    let mut pc: u32 = 0x8327CB38;
    'dispatch: loop {
        match pc {
            0x8327CB38 => {
    //   block [0x8327CB38..0x8327CB78)
	// 8327CB38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327CB3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327CB40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327CB44: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327CB48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327CB4C: 388BE5D4  addi r4, r11, -0x1a2c
	ctx.r[4].s64 = ctx.r[11].s64 + -6700;
	// 8327CB50: 386ADB64  addi r3, r10, -0x249c
	ctx.r[3].s64 = ctx.r[10].s64 + -9372;
	// 8327CB54: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327CB58: 4AFB0379  bl 0x8222ced0
	ctx.lr = 0x8327CB5C;
	sub_8222CED0(ctx, base);
	// 8327CB5C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327CB60: 386905E8  addi r3, r9, 0x5e8
	ctx.r[3].s64 = ctx.r[9].s64 + 1512;
	// 8327CB64: 4BA2D3BD  bl 0x82ca9f20
	ctx.lr = 0x8327CB68;
	sub_82CA9F20(ctx, base);
	// 8327CB68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327CB6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327CB70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327CB74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327CB78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327CB78 size=56
    let mut pc: u32 = 0x8327CB78;
    'dispatch: loop {
        match pc {
            0x8327CB78 => {
    //   block [0x8327CB78..0x8327CBB0)
	// 8327CB78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327CB7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327CB80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327CB84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327CB88: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327CB8C: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 8327CB90: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327CB94: 4AF771C5  bl 0x821f3d58
	ctx.lr = 0x8327CB98;
	sub_821F3D58(ctx, base);
	// 8327CB98: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327CB9C: 906ADB68  stw r3, -0x2498(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9368 as u32), ctx.r[3].u32 ) };
	// 8327CBA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327CBA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327CBA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327CBAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327CBB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327CBB0 size=56
    let mut pc: u32 = 0x8327CBB0;
    'dispatch: loop {
        match pc {
            0x8327CBB0 => {
    //   block [0x8327CBB0..0x8327CBE8)
	// 8327CBB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327CBB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327CBB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327CBBC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327CBC0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327CBC4: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 8327CBC8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327CBCC: 4AF7718D  bl 0x821f3d58
	ctx.lr = 0x8327CBD0;
	sub_821F3D58(ctx, base);
	// 8327CBD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327CBD4: 906ADB6C  stw r3, -0x2494(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9364 as u32), ctx.r[3].u32 ) };
	// 8327CBD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327CBDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327CBE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327CBE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327CBE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327CBE8 size=56
    let mut pc: u32 = 0x8327CBE8;
    'dispatch: loop {
        match pc {
            0x8327CBE8 => {
    //   block [0x8327CBE8..0x8327CC20)
	// 8327CBE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327CBEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327CBF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327CBF4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327CBF8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327CBFC: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 8327CC00: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327CC04: 4AF77155  bl 0x821f3d58
	ctx.lr = 0x8327CC08;
	sub_821F3D58(ctx, base);
	// 8327CC08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327CC0C: 906ADB70  stw r3, -0x2490(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9360 as u32), ctx.r[3].u32 ) };
	// 8327CC10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327CC14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327CC18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327CC1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327CC20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327CC20 size=56
    let mut pc: u32 = 0x8327CC20;
    'dispatch: loop {
        match pc {
            0x8327CC20 => {
    //   block [0x8327CC20..0x8327CC58)
	// 8327CC20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327CC24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327CC28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327CC2C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327CC30: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327CC34: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 8327CC38: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327CC3C: 4AF7711D  bl 0x821f3d58
	ctx.lr = 0x8327CC40;
	sub_821F3D58(ctx, base);
	// 8327CC40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327CC44: 906ADB74  stw r3, -0x248c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9356 as u32), ctx.r[3].u32 ) };
	// 8327CC48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327CC4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327CC50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327CC54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327CC58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327CC58 size=56
    let mut pc: u32 = 0x8327CC58;
    'dispatch: loop {
        match pc {
            0x8327CC58 => {
    //   block [0x8327CC58..0x8327CC90)
	// 8327CC58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327CC5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327CC60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327CC64: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327CC68: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327CC6C: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 8327CC70: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327CC74: 4AF770E5  bl 0x821f3d58
	ctx.lr = 0x8327CC78;
	sub_821F3D58(ctx, base);
	// 8327CC78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327CC7C: 906ADB78  stw r3, -0x2488(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9352 as u32), ctx.r[3].u32 ) };
	// 8327CC80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327CC84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327CC88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327CC8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327CC90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327CC90 size=56
    let mut pc: u32 = 0x8327CC90;
    'dispatch: loop {
        match pc {
            0x8327CC90 => {
    //   block [0x8327CC90..0x8327CCC8)
	// 8327CC90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327CC94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327CC98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327CC9C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327CCA0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327CCA4: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 8327CCA8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327CCAC: 4AF770AD  bl 0x821f3d58
	ctx.lr = 0x8327CCB0;
	sub_821F3D58(ctx, base);
	// 8327CCB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327CCB4: 906ADB7C  stw r3, -0x2484(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9348 as u32), ctx.r[3].u32 ) };
	// 8327CCB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327CCBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327CCC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327CCC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327CCC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327CCC8 size=56
    let mut pc: u32 = 0x8327CCC8;
    'dispatch: loop {
        match pc {
            0x8327CCC8 => {
    //   block [0x8327CCC8..0x8327CD00)
	// 8327CCC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327CCCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327CCD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327CCD4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327CCD8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327CCDC: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 8327CCE0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327CCE4: 4AF77075  bl 0x821f3d58
	ctx.lr = 0x8327CCE8;
	sub_821F3D58(ctx, base);
	// 8327CCE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327CCEC: 906ADB80  stw r3, -0x2480(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9344 as u32), ctx.r[3].u32 ) };
	// 8327CCF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327CCF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327CCF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327CCFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327CD00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327CD00 size=56
    let mut pc: u32 = 0x8327CD00;
    'dispatch: loop {
        match pc {
            0x8327CD00 => {
    //   block [0x8327CD00..0x8327CD38)
	// 8327CD00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327CD04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327CD08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327CD0C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327CD10: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327CD14: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 8327CD18: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327CD1C: 4AF7703D  bl 0x821f3d58
	ctx.lr = 0x8327CD20;
	sub_821F3D58(ctx, base);
	// 8327CD20: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327CD24: 906ADB84  stw r3, -0x247c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9340 as u32), ctx.r[3].u32 ) };
	// 8327CD28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327CD2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327CD30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327CD34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327CD38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327CD38 size=56
    let mut pc: u32 = 0x8327CD38;
    'dispatch: loop {
        match pc {
            0x8327CD38 => {
    //   block [0x8327CD38..0x8327CD70)
	// 8327CD38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327CD3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327CD40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327CD44: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327CD48: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327CD4C: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 8327CD50: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327CD54: 4AF77005  bl 0x821f3d58
	ctx.lr = 0x8327CD58;
	sub_821F3D58(ctx, base);
	// 8327CD58: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327CD5C: 906ADB88  stw r3, -0x2478(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9336 as u32), ctx.r[3].u32 ) };
	// 8327CD60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327CD64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327CD68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327CD6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327CD70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327CD70 size=56
    let mut pc: u32 = 0x8327CD70;
    'dispatch: loop {
        match pc {
            0x8327CD70 => {
    //   block [0x8327CD70..0x8327CDA8)
	// 8327CD70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327CD74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327CD78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327CD7C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327CD80: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327CD84: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 8327CD88: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327CD8C: 4AF76FCD  bl 0x821f3d58
	ctx.lr = 0x8327CD90;
	sub_821F3D58(ctx, base);
	// 8327CD90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327CD94: 906ADB8C  stw r3, -0x2474(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9332 as u32), ctx.r[3].u32 ) };
	// 8327CD98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327CD9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327CDA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327CDA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327CDA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327CDA8 size=56
    let mut pc: u32 = 0x8327CDA8;
    'dispatch: loop {
        match pc {
            0x8327CDA8 => {
    //   block [0x8327CDA8..0x8327CDE0)
	// 8327CDA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327CDAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327CDB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327CDB4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327CDB8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327CDBC: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 8327CDC0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327CDC4: 4AF76F95  bl 0x821f3d58
	ctx.lr = 0x8327CDC8;
	sub_821F3D58(ctx, base);
	// 8327CDC8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327CDCC: 906ADB90  stw r3, -0x2470(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9328 as u32), ctx.r[3].u32 ) };
	// 8327CDD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327CDD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327CDD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327CDDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327CDE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327CDE0 size=56
    let mut pc: u32 = 0x8327CDE0;
    'dispatch: loop {
        match pc {
            0x8327CDE0 => {
    //   block [0x8327CDE0..0x8327CE18)
	// 8327CDE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327CDE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327CDE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327CDEC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327CDF0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327CDF4: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 8327CDF8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327CDFC: 4AF76F5D  bl 0x821f3d58
	ctx.lr = 0x8327CE00;
	sub_821F3D58(ctx, base);
	// 8327CE00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327CE04: 906ADB94  stw r3, -0x246c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9324 as u32), ctx.r[3].u32 ) };
	// 8327CE08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327CE0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327CE10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327CE14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327CE18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327CE18 size=56
    let mut pc: u32 = 0x8327CE18;
    'dispatch: loop {
        match pc {
            0x8327CE18 => {
    //   block [0x8327CE18..0x8327CE50)
	// 8327CE18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327CE1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327CE20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327CE24: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327CE28: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327CE2C: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 8327CE30: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327CE34: 4AF76F25  bl 0x821f3d58
	ctx.lr = 0x8327CE38;
	sub_821F3D58(ctx, base);
	// 8327CE38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327CE3C: 906ADB98  stw r3, -0x2468(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9320 as u32), ctx.r[3].u32 ) };
	// 8327CE40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327CE44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327CE48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327CE4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327CE50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327CE50 size=56
    let mut pc: u32 = 0x8327CE50;
    'dispatch: loop {
        match pc {
            0x8327CE50 => {
    //   block [0x8327CE50..0x8327CE88)
	// 8327CE50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327CE54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327CE58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327CE5C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327CE60: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327CE64: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 8327CE68: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327CE6C: 4AF76EED  bl 0x821f3d58
	ctx.lr = 0x8327CE70;
	sub_821F3D58(ctx, base);
	// 8327CE70: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327CE74: 906ADB9C  stw r3, -0x2464(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9316 as u32), ctx.r[3].u32 ) };
	// 8327CE78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327CE7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327CE80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327CE84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327CE88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327CE88 size=56
    let mut pc: u32 = 0x8327CE88;
    'dispatch: loop {
        match pc {
            0x8327CE88 => {
    //   block [0x8327CE88..0x8327CEC0)
	// 8327CE88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327CE8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327CE90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327CE94: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327CE98: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327CE9C: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 8327CEA0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327CEA4: 4AF76EB5  bl 0x821f3d58
	ctx.lr = 0x8327CEA8;
	sub_821F3D58(ctx, base);
	// 8327CEA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327CEAC: 906ADBA0  stw r3, -0x2460(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9312 as u32), ctx.r[3].u32 ) };
	// 8327CEB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327CEB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327CEB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327CEBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327CEC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327CEC0 size=56
    let mut pc: u32 = 0x8327CEC0;
    'dispatch: loop {
        match pc {
            0x8327CEC0 => {
    //   block [0x8327CEC0..0x8327CEF8)
	// 8327CEC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327CEC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327CEC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327CECC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327CED0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327CED4: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 8327CED8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327CEDC: 4AF76E7D  bl 0x821f3d58
	ctx.lr = 0x8327CEE0;
	sub_821F3D58(ctx, base);
	// 8327CEE0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327CEE4: 906ADBA4  stw r3, -0x245c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9308 as u32), ctx.r[3].u32 ) };
	// 8327CEE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327CEEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327CEF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327CEF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327CEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327CEF8 size=56
    let mut pc: u32 = 0x8327CEF8;
    'dispatch: loop {
        match pc {
            0x8327CEF8 => {
    //   block [0x8327CEF8..0x8327CF30)
	// 8327CEF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327CEFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327CF00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327CF04: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327CF08: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327CF0C: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 8327CF10: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327CF14: 4AF76E45  bl 0x821f3d58
	ctx.lr = 0x8327CF18;
	sub_821F3D58(ctx, base);
	// 8327CF18: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327CF1C: 906ADBA8  stw r3, -0x2458(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9304 as u32), ctx.r[3].u32 ) };
	// 8327CF20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327CF24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327CF28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327CF2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327CF30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327CF30 size=56
    let mut pc: u32 = 0x8327CF30;
    'dispatch: loop {
        match pc {
            0x8327CF30 => {
    //   block [0x8327CF30..0x8327CF68)
	// 8327CF30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327CF34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327CF38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327CF3C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327CF40: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327CF44: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 8327CF48: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327CF4C: 4AF76E0D  bl 0x821f3d58
	ctx.lr = 0x8327CF50;
	sub_821F3D58(ctx, base);
	// 8327CF50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327CF54: 906ADBAC  stw r3, -0x2454(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9300 as u32), ctx.r[3].u32 ) };
	// 8327CF58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327CF5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327CF60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327CF64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327CF68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327CF68 size=56
    let mut pc: u32 = 0x8327CF68;
    'dispatch: loop {
        match pc {
            0x8327CF68 => {
    //   block [0x8327CF68..0x8327CFA0)
	// 8327CF68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327CF6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327CF70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327CF74: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327CF78: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327CF7C: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 8327CF80: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327CF84: 4AF76DD5  bl 0x821f3d58
	ctx.lr = 0x8327CF88;
	sub_821F3D58(ctx, base);
	// 8327CF88: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327CF8C: 906ADBB0  stw r3, -0x2450(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9296 as u32), ctx.r[3].u32 ) };
	// 8327CF90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327CF94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327CF98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327CF9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327CFA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327CFA0 size=56
    let mut pc: u32 = 0x8327CFA0;
    'dispatch: loop {
        match pc {
            0x8327CFA0 => {
    //   block [0x8327CFA0..0x8327CFD8)
	// 8327CFA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327CFA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327CFA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327CFAC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327CFB0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327CFB4: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 8327CFB8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327CFBC: 4AF76D9D  bl 0x821f3d58
	ctx.lr = 0x8327CFC0;
	sub_821F3D58(ctx, base);
	// 8327CFC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327CFC4: 906ADBB4  stw r3, -0x244c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9292 as u32), ctx.r[3].u32 ) };
	// 8327CFC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327CFCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327CFD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327CFD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327CFD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327CFD8 size=56
    let mut pc: u32 = 0x8327CFD8;
    'dispatch: loop {
        match pc {
            0x8327CFD8 => {
    //   block [0x8327CFD8..0x8327D010)
	// 8327CFD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327CFDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327CFE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327CFE4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327CFE8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327CFEC: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 8327CFF0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327CFF4: 4AF76D65  bl 0x821f3d58
	ctx.lr = 0x8327CFF8;
	sub_821F3D58(ctx, base);
	// 8327CFF8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327CFFC: 906ADBB8  stw r3, -0x2448(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9288 as u32), ctx.r[3].u32 ) };
	// 8327D000: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D004: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D008: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D00C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D010 size=64
    let mut pc: u32 = 0x8327D010;
    'dispatch: loop {
        match pc {
            0x8327D010 => {
    //   block [0x8327D010..0x8327D050)
	// 8327D010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D018: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D01C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327D020: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D024: 388BE5F8  addi r4, r11, -0x1a08
	ctx.r[4].s64 = ctx.r[11].s64 + -6664;
	// 8327D028: 386ADBBC  addi r3, r10, -0x2444
	ctx.r[3].s64 = ctx.r[10].s64 + -9284;
	// 8327D02C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327D030: 4AFAFEA1  bl 0x8222ced0
	ctx.lr = 0x8327D034;
	sub_8222CED0(ctx, base);
	// 8327D034: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327D038: 386905F8  addi r3, r9, 0x5f8
	ctx.r[3].s64 = ctx.r[9].s64 + 1528;
	// 8327D03C: 4BA2CEE5  bl 0x82ca9f20
	ctx.lr = 0x8327D040;
	sub_82CA9F20(ctx, base);
	// 8327D040: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D044: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D048: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D04C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D050 size=64
    let mut pc: u32 = 0x8327D050;
    'dispatch: loop {
        match pc {
            0x8327D050 => {
    //   block [0x8327D050..0x8327D090)
	// 8327D050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D058: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D05C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327D060: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D064: 388BE638  addi r4, r11, -0x19c8
	ctx.r[4].s64 = ctx.r[11].s64 + -6600;
	// 8327D068: 386ADBC0  addi r3, r10, -0x2440
	ctx.r[3].s64 = ctx.r[10].s64 + -9280;
	// 8327D06C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327D070: 4AFAFE61  bl 0x8222ced0
	ctx.lr = 0x8327D074;
	sub_8222CED0(ctx, base);
	// 8327D074: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327D078: 38690608  addi r3, r9, 0x608
	ctx.r[3].s64 = ctx.r[9].s64 + 1544;
	// 8327D07C: 4BA2CEA5  bl 0x82ca9f20
	ctx.lr = 0x8327D080;
	sub_82CA9F20(ctx, base);
	// 8327D080: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D084: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D088: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D08C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D090 size=64
    let mut pc: u32 = 0x8327D090;
    'dispatch: loop {
        match pc {
            0x8327D090 => {
    //   block [0x8327D090..0x8327D0D0)
	// 8327D090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D098: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D09C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327D0A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D0A4: 388BE648  addi r4, r11, -0x19b8
	ctx.r[4].s64 = ctx.r[11].s64 + -6584;
	// 8327D0A8: 386ADBC4  addi r3, r10, -0x243c
	ctx.r[3].s64 = ctx.r[10].s64 + -9276;
	// 8327D0AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327D0B0: 4AFAFE21  bl 0x8222ced0
	ctx.lr = 0x8327D0B4;
	sub_8222CED0(ctx, base);
	// 8327D0B4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327D0B8: 38690618  addi r3, r9, 0x618
	ctx.r[3].s64 = ctx.r[9].s64 + 1560;
	// 8327D0BC: 4BA2CE65  bl 0x82ca9f20
	ctx.lr = 0x8327D0C0;
	sub_82CA9F20(ctx, base);
	// 8327D0C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D0C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D0C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D0CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D0D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D0D0 size=64
    let mut pc: u32 = 0x8327D0D0;
    'dispatch: loop {
        match pc {
            0x8327D0D0 => {
    //   block [0x8327D0D0..0x8327D110)
	// 8327D0D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D0D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D0D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D0DC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327D0E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D0E4: 388BE690  addi r4, r11, -0x1970
	ctx.r[4].s64 = ctx.r[11].s64 + -6512;
	// 8327D0E8: 386ADBC8  addi r3, r10, -0x2438
	ctx.r[3].s64 = ctx.r[10].s64 + -9272;
	// 8327D0EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327D0F0: 4AFAFDE1  bl 0x8222ced0
	ctx.lr = 0x8327D0F4;
	sub_8222CED0(ctx, base);
	// 8327D0F4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327D0F8: 38690628  addi r3, r9, 0x628
	ctx.r[3].s64 = ctx.r[9].s64 + 1576;
	// 8327D0FC: 4BA2CE25  bl 0x82ca9f20
	ctx.lr = 0x8327D100;
	sub_82CA9F20(ctx, base);
	// 8327D100: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D104: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D108: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D10C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D110 size=64
    let mut pc: u32 = 0x8327D110;
    'dispatch: loop {
        match pc {
            0x8327D110 => {
    //   block [0x8327D110..0x8327D150)
	// 8327D110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D118: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D11C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327D120: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D124: 388BE6D8  addi r4, r11, -0x1928
	ctx.r[4].s64 = ctx.r[11].s64 + -6440;
	// 8327D128: 386ADBCC  addi r3, r10, -0x2434
	ctx.r[3].s64 = ctx.r[10].s64 + -9268;
	// 8327D12C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327D130: 4AFAFDA1  bl 0x8222ced0
	ctx.lr = 0x8327D134;
	sub_8222CED0(ctx, base);
	// 8327D134: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327D138: 38690638  addi r3, r9, 0x638
	ctx.r[3].s64 = ctx.r[9].s64 + 1592;
	// 8327D13C: 4BA2CDE5  bl 0x82ca9f20
	ctx.lr = 0x8327D140;
	sub_82CA9F20(ctx, base);
	// 8327D140: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D144: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D148: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D14C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D150 size=64
    let mut pc: u32 = 0x8327D150;
    'dispatch: loop {
        match pc {
            0x8327D150 => {
    //   block [0x8327D150..0x8327D190)
	// 8327D150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D158: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D15C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327D160: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D164: 388BE728  addi r4, r11, -0x18d8
	ctx.r[4].s64 = ctx.r[11].s64 + -6360;
	// 8327D168: 386ADBD0  addi r3, r10, -0x2430
	ctx.r[3].s64 = ctx.r[10].s64 + -9264;
	// 8327D16C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327D170: 4AFAFD61  bl 0x8222ced0
	ctx.lr = 0x8327D174;
	sub_8222CED0(ctx, base);
	// 8327D174: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327D178: 38690648  addi r3, r9, 0x648
	ctx.r[3].s64 = ctx.r[9].s64 + 1608;
	// 8327D17C: 4BA2CDA5  bl 0x82ca9f20
	ctx.lr = 0x8327D180;
	sub_82CA9F20(ctx, base);
	// 8327D180: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D184: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D188: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D18C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D190 size=64
    let mut pc: u32 = 0x8327D190;
    'dispatch: loop {
        match pc {
            0x8327D190 => {
    //   block [0x8327D190..0x8327D1D0)
	// 8327D190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D198: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D19C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327D1A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D1A4: 388BE770  addi r4, r11, -0x1890
	ctx.r[4].s64 = ctx.r[11].s64 + -6288;
	// 8327D1A8: 386ADBD4  addi r3, r10, -0x242c
	ctx.r[3].s64 = ctx.r[10].s64 + -9260;
	// 8327D1AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327D1B0: 4AFAFD21  bl 0x8222ced0
	ctx.lr = 0x8327D1B4;
	sub_8222CED0(ctx, base);
	// 8327D1B4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327D1B8: 38690658  addi r3, r9, 0x658
	ctx.r[3].s64 = ctx.r[9].s64 + 1624;
	// 8327D1BC: 4BA2CD65  bl 0x82ca9f20
	ctx.lr = 0x8327D1C0;
	sub_82CA9F20(ctx, base);
	// 8327D1C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D1C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D1C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D1CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D1D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D1D0 size=64
    let mut pc: u32 = 0x8327D1D0;
    'dispatch: loop {
        match pc {
            0x8327D1D0 => {
    //   block [0x8327D1D0..0x8327D210)
	// 8327D1D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D1D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D1D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D1DC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327D1E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D1E4: 388BE7C0  addi r4, r11, -0x1840
	ctx.r[4].s64 = ctx.r[11].s64 + -6208;
	// 8327D1E8: 386ADBD8  addi r3, r10, -0x2428
	ctx.r[3].s64 = ctx.r[10].s64 + -9256;
	// 8327D1EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327D1F0: 4AFAFCE1  bl 0x8222ced0
	ctx.lr = 0x8327D1F4;
	sub_8222CED0(ctx, base);
	// 8327D1F4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327D1F8: 38690668  addi r3, r9, 0x668
	ctx.r[3].s64 = ctx.r[9].s64 + 1640;
	// 8327D1FC: 4BA2CD25  bl 0x82ca9f20
	ctx.lr = 0x8327D200;
	sub_82CA9F20(ctx, base);
	// 8327D200: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D204: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D208: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D20C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D210 size=64
    let mut pc: u32 = 0x8327D210;
    'dispatch: loop {
        match pc {
            0x8327D210 => {
    //   block [0x8327D210..0x8327D250)
	// 8327D210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D218: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D21C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327D220: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D224: 388BE810  addi r4, r11, -0x17f0
	ctx.r[4].s64 = ctx.r[11].s64 + -6128;
	// 8327D228: 386ADBDC  addi r3, r10, -0x2424
	ctx.r[3].s64 = ctx.r[10].s64 + -9252;
	// 8327D22C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327D230: 4AFAFCA1  bl 0x8222ced0
	ctx.lr = 0x8327D234;
	sub_8222CED0(ctx, base);
	// 8327D234: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327D238: 38690678  addi r3, r9, 0x678
	ctx.r[3].s64 = ctx.r[9].s64 + 1656;
	// 8327D23C: 4BA2CCE5  bl 0x82ca9f20
	ctx.lr = 0x8327D240;
	sub_82CA9F20(ctx, base);
	// 8327D240: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D244: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D248: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D24C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D250 size=64
    let mut pc: u32 = 0x8327D250;
    'dispatch: loop {
        match pc {
            0x8327D250 => {
    //   block [0x8327D250..0x8327D290)
	// 8327D250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D258: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D25C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327D260: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D264: 388BE850  addi r4, r11, -0x17b0
	ctx.r[4].s64 = ctx.r[11].s64 + -6064;
	// 8327D268: 386ADBE0  addi r3, r10, -0x2420
	ctx.r[3].s64 = ctx.r[10].s64 + -9248;
	// 8327D26C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327D270: 4AFAFC61  bl 0x8222ced0
	ctx.lr = 0x8327D274;
	sub_8222CED0(ctx, base);
	// 8327D274: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327D278: 38690688  addi r3, r9, 0x688
	ctx.r[3].s64 = ctx.r[9].s64 + 1672;
	// 8327D27C: 4BA2CCA5  bl 0x82ca9f20
	ctx.lr = 0x8327D280;
	sub_82CA9F20(ctx, base);
	// 8327D280: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D284: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D288: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D28C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D290 size=64
    let mut pc: u32 = 0x8327D290;
    'dispatch: loop {
        match pc {
            0x8327D290 => {
    //   block [0x8327D290..0x8327D2D0)
	// 8327D290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D298: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D29C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327D2A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D2A4: 388BE898  addi r4, r11, -0x1768
	ctx.r[4].s64 = ctx.r[11].s64 + -5992;
	// 8327D2A8: 386ADBE4  addi r3, r10, -0x241c
	ctx.r[3].s64 = ctx.r[10].s64 + -9244;
	// 8327D2AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327D2B0: 4AFAFC21  bl 0x8222ced0
	ctx.lr = 0x8327D2B4;
	sub_8222CED0(ctx, base);
	// 8327D2B4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327D2B8: 38690698  addi r3, r9, 0x698
	ctx.r[3].s64 = ctx.r[9].s64 + 1688;
	// 8327D2BC: 4BA2CC65  bl 0x82ca9f20
	ctx.lr = 0x8327D2C0;
	sub_82CA9F20(ctx, base);
	// 8327D2C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D2C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D2C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D2CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D2D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D2D0 size=64
    let mut pc: u32 = 0x8327D2D0;
    'dispatch: loop {
        match pc {
            0x8327D2D0 => {
    //   block [0x8327D2D0..0x8327D310)
	// 8327D2D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D2D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D2D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D2DC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327D2E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D2E4: 388BE8E0  addi r4, r11, -0x1720
	ctx.r[4].s64 = ctx.r[11].s64 + -5920;
	// 8327D2E8: 386ADBE8  addi r3, r10, -0x2418
	ctx.r[3].s64 = ctx.r[10].s64 + -9240;
	// 8327D2EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327D2F0: 4AFAFBE1  bl 0x8222ced0
	ctx.lr = 0x8327D2F4;
	sub_8222CED0(ctx, base);
	// 8327D2F4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327D2F8: 386906A8  addi r3, r9, 0x6a8
	ctx.r[3].s64 = ctx.r[9].s64 + 1704;
	// 8327D2FC: 4BA2CC25  bl 0x82ca9f20
	ctx.lr = 0x8327D300;
	sub_82CA9F20(ctx, base);
	// 8327D300: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D304: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D308: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D30C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D310 size=64
    let mut pc: u32 = 0x8327D310;
    'dispatch: loop {
        match pc {
            0x8327D310 => {
    //   block [0x8327D310..0x8327D350)
	// 8327D310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D318: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D31C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327D320: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D324: 388BE928  addi r4, r11, -0x16d8
	ctx.r[4].s64 = ctx.r[11].s64 + -5848;
	// 8327D328: 386ADBEC  addi r3, r10, -0x2414
	ctx.r[3].s64 = ctx.r[10].s64 + -9236;
	// 8327D32C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327D330: 4AFAFBA1  bl 0x8222ced0
	ctx.lr = 0x8327D334;
	sub_8222CED0(ctx, base);
	// 8327D334: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327D338: 386906B8  addi r3, r9, 0x6b8
	ctx.r[3].s64 = ctx.r[9].s64 + 1720;
	// 8327D33C: 4BA2CBE5  bl 0x82ca9f20
	ctx.lr = 0x8327D340;
	sub_82CA9F20(ctx, base);
	// 8327D340: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D344: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D348: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D34C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D350 size=64
    let mut pc: u32 = 0x8327D350;
    'dispatch: loop {
        match pc {
            0x8327D350 => {
    //   block [0x8327D350..0x8327D390)
	// 8327D350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D358: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D35C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327D360: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D364: 388BE970  addi r4, r11, -0x1690
	ctx.r[4].s64 = ctx.r[11].s64 + -5776;
	// 8327D368: 386ADBF0  addi r3, r10, -0x2410
	ctx.r[3].s64 = ctx.r[10].s64 + -9232;
	// 8327D36C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327D370: 4AFAFB61  bl 0x8222ced0
	ctx.lr = 0x8327D374;
	sub_8222CED0(ctx, base);
	// 8327D374: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327D378: 386906C8  addi r3, r9, 0x6c8
	ctx.r[3].s64 = ctx.r[9].s64 + 1736;
	// 8327D37C: 4BA2CBA5  bl 0x82ca9f20
	ctx.lr = 0x8327D380;
	sub_82CA9F20(ctx, base);
	// 8327D380: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D384: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D388: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D38C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D390 size=64
    let mut pc: u32 = 0x8327D390;
    'dispatch: loop {
        match pc {
            0x8327D390 => {
    //   block [0x8327D390..0x8327D3D0)
	// 8327D390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D398: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D39C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327D3A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D3A4: 388BE9B8  addi r4, r11, -0x1648
	ctx.r[4].s64 = ctx.r[11].s64 + -5704;
	// 8327D3A8: 386ADBF4  addi r3, r10, -0x240c
	ctx.r[3].s64 = ctx.r[10].s64 + -9228;
	// 8327D3AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327D3B0: 4AFAFB21  bl 0x8222ced0
	ctx.lr = 0x8327D3B4;
	sub_8222CED0(ctx, base);
	// 8327D3B4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327D3B8: 386906D8  addi r3, r9, 0x6d8
	ctx.r[3].s64 = ctx.r[9].s64 + 1752;
	// 8327D3BC: 4BA2CB65  bl 0x82ca9f20
	ctx.lr = 0x8327D3C0;
	sub_82CA9F20(ctx, base);
	// 8327D3C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D3C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D3C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D3CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D3D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D3D0 size=56
    let mut pc: u32 = 0x8327D3D0;
    'dispatch: loop {
        match pc {
            0x8327D3D0 => {
    //   block [0x8327D3D0..0x8327D408)
	// 8327D3D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D3D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D3D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D3DC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327D3E0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327D3E4: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 8327D3E8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327D3EC: 4AF7696D  bl 0x821f3d58
	ctx.lr = 0x8327D3F0;
	sub_821F3D58(ctx, base);
	// 8327D3F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D3F4: 906ADBF8  stw r3, -0x2408(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9224 as u32), ctx.r[3].u32 ) };
	// 8327D3F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D3FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D400: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D404: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D408 size=56
    let mut pc: u32 = 0x8327D408;
    'dispatch: loop {
        match pc {
            0x8327D408 => {
    //   block [0x8327D408..0x8327D440)
	// 8327D408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D40C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D410: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D414: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327D418: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327D41C: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 8327D420: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327D424: 4AF76935  bl 0x821f3d58
	ctx.lr = 0x8327D428;
	sub_821F3D58(ctx, base);
	// 8327D428: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D42C: 906ADBFC  stw r3, -0x2404(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9220 as u32), ctx.r[3].u32 ) };
	// 8327D430: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D434: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D438: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D43C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D440 size=56
    let mut pc: u32 = 0x8327D440;
    'dispatch: loop {
        match pc {
            0x8327D440 => {
    //   block [0x8327D440..0x8327D478)
	// 8327D440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D448: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D44C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327D450: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327D454: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 8327D458: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327D45C: 4AF768FD  bl 0x821f3d58
	ctx.lr = 0x8327D460;
	sub_821F3D58(ctx, base);
	// 8327D460: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D464: 906ADC00  stw r3, -0x2400(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9216 as u32), ctx.r[3].u32 ) };
	// 8327D468: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D46C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D470: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D474: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D478 size=56
    let mut pc: u32 = 0x8327D478;
    'dispatch: loop {
        match pc {
            0x8327D478 => {
    //   block [0x8327D478..0x8327D4B0)
	// 8327D478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D47C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D480: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D484: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327D488: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327D48C: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 8327D490: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327D494: 4AF768C5  bl 0x821f3d58
	ctx.lr = 0x8327D498;
	sub_821F3D58(ctx, base);
	// 8327D498: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D49C: 906ADC04  stw r3, -0x23fc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9212 as u32), ctx.r[3].u32 ) };
	// 8327D4A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D4A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D4A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D4AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D4B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D4B0 size=56
    let mut pc: u32 = 0x8327D4B0;
    'dispatch: loop {
        match pc {
            0x8327D4B0 => {
    //   block [0x8327D4B0..0x8327D4E8)
	// 8327D4B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D4B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D4B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D4BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327D4C0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327D4C4: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 8327D4C8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327D4CC: 4AF7688D  bl 0x821f3d58
	ctx.lr = 0x8327D4D0;
	sub_821F3D58(ctx, base);
	// 8327D4D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D4D4: 906ADC08  stw r3, -0x23f8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9208 as u32), ctx.r[3].u32 ) };
	// 8327D4D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D4DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D4E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D4E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D4E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D4E8 size=56
    let mut pc: u32 = 0x8327D4E8;
    'dispatch: loop {
        match pc {
            0x8327D4E8 => {
    //   block [0x8327D4E8..0x8327D520)
	// 8327D4E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D4EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D4F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D4F4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327D4F8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327D4FC: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 8327D500: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327D504: 4AF76855  bl 0x821f3d58
	ctx.lr = 0x8327D508;
	sub_821F3D58(ctx, base);
	// 8327D508: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D50C: 906ADC0C  stw r3, -0x23f4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9204 as u32), ctx.r[3].u32 ) };
	// 8327D510: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D514: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D518: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D51C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D520 size=56
    let mut pc: u32 = 0x8327D520;
    'dispatch: loop {
        match pc {
            0x8327D520 => {
    //   block [0x8327D520..0x8327D558)
	// 8327D520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D528: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D52C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327D530: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327D534: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 8327D538: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327D53C: 4AF7681D  bl 0x821f3d58
	ctx.lr = 0x8327D540;
	sub_821F3D58(ctx, base);
	// 8327D540: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D544: 906ADC10  stw r3, -0x23f0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9200 as u32), ctx.r[3].u32 ) };
	// 8327D548: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D54C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D550: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D554: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D558 size=56
    let mut pc: u32 = 0x8327D558;
    'dispatch: loop {
        match pc {
            0x8327D558 => {
    //   block [0x8327D558..0x8327D590)
	// 8327D558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D55C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D560: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D564: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327D568: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327D56C: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 8327D570: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327D574: 4AF767E5  bl 0x821f3d58
	ctx.lr = 0x8327D578;
	sub_821F3D58(ctx, base);
	// 8327D578: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D57C: 906ADC14  stw r3, -0x23ec(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9196 as u32), ctx.r[3].u32 ) };
	// 8327D580: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D584: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D588: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D58C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D590 size=56
    let mut pc: u32 = 0x8327D590;
    'dispatch: loop {
        match pc {
            0x8327D590 => {
    //   block [0x8327D590..0x8327D5C8)
	// 8327D590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D598: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D59C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327D5A0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327D5A4: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 8327D5A8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327D5AC: 4AF767AD  bl 0x821f3d58
	ctx.lr = 0x8327D5B0;
	sub_821F3D58(ctx, base);
	// 8327D5B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D5B4: 906ADC18  stw r3, -0x23e8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9192 as u32), ctx.r[3].u32 ) };
	// 8327D5B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D5BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D5C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D5C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D5C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D5C8 size=56
    let mut pc: u32 = 0x8327D5C8;
    'dispatch: loop {
        match pc {
            0x8327D5C8 => {
    //   block [0x8327D5C8..0x8327D600)
	// 8327D5C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D5CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D5D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D5D4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327D5D8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327D5DC: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 8327D5E0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327D5E4: 4AF76775  bl 0x821f3d58
	ctx.lr = 0x8327D5E8;
	sub_821F3D58(ctx, base);
	// 8327D5E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D5EC: 906ADC1C  stw r3, -0x23e4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9188 as u32), ctx.r[3].u32 ) };
	// 8327D5F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D5F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D5F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D5FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D600 size=56
    let mut pc: u32 = 0x8327D600;
    'dispatch: loop {
        match pc {
            0x8327D600 => {
    //   block [0x8327D600..0x8327D638)
	// 8327D600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D604: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D608: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D60C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327D610: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327D614: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 8327D618: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327D61C: 4AF7673D  bl 0x821f3d58
	ctx.lr = 0x8327D620;
	sub_821F3D58(ctx, base);
	// 8327D620: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D624: 906ADC20  stw r3, -0x23e0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9184 as u32), ctx.r[3].u32 ) };
	// 8327D628: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D62C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D630: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D634: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D638 size=56
    let mut pc: u32 = 0x8327D638;
    'dispatch: loop {
        match pc {
            0x8327D638 => {
    //   block [0x8327D638..0x8327D670)
	// 8327D638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D63C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D640: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D644: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327D648: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327D64C: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 8327D650: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327D654: 4AF76705  bl 0x821f3d58
	ctx.lr = 0x8327D658;
	sub_821F3D58(ctx, base);
	// 8327D658: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D65C: 906ADC24  stw r3, -0x23dc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9180 as u32), ctx.r[3].u32 ) };
	// 8327D660: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D664: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D668: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D66C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D670 size=56
    let mut pc: u32 = 0x8327D670;
    'dispatch: loop {
        match pc {
            0x8327D670 => {
    //   block [0x8327D670..0x8327D6A8)
	// 8327D670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D678: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D67C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327D680: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327D684: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 8327D688: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327D68C: 4AF766CD  bl 0x821f3d58
	ctx.lr = 0x8327D690;
	sub_821F3D58(ctx, base);
	// 8327D690: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D694: 906ADC28  stw r3, -0x23d8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9176 as u32), ctx.r[3].u32 ) };
	// 8327D698: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D69C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D6A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D6A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D6A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D6A8 size=56
    let mut pc: u32 = 0x8327D6A8;
    'dispatch: loop {
        match pc {
            0x8327D6A8 => {
    //   block [0x8327D6A8..0x8327D6E0)
	// 8327D6A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D6AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D6B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D6B4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327D6B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327D6BC: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 8327D6C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327D6C4: 4AF76695  bl 0x821f3d58
	ctx.lr = 0x8327D6C8;
	sub_821F3D58(ctx, base);
	// 8327D6C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D6CC: 906ADC2C  stw r3, -0x23d4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9172 as u32), ctx.r[3].u32 ) };
	// 8327D6D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D6D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D6D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D6DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D6E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D6E0 size=56
    let mut pc: u32 = 0x8327D6E0;
    'dispatch: loop {
        match pc {
            0x8327D6E0 => {
    //   block [0x8327D6E0..0x8327D718)
	// 8327D6E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D6E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D6E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D6EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327D6F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327D6F4: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 8327D6F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327D6FC: 4AF7665D  bl 0x821f3d58
	ctx.lr = 0x8327D700;
	sub_821F3D58(ctx, base);
	// 8327D700: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D704: 906ADC30  stw r3, -0x23d0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9168 as u32), ctx.r[3].u32 ) };
	// 8327D708: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D70C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D710: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D714: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D718 size=56
    let mut pc: u32 = 0x8327D718;
    'dispatch: loop {
        match pc {
            0x8327D718 => {
    //   block [0x8327D718..0x8327D750)
	// 8327D718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D71C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D720: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D724: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327D728: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327D72C: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 8327D730: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327D734: 4AF76625  bl 0x821f3d58
	ctx.lr = 0x8327D738;
	sub_821F3D58(ctx, base);
	// 8327D738: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D73C: 906ADC34  stw r3, -0x23cc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9164 as u32), ctx.r[3].u32 ) };
	// 8327D740: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D744: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D748: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D74C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D750 size=56
    let mut pc: u32 = 0x8327D750;
    'dispatch: loop {
        match pc {
            0x8327D750 => {
    //   block [0x8327D750..0x8327D788)
	// 8327D750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D758: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D75C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327D760: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327D764: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 8327D768: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327D76C: 4AF765ED  bl 0x821f3d58
	ctx.lr = 0x8327D770;
	sub_821F3D58(ctx, base);
	// 8327D770: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D774: 906ADC38  stw r3, -0x23c8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9160 as u32), ctx.r[3].u32 ) };
	// 8327D778: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D77C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D780: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D784: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D788 size=56
    let mut pc: u32 = 0x8327D788;
    'dispatch: loop {
        match pc {
            0x8327D788 => {
    //   block [0x8327D788..0x8327D7C0)
	// 8327D788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D78C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D790: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D794: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327D798: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327D79C: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 8327D7A0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327D7A4: 4AF765B5  bl 0x821f3d58
	ctx.lr = 0x8327D7A8;
	sub_821F3D58(ctx, base);
	// 8327D7A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D7AC: 906ADC3C  stw r3, -0x23c4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9156 as u32), ctx.r[3].u32 ) };
	// 8327D7B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D7B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D7B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D7BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D7C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D7C0 size=56
    let mut pc: u32 = 0x8327D7C0;
    'dispatch: loop {
        match pc {
            0x8327D7C0 => {
    //   block [0x8327D7C0..0x8327D7F8)
	// 8327D7C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D7C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D7C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D7CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327D7D0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327D7D4: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 8327D7D8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327D7DC: 4AF7657D  bl 0x821f3d58
	ctx.lr = 0x8327D7E0;
	sub_821F3D58(ctx, base);
	// 8327D7E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D7E4: 906ADC40  stw r3, -0x23c0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9152 as u32), ctx.r[3].u32 ) };
	// 8327D7E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D7EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D7F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D7F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D7F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D7F8 size=56
    let mut pc: u32 = 0x8327D7F8;
    'dispatch: loop {
        match pc {
            0x8327D7F8 => {
    //   block [0x8327D7F8..0x8327D830)
	// 8327D7F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D7FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D800: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D804: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327D808: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327D80C: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 8327D810: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327D814: 4AF76545  bl 0x821f3d58
	ctx.lr = 0x8327D818;
	sub_821F3D58(ctx, base);
	// 8327D818: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D81C: 906ADC44  stw r3, -0x23bc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9148 as u32), ctx.r[3].u32 ) };
	// 8327D820: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D824: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D828: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D82C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D830 size=56
    let mut pc: u32 = 0x8327D830;
    'dispatch: loop {
        match pc {
            0x8327D830 => {
    //   block [0x8327D830..0x8327D868)
	// 8327D830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D838: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D83C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327D840: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327D844: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 8327D848: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327D84C: 4AF7650D  bl 0x821f3d58
	ctx.lr = 0x8327D850;
	sub_821F3D58(ctx, base);
	// 8327D850: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D854: 906ADC48  stw r3, -0x23b8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9144 as u32), ctx.r[3].u32 ) };
	// 8327D858: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D85C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D860: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D864: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D868 size=64
    let mut pc: u32 = 0x8327D868;
    'dispatch: loop {
        match pc {
            0x8327D868 => {
    //   block [0x8327D868..0x8327D8A8)
	// 8327D868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D86C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D870: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D874: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327D878: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D87C: 388BE638  addi r4, r11, -0x19c8
	ctx.r[4].s64 = ctx.r[11].s64 + -6600;
	// 8327D880: 386ADC4C  addi r3, r10, -0x23b4
	ctx.r[3].s64 = ctx.r[10].s64 + -9140;
	// 8327D884: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327D888: 4AFAF649  bl 0x8222ced0
	ctx.lr = 0x8327D88C;
	sub_8222CED0(ctx, base);
	// 8327D88C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327D890: 386906E8  addi r3, r9, 0x6e8
	ctx.r[3].s64 = ctx.r[9].s64 + 1768;
	// 8327D894: 4BA2C68D  bl 0x82ca9f20
	ctx.lr = 0x8327D898;
	sub_82CA9F20(ctx, base);
	// 8327D898: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D89C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D8A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D8A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D8A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D8A8 size=64
    let mut pc: u32 = 0x8327D8A8;
    'dispatch: loop {
        match pc {
            0x8327D8A8 => {
    //   block [0x8327D8A8..0x8327D8E8)
	// 8327D8A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D8AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D8B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D8B4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327D8B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D8BC: 388BEB38  addi r4, r11, -0x14c8
	ctx.r[4].s64 = ctx.r[11].s64 + -5320;
	// 8327D8C0: 386ADC50  addi r3, r10, -0x23b0
	ctx.r[3].s64 = ctx.r[10].s64 + -9136;
	// 8327D8C4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327D8C8: 4AFAF609  bl 0x8222ced0
	ctx.lr = 0x8327D8CC;
	sub_8222CED0(ctx, base);
	// 8327D8CC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327D8D0: 386906F8  addi r3, r9, 0x6f8
	ctx.r[3].s64 = ctx.r[9].s64 + 1784;
	// 8327D8D4: 4BA2C64D  bl 0x82ca9f20
	ctx.lr = 0x8327D8D8;
	sub_82CA9F20(ctx, base);
	// 8327D8D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D8DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D8E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D8E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D8E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D8E8 size=64
    let mut pc: u32 = 0x8327D8E8;
    'dispatch: loop {
        match pc {
            0x8327D8E8 => {
    //   block [0x8327D8E8..0x8327D928)
	// 8327D8E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D8EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D8F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D8F4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327D8F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D8FC: 388BEB70  addi r4, r11, -0x1490
	ctx.r[4].s64 = ctx.r[11].s64 + -5264;
	// 8327D900: 386ADC54  addi r3, r10, -0x23ac
	ctx.r[3].s64 = ctx.r[10].s64 + -9132;
	// 8327D904: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327D908: 4AFAF5C9  bl 0x8222ced0
	ctx.lr = 0x8327D90C;
	sub_8222CED0(ctx, base);
	// 8327D90C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327D910: 38690708  addi r3, r9, 0x708
	ctx.r[3].s64 = ctx.r[9].s64 + 1800;
	// 8327D914: 4BA2C60D  bl 0x82ca9f20
	ctx.lr = 0x8327D918;
	sub_82CA9F20(ctx, base);
	// 8327D918: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D91C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D920: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D924: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D928 size=64
    let mut pc: u32 = 0x8327D928;
    'dispatch: loop {
        match pc {
            0x8327D928 => {
    //   block [0x8327D928..0x8327D968)
	// 8327D928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D92C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D930: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D934: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327D938: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D93C: 388BEBAC  addi r4, r11, -0x1454
	ctx.r[4].s64 = ctx.r[11].s64 + -5204;
	// 8327D940: 386ADC58  addi r3, r10, -0x23a8
	ctx.r[3].s64 = ctx.r[10].s64 + -9128;
	// 8327D944: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327D948: 4AFAF589  bl 0x8222ced0
	ctx.lr = 0x8327D94C;
	sub_8222CED0(ctx, base);
	// 8327D94C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327D950: 38690718  addi r3, r9, 0x718
	ctx.r[3].s64 = ctx.r[9].s64 + 1816;
	// 8327D954: 4BA2C5CD  bl 0x82ca9f20
	ctx.lr = 0x8327D958;
	sub_82CA9F20(ctx, base);
	// 8327D958: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D95C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D960: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D964: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D968 size=64
    let mut pc: u32 = 0x8327D968;
    'dispatch: loop {
        match pc {
            0x8327D968 => {
    //   block [0x8327D968..0x8327D9A8)
	// 8327D968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D96C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D970: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D974: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327D978: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D97C: 388BEBC0  addi r4, r11, -0x1440
	ctx.r[4].s64 = ctx.r[11].s64 + -5184;
	// 8327D980: 386ADC5C  addi r3, r10, -0x23a4
	ctx.r[3].s64 = ctx.r[10].s64 + -9124;
	// 8327D984: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327D988: 4AFAF549  bl 0x8222ced0
	ctx.lr = 0x8327D98C;
	sub_8222CED0(ctx, base);
	// 8327D98C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327D990: 38690728  addi r3, r9, 0x728
	ctx.r[3].s64 = ctx.r[9].s64 + 1832;
	// 8327D994: 4BA2C58D  bl 0x82ca9f20
	ctx.lr = 0x8327D998;
	sub_82CA9F20(ctx, base);
	// 8327D998: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D99C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D9A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D9A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D9A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D9A8 size=64
    let mut pc: u32 = 0x8327D9A8;
    'dispatch: loop {
        match pc {
            0x8327D9A8 => {
    //   block [0x8327D9A8..0x8327D9E8)
	// 8327D9A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D9AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D9B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D9B4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327D9B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D9BC: 388BEBD0  addi r4, r11, -0x1430
	ctx.r[4].s64 = ctx.r[11].s64 + -5168;
	// 8327D9C0: 386ADC60  addi r3, r10, -0x23a0
	ctx.r[3].s64 = ctx.r[10].s64 + -9120;
	// 8327D9C4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327D9C8: 4AFAF509  bl 0x8222ced0
	ctx.lr = 0x8327D9CC;
	sub_8222CED0(ctx, base);
	// 8327D9CC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327D9D0: 38690738  addi r3, r9, 0x738
	ctx.r[3].s64 = ctx.r[9].s64 + 1848;
	// 8327D9D4: 4BA2C54D  bl 0x82ca9f20
	ctx.lr = 0x8327D9D8;
	sub_82CA9F20(ctx, base);
	// 8327D9D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D9DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D9E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D9E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D9E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D9E8 size=64
    let mut pc: u32 = 0x8327D9E8;
    'dispatch: loop {
        match pc {
            0x8327D9E8 => {
    //   block [0x8327D9E8..0x8327DA28)
	// 8327D9E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D9EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D9F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D9F4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327D9F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D9FC: 388BEBE0  addi r4, r11, -0x1420
	ctx.r[4].s64 = ctx.r[11].s64 + -5152;
	// 8327DA00: 386ADC64  addi r3, r10, -0x239c
	ctx.r[3].s64 = ctx.r[10].s64 + -9116;
	// 8327DA04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327DA08: 4AFAF4C9  bl 0x8222ced0
	ctx.lr = 0x8327DA0C;
	sub_8222CED0(ctx, base);
	// 8327DA0C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327DA10: 38690748  addi r3, r9, 0x748
	ctx.r[3].s64 = ctx.r[9].s64 + 1864;
	// 8327DA14: 4BA2C50D  bl 0x82ca9f20
	ctx.lr = 0x8327DA18;
	sub_82CA9F20(ctx, base);
	// 8327DA18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327DA1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327DA20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327DA24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327DA28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327DA28 size=64
    let mut pc: u32 = 0x8327DA28;
    'dispatch: loop {
        match pc {
            0x8327DA28 => {
    //   block [0x8327DA28..0x8327DA68)
	// 8327DA28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327DA2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327DA30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327DA34: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327DA38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327DA3C: 388BEBF0  addi r4, r11, -0x1410
	ctx.r[4].s64 = ctx.r[11].s64 + -5136;
	// 8327DA40: 386ADC68  addi r3, r10, -0x2398
	ctx.r[3].s64 = ctx.r[10].s64 + -9112;
	// 8327DA44: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327DA48: 4AFAF489  bl 0x8222ced0
	ctx.lr = 0x8327DA4C;
	sub_8222CED0(ctx, base);
	// 8327DA4C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327DA50: 38690758  addi r3, r9, 0x758
	ctx.r[3].s64 = ctx.r[9].s64 + 1880;
	// 8327DA54: 4BA2C4CD  bl 0x82ca9f20
	ctx.lr = 0x8327DA58;
	sub_82CA9F20(ctx, base);
	// 8327DA58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327DA5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327DA60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327DA64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327DA68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327DA68 size=64
    let mut pc: u32 = 0x8327DA68;
    'dispatch: loop {
        match pc {
            0x8327DA68 => {
    //   block [0x8327DA68..0x8327DAA8)
	// 8327DA68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327DA6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327DA70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327DA74: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327DA78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327DA7C: 388BEC00  addi r4, r11, -0x1400
	ctx.r[4].s64 = ctx.r[11].s64 + -5120;
	// 8327DA80: 386ADC6C  addi r3, r10, -0x2394
	ctx.r[3].s64 = ctx.r[10].s64 + -9108;
	// 8327DA84: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327DA88: 4AFAF449  bl 0x8222ced0
	ctx.lr = 0x8327DA8C;
	sub_8222CED0(ctx, base);
	// 8327DA8C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327DA90: 38690768  addi r3, r9, 0x768
	ctx.r[3].s64 = ctx.r[9].s64 + 1896;
	// 8327DA94: 4BA2C48D  bl 0x82ca9f20
	ctx.lr = 0x8327DA98;
	sub_82CA9F20(ctx, base);
	// 8327DA98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327DA9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327DAA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327DAA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327DAA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327DAA8 size=64
    let mut pc: u32 = 0x8327DAA8;
    'dispatch: loop {
        match pc {
            0x8327DAA8 => {
    //   block [0x8327DAA8..0x8327DAE8)
	// 8327DAA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327DAAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327DAB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327DAB4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327DAB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327DABC: 388BEC10  addi r4, r11, -0x13f0
	ctx.r[4].s64 = ctx.r[11].s64 + -5104;
	// 8327DAC0: 386ADC70  addi r3, r10, -0x2390
	ctx.r[3].s64 = ctx.r[10].s64 + -9104;
	// 8327DAC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327DAC8: 4AFAF409  bl 0x8222ced0
	ctx.lr = 0x8327DACC;
	sub_8222CED0(ctx, base);
	// 8327DACC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327DAD0: 38690778  addi r3, r9, 0x778
	ctx.r[3].s64 = ctx.r[9].s64 + 1912;
	// 8327DAD4: 4BA2C44D  bl 0x82ca9f20
	ctx.lr = 0x8327DAD8;
	sub_82CA9F20(ctx, base);
	// 8327DAD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327DADC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327DAE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327DAE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327DAE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327DAE8 size=64
    let mut pc: u32 = 0x8327DAE8;
    'dispatch: loop {
        match pc {
            0x8327DAE8 => {
    //   block [0x8327DAE8..0x8327DB28)
	// 8327DAE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327DAEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327DAF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327DAF4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327DAF8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327DAFC: 388BEC20  addi r4, r11, -0x13e0
	ctx.r[4].s64 = ctx.r[11].s64 + -5088;
	// 8327DB00: 386ADC74  addi r3, r10, -0x238c
	ctx.r[3].s64 = ctx.r[10].s64 + -9100;
	// 8327DB04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327DB08: 4AFAF3C9  bl 0x8222ced0
	ctx.lr = 0x8327DB0C;
	sub_8222CED0(ctx, base);
	// 8327DB0C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327DB10: 38690788  addi r3, r9, 0x788
	ctx.r[3].s64 = ctx.r[9].s64 + 1928;
	// 8327DB14: 4BA2C40D  bl 0x82ca9f20
	ctx.lr = 0x8327DB18;
	sub_82CA9F20(ctx, base);
	// 8327DB18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327DB1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327DB20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327DB24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327DB28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327DB28 size=64
    let mut pc: u32 = 0x8327DB28;
    'dispatch: loop {
        match pc {
            0x8327DB28 => {
    //   block [0x8327DB28..0x8327DB68)
	// 8327DB28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327DB2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327DB30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327DB34: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327DB38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327DB3C: 388BEC2C  addi r4, r11, -0x13d4
	ctx.r[4].s64 = ctx.r[11].s64 + -5076;
	// 8327DB40: 386ADC78  addi r3, r10, -0x2388
	ctx.r[3].s64 = ctx.r[10].s64 + -9096;
	// 8327DB44: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327DB48: 4AFAF389  bl 0x8222ced0
	ctx.lr = 0x8327DB4C;
	sub_8222CED0(ctx, base);
	// 8327DB4C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327DB50: 38690798  addi r3, r9, 0x798
	ctx.r[3].s64 = ctx.r[9].s64 + 1944;
	// 8327DB54: 4BA2C3CD  bl 0x82ca9f20
	ctx.lr = 0x8327DB58;
	sub_82CA9F20(ctx, base);
	// 8327DB58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327DB5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327DB60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327DB64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327DB68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327DB68 size=64
    let mut pc: u32 = 0x8327DB68;
    'dispatch: loop {
        match pc {
            0x8327DB68 => {
    //   block [0x8327DB68..0x8327DBA8)
	// 8327DB68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327DB6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327DB70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327DB74: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327DB78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327DB7C: 388BEC38  addi r4, r11, -0x13c8
	ctx.r[4].s64 = ctx.r[11].s64 + -5064;
	// 8327DB80: 386ADC7C  addi r3, r10, -0x2384
	ctx.r[3].s64 = ctx.r[10].s64 + -9092;
	// 8327DB84: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327DB88: 4AFAF349  bl 0x8222ced0
	ctx.lr = 0x8327DB8C;
	sub_8222CED0(ctx, base);
	// 8327DB8C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327DB90: 386907A8  addi r3, r9, 0x7a8
	ctx.r[3].s64 = ctx.r[9].s64 + 1960;
	// 8327DB94: 4BA2C38D  bl 0x82ca9f20
	ctx.lr = 0x8327DB98;
	sub_82CA9F20(ctx, base);
	// 8327DB98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327DB9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327DBA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327DBA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327DBA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327DBA8 size=64
    let mut pc: u32 = 0x8327DBA8;
    'dispatch: loop {
        match pc {
            0x8327DBA8 => {
    //   block [0x8327DBA8..0x8327DBE8)
	// 8327DBA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327DBAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327DBB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327DBB4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327DBB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327DBBC: 388BEC48  addi r4, r11, -0x13b8
	ctx.r[4].s64 = ctx.r[11].s64 + -5048;
	// 8327DBC0: 386ADC80  addi r3, r10, -0x2380
	ctx.r[3].s64 = ctx.r[10].s64 + -9088;
	// 8327DBC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327DBC8: 4AFAF309  bl 0x8222ced0
	ctx.lr = 0x8327DBCC;
	sub_8222CED0(ctx, base);
	// 8327DBCC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327DBD0: 386907B8  addi r3, r9, 0x7b8
	ctx.r[3].s64 = ctx.r[9].s64 + 1976;
	// 8327DBD4: 4BA2C34D  bl 0x82ca9f20
	ctx.lr = 0x8327DBD8;
	sub_82CA9F20(ctx, base);
	// 8327DBD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327DBDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327DBE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327DBE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327DBE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327DBE8 size=64
    let mut pc: u32 = 0x8327DBE8;
    'dispatch: loop {
        match pc {
            0x8327DBE8 => {
    //   block [0x8327DBE8..0x8327DC28)
	// 8327DBE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327DBEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327DBF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327DBF4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327DBF8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327DBFC: 388BEC58  addi r4, r11, -0x13a8
	ctx.r[4].s64 = ctx.r[11].s64 + -5032;
	// 8327DC00: 386ADC84  addi r3, r10, -0x237c
	ctx.r[3].s64 = ctx.r[10].s64 + -9084;
	// 8327DC04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327DC08: 4AFAF2C9  bl 0x8222ced0
	ctx.lr = 0x8327DC0C;
	sub_8222CED0(ctx, base);
	// 8327DC0C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327DC10: 386907C8  addi r3, r9, 0x7c8
	ctx.r[3].s64 = ctx.r[9].s64 + 1992;
	// 8327DC14: 4BA2C30D  bl 0x82ca9f20
	ctx.lr = 0x8327DC18;
	sub_82CA9F20(ctx, base);
	// 8327DC18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327DC1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327DC20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327DC24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327DC28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327DC28 size=64
    let mut pc: u32 = 0x8327DC28;
    'dispatch: loop {
        match pc {
            0x8327DC28 => {
    //   block [0x8327DC28..0x8327DC68)
	// 8327DC28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327DC2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327DC30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327DC34: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327DC38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327DC3C: 388BEC68  addi r4, r11, -0x1398
	ctx.r[4].s64 = ctx.r[11].s64 + -5016;
	// 8327DC40: 386ADC88  addi r3, r10, -0x2378
	ctx.r[3].s64 = ctx.r[10].s64 + -9080;
	// 8327DC44: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327DC48: 4AFAF289  bl 0x8222ced0
	ctx.lr = 0x8327DC4C;
	sub_8222CED0(ctx, base);
	// 8327DC4C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327DC50: 386907D8  addi r3, r9, 0x7d8
	ctx.r[3].s64 = ctx.r[9].s64 + 2008;
	// 8327DC54: 4BA2C2CD  bl 0x82ca9f20
	ctx.lr = 0x8327DC58;
	sub_82CA9F20(ctx, base);
	// 8327DC58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327DC5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327DC60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327DC64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327DC68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327DC68 size=64
    let mut pc: u32 = 0x8327DC68;
    'dispatch: loop {
        match pc {
            0x8327DC68 => {
    //   block [0x8327DC68..0x8327DCA8)
	// 8327DC68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327DC6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327DC70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327DC74: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327DC78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327DC7C: 388BECA4  addi r4, r11, -0x135c
	ctx.r[4].s64 = ctx.r[11].s64 + -4956;
	// 8327DC80: 386ADC8C  addi r3, r10, -0x2374
	ctx.r[3].s64 = ctx.r[10].s64 + -9076;
	// 8327DC84: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327DC88: 4AFAF249  bl 0x8222ced0
	ctx.lr = 0x8327DC8C;
	sub_8222CED0(ctx, base);
	// 8327DC8C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327DC90: 386907E8  addi r3, r9, 0x7e8
	ctx.r[3].s64 = ctx.r[9].s64 + 2024;
	// 8327DC94: 4BA2C28D  bl 0x82ca9f20
	ctx.lr = 0x8327DC98;
	sub_82CA9F20(ctx, base);
	// 8327DC98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327DC9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327DCA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327DCA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327DCA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327DCA8 size=64
    let mut pc: u32 = 0x8327DCA8;
    'dispatch: loop {
        match pc {
            0x8327DCA8 => {
    //   block [0x8327DCA8..0x8327DCE8)
	// 8327DCA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327DCAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327DCB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327DCB4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327DCB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327DCBC: 388BECB4  addi r4, r11, -0x134c
	ctx.r[4].s64 = ctx.r[11].s64 + -4940;
	// 8327DCC0: 386ADC90  addi r3, r10, -0x2370
	ctx.r[3].s64 = ctx.r[10].s64 + -9072;
	// 8327DCC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327DCC8: 4AFAF209  bl 0x8222ced0
	ctx.lr = 0x8327DCCC;
	sub_8222CED0(ctx, base);
	// 8327DCCC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327DCD0: 386907F8  addi r3, r9, 0x7f8
	ctx.r[3].s64 = ctx.r[9].s64 + 2040;
	// 8327DCD4: 4BA2C24D  bl 0x82ca9f20
	ctx.lr = 0x8327DCD8;
	sub_82CA9F20(ctx, base);
	// 8327DCD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327DCDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327DCE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327DCE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327DCE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327DCE8 size=64
    let mut pc: u32 = 0x8327DCE8;
    'dispatch: loop {
        match pc {
            0x8327DCE8 => {
    //   block [0x8327DCE8..0x8327DD28)
	// 8327DCE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327DCEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327DCF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327DCF4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327DCF8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327DCFC: 388BECC4  addi r4, r11, -0x133c
	ctx.r[4].s64 = ctx.r[11].s64 + -4924;
	// 8327DD00: 386ADC94  addi r3, r10, -0x236c
	ctx.r[3].s64 = ctx.r[10].s64 + -9068;
	// 8327DD04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327DD08: 4AFAF1C9  bl 0x8222ced0
	ctx.lr = 0x8327DD0C;
	sub_8222CED0(ctx, base);
	// 8327DD0C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327DD10: 38690808  addi r3, r9, 0x808
	ctx.r[3].s64 = ctx.r[9].s64 + 2056;
	// 8327DD14: 4BA2C20D  bl 0x82ca9f20
	ctx.lr = 0x8327DD18;
	sub_82CA9F20(ctx, base);
	// 8327DD18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327DD1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327DD20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327DD24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327DD28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327DD28 size=64
    let mut pc: u32 = 0x8327DD28;
    'dispatch: loop {
        match pc {
            0x8327DD28 => {
    //   block [0x8327DD28..0x8327DD68)
	// 8327DD28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327DD2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327DD30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327DD34: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327DD38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327DD3C: 388BECD4  addi r4, r11, -0x132c
	ctx.r[4].s64 = ctx.r[11].s64 + -4908;
	// 8327DD40: 386ADC98  addi r3, r10, -0x2368
	ctx.r[3].s64 = ctx.r[10].s64 + -9064;
	// 8327DD44: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327DD48: 4AFAF189  bl 0x8222ced0
	ctx.lr = 0x8327DD4C;
	sub_8222CED0(ctx, base);
	// 8327DD4C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327DD50: 38690818  addi r3, r9, 0x818
	ctx.r[3].s64 = ctx.r[9].s64 + 2072;
	// 8327DD54: 4BA2C1CD  bl 0x82ca9f20
	ctx.lr = 0x8327DD58;
	sub_82CA9F20(ctx, base);
	// 8327DD58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327DD5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327DD60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327DD64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327DD68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327DD68 size=64
    let mut pc: u32 = 0x8327DD68;
    'dispatch: loop {
        match pc {
            0x8327DD68 => {
    //   block [0x8327DD68..0x8327DDA8)
	// 8327DD68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327DD6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327DD70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327DD74: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327DD78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327DD7C: 388BECE4  addi r4, r11, -0x131c
	ctx.r[4].s64 = ctx.r[11].s64 + -4892;
	// 8327DD80: 386ADC9C  addi r3, r10, -0x2364
	ctx.r[3].s64 = ctx.r[10].s64 + -9060;
	// 8327DD84: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327DD88: 4AFAF149  bl 0x8222ced0
	ctx.lr = 0x8327DD8C;
	sub_8222CED0(ctx, base);
	// 8327DD8C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327DD90: 38690828  addi r3, r9, 0x828
	ctx.r[3].s64 = ctx.r[9].s64 + 2088;
	// 8327DD94: 4BA2C18D  bl 0x82ca9f20
	ctx.lr = 0x8327DD98;
	sub_82CA9F20(ctx, base);
	// 8327DD98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327DD9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327DDA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327DDA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327DDA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327DDA8 size=64
    let mut pc: u32 = 0x8327DDA8;
    'dispatch: loop {
        match pc {
            0x8327DDA8 => {
    //   block [0x8327DDA8..0x8327DDE8)
	// 8327DDA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327DDAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327DDB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327DDB4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327DDB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327DDBC: 388BECF8  addi r4, r11, -0x1308
	ctx.r[4].s64 = ctx.r[11].s64 + -4872;
	// 8327DDC0: 386ADCA0  addi r3, r10, -0x2360
	ctx.r[3].s64 = ctx.r[10].s64 + -9056;
	// 8327DDC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327DDC8: 4AFAF109  bl 0x8222ced0
	ctx.lr = 0x8327DDCC;
	sub_8222CED0(ctx, base);
	// 8327DDCC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327DDD0: 38690838  addi r3, r9, 0x838
	ctx.r[3].s64 = ctx.r[9].s64 + 2104;
	// 8327DDD4: 4BA2C14D  bl 0x82ca9f20
	ctx.lr = 0x8327DDD8;
	sub_82CA9F20(ctx, base);
	// 8327DDD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327DDDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327DDE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327DDE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327DDE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327DDE8 size=56
    let mut pc: u32 = 0x8327DDE8;
    'dispatch: loop {
        match pc {
            0x8327DDE8 => {
    //   block [0x8327DDE8..0x8327DE20)
	// 8327DDE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327DDEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327DDF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327DDF4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327DDF8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327DDFC: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 8327DE00: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327DE04: 4AF75F55  bl 0x821f3d58
	ctx.lr = 0x8327DE08;
	sub_821F3D58(ctx, base);
	// 8327DE08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327DE0C: 906ADCA4  stw r3, -0x235c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9052 as u32), ctx.r[3].u32 ) };
	// 8327DE10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327DE14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327DE18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327DE1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327DE20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327DE20 size=56
    let mut pc: u32 = 0x8327DE20;
    'dispatch: loop {
        match pc {
            0x8327DE20 => {
    //   block [0x8327DE20..0x8327DE58)
	// 8327DE20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327DE24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327DE28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327DE2C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327DE30: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327DE34: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 8327DE38: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327DE3C: 4AF75F1D  bl 0x821f3d58
	ctx.lr = 0x8327DE40;
	sub_821F3D58(ctx, base);
	// 8327DE40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327DE44: 906ADCA8  stw r3, -0x2358(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9048 as u32), ctx.r[3].u32 ) };
	// 8327DE48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327DE4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327DE50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327DE54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327DE58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327DE58 size=56
    let mut pc: u32 = 0x8327DE58;
    'dispatch: loop {
        match pc {
            0x8327DE58 => {
    //   block [0x8327DE58..0x8327DE90)
	// 8327DE58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327DE5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327DE60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327DE64: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327DE68: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327DE6C: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 8327DE70: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327DE74: 4AF75EE5  bl 0x821f3d58
	ctx.lr = 0x8327DE78;
	sub_821F3D58(ctx, base);
	// 8327DE78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327DE7C: 906ADCAC  stw r3, -0x2354(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9044 as u32), ctx.r[3].u32 ) };
	// 8327DE80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327DE84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327DE88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327DE8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327DE90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327DE90 size=56
    let mut pc: u32 = 0x8327DE90;
    'dispatch: loop {
        match pc {
            0x8327DE90 => {
    //   block [0x8327DE90..0x8327DEC8)
	// 8327DE90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327DE94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327DE98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327DE9C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327DEA0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327DEA4: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 8327DEA8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327DEAC: 4AF75EAD  bl 0x821f3d58
	ctx.lr = 0x8327DEB0;
	sub_821F3D58(ctx, base);
	// 8327DEB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327DEB4: 906ADCB0  stw r3, -0x2350(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9040 as u32), ctx.r[3].u32 ) };
	// 8327DEB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327DEBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327DEC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327DEC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327DEC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327DEC8 size=56
    let mut pc: u32 = 0x8327DEC8;
    'dispatch: loop {
        match pc {
            0x8327DEC8 => {
    //   block [0x8327DEC8..0x8327DF00)
	// 8327DEC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327DECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327DED0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327DED4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327DED8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327DEDC: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 8327DEE0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327DEE4: 4AF75E75  bl 0x821f3d58
	ctx.lr = 0x8327DEE8;
	sub_821F3D58(ctx, base);
	// 8327DEE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327DEEC: 906ADCB4  stw r3, -0x234c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9036 as u32), ctx.r[3].u32 ) };
	// 8327DEF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327DEF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327DEF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327DEFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327DF00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327DF00 size=56
    let mut pc: u32 = 0x8327DF00;
    'dispatch: loop {
        match pc {
            0x8327DF00 => {
    //   block [0x8327DF00..0x8327DF38)
	// 8327DF00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327DF04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327DF08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327DF0C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327DF10: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327DF14: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 8327DF18: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327DF1C: 4AF75E3D  bl 0x821f3d58
	ctx.lr = 0x8327DF20;
	sub_821F3D58(ctx, base);
	// 8327DF20: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327DF24: 906ADCB8  stw r3, -0x2348(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9032 as u32), ctx.r[3].u32 ) };
	// 8327DF28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327DF2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327DF30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327DF34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327DF38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327DF38 size=56
    let mut pc: u32 = 0x8327DF38;
    'dispatch: loop {
        match pc {
            0x8327DF38 => {
    //   block [0x8327DF38..0x8327DF70)
	// 8327DF38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327DF3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327DF40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327DF44: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327DF48: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327DF4C: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 8327DF50: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327DF54: 4AF75E05  bl 0x821f3d58
	ctx.lr = 0x8327DF58;
	sub_821F3D58(ctx, base);
	// 8327DF58: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327DF5C: 906ADCBC  stw r3, -0x2344(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9028 as u32), ctx.r[3].u32 ) };
	// 8327DF60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327DF64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327DF68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327DF6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327DF70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327DF70 size=56
    let mut pc: u32 = 0x8327DF70;
    'dispatch: loop {
        match pc {
            0x8327DF70 => {
    //   block [0x8327DF70..0x8327DFA8)
	// 8327DF70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327DF74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327DF78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327DF7C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327DF80: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327DF84: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 8327DF88: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327DF8C: 4AF75DCD  bl 0x821f3d58
	ctx.lr = 0x8327DF90;
	sub_821F3D58(ctx, base);
	// 8327DF90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327DF94: 906ADCC0  stw r3, -0x2340(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9024 as u32), ctx.r[3].u32 ) };
	// 8327DF98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327DF9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327DFA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327DFA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327DFA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327DFA8 size=56
    let mut pc: u32 = 0x8327DFA8;
    'dispatch: loop {
        match pc {
            0x8327DFA8 => {
    //   block [0x8327DFA8..0x8327DFE0)
	// 8327DFA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327DFAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327DFB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327DFB4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327DFB8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327DFBC: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 8327DFC0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327DFC4: 4AF75D95  bl 0x821f3d58
	ctx.lr = 0x8327DFC8;
	sub_821F3D58(ctx, base);
	// 8327DFC8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327DFCC: 906ADCC4  stw r3, -0x233c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9020 as u32), ctx.r[3].u32 ) };
	// 8327DFD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327DFD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327DFD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327DFDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327DFE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327DFE0 size=56
    let mut pc: u32 = 0x8327DFE0;
    'dispatch: loop {
        match pc {
            0x8327DFE0 => {
    //   block [0x8327DFE0..0x8327E018)
	// 8327DFE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327DFE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327DFE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327DFEC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327DFF0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327DFF4: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 8327DFF8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327DFFC: 4AF75D5D  bl 0x821f3d58
	ctx.lr = 0x8327E000;
	sub_821F3D58(ctx, base);
	// 8327E000: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E004: 906ADCC8  stw r3, -0x2338(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9016 as u32), ctx.r[3].u32 ) };
	// 8327E008: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E00C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E010: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E014: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E018 size=56
    let mut pc: u32 = 0x8327E018;
    'dispatch: loop {
        match pc {
            0x8327E018 => {
    //   block [0x8327E018..0x8327E050)
	// 8327E018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E01C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E020: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E024: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327E028: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327E02C: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 8327E030: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327E034: 4AF75D25  bl 0x821f3d58
	ctx.lr = 0x8327E038;
	sub_821F3D58(ctx, base);
	// 8327E038: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E03C: 906ADCCC  stw r3, -0x2334(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9012 as u32), ctx.r[3].u32 ) };
	// 8327E040: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E044: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E048: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E04C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E050 size=56
    let mut pc: u32 = 0x8327E050;
    'dispatch: loop {
        match pc {
            0x8327E050 => {
    //   block [0x8327E050..0x8327E088)
	// 8327E050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E058: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E05C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327E060: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327E064: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 8327E068: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327E06C: 4AF75CED  bl 0x821f3d58
	ctx.lr = 0x8327E070;
	sub_821F3D58(ctx, base);
	// 8327E070: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E074: 906ADCD0  stw r3, -0x2330(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9008 as u32), ctx.r[3].u32 ) };
	// 8327E078: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E07C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E080: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E084: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E088 size=56
    let mut pc: u32 = 0x8327E088;
    'dispatch: loop {
        match pc {
            0x8327E088 => {
    //   block [0x8327E088..0x8327E0C0)
	// 8327E088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E08C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E090: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E094: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327E098: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327E09C: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 8327E0A0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327E0A4: 4AF75CB5  bl 0x821f3d58
	ctx.lr = 0x8327E0A8;
	sub_821F3D58(ctx, base);
	// 8327E0A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E0AC: 906ADCD4  stw r3, -0x232c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9004 as u32), ctx.r[3].u32 ) };
	// 8327E0B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E0B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E0B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E0BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E0C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E0C0 size=56
    let mut pc: u32 = 0x8327E0C0;
    'dispatch: loop {
        match pc {
            0x8327E0C0 => {
    //   block [0x8327E0C0..0x8327E0F8)
	// 8327E0C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E0C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E0C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E0CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327E0D0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327E0D4: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 8327E0D8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327E0DC: 4AF75C7D  bl 0x821f3d58
	ctx.lr = 0x8327E0E0;
	sub_821F3D58(ctx, base);
	// 8327E0E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E0E4: 906ADCD8  stw r3, -0x2328(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9000 as u32), ctx.r[3].u32 ) };
	// 8327E0E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E0EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E0F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E0F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E0F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E0F8 size=56
    let mut pc: u32 = 0x8327E0F8;
    'dispatch: loop {
        match pc {
            0x8327E0F8 => {
    //   block [0x8327E0F8..0x8327E130)
	// 8327E0F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E0FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E100: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E104: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327E108: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327E10C: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 8327E110: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327E114: 4AF75C45  bl 0x821f3d58
	ctx.lr = 0x8327E118;
	sub_821F3D58(ctx, base);
	// 8327E118: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E11C: 906ADCDC  stw r3, -0x2324(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8996 as u32), ctx.r[3].u32 ) };
	// 8327E120: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E124: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E128: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E12C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E130 size=56
    let mut pc: u32 = 0x8327E130;
    'dispatch: loop {
        match pc {
            0x8327E130 => {
    //   block [0x8327E130..0x8327E168)
	// 8327E130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E138: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E13C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327E140: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327E144: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 8327E148: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327E14C: 4AF75C0D  bl 0x821f3d58
	ctx.lr = 0x8327E150;
	sub_821F3D58(ctx, base);
	// 8327E150: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E154: 906ADCE0  stw r3, -0x2320(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8992 as u32), ctx.r[3].u32 ) };
	// 8327E158: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E15C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E160: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E164: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E168 size=56
    let mut pc: u32 = 0x8327E168;
    'dispatch: loop {
        match pc {
            0x8327E168 => {
    //   block [0x8327E168..0x8327E1A0)
	// 8327E168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E16C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E170: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E174: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327E178: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327E17C: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 8327E180: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327E184: 4AF75BD5  bl 0x821f3d58
	ctx.lr = 0x8327E188;
	sub_821F3D58(ctx, base);
	// 8327E188: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E18C: 906ADCE4  stw r3, -0x231c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8988 as u32), ctx.r[3].u32 ) };
	// 8327E190: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E194: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E198: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E19C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E1A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E1A0 size=56
    let mut pc: u32 = 0x8327E1A0;
    'dispatch: loop {
        match pc {
            0x8327E1A0 => {
    //   block [0x8327E1A0..0x8327E1D8)
	// 8327E1A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E1A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E1A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E1AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327E1B0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327E1B4: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 8327E1B8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327E1BC: 4AF75B9D  bl 0x821f3d58
	ctx.lr = 0x8327E1C0;
	sub_821F3D58(ctx, base);
	// 8327E1C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E1C4: 906ADCE8  stw r3, -0x2318(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8984 as u32), ctx.r[3].u32 ) };
	// 8327E1C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E1CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E1D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E1D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E1D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E1D8 size=56
    let mut pc: u32 = 0x8327E1D8;
    'dispatch: loop {
        match pc {
            0x8327E1D8 => {
    //   block [0x8327E1D8..0x8327E210)
	// 8327E1D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E1DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E1E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E1E4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327E1E8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327E1EC: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 8327E1F0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327E1F4: 4AF75B65  bl 0x821f3d58
	ctx.lr = 0x8327E1F8;
	sub_821F3D58(ctx, base);
	// 8327E1F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E1FC: 906ADCEC  stw r3, -0x2314(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8980 as u32), ctx.r[3].u32 ) };
	// 8327E200: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E204: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E208: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E20C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E210 size=56
    let mut pc: u32 = 0x8327E210;
    'dispatch: loop {
        match pc {
            0x8327E210 => {
    //   block [0x8327E210..0x8327E248)
	// 8327E210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E218: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E21C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327E220: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327E224: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 8327E228: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327E22C: 4AF75B2D  bl 0x821f3d58
	ctx.lr = 0x8327E230;
	sub_821F3D58(ctx, base);
	// 8327E230: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E234: 906ADCF0  stw r3, -0x2310(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8976 as u32), ctx.r[3].u32 ) };
	// 8327E238: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E23C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E240: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E244: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E248 size=56
    let mut pc: u32 = 0x8327E248;
    'dispatch: loop {
        match pc {
            0x8327E248 => {
    //   block [0x8327E248..0x8327E280)
	// 8327E248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E24C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E250: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E254: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327E258: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327E25C: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 8327E260: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327E264: 4AF75AF5  bl 0x821f3d58
	ctx.lr = 0x8327E268;
	sub_821F3D58(ctx, base);
	// 8327E268: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E26C: 906ADCF4  stw r3, -0x230c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8972 as u32), ctx.r[3].u32 ) };
	// 8327E270: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E274: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E278: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E27C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E280 size=64
    let mut pc: u32 = 0x8327E280;
    'dispatch: loop {
        match pc {
            0x8327E280 => {
    //   block [0x8327E280..0x8327E2C0)
	// 8327E280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E284: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E288: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E28C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327E290: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E294: 388BED78  addi r4, r11, -0x1288
	ctx.r[4].s64 = ctx.r[11].s64 + -4744;
	// 8327E298: 386ADCF8  addi r3, r10, -0x2308
	ctx.r[3].s64 = ctx.r[10].s64 + -8968;
	// 8327E29C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327E2A0: 4AFAEC31  bl 0x8222ced0
	ctx.lr = 0x8327E2A4;
	sub_8222CED0(ctx, base);
	// 8327E2A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327E2A8: 38690848  addi r3, r9, 0x848
	ctx.r[3].s64 = ctx.r[9].s64 + 2120;
	// 8327E2AC: 4BA2BC75  bl 0x82ca9f20
	ctx.lr = 0x8327E2B0;
	sub_82CA9F20(ctx, base);
	// 8327E2B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E2B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E2B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E2BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E2C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E2C0 size=64
    let mut pc: u32 = 0x8327E2C0;
    'dispatch: loop {
        match pc {
            0x8327E2C0 => {
    //   block [0x8327E2C0..0x8327E300)
	// 8327E2C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E2C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E2C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E2CC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327E2D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E2D4: 388BEDB8  addi r4, r11, -0x1248
	ctx.r[4].s64 = ctx.r[11].s64 + -4680;
	// 8327E2D8: 386ADCFC  addi r3, r10, -0x2304
	ctx.r[3].s64 = ctx.r[10].s64 + -8964;
	// 8327E2DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327E2E0: 4AFAEBF1  bl 0x8222ced0
	ctx.lr = 0x8327E2E4;
	sub_8222CED0(ctx, base);
	// 8327E2E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327E2E8: 38690858  addi r3, r9, 0x858
	ctx.r[3].s64 = ctx.r[9].s64 + 2136;
	// 8327E2EC: 4BA2BC35  bl 0x82ca9f20
	ctx.lr = 0x8327E2F0;
	sub_82CA9F20(ctx, base);
	// 8327E2F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E2F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E2F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E2FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E300 size=64
    let mut pc: u32 = 0x8327E300;
    'dispatch: loop {
        match pc {
            0x8327E300 => {
    //   block [0x8327E300..0x8327E340)
	// 8327E300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E308: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E30C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327E310: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E314: 388BEE00  addi r4, r11, -0x1200
	ctx.r[4].s64 = ctx.r[11].s64 + -4608;
	// 8327E318: 386ADD00  addi r3, r10, -0x2300
	ctx.r[3].s64 = ctx.r[10].s64 + -8960;
	// 8327E31C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327E320: 4AFAEBB1  bl 0x8222ced0
	ctx.lr = 0x8327E324;
	sub_8222CED0(ctx, base);
	// 8327E324: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327E328: 38690868  addi r3, r9, 0x868
	ctx.r[3].s64 = ctx.r[9].s64 + 2152;
	// 8327E32C: 4BA2BBF5  bl 0x82ca9f20
	ctx.lr = 0x8327E330;
	sub_82CA9F20(ctx, base);
	// 8327E330: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E334: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E338: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E33C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E340 size=64
    let mut pc: u32 = 0x8327E340;
    'dispatch: loop {
        match pc {
            0x8327E340 => {
    //   block [0x8327E340..0x8327E380)
	// 8327E340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E348: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E34C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327E350: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E354: 388BEE4C  addi r4, r11, -0x11b4
	ctx.r[4].s64 = ctx.r[11].s64 + -4532;
	// 8327E358: 386ADD04  addi r3, r10, -0x22fc
	ctx.r[3].s64 = ctx.r[10].s64 + -8956;
	// 8327E35C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327E360: 4AFAEB71  bl 0x8222ced0
	ctx.lr = 0x8327E364;
	sub_8222CED0(ctx, base);
	// 8327E364: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327E368: 38690878  addi r3, r9, 0x878
	ctx.r[3].s64 = ctx.r[9].s64 + 2168;
	// 8327E36C: 4BA2BBB5  bl 0x82ca9f20
	ctx.lr = 0x8327E370;
	sub_82CA9F20(ctx, base);
	// 8327E370: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E374: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E378: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E37C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E380 size=64
    let mut pc: u32 = 0x8327E380;
    'dispatch: loop {
        match pc {
            0x8327E380 => {
    //   block [0x8327E380..0x8327E3C0)
	// 8327E380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E388: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E38C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327E390: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E394: 388BEE5C  addi r4, r11, -0x11a4
	ctx.r[4].s64 = ctx.r[11].s64 + -4516;
	// 8327E398: 386ADD08  addi r3, r10, -0x22f8
	ctx.r[3].s64 = ctx.r[10].s64 + -8952;
	// 8327E39C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327E3A0: 4AFAEB31  bl 0x8222ced0
	ctx.lr = 0x8327E3A4;
	sub_8222CED0(ctx, base);
	// 8327E3A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327E3A8: 38690888  addi r3, r9, 0x888
	ctx.r[3].s64 = ctx.r[9].s64 + 2184;
	// 8327E3AC: 4BA2BB75  bl 0x82ca9f20
	ctx.lr = 0x8327E3B0;
	sub_82CA9F20(ctx, base);
	// 8327E3B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E3B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E3B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E3BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E3C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E3C0 size=64
    let mut pc: u32 = 0x8327E3C0;
    'dispatch: loop {
        match pc {
            0x8327E3C0 => {
    //   block [0x8327E3C0..0x8327E400)
	// 8327E3C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E3C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E3C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E3CC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327E3D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E3D4: 388BEE68  addi r4, r11, -0x1198
	ctx.r[4].s64 = ctx.r[11].s64 + -4504;
	// 8327E3D8: 386ADD0C  addi r3, r10, -0x22f4
	ctx.r[3].s64 = ctx.r[10].s64 + -8948;
	// 8327E3DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327E3E0: 4AFAEAF1  bl 0x8222ced0
	ctx.lr = 0x8327E3E4;
	sub_8222CED0(ctx, base);
	// 8327E3E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327E3E8: 38690898  addi r3, r9, 0x898
	ctx.r[3].s64 = ctx.r[9].s64 + 2200;
	// 8327E3EC: 4BA2BB35  bl 0x82ca9f20
	ctx.lr = 0x8327E3F0;
	sub_82CA9F20(ctx, base);
	// 8327E3F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E3F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E3F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E3FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E400 size=64
    let mut pc: u32 = 0x8327E400;
    'dispatch: loop {
        match pc {
            0x8327E400 => {
    //   block [0x8327E400..0x8327E440)
	// 8327E400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E408: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E40C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327E410: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E414: 388BEE74  addi r4, r11, -0x118c
	ctx.r[4].s64 = ctx.r[11].s64 + -4492;
	// 8327E418: 386ADD10  addi r3, r10, -0x22f0
	ctx.r[3].s64 = ctx.r[10].s64 + -8944;
	// 8327E41C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327E420: 4AFAEAB1  bl 0x8222ced0
	ctx.lr = 0x8327E424;
	sub_8222CED0(ctx, base);
	// 8327E424: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327E428: 386908A8  addi r3, r9, 0x8a8
	ctx.r[3].s64 = ctx.r[9].s64 + 2216;
	// 8327E42C: 4BA2BAF5  bl 0x82ca9f20
	ctx.lr = 0x8327E430;
	sub_82CA9F20(ctx, base);
	// 8327E430: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E434: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E438: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E43C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E440 size=64
    let mut pc: u32 = 0x8327E440;
    'dispatch: loop {
        match pc {
            0x8327E440 => {
    //   block [0x8327E440..0x8327E480)
	// 8327E440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E448: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E44C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327E450: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E454: 388BEE84  addi r4, r11, -0x117c
	ctx.r[4].s64 = ctx.r[11].s64 + -4476;
	// 8327E458: 386ADD14  addi r3, r10, -0x22ec
	ctx.r[3].s64 = ctx.r[10].s64 + -8940;
	// 8327E45C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327E460: 4AFAEA71  bl 0x8222ced0
	ctx.lr = 0x8327E464;
	sub_8222CED0(ctx, base);
	// 8327E464: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327E468: 386908B8  addi r3, r9, 0x8b8
	ctx.r[3].s64 = ctx.r[9].s64 + 2232;
	// 8327E46C: 4BA2BAB5  bl 0x82ca9f20
	ctx.lr = 0x8327E470;
	sub_82CA9F20(ctx, base);
	// 8327E470: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E474: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E478: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E47C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E480 size=64
    let mut pc: u32 = 0x8327E480;
    'dispatch: loop {
        match pc {
            0x8327E480 => {
    //   block [0x8327E480..0x8327E4C0)
	// 8327E480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E488: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E48C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327E490: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E494: 388BEE90  addi r4, r11, -0x1170
	ctx.r[4].s64 = ctx.r[11].s64 + -4464;
	// 8327E498: 386ADD18  addi r3, r10, -0x22e8
	ctx.r[3].s64 = ctx.r[10].s64 + -8936;
	// 8327E49C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327E4A0: 4AFAEA31  bl 0x8222ced0
	ctx.lr = 0x8327E4A4;
	sub_8222CED0(ctx, base);
	// 8327E4A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327E4A8: 386908C8  addi r3, r9, 0x8c8
	ctx.r[3].s64 = ctx.r[9].s64 + 2248;
	// 8327E4AC: 4BA2BA75  bl 0x82ca9f20
	ctx.lr = 0x8327E4B0;
	sub_82CA9F20(ctx, base);
	// 8327E4B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E4B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E4B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E4BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E4C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E4C0 size=64
    let mut pc: u32 = 0x8327E4C0;
    'dispatch: loop {
        match pc {
            0x8327E4C0 => {
    //   block [0x8327E4C0..0x8327E500)
	// 8327E4C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E4C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E4C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E4CC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327E4D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E4D4: 388BEE9C  addi r4, r11, -0x1164
	ctx.r[4].s64 = ctx.r[11].s64 + -4452;
	// 8327E4D8: 386ADD1C  addi r3, r10, -0x22e4
	ctx.r[3].s64 = ctx.r[10].s64 + -8932;
	// 8327E4DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327E4E0: 4AFAE9F1  bl 0x8222ced0
	ctx.lr = 0x8327E4E4;
	sub_8222CED0(ctx, base);
	// 8327E4E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327E4E8: 386908D8  addi r3, r9, 0x8d8
	ctx.r[3].s64 = ctx.r[9].s64 + 2264;
	// 8327E4EC: 4BA2BA35  bl 0x82ca9f20
	ctx.lr = 0x8327E4F0;
	sub_82CA9F20(ctx, base);
	// 8327E4F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E4F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E4F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E4FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E500 size=64
    let mut pc: u32 = 0x8327E500;
    'dispatch: loop {
        match pc {
            0x8327E500 => {
    //   block [0x8327E500..0x8327E540)
	// 8327E500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E508: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E50C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327E510: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E514: 388BEEB4  addi r4, r11, -0x114c
	ctx.r[4].s64 = ctx.r[11].s64 + -4428;
	// 8327E518: 386ADD20  addi r3, r10, -0x22e0
	ctx.r[3].s64 = ctx.r[10].s64 + -8928;
	// 8327E51C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327E520: 4AFAE9B1  bl 0x8222ced0
	ctx.lr = 0x8327E524;
	sub_8222CED0(ctx, base);
	// 8327E524: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327E528: 386908E8  addi r3, r9, 0x8e8
	ctx.r[3].s64 = ctx.r[9].s64 + 2280;
	// 8327E52C: 4BA2B9F5  bl 0x82ca9f20
	ctx.lr = 0x8327E530;
	sub_82CA9F20(ctx, base);
	// 8327E530: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E534: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E538: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E53C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E540 size=64
    let mut pc: u32 = 0x8327E540;
    'dispatch: loop {
        match pc {
            0x8327E540 => {
    //   block [0x8327E540..0x8327E580)
	// 8327E540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E548: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E54C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327E550: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E554: 388BEEC0  addi r4, r11, -0x1140
	ctx.r[4].s64 = ctx.r[11].s64 + -4416;
	// 8327E558: 386ADD24  addi r3, r10, -0x22dc
	ctx.r[3].s64 = ctx.r[10].s64 + -8924;
	// 8327E55C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327E560: 4AFAE971  bl 0x8222ced0
	ctx.lr = 0x8327E564;
	sub_8222CED0(ctx, base);
	// 8327E564: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327E568: 386908F8  addi r3, r9, 0x8f8
	ctx.r[3].s64 = ctx.r[9].s64 + 2296;
	// 8327E56C: 4BA2B9B5  bl 0x82ca9f20
	ctx.lr = 0x8327E570;
	sub_82CA9F20(ctx, base);
	// 8327E570: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E574: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E578: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E57C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E580 size=64
    let mut pc: u32 = 0x8327E580;
    'dispatch: loop {
        match pc {
            0x8327E580 => {
    //   block [0x8327E580..0x8327E5C0)
	// 8327E580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E588: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E58C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327E590: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E594: 388BEED0  addi r4, r11, -0x1130
	ctx.r[4].s64 = ctx.r[11].s64 + -4400;
	// 8327E598: 386ADD28  addi r3, r10, -0x22d8
	ctx.r[3].s64 = ctx.r[10].s64 + -8920;
	// 8327E59C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327E5A0: 4AFAE931  bl 0x8222ced0
	ctx.lr = 0x8327E5A4;
	sub_8222CED0(ctx, base);
	// 8327E5A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327E5A8: 38690908  addi r3, r9, 0x908
	ctx.r[3].s64 = ctx.r[9].s64 + 2312;
	// 8327E5AC: 4BA2B975  bl 0x82ca9f20
	ctx.lr = 0x8327E5B0;
	sub_82CA9F20(ctx, base);
	// 8327E5B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E5B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E5B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E5BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E5C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E5C0 size=64
    let mut pc: u32 = 0x8327E5C0;
    'dispatch: loop {
        match pc {
            0x8327E5C0 => {
    //   block [0x8327E5C0..0x8327E600)
	// 8327E5C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E5C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E5C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E5CC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327E5D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E5D4: 388BEEDC  addi r4, r11, -0x1124
	ctx.r[4].s64 = ctx.r[11].s64 + -4388;
	// 8327E5D8: 386ADD2C  addi r3, r10, -0x22d4
	ctx.r[3].s64 = ctx.r[10].s64 + -8916;
	// 8327E5DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327E5E0: 4AFAE8F1  bl 0x8222ced0
	ctx.lr = 0x8327E5E4;
	sub_8222CED0(ctx, base);
	// 8327E5E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327E5E8: 38690918  addi r3, r9, 0x918
	ctx.r[3].s64 = ctx.r[9].s64 + 2328;
	// 8327E5EC: 4BA2B935  bl 0x82ca9f20
	ctx.lr = 0x8327E5F0;
	sub_82CA9F20(ctx, base);
	// 8327E5F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E5F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E5F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E5FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E600 size=56
    let mut pc: u32 = 0x8327E600;
    'dispatch: loop {
        match pc {
            0x8327E600 => {
    //   block [0x8327E600..0x8327E638)
	// 8327E600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E604: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E608: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E60C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327E610: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327E614: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 8327E618: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327E61C: 4AF7573D  bl 0x821f3d58
	ctx.lr = 0x8327E620;
	sub_821F3D58(ctx, base);
	// 8327E620: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E624: 906ADD30  stw r3, -0x22d0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8912 as u32), ctx.r[3].u32 ) };
	// 8327E628: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E62C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E630: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E634: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E638 size=56
    let mut pc: u32 = 0x8327E638;
    'dispatch: loop {
        match pc {
            0x8327E638 => {
    //   block [0x8327E638..0x8327E670)
	// 8327E638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E63C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E640: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E644: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327E648: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327E64C: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 8327E650: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327E654: 4AF75705  bl 0x821f3d58
	ctx.lr = 0x8327E658;
	sub_821F3D58(ctx, base);
	// 8327E658: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E65C: 906ADD34  stw r3, -0x22cc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8908 as u32), ctx.r[3].u32 ) };
	// 8327E660: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E664: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E668: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E66C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E670 size=56
    let mut pc: u32 = 0x8327E670;
    'dispatch: loop {
        match pc {
            0x8327E670 => {
    //   block [0x8327E670..0x8327E6A8)
	// 8327E670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E678: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E67C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327E680: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327E684: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 8327E688: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327E68C: 4AF756CD  bl 0x821f3d58
	ctx.lr = 0x8327E690;
	sub_821F3D58(ctx, base);
	// 8327E690: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E694: 906ADD38  stw r3, -0x22c8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8904 as u32), ctx.r[3].u32 ) };
	// 8327E698: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E69C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E6A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E6A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E6A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E6A8 size=56
    let mut pc: u32 = 0x8327E6A8;
    'dispatch: loop {
        match pc {
            0x8327E6A8 => {
    //   block [0x8327E6A8..0x8327E6E0)
	// 8327E6A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E6AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E6B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E6B4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327E6B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327E6BC: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 8327E6C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327E6C4: 4AF75695  bl 0x821f3d58
	ctx.lr = 0x8327E6C8;
	sub_821F3D58(ctx, base);
	// 8327E6C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E6CC: 906ADD3C  stw r3, -0x22c4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8900 as u32), ctx.r[3].u32 ) };
	// 8327E6D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E6D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E6D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E6DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E6E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E6E0 size=56
    let mut pc: u32 = 0x8327E6E0;
    'dispatch: loop {
        match pc {
            0x8327E6E0 => {
    //   block [0x8327E6E0..0x8327E718)
	// 8327E6E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E6E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E6E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E6EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327E6F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327E6F4: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 8327E6F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327E6FC: 4AF7565D  bl 0x821f3d58
	ctx.lr = 0x8327E700;
	sub_821F3D58(ctx, base);
	// 8327E700: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E704: 906ADD40  stw r3, -0x22c0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8896 as u32), ctx.r[3].u32 ) };
	// 8327E708: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E70C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E710: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E714: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E718 size=56
    let mut pc: u32 = 0x8327E718;
    'dispatch: loop {
        match pc {
            0x8327E718 => {
    //   block [0x8327E718..0x8327E750)
	// 8327E718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E71C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E720: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E724: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327E728: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327E72C: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 8327E730: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327E734: 4AF75625  bl 0x821f3d58
	ctx.lr = 0x8327E738;
	sub_821F3D58(ctx, base);
	// 8327E738: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E73C: 906ADD44  stw r3, -0x22bc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8892 as u32), ctx.r[3].u32 ) };
	// 8327E740: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E744: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E748: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E74C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E750 size=56
    let mut pc: u32 = 0x8327E750;
    'dispatch: loop {
        match pc {
            0x8327E750 => {
    //   block [0x8327E750..0x8327E788)
	// 8327E750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E758: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E75C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327E760: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327E764: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 8327E768: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327E76C: 4AF755ED  bl 0x821f3d58
	ctx.lr = 0x8327E770;
	sub_821F3D58(ctx, base);
	// 8327E770: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E774: 906ADD48  stw r3, -0x22b8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8888 as u32), ctx.r[3].u32 ) };
	// 8327E778: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E77C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E780: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E784: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E788 size=56
    let mut pc: u32 = 0x8327E788;
    'dispatch: loop {
        match pc {
            0x8327E788 => {
    //   block [0x8327E788..0x8327E7C0)
	// 8327E788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E78C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E790: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E794: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327E798: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327E79C: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 8327E7A0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327E7A4: 4AF755B5  bl 0x821f3d58
	ctx.lr = 0x8327E7A8;
	sub_821F3D58(ctx, base);
	// 8327E7A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E7AC: 906ADD4C  stw r3, -0x22b4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8884 as u32), ctx.r[3].u32 ) };
	// 8327E7B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E7B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E7B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E7BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E7C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E7C0 size=56
    let mut pc: u32 = 0x8327E7C0;
    'dispatch: loop {
        match pc {
            0x8327E7C0 => {
    //   block [0x8327E7C0..0x8327E7F8)
	// 8327E7C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E7C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E7C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E7CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327E7D0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327E7D4: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 8327E7D8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327E7DC: 4AF7557D  bl 0x821f3d58
	ctx.lr = 0x8327E7E0;
	sub_821F3D58(ctx, base);
	// 8327E7E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E7E4: 906ADD50  stw r3, -0x22b0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8880 as u32), ctx.r[3].u32 ) };
	// 8327E7E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E7EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E7F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E7F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E7F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E7F8 size=56
    let mut pc: u32 = 0x8327E7F8;
    'dispatch: loop {
        match pc {
            0x8327E7F8 => {
    //   block [0x8327E7F8..0x8327E830)
	// 8327E7F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E7FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E800: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E804: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327E808: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327E80C: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 8327E810: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327E814: 4AF75545  bl 0x821f3d58
	ctx.lr = 0x8327E818;
	sub_821F3D58(ctx, base);
	// 8327E818: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E81C: 906ADD54  stw r3, -0x22ac(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8876 as u32), ctx.r[3].u32 ) };
	// 8327E820: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E824: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E828: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E82C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E830 size=56
    let mut pc: u32 = 0x8327E830;
    'dispatch: loop {
        match pc {
            0x8327E830 => {
    //   block [0x8327E830..0x8327E868)
	// 8327E830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E838: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E83C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327E840: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327E844: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 8327E848: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327E84C: 4AF7550D  bl 0x821f3d58
	ctx.lr = 0x8327E850;
	sub_821F3D58(ctx, base);
	// 8327E850: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E854: 906ADD58  stw r3, -0x22a8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8872 as u32), ctx.r[3].u32 ) };
	// 8327E858: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E85C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E860: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E864: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E868 size=56
    let mut pc: u32 = 0x8327E868;
    'dispatch: loop {
        match pc {
            0x8327E868 => {
    //   block [0x8327E868..0x8327E8A0)
	// 8327E868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E86C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E870: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E874: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327E878: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327E87C: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 8327E880: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327E884: 4AF754D5  bl 0x821f3d58
	ctx.lr = 0x8327E888;
	sub_821F3D58(ctx, base);
	// 8327E888: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E88C: 906ADD5C  stw r3, -0x22a4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8868 as u32), ctx.r[3].u32 ) };
	// 8327E890: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E894: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E898: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E89C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E8A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E8A0 size=56
    let mut pc: u32 = 0x8327E8A0;
    'dispatch: loop {
        match pc {
            0x8327E8A0 => {
    //   block [0x8327E8A0..0x8327E8D8)
	// 8327E8A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E8A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E8A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E8AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327E8B0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327E8B4: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 8327E8B8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327E8BC: 4AF7549D  bl 0x821f3d58
	ctx.lr = 0x8327E8C0;
	sub_821F3D58(ctx, base);
	// 8327E8C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E8C4: 906ADD60  stw r3, -0x22a0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8864 as u32), ctx.r[3].u32 ) };
	// 8327E8C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E8CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E8D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E8D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E8D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E8D8 size=56
    let mut pc: u32 = 0x8327E8D8;
    'dispatch: loop {
        match pc {
            0x8327E8D8 => {
    //   block [0x8327E8D8..0x8327E910)
	// 8327E8D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E8DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E8E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E8E4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327E8E8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327E8EC: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 8327E8F0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327E8F4: 4AF75465  bl 0x821f3d58
	ctx.lr = 0x8327E8F8;
	sub_821F3D58(ctx, base);
	// 8327E8F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E8FC: 906ADD64  stw r3, -0x229c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8860 as u32), ctx.r[3].u32 ) };
	// 8327E900: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E904: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E908: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E90C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E910 size=56
    let mut pc: u32 = 0x8327E910;
    'dispatch: loop {
        match pc {
            0x8327E910 => {
    //   block [0x8327E910..0x8327E948)
	// 8327E910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E918: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E91C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327E920: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327E924: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 8327E928: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327E92C: 4AF7542D  bl 0x821f3d58
	ctx.lr = 0x8327E930;
	sub_821F3D58(ctx, base);
	// 8327E930: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E934: 906ADD68  stw r3, -0x2298(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8856 as u32), ctx.r[3].u32 ) };
	// 8327E938: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E93C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E940: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E944: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E948 size=56
    let mut pc: u32 = 0x8327E948;
    'dispatch: loop {
        match pc {
            0x8327E948 => {
    //   block [0x8327E948..0x8327E980)
	// 8327E948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E94C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E950: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E954: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327E958: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327E95C: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 8327E960: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327E964: 4AF753F5  bl 0x821f3d58
	ctx.lr = 0x8327E968;
	sub_821F3D58(ctx, base);
	// 8327E968: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E96C: 906ADD6C  stw r3, -0x2294(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8852 as u32), ctx.r[3].u32 ) };
	// 8327E970: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E974: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E978: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E97C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E980 size=56
    let mut pc: u32 = 0x8327E980;
    'dispatch: loop {
        match pc {
            0x8327E980 => {
    //   block [0x8327E980..0x8327E9B8)
	// 8327E980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E988: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E98C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327E990: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327E994: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 8327E998: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327E99C: 4AF753BD  bl 0x821f3d58
	ctx.lr = 0x8327E9A0;
	sub_821F3D58(ctx, base);
	// 8327E9A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E9A4: 906ADD70  stw r3, -0x2290(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8848 as u32), ctx.r[3].u32 ) };
	// 8327E9A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E9AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E9B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E9B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E9B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E9B8 size=56
    let mut pc: u32 = 0x8327E9B8;
    'dispatch: loop {
        match pc {
            0x8327E9B8 => {
    //   block [0x8327E9B8..0x8327E9F0)
	// 8327E9B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E9BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E9C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E9C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327E9C8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327E9CC: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 8327E9D0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327E9D4: 4AF75385  bl 0x821f3d58
	ctx.lr = 0x8327E9D8;
	sub_821F3D58(ctx, base);
	// 8327E9D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E9DC: 906ADD74  stw r3, -0x228c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8844 as u32), ctx.r[3].u32 ) };
	// 8327E9E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E9E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E9E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E9EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E9F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E9F0 size=56
    let mut pc: u32 = 0x8327E9F0;
    'dispatch: loop {
        match pc {
            0x8327E9F0 => {
    //   block [0x8327E9F0..0x8327EA28)
	// 8327E9F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E9F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E9F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E9FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327EA00: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327EA04: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 8327EA08: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327EA0C: 4AF7534D  bl 0x821f3d58
	ctx.lr = 0x8327EA10;
	sub_821F3D58(ctx, base);
	// 8327EA10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327EA14: 906ADD78  stw r3, -0x2288(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8840 as u32), ctx.r[3].u32 ) };
	// 8327EA18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327EA1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327EA20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327EA24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327EA28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327EA28 size=56
    let mut pc: u32 = 0x8327EA28;
    'dispatch: loop {
        match pc {
            0x8327EA28 => {
    //   block [0x8327EA28..0x8327EA60)
	// 8327EA28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327EA2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327EA30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327EA34: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327EA38: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327EA3C: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 8327EA40: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327EA44: 4AF75315  bl 0x821f3d58
	ctx.lr = 0x8327EA48;
	sub_821F3D58(ctx, base);
	// 8327EA48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327EA4C: 906ADD7C  stw r3, -0x2284(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8836 as u32), ctx.r[3].u32 ) };
	// 8327EA50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327EA54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327EA58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327EA5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327EA60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327EA60 size=56
    let mut pc: u32 = 0x8327EA60;
    'dispatch: loop {
        match pc {
            0x8327EA60 => {
    //   block [0x8327EA60..0x8327EA98)
	// 8327EA60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327EA64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327EA68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327EA6C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327EA70: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327EA74: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 8327EA78: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327EA7C: 4AF752DD  bl 0x821f3d58
	ctx.lr = 0x8327EA80;
	sub_821F3D58(ctx, base);
	// 8327EA80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327EA84: 906ADD80  stw r3, -0x2280(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8832 as u32), ctx.r[3].u32 ) };
	// 8327EA88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327EA8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327EA90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327EA94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327EA98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327EA98 size=64
    let mut pc: u32 = 0x8327EA98;
    'dispatch: loop {
        match pc {
            0x8327EA98 => {
    //   block [0x8327EA98..0x8327EAD8)
	// 8327EA98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327EA9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327EAA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327EAA4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327EAA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327EAAC: 388BE638  addi r4, r11, -0x19c8
	ctx.r[4].s64 = ctx.r[11].s64 + -6600;
	// 8327EAB0: 386ADD84  addi r3, r10, -0x227c
	ctx.r[3].s64 = ctx.r[10].s64 + -8828;
	// 8327EAB4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327EAB8: 4AFAE419  bl 0x8222ced0
	ctx.lr = 0x8327EABC;
	sub_8222CED0(ctx, base);
	// 8327EABC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327EAC0: 38690928  addi r3, r9, 0x928
	ctx.r[3].s64 = ctx.r[9].s64 + 2344;
	// 8327EAC4: 4BA2B45D  bl 0x82ca9f20
	ctx.lr = 0x8327EAC8;
	sub_82CA9F20(ctx, base);
	// 8327EAC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327EACC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327EAD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327EAD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327EAD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327EAD8 size=64
    let mut pc: u32 = 0x8327EAD8;
    'dispatch: loop {
        match pc {
            0x8327EAD8 => {
    //   block [0x8327EAD8..0x8327EB18)
	// 8327EAD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327EADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327EAE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327EAE4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327EAE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327EAEC: 388BEF10  addi r4, r11, -0x10f0
	ctx.r[4].s64 = ctx.r[11].s64 + -4336;
	// 8327EAF0: 386ADD88  addi r3, r10, -0x2278
	ctx.r[3].s64 = ctx.r[10].s64 + -8824;
	// 8327EAF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327EAF8: 4AFAE3D9  bl 0x8222ced0
	ctx.lr = 0x8327EAFC;
	sub_8222CED0(ctx, base);
	// 8327EAFC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327EB00: 38690938  addi r3, r9, 0x938
	ctx.r[3].s64 = ctx.r[9].s64 + 2360;
	// 8327EB04: 4BA2B41D  bl 0x82ca9f20
	ctx.lr = 0x8327EB08;
	sub_82CA9F20(ctx, base);
	// 8327EB08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327EB0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327EB10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327EB14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327EB18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327EB18 size=64
    let mut pc: u32 = 0x8327EB18;
    'dispatch: loop {
        match pc {
            0x8327EB18 => {
    //   block [0x8327EB18..0x8327EB58)
	// 8327EB18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327EB1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327EB20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327EB24: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327EB28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327EB2C: 388BEF48  addi r4, r11, -0x10b8
	ctx.r[4].s64 = ctx.r[11].s64 + -4280;
	// 8327EB30: 386ADD8C  addi r3, r10, -0x2274
	ctx.r[3].s64 = ctx.r[10].s64 + -8820;
	// 8327EB34: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327EB38: 4AFAE399  bl 0x8222ced0
	ctx.lr = 0x8327EB3C;
	sub_8222CED0(ctx, base);
	// 8327EB3C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327EB40: 38690948  addi r3, r9, 0x948
	ctx.r[3].s64 = ctx.r[9].s64 + 2376;
	// 8327EB44: 4BA2B3DD  bl 0x82ca9f20
	ctx.lr = 0x8327EB48;
	sub_82CA9F20(ctx, base);
	// 8327EB48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327EB4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327EB50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327EB54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327EB58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327EB58 size=64
    let mut pc: u32 = 0x8327EB58;
    'dispatch: loop {
        match pc {
            0x8327EB58 => {
    //   block [0x8327EB58..0x8327EB98)
	// 8327EB58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327EB5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327EB60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327EB64: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327EB68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327EB6C: 388BEF80  addi r4, r11, -0x1080
	ctx.r[4].s64 = ctx.r[11].s64 + -4224;
	// 8327EB70: 386ADD90  addi r3, r10, -0x2270
	ctx.r[3].s64 = ctx.r[10].s64 + -8816;
	// 8327EB74: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327EB78: 4AFAE359  bl 0x8222ced0
	ctx.lr = 0x8327EB7C;
	sub_8222CED0(ctx, base);
	// 8327EB7C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327EB80: 38690958  addi r3, r9, 0x958
	ctx.r[3].s64 = ctx.r[9].s64 + 2392;
	// 8327EB84: 4BA2B39D  bl 0x82ca9f20
	ctx.lr = 0x8327EB88;
	sub_82CA9F20(ctx, base);
	// 8327EB88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327EB8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327EB90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327EB94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327EB98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327EB98 size=64
    let mut pc: u32 = 0x8327EB98;
    'dispatch: loop {
        match pc {
            0x8327EB98 => {
    //   block [0x8327EB98..0x8327EBD8)
	// 8327EB98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327EB9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327EBA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327EBA4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327EBA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327EBAC: 388BEF88  addi r4, r11, -0x1078
	ctx.r[4].s64 = ctx.r[11].s64 + -4216;
	// 8327EBB0: 386ADD94  addi r3, r10, -0x226c
	ctx.r[3].s64 = ctx.r[10].s64 + -8812;
	// 8327EBB4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327EBB8: 4AFAE319  bl 0x8222ced0
	ctx.lr = 0x8327EBBC;
	sub_8222CED0(ctx, base);
	// 8327EBBC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327EBC0: 38690968  addi r3, r9, 0x968
	ctx.r[3].s64 = ctx.r[9].s64 + 2408;
	// 8327EBC4: 4BA2B35D  bl 0x82ca9f20
	ctx.lr = 0x8327EBC8;
	sub_82CA9F20(ctx, base);
	// 8327EBC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327EBCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327EBD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327EBD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327EBD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327EBD8 size=64
    let mut pc: u32 = 0x8327EBD8;
    'dispatch: loop {
        match pc {
            0x8327EBD8 => {
    //   block [0x8327EBD8..0x8327EC18)
	// 8327EBD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327EBDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327EBE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327EBE4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327EBE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327EBEC: 388BEF90  addi r4, r11, -0x1070
	ctx.r[4].s64 = ctx.r[11].s64 + -4208;
	// 8327EBF0: 386ADD98  addi r3, r10, -0x2268
	ctx.r[3].s64 = ctx.r[10].s64 + -8808;
	// 8327EBF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327EBF8: 4AFAE2D9  bl 0x8222ced0
	ctx.lr = 0x8327EBFC;
	sub_8222CED0(ctx, base);
	// 8327EBFC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327EC00: 38690978  addi r3, r9, 0x978
	ctx.r[3].s64 = ctx.r[9].s64 + 2424;
	// 8327EC04: 4BA2B31D  bl 0x82ca9f20
	ctx.lr = 0x8327EC08;
	sub_82CA9F20(ctx, base);
	// 8327EC08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327EC0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327EC10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327EC14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327EC18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327EC18 size=64
    let mut pc: u32 = 0x8327EC18;
    'dispatch: loop {
        match pc {
            0x8327EC18 => {
    //   block [0x8327EC18..0x8327EC58)
	// 8327EC18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327EC1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327EC20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327EC24: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327EC28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327EC2C: 388BEF98  addi r4, r11, -0x1068
	ctx.r[4].s64 = ctx.r[11].s64 + -4200;
	// 8327EC30: 386ADD9C  addi r3, r10, -0x2264
	ctx.r[3].s64 = ctx.r[10].s64 + -8804;
	// 8327EC34: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327EC38: 4AFAE299  bl 0x8222ced0
	ctx.lr = 0x8327EC3C;
	sub_8222CED0(ctx, base);
	// 8327EC3C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327EC40: 38690988  addi r3, r9, 0x988
	ctx.r[3].s64 = ctx.r[9].s64 + 2440;
	// 8327EC44: 4BA2B2DD  bl 0x82ca9f20
	ctx.lr = 0x8327EC48;
	sub_82CA9F20(ctx, base);
	// 8327EC48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327EC4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327EC50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327EC54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327EC58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327EC58 size=64
    let mut pc: u32 = 0x8327EC58;
    'dispatch: loop {
        match pc {
            0x8327EC58 => {
    //   block [0x8327EC58..0x8327EC98)
	// 8327EC58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327EC5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327EC60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327EC64: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327EC68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327EC6C: 388BEFA4  addi r4, r11, -0x105c
	ctx.r[4].s64 = ctx.r[11].s64 + -4188;
	// 8327EC70: 386ADDA0  addi r3, r10, -0x2260
	ctx.r[3].s64 = ctx.r[10].s64 + -8800;
	// 8327EC74: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327EC78: 4AFAE259  bl 0x8222ced0
	ctx.lr = 0x8327EC7C;
	sub_8222CED0(ctx, base);
	// 8327EC7C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327EC80: 38690998  addi r3, r9, 0x998
	ctx.r[3].s64 = ctx.r[9].s64 + 2456;
	// 8327EC84: 4BA2B29D  bl 0x82ca9f20
	ctx.lr = 0x8327EC88;
	sub_82CA9F20(ctx, base);
	// 8327EC88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327EC8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327EC90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327EC94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327EC98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327EC98 size=64
    let mut pc: u32 = 0x8327EC98;
    'dispatch: loop {
        match pc {
            0x8327EC98 => {
    //   block [0x8327EC98..0x8327ECD8)
	// 8327EC98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327EC9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327ECA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327ECA4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327ECA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327ECAC: 388BEFAC  addi r4, r11, -0x1054
	ctx.r[4].s64 = ctx.r[11].s64 + -4180;
	// 8327ECB0: 386ADDA4  addi r3, r10, -0x225c
	ctx.r[3].s64 = ctx.r[10].s64 + -8796;
	// 8327ECB4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327ECB8: 4AFAE219  bl 0x8222ced0
	ctx.lr = 0x8327ECBC;
	sub_8222CED0(ctx, base);
	// 8327ECBC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327ECC0: 386909A8  addi r3, r9, 0x9a8
	ctx.r[3].s64 = ctx.r[9].s64 + 2472;
	// 8327ECC4: 4BA2B25D  bl 0x82ca9f20
	ctx.lr = 0x8327ECC8;
	sub_82CA9F20(ctx, base);
	// 8327ECC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327ECCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327ECD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327ECD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327ECD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327ECD8 size=64
    let mut pc: u32 = 0x8327ECD8;
    'dispatch: loop {
        match pc {
            0x8327ECD8 => {
    //   block [0x8327ECD8..0x8327ED18)
	// 8327ECD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327ECDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327ECE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327ECE4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8327ECE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327ECEC: 388B8F6C  addi r4, r11, -0x7094
	ctx.r[4].s64 = ctx.r[11].s64 + -28820;
	// 8327ECF0: 386ADDA8  addi r3, r10, -0x2258
	ctx.r[3].s64 = ctx.r[10].s64 + -8792;
	// 8327ECF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327ECF8: 4AFAE1D9  bl 0x8222ced0
	ctx.lr = 0x8327ECFC;
	sub_8222CED0(ctx, base);
	// 8327ECFC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327ED00: 386909B8  addi r3, r9, 0x9b8
	ctx.r[3].s64 = ctx.r[9].s64 + 2488;
	// 8327ED04: 4BA2B21D  bl 0x82ca9f20
	ctx.lr = 0x8327ED08;
	sub_82CA9F20(ctx, base);
	// 8327ED08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327ED0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327ED10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327ED14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327ED18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327ED18 size=64
    let mut pc: u32 = 0x8327ED18;
    'dispatch: loop {
        match pc {
            0x8327ED18 => {
    //   block [0x8327ED18..0x8327ED58)
	// 8327ED18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327ED1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327ED20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327ED24: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327ED28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327ED2C: 388BEFBC  addi r4, r11, -0x1044
	ctx.r[4].s64 = ctx.r[11].s64 + -4164;
	// 8327ED30: 386ADDAC  addi r3, r10, -0x2254
	ctx.r[3].s64 = ctx.r[10].s64 + -8788;
	// 8327ED34: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327ED38: 4AFAE199  bl 0x8222ced0
	ctx.lr = 0x8327ED3C;
	sub_8222CED0(ctx, base);
	// 8327ED3C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327ED40: 386909C8  addi r3, r9, 0x9c8
	ctx.r[3].s64 = ctx.r[9].s64 + 2504;
	// 8327ED44: 4BA2B1DD  bl 0x82ca9f20
	ctx.lr = 0x8327ED48;
	sub_82CA9F20(ctx, base);
	// 8327ED48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327ED4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327ED50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327ED54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327ED58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327ED58 size=64
    let mut pc: u32 = 0x8327ED58;
    'dispatch: loop {
        match pc {
            0x8327ED58 => {
    //   block [0x8327ED58..0x8327ED98)
	// 8327ED58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327ED5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327ED60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327ED64: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327ED68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327ED6C: 388BEFC4  addi r4, r11, -0x103c
	ctx.r[4].s64 = ctx.r[11].s64 + -4156;
	// 8327ED70: 386ADDB0  addi r3, r10, -0x2250
	ctx.r[3].s64 = ctx.r[10].s64 + -8784;
	// 8327ED74: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327ED78: 4AFAE159  bl 0x8222ced0
	ctx.lr = 0x8327ED7C;
	sub_8222CED0(ctx, base);
	// 8327ED7C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327ED80: 386909D8  addi r3, r9, 0x9d8
	ctx.r[3].s64 = ctx.r[9].s64 + 2520;
	// 8327ED84: 4BA2B19D  bl 0x82ca9f20
	ctx.lr = 0x8327ED88;
	sub_82CA9F20(ctx, base);
	// 8327ED88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327ED8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327ED90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327ED94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327ED98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327ED98 size=64
    let mut pc: u32 = 0x8327ED98;
    'dispatch: loop {
        match pc {
            0x8327ED98 => {
    //   block [0x8327ED98..0x8327EDD8)
	// 8327ED98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327ED9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327EDA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327EDA4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327EDA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327EDAC: 388BEFD8  addi r4, r11, -0x1028
	ctx.r[4].s64 = ctx.r[11].s64 + -4136;
	// 8327EDB0: 386ADDB4  addi r3, r10, -0x224c
	ctx.r[3].s64 = ctx.r[10].s64 + -8780;
	// 8327EDB4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327EDB8: 4AFAE119  bl 0x8222ced0
	ctx.lr = 0x8327EDBC;
	sub_8222CED0(ctx, base);
	// 8327EDBC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327EDC0: 386909E8  addi r3, r9, 0x9e8
	ctx.r[3].s64 = ctx.r[9].s64 + 2536;
	// 8327EDC4: 4BA2B15D  bl 0x82ca9f20
	ctx.lr = 0x8327EDC8;
	sub_82CA9F20(ctx, base);
	// 8327EDC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327EDCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327EDD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327EDD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327EDD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327EDD8 size=64
    let mut pc: u32 = 0x8327EDD8;
    'dispatch: loop {
        match pc {
            0x8327EDD8 => {
    //   block [0x8327EDD8..0x8327EE18)
	// 8327EDD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327EDDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327EDE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327EDE4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327EDE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327EDEC: 388BEFEC  addi r4, r11, -0x1014
	ctx.r[4].s64 = ctx.r[11].s64 + -4116;
	// 8327EDF0: 386ADDB8  addi r3, r10, -0x2248
	ctx.r[3].s64 = ctx.r[10].s64 + -8776;
	// 8327EDF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327EDF8: 4AFAE0D9  bl 0x8222ced0
	ctx.lr = 0x8327EDFC;
	sub_8222CED0(ctx, base);
	// 8327EDFC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327EE00: 386909F8  addi r3, r9, 0x9f8
	ctx.r[3].s64 = ctx.r[9].s64 + 2552;
	// 8327EE04: 4BA2B11D  bl 0x82ca9f20
	ctx.lr = 0x8327EE08;
	sub_82CA9F20(ctx, base);
	// 8327EE08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327EE0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327EE10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327EE14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327EE18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327EE18 size=64
    let mut pc: u32 = 0x8327EE18;
    'dispatch: loop {
        match pc {
            0x8327EE18 => {
    //   block [0x8327EE18..0x8327EE58)
	// 8327EE18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327EE1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327EE20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327EE24: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327EE28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327EE2C: 388BF004  addi r4, r11, -0xffc
	ctx.r[4].s64 = ctx.r[11].s64 + -4092;
	// 8327EE30: 386ADDBC  addi r3, r10, -0x2244
	ctx.r[3].s64 = ctx.r[10].s64 + -8772;
	// 8327EE34: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327EE38: 4AFAE099  bl 0x8222ced0
	ctx.lr = 0x8327EE3C;
	sub_8222CED0(ctx, base);
	// 8327EE3C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327EE40: 38690A08  addi r3, r9, 0xa08
	ctx.r[3].s64 = ctx.r[9].s64 + 2568;
	// 8327EE44: 4BA2B0DD  bl 0x82ca9f20
	ctx.lr = 0x8327EE48;
	sub_82CA9F20(ctx, base);
	// 8327EE48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327EE4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327EE50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327EE54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327EE58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327EE58 size=64
    let mut pc: u32 = 0x8327EE58;
    'dispatch: loop {
        match pc {
            0x8327EE58 => {
    //   block [0x8327EE58..0x8327EE98)
	// 8327EE58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327EE5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327EE60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327EE64: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327EE68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327EE6C: 388BF014  addi r4, r11, -0xfec
	ctx.r[4].s64 = ctx.r[11].s64 + -4076;
	// 8327EE70: 386ADDC0  addi r3, r10, -0x2240
	ctx.r[3].s64 = ctx.r[10].s64 + -8768;
	// 8327EE74: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327EE78: 4AFAE059  bl 0x8222ced0
	ctx.lr = 0x8327EE7C;
	sub_8222CED0(ctx, base);
	// 8327EE7C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327EE80: 38690A18  addi r3, r9, 0xa18
	ctx.r[3].s64 = ctx.r[9].s64 + 2584;
	// 8327EE84: 4BA2B09D  bl 0x82ca9f20
	ctx.lr = 0x8327EE88;
	sub_82CA9F20(ctx, base);
	// 8327EE88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327EE8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327EE90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327EE94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327EE98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327EE98 size=64
    let mut pc: u32 = 0x8327EE98;
    'dispatch: loop {
        match pc {
            0x8327EE98 => {
    //   block [0x8327EE98..0x8327EED8)
	// 8327EE98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327EE9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327EEA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327EEA4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327EEA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327EEAC: 388BF028  addi r4, r11, -0xfd8
	ctx.r[4].s64 = ctx.r[11].s64 + -4056;
	// 8327EEB0: 386ADDC4  addi r3, r10, -0x223c
	ctx.r[3].s64 = ctx.r[10].s64 + -8764;
	// 8327EEB4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327EEB8: 4AFAE019  bl 0x8222ced0
	ctx.lr = 0x8327EEBC;
	sub_8222CED0(ctx, base);
	// 8327EEBC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327EEC0: 38690A28  addi r3, r9, 0xa28
	ctx.r[3].s64 = ctx.r[9].s64 + 2600;
	// 8327EEC4: 4BA2B05D  bl 0x82ca9f20
	ctx.lr = 0x8327EEC8;
	sub_82CA9F20(ctx, base);
	// 8327EEC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327EECC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327EED0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327EED4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327EED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327EED8 size=64
    let mut pc: u32 = 0x8327EED8;
    'dispatch: loop {
        match pc {
            0x8327EED8 => {
    //   block [0x8327EED8..0x8327EF18)
	// 8327EED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327EEDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327EEE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327EEE4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327EEE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327EEEC: 388BF040  addi r4, r11, -0xfc0
	ctx.r[4].s64 = ctx.r[11].s64 + -4032;
	// 8327EEF0: 386ADDC8  addi r3, r10, -0x2238
	ctx.r[3].s64 = ctx.r[10].s64 + -8760;
	// 8327EEF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327EEF8: 4AFADFD9  bl 0x8222ced0
	ctx.lr = 0x8327EEFC;
	sub_8222CED0(ctx, base);
	// 8327EEFC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327EF00: 38690A38  addi r3, r9, 0xa38
	ctx.r[3].s64 = ctx.r[9].s64 + 2616;
	// 8327EF04: 4BA2B01D  bl 0x82ca9f20
	ctx.lr = 0x8327EF08;
	sub_82CA9F20(ctx, base);
	// 8327EF08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327EF0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327EF10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327EF14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327EF18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327EF18 size=64
    let mut pc: u32 = 0x8327EF18;
    'dispatch: loop {
        match pc {
            0x8327EF18 => {
    //   block [0x8327EF18..0x8327EF58)
	// 8327EF18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327EF1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327EF20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327EF24: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327EF28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327EF2C: 388BF054  addi r4, r11, -0xfac
	ctx.r[4].s64 = ctx.r[11].s64 + -4012;
	// 8327EF30: 386ADDCC  addi r3, r10, -0x2234
	ctx.r[3].s64 = ctx.r[10].s64 + -8756;
	// 8327EF34: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327EF38: 4AFADF99  bl 0x8222ced0
	ctx.lr = 0x8327EF3C;
	sub_8222CED0(ctx, base);
	// 8327EF3C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327EF40: 38690A48  addi r3, r9, 0xa48
	ctx.r[3].s64 = ctx.r[9].s64 + 2632;
	// 8327EF44: 4BA2AFDD  bl 0x82ca9f20
	ctx.lr = 0x8327EF48;
	sub_82CA9F20(ctx, base);
	// 8327EF48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327EF4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327EF50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327EF54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327EF58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327EF58 size=64
    let mut pc: u32 = 0x8327EF58;
    'dispatch: loop {
        match pc {
            0x8327EF58 => {
    //   block [0x8327EF58..0x8327EF98)
	// 8327EF58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327EF5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327EF60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327EF64: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327EF68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327EF6C: 388BF064  addi r4, r11, -0xf9c
	ctx.r[4].s64 = ctx.r[11].s64 + -3996;
	// 8327EF70: 386ADDD0  addi r3, r10, -0x2230
	ctx.r[3].s64 = ctx.r[10].s64 + -8752;
	// 8327EF74: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327EF78: 4AFADF59  bl 0x8222ced0
	ctx.lr = 0x8327EF7C;
	sub_8222CED0(ctx, base);
	// 8327EF7C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327EF80: 38690A58  addi r3, r9, 0xa58
	ctx.r[3].s64 = ctx.r[9].s64 + 2648;
	// 8327EF84: 4BA2AF9D  bl 0x82ca9f20
	ctx.lr = 0x8327EF88;
	sub_82CA9F20(ctx, base);
	// 8327EF88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327EF8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327EF90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327EF94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327EF98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327EF98 size=64
    let mut pc: u32 = 0x8327EF98;
    'dispatch: loop {
        match pc {
            0x8327EF98 => {
    //   block [0x8327EF98..0x8327EFD8)
	// 8327EF98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327EF9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327EFA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327EFA4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327EFA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327EFAC: 388BF080  addi r4, r11, -0xf80
	ctx.r[4].s64 = ctx.r[11].s64 + -3968;
	// 8327EFB0: 386ADDD4  addi r3, r10, -0x222c
	ctx.r[3].s64 = ctx.r[10].s64 + -8748;
	// 8327EFB4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327EFB8: 4AFADF19  bl 0x8222ced0
	ctx.lr = 0x8327EFBC;
	sub_8222CED0(ctx, base);
	// 8327EFBC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327EFC0: 38690A68  addi r3, r9, 0xa68
	ctx.r[3].s64 = ctx.r[9].s64 + 2664;
	// 8327EFC4: 4BA2AF5D  bl 0x82ca9f20
	ctx.lr = 0x8327EFC8;
	sub_82CA9F20(ctx, base);
	// 8327EFC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327EFCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327EFD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327EFD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327EFD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327EFD8 size=64
    let mut pc: u32 = 0x8327EFD8;
    'dispatch: loop {
        match pc {
            0x8327EFD8 => {
    //   block [0x8327EFD8..0x8327F018)
	// 8327EFD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327EFDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327EFE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327EFE4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327EFE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327EFEC: 388BF090  addi r4, r11, -0xf70
	ctx.r[4].s64 = ctx.r[11].s64 + -3952;
	// 8327EFF0: 386ADDD8  addi r3, r10, -0x2228
	ctx.r[3].s64 = ctx.r[10].s64 + -8744;
	// 8327EFF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327EFF8: 4AFADED9  bl 0x8222ced0
	ctx.lr = 0x8327EFFC;
	sub_8222CED0(ctx, base);
	// 8327EFFC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F000: 38690A78  addi r3, r9, 0xa78
	ctx.r[3].s64 = ctx.r[9].s64 + 2680;
	// 8327F004: 4BA2AF1D  bl 0x82ca9f20
	ctx.lr = 0x8327F008;
	sub_82CA9F20(ctx, base);
	// 8327F008: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F00C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F010: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F014: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F018 size=64
    let mut pc: u32 = 0x8327F018;
    'dispatch: loop {
        match pc {
            0x8327F018 => {
    //   block [0x8327F018..0x8327F058)
	// 8327F018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F01C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F020: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F024: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F028: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F02C: 388BF0A8  addi r4, r11, -0xf58
	ctx.r[4].s64 = ctx.r[11].s64 + -3928;
	// 8327F030: 386ADDDC  addi r3, r10, -0x2224
	ctx.r[3].s64 = ctx.r[10].s64 + -8740;
	// 8327F034: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F038: 4AFADE99  bl 0x8222ced0
	ctx.lr = 0x8327F03C;
	sub_8222CED0(ctx, base);
	// 8327F03C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F040: 38690A88  addi r3, r9, 0xa88
	ctx.r[3].s64 = ctx.r[9].s64 + 2696;
	// 8327F044: 4BA2AEDD  bl 0x82ca9f20
	ctx.lr = 0x8327F048;
	sub_82CA9F20(ctx, base);
	// 8327F048: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F04C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F050: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F054: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F058 size=64
    let mut pc: u32 = 0x8327F058;
    'dispatch: loop {
        match pc {
            0x8327F058 => {
    //   block [0x8327F058..0x8327F098)
	// 8327F058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F05C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F060: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F064: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F068: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F06C: 388BF0C0  addi r4, r11, -0xf40
	ctx.r[4].s64 = ctx.r[11].s64 + -3904;
	// 8327F070: 386ADDE0  addi r3, r10, -0x2220
	ctx.r[3].s64 = ctx.r[10].s64 + -8736;
	// 8327F074: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F078: 4AFADE59  bl 0x8222ced0
	ctx.lr = 0x8327F07C;
	sub_8222CED0(ctx, base);
	// 8327F07C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F080: 38690A98  addi r3, r9, 0xa98
	ctx.r[3].s64 = ctx.r[9].s64 + 2712;
	// 8327F084: 4BA2AE9D  bl 0x82ca9f20
	ctx.lr = 0x8327F088;
	sub_82CA9F20(ctx, base);
	// 8327F088: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F08C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F090: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F094: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


