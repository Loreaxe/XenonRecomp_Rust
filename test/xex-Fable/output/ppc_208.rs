pub fn sub_832476E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832476E8 size=56
    let mut pc: u32 = 0x832476E8;
    'dispatch: loop {
        match pc {
            0x832476E8 => {
    //   block [0x832476E8..0x83247720)
	// 832476E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832476EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832476F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832476F4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832476F8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832476FC: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 83247700: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83247704: 4AFAC655  bl 0x821f3d58
	ctx.lr = 0x83247708;
	sub_821F3D58(ctx, base);
	// 83247708: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324770C: 906A7370  stw r3, 0x7370(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(29552 as u32), ctx.r[3].u32 ) };
	// 83247710: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83247714: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83247718: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324771C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83247720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83247720 size=56
    let mut pc: u32 = 0x83247720;
    'dispatch: loop {
        match pc {
            0x83247720 => {
    //   block [0x83247720..0x83247758)
	// 83247720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83247724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83247728: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324772C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83247730: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83247734: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 83247738: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324773C: 4AFAC61D  bl 0x821f3d58
	ctx.lr = 0x83247740;
	sub_821F3D58(ctx, base);
	// 83247740: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83247744: 906A7374  stw r3, 0x7374(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(29556 as u32), ctx.r[3].u32 ) };
	// 83247748: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324774C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83247750: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83247754: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83247758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83247758 size=56
    let mut pc: u32 = 0x83247758;
    'dispatch: loop {
        match pc {
            0x83247758 => {
    //   block [0x83247758..0x83247790)
	// 83247758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324775C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83247760: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83247764: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83247768: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324776C: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 83247770: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83247774: 4AFAC5E5  bl 0x821f3d58
	ctx.lr = 0x83247778;
	sub_821F3D58(ctx, base);
	// 83247778: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324777C: 906A7378  stw r3, 0x7378(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(29560 as u32), ctx.r[3].u32 ) };
	// 83247780: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83247784: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83247788: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324778C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83247790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83247790 size=56
    let mut pc: u32 = 0x83247790;
    'dispatch: loop {
        match pc {
            0x83247790 => {
    //   block [0x83247790..0x832477C8)
	// 83247790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83247794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83247798: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324779C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832477A0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832477A4: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 832477A8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832477AC: 4AFAC5AD  bl 0x821f3d58
	ctx.lr = 0x832477B0;
	sub_821F3D58(ctx, base);
	// 832477B0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 832477B4: 906A737C  stw r3, 0x737c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(29564 as u32), ctx.r[3].u32 ) };
	// 832477B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832477BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832477C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832477C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832477C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832477C8 size=56
    let mut pc: u32 = 0x832477C8;
    'dispatch: loop {
        match pc {
            0x832477C8 => {
    //   block [0x832477C8..0x83247800)
	// 832477C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832477CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832477D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832477D4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832477D8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832477DC: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 832477E0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832477E4: 4AFAC575  bl 0x821f3d58
	ctx.lr = 0x832477E8;
	sub_821F3D58(ctx, base);
	// 832477E8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 832477EC: 906A7380  stw r3, 0x7380(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(29568 as u32), ctx.r[3].u32 ) };
	// 832477F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832477F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832477F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832477FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83247800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83247800 size=56
    let mut pc: u32 = 0x83247800;
    'dispatch: loop {
        match pc {
            0x83247800 => {
    //   block [0x83247800..0x83247838)
	// 83247800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83247804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83247808: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324780C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83247810: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83247814: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 83247818: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324781C: 4AFAC53D  bl 0x821f3d58
	ctx.lr = 0x83247820;
	sub_821F3D58(ctx, base);
	// 83247820: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83247824: 906A7384  stw r3, 0x7384(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(29572 as u32), ctx.r[3].u32 ) };
	// 83247828: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324782C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83247830: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83247834: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83247838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83247838 size=56
    let mut pc: u32 = 0x83247838;
    'dispatch: loop {
        match pc {
            0x83247838 => {
    //   block [0x83247838..0x83247870)
	// 83247838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324783C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83247840: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83247844: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83247848: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324784C: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 83247850: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83247854: 4AFAC505  bl 0x821f3d58
	ctx.lr = 0x83247858;
	sub_821F3D58(ctx, base);
	// 83247858: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324785C: 906A7388  stw r3, 0x7388(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(29576 as u32), ctx.r[3].u32 ) };
	// 83247860: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83247864: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83247868: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324786C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83247870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83247870 size=56
    let mut pc: u32 = 0x83247870;
    'dispatch: loop {
        match pc {
            0x83247870 => {
    //   block [0x83247870..0x832478A8)
	// 83247870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83247874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83247878: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324787C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83247880: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83247884: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 83247888: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324788C: 4AFAC4CD  bl 0x821f3d58
	ctx.lr = 0x83247890;
	sub_821F3D58(ctx, base);
	// 83247890: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83247894: 906A738C  stw r3, 0x738c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(29580 as u32), ctx.r[3].u32 ) };
	// 83247898: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324789C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832478A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832478A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832478A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832478A8 size=56
    let mut pc: u32 = 0x832478A8;
    'dispatch: loop {
        match pc {
            0x832478A8 => {
    //   block [0x832478A8..0x832478E0)
	// 832478A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832478AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832478B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832478B4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832478B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832478BC: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 832478C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832478C4: 4AFAC495  bl 0x821f3d58
	ctx.lr = 0x832478C8;
	sub_821F3D58(ctx, base);
	// 832478C8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 832478CC: 906A7390  stw r3, 0x7390(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(29584 as u32), ctx.r[3].u32 ) };
	// 832478D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832478D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832478D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832478DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832478E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832478E0 size=56
    let mut pc: u32 = 0x832478E0;
    'dispatch: loop {
        match pc {
            0x832478E0 => {
    //   block [0x832478E0..0x83247918)
	// 832478E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832478E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832478E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832478EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832478F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832478F4: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 832478F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832478FC: 4AFAC45D  bl 0x821f3d58
	ctx.lr = 0x83247900;
	sub_821F3D58(ctx, base);
	// 83247900: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83247904: 906A7394  stw r3, 0x7394(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(29588 as u32), ctx.r[3].u32 ) };
	// 83247908: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324790C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83247910: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83247914: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83247918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83247918 size=56
    let mut pc: u32 = 0x83247918;
    'dispatch: loop {
        match pc {
            0x83247918 => {
    //   block [0x83247918..0x83247950)
	// 83247918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324791C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83247920: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83247924: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83247928: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324792C: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 83247930: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83247934: 4AFAC425  bl 0x821f3d58
	ctx.lr = 0x83247938;
	sub_821F3D58(ctx, base);
	// 83247938: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324793C: 906A7398  stw r3, 0x7398(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(29592 as u32), ctx.r[3].u32 ) };
	// 83247940: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83247944: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83247948: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324794C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83247950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83247950 size=56
    let mut pc: u32 = 0x83247950;
    'dispatch: loop {
        match pc {
            0x83247950 => {
    //   block [0x83247950..0x83247988)
	// 83247950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83247954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83247958: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324795C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83247960: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83247964: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 83247968: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324796C: 4AFAC3ED  bl 0x821f3d58
	ctx.lr = 0x83247970;
	sub_821F3D58(ctx, base);
	// 83247970: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83247974: 906A739C  stw r3, 0x739c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(29596 as u32), ctx.r[3].u32 ) };
	// 83247978: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324797C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83247980: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83247984: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83247988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83247988 size=56
    let mut pc: u32 = 0x83247988;
    'dispatch: loop {
        match pc {
            0x83247988 => {
    //   block [0x83247988..0x832479C0)
	// 83247988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324798C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83247990: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83247994: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83247998: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324799C: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 832479A0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832479A4: 4AFAC3B5  bl 0x821f3d58
	ctx.lr = 0x832479A8;
	sub_821F3D58(ctx, base);
	// 832479A8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 832479AC: 906A73A0  stw r3, 0x73a0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(29600 as u32), ctx.r[3].u32 ) };
	// 832479B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832479B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832479B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832479BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832479C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832479C0 size=56
    let mut pc: u32 = 0x832479C0;
    'dispatch: loop {
        match pc {
            0x832479C0 => {
    //   block [0x832479C0..0x832479F8)
	// 832479C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832479C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832479C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832479CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832479D0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832479D4: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 832479D8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832479DC: 4AFAC37D  bl 0x821f3d58
	ctx.lr = 0x832479E0;
	sub_821F3D58(ctx, base);
	// 832479E0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 832479E4: 906A73A4  stw r3, 0x73a4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(29604 as u32), ctx.r[3].u32 ) };
	// 832479E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832479EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832479F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832479F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832479F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832479F8 size=56
    let mut pc: u32 = 0x832479F8;
    'dispatch: loop {
        match pc {
            0x832479F8 => {
    //   block [0x832479F8..0x83247A30)
	// 832479F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832479FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83247A00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83247A04: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83247A08: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83247A0C: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 83247A10: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83247A14: 4AFAC345  bl 0x821f3d58
	ctx.lr = 0x83247A18;
	sub_821F3D58(ctx, base);
	// 83247A18: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83247A1C: 906A73A8  stw r3, 0x73a8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(29608 as u32), ctx.r[3].u32 ) };
	// 83247A20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83247A24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83247A28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83247A2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83247A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83247A30 size=56
    let mut pc: u32 = 0x83247A30;
    'dispatch: loop {
        match pc {
            0x83247A30 => {
    //   block [0x83247A30..0x83247A68)
	// 83247A30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83247A34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83247A38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83247A3C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83247A40: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83247A44: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 83247A48: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83247A4C: 4AFAC30D  bl 0x821f3d58
	ctx.lr = 0x83247A50;
	sub_821F3D58(ctx, base);
	// 83247A50: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83247A54: 906A73AC  stw r3, 0x73ac(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(29612 as u32), ctx.r[3].u32 ) };
	// 83247A58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83247A5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83247A60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83247A64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83247A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83247A68 size=56
    let mut pc: u32 = 0x83247A68;
    'dispatch: loop {
        match pc {
            0x83247A68 => {
    //   block [0x83247A68..0x83247AA0)
	// 83247A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83247A6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83247A70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83247A74: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83247A78: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83247A7C: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 83247A80: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83247A84: 4AFAC2D5  bl 0x821f3d58
	ctx.lr = 0x83247A88;
	sub_821F3D58(ctx, base);
	// 83247A88: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83247A8C: 906A73B0  stw r3, 0x73b0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(29616 as u32), ctx.r[3].u32 ) };
	// 83247A90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83247A94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83247A98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83247A9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83247AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83247AA0 size=64
    let mut pc: u32 = 0x83247AA0;
    'dispatch: loop {
        match pc {
            0x83247AA0 => {
    //   block [0x83247AA0..0x83247AE0)
	// 83247AA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83247AA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83247AA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83247AAC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83247AB0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83247AB4: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 83247AB8: 386A73B4  addi r3, r10, 0x73b4
	ctx.r[3].s64 = ctx.r[10].s64 + 29620;
	// 83247ABC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83247AC0: 4AFE5411  bl 0x8222ced0
	ctx.lr = 0x83247AC4;
	sub_8222CED0(ctx, base);
	// 83247AC4: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 83247AC8: 386973C0  addi r3, r9, 0x73c0
	ctx.r[3].s64 = ctx.r[9].s64 + 29632;
	// 83247ACC: 4BA62455  bl 0x82ca9f20
	ctx.lr = 0x83247AD0;
	sub_82CA9F20(ctx, base);
	// 83247AD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83247AD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83247AD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83247ADC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83247AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83247AE0 size=56
    let mut pc: u32 = 0x83247AE0;
    'dispatch: loop {
        match pc {
            0x83247AE0 => {
    //   block [0x83247AE0..0x83247B18)
	// 83247AE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83247AE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83247AE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83247AEC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83247AF0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83247AF4: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 83247AF8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83247AFC: 4AFAC25D  bl 0x821f3d58
	ctx.lr = 0x83247B00;
	sub_821F3D58(ctx, base);
	// 83247B00: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83247B04: 906A73B8  stw r3, 0x73b8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(29624 as u32), ctx.r[3].u32 ) };
	// 83247B08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83247B0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83247B10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83247B14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83247B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83247B18 size=56
    let mut pc: u32 = 0x83247B18;
    'dispatch: loop {
        match pc {
            0x83247B18 => {
    //   block [0x83247B18..0x83247B50)
	// 83247B18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83247B1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83247B20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83247B24: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83247B28: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83247B2C: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 83247B30: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83247B34: 4AFAC225  bl 0x821f3d58
	ctx.lr = 0x83247B38;
	sub_821F3D58(ctx, base);
	// 83247B38: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83247B3C: 906A73BC  stw r3, 0x73bc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(29628 as u32), ctx.r[3].u32 ) };
	// 83247B40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83247B44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83247B48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83247B4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83247B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83247B50 size=56
    let mut pc: u32 = 0x83247B50;
    'dispatch: loop {
        match pc {
            0x83247B50 => {
    //   block [0x83247B50..0x83247B88)
	// 83247B50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83247B54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83247B58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83247B5C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83247B60: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83247B64: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 83247B68: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83247B6C: 4AFAC1ED  bl 0x821f3d58
	ctx.lr = 0x83247B70;
	sub_821F3D58(ctx, base);
	// 83247B70: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83247B74: 906A73C0  stw r3, 0x73c0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(29632 as u32), ctx.r[3].u32 ) };
	// 83247B78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83247B7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83247B80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83247B84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83247B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83247B88 size=56
    let mut pc: u32 = 0x83247B88;
    'dispatch: loop {
        match pc {
            0x83247B88 => {
    //   block [0x83247B88..0x83247BC0)
	// 83247B88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83247B8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83247B90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83247B94: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83247B98: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83247B9C: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 83247BA0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83247BA4: 4AFAC1B5  bl 0x821f3d58
	ctx.lr = 0x83247BA8;
	sub_821F3D58(ctx, base);
	// 83247BA8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83247BAC: 906A73C4  stw r3, 0x73c4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(29636 as u32), ctx.r[3].u32 ) };
	// 83247BB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83247BB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83247BB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83247BBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83247BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83247BC0 size=56
    let mut pc: u32 = 0x83247BC0;
    'dispatch: loop {
        match pc {
            0x83247BC0 => {
    //   block [0x83247BC0..0x83247BF8)
	// 83247BC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83247BC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83247BC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83247BCC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83247BD0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83247BD4: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 83247BD8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83247BDC: 4AFAC17D  bl 0x821f3d58
	ctx.lr = 0x83247BE0;
	sub_821F3D58(ctx, base);
	// 83247BE0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83247BE4: 906A73C8  stw r3, 0x73c8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(29640 as u32), ctx.r[3].u32 ) };
	// 83247BE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83247BEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83247BF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83247BF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83247BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83247BF8 size=56
    let mut pc: u32 = 0x83247BF8;
    'dispatch: loop {
        match pc {
            0x83247BF8 => {
    //   block [0x83247BF8..0x83247C30)
	// 83247BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83247BFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83247C00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83247C04: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83247C08: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83247C0C: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 83247C10: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83247C14: 4AFAC145  bl 0x821f3d58
	ctx.lr = 0x83247C18;
	sub_821F3D58(ctx, base);
	// 83247C18: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83247C1C: 906A73CC  stw r3, 0x73cc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(29644 as u32), ctx.r[3].u32 ) };
	// 83247C20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83247C24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83247C28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83247C2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83247C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83247C30 size=56
    let mut pc: u32 = 0x83247C30;
    'dispatch: loop {
        match pc {
            0x83247C30 => {
    //   block [0x83247C30..0x83247C68)
	// 83247C30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83247C34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83247C38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83247C3C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83247C40: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83247C44: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 83247C48: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83247C4C: 4AFAC10D  bl 0x821f3d58
	ctx.lr = 0x83247C50;
	sub_821F3D58(ctx, base);
	// 83247C50: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83247C54: 906A73D0  stw r3, 0x73d0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(29648 as u32), ctx.r[3].u32 ) };
	// 83247C58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83247C5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83247C60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83247C64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83247C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83247C68 size=56
    let mut pc: u32 = 0x83247C68;
    'dispatch: loop {
        match pc {
            0x83247C68 => {
    //   block [0x83247C68..0x83247CA0)
	// 83247C68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83247C6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83247C70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83247C74: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83247C78: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83247C7C: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 83247C80: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83247C84: 4AFAC0D5  bl 0x821f3d58
	ctx.lr = 0x83247C88;
	sub_821F3D58(ctx, base);
	// 83247C88: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83247C8C: 906A73D4  stw r3, 0x73d4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(29652 as u32), ctx.r[3].u32 ) };
	// 83247C90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83247C94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83247C98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83247C9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83247CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83247CA0 size=56
    let mut pc: u32 = 0x83247CA0;
    'dispatch: loop {
        match pc {
            0x83247CA0 => {
    //   block [0x83247CA0..0x83247CD8)
	// 83247CA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83247CA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83247CA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83247CAC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83247CB0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83247CB4: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 83247CB8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83247CBC: 4AFAC09D  bl 0x821f3d58
	ctx.lr = 0x83247CC0;
	sub_821F3D58(ctx, base);
	// 83247CC0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83247CC4: 906A73D8  stw r3, 0x73d8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(29656 as u32), ctx.r[3].u32 ) };
	// 83247CC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83247CCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83247CD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83247CD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83247CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83247CD8 size=56
    let mut pc: u32 = 0x83247CD8;
    'dispatch: loop {
        match pc {
            0x83247CD8 => {
    //   block [0x83247CD8..0x83247D10)
	// 83247CD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83247CDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83247CE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83247CE4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83247CE8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83247CEC: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 83247CF0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83247CF4: 4AFAC065  bl 0x821f3d58
	ctx.lr = 0x83247CF8;
	sub_821F3D58(ctx, base);
	// 83247CF8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83247CFC: 906A73DC  stw r3, 0x73dc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(29660 as u32), ctx.r[3].u32 ) };
	// 83247D00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83247D04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83247D08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83247D0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83247D10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83247D10 size=56
    let mut pc: u32 = 0x83247D10;
    'dispatch: loop {
        match pc {
            0x83247D10 => {
    //   block [0x83247D10..0x83247D48)
	// 83247D10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83247D14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83247D18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83247D1C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83247D20: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83247D24: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 83247D28: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83247D2C: 4AFAC02D  bl 0x821f3d58
	ctx.lr = 0x83247D30;
	sub_821F3D58(ctx, base);
	// 83247D30: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83247D34: 906A73E0  stw r3, 0x73e0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(29664 as u32), ctx.r[3].u32 ) };
	// 83247D38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83247D3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83247D40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83247D44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83247D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83247D48 size=56
    let mut pc: u32 = 0x83247D48;
    'dispatch: loop {
        match pc {
            0x83247D48 => {
    //   block [0x83247D48..0x83247D80)
	// 83247D48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83247D4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83247D50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83247D54: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83247D58: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83247D5C: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 83247D60: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83247D64: 4AFABFF5  bl 0x821f3d58
	ctx.lr = 0x83247D68;
	sub_821F3D58(ctx, base);
	// 83247D68: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83247D6C: 906A73E4  stw r3, 0x73e4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(29668 as u32), ctx.r[3].u32 ) };
	// 83247D70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83247D74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83247D78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83247D7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83247D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83247D80 size=56
    let mut pc: u32 = 0x83247D80;
    'dispatch: loop {
        match pc {
            0x83247D80 => {
    //   block [0x83247D80..0x83247DB8)
	// 83247D80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83247D84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83247D88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83247D8C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83247D90: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83247D94: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 83247D98: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83247D9C: 4AFABFBD  bl 0x821f3d58
	ctx.lr = 0x83247DA0;
	sub_821F3D58(ctx, base);
	// 83247DA0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83247DA4: 906A73E8  stw r3, 0x73e8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(29672 as u32), ctx.r[3].u32 ) };
	// 83247DA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83247DAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83247DB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83247DB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83247DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83247DB8 size=56
    let mut pc: u32 = 0x83247DB8;
    'dispatch: loop {
        match pc {
            0x83247DB8 => {
    //   block [0x83247DB8..0x83247DF0)
	// 83247DB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83247DBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83247DC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83247DC4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83247DC8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83247DCC: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 83247DD0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83247DD4: 4AFABF85  bl 0x821f3d58
	ctx.lr = 0x83247DD8;
	sub_821F3D58(ctx, base);
	// 83247DD8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83247DDC: 906A73EC  stw r3, 0x73ec(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(29676 as u32), ctx.r[3].u32 ) };
	// 83247DE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83247DE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83247DE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83247DEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83247DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83247DF0 size=56
    let mut pc: u32 = 0x83247DF0;
    'dispatch: loop {
        match pc {
            0x83247DF0 => {
    //   block [0x83247DF0..0x83247E28)
	// 83247DF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83247DF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83247DF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83247DFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83247E00: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83247E04: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 83247E08: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83247E0C: 4AFABF4D  bl 0x821f3d58
	ctx.lr = 0x83247E10;
	sub_821F3D58(ctx, base);
	// 83247E10: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83247E14: 906A73F0  stw r3, 0x73f0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(29680 as u32), ctx.r[3].u32 ) };
	// 83247E18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83247E1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83247E20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83247E24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83247E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83247E28 size=56
    let mut pc: u32 = 0x83247E28;
    'dispatch: loop {
        match pc {
            0x83247E28 => {
    //   block [0x83247E28..0x83247E60)
	// 83247E28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83247E2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83247E30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83247E34: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83247E38: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83247E3C: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 83247E40: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83247E44: 4AFABF15  bl 0x821f3d58
	ctx.lr = 0x83247E48;
	sub_821F3D58(ctx, base);
	// 83247E48: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83247E4C: 906A73F4  stw r3, 0x73f4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(29684 as u32), ctx.r[3].u32 ) };
	// 83247E50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83247E54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83247E58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83247E5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83247E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83247E60 size=56
    let mut pc: u32 = 0x83247E60;
    'dispatch: loop {
        match pc {
            0x83247E60 => {
    //   block [0x83247E60..0x83247E98)
	// 83247E60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83247E64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83247E68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83247E6C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83247E70: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83247E74: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 83247E78: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83247E7C: 4AFABEDD  bl 0x821f3d58
	ctx.lr = 0x83247E80;
	sub_821F3D58(ctx, base);
	// 83247E80: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83247E84: 906A73F8  stw r3, 0x73f8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(29688 as u32), ctx.r[3].u32 ) };
	// 83247E88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83247E8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83247E90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83247E94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83247E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83247E98 size=56
    let mut pc: u32 = 0x83247E98;
    'dispatch: loop {
        match pc {
            0x83247E98 => {
    //   block [0x83247E98..0x83247ED0)
	// 83247E98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83247E9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83247EA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83247EA4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83247EA8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83247EAC: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 83247EB0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83247EB4: 4AFABEA5  bl 0x821f3d58
	ctx.lr = 0x83247EB8;
	sub_821F3D58(ctx, base);
	// 83247EB8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83247EBC: 906A73FC  stw r3, 0x73fc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(29692 as u32), ctx.r[3].u32 ) };
	// 83247EC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83247EC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83247EC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83247ECC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83247ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83247ED0 size=56
    let mut pc: u32 = 0x83247ED0;
    'dispatch: loop {
        match pc {
            0x83247ED0 => {
    //   block [0x83247ED0..0x83247F08)
	// 83247ED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83247ED4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83247ED8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83247EDC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83247EE0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83247EE4: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 83247EE8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83247EEC: 4AFABE6D  bl 0x821f3d58
	ctx.lr = 0x83247EF0;
	sub_821F3D58(ctx, base);
	// 83247EF0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83247EF4: 906A7400  stw r3, 0x7400(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(29696 as u32), ctx.r[3].u32 ) };
	// 83247EF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83247EFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83247F00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83247F04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83247F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83247F08 size=56
    let mut pc: u32 = 0x83247F08;
    'dispatch: loop {
        match pc {
            0x83247F08 => {
    //   block [0x83247F08..0x83247F40)
	// 83247F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83247F0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83247F10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83247F14: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83247F18: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83247F1C: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 83247F20: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83247F24: 4AFABE35  bl 0x821f3d58
	ctx.lr = 0x83247F28;
	sub_821F3D58(ctx, base);
	// 83247F28: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83247F2C: 906A7404  stw r3, 0x7404(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(29700 as u32), ctx.r[3].u32 ) };
	// 83247F30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83247F34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83247F38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83247F3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83247F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83247F40 size=56
    let mut pc: u32 = 0x83247F40;
    'dispatch: loop {
        match pc {
            0x83247F40 => {
    //   block [0x83247F40..0x83247F78)
	// 83247F40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83247F44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83247F48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83247F4C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83247F50: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83247F54: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 83247F58: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83247F5C: 4AFABDFD  bl 0x821f3d58
	ctx.lr = 0x83247F60;
	sub_821F3D58(ctx, base);
	// 83247F60: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83247F64: 906A7408  stw r3, 0x7408(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(29704 as u32), ctx.r[3].u32 ) };
	// 83247F68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83247F6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83247F70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83247F74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83247F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83247F78 size=52
    let mut pc: u32 = 0x83247F78;
    'dispatch: loop {
        match pc {
            0x83247F78 => {
    //   block [0x83247F78..0x83247FAC)
	// 83247F78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83247F7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83247F80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83247F84: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 83247F88: 386B740C  addi r3, r11, 0x740c
	ctx.r[3].s64 = ctx.r[11].s64 + 29708;
	// 83247F8C: 48071CF9  bl 0x832b9c84
	ctx.lr = 0x83247F90;
	// extern call 0x832B9C84 → crate::xboxkrnl::RtlInitializeCriticalSection
	crate::xboxkrnl::RtlInitializeCriticalSection(ctx, base);
	// 83247F90: 3D40832A  lis r10, -0x7cd6
	ctx.r[10].s64 = -2094399488;
	// 83247F94: 386A73D0  addi r3, r10, 0x73d0
	ctx.r[3].s64 = ctx.r[10].s64 + 29648;
	// 83247F98: 4BA61F89  bl 0x82ca9f20
	ctx.lr = 0x83247F9C;
	sub_82CA9F20(ctx, base);
	// 83247F9C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83247FA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83247FA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83247FA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83247FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83247FB0 size=64
    let mut pc: u32 = 0x83247FB0;
    'dispatch: loop {
        match pc {
            0x83247FB0 => {
    //   block [0x83247FB0..0x83247FF0)
	// 83247FB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83247FB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83247FB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83247FBC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83247FC0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83247FC4: 388B455C  addi r4, r11, 0x455c
	ctx.r[4].s64 = ctx.r[11].s64 + 17756;
	// 83247FC8: 386A7428  addi r3, r10, 0x7428
	ctx.r[3].s64 = ctx.r[10].s64 + 29736;
	// 83247FCC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83247FD0: 4AFE4F01  bl 0x8222ced0
	ctx.lr = 0x83247FD4;
	sub_8222CED0(ctx, base);
	// 83247FD4: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 83247FD8: 386973D8  addi r3, r9, 0x73d8
	ctx.r[3].s64 = ctx.r[9].s64 + 29656;
	// 83247FDC: 4BA61F45  bl 0x82ca9f20
	ctx.lr = 0x83247FE0;
	sub_82CA9F20(ctx, base);
	// 83247FE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83247FE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83247FE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83247FEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83247FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83247FF0 size=64
    let mut pc: u32 = 0x83247FF0;
    'dispatch: loop {
        match pc {
            0x83247FF0 => {
    //   block [0x83247FF0..0x83248030)
	// 83247FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83247FF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83247FF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83247FFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83248000: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83248004: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83248008: 386A742C  addi r3, r10, 0x742c
	ctx.r[3].s64 = ctx.r[10].s64 + 29740;
	// 8324800C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83248010: 4AFE4EC1  bl 0x8222ced0
	ctx.lr = 0x83248014;
	sub_8222CED0(ctx, base);
	// 83248014: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 83248018: 386973E8  addi r3, r9, 0x73e8
	ctx.r[3].s64 = ctx.r[9].s64 + 29672;
	// 8324801C: 4BA61F05  bl 0x82ca9f20
	ctx.lr = 0x83248020;
	sub_82CA9F20(ctx, base);
	// 83248020: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83248024: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83248028: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324802C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83248030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83248030 size=64
    let mut pc: u32 = 0x83248030;
    'dispatch: loop {
        match pc {
            0x83248030 => {
    //   block [0x83248030..0x83248070)
	// 83248030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83248034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83248038: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324803C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83248040: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83248044: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83248048: 386A7430  addi r3, r10, 0x7430
	ctx.r[3].s64 = ctx.r[10].s64 + 29744;
	// 8324804C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83248050: 4AFE4E81  bl 0x8222ced0
	ctx.lr = 0x83248054;
	sub_8222CED0(ctx, base);
	// 83248054: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 83248058: 386973F8  addi r3, r9, 0x73f8
	ctx.r[3].s64 = ctx.r[9].s64 + 29688;
	// 8324805C: 4BA61EC5  bl 0x82ca9f20
	ctx.lr = 0x83248060;
	sub_82CA9F20(ctx, base);
	// 83248060: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83248064: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83248068: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324806C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83248070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83248070 size=64
    let mut pc: u32 = 0x83248070;
    'dispatch: loop {
        match pc {
            0x83248070 => {
    //   block [0x83248070..0x832480B0)
	// 83248070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83248074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83248078: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324807C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83248080: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83248084: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83248088: 386A7434  addi r3, r10, 0x7434
	ctx.r[3].s64 = ctx.r[10].s64 + 29748;
	// 8324808C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83248090: 4AFE4E41  bl 0x8222ced0
	ctx.lr = 0x83248094;
	sub_8222CED0(ctx, base);
	// 83248094: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 83248098: 38697408  addi r3, r9, 0x7408
	ctx.r[3].s64 = ctx.r[9].s64 + 29704;
	// 8324809C: 4BA61E85  bl 0x82ca9f20
	ctx.lr = 0x832480A0;
	sub_82CA9F20(ctx, base);
	// 832480A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832480A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832480A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832480AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832480B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832480B0 size=64
    let mut pc: u32 = 0x832480B0;
    'dispatch: loop {
        match pc {
            0x832480B0 => {
    //   block [0x832480B0..0x832480F0)
	// 832480B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832480B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832480B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832480BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832480C0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 832480C4: 388B4888  addi r4, r11, 0x4888
	ctx.r[4].s64 = ctx.r[11].s64 + 18568;
	// 832480C8: 386A7438  addi r3, r10, 0x7438
	ctx.r[3].s64 = ctx.r[10].s64 + 29752;
	// 832480CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832480D0: 4AFE4E01  bl 0x8222ced0
	ctx.lr = 0x832480D4;
	sub_8222CED0(ctx, base);
	// 832480D4: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 832480D8: 38697418  addi r3, r9, 0x7418
	ctx.r[3].s64 = ctx.r[9].s64 + 29720;
	// 832480DC: 4BA61E45  bl 0x82ca9f20
	ctx.lr = 0x832480E0;
	sub_82CA9F20(ctx, base);
	// 832480E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832480E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832480E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832480EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832480F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832480F0 size=64
    let mut pc: u32 = 0x832480F0;
    'dispatch: loop {
        match pc {
            0x832480F0 => {
    //   block [0x832480F0..0x83248130)
	// 832480F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832480F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832480F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832480FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83248100: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83248104: 388B49A8  addi r4, r11, 0x49a8
	ctx.r[4].s64 = ctx.r[11].s64 + 18856;
	// 83248108: 386A743C  addi r3, r10, 0x743c
	ctx.r[3].s64 = ctx.r[10].s64 + 29756;
	// 8324810C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83248110: 4AFE4DC1  bl 0x8222ced0
	ctx.lr = 0x83248114;
	sub_8222CED0(ctx, base);
	// 83248114: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 83248118: 38697428  addi r3, r9, 0x7428
	ctx.r[3].s64 = ctx.r[9].s64 + 29736;
	// 8324811C: 4BA61E05  bl 0x82ca9f20
	ctx.lr = 0x83248120;
	sub_82CA9F20(ctx, base);
	// 83248120: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83248124: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83248128: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324812C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83248130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83248130 size=64
    let mut pc: u32 = 0x83248130;
    'dispatch: loop {
        match pc {
            0x83248130 => {
    //   block [0x83248130..0x83248170)
	// 83248130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83248134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83248138: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324813C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83248140: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83248144: 388B49C4  addi r4, r11, 0x49c4
	ctx.r[4].s64 = ctx.r[11].s64 + 18884;
	// 83248148: 386A7440  addi r3, r10, 0x7440
	ctx.r[3].s64 = ctx.r[10].s64 + 29760;
	// 8324814C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83248150: 4AFE4D81  bl 0x8222ced0
	ctx.lr = 0x83248154;
	sub_8222CED0(ctx, base);
	// 83248154: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 83248158: 38697438  addi r3, r9, 0x7438
	ctx.r[3].s64 = ctx.r[9].s64 + 29752;
	// 8324815C: 4BA61DC5  bl 0x82ca9f20
	ctx.lr = 0x83248160;
	sub_82CA9F20(ctx, base);
	// 83248160: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83248164: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83248168: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324816C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83248170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83248170 size=64
    let mut pc: u32 = 0x83248170;
    'dispatch: loop {
        match pc {
            0x83248170 => {
    //   block [0x83248170..0x832481B0)
	// 83248170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83248174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83248178: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324817C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83248180: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83248184: 388B49D8  addi r4, r11, 0x49d8
	ctx.r[4].s64 = ctx.r[11].s64 + 18904;
	// 83248188: 386A7444  addi r3, r10, 0x7444
	ctx.r[3].s64 = ctx.r[10].s64 + 29764;
	// 8324818C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83248190: 4AFE4D41  bl 0x8222ced0
	ctx.lr = 0x83248194;
	sub_8222CED0(ctx, base);
	// 83248194: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 83248198: 38697448  addi r3, r9, 0x7448
	ctx.r[3].s64 = ctx.r[9].s64 + 29768;
	// 8324819C: 4BA61D85  bl 0x82ca9f20
	ctx.lr = 0x832481A0;
	sub_82CA9F20(ctx, base);
	// 832481A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832481A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832481A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832481AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832481B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832481B0 size=64
    let mut pc: u32 = 0x832481B0;
    'dispatch: loop {
        match pc {
            0x832481B0 => {
    //   block [0x832481B0..0x832481F0)
	// 832481B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832481B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832481B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832481BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832481C0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 832481C4: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 832481C8: 386A7448  addi r3, r10, 0x7448
	ctx.r[3].s64 = ctx.r[10].s64 + 29768;
	// 832481CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832481D0: 4AFE4D01  bl 0x8222ced0
	ctx.lr = 0x832481D4;
	sub_8222CED0(ctx, base);
	// 832481D4: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 832481D8: 38697458  addi r3, r9, 0x7458
	ctx.r[3].s64 = ctx.r[9].s64 + 29784;
	// 832481DC: 4BA61D45  bl 0x82ca9f20
	ctx.lr = 0x832481E0;
	sub_82CA9F20(ctx, base);
	// 832481E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832481E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832481E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832481EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832481F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832481F0 size=64
    let mut pc: u32 = 0x832481F0;
    'dispatch: loop {
        match pc {
            0x832481F0 => {
    //   block [0x832481F0..0x83248230)
	// 832481F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832481F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832481F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832481FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83248200: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83248204: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83248208: 386A744C  addi r3, r10, 0x744c
	ctx.r[3].s64 = ctx.r[10].s64 + 29772;
	// 8324820C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83248210: 4AFE4CC1  bl 0x8222ced0
	ctx.lr = 0x83248214;
	sub_8222CED0(ctx, base);
	// 83248214: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 83248218: 38697468  addi r3, r9, 0x7468
	ctx.r[3].s64 = ctx.r[9].s64 + 29800;
	// 8324821C: 4BA61D05  bl 0x82ca9f20
	ctx.lr = 0x83248220;
	sub_82CA9F20(ctx, base);
	// 83248220: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83248224: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83248228: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324822C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83248230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83248230 size=64
    let mut pc: u32 = 0x83248230;
    'dispatch: loop {
        match pc {
            0x83248230 => {
    //   block [0x83248230..0x83248270)
	// 83248230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83248234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83248238: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324823C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83248240: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83248244: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83248248: 386A7450  addi r3, r10, 0x7450
	ctx.r[3].s64 = ctx.r[10].s64 + 29776;
	// 8324824C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83248250: 4AFE4C81  bl 0x8222ced0
	ctx.lr = 0x83248254;
	sub_8222CED0(ctx, base);
	// 83248254: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 83248258: 38697478  addi r3, r9, 0x7478
	ctx.r[3].s64 = ctx.r[9].s64 + 29816;
	// 8324825C: 4BA61CC5  bl 0x82ca9f20
	ctx.lr = 0x83248260;
	sub_82CA9F20(ctx, base);
	// 83248260: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83248264: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83248268: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324826C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83248270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83248270 size=64
    let mut pc: u32 = 0x83248270;
    'dispatch: loop {
        match pc {
            0x83248270 => {
    //   block [0x83248270..0x832482B0)
	// 83248270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83248274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83248278: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324827C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83248280: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83248284: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 83248288: 386A7454  addi r3, r10, 0x7454
	ctx.r[3].s64 = ctx.r[10].s64 + 29780;
	// 8324828C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83248290: 4AFE4C41  bl 0x8222ced0
	ctx.lr = 0x83248294;
	sub_8222CED0(ctx, base);
	// 83248294: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 83248298: 38697488  addi r3, r9, 0x7488
	ctx.r[3].s64 = ctx.r[9].s64 + 29832;
	// 8324829C: 4BA61C85  bl 0x82ca9f20
	ctx.lr = 0x832482A0;
	sub_82CA9F20(ctx, base);
	// 832482A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832482A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832482A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832482AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832482B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832482B0 size=20
    let mut pc: u32 = 0x832482B0;
    'dispatch: loop {
        match pc {
            0x832482B0 => {
    //   block [0x832482B0..0x832482C4)
	// 832482B0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832482B4: 1000038C  vspltisw v0, 0
	for i in 0..4 {
		ctx.v[0].u32[i] = 0;
	}
	// 832482B8: 394B7460  addi r10, r11, 0x7460
	ctx.r[10].s64 = ctx.r[11].s64 + 29792;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832482C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832482C8 size=20
    let mut pc: u32 = 0x832482C8;
    'dispatch: loop {
        match pc {
            0x832482C8 => {
    //   block [0x832482C8..0x832482DC)
	// 832482C8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832482CC: 1000038C  vspltisw v0, 0
	for i in 0..4 {
		ctx.v[0].u32[i] = 0;
	}
	// 832482D0: 394B7470  addi r10, r11, 0x7470
	ctx.r[10].s64 = ctx.r[11].s64 + 29808;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832482E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x832482E0 size=56
    let mut pc: u32 = 0x832482E0;
    'dispatch: loop {
        match pc {
            0x832482E0 => {
    //   block [0x832482E0..0x83248318)
	// 832482E0: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832482E4: 3941FFF0  addi r10, r1, -0x10
	ctx.r[10].s64 = ctx.r[1].s64 + -16;
	// 832482E8: 3D20820A  lis r9, -0x7df6
	ctx.r[9].s64 = -2113273856;
	// 832482EC: 3D008349  lis r8, -0x7cb7
	ctx.r[8].s64 = -2092367872;
	// 832482F0: 38E99160  addi r7, r9, -0x6ea0
	ctx.r[7].s64 = ctx.r[9].s64 + -28320;
	// 832482F4: C00B9490  lfs f0, -0x6b70(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832482F8: 38C87480  addi r6, r8, 0x7480
	ctx.r[6].s64 = ctx.r[8].s64 + 29824;
	// 832482FC: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83248318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83248318 size=56
    let mut pc: u32 = 0x83248318;
    'dispatch: loop {
        match pc {
            0x83248318 => {
    //   block [0x83248318..0x83248350)
	// 83248318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324831C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83248320: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83248324: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83248328: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324832C: 386B4CEC  addi r3, r11, 0x4cec
	ctx.r[3].s64 = ctx.r[11].s64 + 19692;
	// 83248330: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83248334: 4AFABA25  bl 0x821f3d58
	ctx.lr = 0x83248338;
	sub_821F3D58(ctx, base);
	// 83248338: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324833C: 906A7458  stw r3, 0x7458(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(29784 as u32), ctx.r[3].u32 ) };
	// 83248340: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83248344: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83248348: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324834C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83248350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83248350 size=12
    let mut pc: u32 = 0x83248350;
    'dispatch: loop {
        match pc {
            0x83248350 => {
    //   block [0x83248350..0x8324835C)
	// 83248350: 3D60832A  lis r11, -0x7cd6
	ctx.r[11].s64 = -2094399488;
	// 83248354: 386B7498  addi r3, r11, 0x7498
	ctx.r[3].s64 = ctx.r[11].s64 + 29848;
	// 83248358: 4BA61BC8  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83248360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83248360 size=12
    let mut pc: u32 = 0x83248360;
    'dispatch: loop {
        match pc {
            0x83248360 => {
    //   block [0x83248360..0x8324836C)
	// 83248360: 3D60832A  lis r11, -0x7cd6
	ctx.r[11].s64 = -2094399488;
	// 83248364: 386B74A8  addi r3, r11, 0x74a8
	ctx.r[3].s64 = ctx.r[11].s64 + 29864;
	// 83248368: 4BA61BB8  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83248370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83248370 size=12
    let mut pc: u32 = 0x83248370;
    'dispatch: loop {
        match pc {
            0x83248370 => {
    //   block [0x83248370..0x8324837C)
	// 83248370: 3D60832A  lis r11, -0x7cd6
	ctx.r[11].s64 = -2094399488;
	// 83248374: 386B7510  addi r3, r11, 0x7510
	ctx.r[3].s64 = ctx.r[11].s64 + 29968;
	// 83248378: 4BA61BA8  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83248380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83248380 size=12
    let mut pc: u32 = 0x83248380;
    'dispatch: loop {
        match pc {
            0x83248380 => {
    //   block [0x83248380..0x8324838C)
	// 83248380: 3D60832A  lis r11, -0x7cd6
	ctx.r[11].s64 = -2094399488;
	// 83248384: 386B7578  addi r3, r11, 0x7578
	ctx.r[3].s64 = ctx.r[11].s64 + 30072;
	// 83248388: 4BA61B98  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83248390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83248390 size=56
    let mut pc: u32 = 0x83248390;
    'dispatch: loop {
        match pc {
            0x83248390 => {
    //   block [0x83248390..0x832483C8)
	// 83248390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83248394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83248398: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324839C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832483A0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832483A4: 386B4CFC  addi r3, r11, 0x4cfc
	ctx.r[3].s64 = ctx.r[11].s64 + 19708;
	// 832483A8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832483AC: 4AFAB9AD  bl 0x821f3d58
	ctx.lr = 0x832483B0;
	sub_821F3D58(ctx, base);
	// 832483B0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 832483B4: 906A745C  stw r3, 0x745c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(29788 as u32), ctx.r[3].u32 ) };
	// 832483B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832483BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832483C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832483C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832483C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832483C8 size=64
    let mut pc: u32 = 0x832483C8;
    'dispatch: loop {
        match pc {
            0x832483C8 => {
    //   block [0x832483C8..0x83248408)
	// 832483C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832483CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832483D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832483D4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832483D8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 832483DC: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 832483E0: 386A74C8  addi r3, r10, 0x74c8
	ctx.r[3].s64 = ctx.r[10].s64 + 29896;
	// 832483E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832483E8: 4AFE4AE9  bl 0x8222ced0
	ctx.lr = 0x832483EC;
	sub_8222CED0(ctx, base);
	// 832483EC: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 832483F0: 38697600  addi r3, r9, 0x7600
	ctx.r[3].s64 = ctx.r[9].s64 + 30208;
	// 832483F4: 4BA61B2D  bl 0x82ca9f20
	ctx.lr = 0x832483F8;
	sub_82CA9F20(ctx, base);
	// 832483F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832483FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83248400: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83248404: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83248408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83248408 size=64
    let mut pc: u32 = 0x83248408;
    'dispatch: loop {
        match pc {
            0x83248408 => {
    //   block [0x83248408..0x83248448)
	// 83248408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324840C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83248410: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83248414: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83248418: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324841C: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83248420: 386A74CC  addi r3, r10, 0x74cc
	ctx.r[3].s64 = ctx.r[10].s64 + 29900;
	// 83248424: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83248428: 4AFE4AA9  bl 0x8222ced0
	ctx.lr = 0x8324842C;
	sub_8222CED0(ctx, base);
	// 8324842C: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 83248430: 38697610  addi r3, r9, 0x7610
	ctx.r[3].s64 = ctx.r[9].s64 + 30224;
	// 83248434: 4BA61AED  bl 0x82ca9f20
	ctx.lr = 0x83248438;
	sub_82CA9F20(ctx, base);
	// 83248438: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324843C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83248440: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83248444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83248448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83248448 size=64
    let mut pc: u32 = 0x83248448;
    'dispatch: loop {
        match pc {
            0x83248448 => {
    //   block [0x83248448..0x83248488)
	// 83248448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324844C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83248450: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83248454: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83248458: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324845C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83248460: 386A74D0  addi r3, r10, 0x74d0
	ctx.r[3].s64 = ctx.r[10].s64 + 29904;
	// 83248464: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83248468: 4AFE4A69  bl 0x8222ced0
	ctx.lr = 0x8324846C;
	sub_8222CED0(ctx, base);
	// 8324846C: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 83248470: 38697620  addi r3, r9, 0x7620
	ctx.r[3].s64 = ctx.r[9].s64 + 30240;
	// 83248474: 4BA61AAD  bl 0x82ca9f20
	ctx.lr = 0x83248478;
	sub_82CA9F20(ctx, base);
	// 83248478: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324847C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83248480: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83248484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83248488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83248488 size=64
    let mut pc: u32 = 0x83248488;
    'dispatch: loop {
        match pc {
            0x83248488 => {
    //   block [0x83248488..0x832484C8)
	// 83248488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324848C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83248490: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83248494: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83248498: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324849C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 832484A0: 386A74D4  addi r3, r10, 0x74d4
	ctx.r[3].s64 = ctx.r[10].s64 + 29908;
	// 832484A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832484A8: 4AFE4A29  bl 0x8222ced0
	ctx.lr = 0x832484AC;
	sub_8222CED0(ctx, base);
	// 832484AC: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 832484B0: 38697640  addi r3, r9, 0x7640
	ctx.r[3].s64 = ctx.r[9].s64 + 30272;
	// 832484B4: 4BA61A6D  bl 0x82ca9f20
	ctx.lr = 0x832484B8;
	sub_82CA9F20(ctx, base);
	// 832484B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832484BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832484C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832484C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832484C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832484C8 size=64
    let mut pc: u32 = 0x832484C8;
    'dispatch: loop {
        match pc {
            0x832484C8 => {
    //   block [0x832484C8..0x83248508)
	// 832484C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832484CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832484D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832484D4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832484D8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 832484DC: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 832484E0: 386A74D8  addi r3, r10, 0x74d8
	ctx.r[3].s64 = ctx.r[10].s64 + 29912;
	// 832484E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832484E8: 4AFE49E9  bl 0x8222ced0
	ctx.lr = 0x832484EC;
	sub_8222CED0(ctx, base);
	// 832484EC: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 832484F0: 38697650  addi r3, r9, 0x7650
	ctx.r[3].s64 = ctx.r[9].s64 + 30288;
	// 832484F4: 4BA61A2D  bl 0x82ca9f20
	ctx.lr = 0x832484F8;
	sub_82CA9F20(ctx, base);
	// 832484F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832484FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83248500: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83248504: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83248508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83248508 size=64
    let mut pc: u32 = 0x83248508;
    'dispatch: loop {
        match pc {
            0x83248508 => {
    //   block [0x83248508..0x83248548)
	// 83248508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324850C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83248510: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83248514: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83248518: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324851C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83248520: 386A74DC  addi r3, r10, 0x74dc
	ctx.r[3].s64 = ctx.r[10].s64 + 29916;
	// 83248524: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83248528: 4AFE49A9  bl 0x8222ced0
	ctx.lr = 0x8324852C;
	sub_8222CED0(ctx, base);
	// 8324852C: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 83248530: 38697660  addi r3, r9, 0x7660
	ctx.r[3].s64 = ctx.r[9].s64 + 30304;
	// 83248534: 4BA619ED  bl 0x82ca9f20
	ctx.lr = 0x83248538;
	sub_82CA9F20(ctx, base);
	// 83248538: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324853C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83248540: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83248544: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83248548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83248548 size=264
    let mut pc: u32 = 0x83248548;
    'dispatch: loop {
        match pc {
            0x83248548 => {
    //   block [0x83248548..0x83248650)
	// 83248548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324854C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83248550: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83248554: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83248558: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324855C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83248560: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83248564: 3BCB7088  addi r30, r11, 0x7088
	ctx.r[30].s64 = ctx.r[11].s64 + 28808;
	// 83248568: 3BEA74E0  addi r31, r10, 0x74e0
	ctx.r[31].s64 = ctx.r[10].s64 + 29920;
	// 8324856C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83248570: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83248574: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83248578: 4AFE4959  bl 0x8222ced0
	ctx.lr = 0x8324857C;
	sub_8222CED0(ctx, base);
	// 8324857C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83248580: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83248584: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 83248588: 4AFE4949  bl 0x8222ced0
	ctx.lr = 0x8324858C;
	sub_8222CED0(ctx, base);
	// 8324858C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83248590: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83248594: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 83248598: 4AFE4939  bl 0x8222ced0
	ctx.lr = 0x8324859C;
	sub_8222CED0(ctx, base);
	// 8324859C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 832485A0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832485A4: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 832485A8: 4AFE4929  bl 0x8222ced0
	ctx.lr = 0x832485AC;
	sub_8222CED0(ctx, base);
	// 832485AC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 832485B0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832485B4: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 832485B8: 4AFE4919  bl 0x8222ced0
	ctx.lr = 0x832485BC;
	sub_8222CED0(ctx, base);
	// 832485BC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 832485C0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832485C4: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 832485C8: 4AFE4909  bl 0x8222ced0
	ctx.lr = 0x832485CC;
	sub_8222CED0(ctx, base);
	// 832485CC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 832485D0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832485D4: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 832485D8: 4AFE48F9  bl 0x8222ced0
	ctx.lr = 0x832485DC;
	sub_8222CED0(ctx, base);
	// 832485DC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 832485E0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832485E4: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 832485E8: 4AFE48E9  bl 0x8222ced0
	ctx.lr = 0x832485EC;
	sub_8222CED0(ctx, base);
	// 832485EC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 832485F0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832485F4: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 832485F8: 4AFE48D9  bl 0x8222ced0
	ctx.lr = 0x832485FC;
	sub_8222CED0(ctx, base);
	// 832485FC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83248600: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83248604: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 83248608: 4AFE48C9  bl 0x8222ced0
	ctx.lr = 0x8324860C;
	sub_8222CED0(ctx, base);
	// 8324860C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83248610: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83248614: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 83248618: 4AFE48B9  bl 0x8222ced0
	ctx.lr = 0x8324861C;
	sub_8222CED0(ctx, base);
	// 8324861C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83248620: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83248624: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 83248628: 4AFE48A9  bl 0x8222ced0
	ctx.lr = 0x8324862C;
	sub_8222CED0(ctx, base);
	// 8324862C: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 83248630: 38697670  addi r3, r9, 0x7670
	ctx.r[3].s64 = ctx.r[9].s64 + 30320;
	// 83248634: 4BA618ED  bl 0x82ca9f20
	ctx.lr = 0x83248638;
	sub_82CA9F20(ctx, base);
	// 83248638: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8324863C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83248640: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83248644: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83248648: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8324864C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83248650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83248650 size=296
    let mut pc: u32 = 0x83248650;
    'dispatch: loop {
        match pc {
            0x83248650 => {
    //   block [0x83248650..0x83248778)
	// 83248650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83248654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83248658: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8324865C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83248660: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 83248664: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 83248668: 3BEB7510  addi r31, r11, 0x7510
	ctx.r[31].s64 = ctx.r[11].s64 + 29968;
	// 8324866C: 388A7088  addi r4, r10, 0x7088
	ctx.r[4].s64 = ctx.r[10].s64 + 28808;
	// 83248670: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83248674: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83248678: 4AFE4859  bl 0x8222ced0
	ctx.lr = 0x8324867C;
	sub_8222CED0(ctx, base);
	// 8324867C: 3D20820A  lis r9, -0x7df6
	ctx.r[9].s64 = -2113273856;
	// 83248680: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83248684: 38897154  addi r4, r9, 0x7154
	ctx.r[4].s64 = ctx.r[9].s64 + 29012;
	// 83248688: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8324868C: 4AFE4845  bl 0x8222ced0
	ctx.lr = 0x83248690;
	sub_8222CED0(ctx, base);
	// 83248690: 3D00820A  lis r8, -0x7df6
	ctx.r[8].s64 = -2113273856;
	// 83248694: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83248698: 38887140  addi r4, r8, 0x7140
	ctx.r[4].s64 = ctx.r[8].s64 + 28992;
	// 8324869C: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 832486A0: 4AFE4831  bl 0x8222ced0
	ctx.lr = 0x832486A4;
	sub_8222CED0(ctx, base);
	// 832486A4: 3CE0820A  lis r7, -0x7df6
	ctx.r[7].s64 = -2113273856;
	// 832486A8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832486AC: 3887712C  addi r4, r7, 0x712c
	ctx.r[4].s64 = ctx.r[7].s64 + 28972;
	// 832486B0: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 832486B4: 4AFE481D  bl 0x8222ced0
	ctx.lr = 0x832486B8;
	sub_8222CED0(ctx, base);
	// 832486B8: 3CC0820A  lis r6, -0x7df6
	ctx.r[6].s64 = -2113273856;
	// 832486BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832486C0: 38867118  addi r4, r6, 0x7118
	ctx.r[4].s64 = ctx.r[6].s64 + 28952;
	// 832486C4: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 832486C8: 4AFE4809  bl 0x8222ced0
	ctx.lr = 0x832486CC;
	sub_8222CED0(ctx, base);
	// 832486CC: 3C80820A  lis r4, -0x7df6
	ctx.r[4].s64 = -2113273856;
	// 832486D0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832486D4: 38847104  addi r4, r4, 0x7104
	ctx.r[4].s64 = ctx.r[4].s64 + 28932;
	// 832486D8: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 832486DC: 4AFE47F5  bl 0x8222ced0
	ctx.lr = 0x832486E0;
	sub_8222CED0(ctx, base);
	// 832486E0: 3C60820A  lis r3, -0x7df6
	ctx.r[3].s64 = -2113273856;
	// 832486E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832486E8: 388370F0  addi r4, r3, 0x70f0
	ctx.r[4].s64 = ctx.r[3].s64 + 28912;
	// 832486EC: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 832486F0: 4AFE47E1  bl 0x8222ced0
	ctx.lr = 0x832486F4;
	sub_8222CED0(ctx, base);
	// 832486F4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832486F8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832486FC: 388B70DC  addi r4, r11, 0x70dc
	ctx.r[4].s64 = ctx.r[11].s64 + 28892;
	// 83248700: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 83248704: 4AFE47CD  bl 0x8222ced0
	ctx.lr = 0x83248708;
	sub_8222CED0(ctx, base);
	// 83248708: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 8324870C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83248710: 388A70C8  addi r4, r10, 0x70c8
	ctx.r[4].s64 = ctx.r[10].s64 + 28872;
	// 83248714: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 83248718: 4AFE47B9  bl 0x8222ced0
	ctx.lr = 0x8324871C;
	sub_8222CED0(ctx, base);
	// 8324871C: 3D20820A  lis r9, -0x7df6
	ctx.r[9].s64 = -2113273856;
	// 83248720: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83248724: 388970B4  addi r4, r9, 0x70b4
	ctx.r[4].s64 = ctx.r[9].s64 + 28852;
	// 83248728: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 8324872C: 4AFE47A5  bl 0x8222ced0
	ctx.lr = 0x83248730;
	sub_8222CED0(ctx, base);
	// 83248730: 3D00820A  lis r8, -0x7df6
	ctx.r[8].s64 = -2113273856;
	// 83248734: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83248738: 388870A0  addi r4, r8, 0x70a0
	ctx.r[4].s64 = ctx.r[8].s64 + 28832;
	// 8324873C: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 83248740: 4AFE4791  bl 0x8222ced0
	ctx.lr = 0x83248744;
	sub_8222CED0(ctx, base);
	// 83248744: 3CE0820A  lis r7, -0x7df6
	ctx.r[7].s64 = -2113273856;
	// 83248748: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324874C: 3887708C  addi r4, r7, 0x708c
	ctx.r[4].s64 = ctx.r[7].s64 + 28812;
	// 83248750: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 83248754: 4AFE477D  bl 0x8222ced0
	ctx.lr = 0x83248758;
	sub_8222CED0(ctx, base);
	// 83248758: 3CC0832A  lis r6, -0x7cd6
	ctx.r[6].s64 = -2094399488;
	// 8324875C: 386676D8  addi r3, r6, 0x76d8
	ctx.r[3].s64 = ctx.r[6].s64 + 30424;
	// 83248760: 4BA617C1  bl 0x82ca9f20
	ctx.lr = 0x83248764;
	sub_82CA9F20(ctx, base);
	// 83248764: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83248768: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324876C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83248770: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83248774: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83248778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83248778 size=296
    let mut pc: u32 = 0x83248778;
    'dispatch: loop {
        match pc {
            0x83248778 => {
    //   block [0x83248778..0x832488A0)
	// 83248778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324877C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83248780: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83248784: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83248788: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 8324878C: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 83248790: 3BEB7540  addi r31, r11, 0x7540
	ctx.r[31].s64 = ctx.r[11].s64 + 30016;
	// 83248794: 388A7080  addi r4, r10, 0x7080
	ctx.r[4].s64 = ctx.r[10].s64 + 28800;
	// 83248798: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8324879C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832487A0: 4AFE4731  bl 0x8222ced0
	ctx.lr = 0x832487A4;
	sub_8222CED0(ctx, base);
	// 832487A4: 3D20820A  lis r9, -0x7df6
	ctx.r[9].s64 = -2113273856;
	// 832487A8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832487AC: 388971AC  addi r4, r9, 0x71ac
	ctx.r[4].s64 = ctx.r[9].s64 + 29100;
	// 832487B0: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 832487B4: 4AFE471D  bl 0x8222ced0
	ctx.lr = 0x832487B8;
	sub_8222CED0(ctx, base);
	// 832487B8: 3D00820A  lis r8, -0x7df6
	ctx.r[8].s64 = -2113273856;
	// 832487BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832487C0: 388871A8  addi r4, r8, 0x71a8
	ctx.r[4].s64 = ctx.r[8].s64 + 29096;
	// 832487C4: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 832487C8: 4AFE4709  bl 0x8222ced0
	ctx.lr = 0x832487CC;
	sub_8222CED0(ctx, base);
	// 832487CC: 3CE0820A  lis r7, -0x7df6
	ctx.r[7].s64 = -2113273856;
	// 832487D0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832487D4: 388771A0  addi r4, r7, 0x71a0
	ctx.r[4].s64 = ctx.r[7].s64 + 29088;
	// 832487D8: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 832487DC: 4AFE46F5  bl 0x8222ced0
	ctx.lr = 0x832487E0;
	sub_8222CED0(ctx, base);
	// 832487E0: 3CC0820A  lis r6, -0x7df6
	ctx.r[6].s64 = -2113273856;
	// 832487E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832487E8: 38867198  addi r4, r6, 0x7198
	ctx.r[4].s64 = ctx.r[6].s64 + 29080;
	// 832487EC: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 832487F0: 4AFE46E1  bl 0x8222ced0
	ctx.lr = 0x832487F4;
	sub_8222CED0(ctx, base);
	// 832487F4: 3C80820A  lis r4, -0x7df6
	ctx.r[4].s64 = -2113273856;
	// 832487F8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832487FC: 3884718C  addi r4, r4, 0x718c
	ctx.r[4].s64 = ctx.r[4].s64 + 29068;
	// 83248800: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 83248804: 4AFE46CD  bl 0x8222ced0
	ctx.lr = 0x83248808;
	sub_8222CED0(ctx, base);
	// 83248808: 3C60820A  lis r3, -0x7df6
	ctx.r[3].s64 = -2113273856;
	// 8324880C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83248810: 38837188  addi r4, r3, 0x7188
	ctx.r[4].s64 = ctx.r[3].s64 + 29064;
	// 83248814: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 83248818: 4AFE46B9  bl 0x8222ced0
	ctx.lr = 0x8324881C;
	sub_8222CED0(ctx, base);
	// 8324881C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83248820: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83248824: 388B7180  addi r4, r11, 0x7180
	ctx.r[4].s64 = ctx.r[11].s64 + 29056;
	// 83248828: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 8324882C: 4AFE46A5  bl 0x8222ced0
	ctx.lr = 0x83248830;
	sub_8222CED0(ctx, base);
	// 83248830: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 83248834: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83248838: 388A7178  addi r4, r10, 0x7178
	ctx.r[4].s64 = ctx.r[10].s64 + 29048;
	// 8324883C: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 83248840: 4AFE4691  bl 0x8222ced0
	ctx.lr = 0x83248844;
	sub_8222CED0(ctx, base);
	// 83248844: 3D20820A  lis r9, -0x7df6
	ctx.r[9].s64 = -2113273856;
	// 83248848: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324884C: 38897174  addi r4, r9, 0x7174
	ctx.r[4].s64 = ctx.r[9].s64 + 29044;
	// 83248850: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 83248854: 4AFE467D  bl 0x8222ced0
	ctx.lr = 0x83248858;
	sub_8222CED0(ctx, base);
	// 83248858: 3D00820A  lis r8, -0x7df6
	ctx.r[8].s64 = -2113273856;
	// 8324885C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83248860: 3888716C  addi r4, r8, 0x716c
	ctx.r[4].s64 = ctx.r[8].s64 + 29036;
	// 83248864: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 83248868: 4AFE4669  bl 0x8222ced0
	ctx.lr = 0x8324886C;
	sub_8222CED0(ctx, base);
	// 8324886C: 3CE0820A  lis r7, -0x7df6
	ctx.r[7].s64 = -2113273856;
	// 83248870: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83248874: 38877168  addi r4, r7, 0x7168
	ctx.r[4].s64 = ctx.r[7].s64 + 29032;
	// 83248878: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 8324887C: 4AFE4655  bl 0x8222ced0
	ctx.lr = 0x83248880;
	sub_8222CED0(ctx, base);
	// 83248880: 3CC0832A  lis r6, -0x7cd6
	ctx.r[6].s64 = -2094399488;
	// 83248884: 38667740  addi r3, r6, 0x7740
	ctx.r[3].s64 = ctx.r[6].s64 + 30528;
	// 83248888: 4BA61699  bl 0x82ca9f20
	ctx.lr = 0x8324888C;
	sub_82CA9F20(ctx, base);
	// 8324888C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83248890: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83248894: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83248898: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8324889C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832488A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832488A0 size=64
    let mut pc: u32 = 0x832488A0;
    'dispatch: loop {
        match pc {
            0x832488A0 => {
    //   block [0x832488A0..0x832488E0)
	// 832488A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832488A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832488A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832488AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832488B0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 832488B4: 388B7770  addi r4, r11, 0x7770
	ctx.r[4].s64 = ctx.r[11].s64 + 30576;
	// 832488B8: 386A7570  addi r3, r10, 0x7570
	ctx.r[3].s64 = ctx.r[10].s64 + 30064;
	// 832488BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832488C0: 4AFE4611  bl 0x8222ced0
	ctx.lr = 0x832488C4;
	sub_8222CED0(ctx, base);
	// 832488C4: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 832488C8: 38697810  addi r3, r9, 0x7810
	ctx.r[3].s64 = ctx.r[9].s64 + 30736;
	// 832488CC: 4BA61655  bl 0x82ca9f20
	ctx.lr = 0x832488D0;
	sub_82CA9F20(ctx, base);
	// 832488D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832488D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832488D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832488DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832488E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832488E0 size=64
    let mut pc: u32 = 0x832488E0;
    'dispatch: loop {
        match pc {
            0x832488E0 => {
    //   block [0x832488E0..0x83248920)
	// 832488E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832488E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832488E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832488EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832488F0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 832488F4: 388B7778  addi r4, r11, 0x7778
	ctx.r[4].s64 = ctx.r[11].s64 + 30584;
	// 832488F8: 386A7574  addi r3, r10, 0x7574
	ctx.r[3].s64 = ctx.r[10].s64 + 30068;
	// 832488FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83248900: 4AFE45D1  bl 0x8222ced0
	ctx.lr = 0x83248904;
	sub_8222CED0(ctx, base);
	// 83248904: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 83248908: 38697820  addi r3, r9, 0x7820
	ctx.r[3].s64 = ctx.r[9].s64 + 30752;
	// 8324890C: 4BA61615  bl 0x82ca9f20
	ctx.lr = 0x83248910;
	sub_82CA9F20(ctx, base);
	// 83248910: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83248914: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83248918: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324891C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83248920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83248920 size=64
    let mut pc: u32 = 0x83248920;
    'dispatch: loop {
        match pc {
            0x83248920 => {
    //   block [0x83248920..0x83248960)
	// 83248920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83248924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83248928: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324892C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83248930: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83248934: 388B778C  addi r4, r11, 0x778c
	ctx.r[4].s64 = ctx.r[11].s64 + 30604;
	// 83248938: 386A7578  addi r3, r10, 0x7578
	ctx.r[3].s64 = ctx.r[10].s64 + 30072;
	// 8324893C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83248940: 4AFE4591  bl 0x8222ced0
	ctx.lr = 0x83248944;
	sub_8222CED0(ctx, base);
	// 83248944: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 83248948: 38697830  addi r3, r9, 0x7830
	ctx.r[3].s64 = ctx.r[9].s64 + 30768;
	// 8324894C: 4BA615D5  bl 0x82ca9f20
	ctx.lr = 0x83248950;
	sub_82CA9F20(ctx, base);
	// 83248950: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83248954: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83248958: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324895C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83248960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83248960 size=64
    let mut pc: u32 = 0x83248960;
    'dispatch: loop {
        match pc {
            0x83248960 => {
    //   block [0x83248960..0x832489A0)
	// 83248960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83248964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83248968: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324896C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83248970: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83248974: 388B77A4  addi r4, r11, 0x77a4
	ctx.r[4].s64 = ctx.r[11].s64 + 30628;
	// 83248978: 386A757C  addi r3, r10, 0x757c
	ctx.r[3].s64 = ctx.r[10].s64 + 30076;
	// 8324897C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83248980: 4AFE4551  bl 0x8222ced0
	ctx.lr = 0x83248984;
	sub_8222CED0(ctx, base);
	// 83248984: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 83248988: 38697840  addi r3, r9, 0x7840
	ctx.r[3].s64 = ctx.r[9].s64 + 30784;
	// 8324898C: 4BA61595  bl 0x82ca9f20
	ctx.lr = 0x83248990;
	sub_82CA9F20(ctx, base);
	// 83248990: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83248994: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83248998: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324899C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832489A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832489A0 size=64
    let mut pc: u32 = 0x832489A0;
    'dispatch: loop {
        match pc {
            0x832489A0 => {
    //   block [0x832489A0..0x832489E0)
	// 832489A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832489A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832489A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832489AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832489B0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 832489B4: 388B77BC  addi r4, r11, 0x77bc
	ctx.r[4].s64 = ctx.r[11].s64 + 30652;
	// 832489B8: 386A7580  addi r3, r10, 0x7580
	ctx.r[3].s64 = ctx.r[10].s64 + 30080;
	// 832489BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832489C0: 4AFE4511  bl 0x8222ced0
	ctx.lr = 0x832489C4;
	sub_8222CED0(ctx, base);
	// 832489C4: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 832489C8: 38697850  addi r3, r9, 0x7850
	ctx.r[3].s64 = ctx.r[9].s64 + 30800;
	// 832489CC: 4BA61555  bl 0x82ca9f20
	ctx.lr = 0x832489D0;
	sub_82CA9F20(ctx, base);
	// 832489D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832489D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832489D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832489DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832489E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832489E0 size=64
    let mut pc: u32 = 0x832489E0;
    'dispatch: loop {
        match pc {
            0x832489E0 => {
    //   block [0x832489E0..0x83248A20)
	// 832489E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832489E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832489E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832489EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832489F0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 832489F4: 388B77D4  addi r4, r11, 0x77d4
	ctx.r[4].s64 = ctx.r[11].s64 + 30676;
	// 832489F8: 386A7584  addi r3, r10, 0x7584
	ctx.r[3].s64 = ctx.r[10].s64 + 30084;
	// 832489FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83248A00: 4AFE44D1  bl 0x8222ced0
	ctx.lr = 0x83248A04;
	sub_8222CED0(ctx, base);
	// 83248A04: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 83248A08: 38697860  addi r3, r9, 0x7860
	ctx.r[3].s64 = ctx.r[9].s64 + 30816;
	// 83248A0C: 4BA61515  bl 0x82ca9f20
	ctx.lr = 0x83248A10;
	sub_82CA9F20(ctx, base);
	// 83248A10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83248A14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83248A18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83248A1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83248A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83248A20 size=64
    let mut pc: u32 = 0x83248A20;
    'dispatch: loop {
        match pc {
            0x83248A20 => {
    //   block [0x83248A20..0x83248A60)
	// 83248A20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83248A24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83248A28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83248A2C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83248A30: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83248A34: 388B77E0  addi r4, r11, 0x77e0
	ctx.r[4].s64 = ctx.r[11].s64 + 30688;
	// 83248A38: 386A7588  addi r3, r10, 0x7588
	ctx.r[3].s64 = ctx.r[10].s64 + 30088;
	// 83248A3C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83248A40: 4AFE4491  bl 0x8222ced0
	ctx.lr = 0x83248A44;
	sub_8222CED0(ctx, base);
	// 83248A44: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 83248A48: 38697870  addi r3, r9, 0x7870
	ctx.r[3].s64 = ctx.r[9].s64 + 30832;
	// 83248A4C: 4BA614D5  bl 0x82ca9f20
	ctx.lr = 0x83248A50;
	sub_82CA9F20(ctx, base);
	// 83248A50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83248A54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83248A58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83248A5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83248A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83248A60 size=48
    let mut pc: u32 = 0x83248A60;
    'dispatch: loop {
        match pc {
            0x83248A60 => {
    //   block [0x83248A60..0x83248A68)
	// 83248A60: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 83248A64: 390B7088  addi r8, r11, 0x7088
	ctx.r[8].s64 = ctx.r[11].s64 + 28808;
	pc = 0x83248A68; continue 'dispatch;
            }
            0x83248A68 => {
    //   block [0x83248A68..0x83248A90)
	// 83248A68: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 83248A6C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83248A70: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 83248A74: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83248A78: 7D40412D  stwcx. r10, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 83248A7C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83248A80: 4082FFE8  bne 0x83248a68
	if !ctx.cr[0].eq {
	pc = 0x83248A68; continue 'dispatch;
	}
	// 83248A84: 3CE0832A  lis r7, -0x7cd6
	ctx.r[7].s64 = -2094399488;
	// 83248A88: 38677880  addi r3, r7, 0x7880
	ctx.r[3].s64 = ctx.r[7].s64 + 30848;
	// 83248A8C: 4BA61494  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83248A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83248A90 size=48
    let mut pc: u32 = 0x83248A90;
    'dispatch: loop {
        match pc {
            0x83248A90 => {
    //   block [0x83248A90..0x83248A98)
	// 83248A90: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 83248A94: 390B7088  addi r8, r11, 0x7088
	ctx.r[8].s64 = ctx.r[11].s64 + 28808;
	pc = 0x83248A98; continue 'dispatch;
            }
            0x83248A98 => {
    //   block [0x83248A98..0x83248AC0)
	// 83248A98: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 83248A9C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83248AA0: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 83248AA4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83248AA8: 7D40412D  stwcx. r10, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 83248AAC: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83248AB0: 4082FFE8  bne 0x83248a98
	if !ctx.cr[0].eq {
	pc = 0x83248A98; continue 'dispatch;
	}
	// 83248AB4: 3CE0832A  lis r7, -0x7cd6
	ctx.r[7].s64 = -2094399488;
	// 83248AB8: 38677890  addi r3, r7, 0x7890
	ctx.r[3].s64 = ctx.r[7].s64 + 30864;
	// 83248ABC: 4BA61464  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83248AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83248AC0 size=48
    let mut pc: u32 = 0x83248AC0;
    'dispatch: loop {
        match pc {
            0x83248AC0 => {
    //   block [0x83248AC0..0x83248AC8)
	// 83248AC0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 83248AC4: 390B7088  addi r8, r11, 0x7088
	ctx.r[8].s64 = ctx.r[11].s64 + 28808;
	pc = 0x83248AC8; continue 'dispatch;
            }
            0x83248AC8 => {
    //   block [0x83248AC8..0x83248AF0)
	// 83248AC8: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 83248ACC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83248AD0: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 83248AD4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83248AD8: 7D40412D  stwcx. r10, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 83248ADC: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83248AE0: 4082FFE8  bne 0x83248ac8
	if !ctx.cr[0].eq {
	pc = 0x83248AC8; continue 'dispatch;
	}
	// 83248AE4: 3CE0832A  lis r7, -0x7cd6
	ctx.r[7].s64 = -2094399488;
	// 83248AE8: 386778A0  addi r3, r7, 0x78a0
	ctx.r[3].s64 = ctx.r[7].s64 + 30880;
	// 83248AEC: 4BA61434  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83248AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83248AF0 size=64
    let mut pc: u32 = 0x83248AF0;
    'dispatch: loop {
        match pc {
            0x83248AF0 => {
    //   block [0x83248AF0..0x83248B30)
	// 83248AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83248AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83248AF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83248AFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83248B00: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83248B04: 388B49D8  addi r4, r11, 0x49d8
	ctx.r[4].s64 = ctx.r[11].s64 + 18904;
	// 83248B08: 386A7598  addi r3, r10, 0x7598
	ctx.r[3].s64 = ctx.r[10].s64 + 30104;
	// 83248B0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83248B10: 4AFE43C1  bl 0x8222ced0
	ctx.lr = 0x83248B14;
	sub_8222CED0(ctx, base);
	// 83248B14: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 83248B18: 386978B0  addi r3, r9, 0x78b0
	ctx.r[3].s64 = ctx.r[9].s64 + 30896;
	// 83248B1C: 4BA61405  bl 0x82ca9f20
	ctx.lr = 0x83248B20;
	sub_82CA9F20(ctx, base);
	// 83248B20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83248B24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83248B28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83248B2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83248B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83248B30 size=64
    let mut pc: u32 = 0x83248B30;
    'dispatch: loop {
        match pc {
            0x83248B30 => {
    //   block [0x83248B30..0x83248B70)
	// 83248B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83248B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83248B38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83248B3C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83248B40: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83248B44: 388B77EC  addi r4, r11, 0x77ec
	ctx.r[4].s64 = ctx.r[11].s64 + 30700;
	// 83248B48: 386A759C  addi r3, r10, 0x759c
	ctx.r[3].s64 = ctx.r[10].s64 + 30108;
	// 83248B4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83248B50: 4AFE4381  bl 0x8222ced0
	ctx.lr = 0x83248B54;
	sub_8222CED0(ctx, base);
	// 83248B54: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 83248B58: 386978C0  addi r3, r9, 0x78c0
	ctx.r[3].s64 = ctx.r[9].s64 + 30912;
	// 83248B5C: 4BA613C5  bl 0x82ca9f20
	ctx.lr = 0x83248B60;
	sub_82CA9F20(ctx, base);
	// 83248B60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83248B64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83248B68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83248B6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83248B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83248B70 size=64
    let mut pc: u32 = 0x83248B70;
    'dispatch: loop {
        match pc {
            0x83248B70 => {
    //   block [0x83248B70..0x83248BB0)
	// 83248B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83248B74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83248B78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83248B7C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83248B80: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83248B84: 388B77F8  addi r4, r11, 0x77f8
	ctx.r[4].s64 = ctx.r[11].s64 + 30712;
	// 83248B88: 386A75A0  addi r3, r10, 0x75a0
	ctx.r[3].s64 = ctx.r[10].s64 + 30112;
	// 83248B8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83248B90: 4AFE4341  bl 0x8222ced0
	ctx.lr = 0x83248B94;
	sub_8222CED0(ctx, base);
	// 83248B94: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 83248B98: 386978D0  addi r3, r9, 0x78d0
	ctx.r[3].s64 = ctx.r[9].s64 + 30928;
	// 83248B9C: 4BA61385  bl 0x82ca9f20
	ctx.lr = 0x83248BA0;
	sub_82CA9F20(ctx, base);
	// 83248BA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83248BA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83248BA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83248BAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83248BB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83248BB0 size=64
    let mut pc: u32 = 0x83248BB0;
    'dispatch: loop {
        match pc {
            0x83248BB0 => {
    //   block [0x83248BB0..0x83248BF0)
	// 83248BB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83248BB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83248BB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83248BBC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83248BC0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83248BC4: 388B7814  addi r4, r11, 0x7814
	ctx.r[4].s64 = ctx.r[11].s64 + 30740;
	// 83248BC8: 386A75A4  addi r3, r10, 0x75a4
	ctx.r[3].s64 = ctx.r[10].s64 + 30116;
	// 83248BCC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83248BD0: 4AFE4301  bl 0x8222ced0
	ctx.lr = 0x83248BD4;
	sub_8222CED0(ctx, base);
	// 83248BD4: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 83248BD8: 386978E0  addi r3, r9, 0x78e0
	ctx.r[3].s64 = ctx.r[9].s64 + 30944;
	// 83248BDC: 4BA61345  bl 0x82ca9f20
	ctx.lr = 0x83248BE0;
	sub_82CA9F20(ctx, base);
	// 83248BE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83248BE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83248BE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83248BEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83248BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83248BF0 size=64
    let mut pc: u32 = 0x83248BF0;
    'dispatch: loop {
        match pc {
            0x83248BF0 => {
    //   block [0x83248BF0..0x83248C30)
	// 83248BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83248BF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83248BF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83248BFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83248C00: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83248C04: 388B7824  addi r4, r11, 0x7824
	ctx.r[4].s64 = ctx.r[11].s64 + 30756;
	// 83248C08: 386A75A8  addi r3, r10, 0x75a8
	ctx.r[3].s64 = ctx.r[10].s64 + 30120;
	// 83248C0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83248C10: 4AFE42C1  bl 0x8222ced0
	ctx.lr = 0x83248C14;
	sub_8222CED0(ctx, base);
	// 83248C14: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 83248C18: 386978F0  addi r3, r9, 0x78f0
	ctx.r[3].s64 = ctx.r[9].s64 + 30960;
	// 83248C1C: 4BA61305  bl 0x82ca9f20
	ctx.lr = 0x83248C20;
	sub_82CA9F20(ctx, base);
	// 83248C20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83248C24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83248C28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83248C2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83248C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83248C30 size=64
    let mut pc: u32 = 0x83248C30;
    'dispatch: loop {
        match pc {
            0x83248C30 => {
    //   block [0x83248C30..0x83248C70)
	// 83248C30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83248C34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83248C38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83248C3C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83248C40: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83248C44: 388B7838  addi r4, r11, 0x7838
	ctx.r[4].s64 = ctx.r[11].s64 + 30776;
	// 83248C48: 386A75AC  addi r3, r10, 0x75ac
	ctx.r[3].s64 = ctx.r[10].s64 + 30124;
	// 83248C4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83248C50: 4AFE4281  bl 0x8222ced0
	ctx.lr = 0x83248C54;
	sub_8222CED0(ctx, base);
	// 83248C54: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 83248C58: 38697900  addi r3, r9, 0x7900
	ctx.r[3].s64 = ctx.r[9].s64 + 30976;
	// 83248C5C: 4BA612C5  bl 0x82ca9f20
	ctx.lr = 0x83248C60;
	sub_82CA9F20(ctx, base);
	// 83248C60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83248C64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83248C68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83248C6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83248C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83248C70 size=64
    let mut pc: u32 = 0x83248C70;
    'dispatch: loop {
        match pc {
            0x83248C70 => {
    //   block [0x83248C70..0x83248CB0)
	// 83248C70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83248C74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83248C78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83248C7C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83248C80: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83248C84: 388B785C  addi r4, r11, 0x785c
	ctx.r[4].s64 = ctx.r[11].s64 + 30812;
	// 83248C88: 386A75B0  addi r3, r10, 0x75b0
	ctx.r[3].s64 = ctx.r[10].s64 + 30128;
	// 83248C8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83248C90: 4AFE4241  bl 0x8222ced0
	ctx.lr = 0x83248C94;
	sub_8222CED0(ctx, base);
	// 83248C94: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 83248C98: 38697910  addi r3, r9, 0x7910
	ctx.r[3].s64 = ctx.r[9].s64 + 30992;
	// 83248C9C: 4BA61285  bl 0x82ca9f20
	ctx.lr = 0x83248CA0;
	sub_82CA9F20(ctx, base);
	// 83248CA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83248CA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83248CA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83248CAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83248CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83248CB0 size=64
    let mut pc: u32 = 0x83248CB0;
    'dispatch: loop {
        match pc {
            0x83248CB0 => {
    //   block [0x83248CB0..0x83248CF0)
	// 83248CB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83248CB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83248CB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83248CBC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83248CC0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83248CC4: 388B7838  addi r4, r11, 0x7838
	ctx.r[4].s64 = ctx.r[11].s64 + 30776;
	// 83248CC8: 386A75B4  addi r3, r10, 0x75b4
	ctx.r[3].s64 = ctx.r[10].s64 + 30132;
	// 83248CCC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83248CD0: 4AFE4201  bl 0x8222ced0
	ctx.lr = 0x83248CD4;
	sub_8222CED0(ctx, base);
	// 83248CD4: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 83248CD8: 38697920  addi r3, r9, 0x7920
	ctx.r[3].s64 = ctx.r[9].s64 + 31008;
	// 83248CDC: 4BA61245  bl 0x82ca9f20
	ctx.lr = 0x83248CE0;
	sub_82CA9F20(ctx, base);
	// 83248CE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83248CE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83248CE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83248CEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83248CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83248CF0 size=64
    let mut pc: u32 = 0x83248CF0;
    'dispatch: loop {
        match pc {
            0x83248CF0 => {
    //   block [0x83248CF0..0x83248D30)
	// 83248CF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83248CF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83248CF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83248CFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83248D00: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83248D04: 388B785C  addi r4, r11, 0x785c
	ctx.r[4].s64 = ctx.r[11].s64 + 30812;
	// 83248D08: 386A75B8  addi r3, r10, 0x75b8
	ctx.r[3].s64 = ctx.r[10].s64 + 30136;
	// 83248D0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83248D10: 4AFE41C1  bl 0x8222ced0
	ctx.lr = 0x83248D14;
	sub_8222CED0(ctx, base);
	// 83248D14: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 83248D18: 38697930  addi r3, r9, 0x7930
	ctx.r[3].s64 = ctx.r[9].s64 + 31024;
	// 83248D1C: 4BA61205  bl 0x82ca9f20
	ctx.lr = 0x83248D20;
	sub_82CA9F20(ctx, base);
	// 83248D20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83248D24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83248D28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83248D2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83248D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83248D30 size=48
    let mut pc: u32 = 0x83248D30;
    'dispatch: loop {
        match pc {
            0x83248D30 => {
    //   block [0x83248D30..0x83248D38)
	// 83248D30: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 83248D34: 390B7088  addi r8, r11, 0x7088
	ctx.r[8].s64 = ctx.r[11].s64 + 28808;
	pc = 0x83248D38; continue 'dispatch;
            }
            0x83248D38 => {
    //   block [0x83248D38..0x83248D60)
	// 83248D38: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 83248D3C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83248D40: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 83248D44: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83248D48: 7D40412D  stwcx. r10, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 83248D4C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83248D50: 4082FFE8  bne 0x83248d38
	if !ctx.cr[0].eq {
	pc = 0x83248D38; continue 'dispatch;
	}
	// 83248D54: 3CE0832A  lis r7, -0x7cd6
	ctx.r[7].s64 = -2094399488;
	// 83248D58: 38677940  addi r3, r7, 0x7940
	ctx.r[3].s64 = ctx.r[7].s64 + 31040;
	// 83248D5C: 4BA611C4  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83248D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83248D60 size=64
    let mut pc: u32 = 0x83248D60;
    'dispatch: loop {
        match pc {
            0x83248D60 => {
    //   block [0x83248D60..0x83248DA0)
	// 83248D60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83248D64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83248D68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83248D6C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83248D70: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83248D74: 388B787C  addi r4, r11, 0x787c
	ctx.r[4].s64 = ctx.r[11].s64 + 30844;
	// 83248D78: 386A75C0  addi r3, r10, 0x75c0
	ctx.r[3].s64 = ctx.r[10].s64 + 30144;
	// 83248D7C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83248D80: 4AFE4151  bl 0x8222ced0
	ctx.lr = 0x83248D84;
	sub_8222CED0(ctx, base);
	// 83248D84: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 83248D88: 38697950  addi r3, r9, 0x7950
	ctx.r[3].s64 = ctx.r[9].s64 + 31056;
	// 83248D8C: 4BA61195  bl 0x82ca9f20
	ctx.lr = 0x83248D90;
	sub_82CA9F20(ctx, base);
	// 83248D90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83248D94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83248D98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83248D9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83248DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83248DA0 size=48
    let mut pc: u32 = 0x83248DA0;
    'dispatch: loop {
        match pc {
            0x83248DA0 => {
    //   block [0x83248DA0..0x83248DA8)
	// 83248DA0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 83248DA4: 390B7088  addi r8, r11, 0x7088
	ctx.r[8].s64 = ctx.r[11].s64 + 28808;
	pc = 0x83248DA8; continue 'dispatch;
            }
            0x83248DA8 => {
    //   block [0x83248DA8..0x83248DD0)
	// 83248DA8: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 83248DAC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83248DB0: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 83248DB4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83248DB8: 7D40412D  stwcx. r10, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 83248DBC: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83248DC0: 4082FFE8  bne 0x83248da8
	if !ctx.cr[0].eq {
	pc = 0x83248DA8; continue 'dispatch;
	}
	// 83248DC4: 3CE0832A  lis r7, -0x7cd6
	ctx.r[7].s64 = -2094399488;
	// 83248DC8: 38677960  addi r3, r7, 0x7960
	ctx.r[3].s64 = ctx.r[7].s64 + 31072;
	// 83248DCC: 4BA61154  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83248DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83248DD0 size=12
    let mut pc: u32 = 0x83248DD0;
    'dispatch: loop {
        match pc {
            0x83248DD0 => {
    //   block [0x83248DD0..0x83248DDC)
	// 83248DD0: 3D60832A  lis r11, -0x7cd6
	ctx.r[11].s64 = -2094399488;
	// 83248DD4: 386B7970  addi r3, r11, 0x7970
	ctx.r[3].s64 = ctx.r[11].s64 + 31088;
	// 83248DD8: 4BA61148  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83248DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83248DE0 size=12
    let mut pc: u32 = 0x83248DE0;
    'dispatch: loop {
        match pc {
            0x83248DE0 => {
    //   block [0x83248DE0..0x83248DEC)
	// 83248DE0: 3D60832A  lis r11, -0x7cd6
	ctx.r[11].s64 = -2094399488;
	// 83248DE4: 386B7980  addi r3, r11, 0x7980
	ctx.r[3].s64 = ctx.r[11].s64 + 31104;
	// 83248DE8: 4BA61138  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83248DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83248DF0 size=96
    let mut pc: u32 = 0x83248DF0;
    'dispatch: loop {
        match pc {
            0x83248DF0 => {
    //   block [0x83248DF0..0x83248E14)
	// 83248DF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83248DF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83248DF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83248DFC: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 83248E00: 4AFD6459  bl 0x8221f258
	ctx.lr = 0x83248E04;
	sub_8221F258(ctx, base);
	// 83248E04: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83248E08: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83248E0C: 419A0008  beq cr6, 0x83248e14
	if ctx.cr[6].eq {
	pc = 0x83248E14; continue 'dispatch;
	}
	// 83248E10: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x83248E14; continue 'dispatch;
            }
            0x83248E14 => {
    //   block [0x83248E14..0x83248E20)
	// 83248E14: 354B0004  addic. r10, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83248E18: 41820008  beq 0x83248e20
	if ctx.cr[0].eq {
	pc = 0x83248E20; continue 'dispatch;
	}
	// 83248E1C: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x83248E20; continue 'dispatch;
            }
            0x83248E20 => {
    //   block [0x83248E20..0x83248E50)
	// 83248E20: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 83248E24: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83248E28: 390975E8  addi r8, r9, 0x75e8
	ctx.r[8].s64 = ctx.r[9].s64 + 30184;
	// 83248E2C: 3CE0832A  lis r7, -0x7cd6
	ctx.r[7].s64 = -2094399488;
	// 83248E30: 38677990  addi r3, r7, 0x7990
	ctx.r[3].s64 = ctx.r[7].s64 + 31120;
	// 83248E34: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83248E38: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83248E3C: 4BA610E5  bl 0x82ca9f20
	ctx.lr = 0x83248E40;
	sub_82CA9F20(ctx, base);
	// 83248E40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83248E44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83248E48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83248E4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83248E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83248E50 size=20
    let mut pc: u32 = 0x83248E50;
    'dispatch: loop {
        match pc {
            0x83248E50 => {
    //   block [0x83248E50..0x83248E64)
	// 83248E50: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 83248E54: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83248E58: C80B9668  lfd f0, -0x6998(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-27032 as u32) ) };
	// 83248E5C: D80A75F8  stfd f0, 0x75f8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(30200 as u32), ctx.f[0].u64 ) };
	// 83248E60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83248E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83248E68 size=96
    let mut pc: u32 = 0x83248E68;
    'dispatch: loop {
        match pc {
            0x83248E68 => {
    //   block [0x83248E68..0x83248EC8)
	// 83248E68: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83248E6C: 3901FFF0  addi r8, r1, -0x10
	ctx.r[8].s64 = ctx.r[1].s64 + -16;
	// 83248E70: 392B9490  addi r9, r11, -0x6b70
	ctx.r[9].s64 = ctx.r[11].s64 + -27504;
	// 83248E74: 3941FFF4  addi r10, r1, -0xc
	ctx.r[10].s64 = ctx.r[1].s64 + -12;
	// 83248E78: 38E1FFF4  addi r7, r1, -0xc
	ctx.r[7].s64 = ctx.r[1].s64 + -12;
	// 83248E7C: C00B9490  lfs f0, -0x6b70(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83248E80: 38C1FFF0  addi r6, r1, -0x10
	ctx.r[6].s64 = ctx.r[1].s64 + -16;
	// 83248E84: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 83248E88: 3CA08349  lis r5, -0x7cb7
	ctx.r[5].s64 = -2092367872;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83248EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83248EC8 size=96
    let mut pc: u32 = 0x83248EC8;
    'dispatch: loop {
        match pc {
            0x83248EC8 => {
    //   block [0x83248EC8..0x83248F28)
	// 83248EC8: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83248ECC: 3941FFF4  addi r10, r1, -0xc
	ctx.r[10].s64 = ctx.r[1].s64 + -12;
	// 83248ED0: 392B9490  addi r9, r11, -0x6b70
	ctx.r[9].s64 = ctx.r[11].s64 + -27504;
	// 83248ED4: 3901FFF8  addi r8, r1, -8
	ctx.r[8].s64 = ctx.r[1].s64 + -8;
	// 83248ED8: 38E1FFF0  addi r7, r1, -0x10
	ctx.r[7].s64 = ctx.r[1].s64 + -16;
	// 83248EDC: C00B9490  lfs f0, -0x6b70(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83248EE0: 38C1FFF8  addi r6, r1, -8
	ctx.r[6].s64 = ctx.r[1].s64 + -8;
	// 83248EE4: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 83248EE8: 3CA08349  lis r5, -0x7cb7
	ctx.r[5].s64 = -2092367872;
	// 83248EEC: C009FFF4  lfs f0, -0xc(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83248EF0: D001FFF4  stfs f0, -0xc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), tmp.u32 ) };
	// 83248EF4: 38857610  addi r4, r5, 0x7610
	ctx.r[4].s64 = ctx.r[5].s64 + 30224;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83248F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83248F28 size=88
    let mut pc: u32 = 0x83248F28;
    'dispatch: loop {
        match pc {
            0x83248F28 => {
    //   block [0x83248F28..0x83248F80)
	// 83248F28: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83248F2C: 3941FFF0  addi r10, r1, -0x10
	ctx.r[10].s64 = ctx.r[1].s64 + -16;
	// 83248F30: 3921FFF4  addi r9, r1, -0xc
	ctx.r[9].s64 = ctx.r[1].s64 + -12;
	// 83248F34: 3901FFF4  addi r8, r1, -0xc
	ctx.r[8].s64 = ctx.r[1].s64 + -12;
	// 83248F38: 38E1FFF0  addi r7, r1, -0x10
	ctx.r[7].s64 = ctx.r[1].s64 + -16;
	// 83248F3C: C00B9484  lfs f0, -0x6b7c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83248F40: 3CC08349  lis r6, -0x7cb7
	ctx.r[6].s64 = -2092367872;
	// 83248F44: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83248F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83248F80 size=64
    let mut pc: u32 = 0x83248F80;
    'dispatch: loop {
        match pc {
            0x83248F80 => {
    //   block [0x83248F80..0x83248FC0)
	// 83248F80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83248F84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83248F88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83248F8C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83248F90: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83248F94: 388B7F80  addi r4, r11, 0x7f80
	ctx.r[4].s64 = ctx.r[11].s64 + 32640;
	// 83248F98: 386A75F4  addi r3, r10, 0x75f4
	ctx.r[3].s64 = ctx.r[10].s64 + 30196;
	// 83248F9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83248FA0: 4AFE3F31  bl 0x8222ced0
	ctx.lr = 0x83248FA4;
	sub_8222CED0(ctx, base);
	// 83248FA4: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 83248FA8: 386979E0  addi r3, r9, 0x79e0
	ctx.r[3].s64 = ctx.r[9].s64 + 31200;
	// 83248FAC: 4BA60F75  bl 0x82ca9f20
	ctx.lr = 0x83248FB0;
	sub_82CA9F20(ctx, base);
	// 83248FB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83248FB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83248FB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83248FBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83248FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83248FC0 size=64
    let mut pc: u32 = 0x83248FC0;
    'dispatch: loop {
        match pc {
            0x83248FC0 => {
    //   block [0x83248FC0..0x83249000)
	// 83248FC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83248FC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83248FC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83248FCC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83248FD0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83248FD4: 388B77EC  addi r4, r11, 0x77ec
	ctx.r[4].s64 = ctx.r[11].s64 + 30700;
	// 83248FD8: 386A7630  addi r3, r10, 0x7630
	ctx.r[3].s64 = ctx.r[10].s64 + 30256;
	// 83248FDC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83248FE0: 4AFE3EF1  bl 0x8222ced0
	ctx.lr = 0x83248FE4;
	sub_8222CED0(ctx, base);
	// 83248FE4: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 83248FE8: 386979F0  addi r3, r9, 0x79f0
	ctx.r[3].s64 = ctx.r[9].s64 + 31216;
	// 83248FEC: 4BA60F35  bl 0x82ca9f20
	ctx.lr = 0x83248FF0;
	sub_82CA9F20(ctx, base);
	// 83248FF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83248FF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83248FF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83248FFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249000 size=64
    let mut pc: u32 = 0x83249000;
    'dispatch: loop {
        match pc {
            0x83249000 => {
    //   block [0x83249000..0x83249040)
	// 83249000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249008: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324900C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83249010: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249014: 388B7F98  addi r4, r11, 0x7f98
	ctx.r[4].s64 = ctx.r[11].s64 + 32664;
	// 83249018: 386A7634  addi r3, r10, 0x7634
	ctx.r[3].s64 = ctx.r[10].s64 + 30260;
	// 8324901C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249020: 4AFE3EB1  bl 0x8222ced0
	ctx.lr = 0x83249024;
	sub_8222CED0(ctx, base);
	// 83249024: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 83249028: 38697A00  addi r3, r9, 0x7a00
	ctx.r[3].s64 = ctx.r[9].s64 + 31232;
	// 8324902C: 4BA60EF5  bl 0x82ca9f20
	ctx.lr = 0x83249030;
	sub_82CA9F20(ctx, base);
	// 83249030: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249034: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249038: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324903C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249040 size=64
    let mut pc: u32 = 0x83249040;
    'dispatch: loop {
        match pc {
            0x83249040 => {
    //   block [0x83249040..0x83249080)
	// 83249040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249048: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324904C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83249050: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249054: 388B04E8  addi r4, r11, 0x4e8
	ctx.r[4].s64 = ctx.r[11].s64 + 1256;
	// 83249058: 386A7638  addi r3, r10, 0x7638
	ctx.r[3].s64 = ctx.r[10].s64 + 30264;
	// 8324905C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249060: 4AFE3E71  bl 0x8222ced0
	ctx.lr = 0x83249064;
	sub_8222CED0(ctx, base);
	// 83249064: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 83249068: 38697A10  addi r3, r9, 0x7a10
	ctx.r[3].s64 = ctx.r[9].s64 + 31248;
	// 8324906C: 4BA60EB5  bl 0x82ca9f20
	ctx.lr = 0x83249070;
	sub_82CA9F20(ctx, base);
	// 83249070: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249074: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249078: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324907C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249080 size=64
    let mut pc: u32 = 0x83249080;
    'dispatch: loop {
        match pc {
            0x83249080 => {
    //   block [0x83249080..0x832490C0)
	// 83249080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249088: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324908C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83249090: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249094: 388B7FB8  addi r4, r11, 0x7fb8
	ctx.r[4].s64 = ctx.r[11].s64 + 32696;
	// 83249098: 386A763C  addi r3, r10, 0x763c
	ctx.r[3].s64 = ctx.r[10].s64 + 30268;
	// 8324909C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832490A0: 4AFE3E31  bl 0x8222ced0
	ctx.lr = 0x832490A4;
	sub_8222CED0(ctx, base);
	// 832490A4: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 832490A8: 38697A20  addi r3, r9, 0x7a20
	ctx.r[3].s64 = ctx.r[9].s64 + 31264;
	// 832490AC: 4BA60E75  bl 0x82ca9f20
	ctx.lr = 0x832490B0;
	sub_82CA9F20(ctx, base);
	// 832490B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832490B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832490B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832490BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832490C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832490C0 size=64
    let mut pc: u32 = 0x832490C0;
    'dispatch: loop {
        match pc {
            0x832490C0 => {
    //   block [0x832490C0..0x83249100)
	// 832490C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832490C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832490C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832490CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832490D0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 832490D4: 388B7FE4  addi r4, r11, 0x7fe4
	ctx.r[4].s64 = ctx.r[11].s64 + 32740;
	// 832490D8: 386A7640  addi r3, r10, 0x7640
	ctx.r[3].s64 = ctx.r[10].s64 + 30272;
	// 832490DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832490E0: 4AFE3DF1  bl 0x8222ced0
	ctx.lr = 0x832490E4;
	sub_8222CED0(ctx, base);
	// 832490E4: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 832490E8: 38697A30  addi r3, r9, 0x7a30
	ctx.r[3].s64 = ctx.r[9].s64 + 31280;
	// 832490EC: 4BA60E35  bl 0x82ca9f20
	ctx.lr = 0x832490F0;
	sub_82CA9F20(ctx, base);
	// 832490F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832490F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832490F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832490FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249100 size=64
    let mut pc: u32 = 0x83249100;
    'dispatch: loop {
        match pc {
            0x83249100 => {
    //   block [0x83249100..0x83249140)
	// 83249100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249108: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324910C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83249110: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249114: 388B800C  addi r4, r11, -0x7ff4
	ctx.r[4].s64 = ctx.r[11].s64 + -32756;
	// 83249118: 386A7644  addi r3, r10, 0x7644
	ctx.r[3].s64 = ctx.r[10].s64 + 30276;
	// 8324911C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249120: 4AFE3DB1  bl 0x8222ced0
	ctx.lr = 0x83249124;
	sub_8222CED0(ctx, base);
	// 83249124: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 83249128: 38697A40  addi r3, r9, 0x7a40
	ctx.r[3].s64 = ctx.r[9].s64 + 31296;
	// 8324912C: 4BA60DF5  bl 0x82ca9f20
	ctx.lr = 0x83249130;
	sub_82CA9F20(ctx, base);
	// 83249130: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249134: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249138: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324913C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249140 size=64
    let mut pc: u32 = 0x83249140;
    'dispatch: loop {
        match pc {
            0x83249140 => {
    //   block [0x83249140..0x83249180)
	// 83249140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249148: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324914C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83249150: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249154: 388B8034  addi r4, r11, -0x7fcc
	ctx.r[4].s64 = ctx.r[11].s64 + -32716;
	// 83249158: 386A7648  addi r3, r10, 0x7648
	ctx.r[3].s64 = ctx.r[10].s64 + 30280;
	// 8324915C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249160: 4AFE3D71  bl 0x8222ced0
	ctx.lr = 0x83249164;
	sub_8222CED0(ctx, base);
	// 83249164: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 83249168: 38697A50  addi r3, r9, 0x7a50
	ctx.r[3].s64 = ctx.r[9].s64 + 31312;
	// 8324916C: 4BA60DB5  bl 0x82ca9f20
	ctx.lr = 0x83249170;
	sub_82CA9F20(ctx, base);
	// 83249170: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249174: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249178: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324917C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249180 size=64
    let mut pc: u32 = 0x83249180;
    'dispatch: loop {
        match pc {
            0x83249180 => {
    //   block [0x83249180..0x832491C0)
	// 83249180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249188: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324918C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83249190: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249194: 388B8064  addi r4, r11, -0x7f9c
	ctx.r[4].s64 = ctx.r[11].s64 + -32668;
	// 83249198: 386A764C  addi r3, r10, 0x764c
	ctx.r[3].s64 = ctx.r[10].s64 + 30284;
	// 8324919C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832491A0: 4AFE3D31  bl 0x8222ced0
	ctx.lr = 0x832491A4;
	sub_8222CED0(ctx, base);
	// 832491A4: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 832491A8: 38697A60  addi r3, r9, 0x7a60
	ctx.r[3].s64 = ctx.r[9].s64 + 31328;
	// 832491AC: 4BA60D75  bl 0x82ca9f20
	ctx.lr = 0x832491B0;
	sub_82CA9F20(ctx, base);
	// 832491B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832491B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832491B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832491BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832491C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832491C0 size=64
    let mut pc: u32 = 0x832491C0;
    'dispatch: loop {
        match pc {
            0x832491C0 => {
    //   block [0x832491C0..0x83249200)
	// 832491C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832491C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832491C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832491CC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832491D0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 832491D4: 388B8090  addi r4, r11, -0x7f70
	ctx.r[4].s64 = ctx.r[11].s64 + -32624;
	// 832491D8: 386A7650  addi r3, r10, 0x7650
	ctx.r[3].s64 = ctx.r[10].s64 + 30288;
	// 832491DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832491E0: 4AFE3CF1  bl 0x8222ced0
	ctx.lr = 0x832491E4;
	sub_8222CED0(ctx, base);
	// 832491E4: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 832491E8: 38697A70  addi r3, r9, 0x7a70
	ctx.r[3].s64 = ctx.r[9].s64 + 31344;
	// 832491EC: 4BA60D35  bl 0x82ca9f20
	ctx.lr = 0x832491F0;
	sub_82CA9F20(ctx, base);
	// 832491F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832491F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832491F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832491FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249200 size=64
    let mut pc: u32 = 0x83249200;
    'dispatch: loop {
        match pc {
            0x83249200 => {
    //   block [0x83249200..0x83249240)
	// 83249200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249208: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324920C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83249210: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249214: 388B80BC  addi r4, r11, -0x7f44
	ctx.r[4].s64 = ctx.r[11].s64 + -32580;
	// 83249218: 386A7654  addi r3, r10, 0x7654
	ctx.r[3].s64 = ctx.r[10].s64 + 30292;
	// 8324921C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249220: 4AFE3CB1  bl 0x8222ced0
	ctx.lr = 0x83249224;
	sub_8222CED0(ctx, base);
	// 83249224: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 83249228: 38697A80  addi r3, r9, 0x7a80
	ctx.r[3].s64 = ctx.r[9].s64 + 31360;
	// 8324922C: 4BA60CF5  bl 0x82ca9f20
	ctx.lr = 0x83249230;
	sub_82CA9F20(ctx, base);
	// 83249230: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249234: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249238: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324923C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249240 size=64
    let mut pc: u32 = 0x83249240;
    'dispatch: loop {
        match pc {
            0x83249240 => {
    //   block [0x83249240..0x83249280)
	// 83249240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249244: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249248: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324924C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83249250: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249254: 388B80E8  addi r4, r11, -0x7f18
	ctx.r[4].s64 = ctx.r[11].s64 + -32536;
	// 83249258: 386A7658  addi r3, r10, 0x7658
	ctx.r[3].s64 = ctx.r[10].s64 + 30296;
	// 8324925C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249260: 4AFE3C71  bl 0x8222ced0
	ctx.lr = 0x83249264;
	sub_8222CED0(ctx, base);
	// 83249264: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 83249268: 38697A90  addi r3, r9, 0x7a90
	ctx.r[3].s64 = ctx.r[9].s64 + 31376;
	// 8324926C: 4BA60CB5  bl 0x82ca9f20
	ctx.lr = 0x83249270;
	sub_82CA9F20(ctx, base);
	// 83249270: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249274: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249278: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324927C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249280 size=64
    let mut pc: u32 = 0x83249280;
    'dispatch: loop {
        match pc {
            0x83249280 => {
    //   block [0x83249280..0x832492C0)
	// 83249280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249284: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249288: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324928C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83249290: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249294: 388B8114  addi r4, r11, -0x7eec
	ctx.r[4].s64 = ctx.r[11].s64 + -32492;
	// 83249298: 386A765C  addi r3, r10, 0x765c
	ctx.r[3].s64 = ctx.r[10].s64 + 30300;
	// 8324929C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832492A0: 4AFE3C31  bl 0x8222ced0
	ctx.lr = 0x832492A4;
	sub_8222CED0(ctx, base);
	// 832492A4: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 832492A8: 38697AA0  addi r3, r9, 0x7aa0
	ctx.r[3].s64 = ctx.r[9].s64 + 31392;
	// 832492AC: 4BA60C75  bl 0x82ca9f20
	ctx.lr = 0x832492B0;
	sub_82CA9F20(ctx, base);
	// 832492B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832492B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832492B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832492BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832492C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832492C0 size=64
    let mut pc: u32 = 0x832492C0;
    'dispatch: loop {
        match pc {
            0x832492C0 => {
    //   block [0x832492C0..0x83249300)
	// 832492C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832492C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832492C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832492CC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832492D0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 832492D4: 388B8140  addi r4, r11, -0x7ec0
	ctx.r[4].s64 = ctx.r[11].s64 + -32448;
	// 832492D8: 386A7660  addi r3, r10, 0x7660
	ctx.r[3].s64 = ctx.r[10].s64 + 30304;
	// 832492DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832492E0: 4AFE3BF1  bl 0x8222ced0
	ctx.lr = 0x832492E4;
	sub_8222CED0(ctx, base);
	// 832492E4: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 832492E8: 38697AB0  addi r3, r9, 0x7ab0
	ctx.r[3].s64 = ctx.r[9].s64 + 31408;
	// 832492EC: 4BA60C35  bl 0x82ca9f20
	ctx.lr = 0x832492F0;
	sub_82CA9F20(ctx, base);
	// 832492F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832492F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832492F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832492FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249300 size=64
    let mut pc: u32 = 0x83249300;
    'dispatch: loop {
        match pc {
            0x83249300 => {
    //   block [0x83249300..0x83249340)
	// 83249300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249308: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324930C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83249310: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249314: 388B816C  addi r4, r11, -0x7e94
	ctx.r[4].s64 = ctx.r[11].s64 + -32404;
	// 83249318: 386A7664  addi r3, r10, 0x7664
	ctx.r[3].s64 = ctx.r[10].s64 + 30308;
	// 8324931C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249320: 4AFE3BB1  bl 0x8222ced0
	ctx.lr = 0x83249324;
	sub_8222CED0(ctx, base);
	// 83249324: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 83249328: 38697AC0  addi r3, r9, 0x7ac0
	ctx.r[3].s64 = ctx.r[9].s64 + 31424;
	// 8324932C: 4BA60BF5  bl 0x82ca9f20
	ctx.lr = 0x83249330;
	sub_82CA9F20(ctx, base);
	// 83249330: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249334: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249338: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324933C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83249340 size=88
    let mut pc: u32 = 0x83249340;
    'dispatch: loop {
        match pc {
            0x83249340 => {
    //   block [0x83249340..0x83249398)
	// 83249340: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83249344: 3941FFF0  addi r10, r1, -0x10
	ctx.r[10].s64 = ctx.r[1].s64 + -16;
	// 83249348: 3921FFF4  addi r9, r1, -0xc
	ctx.r[9].s64 = ctx.r[1].s64 + -12;
	// 8324934C: 3901FFF4  addi r8, r1, -0xc
	ctx.r[8].s64 = ctx.r[1].s64 + -12;
	// 83249350: 38E1FFF0  addi r7, r1, -0x10
	ctx.r[7].s64 = ctx.r[1].s64 + -16;
	// 83249354: C00B9490  lfs f0, -0x6b70(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83249358: 3CC08349  lis r6, -0x7cb7
	ctx.r[6].s64 = -2092367872;
	// 8324935C: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83249398 size=96
    let mut pc: u32 = 0x83249398;
    'dispatch: loop {
        match pc {
            0x83249398 => {
    //   block [0x83249398..0x832493F8)
	// 83249398: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324939C: 3901FFF0  addi r8, r1, -0x10
	ctx.r[8].s64 = ctx.r[1].s64 + -16;
	// 832493A0: 392B9484  addi r9, r11, -0x6b7c
	ctx.r[9].s64 = ctx.r[11].s64 + -27516;
	// 832493A4: 3941FFF4  addi r10, r1, -0xc
	ctx.r[10].s64 = ctx.r[1].s64 + -12;
	// 832493A8: 38E1FFF4  addi r7, r1, -0xc
	ctx.r[7].s64 = ctx.r[1].s64 + -12;
	// 832493AC: C00B9484  lfs f0, -0x6b7c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832493B0: 38C1FFF0  addi r6, r1, -0x10
	ctx.r[6].s64 = ctx.r[1].s64 + -16;
	// 832493B4: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 832493B8: 3CA08349  lis r5, -0x7cb7
	ctx.r[5].s64 = -2092367872;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832493F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832493F8 size=64
    let mut pc: u32 = 0x832493F8;
    'dispatch: loop {
        match pc {
            0x832493F8 => {
    //   block [0x832493F8..0x83249438)
	// 832493F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832493FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249400: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249404: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83249408: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324940C: 388B819C  addi r4, r11, -0x7e64
	ctx.r[4].s64 = ctx.r[11].s64 + -32356;
	// 83249410: 386A7668  addi r3, r10, 0x7668
	ctx.r[3].s64 = ctx.r[10].s64 + 30312;
	// 83249414: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249418: 4AFE3AB9  bl 0x8222ced0
	ctx.lr = 0x8324941C;
	sub_8222CED0(ctx, base);
	// 8324941C: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 83249420: 38697AD0  addi r3, r9, 0x7ad0
	ctx.r[3].s64 = ctx.r[9].s64 + 31440;
	// 83249424: 4BA60AFD  bl 0x82ca9f20
	ctx.lr = 0x83249428;
	sub_82CA9F20(ctx, base);
	// 83249428: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324942C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249430: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249434: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249438 size=64
    let mut pc: u32 = 0x83249438;
    'dispatch: loop {
        match pc {
            0x83249438 => {
    //   block [0x83249438..0x83249478)
	// 83249438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324943C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249440: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249444: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83249448: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324944C: 388B81CC  addi r4, r11, -0x7e34
	ctx.r[4].s64 = ctx.r[11].s64 + -32308;
	// 83249450: 386A766C  addi r3, r10, 0x766c
	ctx.r[3].s64 = ctx.r[10].s64 + 30316;
	// 83249454: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249458: 4AFE3A79  bl 0x8222ced0
	ctx.lr = 0x8324945C;
	sub_8222CED0(ctx, base);
	// 8324945C: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 83249460: 38697AE0  addi r3, r9, 0x7ae0
	ctx.r[3].s64 = ctx.r[9].s64 + 31456;
	// 83249464: 4BA60ABD  bl 0x82ca9f20
	ctx.lr = 0x83249468;
	sub_82CA9F20(ctx, base);
	// 83249468: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324946C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249470: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249474: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249478 size=192
    let mut pc: u32 = 0x83249478;
    'dispatch: loop {
        match pc {
            0x83249478 => {
    //   block [0x83249478..0x832494D0)
	// 83249478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324947C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249480: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83249484: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249488: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324948C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249490: 388B81F0  addi r4, r11, -0x7e10
	ctx.r[4].s64 = ctx.r[11].s64 + -32272;
	// 83249494: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 83249498: 4AFE3A39  bl 0x8222ced0
	ctx.lr = 0x8324949C;
	sub_8222CED0(ctx, base);
	// 8324949C: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 832494A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832494A4: 4AFA5695  bl 0x821eeb38
	ctx.lr = 0x832494A8;
	sub_821EEB38(ctx, base);
	// 832494A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832494AC: 4B9BA345  bl 0x82c037f0
	ctx.lr = 0x832494B0;
	sub_82C037F0(ctx, base);
	// 832494B0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 832494B4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 832494B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832494BC: 916A7690  stw r11, 0x7690(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30352 as u32), ctx.r[11].u32 ) };
	// 832494C0: 4AF7D2A9  bl 0x821c6768
	ctx.lr = 0x832494C4;
	sub_821C6768(ctx, base);
	// 832494C4: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 832494C8: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 832494CC: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	pc = 0x832494D0; continue 'dispatch;
            }
            0x832494D0 => {
    //   block [0x832494D0..0x832494FC)
	// 832494D0: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 832494D4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832494D8: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 832494DC: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 832494E0: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832494E4: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832494E8: 4082FFE8  bne 0x832494d0
	if !ctx.cr[0].eq {
	pc = 0x832494D0; continue 'dispatch;
	}
	// 832494EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832494F0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 832494F4: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 832494F8: 4AF7D271  bl 0x821c6768
	ctx.lr = 0x832494FC;
	sub_821C6768(ctx, base);
	pc = 0x832494FC; continue 'dispatch;
            }
            0x832494FC => {
    //   block [0x832494FC..0x83249538)
	// 832494FC: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 83249500: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83249504: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 83249508: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 8324950C: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 83249510: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83249514: 4082FFE8  bne 0x832494fc
	if !ctx.cr[0].eq {
	pc = 0x832494FC; continue 'dispatch;
	}
	// 83249518: 3D60832A  lis r11, -0x7cd6
	ctx.r[11].s64 = -2094399488;
	// 8324951C: 386B7AF0  addi r3, r11, 0x7af0
	ctx.r[3].s64 = ctx.r[11].s64 + 31472;
	// 83249520: 4BA60A01  bl 0x82ca9f20
	ctx.lr = 0x83249524;
	sub_82CA9F20(ctx, base);
	// 83249524: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83249528: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324952C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249530: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83249534: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249538 size=64
    let mut pc: u32 = 0x83249538;
    'dispatch: loop {
        match pc {
            0x83249538 => {
    //   block [0x83249538..0x83249578)
	// 83249538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324953C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249540: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249544: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83249548: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324954C: 388B4888  addi r4, r11, 0x4888
	ctx.r[4].s64 = ctx.r[11].s64 + 18568;
	// 83249550: 386A7694  addi r3, r10, 0x7694
	ctx.r[3].s64 = ctx.r[10].s64 + 30356;
	// 83249554: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249558: 4AFE3979  bl 0x8222ced0
	ctx.lr = 0x8324955C;
	sub_8222CED0(ctx, base);
	// 8324955C: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 83249560: 38697AF8  addi r3, r9, 0x7af8
	ctx.r[3].s64 = ctx.r[9].s64 + 31480;
	// 83249564: 4BA609BD  bl 0x82ca9f20
	ctx.lr = 0x83249568;
	sub_82CA9F20(ctx, base);
	// 83249568: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324956C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249570: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249574: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249578 size=64
    let mut pc: u32 = 0x83249578;
    'dispatch: loop {
        match pc {
            0x83249578 => {
    //   block [0x83249578..0x832495B8)
	// 83249578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324957C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249580: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249584: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83249588: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324958C: 388B824C  addi r4, r11, -0x7db4
	ctx.r[4].s64 = ctx.r[11].s64 + -32180;
	// 83249590: 386A7698  addi r3, r10, 0x7698
	ctx.r[3].s64 = ctx.r[10].s64 + 30360;
	// 83249594: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249598: 4AFE3939  bl 0x8222ced0
	ctx.lr = 0x8324959C;
	sub_8222CED0(ctx, base);
	// 8324959C: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 832495A0: 38697B08  addi r3, r9, 0x7b08
	ctx.r[3].s64 = ctx.r[9].s64 + 31496;
	// 832495A4: 4BA6097D  bl 0x82ca9f20
	ctx.lr = 0x832495A8;
	sub_82CA9F20(ctx, base);
	// 832495A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832495AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832495B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832495B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832495B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832495B8 size=204
    let mut pc: u32 = 0x832495B8;
    'dispatch: loop {
        match pc {
            0x832495B8 => {
    //   block [0x832495B8..0x83249614)
	// 832495B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832495BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832495C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832495C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832495C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832495CC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832495D0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832495D4: 388B827C  addi r4, r11, -0x7d84
	ctx.r[4].s64 = ctx.r[11].s64 + -32132;
	// 832495D8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 832495DC: 4AFE38F5  bl 0x8222ced0
	ctx.lr = 0x832495E0;
	sub_8222CED0(ctx, base);
	// 832495E0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 832495E4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832495E8: 4AFA5551  bl 0x821eeb38
	ctx.lr = 0x832495EC;
	sub_821EEB38(ctx, base);
	// 832495EC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832495F0: 4B9BA201  bl 0x82c037f0
	ctx.lr = 0x832495F4;
	sub_82C037F0(ctx, base);
	// 832495F4: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 832495F8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 832495FC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83249600: 916A769C  stw r11, 0x769c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30364 as u32), ctx.r[11].u32 ) };
	// 83249604: 4AF7D165  bl 0x821c6768
	ctx.lr = 0x83249608;
	sub_821C6768(ctx, base);
	// 83249608: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 8324960C: 3BC97088  addi r30, r9, 0x7088
	ctx.r[30].s64 = ctx.r[9].s64 + 28808;
	// 83249610: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	pc = 0x83249614; continue 'dispatch;
            }
            0x83249614 => {
    //   block [0x83249614..0x83249640)
	// 83249614: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 83249618: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324961C: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 83249620: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 83249624: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 83249628: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324962C: 4082FFE8  bne 0x83249614
	if !ctx.cr[0].eq {
	pc = 0x83249614; continue 'dispatch;
	}
	// 83249630: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83249634: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 83249638: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 8324963C: 4AF7D12D  bl 0x821c6768
	ctx.lr = 0x83249640;
	sub_821C6768(ctx, base);
	pc = 0x83249640; continue 'dispatch;
            }
            0x83249640 => {
    //   block [0x83249640..0x83249684)
	// 83249640: 7C8000A6  mfmsr r4
	ctx.r[4].u64 = ctx.msr;
	// 83249644: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83249648: 7CA0F028  lwarx r5, 0, r30
	// lwarx
	let ea = ctx.r[30].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[5].u64 = ctx.reserved.u32 as u64;
	// 8324964C: 38A5FFFF  addi r5, r5, -1
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	// 83249650: 7CA0F12D  stwcx. r5, 0, r30
	// stwcx.
	let addr = ctx.r[30].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[5].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 83249654: 7C810164  mtmsrd r4, 1
	ctx.msr = (ctx.r[4].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83249658: 4082FFE8  bne 0x83249640
	if !ctx.cr[0].eq {
	pc = 0x83249640; continue 'dispatch;
	}
	// 8324965C: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 83249660: 3C60832A  lis r3, -0x7cd6
	ctx.r[3].s64 = -2094399488;
	// 83249664: 38637B18  addi r3, r3, 0x7b18
	ctx.r[3].s64 = ctx.r[3].s64 + 31512;
	// 83249668: 4BA608B9  bl 0x82ca9f20
	ctx.lr = 0x8324966C;
	sub_82CA9F20(ctx, base);
	// 8324966C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83249670: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249674: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249678: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8324967C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83249680: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249688 size=64
    let mut pc: u32 = 0x83249688;
    'dispatch: loop {
        match pc {
            0x83249688 => {
    //   block [0x83249688..0x832496C8)
	// 83249688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324968C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249690: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249694: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83249698: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324969C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 832496A0: 386A76A0  addi r3, r10, 0x76a0
	ctx.r[3].s64 = ctx.r[10].s64 + 30368;
	// 832496A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832496A8: 4AFE3829  bl 0x8222ced0
	ctx.lr = 0x832496AC;
	sub_8222CED0(ctx, base);
	// 832496AC: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 832496B0: 38697B20  addi r3, r9, 0x7b20
	ctx.r[3].s64 = ctx.r[9].s64 + 31520;
	// 832496B4: 4BA6086D  bl 0x82ca9f20
	ctx.lr = 0x832496B8;
	sub_82CA9F20(ctx, base);
	// 832496B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832496BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832496C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832496C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832496C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832496C8 size=64
    let mut pc: u32 = 0x832496C8;
    'dispatch: loop {
        match pc {
            0x832496C8 => {
    //   block [0x832496C8..0x83249708)
	// 832496C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832496CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832496D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832496D4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832496D8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 832496DC: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 832496E0: 386A76A4  addi r3, r10, 0x76a4
	ctx.r[3].s64 = ctx.r[10].s64 + 30372;
	// 832496E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832496E8: 4AFE37E9  bl 0x8222ced0
	ctx.lr = 0x832496EC;
	sub_8222CED0(ctx, base);
	// 832496EC: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 832496F0: 38697B30  addi r3, r9, 0x7b30
	ctx.r[3].s64 = ctx.r[9].s64 + 31536;
	// 832496F4: 4BA6082D  bl 0x82ca9f20
	ctx.lr = 0x832496F8;
	sub_82CA9F20(ctx, base);
	// 832496F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832496FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249700: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249708 size=64
    let mut pc: u32 = 0x83249708;
    'dispatch: loop {
        match pc {
            0x83249708 => {
    //   block [0x83249708..0x83249748)
	// 83249708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324970C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249710: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249714: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83249718: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324971C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83249720: 386A76A8  addi r3, r10, 0x76a8
	ctx.r[3].s64 = ctx.r[10].s64 + 30376;
	// 83249724: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249728: 4AFE37A9  bl 0x8222ced0
	ctx.lr = 0x8324972C;
	sub_8222CED0(ctx, base);
	// 8324972C: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 83249730: 38697B40  addi r3, r9, 0x7b40
	ctx.r[3].s64 = ctx.r[9].s64 + 31552;
	// 83249734: 4BA607ED  bl 0x82ca9f20
	ctx.lr = 0x83249738;
	sub_82CA9F20(ctx, base);
	// 83249738: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324973C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249740: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249744: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83249748 size=12
    let mut pc: u32 = 0x83249748;
    'dispatch: loop {
        match pc {
            0x83249748 => {
    //   block [0x83249748..0x83249754)
	// 83249748: 3D60832A  lis r11, -0x7cd6
	ctx.r[11].s64 = -2094399488;
	// 8324974C: 386B7B50  addi r3, r11, 0x7b50
	ctx.r[3].s64 = ctx.r[11].s64 + 31568;
	// 83249750: 4BA607D0  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249758 size=64
    let mut pc: u32 = 0x83249758;
    'dispatch: loop {
        match pc {
            0x83249758 => {
    //   block [0x83249758..0x83249798)
	// 83249758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324975C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249760: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249764: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83249768: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324976C: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 83249770: 386A76BC  addi r3, r10, 0x76bc
	ctx.r[3].s64 = ctx.r[10].s64 + 30396;
	// 83249774: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249778: 4AFE3759  bl 0x8222ced0
	ctx.lr = 0x8324977C;
	sub_8222CED0(ctx, base);
	// 8324977C: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 83249780: 38697BB8  addi r3, r9, 0x7bb8
	ctx.r[3].s64 = ctx.r[9].s64 + 31672;
	// 83249784: 4BA6079D  bl 0x82ca9f20
	ctx.lr = 0x83249788;
	sub_82CA9F20(ctx, base);
	// 83249788: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324978C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249790: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249794: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249798 size=376
    let mut pc: u32 = 0x83249798;
    'dispatch: loop {
        match pc {
            0x83249798 => {
    //   block [0x83249798..0x83249910)
	// 83249798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324979C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832497A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832497A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832497A8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832497AC: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 832497B0: 3BEB76C0  addi r31, r11, 0x76c0
	ctx.r[31].s64 = ctx.r[11].s64 + 30400;
	// 832497B4: 388A8A50  addi r4, r10, -0x75b0
	ctx.r[4].s64 = ctx.r[10].s64 + -30128;
	// 832497B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832497BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832497C0: 4AFE3711  bl 0x8222ced0
	ctx.lr = 0x832497C4;
	sub_8222CED0(ctx, base);
	// 832497C4: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 832497C8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832497CC: 38898A30  addi r4, r9, -0x75d0
	ctx.r[4].s64 = ctx.r[9].s64 + -30160;
	// 832497D0: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 832497D4: 4AFE36FD  bl 0x8222ced0
	ctx.lr = 0x832497D8;
	sub_8222CED0(ctx, base);
	// 832497D8: 3D00820B  lis r8, -0x7df5
	ctx.r[8].s64 = -2113208320;
	// 832497DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832497E0: 38888A10  addi r4, r8, -0x75f0
	ctx.r[4].s64 = ctx.r[8].s64 + -30192;
	// 832497E4: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 832497E8: 4AFE36E9  bl 0x8222ced0
	ctx.lr = 0x832497EC;
	sub_8222CED0(ctx, base);
	// 832497EC: 3CE0820B  lis r7, -0x7df5
	ctx.r[7].s64 = -2113208320;
	// 832497F0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832497F4: 388789F0  addi r4, r7, -0x7610
	ctx.r[4].s64 = ctx.r[7].s64 + -30224;
	// 832497F8: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 832497FC: 4AFE36D5  bl 0x8222ced0
	ctx.lr = 0x83249800;
	sub_8222CED0(ctx, base);
	// 83249800: 3CC0820B  lis r6, -0x7df5
	ctx.r[6].s64 = -2113208320;
	// 83249804: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249808: 388689D0  addi r4, r6, -0x7630
	ctx.r[4].s64 = ctx.r[6].s64 + -30256;
	// 8324980C: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 83249810: 4AFE36C1  bl 0x8222ced0
	ctx.lr = 0x83249814;
	sub_8222CED0(ctx, base);
	// 83249814: 3C80820B  lis r4, -0x7df5
	ctx.r[4].s64 = -2113208320;
	// 83249818: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324981C: 388489B0  addi r4, r4, -0x7650
	ctx.r[4].s64 = ctx.r[4].s64 + -30288;
	// 83249820: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 83249824: 4AFE36AD  bl 0x8222ced0
	ctx.lr = 0x83249828;
	sub_8222CED0(ctx, base);
	// 83249828: 3C60820B  lis r3, -0x7df5
	ctx.r[3].s64 = -2113208320;
	// 8324982C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249830: 38838990  addi r4, r3, -0x7670
	ctx.r[4].s64 = ctx.r[3].s64 + -30320;
	// 83249834: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 83249838: 4AFE3699  bl 0x8222ced0
	ctx.lr = 0x8324983C;
	sub_8222CED0(ctx, base);
	// 8324983C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83249840: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249844: 388B8970  addi r4, r11, -0x7690
	ctx.r[4].s64 = ctx.r[11].s64 + -30352;
	// 83249848: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 8324984C: 4AFE3685  bl 0x8222ced0
	ctx.lr = 0x83249850;
	sub_8222CED0(ctx, base);
	// 83249850: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 83249854: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249858: 388A8950  addi r4, r10, -0x76b0
	ctx.r[4].s64 = ctx.r[10].s64 + -30384;
	// 8324985C: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 83249860: 4AFE3671  bl 0x8222ced0
	ctx.lr = 0x83249864;
	sub_8222CED0(ctx, base);
	// 83249864: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 83249868: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324986C: 38898930  addi r4, r9, -0x76d0
	ctx.r[4].s64 = ctx.r[9].s64 + -30416;
	// 83249870: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 83249874: 4AFE365D  bl 0x8222ced0
	ctx.lr = 0x83249878;
	sub_8222CED0(ctx, base);
	// 83249878: 3D00820B  lis r8, -0x7df5
	ctx.r[8].s64 = -2113208320;
	// 8324987C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249880: 38888914  addi r4, r8, -0x76ec
	ctx.r[4].s64 = ctx.r[8].s64 + -30444;
	// 83249884: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 83249888: 4AFE3649  bl 0x8222ced0
	ctx.lr = 0x8324988C;
	sub_8222CED0(ctx, base);
	// 8324988C: 3CE0820B  lis r7, -0x7df5
	ctx.r[7].s64 = -2113208320;
	// 83249890: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249894: 388788F0  addi r4, r7, -0x7710
	ctx.r[4].s64 = ctx.r[7].s64 + -30480;
	// 83249898: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 8324989C: 4AFE3635  bl 0x8222ced0
	ctx.lr = 0x832498A0;
	sub_8222CED0(ctx, base);
	// 832498A0: 3CC0820B  lis r6, -0x7df5
	ctx.r[6].s64 = -2113208320;
	// 832498A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832498A8: 388688CC  addi r4, r6, -0x7734
	ctx.r[4].s64 = ctx.r[6].s64 + -30516;
	// 832498AC: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 832498B0: 4AFE3621  bl 0x8222ced0
	ctx.lr = 0x832498B4;
	sub_8222CED0(ctx, base);
	// 832498B4: 3C80820B  lis r4, -0x7df5
	ctx.r[4].s64 = -2113208320;
	// 832498B8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832498BC: 388488AC  addi r4, r4, -0x7754
	ctx.r[4].s64 = ctx.r[4].s64 + -30548;
	// 832498C0: 387F0034  addi r3, r31, 0x34
	ctx.r[3].s64 = ctx.r[31].s64 + 52;
	// 832498C4: 4AFE360D  bl 0x8222ced0
	ctx.lr = 0x832498C8;
	sub_8222CED0(ctx, base);
	// 832498C8: 3C60820B  lis r3, -0x7df5
	ctx.r[3].s64 = -2113208320;
	// 832498CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832498D0: 3883888C  addi r4, r3, -0x7774
	ctx.r[4].s64 = ctx.r[3].s64 + -30580;
	// 832498D4: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 832498D8: 4AFE35F9  bl 0x8222ced0
	ctx.lr = 0x832498DC;
	sub_8222CED0(ctx, base);
	// 832498DC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832498E0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832498E4: 388B8868  addi r4, r11, -0x7798
	ctx.r[4].s64 = ctx.r[11].s64 + -30616;
	// 832498E8: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 832498EC: 4AFE35E5  bl 0x8222ced0
	ctx.lr = 0x832498F0;
	sub_8222CED0(ctx, base);
	// 832498F0: 3D40832A  lis r10, -0x7cd6
	ctx.r[10].s64 = -2094399488;
	// 832498F4: 386A7BC8  addi r3, r10, 0x7bc8
	ctx.r[3].s64 = ctx.r[10].s64 + 31688;
	// 832498F8: 4BA60629  bl 0x82ca9f20
	ctx.lr = 0x832498FC;
	sub_82CA9F20(ctx, base);
	// 832498FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249900: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249904: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249908: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8324990C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249910 size=376
    let mut pc: u32 = 0x83249910;
    'dispatch: loop {
        match pc {
            0x83249910 => {
    //   block [0x83249910..0x83249A88)
	// 83249910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249918: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8324991C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249920: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 83249924: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 83249928: 3BEB7700  addi r31, r11, 0x7700
	ctx.r[31].s64 = ctx.r[11].s64 + 30464;
	// 8324992C: 388A8C58  addi r4, r10, -0x73a8
	ctx.r[4].s64 = ctx.r[10].s64 + -29608;
	// 83249930: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83249934: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249938: 4AFE3599  bl 0x8222ced0
	ctx.lr = 0x8324993C;
	sub_8222CED0(ctx, base);
	// 8324993C: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 83249940: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249944: 38898C38  addi r4, r9, -0x73c8
	ctx.r[4].s64 = ctx.r[9].s64 + -29640;
	// 83249948: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8324994C: 4AFE3585  bl 0x8222ced0
	ctx.lr = 0x83249950;
	sub_8222CED0(ctx, base);
	// 83249950: 3D00820B  lis r8, -0x7df5
	ctx.r[8].s64 = -2113208320;
	// 83249954: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249958: 38888C18  addi r4, r8, -0x73e8
	ctx.r[4].s64 = ctx.r[8].s64 + -29672;
	// 8324995C: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 83249960: 4AFE3571  bl 0x8222ced0
	ctx.lr = 0x83249964;
	sub_8222CED0(ctx, base);
	// 83249964: 3CE0820B  lis r7, -0x7df5
	ctx.r[7].s64 = -2113208320;
	// 83249968: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324996C: 38878BF8  addi r4, r7, -0x7408
	ctx.r[4].s64 = ctx.r[7].s64 + -29704;
	// 83249970: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 83249974: 4AFE355D  bl 0x8222ced0
	ctx.lr = 0x83249978;
	sub_8222CED0(ctx, base);
	// 83249978: 3CC0820B  lis r6, -0x7df5
	ctx.r[6].s64 = -2113208320;
	// 8324997C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249980: 38868BD8  addi r4, r6, -0x7428
	ctx.r[4].s64 = ctx.r[6].s64 + -29736;
	// 83249984: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 83249988: 4AFE3549  bl 0x8222ced0
	ctx.lr = 0x8324998C;
	sub_8222CED0(ctx, base);
	// 8324998C: 3C80820B  lis r4, -0x7df5
	ctx.r[4].s64 = -2113208320;
	// 83249990: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249994: 38848BB8  addi r4, r4, -0x7448
	ctx.r[4].s64 = ctx.r[4].s64 + -29768;
	// 83249998: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 8324999C: 4AFE3535  bl 0x8222ced0
	ctx.lr = 0x832499A0;
	sub_8222CED0(ctx, base);
	// 832499A0: 3C60820B  lis r3, -0x7df5
	ctx.r[3].s64 = -2113208320;
	// 832499A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832499A8: 38838B98  addi r4, r3, -0x7468
	ctx.r[4].s64 = ctx.r[3].s64 + -29800;
	// 832499AC: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 832499B0: 4AFE3521  bl 0x8222ced0
	ctx.lr = 0x832499B4;
	sub_8222CED0(ctx, base);
	// 832499B4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832499B8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832499BC: 388B8B78  addi r4, r11, -0x7488
	ctx.r[4].s64 = ctx.r[11].s64 + -29832;
	// 832499C0: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 832499C4: 4AFE350D  bl 0x8222ced0
	ctx.lr = 0x832499C8;
	sub_8222CED0(ctx, base);
	// 832499C8: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 832499CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832499D0: 388A8B58  addi r4, r10, -0x74a8
	ctx.r[4].s64 = ctx.r[10].s64 + -29864;
	// 832499D4: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 832499D8: 4AFE34F9  bl 0x8222ced0
	ctx.lr = 0x832499DC;
	sub_8222CED0(ctx, base);
	// 832499DC: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 832499E0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832499E4: 38898B38  addi r4, r9, -0x74c8
	ctx.r[4].s64 = ctx.r[9].s64 + -29896;
	// 832499E8: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 832499EC: 4AFE34E5  bl 0x8222ced0
	ctx.lr = 0x832499F0;
	sub_8222CED0(ctx, base);
	// 832499F0: 3D00820B  lis r8, -0x7df5
	ctx.r[8].s64 = -2113208320;
	// 832499F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832499F8: 38888B1C  addi r4, r8, -0x74e4
	ctx.r[4].s64 = ctx.r[8].s64 + -29924;
	// 832499FC: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 83249A00: 4AFE34D1  bl 0x8222ced0
	ctx.lr = 0x83249A04;
	sub_8222CED0(ctx, base);
	// 83249A04: 3CE0820B  lis r7, -0x7df5
	ctx.r[7].s64 = -2113208320;
	// 83249A08: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249A0C: 38878AF8  addi r4, r7, -0x7508
	ctx.r[4].s64 = ctx.r[7].s64 + -29960;
	// 83249A10: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 83249A14: 4AFE34BD  bl 0x8222ced0
	ctx.lr = 0x83249A18;
	sub_8222CED0(ctx, base);
	// 83249A18: 3CC0820B  lis r6, -0x7df5
	ctx.r[6].s64 = -2113208320;
	// 83249A1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249A20: 38868AD4  addi r4, r6, -0x752c
	ctx.r[4].s64 = ctx.r[6].s64 + -29996;
	// 83249A24: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 83249A28: 4AFE34A9  bl 0x8222ced0
	ctx.lr = 0x83249A2C;
	sub_8222CED0(ctx, base);
	// 83249A2C: 3C80820B  lis r4, -0x7df5
	ctx.r[4].s64 = -2113208320;
	// 83249A30: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249A34: 38848AB4  addi r4, r4, -0x754c
	ctx.r[4].s64 = ctx.r[4].s64 + -30028;
	// 83249A38: 387F0034  addi r3, r31, 0x34
	ctx.r[3].s64 = ctx.r[31].s64 + 52;
	// 83249A3C: 4AFE3495  bl 0x8222ced0
	ctx.lr = 0x83249A40;
	sub_8222CED0(ctx, base);
	// 83249A40: 3C60820B  lis r3, -0x7df5
	ctx.r[3].s64 = -2113208320;
	// 83249A44: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249A48: 38838A94  addi r4, r3, -0x756c
	ctx.r[4].s64 = ctx.r[3].s64 + -30060;
	// 83249A4C: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 83249A50: 4AFE3481  bl 0x8222ced0
	ctx.lr = 0x83249A54;
	sub_8222CED0(ctx, base);
	// 83249A54: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83249A58: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249A5C: 388B8A70  addi r4, r11, -0x7590
	ctx.r[4].s64 = ctx.r[11].s64 + -30096;
	// 83249A60: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 83249A64: 4AFE346D  bl 0x8222ced0
	ctx.lr = 0x83249A68;
	sub_8222CED0(ctx, base);
	// 83249A68: 3D40832A  lis r10, -0x7cd6
	ctx.r[10].s64 = -2094399488;
	// 83249A6C: 386A7C30  addi r3, r10, 0x7c30
	ctx.r[3].s64 = ctx.r[10].s64 + 31792;
	// 83249A70: 4BA604B1  bl 0x82ca9f20
	ctx.lr = 0x83249A74;
	sub_82CA9F20(ctx, base);
	// 83249A74: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249A78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249A7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249A80: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83249A84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249A88 size=56
    let mut pc: u32 = 0x83249A88;
    'dispatch: loop {
        match pc {
            0x83249A88 => {
    //   block [0x83249A88..0x83249AC0)
	// 83249A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249A8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249A90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249A94: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83249A98: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83249A9C: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 83249AA0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83249AA4: 4AFAA2B5  bl 0x821f3d58
	ctx.lr = 0x83249AA8;
	sub_821F3D58(ctx, base);
	// 83249AA8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249AAC: 906A7740  stw r3, 0x7740(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30528 as u32), ctx.r[3].u32 ) };
	// 83249AB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249AB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249AB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249ABC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249AC0 size=56
    let mut pc: u32 = 0x83249AC0;
    'dispatch: loop {
        match pc {
            0x83249AC0 => {
    //   block [0x83249AC0..0x83249AF8)
	// 83249AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249AC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249AC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249ACC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83249AD0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83249AD4: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 83249AD8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83249ADC: 4AFAA27D  bl 0x821f3d58
	ctx.lr = 0x83249AE0;
	sub_821F3D58(ctx, base);
	// 83249AE0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249AE4: 906A7744  stw r3, 0x7744(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30532 as u32), ctx.r[3].u32 ) };
	// 83249AE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249AEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249AF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249AF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249AF8 size=56
    let mut pc: u32 = 0x83249AF8;
    'dispatch: loop {
        match pc {
            0x83249AF8 => {
    //   block [0x83249AF8..0x83249B30)
	// 83249AF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249AFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249B00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249B04: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83249B08: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83249B0C: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 83249B10: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83249B14: 4AFAA245  bl 0x821f3d58
	ctx.lr = 0x83249B18;
	sub_821F3D58(ctx, base);
	// 83249B18: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249B1C: 906A7748  stw r3, 0x7748(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30536 as u32), ctx.r[3].u32 ) };
	// 83249B20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249B24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249B28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249B2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249B30 size=56
    let mut pc: u32 = 0x83249B30;
    'dispatch: loop {
        match pc {
            0x83249B30 => {
    //   block [0x83249B30..0x83249B68)
	// 83249B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249B38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249B3C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83249B40: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83249B44: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 83249B48: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83249B4C: 4AFAA20D  bl 0x821f3d58
	ctx.lr = 0x83249B50;
	sub_821F3D58(ctx, base);
	// 83249B50: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249B54: 906A774C  stw r3, 0x774c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30540 as u32), ctx.r[3].u32 ) };
	// 83249B58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249B5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249B60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249B64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249B68 size=56
    let mut pc: u32 = 0x83249B68;
    'dispatch: loop {
        match pc {
            0x83249B68 => {
    //   block [0x83249B68..0x83249BA0)
	// 83249B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249B6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249B70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249B74: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83249B78: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83249B7C: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 83249B80: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83249B84: 4AFAA1D5  bl 0x821f3d58
	ctx.lr = 0x83249B88;
	sub_821F3D58(ctx, base);
	// 83249B88: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249B8C: 906A7750  stw r3, 0x7750(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30544 as u32), ctx.r[3].u32 ) };
	// 83249B90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249B94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249B98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249B9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249BA0 size=56
    let mut pc: u32 = 0x83249BA0;
    'dispatch: loop {
        match pc {
            0x83249BA0 => {
    //   block [0x83249BA0..0x83249BD8)
	// 83249BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249BA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249BA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249BAC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83249BB0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83249BB4: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 83249BB8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83249BBC: 4AFAA19D  bl 0x821f3d58
	ctx.lr = 0x83249BC0;
	sub_821F3D58(ctx, base);
	// 83249BC0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249BC4: 906A7754  stw r3, 0x7754(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30548 as u32), ctx.r[3].u32 ) };
	// 83249BC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249BCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249BD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249BD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249BD8 size=56
    let mut pc: u32 = 0x83249BD8;
    'dispatch: loop {
        match pc {
            0x83249BD8 => {
    //   block [0x83249BD8..0x83249C10)
	// 83249BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249BDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249BE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249BE4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83249BE8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83249BEC: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 83249BF0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83249BF4: 4AFAA165  bl 0x821f3d58
	ctx.lr = 0x83249BF8;
	sub_821F3D58(ctx, base);
	// 83249BF8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249BFC: 906A7758  stw r3, 0x7758(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30552 as u32), ctx.r[3].u32 ) };
	// 83249C00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249C04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249C08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249C0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249C10 size=56
    let mut pc: u32 = 0x83249C10;
    'dispatch: loop {
        match pc {
            0x83249C10 => {
    //   block [0x83249C10..0x83249C48)
	// 83249C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249C14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249C18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249C1C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83249C20: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83249C24: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 83249C28: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83249C2C: 4AFAA12D  bl 0x821f3d58
	ctx.lr = 0x83249C30;
	sub_821F3D58(ctx, base);
	// 83249C30: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249C34: 906A775C  stw r3, 0x775c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30556 as u32), ctx.r[3].u32 ) };
	// 83249C38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249C3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249C40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249C44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249C48 size=56
    let mut pc: u32 = 0x83249C48;
    'dispatch: loop {
        match pc {
            0x83249C48 => {
    //   block [0x83249C48..0x83249C80)
	// 83249C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249C4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249C50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249C54: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83249C58: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83249C5C: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 83249C60: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83249C64: 4AFAA0F5  bl 0x821f3d58
	ctx.lr = 0x83249C68;
	sub_821F3D58(ctx, base);
	// 83249C68: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249C6C: 906A7760  stw r3, 0x7760(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30560 as u32), ctx.r[3].u32 ) };
	// 83249C70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249C74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249C78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249C7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249C80 size=56
    let mut pc: u32 = 0x83249C80;
    'dispatch: loop {
        match pc {
            0x83249C80 => {
    //   block [0x83249C80..0x83249CB8)
	// 83249C80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249C84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249C88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249C8C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83249C90: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83249C94: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 83249C98: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83249C9C: 4AFAA0BD  bl 0x821f3d58
	ctx.lr = 0x83249CA0;
	sub_821F3D58(ctx, base);
	// 83249CA0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249CA4: 906A7764  stw r3, 0x7764(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30564 as u32), ctx.r[3].u32 ) };
	// 83249CA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249CAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249CB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249CB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249CB8 size=56
    let mut pc: u32 = 0x83249CB8;
    'dispatch: loop {
        match pc {
            0x83249CB8 => {
    //   block [0x83249CB8..0x83249CF0)
	// 83249CB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249CBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249CC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249CC4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83249CC8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83249CCC: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 83249CD0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83249CD4: 4AFAA085  bl 0x821f3d58
	ctx.lr = 0x83249CD8;
	sub_821F3D58(ctx, base);
	// 83249CD8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249CDC: 906A7768  stw r3, 0x7768(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30568 as u32), ctx.r[3].u32 ) };
	// 83249CE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249CE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249CE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249CEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249CF0 size=56
    let mut pc: u32 = 0x83249CF0;
    'dispatch: loop {
        match pc {
            0x83249CF0 => {
    //   block [0x83249CF0..0x83249D28)
	// 83249CF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249CF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249CF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249CFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83249D00: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83249D04: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 83249D08: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83249D0C: 4AFAA04D  bl 0x821f3d58
	ctx.lr = 0x83249D10;
	sub_821F3D58(ctx, base);
	// 83249D10: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249D14: 906A776C  stw r3, 0x776c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30572 as u32), ctx.r[3].u32 ) };
	// 83249D18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249D1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249D20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249D24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249D28 size=56
    let mut pc: u32 = 0x83249D28;
    'dispatch: loop {
        match pc {
            0x83249D28 => {
    //   block [0x83249D28..0x83249D60)
	// 83249D28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249D2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249D30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249D34: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83249D38: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83249D3C: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 83249D40: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83249D44: 4AFAA015  bl 0x821f3d58
	ctx.lr = 0x83249D48;
	sub_821F3D58(ctx, base);
	// 83249D48: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249D4C: 906A7770  stw r3, 0x7770(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30576 as u32), ctx.r[3].u32 ) };
	// 83249D50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249D54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249D58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249D5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249D60 size=56
    let mut pc: u32 = 0x83249D60;
    'dispatch: loop {
        match pc {
            0x83249D60 => {
    //   block [0x83249D60..0x83249D98)
	// 83249D60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249D64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249D68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249D6C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83249D70: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83249D74: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 83249D78: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83249D7C: 4AFA9FDD  bl 0x821f3d58
	ctx.lr = 0x83249D80;
	sub_821F3D58(ctx, base);
	// 83249D80: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249D84: 906A7774  stw r3, 0x7774(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30580 as u32), ctx.r[3].u32 ) };
	// 83249D88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249D8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249D90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249D94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249D98 size=56
    let mut pc: u32 = 0x83249D98;
    'dispatch: loop {
        match pc {
            0x83249D98 => {
    //   block [0x83249D98..0x83249DD0)
	// 83249D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249D9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249DA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249DA4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83249DA8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83249DAC: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 83249DB0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83249DB4: 4AFA9FA5  bl 0x821f3d58
	ctx.lr = 0x83249DB8;
	sub_821F3D58(ctx, base);
	// 83249DB8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249DBC: 906A7778  stw r3, 0x7778(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30584 as u32), ctx.r[3].u32 ) };
	// 83249DC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249DC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249DC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249DCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249DD0 size=56
    let mut pc: u32 = 0x83249DD0;
    'dispatch: loop {
        match pc {
            0x83249DD0 => {
    //   block [0x83249DD0..0x83249E08)
	// 83249DD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249DD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249DD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249DDC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83249DE0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83249DE4: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 83249DE8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83249DEC: 4AFA9F6D  bl 0x821f3d58
	ctx.lr = 0x83249DF0;
	sub_821F3D58(ctx, base);
	// 83249DF0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249DF4: 906A777C  stw r3, 0x777c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30588 as u32), ctx.r[3].u32 ) };
	// 83249DF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249DFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249E00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249E04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249E08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249E08 size=56
    let mut pc: u32 = 0x83249E08;
    'dispatch: loop {
        match pc {
            0x83249E08 => {
    //   block [0x83249E08..0x83249E40)
	// 83249E08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249E0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249E10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249E14: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83249E18: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83249E1C: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 83249E20: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83249E24: 4AFA9F35  bl 0x821f3d58
	ctx.lr = 0x83249E28;
	sub_821F3D58(ctx, base);
	// 83249E28: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249E2C: 906A7780  stw r3, 0x7780(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30592 as u32), ctx.r[3].u32 ) };
	// 83249E30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249E34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249E38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249E3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249E40 size=56
    let mut pc: u32 = 0x83249E40;
    'dispatch: loop {
        match pc {
            0x83249E40 => {
    //   block [0x83249E40..0x83249E78)
	// 83249E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249E44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249E48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249E4C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83249E50: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83249E54: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 83249E58: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83249E5C: 4AFA9EFD  bl 0x821f3d58
	ctx.lr = 0x83249E60;
	sub_821F3D58(ctx, base);
	// 83249E60: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249E64: 906A7784  stw r3, 0x7784(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30596 as u32), ctx.r[3].u32 ) };
	// 83249E68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249E6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249E70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249E74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249E78 size=56
    let mut pc: u32 = 0x83249E78;
    'dispatch: loop {
        match pc {
            0x83249E78 => {
    //   block [0x83249E78..0x83249EB0)
	// 83249E78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249E7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249E80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249E84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83249E88: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83249E8C: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 83249E90: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83249E94: 4AFA9EC5  bl 0x821f3d58
	ctx.lr = 0x83249E98;
	sub_821F3D58(ctx, base);
	// 83249E98: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249E9C: 906A7788  stw r3, 0x7788(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30600 as u32), ctx.r[3].u32 ) };
	// 83249EA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249EA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249EA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249EAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249EB0 size=56
    let mut pc: u32 = 0x83249EB0;
    'dispatch: loop {
        match pc {
            0x83249EB0 => {
    //   block [0x83249EB0..0x83249EE8)
	// 83249EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249EB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249EB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249EBC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83249EC0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83249EC4: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 83249EC8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83249ECC: 4AFA9E8D  bl 0x821f3d58
	ctx.lr = 0x83249ED0;
	sub_821F3D58(ctx, base);
	// 83249ED0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249ED4: 906A778C  stw r3, 0x778c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30604 as u32), ctx.r[3].u32 ) };
	// 83249ED8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249EDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249EE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249EE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249EE8 size=56
    let mut pc: u32 = 0x83249EE8;
    'dispatch: loop {
        match pc {
            0x83249EE8 => {
    //   block [0x83249EE8..0x83249F20)
	// 83249EE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249EEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249EF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249EF4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83249EF8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83249EFC: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 83249F00: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83249F04: 4AFA9E55  bl 0x821f3d58
	ctx.lr = 0x83249F08;
	sub_821F3D58(ctx, base);
	// 83249F08: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249F0C: 906A7790  stw r3, 0x7790(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30608 as u32), ctx.r[3].u32 ) };
	// 83249F10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249F14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249F18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249F1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249F20 size=64
    let mut pc: u32 = 0x83249F20;
    'dispatch: loop {
        match pc {
            0x83249F20 => {
    //   block [0x83249F20..0x83249F60)
	// 83249F20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249F24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249F28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249F2C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83249F30: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249F34: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83249F38: 386A7794  addi r3, r10, 0x7794
	ctx.r[3].s64 = ctx.r[10].s64 + 30612;
	// 83249F3C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249F40: 4AFE2F91  bl 0x8222ced0
	ctx.lr = 0x83249F44;
	sub_8222CED0(ctx, base);
	// 83249F44: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 83249F48: 38697C98  addi r3, r9, 0x7c98
	ctx.r[3].s64 = ctx.r[9].s64 + 31896;
	// 83249F4C: 4BA5FFD5  bl 0x82ca9f20
	ctx.lr = 0x83249F50;
	sub_82CA9F20(ctx, base);
	// 83249F50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249F54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249F58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249F5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249F60 size=64
    let mut pc: u32 = 0x83249F60;
    'dispatch: loop {
        match pc {
            0x83249F60 => {
    //   block [0x83249F60..0x83249FA0)
	// 83249F60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249F64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249F68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249F6C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83249F70: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249F74: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83249F78: 386A7798  addi r3, r10, 0x7798
	ctx.r[3].s64 = ctx.r[10].s64 + 30616;
	// 83249F7C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249F80: 4AFE2F51  bl 0x8222ced0
	ctx.lr = 0x83249F84;
	sub_8222CED0(ctx, base);
	// 83249F84: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 83249F88: 38697CA8  addi r3, r9, 0x7ca8
	ctx.r[3].s64 = ctx.r[9].s64 + 31912;
	// 83249F8C: 4BA5FF95  bl 0x82ca9f20
	ctx.lr = 0x83249F90;
	sub_82CA9F20(ctx, base);
	// 83249F90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249F94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249F98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249F9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249FA0 size=64
    let mut pc: u32 = 0x83249FA0;
    'dispatch: loop {
        match pc {
            0x83249FA0 => {
    //   block [0x83249FA0..0x83249FE0)
	// 83249FA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249FA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249FA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249FAC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83249FB0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249FB4: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83249FB8: 386A779C  addi r3, r10, 0x779c
	ctx.r[3].s64 = ctx.r[10].s64 + 30620;
	// 83249FBC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249FC0: 4AFE2F11  bl 0x8222ced0
	ctx.lr = 0x83249FC4;
	sub_8222CED0(ctx, base);
	// 83249FC4: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 83249FC8: 38697CB8  addi r3, r9, 0x7cb8
	ctx.r[3].s64 = ctx.r[9].s64 + 31928;
	// 83249FCC: 4BA5FF55  bl 0x82ca9f20
	ctx.lr = 0x83249FD0;
	sub_82CA9F20(ctx, base);
	// 83249FD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249FD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249FD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249FDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249FE0 size=64
    let mut pc: u32 = 0x83249FE0;
    'dispatch: loop {
        match pc {
            0x83249FE0 => {
    //   block [0x83249FE0..0x8324A020)
	// 83249FE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249FE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249FE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249FEC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83249FF0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249FF4: 388B8C78  addi r4, r11, -0x7388
	ctx.r[4].s64 = ctx.r[11].s64 + -29576;
	// 83249FF8: 386A77A0  addi r3, r10, 0x77a0
	ctx.r[3].s64 = ctx.r[10].s64 + 30624;
	// 83249FFC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324A000: 4AFE2ED1  bl 0x8222ced0
	ctx.lr = 0x8324A004;
	sub_8222CED0(ctx, base);
	// 8324A004: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 8324A008: 38697CC8  addi r3, r9, 0x7cc8
	ctx.r[3].s64 = ctx.r[9].s64 + 31944;
	// 8324A00C: 4BA5FF15  bl 0x82ca9f20
	ctx.lr = 0x8324A010;
	sub_82CA9F20(ctx, base);
	// 8324A010: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324A014: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324A018: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324A01C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324A020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324A020 size=64
    let mut pc: u32 = 0x8324A020;
    'dispatch: loop {
        match pc {
            0x8324A020 => {
    //   block [0x8324A020..0x8324A060)
	// 8324A020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324A024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324A028: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324A02C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324A030: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324A034: 388B8C88  addi r4, r11, -0x7378
	ctx.r[4].s64 = ctx.r[11].s64 + -29560;
	// 8324A038: 386A77A4  addi r3, r10, 0x77a4
	ctx.r[3].s64 = ctx.r[10].s64 + 30628;
	// 8324A03C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324A040: 4AFE2E91  bl 0x8222ced0
	ctx.lr = 0x8324A044;
	sub_8222CED0(ctx, base);
	// 8324A044: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 8324A048: 38697CD8  addi r3, r9, 0x7cd8
	ctx.r[3].s64 = ctx.r[9].s64 + 31960;
	// 8324A04C: 4BA5FED5  bl 0x82ca9f20
	ctx.lr = 0x8324A050;
	sub_82CA9F20(ctx, base);
	// 8324A050: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324A054: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324A058: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324A05C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324A060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324A060 size=64
    let mut pc: u32 = 0x8324A060;
    'dispatch: loop {
        match pc {
            0x8324A060 => {
    //   block [0x8324A060..0x8324A0A0)
	// 8324A060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324A064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324A068: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324A06C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324A070: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324A074: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 8324A078: 386A77A8  addi r3, r10, 0x77a8
	ctx.r[3].s64 = ctx.r[10].s64 + 30632;
	// 8324A07C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324A080: 4AFE2E51  bl 0x8222ced0
	ctx.lr = 0x8324A084;
	sub_8222CED0(ctx, base);
	// 8324A084: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 8324A088: 38697CE8  addi r3, r9, 0x7ce8
	ctx.r[3].s64 = ctx.r[9].s64 + 31976;
	// 8324A08C: 4BA5FE95  bl 0x82ca9f20
	ctx.lr = 0x8324A090;
	sub_82CA9F20(ctx, base);
	// 8324A090: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324A094: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324A098: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324A09C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324A0A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324A0A0 size=64
    let mut pc: u32 = 0x8324A0A0;
    'dispatch: loop {
        match pc {
            0x8324A0A0 => {
    //   block [0x8324A0A0..0x8324A0E0)
	// 8324A0A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324A0A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324A0A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324A0AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324A0B0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324A0B4: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8324A0B8: 386A77AC  addi r3, r10, 0x77ac
	ctx.r[3].s64 = ctx.r[10].s64 + 30636;
	// 8324A0BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324A0C0: 4AFE2E11  bl 0x8222ced0
	ctx.lr = 0x8324A0C4;
	sub_8222CED0(ctx, base);
	// 8324A0C4: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 8324A0C8: 38697CF8  addi r3, r9, 0x7cf8
	ctx.r[3].s64 = ctx.r[9].s64 + 31992;
	// 8324A0CC: 4BA5FE55  bl 0x82ca9f20
	ctx.lr = 0x8324A0D0;
	sub_82CA9F20(ctx, base);
	// 8324A0D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324A0D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324A0D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324A0DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324A0E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324A0E0 size=64
    let mut pc: u32 = 0x8324A0E0;
    'dispatch: loop {
        match pc {
            0x8324A0E0 => {
    //   block [0x8324A0E0..0x8324A120)
	// 8324A0E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324A0E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324A0E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324A0EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324A0F0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324A0F4: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8324A0F8: 386A77B0  addi r3, r10, 0x77b0
	ctx.r[3].s64 = ctx.r[10].s64 + 30640;
	// 8324A0FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324A100: 4AFE2DD1  bl 0x8222ced0
	ctx.lr = 0x8324A104;
	sub_8222CED0(ctx, base);
	// 8324A104: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 8324A108: 38697D08  addi r3, r9, 0x7d08
	ctx.r[3].s64 = ctx.r[9].s64 + 32008;
	// 8324A10C: 4BA5FE15  bl 0x82ca9f20
	ctx.lr = 0x8324A110;
	sub_82CA9F20(ctx, base);
	// 8324A110: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324A114: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324A118: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324A11C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324A120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324A120 size=64
    let mut pc: u32 = 0x8324A120;
    'dispatch: loop {
        match pc {
            0x8324A120 => {
    //   block [0x8324A120..0x8324A160)
	// 8324A120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324A124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324A128: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324A12C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324A130: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324A134: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8324A138: 386A77B4  addi r3, r10, 0x77b4
	ctx.r[3].s64 = ctx.r[10].s64 + 30644;
	// 8324A13C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324A140: 4AFE2D91  bl 0x8222ced0
	ctx.lr = 0x8324A144;
	sub_8222CED0(ctx, base);
	// 8324A144: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 8324A148: 38697D18  addi r3, r9, 0x7d18
	ctx.r[3].s64 = ctx.r[9].s64 + 32024;
	// 8324A14C: 4BA5FDD5  bl 0x82ca9f20
	ctx.lr = 0x8324A150;
	sub_82CA9F20(ctx, base);
	// 8324A150: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324A154: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324A158: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324A15C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324A160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324A160 size=64
    let mut pc: u32 = 0x8324A160;
    'dispatch: loop {
        match pc {
            0x8324A160 => {
    //   block [0x8324A160..0x8324A1A0)
	// 8324A160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324A164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324A168: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324A16C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324A170: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324A174: 388B8E84  addi r4, r11, -0x717c
	ctx.r[4].s64 = ctx.r[11].s64 + -29052;
	// 8324A178: 386A77B8  addi r3, r10, 0x77b8
	ctx.r[3].s64 = ctx.r[10].s64 + 30648;
	// 8324A17C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324A180: 4AFE2D51  bl 0x8222ced0
	ctx.lr = 0x8324A184;
	sub_8222CED0(ctx, base);
	// 8324A184: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 8324A188: 38697D28  addi r3, r9, 0x7d28
	ctx.r[3].s64 = ctx.r[9].s64 + 32040;
	// 8324A18C: 4BA5FD95  bl 0x82ca9f20
	ctx.lr = 0x8324A190;
	sub_82CA9F20(ctx, base);
	// 8324A190: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324A194: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324A198: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324A19C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324A1A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324A1A0 size=56
    let mut pc: u32 = 0x8324A1A0;
    'dispatch: loop {
        match pc {
            0x8324A1A0 => {
    //   block [0x8324A1A0..0x8324A1D8)
	// 8324A1A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324A1A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324A1A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324A1AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324A1B0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324A1B4: 386B4CEC  addi r3, r11, 0x4cec
	ctx.r[3].s64 = ctx.r[11].s64 + 19692;
	// 8324A1B8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324A1BC: 4AFA9B9D  bl 0x821f3d58
	ctx.lr = 0x8324A1C0;
	sub_821F3D58(ctx, base);
	// 8324A1C0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324A1C4: 906A77BC  stw r3, 0x77bc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30652 as u32), ctx.r[3].u32 ) };
	// 8324A1C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324A1CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324A1D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324A1D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324A1D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324A1D8 size=64
    let mut pc: u32 = 0x8324A1D8;
    'dispatch: loop {
        match pc {
            0x8324A1D8 => {
    //   block [0x8324A1D8..0x8324A218)
	// 8324A1D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324A1DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324A1E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324A1E4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324A1E8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324A1EC: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8324A1F0: 386A77C0  addi r3, r10, 0x77c0
	ctx.r[3].s64 = ctx.r[10].s64 + 30656;
	// 8324A1F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324A1F8: 4AFE2CD9  bl 0x8222ced0
	ctx.lr = 0x8324A1FC;
	sub_8222CED0(ctx, base);
	// 8324A1FC: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 8324A200: 38697D90  addi r3, r9, 0x7d90
	ctx.r[3].s64 = ctx.r[9].s64 + 32144;
	// 8324A204: 4BA5FD1D  bl 0x82ca9f20
	ctx.lr = 0x8324A208;
	sub_82CA9F20(ctx, base);
	// 8324A208: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324A20C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324A210: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324A214: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324A218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324A218 size=64
    let mut pc: u32 = 0x8324A218;
    'dispatch: loop {
        match pc {
            0x8324A218 => {
    //   block [0x8324A218..0x8324A258)
	// 8324A218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324A21C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324A220: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324A224: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324A228: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324A22C: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8324A230: 386A77C4  addi r3, r10, 0x77c4
	ctx.r[3].s64 = ctx.r[10].s64 + 30660;
	// 8324A234: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324A238: 4AFE2C99  bl 0x8222ced0
	ctx.lr = 0x8324A23C;
	sub_8222CED0(ctx, base);
	// 8324A23C: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 8324A240: 38697DA0  addi r3, r9, 0x7da0
	ctx.r[3].s64 = ctx.r[9].s64 + 32160;
	// 8324A244: 4BA5FCDD  bl 0x82ca9f20
	ctx.lr = 0x8324A248;
	sub_82CA9F20(ctx, base);
	// 8324A248: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324A24C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324A250: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324A254: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324A258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324A258 size=64
    let mut pc: u32 = 0x8324A258;
    'dispatch: loop {
        match pc {
            0x8324A258 => {
    //   block [0x8324A258..0x8324A298)
	// 8324A258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324A25C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324A260: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324A264: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324A268: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324A26C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8324A270: 386A77C8  addi r3, r10, 0x77c8
	ctx.r[3].s64 = ctx.r[10].s64 + 30664;
	// 8324A274: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324A278: 4AFE2C59  bl 0x8222ced0
	ctx.lr = 0x8324A27C;
	sub_8222CED0(ctx, base);
	// 8324A27C: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 8324A280: 38697DB0  addi r3, r9, 0x7db0
	ctx.r[3].s64 = ctx.r[9].s64 + 32176;
	// 8324A284: 4BA5FC9D  bl 0x82ca9f20
	ctx.lr = 0x8324A288;
	sub_82CA9F20(ctx, base);
	// 8324A288: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324A28C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324A290: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324A294: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324A298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324A298 size=64
    let mut pc: u32 = 0x8324A298;
    'dispatch: loop {
        match pc {
            0x8324A298 => {
    //   block [0x8324A298..0x8324A2D8)
	// 8324A298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324A29C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324A2A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324A2A4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324A2A8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324A2AC: 388B8FE0  addi r4, r11, -0x7020
	ctx.r[4].s64 = ctx.r[11].s64 + -28704;
	// 8324A2B0: 386A77CC  addi r3, r10, 0x77cc
	ctx.r[3].s64 = ctx.r[10].s64 + 30668;
	// 8324A2B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324A2B8: 4AFE2C19  bl 0x8222ced0
	ctx.lr = 0x8324A2BC;
	sub_8222CED0(ctx, base);
	// 8324A2BC: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 8324A2C0: 38697DC0  addi r3, r9, 0x7dc0
	ctx.r[3].s64 = ctx.r[9].s64 + 32192;
	// 8324A2C4: 4BA5FC5D  bl 0x82ca9f20
	ctx.lr = 0x8324A2C8;
	sub_82CA9F20(ctx, base);
	// 8324A2C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324A2CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324A2D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324A2D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324A2D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324A2D8 size=64
    let mut pc: u32 = 0x8324A2D8;
    'dispatch: loop {
        match pc {
            0x8324A2D8 => {
    //   block [0x8324A2D8..0x8324A318)
	// 8324A2D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324A2DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324A2E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324A2E4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324A2E8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324A2EC: 388B8FFC  addi r4, r11, -0x7004
	ctx.r[4].s64 = ctx.r[11].s64 + -28676;
	// 8324A2F0: 386A77D0  addi r3, r10, 0x77d0
	ctx.r[3].s64 = ctx.r[10].s64 + 30672;
	// 8324A2F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324A2F8: 4AFE2BD9  bl 0x8222ced0
	ctx.lr = 0x8324A2FC;
	sub_8222CED0(ctx, base);
	// 8324A2FC: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 8324A300: 38697DD0  addi r3, r9, 0x7dd0
	ctx.r[3].s64 = ctx.r[9].s64 + 32208;
	// 8324A304: 4BA5FC1D  bl 0x82ca9f20
	ctx.lr = 0x8324A308;
	sub_82CA9F20(ctx, base);
	// 8324A308: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324A30C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324A310: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324A314: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324A318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324A318 size=64
    let mut pc: u32 = 0x8324A318;
    'dispatch: loop {
        match pc {
            0x8324A318 => {
    //   block [0x8324A318..0x8324A358)
	// 8324A318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324A31C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324A320: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324A324: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324A328: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324A32C: 388B8FE0  addi r4, r11, -0x7020
	ctx.r[4].s64 = ctx.r[11].s64 + -28704;
	// 8324A330: 386A77D4  addi r3, r10, 0x77d4
	ctx.r[3].s64 = ctx.r[10].s64 + 30676;
	// 8324A334: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324A338: 4AFE2B99  bl 0x8222ced0
	ctx.lr = 0x8324A33C;
	sub_8222CED0(ctx, base);
	// 8324A33C: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 8324A340: 38697DE0  addi r3, r9, 0x7de0
	ctx.r[3].s64 = ctx.r[9].s64 + 32224;
	// 8324A344: 4BA5FBDD  bl 0x82ca9f20
	ctx.lr = 0x8324A348;
	sub_82CA9F20(ctx, base);
	// 8324A348: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324A34C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324A350: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324A354: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324A358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324A358 size=64
    let mut pc: u32 = 0x8324A358;
    'dispatch: loop {
        match pc {
            0x8324A358 => {
    //   block [0x8324A358..0x8324A398)
	// 8324A358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324A35C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324A360: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324A364: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324A368: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324A36C: 388B9014  addi r4, r11, -0x6fec
	ctx.r[4].s64 = ctx.r[11].s64 + -28652;
	// 8324A370: 386A77D8  addi r3, r10, 0x77d8
	ctx.r[3].s64 = ctx.r[10].s64 + 30680;
	// 8324A374: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324A378: 4AFE2B59  bl 0x8222ced0
	ctx.lr = 0x8324A37C;
	sub_8222CED0(ctx, base);
	// 8324A37C: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 8324A380: 38697DF0  addi r3, r9, 0x7df0
	ctx.r[3].s64 = ctx.r[9].s64 + 32240;
	// 8324A384: 4BA5FB9D  bl 0x82ca9f20
	ctx.lr = 0x8324A388;
	sub_82CA9F20(ctx, base);
	// 8324A388: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324A38C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324A390: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324A394: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324A398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324A398 size=64
    let mut pc: u32 = 0x8324A398;
    'dispatch: loop {
        match pc {
            0x8324A398 => {
    //   block [0x8324A398..0x8324A3D8)
	// 8324A398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324A39C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324A3A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324A3A4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324A3A8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324A3AC: 388B902C  addi r4, r11, -0x6fd4
	ctx.r[4].s64 = ctx.r[11].s64 + -28628;
	// 8324A3B0: 386A77DC  addi r3, r10, 0x77dc
	ctx.r[3].s64 = ctx.r[10].s64 + 30684;
	// 8324A3B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324A3B8: 4AFE2B19  bl 0x8222ced0
	ctx.lr = 0x8324A3BC;
	sub_8222CED0(ctx, base);
	// 8324A3BC: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 8324A3C0: 38697E00  addi r3, r9, 0x7e00
	ctx.r[3].s64 = ctx.r[9].s64 + 32256;
	// 8324A3C4: 4BA5FB5D  bl 0x82ca9f20
	ctx.lr = 0x8324A3C8;
	sub_82CA9F20(ctx, base);
	// 8324A3C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324A3CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324A3D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324A3D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324A3D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324A3D8 size=1368
    let mut pc: u32 = 0x8324A3D8;
    'dispatch: loop {
        match pc {
            0x8324A3D8 => {
    //   block [0x8324A3D8..0x8324A430)
	// 8324A3D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324A3DC: 4BA5F031  bl 0x82ca940c
	ctx.lr = 0x8324A3E0;
	sub_82CA93D0(ctx, base);
	// 8324A3E0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324A3E4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324A3E8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324A3EC: 388B9174  addi r4, r11, -0x6e8c
	ctx.r[4].s64 = ctx.r[11].s64 + -28300;
	// 8324A3F0: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8324A3F4: 4AFE2ADD  bl 0x8222ced0
	ctx.lr = 0x8324A3F8;
	sub_8222CED0(ctx, base);
	// 8324A3F8: 38810078  addi r4, r1, 0x78
	ctx.r[4].s64 = ctx.r[1].s64 + 120;
	// 8324A3FC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8324A400: 4AFA4739  bl 0x821eeb38
	ctx.lr = 0x8324A404;
	sub_821EEB38(ctx, base);
	// 8324A404: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8324A408: 4B9B93E9  bl 0x82c037f0
	ctx.lr = 0x8324A40C;
	sub_82C037F0(ctx, base);
	// 8324A40C: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324A410: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8324A414: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8324A418: 3BCA77E0  addi r30, r10, 0x77e0
	ctx.r[30].s64 = ctx.r[10].s64 + 30688;
	// 8324A41C: 916A77E0  stw r11, 0x77e0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30688 as u32), ctx.r[11].u32 ) };
	// 8324A420: 4AF7C349  bl 0x821c6768
	ctx.lr = 0x8324A424;
	sub_821C6768(ctx, base);
	// 8324A424: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 8324A428: 3BA97088  addi r29, r9, 0x7088
	ctx.r[29].s64 = ctx.r[9].s64 + 28808;
	// 8324A42C: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	pc = 0x8324A430; continue 'dispatch;
            }
            0x8324A430 => {
    //   block [0x8324A430..0x8324A460)
	// 8324A430: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 8324A434: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A438: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 8324A43C: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 8324A440: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324A444: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A448: 4082FFE8  bne 0x8324a430
	if !ctx.cr[0].eq {
	pc = 0x8324A430; continue 'dispatch;
	}
	// 8324A44C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8324A450: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8324A454: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 8324A458: 4AF7C311  bl 0x821c6768
	ctx.lr = 0x8324A45C;
	sub_821C6768(ctx, base);
	// 8324A45C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	pc = 0x8324A460; continue 'dispatch;
            }
            0x8324A460 => {
    //   block [0x8324A460..0x8324A4B8)
	// 8324A460: 7C8000A6  mfmsr r4
	ctx.r[4].u64 = ctx.msr;
	// 8324A464: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A468: 7CA01828  lwarx r5, 0, r3
	// lwarx
	let ea = ctx.r[3].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[5].u64 = ctx.reserved.u32 as u64;
	// 8324A46C: 38A5FFFF  addi r5, r5, -1
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	// 8324A470: 7CA0192D  stwcx. r5, 0, r3
	// stwcx.
	let addr = ctx.r[3].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[5].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324A474: 7C810164  mtmsrd r4, 1
	ctx.msr = (ctx.r[4].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A478: 4082FFE8  bne 0x8324a460
	if !ctx.cr[0].eq {
	pc = 0x8324A460; continue 'dispatch;
	}
	// 8324A47C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324A480: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324A484: 388B9148  addi r4, r11, -0x6eb8
	ctx.r[4].s64 = ctx.r[11].s64 + -28344;
	// 8324A488: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8324A48C: 4AFE2A45  bl 0x8222ced0
	ctx.lr = 0x8324A490;
	sub_8222CED0(ctx, base);
	// 8324A490: 38810078  addi r4, r1, 0x78
	ctx.r[4].s64 = ctx.r[1].s64 + 120;
	// 8324A494: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8324A498: 4AFA46A1  bl 0x821eeb38
	ctx.lr = 0x8324A49C;
	sub_821EEB38(ctx, base);
	// 8324A49C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8324A4A0: 4B9B9351  bl 0x82c037f0
	ctx.lr = 0x8324A4A4;
	sub_82C037F0(ctx, base);
	// 8324A4A4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8324A4A8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8324A4AC: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8324A4B0: 4AF7C2B9  bl 0x821c6768
	ctx.lr = 0x8324A4B4;
	sub_821C6768(ctx, base);
	// 8324A4B4: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	pc = 0x8324A4B8; continue 'dispatch;
            }
            0x8324A4B8 => {
    //   block [0x8324A4B8..0x8324A4E4)
	// 8324A4B8: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8324A4BC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A4C0: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8324A4C4: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8324A4C8: 7D40412D  stwcx. r10, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324A4CC: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A4D0: 4082FFE8  bne 0x8324a4b8
	if !ctx.cr[0].eq {
	pc = 0x8324A4B8; continue 'dispatch;
	}
	// 8324A4D4: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 8324A4D8: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8324A4DC: 4AF7C28D  bl 0x821c6768
	ctx.lr = 0x8324A4E0;
	sub_821C6768(ctx, base);
	// 8324A4E0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	pc = 0x8324A4E4; continue 'dispatch;
            }
            0x8324A4E4 => {
    //   block [0x8324A4E4..0x8324A53C)
	// 8324A4E4: 7CC000A6  mfmsr r6
	ctx.r[6].u64 = ctx.msr;
	// 8324A4E8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A4EC: 7CE02828  lwarx r7, 0, r5
	// lwarx
	let ea = ctx.r[5].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[7].u64 = ctx.reserved.u32 as u64;
	// 8324A4F0: 38E7FFFF  addi r7, r7, -1
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	// 8324A4F4: 7CE0292D  stwcx. r7, 0, r5
	// stwcx.
	let addr = ctx.r[5].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[7].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324A4F8: 7CC10164  mtmsrd r6, 1
	ctx.msr = (ctx.r[6].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A4FC: 4082FFE8  bne 0x8324a4e4
	if !ctx.cr[0].eq {
	pc = 0x8324A4E4; continue 'dispatch;
	}
	// 8324A500: 3C80820B  lis r4, -0x7df5
	ctx.r[4].s64 = -2113208320;
	// 8324A504: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324A508: 3884911C  addi r4, r4, -0x6ee4
	ctx.r[4].s64 = ctx.r[4].s64 + -28388;
	// 8324A50C: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8324A510: 4AFE29C1  bl 0x8222ced0
	ctx.lr = 0x8324A514;
	sub_8222CED0(ctx, base);
	// 8324A514: 38810078  addi r4, r1, 0x78
	ctx.r[4].s64 = ctx.r[1].s64 + 120;
	// 8324A518: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8324A51C: 4AFA461D  bl 0x821eeb38
	ctx.lr = 0x8324A520;
	sub_821EEB38(ctx, base);
	// 8324A520: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8324A524: 4B9B92CD  bl 0x82c037f0
	ctx.lr = 0x8324A528;
	sub_82C037F0(ctx, base);
	// 8324A528: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8324A52C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8324A530: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8324A534: 4AF7C235  bl 0x821c6768
	ctx.lr = 0x8324A538;
	sub_821C6768(ctx, base);
	// 8324A538: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	pc = 0x8324A53C; continue 'dispatch;
            }
            0x8324A53C => {
    //   block [0x8324A53C..0x8324A568)
	// 8324A53C: 7D6000A6  mfmsr r11
	ctx.r[11].u64 = ctx.msr;
	// 8324A540: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A544: 7C605028  lwarx r3, 0, r10
	// lwarx
	let ea = ctx.r[10].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[3].u64 = ctx.reserved.u32 as u64;
	// 8324A548: 3863FFFF  addi r3, r3, -1
	ctx.r[3].s64 = ctx.r[3].s64 + -1;
	// 8324A54C: 7C60512D  stwcx. r3, 0, r10
	// stwcx.
	let addr = ctx.r[10].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[3].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324A550: 7D610164  mtmsrd r11, 1
	ctx.msr = (ctx.r[11].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A554: 4082FFE8  bne 0x8324a53c
	if !ctx.cr[0].eq {
	pc = 0x8324A53C; continue 'dispatch;
	}
	// 8324A558: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 8324A55C: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8324A560: 4AF7C209  bl 0x821c6768
	ctx.lr = 0x8324A564;
	sub_821C6768(ctx, base);
	// 8324A564: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	pc = 0x8324A568; continue 'dispatch;
            }
            0x8324A568 => {
    //   block [0x8324A568..0x8324A5C0)
	// 8324A568: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 8324A56C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A570: 7D203828  lwarx r9, 0, r7
	// lwarx
	let ea = ctx.r[7].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 8324A574: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 8324A578: 7D20392D  stwcx. r9, 0, r7
	// stwcx.
	let addr = ctx.r[7].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324A57C: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A580: 4082FFE8  bne 0x8324a568
	if !ctx.cr[0].eq {
	pc = 0x8324A568; continue 'dispatch;
	}
	// 8324A584: 3CC0820B  lis r6, -0x7df5
	ctx.r[6].s64 = -2113208320;
	// 8324A588: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324A58C: 388690F0  addi r4, r6, -0x6f10
	ctx.r[4].s64 = ctx.r[6].s64 + -28432;
	// 8324A590: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8324A594: 4AFE293D  bl 0x8222ced0
	ctx.lr = 0x8324A598;
	sub_8222CED0(ctx, base);
	// 8324A598: 38810078  addi r4, r1, 0x78
	ctx.r[4].s64 = ctx.r[1].s64 + 120;
	// 8324A59C: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 8324A5A0: 4AFA4599  bl 0x821eeb38
	ctx.lr = 0x8324A5A4;
	sub_821EEB38(ctx, base);
	// 8324A5A4: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 8324A5A8: 4B9B9249  bl 0x82c037f0
	ctx.lr = 0x8324A5AC;
	sub_82C037F0(ctx, base);
	// 8324A5AC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8324A5B0: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 8324A5B4: 917E000C  stw r11, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8324A5B8: 4AF7C1B1  bl 0x821c6768
	ctx.lr = 0x8324A5BC;
	sub_821C6768(ctx, base);
	// 8324A5BC: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	pc = 0x8324A5C0; continue 'dispatch;
            }
            0x8324A5C0 => {
    //   block [0x8324A5C0..0x8324A5EC)
	// 8324A5C0: 7C8000A6  mfmsr r4
	ctx.r[4].u64 = ctx.msr;
	// 8324A5C4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A5C8: 7CA05828  lwarx r5, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[5].u64 = ctx.reserved.u32 as u64;
	// 8324A5CC: 38A5FFFF  addi r5, r5, -1
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	// 8324A5D0: 7CA0592D  stwcx. r5, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[5].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324A5D4: 7C810164  mtmsrd r4, 1
	ctx.msr = (ctx.r[4].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A5D8: 4082FFE8  bne 0x8324a5c0
	if !ctx.cr[0].eq {
	pc = 0x8324A5C0; continue 'dispatch;
	}
	// 8324A5DC: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8324A5E0: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8324A5E4: 4AF7C185  bl 0x821c6768
	ctx.lr = 0x8324A5E8;
	sub_821C6768(ctx, base);
	// 8324A5E8: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	pc = 0x8324A5EC; continue 'dispatch;
            }
            0x8324A5EC => {
    //   block [0x8324A5EC..0x8324A644)
	// 8324A5EC: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8324A5F0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A5F4: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8324A5F8: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8324A5FC: 7D40412D  stwcx. r10, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324A600: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A604: 4082FFE8  bne 0x8324a5ec
	if !ctx.cr[0].eq {
	pc = 0x8324A5EC; continue 'dispatch;
	}
	// 8324A608: 3CE0820B  lis r7, -0x7df5
	ctx.r[7].s64 = -2113208320;
	// 8324A60C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324A610: 388790C0  addi r4, r7, -0x6f40
	ctx.r[4].s64 = ctx.r[7].s64 + -28480;
	// 8324A614: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8324A618: 4AFE28B9  bl 0x8222ced0
	ctx.lr = 0x8324A61C;
	sub_8222CED0(ctx, base);
	// 8324A61C: 38810078  addi r4, r1, 0x78
	ctx.r[4].s64 = ctx.r[1].s64 + 120;
	// 8324A620: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8324A624: 4AFA4515  bl 0x821eeb38
	ctx.lr = 0x8324A628;
	sub_821EEB38(ctx, base);
	// 8324A628: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8324A62C: 4B9B91C5  bl 0x82c037f0
	ctx.lr = 0x8324A630;
	sub_82C037F0(ctx, base);
	// 8324A630: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8324A634: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8324A638: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8324A63C: 4AF7C12D  bl 0x821c6768
	ctx.lr = 0x8324A640;
	sub_821C6768(ctx, base);
	// 8324A640: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	pc = 0x8324A644; continue 'dispatch;
            }
            0x8324A644 => {
    //   block [0x8324A644..0x8324A670)
	// 8324A644: 7CA000A6  mfmsr r5
	ctx.r[5].u64 = ctx.msr;
	// 8324A648: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A64C: 7CC02028  lwarx r6, 0, r4
	// lwarx
	let ea = ctx.r[4].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[6].u64 = ctx.reserved.u32 as u64;
	// 8324A650: 38C6FFFF  addi r6, r6, -1
	ctx.r[6].s64 = ctx.r[6].s64 + -1;
	// 8324A654: 7CC0212D  stwcx. r6, 0, r4
	// stwcx.
	let addr = ctx.r[4].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[6].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324A658: 7CA10164  mtmsrd r5, 1
	ctx.msr = (ctx.r[5].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A65C: 4082FFE8  bne 0x8324a644
	if !ctx.cr[0].eq {
	pc = 0x8324A644; continue 'dispatch;
	}
	// 8324A660: 93E10060  stw r31, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[31].u32 ) };
	// 8324A664: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8324A668: 4AF7C101  bl 0x821c6768
	ctx.lr = 0x8324A66C;
	sub_821C6768(ctx, base);
	// 8324A66C: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	pc = 0x8324A670; continue 'dispatch;
            }
            0x8324A670 => {
    //   block [0x8324A670..0x8324A6C8)
	// 8324A670: 7D6000A6  mfmsr r11
	ctx.r[11].u64 = ctx.msr;
	// 8324A674: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A678: 7C605028  lwarx r3, 0, r10
	// lwarx
	let ea = ctx.r[10].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[3].u64 = ctx.reserved.u32 as u64;
	// 8324A67C: 3863FFFF  addi r3, r3, -1
	ctx.r[3].s64 = ctx.r[3].s64 + -1;
	// 8324A680: 7C60512D  stwcx. r3, 0, r10
	// stwcx.
	let addr = ctx.r[10].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[3].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324A684: 7D610164  mtmsrd r11, 1
	ctx.msr = (ctx.r[11].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A688: 4082FFE8  bne 0x8324a670
	if !ctx.cr[0].eq {
	pc = 0x8324A670; continue 'dispatch;
	}
	// 8324A68C: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 8324A690: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324A694: 38899094  addi r4, r9, -0x6f6c
	ctx.r[4].s64 = ctx.r[9].s64 + -28524;
	// 8324A698: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8324A69C: 4AFE2835  bl 0x8222ced0
	ctx.lr = 0x8324A6A0;
	sub_8222CED0(ctx, base);
	// 8324A6A0: 38810078  addi r4, r1, 0x78
	ctx.r[4].s64 = ctx.r[1].s64 + 120;
	// 8324A6A4: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 8324A6A8: 4AFA4491  bl 0x821eeb38
	ctx.lr = 0x8324A6AC;
	sub_821EEB38(ctx, base);
	// 8324A6AC: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 8324A6B0: 4B9B9141  bl 0x82c037f0
	ctx.lr = 0x8324A6B4;
	sub_82C037F0(ctx, base);
	// 8324A6B4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8324A6B8: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 8324A6BC: 917E0014  stw r11, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8324A6C0: 4AF7C0A9  bl 0x821c6768
	ctx.lr = 0x8324A6C4;
	sub_821C6768(ctx, base);
	// 8324A6C4: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	pc = 0x8324A6C8; continue 'dispatch;
            }
            0x8324A6C8 => {
    //   block [0x8324A6C8..0x8324A6F4)
	// 8324A6C8: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 8324A6CC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A6D0: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 8324A6D4: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 8324A6D8: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324A6DC: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A6E0: 4082FFE8  bne 0x8324a6c8
	if !ctx.cr[0].eq {
	pc = 0x8324A6C8; continue 'dispatch;
	}
	// 8324A6E4: 93E10064  stw r31, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[31].u32 ) };
	// 8324A6E8: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8324A6EC: 4AF7C07D  bl 0x821c6768
	ctx.lr = 0x8324A6F0;
	sub_821C6768(ctx, base);
	// 8324A6F0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	pc = 0x8324A6F4; continue 'dispatch;
            }
            0x8324A6F4 => {
    //   block [0x8324A6F4..0x8324A74C)
	// 8324A6F4: 7C8000A6  mfmsr r4
	ctx.r[4].u64 = ctx.msr;
	// 8324A6F8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A6FC: 7CA01828  lwarx r5, 0, r3
	// lwarx
	let ea = ctx.r[3].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[5].u64 = ctx.reserved.u32 as u64;
	// 8324A700: 38A5FFFF  addi r5, r5, -1
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	// 8324A704: 7CA0192D  stwcx. r5, 0, r3
	// stwcx.
	let addr = ctx.r[3].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[5].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324A708: 7C810164  mtmsrd r4, 1
	ctx.msr = (ctx.r[4].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A70C: 4082FFE8  bne 0x8324a6f4
	if !ctx.cr[0].eq {
	pc = 0x8324A6F4; continue 'dispatch;
	}
	// 8324A710: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324A714: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324A718: 388B9068  addi r4, r11, -0x6f98
	ctx.r[4].s64 = ctx.r[11].s64 + -28568;
	// 8324A71C: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8324A720: 4AFE27B1  bl 0x8222ced0
	ctx.lr = 0x8324A724;
	sub_8222CED0(ctx, base);
	// 8324A724: 38810078  addi r4, r1, 0x78
	ctx.r[4].s64 = ctx.r[1].s64 + 120;
	// 8324A728: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8324A72C: 4AFA440D  bl 0x821eeb38
	ctx.lr = 0x8324A730;
	sub_821EEB38(ctx, base);
	// 8324A730: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8324A734: 4B9B90BD  bl 0x82c037f0
	ctx.lr = 0x8324A738;
	sub_82C037F0(ctx, base);
	// 8324A738: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8324A73C: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8324A740: 917E0018  stw r11, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8324A744: 4AF7C025  bl 0x821c6768
	ctx.lr = 0x8324A748;
	sub_821C6768(ctx, base);
	// 8324A748: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	pc = 0x8324A74C; continue 'dispatch;
            }
            0x8324A74C => {
    //   block [0x8324A74C..0x8324A778)
	// 8324A74C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8324A750: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A754: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8324A758: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8324A75C: 7D40412D  stwcx. r10, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324A760: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A764: 4082FFE8  bne 0x8324a74c
	if !ctx.cr[0].eq {
	pc = 0x8324A74C; continue 'dispatch;
	}
	// 8324A768: 93E10068  stw r31, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[31].u32 ) };
	// 8324A76C: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8324A770: 4AF7BFF9  bl 0x821c6768
	ctx.lr = 0x8324A774;
	sub_821C6768(ctx, base);
	// 8324A774: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	pc = 0x8324A778; continue 'dispatch;
            }
            0x8324A778 => {
    //   block [0x8324A778..0x8324A7D0)
	// 8324A778: 7CC000A6  mfmsr r6
	ctx.r[6].u64 = ctx.msr;
	// 8324A77C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A780: 7CE02828  lwarx r7, 0, r5
	// lwarx
	let ea = ctx.r[5].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[7].u64 = ctx.reserved.u32 as u64;
	// 8324A784: 38E7FFFF  addi r7, r7, -1
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	// 8324A788: 7CE0292D  stwcx. r7, 0, r5
	// stwcx.
	let addr = ctx.r[5].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[7].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324A78C: 7CC10164  mtmsrd r6, 1
	ctx.msr = (ctx.r[6].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A790: 4082FFE8  bne 0x8324a778
	if !ctx.cr[0].eq {
	pc = 0x8324A778; continue 'dispatch;
	}
	// 8324A794: 3C80820B  lis r4, -0x7df5
	ctx.r[4].s64 = -2113208320;
	// 8324A798: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324A79C: 38849038  addi r4, r4, -0x6fc8
	ctx.r[4].s64 = ctx.r[4].s64 + -28616;
	// 8324A7A0: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8324A7A4: 4AFE272D  bl 0x8222ced0
	ctx.lr = 0x8324A7A8;
	sub_8222CED0(ctx, base);
	// 8324A7A8: 38810078  addi r4, r1, 0x78
	ctx.r[4].s64 = ctx.r[1].s64 + 120;
	// 8324A7AC: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8324A7B0: 4AFA4389  bl 0x821eeb38
	ctx.lr = 0x8324A7B4;
	sub_821EEB38(ctx, base);
	// 8324A7B4: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8324A7B8: 4B9B9039  bl 0x82c037f0
	ctx.lr = 0x8324A7BC;
	sub_82C037F0(ctx, base);
	// 8324A7BC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8324A7C0: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8324A7C4: 917E001C  stw r11, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 8324A7C8: 4AF7BFA1  bl 0x821c6768
	ctx.lr = 0x8324A7CC;
	sub_821C6768(ctx, base);
	// 8324A7CC: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	pc = 0x8324A7D0; continue 'dispatch;
            }
            0x8324A7D0 => {
    //   block [0x8324A7D0..0x8324A7FC)
	// 8324A7D0: 7D6000A6  mfmsr r11
	ctx.r[11].u64 = ctx.msr;
	// 8324A7D4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A7D8: 7C605028  lwarx r3, 0, r10
	// lwarx
	let ea = ctx.r[10].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[3].u64 = ctx.reserved.u32 as u64;
	// 8324A7DC: 3863FFFF  addi r3, r3, -1
	ctx.r[3].s64 = ctx.r[3].s64 + -1;
	// 8324A7E0: 7C60512D  stwcx. r3, 0, r10
	// stwcx.
	let addr = ctx.r[10].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[3].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324A7E4: 7D610164  mtmsrd r11, 1
	ctx.msr = (ctx.r[11].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A7E8: 4082FFE8  bne 0x8324a7d0
	if !ctx.cr[0].eq {
	pc = 0x8324A7D0; continue 'dispatch;
	}
	// 8324A7EC: 93E1006C  stw r31, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[31].u32 ) };
	// 8324A7F0: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8324A7F4: 4AF7BF75  bl 0x821c6768
	ctx.lr = 0x8324A7F8;
	sub_821C6768(ctx, base);
	// 8324A7F8: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	pc = 0x8324A7FC; continue 'dispatch;
            }
            0x8324A7FC => {
    //   block [0x8324A7FC..0x8324A854)
	// 8324A7FC: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 8324A800: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A804: 7D203828  lwarx r9, 0, r7
	// lwarx
	let ea = ctx.r[7].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 8324A808: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 8324A80C: 7D20392D  stwcx. r9, 0, r7
	// stwcx.
	let addr = ctx.r[7].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324A810: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A814: 4082FFE8  bne 0x8324a7fc
	if !ctx.cr[0].eq {
	pc = 0x8324A7FC; continue 'dispatch;
	}
	// 8324A818: 3CC0820B  lis r6, -0x7df5
	ctx.r[6].s64 = -2113208320;
	// 8324A81C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324A820: 3886819C  addi r4, r6, -0x7e64
	ctx.r[4].s64 = ctx.r[6].s64 + -32356;
	// 8324A824: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8324A828: 4AFE26A9  bl 0x8222ced0
	ctx.lr = 0x8324A82C;
	sub_8222CED0(ctx, base);
	// 8324A82C: 38810078  addi r4, r1, 0x78
	ctx.r[4].s64 = ctx.r[1].s64 + 120;
	// 8324A830: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8324A834: 4AFA4305  bl 0x821eeb38
	ctx.lr = 0x8324A838;
	sub_821EEB38(ctx, base);
	// 8324A838: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8324A83C: 4B9B8FB5  bl 0x82c037f0
	ctx.lr = 0x8324A840;
	sub_82C037F0(ctx, base);
	// 8324A840: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8324A844: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8324A848: 917E0020  stw r11, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8324A84C: 4AF7BF1D  bl 0x821c6768
	ctx.lr = 0x8324A850;
	sub_821C6768(ctx, base);
	// 8324A850: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	pc = 0x8324A854; continue 'dispatch;
            }
            0x8324A854 => {
    //   block [0x8324A854..0x8324A880)
	// 8324A854: 7C8000A6  mfmsr r4
	ctx.r[4].u64 = ctx.msr;
	// 8324A858: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A85C: 7CA05828  lwarx r5, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[5].u64 = ctx.reserved.u32 as u64;
	// 8324A860: 38A5FFFF  addi r5, r5, -1
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	// 8324A864: 7CA0592D  stwcx. r5, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[5].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324A868: 7C810164  mtmsrd r4, 1
	ctx.msr = (ctx.r[4].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A86C: 4082FFE8  bne 0x8324a854
	if !ctx.cr[0].eq {
	pc = 0x8324A854; continue 'dispatch;
	}
	// 8324A870: 93E10070  stw r31, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[31].u32 ) };
	// 8324A874: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8324A878: 4AF7BEF1  bl 0x821c6768
	ctx.lr = 0x8324A87C;
	sub_821C6768(ctx, base);
	// 8324A87C: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	pc = 0x8324A880; continue 'dispatch;
            }
            0x8324A880 => {
    //   block [0x8324A880..0x8324A8D8)
	// 8324A880: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8324A884: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A888: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8324A88C: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8324A890: 7D40412D  stwcx. r10, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324A894: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A898: 4082FFE8  bne 0x8324a880
	if !ctx.cr[0].eq {
	pc = 0x8324A880; continue 'dispatch;
	}
	// 8324A89C: 3CE0820B  lis r7, -0x7df5
	ctx.r[7].s64 = -2113208320;
	// 8324A8A0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324A8A4: 3887824C  addi r4, r7, -0x7db4
	ctx.r[4].s64 = ctx.r[7].s64 + -32180;
	// 8324A8A8: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8324A8AC: 4AFE2625  bl 0x8222ced0
	ctx.lr = 0x8324A8B0;
	sub_8222CED0(ctx, base);
	// 8324A8B0: 38810078  addi r4, r1, 0x78
	ctx.r[4].s64 = ctx.r[1].s64 + 120;
	// 8324A8B4: 38610074  addi r3, r1, 0x74
	ctx.r[3].s64 = ctx.r[1].s64 + 116;
	// 8324A8B8: 4AFA4281  bl 0x821eeb38
	ctx.lr = 0x8324A8BC;
	sub_821EEB38(ctx, base);
	// 8324A8BC: 38610074  addi r3, r1, 0x74
	ctx.r[3].s64 = ctx.r[1].s64 + 116;
	// 8324A8C0: 4B9B8F31  bl 0x82c037f0
	ctx.lr = 0x8324A8C4;
	sub_82C037F0(ctx, base);
	// 8324A8C4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8324A8C8: 38610074  addi r3, r1, 0x74
	ctx.r[3].s64 = ctx.r[1].s64 + 116;
	// 8324A8CC: 917E0024  stw r11, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 8324A8D0: 4AF7BE99  bl 0x821c6768
	ctx.lr = 0x8324A8D4;
	sub_821C6768(ctx, base);
	// 8324A8D4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	pc = 0x8324A8D8; continue 'dispatch;
            }
            0x8324A8D8 => {
    //   block [0x8324A8D8..0x8324A900)
	// 8324A8D8: 7CA000A6  mfmsr r5
	ctx.r[5].u64 = ctx.msr;
	// 8324A8DC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A8E0: 7CC02028  lwarx r6, 0, r4
	// lwarx
	let ea = ctx.r[4].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[6].u64 = ctx.reserved.u32 as u64;
	// 8324A8E4: 38C6FFFF  addi r6, r6, -1
	ctx.r[6].s64 = ctx.r[6].s64 + -1;
	// 8324A8E8: 7CC0212D  stwcx. r6, 0, r4
	// stwcx.
	let addr = ctx.r[4].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[6].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324A8EC: 7CA10164  mtmsrd r5, 1
	ctx.msr = (ctx.r[5].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A8F0: 4082FFE8  bne 0x8324a8d8
	if !ctx.cr[0].eq {
	pc = 0x8324A8D8; continue 'dispatch;
	}
	// 8324A8F4: 93E10074  stw r31, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[31].u32 ) };
	// 8324A8F8: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8324A8FC: 4AF7BE6D  bl 0x821c6768
	ctx.lr = 0x8324A900;
	sub_821C6768(ctx, base);
	pc = 0x8324A900; continue 'dispatch;
            }
            0x8324A900 => {
    //   block [0x8324A900..0x8324A930)
	// 8324A900: 7D6000A6  mfmsr r11
	ctx.r[11].u64 = ctx.msr;
	// 8324A904: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A908: 7C60E828  lwarx r3, 0, r29
	// lwarx
	let ea = ctx.r[29].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[3].u64 = ctx.reserved.u32 as u64;
	// 8324A90C: 3863FFFF  addi r3, r3, -1
	ctx.r[3].s64 = ctx.r[3].s64 + -1;
	// 8324A910: 7C60E92D  stwcx. r3, 0, r29
	// stwcx.
	let addr = ctx.r[29].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[3].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324A914: 7D610164  mtmsrd r11, 1
	ctx.msr = (ctx.r[11].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A918: 4082FFE8  bne 0x8324a900
	if !ctx.cr[0].eq {
	pc = 0x8324A900; continue 'dispatch;
	}
	// 8324A91C: 3D40832A  lis r10, -0x7cd6
	ctx.r[10].s64 = -2094399488;
	// 8324A920: 386A7E10  addi r3, r10, 0x7e10
	ctx.r[3].s64 = ctx.r[10].s64 + 32272;
	// 8324A924: 4BA5F5FD  bl 0x82ca9f20
	ctx.lr = 0x8324A928;
	sub_82CA9F20(ctx, base);
	// 8324A928: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8324A92C: 4BA5EB30  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324A930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324A930 size=192
    let mut pc: u32 = 0x8324A930;
    'dispatch: loop {
        match pc {
            0x8324A930 => {
    //   block [0x8324A930..0x8324A988)
	// 8324A930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324A934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324A938: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8324A93C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324A940: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324A944: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324A948: 388B919C  addi r4, r11, -0x6e64
	ctx.r[4].s64 = ctx.r[11].s64 + -28260;
	// 8324A94C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8324A950: 4AFE2581  bl 0x8222ced0
	ctx.lr = 0x8324A954;
	sub_8222CED0(ctx, base);
	// 8324A954: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8324A958: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8324A95C: 4AFA41DD  bl 0x821eeb38
	ctx.lr = 0x8324A960;
	sub_821EEB38(ctx, base);
	// 8324A960: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8324A964: 4B9B8E8D  bl 0x82c037f0
	ctx.lr = 0x8324A968;
	sub_82C037F0(ctx, base);
	// 8324A968: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324A96C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8324A970: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8324A974: 916A7808  stw r11, 0x7808(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30728 as u32), ctx.r[11].u32 ) };
	// 8324A978: 4AF7BDF1  bl 0x821c6768
	ctx.lr = 0x8324A97C;
	sub_821C6768(ctx, base);
	// 8324A97C: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 8324A980: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 8324A984: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	pc = 0x8324A988; continue 'dispatch;
            }
            0x8324A988 => {
    //   block [0x8324A988..0x8324A9B4)
	// 8324A988: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 8324A98C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A990: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 8324A994: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 8324A998: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324A99C: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A9A0: 4082FFE8  bne 0x8324a988
	if !ctx.cr[0].eq {
	pc = 0x8324A988; continue 'dispatch;
	}
	// 8324A9A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8324A9A8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8324A9AC: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 8324A9B0: 4AF7BDB9  bl 0x821c6768
	ctx.lr = 0x8324A9B4;
	sub_821C6768(ctx, base);
	pc = 0x8324A9B4; continue 'dispatch;
            }
            0x8324A9B4 => {
    //   block [0x8324A9B4..0x8324A9F0)
	// 8324A9B4: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 8324A9B8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A9BC: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 8324A9C0: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 8324A9C4: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324A9C8: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A9CC: 4082FFE8  bne 0x8324a9b4
	if !ctx.cr[0].eq {
	pc = 0x8324A9B4; continue 'dispatch;
	}
	// 8324A9D0: 3D60832A  lis r11, -0x7cd6
	ctx.r[11].s64 = -2094399488;
	// 8324A9D4: 386B7E18  addi r3, r11, 0x7e18
	ctx.r[3].s64 = ctx.r[11].s64 + 32280;
	// 8324A9D8: 4BA5F549  bl 0x82ca9f20
	ctx.lr = 0x8324A9DC;
	sub_82CA9F20(ctx, base);
	// 8324A9DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8324A9E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324A9E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324A9E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8324A9EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324A9F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324A9F0 size=192
    let mut pc: u32 = 0x8324A9F0;
    'dispatch: loop {
        match pc {
            0x8324A9F0 => {
    //   block [0x8324A9F0..0x8324AA48)
	// 8324A9F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324A9F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324A9F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8324A9FC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324AA00: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324AA04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324AA08: 388B91C4  addi r4, r11, -0x6e3c
	ctx.r[4].s64 = ctx.r[11].s64 + -28220;
	// 8324AA0C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8324AA10: 4AFE24C1  bl 0x8222ced0
	ctx.lr = 0x8324AA14;
	sub_8222CED0(ctx, base);
	// 8324AA14: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8324AA18: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8324AA1C: 4AFA411D  bl 0x821eeb38
	ctx.lr = 0x8324AA20;
	sub_821EEB38(ctx, base);
	// 8324AA20: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8324AA24: 4B9B8DCD  bl 0x82c037f0
	ctx.lr = 0x8324AA28;
	sub_82C037F0(ctx, base);
	// 8324AA28: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324AA2C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8324AA30: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8324AA34: 916A780C  stw r11, 0x780c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30732 as u32), ctx.r[11].u32 ) };
	// 8324AA38: 4AF7BD31  bl 0x821c6768
	ctx.lr = 0x8324AA3C;
	sub_821C6768(ctx, base);
	// 8324AA3C: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 8324AA40: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 8324AA44: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	pc = 0x8324AA48; continue 'dispatch;
            }
            0x8324AA48 => {
    //   block [0x8324AA48..0x8324AA74)
	// 8324AA48: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 8324AA4C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324AA50: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 8324AA54: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 8324AA58: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324AA5C: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324AA60: 4082FFE8  bne 0x8324aa48
	if !ctx.cr[0].eq {
	pc = 0x8324AA48; continue 'dispatch;
	}
	// 8324AA64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8324AA68: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8324AA6C: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 8324AA70: 4AF7BCF9  bl 0x821c6768
	ctx.lr = 0x8324AA74;
	sub_821C6768(ctx, base);
	pc = 0x8324AA74; continue 'dispatch;
            }
            0x8324AA74 => {
    //   block [0x8324AA74..0x8324AAB0)
	// 8324AA74: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 8324AA78: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324AA7C: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 8324AA80: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 8324AA84: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324AA88: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324AA8C: 4082FFE8  bne 0x8324aa74
	if !ctx.cr[0].eq {
	pc = 0x8324AA74; continue 'dispatch;
	}
	// 8324AA90: 3D60832A  lis r11, -0x7cd6
	ctx.r[11].s64 = -2094399488;
	// 8324AA94: 386B7E20  addi r3, r11, 0x7e20
	ctx.r[3].s64 = ctx.r[11].s64 + 32288;
	// 8324AA98: 4BA5F489  bl 0x82ca9f20
	ctx.lr = 0x8324AA9C;
	sub_82CA9F20(ctx, base);
	// 8324AA9C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8324AAA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324AAA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324AAA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8324AAAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324AAB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324AAB0 size=52
    let mut pc: u32 = 0x8324AAB0;
    'dispatch: loop {
        match pc {
            0x8324AAB0 => {
    //   block [0x8324AAB0..0x8324AAE4)
	// 8324AAB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324AAB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324AAB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324AABC: 3D60834F  lis r11, -0x7cb1
	ctx.r[11].s64 = -2091974656;
	// 8324AAC0: 386B76C0  addi r3, r11, 0x76c0
	ctx.r[3].s64 = ctx.r[11].s64 + 30400;
	// 8324AAC4: 4806F1C1  bl 0x832b9c84
	ctx.lr = 0x8324AAC8;
	// extern call 0x832B9C84 → crate::xboxkrnl::RtlInitializeCriticalSection
	crate::xboxkrnl::RtlInitializeCriticalSection(ctx, base);
	// 8324AAC8: 3D40832A  lis r10, -0x7cd6
	ctx.r[10].s64 = -2094399488;
	// 8324AACC: 386A7E40  addi r3, r10, 0x7e40
	ctx.r[3].s64 = ctx.r[10].s64 + 32320;
	// 8324AAD0: 4BA5F451  bl 0x82ca9f20
	ctx.lr = 0x8324AAD4;
	sub_82CA9F20(ctx, base);
	// 8324AAD4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324AAD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324AADC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324AAE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324AAE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8324AAE8 size=12
    let mut pc: u32 = 0x8324AAE8;
    'dispatch: loop {
        match pc {
            0x8324AAE8 => {
    //   block [0x8324AAE8..0x8324AAF4)
	// 8324AAE8: 3D60832A  lis r11, -0x7cd6
	ctx.r[11].s64 = -2094399488;
	// 8324AAEC: 386B7E30  addi r3, r11, 0x7e30
	ctx.r[3].s64 = ctx.r[11].s64 + 32304;
	// 8324AAF0: 4BA5F430  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324AAF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324AAF8 size=640
    let mut pc: u32 = 0x8324AAF8;
    'dispatch: loop {
        match pc {
            0x8324AAF8 => {
    //   block [0x8324AAF8..0x8324AB58)
	// 8324AAF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324AAFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324AB00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8324AB04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8324AB08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324AB0C: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 8324AB10: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 8324AB14: 3BEB7810  addi r31, r11, 0x7810
	ctx.r[31].s64 = ctx.r[11].s64 + 30736;
	// 8324AB18: 388A9414  addi r4, r10, -0x6bec
	ctx.r[4].s64 = ctx.r[10].s64 + -27628;
	// 8324AB1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8324AB20: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324AB24: 4AFE23AD  bl 0x8222ced0
	ctx.lr = 0x8324AB28;
	sub_8222CED0(ctx, base);
	// 8324AB28: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 8324AB2C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324AB30: 38899404  addi r4, r9, -0x6bfc
	ctx.r[4].s64 = ctx.r[9].s64 + -27644;
	// 8324AB34: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8324AB38: 4AFE2399  bl 0x8222ced0
	ctx.lr = 0x8324AB3C;
	sub_8222CED0(ctx, base);
	// 8324AB3C: 3D008349  lis r8, -0x7cb7
	ctx.r[8].s64 = -2092367872;
	// 8324AB40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8324AB44: 3BC87088  addi r30, r8, 0x7088
	ctx.r[30].s64 = ctx.r[8].s64 + 28808;
	// 8324AB48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8324AB4C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8324AB50: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8324AB54: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	pc = 0x8324AB58; continue 'dispatch;
            }
            0x8324AB58 => {
    //   block [0x8324AB58..0x8324ABB8)
	// 8324AB58: 7CC000A6  mfmsr r6
	ctx.r[6].u64 = ctx.msr;
	// 8324AB5C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324AB60: 7CE02828  lwarx r7, 0, r5
	// lwarx
	let ea = ctx.r[5].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[7].u64 = ctx.reserved.u32 as u64;
	// 8324AB64: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 8324AB68: 7CE0292D  stwcx. r7, 0, r5
	// stwcx.
	let addr = ctx.r[5].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[7].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324AB6C: 7CC10164  mtmsrd r6, 1
	ctx.msr = (ctx.r[6].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324AB70: 4082FFE8  bne 0x8324ab58
	if !ctx.cr[0].eq {
	pc = 0x8324AB58; continue 'dispatch;
	}
	// 8324AB74: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8324AB78: 3C80820B  lis r4, -0x7df5
	ctx.r[4].s64 = -2113208320;
	// 8324AB7C: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8324AB80: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324AB84: 388493FC  addi r4, r4, -0x6c04
	ctx.r[4].s64 = ctx.r[4].s64 + -27652;
	// 8324AB88: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 8324AB8C: 4AFE2345  bl 0x8222ced0
	ctx.lr = 0x8324AB90;
	sub_8222CED0(ctx, base);
	// 8324AB90: 3C60820B  lis r3, -0x7df5
	ctx.r[3].s64 = -2113208320;
	// 8324AB94: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324AB98: 388393EC  addi r4, r3, -0x6c14
	ctx.r[4].s64 = ctx.r[3].s64 + -27668;
	// 8324AB9C: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 8324ABA0: 4AFE2331  bl 0x8222ced0
	ctx.lr = 0x8324ABA4;
	sub_8222CED0(ctx, base);
	// 8324ABA4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8324ABA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8324ABAC: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 8324ABB0: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 8324ABB4: 915F0020  stw r10, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	pc = 0x8324ABB8; continue 'dispatch;
            }
            0x8324ABB8 => {
    //   block [0x8324ABB8..0x8324AC18)
	// 8324ABB8: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 8324ABBC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324ABC0: 7D203828  lwarx r9, 0, r7
	// lwarx
	let ea = ctx.r[7].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 8324ABC4: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 8324ABC8: 7D20392D  stwcx. r9, 0, r7
	// stwcx.
	let addr = ctx.r[7].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324ABCC: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324ABD0: 4082FFE8  bne 0x8324abb8
	if !ctx.cr[0].eq {
	pc = 0x8324ABB8; continue 'dispatch;
	}
	// 8324ABD4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8324ABD8: 3CC0820B  lis r6, -0x7df5
	ctx.r[6].s64 = -2113208320;
	// 8324ABDC: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 8324ABE0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324ABE4: 388693E4  addi r4, r6, -0x6c1c
	ctx.r[4].s64 = ctx.r[6].s64 + -27676;
	// 8324ABE8: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 8324ABEC: 4AFE22E5  bl 0x8222ced0
	ctx.lr = 0x8324ABF0;
	sub_8222CED0(ctx, base);
	// 8324ABF0: 3C80820B  lis r4, -0x7df5
	ctx.r[4].s64 = -2113208320;
	// 8324ABF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324ABF8: 388493D0  addi r4, r4, -0x6c30
	ctx.r[4].s64 = ctx.r[4].s64 + -27696;
	// 8324ABFC: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 8324AC00: 4AFE22D1  bl 0x8222ced0
	ctx.lr = 0x8324AC04;
	sub_8222CED0(ctx, base);
	// 8324AC04: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8324AC08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8324AC0C: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 8324AC10: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 8324AC14: 915F0034  stw r10, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[10].u32 ) };
	pc = 0x8324AC18; continue 'dispatch;
            }
            0x8324AC18 => {
    //   block [0x8324AC18..0x8324AC74)
	// 8324AC18: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8324AC1C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324AC20: 7C604028  lwarx r3, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[3].u64 = ctx.reserved.u32 as u64;
	// 8324AC24: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 8324AC28: 7C60412D  stwcx. r3, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[3].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324AC2C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324AC30: 4082FFE8  bne 0x8324ac18
	if !ctx.cr[0].eq {
	pc = 0x8324AC18; continue 'dispatch;
	}
	// 8324AC34: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8324AC38: 3CE0820B  lis r7, -0x7df5
	ctx.r[7].s64 = -2113208320;
	// 8324AC3C: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 8324AC40: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324AC44: 388793C4  addi r4, r7, -0x6c3c
	ctx.r[4].s64 = ctx.r[7].s64 + -27708;
	// 8324AC48: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 8324AC4C: 4AFE2285  bl 0x8222ced0
	ctx.lr = 0x8324AC50;
	sub_8222CED0(ctx, base);
	// 8324AC50: 3CC0820B  lis r6, -0x7df5
	ctx.r[6].s64 = -2113208320;
	// 8324AC54: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324AC58: 388693B0  addi r4, r6, -0x6c50
	ctx.r[4].s64 = ctx.r[6].s64 + -27728;
	// 8324AC5C: 387F0040  addi r3, r31, 0x40
	ctx.r[3].s64 = ctx.r[31].s64 + 64;
	// 8324AC60: 4AFE2271  bl 0x8222ced0
	ctx.lr = 0x8324AC64;
	sub_8222CED0(ctx, base);
	// 8324AC64: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8324AC68: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8324AC6C: 917F0044  stw r11, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 8324AC70: 917F0048  stw r11, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	pc = 0x8324AC74; continue 'dispatch;
            }
            0x8324AC74 => {
    //   block [0x8324AC74..0x8324ACD4)
	// 8324AC74: 7C8000A6  mfmsr r4
	ctx.r[4].u64 = ctx.msr;
	// 8324AC78: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324AC7C: 7CA01828  lwarx r5, 0, r3
	// lwarx
	let ea = ctx.r[3].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[5].u64 = ctx.reserved.u32 as u64;
	// 8324AC80: 38A50001  addi r5, r5, 1
	ctx.r[5].s64 = ctx.r[5].s64 + 1;
	// 8324AC84: 7CA0192D  stwcx. r5, 0, r3
	// stwcx.
	let addr = ctx.r[3].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[5].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324AC88: 7C810164  mtmsrd r4, 1
	ctx.msr = (ctx.r[4].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324AC8C: 4082FFE8  bne 0x8324ac74
	if !ctx.cr[0].eq {
	pc = 0x8324AC74; continue 'dispatch;
	}
	// 8324AC90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8324AC94: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 8324AC98: 917F004C  stw r11, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 8324AC9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324ACA0: 388A93A8  addi r4, r10, -0x6c58
	ctx.r[4].s64 = ctx.r[10].s64 + -27736;
	// 8324ACA4: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8324ACA8: 4AFE2229  bl 0x8222ced0
	ctx.lr = 0x8324ACAC;
	sub_8222CED0(ctx, base);
	// 8324ACAC: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 8324ACB0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324ACB4: 38899394  addi r4, r9, -0x6c6c
	ctx.r[4].s64 = ctx.r[9].s64 + -27756;
	// 8324ACB8: 387F0054  addi r3, r31, 0x54
	ctx.r[3].s64 = ctx.r[31].s64 + 84;
	// 8324ACBC: 4AFE2215  bl 0x8222ced0
	ctx.lr = 0x8324ACC0;
	sub_8222CED0(ctx, base);
	// 8324ACC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8324ACC4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8324ACC8: 917F0058  stw r11, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8324ACCC: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8324ACD0: 915F005C  stw r10, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	pc = 0x8324ACD4; continue 'dispatch;
            }
            0x8324ACD4 => {
    //   block [0x8324ACD4..0x8324AD30)
	// 8324ACD4: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 8324ACD8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324ACDC: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 8324ACE0: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 8324ACE4: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324ACE8: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324ACEC: 4082FFE8  bne 0x8324acd4
	if !ctx.cr[0].eq {
	pc = 0x8324ACD4; continue 'dispatch;
	}
	// 8324ACF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8324ACF4: 3C80820B  lis r4, -0x7df5
	ctx.r[4].s64 = -2113208320;
	// 8324ACF8: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 8324ACFC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324AD00: 3884938C  addi r4, r4, -0x6c74
	ctx.r[4].s64 = ctx.r[4].s64 + -27764;
	// 8324AD04: 387F0064  addi r3, r31, 0x64
	ctx.r[3].s64 = ctx.r[31].s64 + 100;
	// 8324AD08: 4AFE21C9  bl 0x8222ced0
	ctx.lr = 0x8324AD0C;
	sub_8222CED0(ctx, base);
	// 8324AD0C: 3C60820B  lis r3, -0x7df5
	ctx.r[3].s64 = -2113208320;
	// 8324AD10: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324AD14: 38839378  addi r4, r3, -0x6c88
	ctx.r[4].s64 = ctx.r[3].s64 + -27784;
	// 8324AD18: 387F0068  addi r3, r31, 0x68
	ctx.r[3].s64 = ctx.r[31].s64 + 104;
	// 8324AD1C: 4AFE21B5  bl 0x8222ced0
	ctx.lr = 0x8324AD20;
	sub_8222CED0(ctx, base);
	// 8324AD20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8324AD24: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8324AD28: 917F006C  stw r11, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8324AD2C: 915F0070  stw r10, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[10].u32 ) };
	pc = 0x8324AD30; continue 'dispatch;
            }
            0x8324AD30 => {
    //   block [0x8324AD30..0x8324AD78)
	// 8324AD30: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 8324AD34: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324AD38: 7D20F028  lwarx r9, 0, r30
	// lwarx
	let ea = ctx.r[30].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 8324AD3C: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 8324AD40: 7D20F12D  stwcx. r9, 0, r30
	// stwcx.
	let addr = ctx.r[30].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324AD44: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324AD48: 4082FFE8  bne 0x8324ad30
	if !ctx.cr[0].eq {
	pc = 0x8324AD30; continue 'dispatch;
	}
	// 8324AD4C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8324AD50: 3CE0832A  lis r7, -0x7cd6
	ctx.r[7].s64 = -2094399488;
	// 8324AD54: 917F0074  stw r11, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8324AD58: 38677EF8  addi r3, r7, 0x7ef8
	ctx.r[3].s64 = ctx.r[7].s64 + 32504;
	// 8324AD5C: 4BA5F1C5  bl 0x82ca9f20
	ctx.lr = 0x8324AD60;
	sub_82CA9F20(ctx, base);
	// 8324AD60: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8324AD64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324AD68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324AD6C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8324AD70: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8324AD74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324AD78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8324AD78 size=24
    let mut pc: u32 = 0x8324AD78;
    'dispatch: loop {
        match pc {
            0x8324AD78 => {
    //   block [0x8324AD78..0x8324AD90)
	// 8324AD78: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 8324AD7C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8324AD80: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 8324AD84: 38697FD8  addi r3, r9, 0x7fd8
	ctx.r[3].s64 = ctx.r[9].s64 + 32728;
	// 8324AD88: 916AC4AC  stw r11, -0x3b54(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-15188 as u32), ctx.r[11].u32 ) };
	// 8324AD8C: 4BA5F194  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324AD90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324AD90 size=64
    let mut pc: u32 = 0x8324AD90;
    'dispatch: loop {
        match pc {
            0x8324AD90 => {
    //   block [0x8324AD90..0x8324ADD0)
	// 8324AD90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324AD94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324AD98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324AD9C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324ADA0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324ADA4: 388B96F0  addi r4, r11, -0x6910
	ctx.r[4].s64 = ctx.r[11].s64 + -26896;
	// 8324ADA8: 386A7888  addi r3, r10, 0x7888
	ctx.r[3].s64 = ctx.r[10].s64 + 30856;
	// 8324ADAC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324ADB0: 4AFE2121  bl 0x8222ced0
	ctx.lr = 0x8324ADB4;
	sub_8222CED0(ctx, base);
	// 8324ADB4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324ADB8: 38698050  addi r3, r9, -0x7fb0
	ctx.r[3].s64 = ctx.r[9].s64 + -32688;
	// 8324ADBC: 4BA5F165  bl 0x82ca9f20
	ctx.lr = 0x8324ADC0;
	sub_82CA9F20(ctx, base);
	// 8324ADC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324ADC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324ADC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324ADCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324ADD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324ADD0 size=64
    let mut pc: u32 = 0x8324ADD0;
    'dispatch: loop {
        match pc {
            0x8324ADD0 => {
    //   block [0x8324ADD0..0x8324AE10)
	// 8324ADD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324ADD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324ADD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324ADDC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324ADE0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324ADE4: 388B96F8  addi r4, r11, -0x6908
	ctx.r[4].s64 = ctx.r[11].s64 + -26888;
	// 8324ADE8: 386A788C  addi r3, r10, 0x788c
	ctx.r[3].s64 = ctx.r[10].s64 + 30860;
	// 8324ADEC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324ADF0: 4AFE20E1  bl 0x8222ced0
	ctx.lr = 0x8324ADF4;
	sub_8222CED0(ctx, base);
	// 8324ADF4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324ADF8: 38698060  addi r3, r9, -0x7fa0
	ctx.r[3].s64 = ctx.r[9].s64 + -32672;
	// 8324ADFC: 4BA5F125  bl 0x82ca9f20
	ctx.lr = 0x8324AE00;
	sub_82CA9F20(ctx, base);
	// 8324AE00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324AE04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324AE08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324AE0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324AE10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324AE10 size=64
    let mut pc: u32 = 0x8324AE10;
    'dispatch: loop {
        match pc {
            0x8324AE10 => {
    //   block [0x8324AE10..0x8324AE50)
	// 8324AE10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324AE14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324AE18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324AE1C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324AE20: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324AE24: 388B970C  addi r4, r11, -0x68f4
	ctx.r[4].s64 = ctx.r[11].s64 + -26868;
	// 8324AE28: 386A7890  addi r3, r10, 0x7890
	ctx.r[3].s64 = ctx.r[10].s64 + 30864;
	// 8324AE2C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324AE30: 4AFE20A1  bl 0x8222ced0
	ctx.lr = 0x8324AE34;
	sub_8222CED0(ctx, base);
	// 8324AE34: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324AE38: 38698070  addi r3, r9, -0x7f90
	ctx.r[3].s64 = ctx.r[9].s64 + -32656;
	// 8324AE3C: 4BA5F0E5  bl 0x82ca9f20
	ctx.lr = 0x8324AE40;
	sub_82CA9F20(ctx, base);
	// 8324AE40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324AE44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324AE48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324AE4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324AE50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324AE50 size=64
    let mut pc: u32 = 0x8324AE50;
    'dispatch: loop {
        match pc {
            0x8324AE50 => {
    //   block [0x8324AE50..0x8324AE90)
	// 8324AE50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324AE54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324AE58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324AE5C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324AE60: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324AE64: 388B9718  addi r4, r11, -0x68e8
	ctx.r[4].s64 = ctx.r[11].s64 + -26856;
	// 8324AE68: 386A7894  addi r3, r10, 0x7894
	ctx.r[3].s64 = ctx.r[10].s64 + 30868;
	// 8324AE6C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324AE70: 4AFE2061  bl 0x8222ced0
	ctx.lr = 0x8324AE74;
	sub_8222CED0(ctx, base);
	// 8324AE74: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324AE78: 38698080  addi r3, r9, -0x7f80
	ctx.r[3].s64 = ctx.r[9].s64 + -32640;
	// 8324AE7C: 4BA5F0A5  bl 0x82ca9f20
	ctx.lr = 0x8324AE80;
	sub_82CA9F20(ctx, base);
	// 8324AE80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324AE84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324AE88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324AE8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324AE90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324AE90 size=64
    let mut pc: u32 = 0x8324AE90;
    'dispatch: loop {
        match pc {
            0x8324AE90 => {
    //   block [0x8324AE90..0x8324AED0)
	// 8324AE90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324AE94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324AE98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324AE9C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324AEA0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324AEA4: 388B9728  addi r4, r11, -0x68d8
	ctx.r[4].s64 = ctx.r[11].s64 + -26840;
	// 8324AEA8: 386A7898  addi r3, r10, 0x7898
	ctx.r[3].s64 = ctx.r[10].s64 + 30872;
	// 8324AEAC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324AEB0: 4AFE2021  bl 0x8222ced0
	ctx.lr = 0x8324AEB4;
	sub_8222CED0(ctx, base);
	// 8324AEB4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324AEB8: 38698090  addi r3, r9, -0x7f70
	ctx.r[3].s64 = ctx.r[9].s64 + -32624;
	// 8324AEBC: 4BA5F065  bl 0x82ca9f20
	ctx.lr = 0x8324AEC0;
	sub_82CA9F20(ctx, base);
	// 8324AEC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324AEC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324AEC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324AECC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324AED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324AED0 size=64
    let mut pc: u32 = 0x8324AED0;
    'dispatch: loop {
        match pc {
            0x8324AED0 => {
    //   block [0x8324AED0..0x8324AF10)
	// 8324AED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324AED4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324AED8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324AEDC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324AEE0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324AEE4: 388B9738  addi r4, r11, -0x68c8
	ctx.r[4].s64 = ctx.r[11].s64 + -26824;
	// 8324AEE8: 386A789C  addi r3, r10, 0x789c
	ctx.r[3].s64 = ctx.r[10].s64 + 30876;
	// 8324AEEC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324AEF0: 4AFE1FE1  bl 0x8222ced0
	ctx.lr = 0x8324AEF4;
	sub_8222CED0(ctx, base);
	// 8324AEF4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324AEF8: 386980A0  addi r3, r9, -0x7f60
	ctx.r[3].s64 = ctx.r[9].s64 + -32608;
	// 8324AEFC: 4BA5F025  bl 0x82ca9f20
	ctx.lr = 0x8324AF00;
	sub_82CA9F20(ctx, base);
	// 8324AF00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324AF04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324AF08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324AF0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324AF10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324AF10 size=64
    let mut pc: u32 = 0x8324AF10;
    'dispatch: loop {
        match pc {
            0x8324AF10 => {
    //   block [0x8324AF10..0x8324AF50)
	// 8324AF10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324AF14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324AF18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324AF1C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324AF20: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324AF24: 388B9744  addi r4, r11, -0x68bc
	ctx.r[4].s64 = ctx.r[11].s64 + -26812;
	// 8324AF28: 386A78A0  addi r3, r10, 0x78a0
	ctx.r[3].s64 = ctx.r[10].s64 + 30880;
	// 8324AF2C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324AF30: 4AFE1FA1  bl 0x8222ced0
	ctx.lr = 0x8324AF34;
	sub_8222CED0(ctx, base);
	// 8324AF34: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324AF38: 386980B0  addi r3, r9, -0x7f50
	ctx.r[3].s64 = ctx.r[9].s64 + -32592;
	// 8324AF3C: 4BA5EFE5  bl 0x82ca9f20
	ctx.lr = 0x8324AF40;
	sub_82CA9F20(ctx, base);
	// 8324AF40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324AF44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324AF48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324AF4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324AF50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324AF50 size=64
    let mut pc: u32 = 0x8324AF50;
    'dispatch: loop {
        match pc {
            0x8324AF50 => {
    //   block [0x8324AF50..0x8324AF90)
	// 8324AF50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324AF54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324AF58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324AF5C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324AF60: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324AF64: 388B9754  addi r4, r11, -0x68ac
	ctx.r[4].s64 = ctx.r[11].s64 + -26796;
	// 8324AF68: 386A78A4  addi r3, r10, 0x78a4
	ctx.r[3].s64 = ctx.r[10].s64 + 30884;
	// 8324AF6C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324AF70: 4AFE1F61  bl 0x8222ced0
	ctx.lr = 0x8324AF74;
	sub_8222CED0(ctx, base);
	// 8324AF74: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324AF78: 386980C0  addi r3, r9, -0x7f40
	ctx.r[3].s64 = ctx.r[9].s64 + -32576;
	// 8324AF7C: 4BA5EFA5  bl 0x82ca9f20
	ctx.lr = 0x8324AF80;
	sub_82CA9F20(ctx, base);
	// 8324AF80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324AF84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324AF88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324AF8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324AF90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324AF90 size=64
    let mut pc: u32 = 0x8324AF90;
    'dispatch: loop {
        match pc {
            0x8324AF90 => {
    //   block [0x8324AF90..0x8324AFD0)
	// 8324AF90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324AF94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324AF98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324AF9C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324AFA0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324AFA4: 388B9764  addi r4, r11, -0x689c
	ctx.r[4].s64 = ctx.r[11].s64 + -26780;
	// 8324AFA8: 386A78A8  addi r3, r10, 0x78a8
	ctx.r[3].s64 = ctx.r[10].s64 + 30888;
	// 8324AFAC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324AFB0: 4AFE1F21  bl 0x8222ced0
	ctx.lr = 0x8324AFB4;
	sub_8222CED0(ctx, base);
	// 8324AFB4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324AFB8: 386980D0  addi r3, r9, -0x7f30
	ctx.r[3].s64 = ctx.r[9].s64 + -32560;
	// 8324AFBC: 4BA5EF65  bl 0x82ca9f20
	ctx.lr = 0x8324AFC0;
	sub_82CA9F20(ctx, base);
	// 8324AFC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324AFC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324AFC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324AFCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324AFD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324AFD0 size=64
    let mut pc: u32 = 0x8324AFD0;
    'dispatch: loop {
        match pc {
            0x8324AFD0 => {
    //   block [0x8324AFD0..0x8324B010)
	// 8324AFD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324AFD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324AFD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324AFDC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324AFE0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324AFE4: 388B977C  addi r4, r11, -0x6884
	ctx.r[4].s64 = ctx.r[11].s64 + -26756;
	// 8324AFE8: 386A78AC  addi r3, r10, 0x78ac
	ctx.r[3].s64 = ctx.r[10].s64 + 30892;
	// 8324AFEC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324AFF0: 4AFE1EE1  bl 0x8222ced0
	ctx.lr = 0x8324AFF4;
	sub_8222CED0(ctx, base);
	// 8324AFF4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324AFF8: 386980E0  addi r3, r9, -0x7f20
	ctx.r[3].s64 = ctx.r[9].s64 + -32544;
	// 8324AFFC: 4BA5EF25  bl 0x82ca9f20
	ctx.lr = 0x8324B000;
	sub_82CA9F20(ctx, base);
	// 8324B000: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B004: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B008: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B00C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B010 size=64
    let mut pc: u32 = 0x8324B010;
    'dispatch: loop {
        match pc {
            0x8324B010 => {
    //   block [0x8324B010..0x8324B050)
	// 8324B010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B018: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B01C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324B020: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B024: 388B9794  addi r4, r11, -0x686c
	ctx.r[4].s64 = ctx.r[11].s64 + -26732;
	// 8324B028: 386A78B0  addi r3, r10, 0x78b0
	ctx.r[3].s64 = ctx.r[10].s64 + 30896;
	// 8324B02C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B030: 4AFE1EA1  bl 0x8222ced0
	ctx.lr = 0x8324B034;
	sub_8222CED0(ctx, base);
	// 8324B034: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B038: 386980F0  addi r3, r9, -0x7f10
	ctx.r[3].s64 = ctx.r[9].s64 + -32528;
	// 8324B03C: 4BA5EEE5  bl 0x82ca9f20
	ctx.lr = 0x8324B040;
	sub_82CA9F20(ctx, base);
	// 8324B040: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B044: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B048: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B04C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B050 size=64
    let mut pc: u32 = 0x8324B050;
    'dispatch: loop {
        match pc {
            0x8324B050 => {
    //   block [0x8324B050..0x8324B090)
	// 8324B050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B058: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B05C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324B060: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B064: 388B97A4  addi r4, r11, -0x685c
	ctx.r[4].s64 = ctx.r[11].s64 + -26716;
	// 8324B068: 386A78B4  addi r3, r10, 0x78b4
	ctx.r[3].s64 = ctx.r[10].s64 + 30900;
	// 8324B06C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B070: 4AFE1E61  bl 0x8222ced0
	ctx.lr = 0x8324B074;
	sub_8222CED0(ctx, base);
	// 8324B074: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B078: 38698100  addi r3, r9, -0x7f00
	ctx.r[3].s64 = ctx.r[9].s64 + -32512;
	// 8324B07C: 4BA5EEA5  bl 0x82ca9f20
	ctx.lr = 0x8324B080;
	sub_82CA9F20(ctx, base);
	// 8324B080: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B084: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B088: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B08C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B090 size=64
    let mut pc: u32 = 0x8324B090;
    'dispatch: loop {
        match pc {
            0x8324B090 => {
    //   block [0x8324B090..0x8324B0D0)
	// 8324B090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B098: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B09C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324B0A0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B0A4: 388B97B8  addi r4, r11, -0x6848
	ctx.r[4].s64 = ctx.r[11].s64 + -26696;
	// 8324B0A8: 386A78B8  addi r3, r10, 0x78b8
	ctx.r[3].s64 = ctx.r[10].s64 + 30904;
	// 8324B0AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B0B0: 4AFE1E21  bl 0x8222ced0
	ctx.lr = 0x8324B0B4;
	sub_8222CED0(ctx, base);
	// 8324B0B4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B0B8: 38698110  addi r3, r9, -0x7ef0
	ctx.r[3].s64 = ctx.r[9].s64 + -32496;
	// 8324B0BC: 4BA5EE65  bl 0x82ca9f20
	ctx.lr = 0x8324B0C0;
	sub_82CA9F20(ctx, base);
	// 8324B0C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B0C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B0C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B0CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B0D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B0D0 size=64
    let mut pc: u32 = 0x8324B0D0;
    'dispatch: loop {
        match pc {
            0x8324B0D0 => {
    //   block [0x8324B0D0..0x8324B110)
	// 8324B0D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B0D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B0D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B0DC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324B0E0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B0E4: 388B97C4  addi r4, r11, -0x683c
	ctx.r[4].s64 = ctx.r[11].s64 + -26684;
	// 8324B0E8: 386A78BC  addi r3, r10, 0x78bc
	ctx.r[3].s64 = ctx.r[10].s64 + 30908;
	// 8324B0EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B0F0: 4AFE1DE1  bl 0x8222ced0
	ctx.lr = 0x8324B0F4;
	sub_8222CED0(ctx, base);
	// 8324B0F4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B0F8: 38698120  addi r3, r9, -0x7ee0
	ctx.r[3].s64 = ctx.r[9].s64 + -32480;
	// 8324B0FC: 4BA5EE25  bl 0x82ca9f20
	ctx.lr = 0x8324B100;
	sub_82CA9F20(ctx, base);
	// 8324B100: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B104: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B108: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B10C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B110 size=64
    let mut pc: u32 = 0x8324B110;
    'dispatch: loop {
        match pc {
            0x8324B110 => {
    //   block [0x8324B110..0x8324B150)
	// 8324B110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B118: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B11C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324B120: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B124: 388B97DC  addi r4, r11, -0x6824
	ctx.r[4].s64 = ctx.r[11].s64 + -26660;
	// 8324B128: 386A78C0  addi r3, r10, 0x78c0
	ctx.r[3].s64 = ctx.r[10].s64 + 30912;
	// 8324B12C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B130: 4AFE1DA1  bl 0x8222ced0
	ctx.lr = 0x8324B134;
	sub_8222CED0(ctx, base);
	// 8324B134: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B138: 38698130  addi r3, r9, -0x7ed0
	ctx.r[3].s64 = ctx.r[9].s64 + -32464;
	// 8324B13C: 4BA5EDE5  bl 0x82ca9f20
	ctx.lr = 0x8324B140;
	sub_82CA9F20(ctx, base);
	// 8324B140: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B144: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B148: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B14C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B150 size=88
    let mut pc: u32 = 0x8324B150;
    'dispatch: loop {
        match pc {
            0x8324B150 => {
    //   block [0x8324B150..0x8324B1A8)
	// 8324B150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B158: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8324B15C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B160: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 8324B164: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B168: 3BEB78C4  addi r31, r11, 0x78c4
	ctx.r[31].s64 = ctx.r[11].s64 + 30916;
	// 8324B16C: 388A7888  addi r4, r10, 0x7888
	ctx.r[4].s64 = ctx.r[10].s64 + 30856;
	// 8324B170: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8324B174: 4AFA50CD  bl 0x821f0240
	ctx.lr = 0x8324B178;
	sub_821F0240(ctx, base);
	// 8324B178: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 8324B17C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8324B180: 388997FC  addi r4, r9, -0x6804
	ctx.r[4].s64 = ctx.r[9].s64 + -26628;
	// 8324B184: 4AF8F83D  bl 0x821da9c0
	ctx.lr = 0x8324B188;
	sub_821DA9C0(ctx, base);
	// 8324B188: 3D00832B  lis r8, -0x7cd5
	ctx.r[8].s64 = -2094333952;
	// 8324B18C: 38688140  addi r3, r8, -0x7ec0
	ctx.r[3].s64 = ctx.r[8].s64 + -32448;
	// 8324B190: 4BA5ED91  bl 0x82ca9f20
	ctx.lr = 0x8324B194;
	sub_82CA9F20(ctx, base);
	// 8324B194: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B198: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B19C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B1A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8324B1A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B1A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B1A8 size=64
    let mut pc: u32 = 0x8324B1A8;
    'dispatch: loop {
        match pc {
            0x8324B1A8 => {
    //   block [0x8324B1A8..0x8324B1E8)
	// 8324B1A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B1AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B1B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B1B4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324B1B8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B1BC: 388B980C  addi r4, r11, -0x67f4
	ctx.r[4].s64 = ctx.r[11].s64 + -26612;
	// 8324B1C0: 386A78C8  addi r3, r10, 0x78c8
	ctx.r[3].s64 = ctx.r[10].s64 + 30920;
	// 8324B1C4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B1C8: 4AFE1D09  bl 0x8222ced0
	ctx.lr = 0x8324B1CC;
	sub_8222CED0(ctx, base);
	// 8324B1CC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B1D0: 38698150  addi r3, r9, -0x7eb0
	ctx.r[3].s64 = ctx.r[9].s64 + -32432;
	// 8324B1D4: 4BA5ED4D  bl 0x82ca9f20
	ctx.lr = 0x8324B1D8;
	sub_82CA9F20(ctx, base);
	// 8324B1D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B1DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B1E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B1E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B1E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B1E8 size=64
    let mut pc: u32 = 0x8324B1E8;
    'dispatch: loop {
        match pc {
            0x8324B1E8 => {
    //   block [0x8324B1E8..0x8324B228)
	// 8324B1E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B1EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B1F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B1F4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324B1F8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B1FC: 388B9824  addi r4, r11, -0x67dc
	ctx.r[4].s64 = ctx.r[11].s64 + -26588;
	// 8324B200: 386A78CC  addi r3, r10, 0x78cc
	ctx.r[3].s64 = ctx.r[10].s64 + 30924;
	// 8324B204: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B208: 4AFE1CC9  bl 0x8222ced0
	ctx.lr = 0x8324B20C;
	sub_8222CED0(ctx, base);
	// 8324B20C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B210: 38698160  addi r3, r9, -0x7ea0
	ctx.r[3].s64 = ctx.r[9].s64 + -32416;
	// 8324B214: 4BA5ED0D  bl 0x82ca9f20
	ctx.lr = 0x8324B218;
	sub_82CA9F20(ctx, base);
	// 8324B218: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B21C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B220: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B224: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B228 size=64
    let mut pc: u32 = 0x8324B228;
    'dispatch: loop {
        match pc {
            0x8324B228 => {
    //   block [0x8324B228..0x8324B268)
	// 8324B228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B22C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B230: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B234: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324B238: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B23C: 388B9838  addi r4, r11, -0x67c8
	ctx.r[4].s64 = ctx.r[11].s64 + -26568;
	// 8324B240: 386A78D0  addi r3, r10, 0x78d0
	ctx.r[3].s64 = ctx.r[10].s64 + 30928;
	// 8324B244: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B248: 4AFE1C89  bl 0x8222ced0
	ctx.lr = 0x8324B24C;
	sub_8222CED0(ctx, base);
	// 8324B24C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B250: 38698170  addi r3, r9, -0x7e90
	ctx.r[3].s64 = ctx.r[9].s64 + -32400;
	// 8324B254: 4BA5ECCD  bl 0x82ca9f20
	ctx.lr = 0x8324B258;
	sub_82CA9F20(ctx, base);
	// 8324B258: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B25C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B260: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B264: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B268 size=64
    let mut pc: u32 = 0x8324B268;
    'dispatch: loop {
        match pc {
            0x8324B268 => {
    //   block [0x8324B268..0x8324B2A8)
	// 8324B268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B26C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B270: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B274: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324B278: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B27C: 388B9840  addi r4, r11, -0x67c0
	ctx.r[4].s64 = ctx.r[11].s64 + -26560;
	// 8324B280: 386A78D4  addi r3, r10, 0x78d4
	ctx.r[3].s64 = ctx.r[10].s64 + 30932;
	// 8324B284: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B288: 4AFE1C49  bl 0x8222ced0
	ctx.lr = 0x8324B28C;
	sub_8222CED0(ctx, base);
	// 8324B28C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B290: 38698180  addi r3, r9, -0x7e80
	ctx.r[3].s64 = ctx.r[9].s64 + -32384;
	// 8324B294: 4BA5EC8D  bl 0x82ca9f20
	ctx.lr = 0x8324B298;
	sub_82CA9F20(ctx, base);
	// 8324B298: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B29C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B2A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B2A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B2A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B2A8 size=88
    let mut pc: u32 = 0x8324B2A8;
    'dispatch: loop {
        match pc {
            0x8324B2A8 => {
    //   block [0x8324B2A8..0x8324B300)
	// 8324B2A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B2AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B2B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8324B2B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B2B8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 8324B2BC: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B2C0: 3BEB78D8  addi r31, r11, 0x78d8
	ctx.r[31].s64 = ctx.r[11].s64 + 30936;
	// 8324B2C4: 388A78D4  addi r4, r10, 0x78d4
	ctx.r[4].s64 = ctx.r[10].s64 + 30932;
	// 8324B2C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8324B2CC: 4AFA4F75  bl 0x821f0240
	ctx.lr = 0x8324B2D0;
	sub_821F0240(ctx, base);
	// 8324B2D0: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 8324B2D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8324B2D8: 38899848  addi r4, r9, -0x67b8
	ctx.r[4].s64 = ctx.r[9].s64 + -26552;
	// 8324B2DC: 4AF8F6E5  bl 0x821da9c0
	ctx.lr = 0x8324B2E0;
	sub_821DA9C0(ctx, base);
	// 8324B2E0: 3D00832B  lis r8, -0x7cd5
	ctx.r[8].s64 = -2094333952;
	// 8324B2E4: 38688190  addi r3, r8, -0x7e70
	ctx.r[3].s64 = ctx.r[8].s64 + -32368;
	// 8324B2E8: 4BA5EC39  bl 0x82ca9f20
	ctx.lr = 0x8324B2EC;
	sub_82CA9F20(ctx, base);
	// 8324B2EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B2F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B2F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B2F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8324B2FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B300 size=64
    let mut pc: u32 = 0x8324B300;
    'dispatch: loop {
        match pc {
            0x8324B300 => {
    //   block [0x8324B300..0x8324B340)
	// 8324B300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B308: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B30C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324B310: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B314: 388B984C  addi r4, r11, -0x67b4
	ctx.r[4].s64 = ctx.r[11].s64 + -26548;
	// 8324B318: 386A78DC  addi r3, r10, 0x78dc
	ctx.r[3].s64 = ctx.r[10].s64 + 30940;
	// 8324B31C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B320: 4AFE1BB1  bl 0x8222ced0
	ctx.lr = 0x8324B324;
	sub_8222CED0(ctx, base);
	// 8324B324: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B328: 386981A0  addi r3, r9, -0x7e60
	ctx.r[3].s64 = ctx.r[9].s64 + -32352;
	// 8324B32C: 4BA5EBF5  bl 0x82ca9f20
	ctx.lr = 0x8324B330;
	sub_82CA9F20(ctx, base);
	// 8324B330: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B334: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B338: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B33C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B340 size=64
    let mut pc: u32 = 0x8324B340;
    'dispatch: loop {
        match pc {
            0x8324B340 => {
    //   block [0x8324B340..0x8324B380)
	// 8324B340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B348: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B34C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324B350: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B354: 388B9850  addi r4, r11, -0x67b0
	ctx.r[4].s64 = ctx.r[11].s64 + -26544;
	// 8324B358: 386A78E0  addi r3, r10, 0x78e0
	ctx.r[3].s64 = ctx.r[10].s64 + 30944;
	// 8324B35C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B360: 4AFE1B71  bl 0x8222ced0
	ctx.lr = 0x8324B364;
	sub_8222CED0(ctx, base);
	// 8324B364: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B368: 386981B0  addi r3, r9, -0x7e50
	ctx.r[3].s64 = ctx.r[9].s64 + -32336;
	// 8324B36C: 4BA5EBB5  bl 0x82ca9f20
	ctx.lr = 0x8324B370;
	sub_82CA9F20(ctx, base);
	// 8324B370: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B374: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B378: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B37C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B380 size=64
    let mut pc: u32 = 0x8324B380;
    'dispatch: loop {
        match pc {
            0x8324B380 => {
    //   block [0x8324B380..0x8324B3C0)
	// 8324B380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B388: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B38C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324B390: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B394: 388B9860  addi r4, r11, -0x67a0
	ctx.r[4].s64 = ctx.r[11].s64 + -26528;
	// 8324B398: 386A78E4  addi r3, r10, 0x78e4
	ctx.r[3].s64 = ctx.r[10].s64 + 30948;
	// 8324B39C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B3A0: 4AFE1B31  bl 0x8222ced0
	ctx.lr = 0x8324B3A4;
	sub_8222CED0(ctx, base);
	// 8324B3A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B3A8: 386981C0  addi r3, r9, -0x7e40
	ctx.r[3].s64 = ctx.r[9].s64 + -32320;
	// 8324B3AC: 4BA5EB75  bl 0x82ca9f20
	ctx.lr = 0x8324B3B0;
	sub_82CA9F20(ctx, base);
	// 8324B3B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B3B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B3B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B3BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B3C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B3C0 size=64
    let mut pc: u32 = 0x8324B3C0;
    'dispatch: loop {
        match pc {
            0x8324B3C0 => {
    //   block [0x8324B3C0..0x8324B400)
	// 8324B3C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B3C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B3C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B3CC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324B3D0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B3D4: 388B987C  addi r4, r11, -0x6784
	ctx.r[4].s64 = ctx.r[11].s64 + -26500;
	// 8324B3D8: 386A78E8  addi r3, r10, 0x78e8
	ctx.r[3].s64 = ctx.r[10].s64 + 30952;
	// 8324B3DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B3E0: 4AFE1AF1  bl 0x8222ced0
	ctx.lr = 0x8324B3E4;
	sub_8222CED0(ctx, base);
	// 8324B3E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B3E8: 386981D0  addi r3, r9, -0x7e30
	ctx.r[3].s64 = ctx.r[9].s64 + -32304;
	// 8324B3EC: 4BA5EB35  bl 0x82ca9f20
	ctx.lr = 0x8324B3F0;
	sub_82CA9F20(ctx, base);
	// 8324B3F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B3F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B3F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B3FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B400 size=64
    let mut pc: u32 = 0x8324B400;
    'dispatch: loop {
        match pc {
            0x8324B400 => {
    //   block [0x8324B400..0x8324B440)
	// 8324B400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B408: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B40C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324B410: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B414: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8324B418: 386A78EC  addi r3, r10, 0x78ec
	ctx.r[3].s64 = ctx.r[10].s64 + 30956;
	// 8324B41C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B420: 4AFE1AB1  bl 0x8222ced0
	ctx.lr = 0x8324B424;
	sub_8222CED0(ctx, base);
	// 8324B424: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B428: 386981E0  addi r3, r9, -0x7e20
	ctx.r[3].s64 = ctx.r[9].s64 + -32288;
	// 8324B42C: 4BA5EAF5  bl 0x82ca9f20
	ctx.lr = 0x8324B430;
	sub_82CA9F20(ctx, base);
	// 8324B430: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B434: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B438: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B43C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B440 size=64
    let mut pc: u32 = 0x8324B440;
    'dispatch: loop {
        match pc {
            0x8324B440 => {
    //   block [0x8324B440..0x8324B480)
	// 8324B440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B448: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B44C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324B450: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B454: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8324B458: 386A78F0  addi r3, r10, 0x78f0
	ctx.r[3].s64 = ctx.r[10].s64 + 30960;
	// 8324B45C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B460: 4AFE1A71  bl 0x8222ced0
	ctx.lr = 0x8324B464;
	sub_8222CED0(ctx, base);
	// 8324B464: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B468: 386981F0  addi r3, r9, -0x7e10
	ctx.r[3].s64 = ctx.r[9].s64 + -32272;
	// 8324B46C: 4BA5EAB5  bl 0x82ca9f20
	ctx.lr = 0x8324B470;
	sub_82CA9F20(ctx, base);
	// 8324B470: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B474: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B478: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B47C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B480 size=64
    let mut pc: u32 = 0x8324B480;
    'dispatch: loop {
        match pc {
            0x8324B480 => {
    //   block [0x8324B480..0x8324B4C0)
	// 8324B480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B488: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B48C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324B490: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B494: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8324B498: 386A78F4  addi r3, r10, 0x78f4
	ctx.r[3].s64 = ctx.r[10].s64 + 30964;
	// 8324B49C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B4A0: 4AFE1A31  bl 0x8222ced0
	ctx.lr = 0x8324B4A4;
	sub_8222CED0(ctx, base);
	// 8324B4A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B4A8: 38698200  addi r3, r9, -0x7e00
	ctx.r[3].s64 = ctx.r[9].s64 + -32256;
	// 8324B4AC: 4BA5EA75  bl 0x82ca9f20
	ctx.lr = 0x8324B4B0;
	sub_82CA9F20(ctx, base);
	// 8324B4B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B4B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B4B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B4BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B4C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B4C0 size=64
    let mut pc: u32 = 0x8324B4C0;
    'dispatch: loop {
        match pc {
            0x8324B4C0 => {
    //   block [0x8324B4C0..0x8324B500)
	// 8324B4C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B4C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B4C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B4CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324B4D0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B4D4: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8324B4D8: 386A78F8  addi r3, r10, 0x78f8
	ctx.r[3].s64 = ctx.r[10].s64 + 30968;
	// 8324B4DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B4E0: 4AFE19F1  bl 0x8222ced0
	ctx.lr = 0x8324B4E4;
	sub_8222CED0(ctx, base);
	// 8324B4E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B4E8: 38698210  addi r3, r9, -0x7df0
	ctx.r[3].s64 = ctx.r[9].s64 + -32240;
	// 8324B4EC: 4BA5EA35  bl 0x82ca9f20
	ctx.lr = 0x8324B4F0;
	sub_82CA9F20(ctx, base);
	// 8324B4F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B4F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B4F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B4FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B500 size=64
    let mut pc: u32 = 0x8324B500;
    'dispatch: loop {
        match pc {
            0x8324B500 => {
    //   block [0x8324B500..0x8324B540)
	// 8324B500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B508: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B50C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324B510: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B514: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8324B518: 386A78FC  addi r3, r10, 0x78fc
	ctx.r[3].s64 = ctx.r[10].s64 + 30972;
	// 8324B51C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B520: 4AFE19B1  bl 0x8222ced0
	ctx.lr = 0x8324B524;
	sub_8222CED0(ctx, base);
	// 8324B524: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B528: 38698220  addi r3, r9, -0x7de0
	ctx.r[3].s64 = ctx.r[9].s64 + -32224;
	// 8324B52C: 4BA5E9F5  bl 0x82ca9f20
	ctx.lr = 0x8324B530;
	sub_82CA9F20(ctx, base);
	// 8324B530: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B534: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B538: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B53C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B540 size=64
    let mut pc: u32 = 0x8324B540;
    'dispatch: loop {
        match pc {
            0x8324B540 => {
    //   block [0x8324B540..0x8324B580)
	// 8324B540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B548: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B54C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324B550: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B554: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8324B558: 386A7900  addi r3, r10, 0x7900
	ctx.r[3].s64 = ctx.r[10].s64 + 30976;
	// 8324B55C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B560: 4AFE1971  bl 0x8222ced0
	ctx.lr = 0x8324B564;
	sub_8222CED0(ctx, base);
	// 8324B564: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B568: 38698230  addi r3, r9, -0x7dd0
	ctx.r[3].s64 = ctx.r[9].s64 + -32208;
	// 8324B56C: 4BA5E9B5  bl 0x82ca9f20
	ctx.lr = 0x8324B570;
	sub_82CA9F20(ctx, base);
	// 8324B570: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B574: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B578: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B57C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B580 size=96
    let mut pc: u32 = 0x8324B580;
    'dispatch: loop {
        match pc {
            0x8324B580 => {
    //   block [0x8324B580..0x8324B5A4)
	// 8324B580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B588: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B58C: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 8324B590: 4AFD3CC9  bl 0x8221f258
	ctx.lr = 0x8324B594;
	sub_8221F258(ctx, base);
	// 8324B594: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8324B598: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8324B59C: 419A0008  beq cr6, 0x8324b5a4
	if ctx.cr[6].eq {
	pc = 0x8324B5A4; continue 'dispatch;
	}
	// 8324B5A0: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x8324B5A4; continue 'dispatch;
            }
            0x8324B5A4 => {
    //   block [0x8324B5A4..0x8324B5B0)
	// 8324B5A4: 354B0004  addic. r10, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8324B5A8: 41820008  beq 0x8324b5b0
	if ctx.cr[0].eq {
	pc = 0x8324B5B0; continue 'dispatch;
	}
	// 8324B5AC: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x8324B5B0; continue 'dispatch;
            }
            0x8324B5B0 => {
    //   block [0x8324B5B0..0x8324B5E0)
	// 8324B5B0: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 8324B5B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8324B5B8: 39097904  addi r8, r9, 0x7904
	ctx.r[8].s64 = ctx.r[9].s64 + 30980;
	// 8324B5BC: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8324B5C0: 38678240  addi r3, r7, -0x7dc0
	ctx.r[3].s64 = ctx.r[7].s64 + -32192;
	// 8324B5C4: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8324B5C8: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8324B5CC: 4BA5E955  bl 0x82ca9f20
	ctx.lr = 0x8324B5D0;
	sub_82CA9F20(ctx, base);
	// 8324B5D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B5D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B5D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B5DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B5E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8324B5E0 size=12
    let mut pc: u32 = 0x8324B5E0;
    'dispatch: loop {
        match pc {
            0x8324B5E0 => {
    //   block [0x8324B5E0..0x8324B5EC)
	// 8324B5E0: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8324B5E4: 386B8300  addi r3, r11, -0x7d00
	ctx.r[3].s64 = ctx.r[11].s64 + -32000;
	// 8324B5E8: 4BA5E938  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B5F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B5F0 size=64
    let mut pc: u32 = 0x8324B5F0;
    'dispatch: loop {
        match pc {
            0x8324B5F0 => {
    //   block [0x8324B5F0..0x8324B630)
	// 8324B5F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B5F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B5F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B5FC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8324B600: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B604: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8324B608: 386A7924  addi r3, r10, 0x7924
	ctx.r[3].s64 = ctx.r[10].s64 + 31012;
	// 8324B60C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B610: 4AFE18C1  bl 0x8222ced0
	ctx.lr = 0x8324B614;
	sub_8222CED0(ctx, base);
	// 8324B614: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B618: 38698358  addi r3, r9, -0x7ca8
	ctx.r[3].s64 = ctx.r[9].s64 + -31912;
	// 8324B61C: 4BA5E905  bl 0x82ca9f20
	ctx.lr = 0x8324B620;
	sub_82CA9F20(ctx, base);
	// 8324B620: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B624: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B628: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B62C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B630 size=64
    let mut pc: u32 = 0x8324B630;
    'dispatch: loop {
        match pc {
            0x8324B630 => {
    //   block [0x8324B630..0x8324B670)
	// 8324B630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B638: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B63C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8324B640: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B644: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8324B648: 386A7928  addi r3, r10, 0x7928
	ctx.r[3].s64 = ctx.r[10].s64 + 31016;
	// 8324B64C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B650: 4AFE1881  bl 0x8222ced0
	ctx.lr = 0x8324B654;
	sub_8222CED0(ctx, base);
	// 8324B654: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B658: 38698368  addi r3, r9, -0x7c98
	ctx.r[3].s64 = ctx.r[9].s64 + -31896;
	// 8324B65C: 4BA5E8C5  bl 0x82ca9f20
	ctx.lr = 0x8324B660;
	sub_82CA9F20(ctx, base);
	// 8324B660: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B664: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B668: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B66C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B670 size=144
    let mut pc: u32 = 0x8324B670;
    'dispatch: loop {
        match pc {
            0x8324B670 => {
    //   block [0x8324B670..0x8324B694)
	// 8324B670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B678: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B67C: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 8324B680: 4AFD3BD9  bl 0x8221f258
	ctx.lr = 0x8324B684;
	sub_8221F258(ctx, base);
	// 8324B684: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8324B688: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8324B68C: 419A0008  beq cr6, 0x8324b694
	if ctx.cr[6].eq {
	pc = 0x8324B694; continue 'dispatch;
	}
	// 8324B690: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8324B694; continue 'dispatch;
            }
            0x8324B694 => {
    //   block [0x8324B694..0x8324B6A0)
	// 8324B694: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8324B698: 41820008  beq 0x8324b6a0
	if ctx.cr[0].eq {
	pc = 0x8324B6A0; continue 'dispatch;
	}
	// 8324B69C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8324B6A0; continue 'dispatch;
            }
            0x8324B6A0 => {
    //   block [0x8324B6A0..0x8324B6AC)
	// 8324B6A0: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8324B6A4: 41820008  beq 0x8324b6ac
	if ctx.cr[0].eq {
	pc = 0x8324B6AC; continue 'dispatch;
	}
	// 8324B6A8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8324B6AC; continue 'dispatch;
            }
            0x8324B6AC => {
    //   block [0x8324B6AC..0x8324B700)
	// 8324B6AC: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 8324B6B0: 99430051  stb r10, 0x51(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(81 as u32), ctx.r[10].u8 ) };
	// 8324B6B4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8324B6B8: 3909792C  addi r8, r9, 0x792c
	ctx.r[8].s64 = ctx.r[9].s64 + 31020;
	// 8324B6BC: 99630050  stb r11, 0x50(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 8324B6C0: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8324B6C4: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8324B6C8: 99630051  stb r11, 0x51(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(81 as u32), ctx.r[11].u8 ) };
	// 8324B6CC: 38678378  addi r3, r7, -0x7c88
	ctx.r[3].s64 = ctx.r[7].s64 + -31880;
	// 8324B6D0: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8324B6D4: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8324B6D8: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8324B6DC: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8324B6E0: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8324B6E4: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8324B6E8: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8324B6EC: 4BA5E835  bl 0x82ca9f20
	ctx.lr = 0x8324B6F0;
	sub_82CA9F20(ctx, base);
	// 8324B6F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B6F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B6F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B6FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B700 size=64
    let mut pc: u32 = 0x8324B700;
    'dispatch: loop {
        match pc {
            0x8324B700 => {
    //   block [0x8324B700..0x8324B740)
	// 8324B700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B708: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B70C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324B710: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B714: 388B9988  addi r4, r11, -0x6678
	ctx.r[4].s64 = ctx.r[11].s64 + -26232;
	// 8324B718: 386A7938  addi r3, r10, 0x7938
	ctx.r[3].s64 = ctx.r[10].s64 + 31032;
	// 8324B71C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B720: 4AFE17B1  bl 0x8222ced0
	ctx.lr = 0x8324B724;
	sub_8222CED0(ctx, base);
	// 8324B724: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B728: 38698388  addi r3, r9, -0x7c78
	ctx.r[3].s64 = ctx.r[9].s64 + -31864;
	// 8324B72C: 4BA5E7F5  bl 0x82ca9f20
	ctx.lr = 0x8324B730;
	sub_82CA9F20(ctx, base);
	// 8324B730: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B734: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B738: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B73C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B740 size=64
    let mut pc: u32 = 0x8324B740;
    'dispatch: loop {
        match pc {
            0x8324B740 => {
    //   block [0x8324B740..0x8324B780)
	// 8324B740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B748: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B74C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324B750: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B754: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8324B758: 386A793C  addi r3, r10, 0x793c
	ctx.r[3].s64 = ctx.r[10].s64 + 31036;
	// 8324B75C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B760: 4AFE1771  bl 0x8222ced0
	ctx.lr = 0x8324B764;
	sub_8222CED0(ctx, base);
	// 8324B764: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B768: 38698398  addi r3, r9, -0x7c68
	ctx.r[3].s64 = ctx.r[9].s64 + -31848;
	// 8324B76C: 4BA5E7B5  bl 0x82ca9f20
	ctx.lr = 0x8324B770;
	sub_82CA9F20(ctx, base);
	// 8324B770: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B774: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B778: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B77C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B780 size=64
    let mut pc: u32 = 0x8324B780;
    'dispatch: loop {
        match pc {
            0x8324B780 => {
    //   block [0x8324B780..0x8324B7C0)
	// 8324B780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B788: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B78C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324B790: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B794: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8324B798: 386A7940  addi r3, r10, 0x7940
	ctx.r[3].s64 = ctx.r[10].s64 + 31040;
	// 8324B79C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B7A0: 4AFE1731  bl 0x8222ced0
	ctx.lr = 0x8324B7A4;
	sub_8222CED0(ctx, base);
	// 8324B7A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B7A8: 386983A8  addi r3, r9, -0x7c58
	ctx.r[3].s64 = ctx.r[9].s64 + -31832;
	// 8324B7AC: 4BA5E775  bl 0x82ca9f20
	ctx.lr = 0x8324B7B0;
	sub_82CA9F20(ctx, base);
	// 8324B7B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B7B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B7B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B7BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B7C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B7C0 size=64
    let mut pc: u32 = 0x8324B7C0;
    'dispatch: loop {
        match pc {
            0x8324B7C0 => {
    //   block [0x8324B7C0..0x8324B800)
	// 8324B7C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B7C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B7C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B7CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324B7D0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B7D4: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8324B7D8: 386A7944  addi r3, r10, 0x7944
	ctx.r[3].s64 = ctx.r[10].s64 + 31044;
	// 8324B7DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B7E0: 4AFE16F1  bl 0x8222ced0
	ctx.lr = 0x8324B7E4;
	sub_8222CED0(ctx, base);
	// 8324B7E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B7E8: 386983B8  addi r3, r9, -0x7c48
	ctx.r[3].s64 = ctx.r[9].s64 + -31816;
	// 8324B7EC: 4BA5E735  bl 0x82ca9f20
	ctx.lr = 0x8324B7F0;
	sub_82CA9F20(ctx, base);
	// 8324B7F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B7F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B7F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B7FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B800 size=64
    let mut pc: u32 = 0x8324B800;
    'dispatch: loop {
        match pc {
            0x8324B800 => {
    //   block [0x8324B800..0x8324B840)
	// 8324B800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B808: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B80C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324B810: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B814: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8324B818: 386A7948  addi r3, r10, 0x7948
	ctx.r[3].s64 = ctx.r[10].s64 + 31048;
	// 8324B81C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B820: 4AFE16B1  bl 0x8222ced0
	ctx.lr = 0x8324B824;
	sub_8222CED0(ctx, base);
	// 8324B824: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B828: 386983C8  addi r3, r9, -0x7c38
	ctx.r[3].s64 = ctx.r[9].s64 + -31800;
	// 8324B82C: 4BA5E6F5  bl 0x82ca9f20
	ctx.lr = 0x8324B830;
	sub_82CA9F20(ctx, base);
	// 8324B830: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B834: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B838: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B83C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B840 size=64
    let mut pc: u32 = 0x8324B840;
    'dispatch: loop {
        match pc {
            0x8324B840 => {
    //   block [0x8324B840..0x8324B880)
	// 8324B840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B848: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B84C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324B850: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B854: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8324B858: 386A794C  addi r3, r10, 0x794c
	ctx.r[3].s64 = ctx.r[10].s64 + 31052;
	// 8324B85C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B860: 4AFE1671  bl 0x8222ced0
	ctx.lr = 0x8324B864;
	sub_8222CED0(ctx, base);
	// 8324B864: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B868: 386983D8  addi r3, r9, -0x7c28
	ctx.r[3].s64 = ctx.r[9].s64 + -31784;
	// 8324B86C: 4BA5E6B5  bl 0x82ca9f20
	ctx.lr = 0x8324B870;
	sub_82CA9F20(ctx, base);
	// 8324B870: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B874: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B878: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B87C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B880 size=64
    let mut pc: u32 = 0x8324B880;
    'dispatch: loop {
        match pc {
            0x8324B880 => {
    //   block [0x8324B880..0x8324B8C0)
	// 8324B880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B888: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B88C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324B890: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B894: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8324B898: 386A7950  addi r3, r10, 0x7950
	ctx.r[3].s64 = ctx.r[10].s64 + 31056;
	// 8324B89C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B8A0: 4AFE1631  bl 0x8222ced0
	ctx.lr = 0x8324B8A4;
	sub_8222CED0(ctx, base);
	// 8324B8A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B8A8: 386983E8  addi r3, r9, -0x7c18
	ctx.r[3].s64 = ctx.r[9].s64 + -31768;
	// 8324B8AC: 4BA5E675  bl 0x82ca9f20
	ctx.lr = 0x8324B8B0;
	sub_82CA9F20(ctx, base);
	// 8324B8B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B8B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B8B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B8BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B8C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B8C0 size=64
    let mut pc: u32 = 0x8324B8C0;
    'dispatch: loop {
        match pc {
            0x8324B8C0 => {
    //   block [0x8324B8C0..0x8324B900)
	// 8324B8C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B8C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B8C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B8CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324B8D0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B8D4: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8324B8D8: 386A7954  addi r3, r10, 0x7954
	ctx.r[3].s64 = ctx.r[10].s64 + 31060;
	// 8324B8DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B8E0: 4AFE15F1  bl 0x8222ced0
	ctx.lr = 0x8324B8E4;
	sub_8222CED0(ctx, base);
	// 8324B8E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B8E8: 386983F8  addi r3, r9, -0x7c08
	ctx.r[3].s64 = ctx.r[9].s64 + -31752;
	// 8324B8EC: 4BA5E635  bl 0x82ca9f20
	ctx.lr = 0x8324B8F0;
	sub_82CA9F20(ctx, base);
	// 8324B8F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B8F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B8F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B8FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B900 size=64
    let mut pc: u32 = 0x8324B900;
    'dispatch: loop {
        match pc {
            0x8324B900 => {
    //   block [0x8324B900..0x8324B940)
	// 8324B900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B908: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B90C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324B910: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B914: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8324B918: 386A7958  addi r3, r10, 0x7958
	ctx.r[3].s64 = ctx.r[10].s64 + 31064;
	// 8324B91C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B920: 4AFE15B1  bl 0x8222ced0
	ctx.lr = 0x8324B924;
	sub_8222CED0(ctx, base);
	// 8324B924: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B928: 38698408  addi r3, r9, -0x7bf8
	ctx.r[3].s64 = ctx.r[9].s64 + -31736;
	// 8324B92C: 4BA5E5F5  bl 0x82ca9f20
	ctx.lr = 0x8324B930;
	sub_82CA9F20(ctx, base);
	// 8324B930: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B934: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B938: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B93C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B940 size=64
    let mut pc: u32 = 0x8324B940;
    'dispatch: loop {
        match pc {
            0x8324B940 => {
    //   block [0x8324B940..0x8324B980)
	// 8324B940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B948: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B94C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324B950: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B954: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8324B958: 386A795C  addi r3, r10, 0x795c
	ctx.r[3].s64 = ctx.r[10].s64 + 31068;
	// 8324B95C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B960: 4AFE1571  bl 0x8222ced0
	ctx.lr = 0x8324B964;
	sub_8222CED0(ctx, base);
	// 8324B964: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B968: 38698418  addi r3, r9, -0x7be8
	ctx.r[3].s64 = ctx.r[9].s64 + -31720;
	// 8324B96C: 4BA5E5B5  bl 0x82ca9f20
	ctx.lr = 0x8324B970;
	sub_82CA9F20(ctx, base);
	// 8324B970: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B974: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B978: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B97C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B980 size=56
    let mut pc: u32 = 0x8324B980;
    'dispatch: loop {
        match pc {
            0x8324B980 => {
    //   block [0x8324B980..0x8324B9B8)
	// 8324B980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B988: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B98C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324B990: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324B994: 386B9D1C  addi r3, r11, -0x62e4
	ctx.r[3].s64 = ctx.r[11].s64 + -25316;
	// 8324B998: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324B99C: 4AFA83BD  bl 0x821f3d58
	ctx.lr = 0x8324B9A0;
	sub_821F3D58(ctx, base);
	// 8324B9A0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B9A4: 906A7960  stw r3, 0x7960(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31072 as u32), ctx.r[3].u32 ) };
	// 8324B9A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B9AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B9B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B9B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B9B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B9B8 size=56
    let mut pc: u32 = 0x8324B9B8;
    'dispatch: loop {
        match pc {
            0x8324B9B8 => {
    //   block [0x8324B9B8..0x8324B9F0)
	// 8324B9B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B9BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B9C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B9C4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324B9C8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324B9CC: 386B9D2C  addi r3, r11, -0x62d4
	ctx.r[3].s64 = ctx.r[11].s64 + -25300;
	// 8324B9D0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324B9D4: 4AFA8385  bl 0x821f3d58
	ctx.lr = 0x8324B9D8;
	sub_821F3D58(ctx, base);
	// 8324B9D8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B9DC: 906A7964  stw r3, 0x7964(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31076 as u32), ctx.r[3].u32 ) };
	// 8324B9E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B9E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B9E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B9EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B9F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B9F0 size=96
    let mut pc: u32 = 0x8324B9F0;
    'dispatch: loop {
        match pc {
            0x8324B9F0 => {
    //   block [0x8324B9F0..0x8324BA14)
	// 8324B9F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B9F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B9F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B9FC: 38600080  li r3, 0x80
	ctx.r[3].s64 = 128;
	// 8324BA00: 4AFD3859  bl 0x8221f258
	ctx.lr = 0x8324BA04;
	sub_8221F258(ctx, base);
	// 8324BA04: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8324BA08: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8324BA0C: 419A0008  beq cr6, 0x8324ba14
	if ctx.cr[6].eq {
	pc = 0x8324BA14; continue 'dispatch;
	}
	// 8324BA10: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x8324BA14; continue 'dispatch;
            }
            0x8324BA14 => {
    //   block [0x8324BA14..0x8324BA20)
	// 8324BA14: 354B0004  addic. r10, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8324BA18: 41820008  beq 0x8324ba20
	if ctx.cr[0].eq {
	pc = 0x8324BA20; continue 'dispatch;
	}
	// 8324BA1C: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x8324BA20; continue 'dispatch;
            }
            0x8324BA20 => {
    //   block [0x8324BA20..0x8324BA50)
	// 8324BA20: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 8324BA24: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8324BA28: 39097968  addi r8, r9, 0x7968
	ctx.r[8].s64 = ctx.r[9].s64 + 31080;
	// 8324BA2C: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8324BA30: 38678428  addi r3, r7, -0x7bd8
	ctx.r[3].s64 = ctx.r[7].s64 + -31704;
	// 8324BA34: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8324BA38: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8324BA3C: 4BA5E4E5  bl 0x82ca9f20
	ctx.lr = 0x8324BA40;
	sub_82CA9F20(ctx, base);
	// 8324BA40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324BA44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324BA48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324BA4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BA50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324BA50 size=96
    let mut pc: u32 = 0x8324BA50;
    'dispatch: loop {
        match pc {
            0x8324BA50 => {
    //   block [0x8324BA50..0x8324BA74)
	// 8324BA50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324BA54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324BA58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324BA5C: 38600080  li r3, 0x80
	ctx.r[3].s64 = 128;
	// 8324BA60: 4AFD37F9  bl 0x8221f258
	ctx.lr = 0x8324BA64;
	sub_8221F258(ctx, base);
	// 8324BA64: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8324BA68: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8324BA6C: 419A0008  beq cr6, 0x8324ba74
	if ctx.cr[6].eq {
	pc = 0x8324BA74; continue 'dispatch;
	}
	// 8324BA70: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x8324BA74; continue 'dispatch;
            }
            0x8324BA74 => {
    //   block [0x8324BA74..0x8324BA80)
	// 8324BA74: 354B0004  addic. r10, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8324BA78: 41820008  beq 0x8324ba80
	if ctx.cr[0].eq {
	pc = 0x8324BA80; continue 'dispatch;
	}
	// 8324BA7C: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x8324BA80; continue 'dispatch;
            }
            0x8324BA80 => {
    //   block [0x8324BA80..0x8324BAB0)
	// 8324BA80: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 8324BA84: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8324BA88: 39097974  addi r8, r9, 0x7974
	ctx.r[8].s64 = ctx.r[9].s64 + 31092;
	// 8324BA8C: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8324BA90: 38678438  addi r3, r7, -0x7bc8
	ctx.r[3].s64 = ctx.r[7].s64 + -31688;
	// 8324BA94: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8324BA98: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8324BA9C: 4BA5E485  bl 0x82ca9f20
	ctx.lr = 0x8324BAA0;
	sub_82CA9F20(ctx, base);
	// 8324BAA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324BAA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324BAA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324BAAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BAB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324BAB0 size=64
    let mut pc: u32 = 0x8324BAB0;
    'dispatch: loop {
        match pc {
            0x8324BAB0 => {
    //   block [0x8324BAB0..0x8324BAF0)
	// 8324BAB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324BAB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324BAB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324BABC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324BAC0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324BAC4: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 8324BAC8: 386A7980  addi r3, r10, 0x7980
	ctx.r[3].s64 = ctx.r[10].s64 + 31104;
	// 8324BACC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324BAD0: 4AFE1401  bl 0x8222ced0
	ctx.lr = 0x8324BAD4;
	sub_8222CED0(ctx, base);
	// 8324BAD4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324BAD8: 38698448  addi r3, r9, -0x7bb8
	ctx.r[3].s64 = ctx.r[9].s64 + -31672;
	// 8324BADC: 4BA5E445  bl 0x82ca9f20
	ctx.lr = 0x8324BAE0;
	sub_82CA9F20(ctx, base);
	// 8324BAE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324BAE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324BAE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324BAEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324BAF0 size=64
    let mut pc: u32 = 0x8324BAF0;
    'dispatch: loop {
        match pc {
            0x8324BAF0 => {
    //   block [0x8324BAF0..0x8324BB30)
	// 8324BAF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324BAF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324BAF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324BAFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324BB00: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324BB04: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8324BB08: 386A7984  addi r3, r10, 0x7984
	ctx.r[3].s64 = ctx.r[10].s64 + 31108;
	// 8324BB0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324BB10: 4AFE13C1  bl 0x8222ced0
	ctx.lr = 0x8324BB14;
	sub_8222CED0(ctx, base);
	// 8324BB14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324BB18: 38698458  addi r3, r9, -0x7ba8
	ctx.r[3].s64 = ctx.r[9].s64 + -31656;
	// 8324BB1C: 4BA5E405  bl 0x82ca9f20
	ctx.lr = 0x8324BB20;
	sub_82CA9F20(ctx, base);
	// 8324BB20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324BB24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324BB28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324BB2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BB30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324BB30 size=64
    let mut pc: u32 = 0x8324BB30;
    'dispatch: loop {
        match pc {
            0x8324BB30 => {
    //   block [0x8324BB30..0x8324BB70)
	// 8324BB30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324BB34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324BB38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324BB3C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324BB40: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324BB44: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8324BB48: 386A7988  addi r3, r10, 0x7988
	ctx.r[3].s64 = ctx.r[10].s64 + 31112;
	// 8324BB4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324BB50: 4AFE1381  bl 0x8222ced0
	ctx.lr = 0x8324BB54;
	sub_8222CED0(ctx, base);
	// 8324BB54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324BB58: 38698468  addi r3, r9, -0x7b98
	ctx.r[3].s64 = ctx.r[9].s64 + -31640;
	// 8324BB5C: 4BA5E3C5  bl 0x82ca9f20
	ctx.lr = 0x8324BB60;
	sub_82CA9F20(ctx, base);
	// 8324BB60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324BB64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324BB68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324BB6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BB70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324BB70 size=64
    let mut pc: u32 = 0x8324BB70;
    'dispatch: loop {
        match pc {
            0x8324BB70 => {
    //   block [0x8324BB70..0x8324BBB0)
	// 8324BB70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324BB74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324BB78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324BB7C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324BB80: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324BB84: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8324BB88: 386A798C  addi r3, r10, 0x798c
	ctx.r[3].s64 = ctx.r[10].s64 + 31116;
	// 8324BB8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324BB90: 4AFE1341  bl 0x8222ced0
	ctx.lr = 0x8324BB94;
	sub_8222CED0(ctx, base);
	// 8324BB94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324BB98: 38698478  addi r3, r9, -0x7b88
	ctx.r[3].s64 = ctx.r[9].s64 + -31624;
	// 8324BB9C: 4BA5E385  bl 0x82ca9f20
	ctx.lr = 0x8324BBA0;
	sub_82CA9F20(ctx, base);
	// 8324BBA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324BBA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324BBA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324BBAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BBB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324BBB0 size=64
    let mut pc: u32 = 0x8324BBB0;
    'dispatch: loop {
        match pc {
            0x8324BBB0 => {
    //   block [0x8324BBB0..0x8324BBF0)
	// 8324BBB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324BBB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324BBB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324BBBC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324BBC0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324BBC4: 388B9FE4  addi r4, r11, -0x601c
	ctx.r[4].s64 = ctx.r[11].s64 + -24604;
	// 8324BBC8: 386A7990  addi r3, r10, 0x7990
	ctx.r[3].s64 = ctx.r[10].s64 + 31120;
	// 8324BBCC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324BBD0: 4AFE1301  bl 0x8222ced0
	ctx.lr = 0x8324BBD4;
	sub_8222CED0(ctx, base);
	// 8324BBD4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324BBD8: 38698488  addi r3, r9, -0x7b78
	ctx.r[3].s64 = ctx.r[9].s64 + -31608;
	// 8324BBDC: 4BA5E345  bl 0x82ca9f20
	ctx.lr = 0x8324BBE0;
	sub_82CA9F20(ctx, base);
	// 8324BBE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324BBE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324BBE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324BBEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BBF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324BBF0 size=56
    let mut pc: u32 = 0x8324BBF0;
    'dispatch: loop {
        match pc {
            0x8324BBF0 => {
    //   block [0x8324BBF0..0x8324BC28)
	// 8324BBF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324BBF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324BBF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324BBFC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324BC00: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324BC04: 386B9FF0  addi r3, r11, -0x6010
	ctx.r[3].s64 = ctx.r[11].s64 + -24592;
	// 8324BC08: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324BC0C: 4AFA814D  bl 0x821f3d58
	ctx.lr = 0x8324BC10;
	sub_821F3D58(ctx, base);
	// 8324BC10: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324BC14: 906A7994  stw r3, 0x7994(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31124 as u32), ctx.r[3].u32 ) };
	// 8324BC18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324BC1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324BC20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324BC24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BC28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324BC28 size=56
    let mut pc: u32 = 0x8324BC28;
    'dispatch: loop {
        match pc {
            0x8324BC28 => {
    //   block [0x8324BC28..0x8324BC60)
	// 8324BC28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324BC2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324BC30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324BC34: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324BC38: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324BC3C: 386BA01C  addi r3, r11, -0x5fe4
	ctx.r[3].s64 = ctx.r[11].s64 + -24548;
	// 8324BC40: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324BC44: 4AFA8115  bl 0x821f3d58
	ctx.lr = 0x8324BC48;
	sub_821F3D58(ctx, base);
	// 8324BC48: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324BC4C: 906A7998  stw r3, 0x7998(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31128 as u32), ctx.r[3].u32 ) };
	// 8324BC50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324BC54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324BC58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324BC5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BC60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324BC60 size=56
    let mut pc: u32 = 0x8324BC60;
    'dispatch: loop {
        match pc {
            0x8324BC60 => {
    //   block [0x8324BC60..0x8324BC98)
	// 8324BC60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324BC64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324BC68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324BC6C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324BC70: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324BC74: 386BA048  addi r3, r11, -0x5fb8
	ctx.r[3].s64 = ctx.r[11].s64 + -24504;
	// 8324BC78: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324BC7C: 4AFA80DD  bl 0x821f3d58
	ctx.lr = 0x8324BC80;
	sub_821F3D58(ctx, base);
	// 8324BC80: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324BC84: 906A799C  stw r3, 0x799c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31132 as u32), ctx.r[3].u32 ) };
	// 8324BC88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324BC8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324BC90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324BC94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BC98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324BC98 size=56
    let mut pc: u32 = 0x8324BC98;
    'dispatch: loop {
        match pc {
            0x8324BC98 => {
    //   block [0x8324BC98..0x8324BCD0)
	// 8324BC98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324BC9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324BCA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324BCA4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324BCA8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324BCAC: 386BA074  addi r3, r11, -0x5f8c
	ctx.r[3].s64 = ctx.r[11].s64 + -24460;
	// 8324BCB0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324BCB4: 4AFA80A5  bl 0x821f3d58
	ctx.lr = 0x8324BCB8;
	sub_821F3D58(ctx, base);
	// 8324BCB8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324BCBC: 906A79A0  stw r3, 0x79a0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31136 as u32), ctx.r[3].u32 ) };
	// 8324BCC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324BCC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324BCC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324BCCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BCD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324BCD0 size=56
    let mut pc: u32 = 0x8324BCD0;
    'dispatch: loop {
        match pc {
            0x8324BCD0 => {
    //   block [0x8324BCD0..0x8324BD08)
	// 8324BCD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324BCD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324BCD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324BCDC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324BCE0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324BCE4: 386BA0A0  addi r3, r11, -0x5f60
	ctx.r[3].s64 = ctx.r[11].s64 + -24416;
	// 8324BCE8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324BCEC: 4AFA806D  bl 0x821f3d58
	ctx.lr = 0x8324BCF0;
	sub_821F3D58(ctx, base);
	// 8324BCF0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324BCF4: 906A79A4  stw r3, 0x79a4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31140 as u32), ctx.r[3].u32 ) };
	// 8324BCF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324BCFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324BD00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324BD04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BD08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8324BD08 size=12
    let mut pc: u32 = 0x8324BD08;
    'dispatch: loop {
        match pc {
            0x8324BD08 => {
    //   block [0x8324BD08..0x8324BD14)
	// 8324BD08: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8324BD0C: 386B8498  addi r3, r11, -0x7b68
	ctx.r[3].s64 = ctx.r[11].s64 + -31592;
	// 8324BD10: 4BA5E210  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BD18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324BD18 size=64
    let mut pc: u32 = 0x8324BD18;
    'dispatch: loop {
        match pc {
            0x8324BD18 => {
    //   block [0x8324BD18..0x8324BD58)
	// 8324BD18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324BD1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324BD20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324BD24: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324BD28: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324BD2C: 388BAA40  addi r4, r11, -0x55c0
	ctx.r[4].s64 = ctx.r[11].s64 + -21952;
	// 8324BD30: 386A79B0  addi r3, r10, 0x79b0
	ctx.r[3].s64 = ctx.r[10].s64 + 31152;
	// 8324BD34: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324BD38: 4AFE1199  bl 0x8222ced0
	ctx.lr = 0x8324BD3C;
	sub_8222CED0(ctx, base);
	// 8324BD3C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324BD40: 38698510  addi r3, r9, -0x7af0
	ctx.r[3].s64 = ctx.r[9].s64 + -31472;
	// 8324BD44: 4BA5E1DD  bl 0x82ca9f20
	ctx.lr = 0x8324BD48;
	sub_82CA9F20(ctx, base);
	// 8324BD48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324BD4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324BD50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324BD54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BD58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324BD58 size=64
    let mut pc: u32 = 0x8324BD58;
    'dispatch: loop {
        match pc {
            0x8324BD58 => {
    //   block [0x8324BD58..0x8324BD98)
	// 8324BD58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324BD5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324BD60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324BD64: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324BD68: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324BD6C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8324BD70: 386A79B4  addi r3, r10, 0x79b4
	ctx.r[3].s64 = ctx.r[10].s64 + 31156;
	// 8324BD74: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324BD78: 4AFE1159  bl 0x8222ced0
	ctx.lr = 0x8324BD7C;
	sub_8222CED0(ctx, base);
	// 8324BD7C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324BD80: 38698520  addi r3, r9, -0x7ae0
	ctx.r[3].s64 = ctx.r[9].s64 + -31456;
	// 8324BD84: 4BA5E19D  bl 0x82ca9f20
	ctx.lr = 0x8324BD88;
	sub_82CA9F20(ctx, base);
	// 8324BD88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324BD8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324BD90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324BD94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BD98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324BD98 size=64
    let mut pc: u32 = 0x8324BD98;
    'dispatch: loop {
        match pc {
            0x8324BD98 => {
    //   block [0x8324BD98..0x8324BDD8)
	// 8324BD98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324BD9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324BDA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324BDA4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324BDA8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324BDAC: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8324BDB0: 386A79B8  addi r3, r10, 0x79b8
	ctx.r[3].s64 = ctx.r[10].s64 + 31160;
	// 8324BDB4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324BDB8: 4AFE1119  bl 0x8222ced0
	ctx.lr = 0x8324BDBC;
	sub_8222CED0(ctx, base);
	// 8324BDBC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324BDC0: 38698530  addi r3, r9, -0x7ad0
	ctx.r[3].s64 = ctx.r[9].s64 + -31440;
	// 8324BDC4: 4BA5E15D  bl 0x82ca9f20
	ctx.lr = 0x8324BDC8;
	sub_82CA9F20(ctx, base);
	// 8324BDC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324BDCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324BDD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324BDD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BDD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324BDD8 size=64
    let mut pc: u32 = 0x8324BDD8;
    'dispatch: loop {
        match pc {
            0x8324BDD8 => {
    //   block [0x8324BDD8..0x8324BE18)
	// 8324BDD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324BDDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324BDE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324BDE4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324BDE8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324BDEC: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8324BDF0: 386A79BC  addi r3, r10, 0x79bc
	ctx.r[3].s64 = ctx.r[10].s64 + 31164;
	// 8324BDF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324BDF8: 4AFE10D9  bl 0x8222ced0
	ctx.lr = 0x8324BDFC;
	sub_8222CED0(ctx, base);
	// 8324BDFC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324BE00: 38698540  addi r3, r9, -0x7ac0
	ctx.r[3].s64 = ctx.r[9].s64 + -31424;
	// 8324BE04: 4BA5E11D  bl 0x82ca9f20
	ctx.lr = 0x8324BE08;
	sub_82CA9F20(ctx, base);
	// 8324BE08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324BE0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324BE10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324BE14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BE18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324BE18 size=64
    let mut pc: u32 = 0x8324BE18;
    'dispatch: loop {
        match pc {
            0x8324BE18 => {
    //   block [0x8324BE18..0x8324BE58)
	// 8324BE18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324BE1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324BE20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324BE24: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324BE28: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324BE2C: 388BABA4  addi r4, r11, -0x545c
	ctx.r[4].s64 = ctx.r[11].s64 + -21596;
	// 8324BE30: 386A79C0  addi r3, r10, 0x79c0
	ctx.r[3].s64 = ctx.r[10].s64 + 31168;
	// 8324BE34: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324BE38: 4AFE1099  bl 0x8222ced0
	ctx.lr = 0x8324BE3C;
	sub_8222CED0(ctx, base);
	// 8324BE3C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324BE40: 38698550  addi r3, r9, -0x7ab0
	ctx.r[3].s64 = ctx.r[9].s64 + -31408;
	// 8324BE44: 4BA5E0DD  bl 0x82ca9f20
	ctx.lr = 0x8324BE48;
	sub_82CA9F20(ctx, base);
	// 8324BE48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324BE4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324BE50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324BE54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BE58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324BE58 size=96
    let mut pc: u32 = 0x8324BE58;
    'dispatch: loop {
        match pc {
            0x8324BE58 => {
    //   block [0x8324BE58..0x8324BE7C)
	// 8324BE58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324BE5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324BE60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324BE64: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 8324BE68: 4AFD33F1  bl 0x8221f258
	ctx.lr = 0x8324BE6C;
	sub_8221F258(ctx, base);
	// 8324BE6C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8324BE70: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8324BE74: 419A0008  beq cr6, 0x8324be7c
	if ctx.cr[6].eq {
	pc = 0x8324BE7C; continue 'dispatch;
	}
	// 8324BE78: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x8324BE7C; continue 'dispatch;
            }
            0x8324BE7C => {
    //   block [0x8324BE7C..0x8324BE88)
	// 8324BE7C: 354B0004  addic. r10, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8324BE80: 41820008  beq 0x8324be88
	if ctx.cr[0].eq {
	pc = 0x8324BE88; continue 'dispatch;
	}
	// 8324BE84: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x8324BE88; continue 'dispatch;
            }
            0x8324BE88 => {
    //   block [0x8324BE88..0x8324BEB8)
	// 8324BE88: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 8324BE8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8324BE90: 390979C4  addi r8, r9, 0x79c4
	ctx.r[8].s64 = ctx.r[9].s64 + 31172;
	// 8324BE94: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8324BE98: 38678560  addi r3, r7, -0x7aa0
	ctx.r[3].s64 = ctx.r[7].s64 + -31392;
	// 8324BE9C: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8324BEA0: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8324BEA4: 4BA5E07D  bl 0x82ca9f20
	ctx.lr = 0x8324BEA8;
	sub_82CA9F20(ctx, base);
	// 8324BEA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324BEAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324BEB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324BEB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BEB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8324BEB8 size=12
    let mut pc: u32 = 0x8324BEB8;
    'dispatch: loop {
        match pc {
            0x8324BEB8 => {
    //   block [0x8324BEB8..0x8324BEC4)
	// 8324BEB8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8324BEBC: 386B85A8  addi r3, r11, -0x7a58
	ctx.r[3].s64 = ctx.r[11].s64 + -31320;
	// 8324BEC0: 4BA5E060  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BEC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8324BEC8 size=48
    let mut pc: u32 = 0x8324BEC8;
    'dispatch: loop {
        match pc {
            0x8324BEC8 => {
    //   block [0x8324BEC8..0x8324BED0)
	// 8324BEC8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 8324BECC: 390B7088  addi r8, r11, 0x7088
	ctx.r[8].s64 = ctx.r[11].s64 + 28808;
	pc = 0x8324BED0; continue 'dispatch;
            }
            0x8324BED0 => {
    //   block [0x8324BED0..0x8324BEF8)
	// 8324BED0: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8324BED4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324BED8: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8324BEDC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8324BEE0: 7D40412D  stwcx. r10, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324BEE4: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324BEE8: 4082FFE8  bne 0x8324bed0
	if !ctx.cr[0].eq {
	pc = 0x8324BED0; continue 'dispatch;
	}
	// 8324BEEC: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8324BEF0: 38678600  addi r3, r7, -0x7a00
	ctx.r[3].s64 = ctx.r[7].s64 + -31232;
	// 8324BEF4: 4BA5E02C  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8324BEF8 size=48
    let mut pc: u32 = 0x8324BEF8;
    'dispatch: loop {
        match pc {
            0x8324BEF8 => {
    //   block [0x8324BEF8..0x8324BF00)
	// 8324BEF8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 8324BEFC: 390B7088  addi r8, r11, 0x7088
	ctx.r[8].s64 = ctx.r[11].s64 + 28808;
	pc = 0x8324BF00; continue 'dispatch;
            }
            0x8324BF00 => {
    //   block [0x8324BF00..0x8324BF28)
	// 8324BF00: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8324BF04: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324BF08: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8324BF0C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8324BF10: 7D40412D  stwcx. r10, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324BF14: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324BF18: 4082FFE8  bne 0x8324bf00
	if !ctx.cr[0].eq {
	pc = 0x8324BF00; continue 'dispatch;
	}
	// 8324BF1C: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8324BF20: 38678610  addi r3, r7, -0x79f0
	ctx.r[3].s64 = ctx.r[7].s64 + -31216;
	// 8324BF24: 4BA5DFFC  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BF28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8324BF28 size=12
    let mut pc: u32 = 0x8324BF28;
    'dispatch: loop {
        match pc {
            0x8324BF28 => {
    //   block [0x8324BF28..0x8324BF34)
	// 8324BF28: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8324BF2C: 386B8620  addi r3, r11, -0x79e0
	ctx.r[3].s64 = ctx.r[11].s64 + -31200;
	// 8324BF30: 4BA5DFF0  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BF38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8324BF38 size=48
    let mut pc: u32 = 0x8324BF38;
    'dispatch: loop {
        match pc {
            0x8324BF38 => {
    //   block [0x8324BF38..0x8324BF40)
	// 8324BF38: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 8324BF3C: 390B7088  addi r8, r11, 0x7088
	ctx.r[8].s64 = ctx.r[11].s64 + 28808;
	pc = 0x8324BF40; continue 'dispatch;
            }
            0x8324BF40 => {
    //   block [0x8324BF40..0x8324BF68)
	// 8324BF40: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8324BF44: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324BF48: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8324BF4C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8324BF50: 7D40412D  stwcx. r10, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324BF54: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324BF58: 4082FFE8  bne 0x8324bf40
	if !ctx.cr[0].eq {
	pc = 0x8324BF40; continue 'dispatch;
	}
	// 8324BF5C: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8324BF60: 38678630  addi r3, r7, -0x79d0
	ctx.r[3].s64 = ctx.r[7].s64 + -31184;
	// 8324BF64: 4BA5DFBC  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BF68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8324BF68 size=48
    let mut pc: u32 = 0x8324BF68;
    'dispatch: loop {
        match pc {
            0x8324BF68 => {
    //   block [0x8324BF68..0x8324BF70)
	// 8324BF68: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 8324BF6C: 390B7088  addi r8, r11, 0x7088
	ctx.r[8].s64 = ctx.r[11].s64 + 28808;
	pc = 0x8324BF70; continue 'dispatch;
            }
            0x8324BF70 => {
    //   block [0x8324BF70..0x8324BF98)
	// 8324BF70: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8324BF74: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324BF78: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8324BF7C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8324BF80: 7D40412D  stwcx. r10, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324BF84: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324BF88: 4082FFE8  bne 0x8324bf70
	if !ctx.cr[0].eq {
	pc = 0x8324BF70; continue 'dispatch;
	}
	// 8324BF8C: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8324BF90: 38678640  addi r3, r7, -0x79c0
	ctx.r[3].s64 = ctx.r[7].s64 + -31168;
	// 8324BF94: 4BA5DF8C  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BF98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324BF98 size=64
    let mut pc: u32 = 0x8324BF98;
    'dispatch: loop {
        match pc {
            0x8324BF98 => {
    //   block [0x8324BF98..0x8324BFD8)
	// 8324BF98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324BF9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324BFA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324BFA4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324BFA8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324BFAC: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8324BFB0: 386A7A10  addi r3, r10, 0x7a10
	ctx.r[3].s64 = ctx.r[10].s64 + 31248;
	// 8324BFB4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324BFB8: 4AFE0F19  bl 0x8222ced0
	ctx.lr = 0x8324BFBC;
	sub_8222CED0(ctx, base);
	// 8324BFBC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324BFC0: 38698650  addi r3, r9, -0x79b0
	ctx.r[3].s64 = ctx.r[9].s64 + -31152;
	// 8324BFC4: 4BA5DF5D  bl 0x82ca9f20
	ctx.lr = 0x8324BFC8;
	sub_82CA9F20(ctx, base);
	// 8324BFC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324BFCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324BFD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324BFD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


