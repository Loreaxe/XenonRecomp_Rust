pub fn sub_82DF26F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF26F0 size=56
    let mut pc: u32 = 0x82DF26F0;
    'dispatch: loop {
        match pc {
            0x82DF26F0 => {
    //   block [0x82DF26F0..0x82DF2728)
	// 82DF26F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF26F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF26F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF26FC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DF2700: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF2704: 4BDDB755  bl 0x82bcde58
	ctx.lr = 0x82DF2708;
	sub_82BCDE58(ctx, base);
	// 82DF2708: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DF270C: 396BFEFD  addi r11, r11, -0x103
	ctx.r[11].s64 = ctx.r[11].s64 + -259;
	// 82DF2710: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82DF2714: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82DF2718: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF271C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF2720: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF2724: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF2728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF2728 size=40
    let mut pc: u32 = 0x82DF2728;
    'dispatch: loop {
        match pc {
            0x82DF2728 => {
    //   block [0x82DF2728..0x82DF2750)
	// 82DF2728: 2B040001  cmplwi cr6, r4, 1
	ctx.cr[6].compare_u32(ctx.r[4].u32, 1 as u32, &mut ctx.xer);
	// 82DF272C: 4198003C  blt cr6, 0x82df2768
	if ctx.cr[6].lt {
		sub_82DF2768(ctx, base);
		return;
	}
	// 82DF2730: 419A0030  beq cr6, 0x82df2760
	if ctx.cr[6].eq {
		sub_82DF2760(ctx, base);
		return;
	}
	// 82DF2734: 2B040003  cmplwi cr6, r4, 3
	ctx.cr[6].compare_u32(ctx.r[4].u32, 3 as u32, &mut ctx.xer);
	// 82DF2738: 41980020  blt cr6, 0x82df2758
	if ctx.cr[6].lt {
		sub_82DF2758(ctx, base);
		return;
	}
	// 82DF273C: 419A0014  beq cr6, 0x82df2750
	if ctx.cr[6].eq {
		sub_82DF2750(ctx, base);
		return;
	}
	// 82DF2740: 2B040005  cmplwi cr6, r4, 5
	ctx.cr[6].compare_u32(ctx.r[4].u32, 5 as u32, &mut ctx.xer);
	// 82DF2744: 40980014  bge cr6, 0x82df2758
	if !ctx.cr[6].lt {
		sub_82DF2758(ctx, base);
		return;
	}
	// 82DF2748: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82DF274C: 48000020  b 0x82df276c
	sub_82DF2768(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF2750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF2750 size=8
    let mut pc: u32 = 0x82DF2750;
    'dispatch: loop {
        match pc {
            0x82DF2750 => {
    //   block [0x82DF2750..0x82DF2758)
	// 82DF2750: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DF2754: 48000018  b 0x82df276c
	sub_82DF2768(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF2758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF2758 size=8
    let mut pc: u32 = 0x82DF2758;
    'dispatch: loop {
        match pc {
            0x82DF2758 => {
    //   block [0x82DF2758..0x82DF2760)
	// 82DF2758: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DF275C: 48000010  b 0x82df276c
	sub_82DF2768(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF2760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF2760 size=8
    let mut pc: u32 = 0x82DF2760;
    'dispatch: loop {
        match pc {
            0x82DF2760 => {
    //   block [0x82DF2760..0x82DF2768)
	// 82DF2760: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 82DF2764: 48000008  b 0x82df276c
	sub_82DF2768(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF2768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF2768 size=12
    let mut pc: u32 = 0x82DF2768;
    'dispatch: loop {
        match pc {
            0x82DF2768 => {
    //   block [0x82DF2768..0x82DF2774)
	// 82DF2768: 3880FFFE  li r4, -2
	ctx.r[4].s64 = -2;
	// 82DF276C: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF2770: 4BDDB5A8  b 0x82bcdd18
	sub_82BCDD18(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF2778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF2778 size=8
    let mut pc: u32 = 0x82DF2778;
    'dispatch: loop {
        match pc {
            0x82DF2778 => {
    //   block [0x82DF2778..0x82DF2780)
	// 82DF2778: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF277C: 4BDDB784  b 0x82bcdf00
	sub_82BCDF00(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF2780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF2780 size=112
    let mut pc: u32 = 0x82DF2780;
    'dispatch: loop {
        match pc {
            0x82DF2780 => {
    //   block [0x82DF2780..0x82DF27F0)
	// 82DF2780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF2784: 483B59E9  bl 0x831a816c
	ctx.lr = 0x82DF2788;
	sub_831A8130(ctx, base);
	// 82DF2788: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF278C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF2790: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF2794: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DF2798: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DF279C: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82DF27A0: 997F0010  stb r11, 0x10(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 82DF27A4: 391F000C  addi r8, r31, 0xc
	ctx.r[8].s64 = ctx.r[31].s64 + 12;
	// 82DF27A8: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82DF27AC: 38E00024  li r7, 0x24
	ctx.r[7].s64 = 36;
	// 82DF27B0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DF27B4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DF27B8: 483D23D1  bl 0x831c4b88
	ctx.lr = 0x82DF27BC;
	sub_831C4B88(ctx, base);
	// 82DF27BC: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82DF27C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF27C4: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DF27C8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DF27CC: 4BFFFE55  bl 0x82df2620
	ctx.lr = 0x82DF27D0;
	sub_82DF2620(ctx, base);
	// 82DF27D0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF27D4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DF27D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF27DC: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DF27E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DF27E4: 4E800421  bctrl
	ctx.lr = 0x82DF27E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DF27E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF27EC: 483B59D0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF27F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF27F0 size=84
    let mut pc: u32 = 0x82DF27F0;
    'dispatch: loop {
        match pc {
            0x82DF27F0 => {
    //   block [0x82DF27F0..0x82DF2844)
	// 82DF27F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF27F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF27F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF27FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF2800: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF2804: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DF2808: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 82DF280C: 396BB214  addi r11, r11, -0x4dec
	ctx.r[11].s64 = ctx.r[11].s64 + -19948;
	// 82DF2810: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF2814: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF2818: 4BDDB4E9  bl 0x82bcdd00
	ctx.lr = 0x82DF281C;
	sub_82BCDD00(ctx, base);
	// 82DF281C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF2820: 4BDDA201  bl 0x82bcca20
	ctx.lr = 0x82DF2824;
	sub_82BCCA20(ctx, base);
	// 82DF2824: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DF2828: 396BB37C  addi r11, r11, -0x4c84
	ctx.r[11].s64 = ctx.r[11].s64 + -19588;
	// 82DF282C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF2830: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF2834: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF2838: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF283C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF2840: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF2848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF2848 size=76
    let mut pc: u32 = 0x82DF2848;
    'dispatch: loop {
        match pc {
            0x82DF2848 => {
    //   block [0x82DF2848..0x82DF2894)
	// 82DF2848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF284C: 483B5921  bl 0x831a816c
	ctx.lr = 0x82DF2850;
	sub_831A8130(ctx, base);
	// 82DF2850: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF2854: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF2858: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DF285C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DF2860: 48007071  bl 0x82df98d0
	ctx.lr = 0x82DF2864;
	sub_82DF98D0(ctx, base);
	// 82DF2864: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DF2868: 3D4082DF  lis r10, -0x7d21
	ctx.r[10].s64 = -2099314688;
	// 82DF286C: 396BB214  addi r11, r11, -0x4dec
	ctx.r[11].s64 = ctx.r[11].s64 + -19948;
	// 82DF2870: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF2874: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF2878: 38CA25F0  addi r6, r10, 0x25f0
	ctx.r[6].s64 = ctx.r[10].s64 + 9712;
	// 82DF287C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DF2880: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DF2884: 4BFFFEFD  bl 0x82df2780
	ctx.lr = 0x82DF2888;
	sub_82DF2780(ctx, base);
	// 82DF2888: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF288C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF2890: 483B592C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF2898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF2898 size=76
    let mut pc: u32 = 0x82DF2898;
    'dispatch: loop {
        match pc {
            0x82DF2898 => {
    //   block [0x82DF2898..0x82DF28E4)
	// 82DF2898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF289C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF28A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DF28A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF28A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF28AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF28B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DF28B4: 4BFFFF3D  bl 0x82df27f0
	ctx.lr = 0x82DF28B8;
	sub_82DF27F0(ctx, base);
	// 82DF28B8: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF28BC: 4182000C  beq 0x82df28c8
	if ctx.cr[0].eq {
	pc = 0x82DF28C8; continue 'dispatch;
	}
	// 82DF28C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF28C4: 4BFFFB15  bl 0x82df23d8
	ctx.lr = 0x82DF28C8;
	sub_82DF23D8(ctx, base);
	// 82DF28C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF28CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF28D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF28D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF28D8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DF28DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF28E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF28E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF28E8 size=76
    let mut pc: u32 = 0x82DF28E8;
    'dispatch: loop {
        match pc {
            0x82DF28E8 => {
    //   block [0x82DF28E8..0x82DF2934)
	// 82DF28E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF28EC: 483B587D  bl 0x831a8168
	ctx.lr = 0x82DF28F0;
	sub_831A8130(ctx, base);
	// 82DF28F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF28F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF28F8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DF28FC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DF2900: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82DF2904: 48006FCD  bl 0x82df98d0
	ctx.lr = 0x82DF2908;
	sub_82DF98D0(ctx, base);
	// 82DF2908: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DF290C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF2910: 396BB214  addi r11, r11, -0x4dec
	ctx.r[11].s64 = ctx.r[11].s64 + -19948;
	// 82DF2914: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DF2918: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF291C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DF2920: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DF2924: 4BFFFE5D  bl 0x82df2780
	ctx.lr = 0x82DF2928;
	sub_82DF2780(ctx, base);
	// 82DF2928: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF292C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DF2930: 483B5888  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF2938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF2938 size=12
    let mut pc: u32 = 0x82DF2938;
    'dispatch: loop {
        match pc {
            0x82DF2938 => {
    //   block [0x82DF2938..0x82DF2944)
	// 82DF2938: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82DF293C: 988B56F5  stb r4, 0x56f5(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(22261 as u32), ctx.r[4].u8 ) };
	// 82DF2940: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF2948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF2948 size=8
    let mut pc: u32 = 0x82DF2948;
    'dispatch: loop {
        match pc {
            0x82DF2948 => {
    //   block [0x82DF2948..0x82DF2950)
	// 82DF2948: 3863085C  addi r3, r3, 0x85c
	ctx.r[3].s64 = ctx.r[3].s64 + 2140;
	// 82DF294C: 48001284  b 0x82df3bd0
	sub_82DF3BD0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF2950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF2950 size=88
    let mut pc: u32 = 0x82DF2950;
    'dispatch: loop {
        match pc {
            0x82DF2950 => {
    //   block [0x82DF2950..0x82DF29A8)
	// 82DF2950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF2954: 483B5815  bl 0x831a8168
	ctx.lr = 0x82DF2958;
	sub_831A8130(ctx, base);
	// 82DF2958: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF295C: 8383001C  lwz r28, 0x1c(r3)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DF2960: 83E30018  lwz r31, 0x18(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DF2964: 7F1FE040  cmplw cr6, r31, r28
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82DF2968: 419A0038  beq cr6, 0x82df29a0
	if ctx.cr[6].eq {
	pc = 0x82DF29A0; continue 'dispatch;
	}
	// 82DF296C: 83AD0000  lwz r29, 0(r13)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF2970: 3BC00010  li r30, 0x10
	ctx.r[30].s64 = 16;
	// 82DF2974: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DF2978: 7C9EE82E  lwzx r4, r30, r29
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82DF297C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF2980: 480080C9  bl 0x82dfaa48
	ctx.lr = 0x82DF2984;
	sub_82DFAA48(ctx, base);
	// 82DF2984: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF2988: 7C9EE82E  lwzx r4, r30, r29
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82DF298C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF2990: 480080B9  bl 0x82dfaa48
	ctx.lr = 0x82DF2994;
	sub_82DFAA48(ctx, base);
	// 82DF2994: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 82DF2998: 7F1FE040  cmplw cr6, r31, r28
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82DF299C: 409AFFD8  bne cr6, 0x82df2974
	if !ctx.cr[6].eq {
	pc = 0x82DF2974; continue 'dispatch;
	}
	// 82DF29A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DF29A4: 483B5814  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF29A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF29A8 size=92
    let mut pc: u32 = 0x82DF29A8;
    'dispatch: loop {
        match pc {
            0x82DF29A8 => {
    //   block [0x82DF29A8..0x82DF2A04)
	// 82DF29A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF29AC: 483B57C1  bl 0x831a816c
	ctx.lr = 0x82DF29B0;
	sub_831A8130(ctx, base);
	// 82DF29B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF29B4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DF29B8: 3BFE001C  addi r31, r30, 0x1c
	ctx.r[31].s64 = ctx.r[30].s64 + 28;
	// 82DF29BC: 807E0020  lwz r3, 0x20(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DF29C0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DF29C4: 419A0008  beq cr6, 0x82df29cc
	if ctx.cr[6].eq {
	pc = 0x82DF29CC; continue 'dispatch;
	}
	// 82DF29C8: 4B4CD8A1  bl 0x822c0268
	ctx.lr = 0x82DF29CC;
	sub_822C0268(ctx, base);
	// 82DF29CC: 3BBE000C  addi r29, r30, 0xc
	ctx.r[29].s64 = ctx.r[30].s64 + 12;
	// 82DF29D0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DF29D4: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82DF29D8: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82DF29DC: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 82DF29E0: 807D0004  lwz r3, 4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF29E4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DF29E8: 419A0008  beq cr6, 0x82df29f0
	if ctx.cr[6].eq {
	pc = 0x82DF29F0; continue 'dispatch;
	}
	// 82DF29EC: 4B4CD87D  bl 0x822c0268
	ctx.lr = 0x82DF29F0;
	sub_822C0268(ctx, base);
	// 82DF29F0: 93DD0004  stw r30, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82DF29F4: 93DD0008  stw r30, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82DF29F8: 93DD000C  stw r30, 0xc(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 82DF29FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF2A00: 483B57BC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF2A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF2A08 size=124
    let mut pc: u32 = 0x82DF2A08;
    'dispatch: loop {
        match pc {
            0x82DF2A08 => {
    //   block [0x82DF2A08..0x82DF2A84)
	// 82DF2A08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF2A0C: 483B5761  bl 0x831a816c
	ctx.lr = 0x82DF2A10;
	sub_831A8130(ctx, base);
	// 82DF2A10: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF2A14: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DF2A18: 3BC0000F  li r30, 0xf
	ctx.r[30].s64 = 15;
	// 82DF2A1C: 3BFD03C8  addi r31, r29, 0x3c8
	ctx.r[31].s64 = ctx.r[29].s64 + 968;
	// 82DF2A20: 3BFFFFD0  addi r31, r31, -0x30
	ctx.r[31].s64 = ctx.r[31].s64 + -48;
	// 82DF2A24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF2A28: 4BFFFF81  bl 0x82df29a8
	ctx.lr = 0x82DF2A2C;
	sub_82DF29A8(ctx, base);
	// 82DF2A2C: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82DF2A30: 4080FFF0  bge 0x82df2a20
	if !ctx.cr[0].lt {
	pc = 0x82DF2A20; continue 'dispatch;
	}
	// 82DF2A34: 397D00A0  addi r11, r29, 0xa0
	ctx.r[11].s64 = ctx.r[29].s64 + 160;
	// 82DF2A38: 3BC00007  li r30, 7
	ctx.r[30].s64 = 7;
	// 82DF2A3C: 3BEB0004  addi r31, r11, 4
	ctx.r[31].s64 = ctx.r[11].s64 + 4;
	// 82DF2A40: 3BFFFFF8  addi r31, r31, -8
	ctx.r[31].s64 = ctx.r[31].s64 + -8;
	// 82DF2A44: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF2A48: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DF2A4C: 419A0008  beq cr6, 0x82df2a54
	if ctx.cr[6].eq {
	pc = 0x82DF2A54; continue 'dispatch;
	}
	// 82DF2A50: 4B4CDE41  bl 0x822c0890
	ctx.lr = 0x82DF2A54;
	sub_822C0890(ctx, base);
	// 82DF2A54: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82DF2A58: 4080FFE8  bge 0x82df2a40
	if !ctx.cr[0].lt {
	pc = 0x82DF2A40; continue 'dispatch;
	}
	// 82DF2A5C: 807D005C  lwz r3, 0x5c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(92 as u32) ) } as u64;
	// 82DF2A60: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DF2A64: 419A0008  beq cr6, 0x82df2a6c
	if ctx.cr[6].eq {
	pc = 0x82DF2A6C; continue 'dispatch;
	}
	// 82DF2A68: 4B4CDE29  bl 0x822c0890
	ctx.lr = 0x82DF2A6C;
	sub_822C0890(ctx, base);
	// 82DF2A6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF2A70: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DF2A74: 387D003C  addi r3, r29, 0x3c
	ctx.r[3].s64 = ctx.r[29].s64 + 60;
	// 82DF2A78: 4B4D63B9  bl 0x822c8e30
	ctx.lr = 0x82DF2A7C;
	sub_822C8E30(ctx, base);
	// 82DF2A7C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF2A80: 483B573C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF2A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF2A88 size=1028
    let mut pc: u32 = 0x82DF2A88;
    'dispatch: loop {
        match pc {
            0x82DF2A88 => {
    //   block [0x82DF2A88..0x82DF2E8C)
	// 82DF2A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF2A8C: 483B56CD  bl 0x831a8158
	ctx.lr = 0x82DF2A90;
	sub_831A8130(ctx, base);
	// 82DF2A90: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF2A94: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82DF2A98: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82DF2A9C: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82DF2AA0: 93E10104  stw r31, 0x104(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(260 as u32), ctx.r[31].u32 ) };
	// 82DF2AA4: 897F0019  lbz r11, 0x19(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(25 as u32) ) } as u64;
	// 82DF2AA8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF2AAC: 419A0048  beq cr6, 0x82df2af4
	if ctx.cr[6].eq {
	pc = 0x82DF2AF4; continue 'dispatch;
	}
	// 82DF2AB0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82DF2AB4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF2AB8: 388B9620  addi r4, r11, -0x69e0
	ctx.r[4].s64 = ctx.r[11].s64 + -27104;
	// 82DF2ABC: 4B4D2E0D  bl 0x822c58c8
	ctx.lr = 0x82DF2AC0;
	sub_822C58C8(ctx, base);
	// 82DF2AC0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DF2AC4: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82DF2AC8: 4B4D73E9  bl 0x822c9eb0
	ctx.lr = 0x82DF2ACC;
	sub_822C9EB0(ctx, base);
	// 82DF2ACC: 4B4D17E5  bl 0x822c42b0
	ctx.lr = 0x82DF2AD0;
	sub_822C42B0(ctx, base);
	// 82DF2AD0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82DF2AD4: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82DF2AD8: 396B9600  addi r11, r11, -0x6a00
	ctx.r[11].s64 = ctx.r[11].s64 + -27136;
	// 82DF2ADC: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82DF2AE0: 4B4D2991  bl 0x822c5470
	ctx.lr = 0x82DF2AE4;
	sub_822C5470(ctx, base);
	// 82DF2AE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF2AE8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DF2AEC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF2AF0: 4B4D21F1  bl 0x822c4ce0
	ctx.lr = 0x82DF2AF4;
	sub_822C4CE0(ctx, base);
	// 82DF2AF4: 38610104  addi r3, r1, 0x104
	ctx.r[3].s64 = ctx.r[1].s64 + 260;
	// 82DF2AF8: 7FFBFB78  mr r27, r31
	ctx.r[27].u64 = ctx.r[31].u64;
	// 82DF2AFC: 4B5AEB8D  bl 0x823a1688
	ctx.lr = 0x82DF2B00;
	sub_823A1688(ctx, base);
	// 82DF2B00: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF2B04: 894B0019  lbz r10, 0x19(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(25 as u32) ) } as u64;
	// 82DF2B08: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF2B0C: 83210104  lwz r25, 0x104(r1)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(260 as u32) ) } as u64;
	// 82DF2B10: 419A000C  beq cr6, 0x82df2b1c
	if ctx.cr[6].eq {
	pc = 0x82DF2B1C; continue 'dispatch;
	}
	// 82DF2B14: 839B0008  lwz r28, 8(r27)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF2B18: 48000028  b 0x82df2b40
	pc = 0x82DF2B40; continue 'dispatch;
	// 82DF2B1C: 815B0008  lwz r10, 8(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF2B20: 894A0019  lbz r10, 0x19(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(25 as u32) ) } as u64;
	// 82DF2B24: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF2B28: 419A000C  beq cr6, 0x82df2b34
	if ctx.cr[6].eq {
	pc = 0x82DF2B34; continue 'dispatch;
	}
	// 82DF2B2C: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 82DF2B30: 48000010  b 0x82df2b40
	pc = 0x82DF2B40; continue 'dispatch;
	// 82DF2B34: 83990008  lwz r28, 8(r25)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF2B38: 7F19D840  cmplw cr6, r25, r27
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82DF2B3C: 409A00DC  bne cr6, 0x82df2c18
	if !ctx.cr[6].eq {
	pc = 0x82DF2C18; continue 'dispatch;
	}
	// 82DF2B40: 897C0019  lbz r11, 0x19(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(25 as u32) ) } as u64;
	// 82DF2B44: 83FB0004  lwz r31, 4(r27)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF2B48: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF2B4C: 409A0008  bne cr6, 0x82df2b54
	if !ctx.cr[6].eq {
	pc = 0x82DF2B54; continue 'dispatch;
	}
	// 82DF2B50: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82DF2B54: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF2B58: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF2B5C: 7F0AD840  cmplw cr6, r10, r27
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82DF2B60: 409A000C  bne cr6, 0x82df2b6c
	if !ctx.cr[6].eq {
	pc = 0x82DF2B6C; continue 'dispatch;
	}
	// 82DF2B64: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82DF2B68: 4800001C  b 0x82df2b84
	pc = 0x82DF2B84; continue 'dispatch;
	// 82DF2B6C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF2B70: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82DF2B74: 409A000C  bne cr6, 0x82df2b80
	if !ctx.cr[6].eq {
	pc = 0x82DF2B80; continue 'dispatch;
	}
	// 82DF2B78: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82DF2B7C: 48000008  b 0x82df2b84
	pc = 0x82DF2B84; continue 'dispatch;
	// 82DF2B80: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82DF2B84: 813A0004  lwz r9, 4(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF2B88: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF2B8C: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82DF2B90: 409A003C  bne cr6, 0x82df2bcc
	if !ctx.cr[6].eq {
	pc = 0x82DF2BCC; continue 'dispatch;
	}
	// 82DF2B94: 897C0019  lbz r11, 0x19(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(25 as u32) ) } as u64;
	// 82DF2B98: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF2B9C: 419A000C  beq cr6, 0x82df2ba8
	if ctx.cr[6].eq {
	pc = 0x82DF2BA8; continue 'dispatch;
	}
	// 82DF2BA0: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 82DF2BA4: 48000024  b 0x82df2bc8
	pc = 0x82DF2BC8; continue 'dispatch;
	// 82DF2BA8: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF2BAC: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 82DF2BB0: 4800000C  b 0x82df2bbc
	pc = 0x82DF2BBC; continue 'dispatch;
	// 82DF2BB4: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82DF2BB8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF2BBC: 890B0019  lbz r8, 0x19(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(25 as u32) ) } as u64;
	// 82DF2BC0: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82DF2BC4: 419AFFF0  beq cr6, 0x82df2bb4
	if ctx.cr[6].eq {
	pc = 0x82DF2BB4; continue 'dispatch;
	}
	// 82DF2BC8: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DF2BCC: 813A0004  lwz r9, 4(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF2BD0: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF2BD4: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82DF2BD8: 409A00D4  bne cr6, 0x82df2cac
	if !ctx.cr[6].eq {
	pc = 0x82DF2CAC; continue 'dispatch;
	}
	// 82DF2BDC: 897C0019  lbz r11, 0x19(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(25 as u32) ) } as u64;
	// 82DF2BE0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF2BE4: 419A000C  beq cr6, 0x82df2bf0
	if ctx.cr[6].eq {
	pc = 0x82DF2BF0; continue 'dispatch;
	}
	// 82DF2BE8: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 82DF2BEC: 48000024  b 0x82df2c10
	pc = 0x82DF2C10; continue 'dispatch;
	// 82DF2BF0: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF2BF4: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 82DF2BF8: 4800000C  b 0x82df2c04
	pc = 0x82DF2C04; continue 'dispatch;
	// 82DF2BFC: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82DF2C00: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF2C04: 890B0019  lbz r8, 0x19(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(25 as u32) ) } as u64;
	// 82DF2C08: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82DF2C0C: 419AFFF0  beq cr6, 0x82df2bfc
	if ctx.cr[6].eq {
	pc = 0x82DF2BFC; continue 'dispatch;
	}
	// 82DF2C10: 91490008  stw r10, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82DF2C14: 48000098  b 0x82df2cac
	pc = 0x82DF2CAC; continue 'dispatch;
	// 82DF2C18: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 82DF2C1C: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF2C20: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF2C24: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF2C28: 7F195840  cmplw cr6, r25, r11
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF2C2C: 409A000C  bne cr6, 0x82df2c38
	if !ctx.cr[6].eq {
	pc = 0x82DF2C38; continue 'dispatch;
	}
	// 82DF2C30: 7F3FCB78  mr r31, r25
	ctx.r[31].u64 = ctx.r[25].u64;
	// 82DF2C34: 4800002C  b 0x82df2c60
	pc = 0x82DF2C60; continue 'dispatch;
	// 82DF2C38: 897C0019  lbz r11, 0x19(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(25 as u32) ) } as u64;
	// 82DF2C3C: 83F90004  lwz r31, 4(r25)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF2C40: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF2C44: 409A0008  bne cr6, 0x82df2c4c
	if !ctx.cr[6].eq {
	pc = 0x82DF2C4C; continue 'dispatch;
	}
	// 82DF2C48: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82DF2C4C: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82DF2C50: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF2C54: 91790008  stw r11, 8(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DF2C58: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF2C5C: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 82DF2C60: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF2C64: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF2C68: 7F0AD840  cmplw cr6, r10, r27
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82DF2C6C: 409A000C  bne cr6, 0x82df2c78
	if !ctx.cr[6].eq {
	pc = 0x82DF2C78; continue 'dispatch;
	}
	// 82DF2C70: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 82DF2C74: 48000020  b 0x82df2c94
	pc = 0x82DF2C94; continue 'dispatch;
	// 82DF2C78: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF2C7C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF2C80: 7F0AD840  cmplw cr6, r10, r27
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82DF2C84: 409A000C  bne cr6, 0x82df2c90
	if !ctx.cr[6].eq {
	pc = 0x82DF2C90; continue 'dispatch;
	}
	// 82DF2C88: 932B0000  stw r25, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 82DF2C8C: 48000008  b 0x82df2c94
	pc = 0x82DF2C94; continue 'dispatch;
	// 82DF2C90: 932B0008  stw r25, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[25].u32 ) };
	// 82DF2C94: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF2C98: 91790004  stw r11, 4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DF2C9C: 897B0018  lbz r11, 0x18(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DF2CA0: 89590018  lbz r10, 0x18(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[25].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DF2CA4: 99790018  stb r11, 0x18(r25)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[25].u32.wrapping_add(24 as u32), ctx.r[11].u8 ) };
	// 82DF2CA8: 995B0018  stb r10, 0x18(r27)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[27].u32.wrapping_add(24 as u32), ctx.r[10].u8 ) };
	// 82DF2CAC: 897B0018  lbz r11, 0x18(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DF2CB0: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82DF2CB4: 409A0198  bne cr6, 0x82df2e4c
	if !ctx.cr[6].eq {
	pc = 0x82DF2E4C; continue 'dispatch;
	}
	// 82DF2CB8: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF2CBC: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82DF2CC0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF2CC4: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF2CC8: 419A0180  beq cr6, 0x82df2e48
	if ctx.cr[6].eq {
	pc = 0x82DF2E48; continue 'dispatch;
	}
	// 82DF2CCC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82DF2CD0: 897C0018  lbz r11, 0x18(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DF2CD4: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82DF2CD8: 409A0170  bne cr6, 0x82df2e48
	if !ctx.cr[6].eq {
	pc = 0x82DF2E48; continue 'dispatch;
	}
	// 82DF2CDC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF2CE0: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF2CE4: 409A00A8  bne cr6, 0x82df2d8c
	if !ctx.cr[6].eq {
	pc = 0x82DF2D8C; continue 'dispatch;
	}
	// 82DF2CE8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF2CEC: 894B0018  lbz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DF2CF0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF2CF4: 409A001C  bne cr6, 0x82df2d10
	if !ctx.cr[6].eq {
	pc = 0x82DF2D10; continue 'dispatch;
	}
	// 82DF2CF8: 9BCB0018  stb r30, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 82DF2CFC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF2D00: 9BBF0018  stb r29, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 82DF2D04: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82DF2D08: 4B651261  bl 0x82443f68
	ctx.lr = 0x82DF2D0C;
	sub_82443F68(ctx, base);
	// 82DF2D0C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF2D10: 894B0019  lbz r10, 0x19(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(25 as u32) ) } as u64;
	// 82DF2D14: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF2D18: 409A00C8  bne cr6, 0x82df2de0
	if !ctx.cr[6].eq {
	pc = 0x82DF2DE0; continue 'dispatch;
	}
	// 82DF2D1C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF2D20: 894A0018  lbz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DF2D24: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82DF2D28: 409A0014  bne cr6, 0x82df2d3c
	if !ctx.cr[6].eq {
	pc = 0x82DF2D3C; continue 'dispatch;
	}
	// 82DF2D2C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF2D30: 894A0018  lbz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DF2D34: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82DF2D38: 419A00A4  beq cr6, 0x82df2ddc
	if ctx.cr[6].eq {
	pc = 0x82DF2DDC; continue 'dispatch;
	}
	// 82DF2D3C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF2D40: 894A0018  lbz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DF2D44: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82DF2D48: 409A0020  bne cr6, 0x82df2d68
	if !ctx.cr[6].eq {
	pc = 0x82DF2D68; continue 'dispatch;
	}
	// 82DF2D4C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF2D50: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DF2D54: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82DF2D58: 9BCA0018  stb r30, 0x18(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 82DF2D5C: 9BAB0018  stb r29, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 82DF2D60: 4B6511A1  bl 0x82443f00
	ctx.lr = 0x82DF2D64;
	sub_82443F00(ctx, base);
	// 82DF2D64: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF2D68: 895F0018  lbz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DF2D6C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF2D70: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82DF2D74: 994B0018  stb r10, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[10].u8 ) };
	// 82DF2D78: 9BDF0018  stb r30, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 82DF2D7C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF2D80: 9BCB0018  stb r30, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 82DF2D84: 4B6511E5  bl 0x82443f68
	ctx.lr = 0x82DF2D88;
	sub_82443F68(ctx, base);
	// 82DF2D88: 480000C0  b 0x82df2e48
	pc = 0x82DF2E48; continue 'dispatch;
	// 82DF2D8C: 894B0018  lbz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DF2D90: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF2D94: 409A001C  bne cr6, 0x82df2db0
	if !ctx.cr[6].eq {
	pc = 0x82DF2DB0; continue 'dispatch;
	}
	// 82DF2D98: 9BCB0018  stb r30, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 82DF2D9C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF2DA0: 9BBF0018  stb r29, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 82DF2DA4: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82DF2DA8: 4B651159  bl 0x82443f00
	ctx.lr = 0x82DF2DAC;
	sub_82443F00(ctx, base);
	// 82DF2DAC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF2DB0: 894B0019  lbz r10, 0x19(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(25 as u32) ) } as u64;
	// 82DF2DB4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF2DB8: 409A0028  bne cr6, 0x82df2de0
	if !ctx.cr[6].eq {
	pc = 0x82DF2DE0; continue 'dispatch;
	}
	// 82DF2DBC: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF2DC0: 894A0018  lbz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DF2DC4: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82DF2DC8: 409A0034  bne cr6, 0x82df2dfc
	if !ctx.cr[6].eq {
	pc = 0x82DF2DFC; continue 'dispatch;
	}
	// 82DF2DCC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF2DD0: 894A0018  lbz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DF2DD4: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82DF2DD8: 409A0024  bne cr6, 0x82df2dfc
	if !ctx.cr[6].eq {
	pc = 0x82DF2DFC; continue 'dispatch;
	}
	// 82DF2DDC: 9BAB0018  stb r29, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 82DF2DE0: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF2DE4: 7FFCFB78  mr r28, r31
	ctx.r[28].u64 = ctx.r[31].u64;
	// 82DF2DE8: 83FF0004  lwz r31, 4(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF2DEC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF2DF0: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF2DF4: 409AFEDC  bne cr6, 0x82df2cd0
	if !ctx.cr[6].eq {
	pc = 0x82DF2CD0; continue 'dispatch;
	}
	// 82DF2DF8: 48000050  b 0x82df2e48
	pc = 0x82DF2E48; continue 'dispatch;
	// 82DF2DFC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF2E00: 894A0018  lbz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DF2E04: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82DF2E08: 409A0020  bne cr6, 0x82df2e28
	if !ctx.cr[6].eq {
	pc = 0x82DF2E28; continue 'dispatch;
	}
	// 82DF2E0C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF2E10: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DF2E14: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82DF2E18: 9BCA0018  stb r30, 0x18(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 82DF2E1C: 9BAB0018  stb r29, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 82DF2E20: 4B651149  bl 0x82443f68
	ctx.lr = 0x82DF2E24;
	sub_82443F68(ctx, base);
	// 82DF2E24: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF2E28: 895F0018  lbz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DF2E2C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF2E30: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82DF2E34: 994B0018  stb r10, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[10].u8 ) };
	// 82DF2E38: 9BDF0018  stb r30, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 82DF2E3C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF2E40: 9BCB0018  stb r30, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 82DF2E44: 4B6510BD  bl 0x82443f00
	ctx.lr = 0x82DF2E48;
	sub_82443F00(ctx, base);
	// 82DF2E48: 9BDC0018  stb r30, 0x18(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 82DF2E4C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DF2E50: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82DF2E54: 480757ED  bl 0x82e68640
	ctx.lr = 0x82DF2E58;
	sub_82E68640(ctx, base);
	// 82DF2E58: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82DF2E5C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82DF2E60: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82DF2E64: 4BFFF325  bl 0x82df2188
	ctx.lr = 0x82DF2E68;
	sub_82DF2188(ctx, base);
	// 82DF2E68: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF2E6C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF2E70: 419A000C  beq cr6, 0x82df2e7c
	if ctx.cr[6].eq {
	pc = 0x82DF2E7C; continue 'dispatch;
	}
	// 82DF2E74: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DF2E78: 917A0008  stw r11, 8(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DF2E7C: 93380000  stw r25, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 82DF2E80: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82DF2E84: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82DF2E88: 483B5320  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF2E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF2E90 size=132
    let mut pc: u32 = 0x82DF2E90;
    'dispatch: loop {
        match pc {
            0x82DF2E90 => {
    //   block [0x82DF2E90..0x82DF2F14)
	// 82DF2E90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF2E94: 483B52D5  bl 0x831a8168
	ctx.lr = 0x82DF2E98;
	sub_831A8130(ctx, base);
	// 82DF2E98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF2E9C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DF2EA0: 90A100A4  stw r5, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[5].u32 ) };
	// 82DF2EA4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DF2EA8: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82DF2EAC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF2EB0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF2EB4: 7F055040  cmplw cr6, r5, r10
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DF2EB8: 409A0044  bne cr6, 0x82df2efc
	if !ctx.cr[6].eq {
	pc = 0x82DF2EFC; continue 'dispatch;
	}
	// 82DF2EBC: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF2EC0: 409A003C  bne cr6, 0x82df2efc
	if !ctx.cr[6].eq {
	pc = 0x82DF2EFC; continue 'dispatch;
	}
	// 82DF2EC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF2EC8: 4800CAC1  bl 0x82dff988
	ctx.lr = 0x82DF2ECC;
	sub_82DFF988(ctx, base);
	// 82DF2ECC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF2ED0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF2ED4: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF2ED8: 48000030  b 0x82df2f08
	pc = 0x82DF2F08; continue 'dispatch;
	// 82DF2EDC: 386100A4  addi r3, r1, 0xa4
	ctx.r[3].s64 = ctx.r[1].s64 + 164;
	// 82DF2EE0: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82DF2EE4: 4B5AE7A5  bl 0x823a1688
	ctx.lr = 0x82DF2EE8;
	sub_823A1688(ctx, base);
	// 82DF2EE8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82DF2EEC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF2EF0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF2EF4: 4BFFFB95  bl 0x82df2a88
	ctx.lr = 0x82DF2EF8;
	sub_82DF2A88(ctx, base);
	// 82DF2EF8: 80A100A4  lwz r5, 0xa4(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82DF2EFC: 7F05F040  cmplw cr6, r5, r30
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82DF2F00: 409AFFDC  bne cr6, 0x82df2edc
	if !ctx.cr[6].eq {
	pc = 0x82DF2EDC; continue 'dispatch;
	}
	// 82DF2F04: 90BD0000  stw r5, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82DF2F08: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DF2F0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DF2F10: 483B52A8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF2F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF2F18 size=88
    let mut pc: u32 = 0x82DF2F18;
    'dispatch: loop {
        match pc {
            0x82DF2F18 => {
    //   block [0x82DF2F18..0x82DF2F70)
	// 82DF2F18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF2F1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF2F20: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF2F24: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF2F28: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF2F2C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF2F30: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF2F34: 80DF0004  lwz r6, 4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF2F38: 80A60000  lwz r5, 0(r6)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF2F3C: 4BFFFF55  bl 0x82df2e90
	ctx.lr = 0x82DF2F40;
	sub_82DF2E90(ctx, base);
	// 82DF2F40: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82DF2F44: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF2F48: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82DF2F4C: 4BFFF23D  bl 0x82df2188
	ctx.lr = 0x82DF2F50;
	sub_82DF2188(ctx, base);
	// 82DF2F50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF2F54: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DF2F58: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DF2F5C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF2F60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF2F64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF2F68: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF2F6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF2F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF2F70 size=120
    let mut pc: u32 = 0x82DF2F70;
    'dispatch: loop {
        match pc {
            0x82DF2F70 => {
    //   block [0x82DF2F70..0x82DF2FE8)
	// 82DF2F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF2F74: 483B51F9  bl 0x831a816c
	ctx.lr = 0x82DF2F78;
	sub_831A8130(ctx, base);
	// 82DF2F78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF2F7C: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82DF2F80: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF2F84: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF2F88: 3BDF0884  addi r30, r31, 0x884
	ctx.r[30].s64 = ctx.r[31].s64 + 2180;
	// 82DF2F8C: 3BA00007  li r29, 7
	ctx.r[29].s64 = 7;
	// 82DF2F90: 996A56F4  stb r11, 0x56f4(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(22260 as u32), ctx.r[11].u8 ) };
	// 82DF2F94: 3BDEFFFC  addi r30, r30, -4
	ctx.r[30].s64 = ctx.r[30].s64 + -4;
	// 82DF2F98: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF2F9C: 4800048D  bl 0x82df3428
	ctx.lr = 0x82DF2FA0;
	sub_82DF3428(ctx, base);
	// 82DF2FA0: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82DF2FA4: 4080FFF0  bge 0x82df2f94
	if !ctx.cr[0].lt {
	pc = 0x82DF2F94; continue 'dispatch;
	}
	// 82DF2FA8: 387F085C  addi r3, r31, 0x85c
	ctx.r[3].s64 = ctx.r[31].s64 + 2140;
	// 82DF2FAC: 4800047D  bl 0x82df3428
	ctx.lr = 0x82DF2FB0;
	sub_82DF3428(ctx, base);
	// 82DF2FB0: 387F084C  addi r3, r31, 0x84c
	ctx.r[3].s64 = ctx.r[31].s64 + 2124;
	// 82DF2FB4: 4B67704D  bl 0x8246a000
	ctx.lr = 0x82DF2FB8;
	sub_8246A000(ctx, base);
	// 82DF2FB8: 387F0444  addi r3, r31, 0x444
	ctx.r[3].s64 = ctx.r[31].s64 + 1092;
	// 82DF2FBC: 4BFFFA4D  bl 0x82df2a08
	ctx.lr = 0x82DF2FC0;
	sub_82DF2A08(ctx, base);
	// 82DF2FC0: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 82DF2FC4: 48020C65  bl 0x82e13c28
	ctx.lr = 0x82DF2FC8;
	sub_82E13C28(ctx, base);
	// 82DF2FC8: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82DF2FCC: 4BFFFF4D  bl 0x82df2f18
	ctx.lr = 0x82DF2FD0;
	sub_82DF2F18(ctx, base);
	// 82DF2FD0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF2FD4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DF2FD8: 419A0008  beq cr6, 0x82df2fe0
	if ctx.cr[6].eq {
	pc = 0x82DF2FE0; continue 'dispatch;
	}
	// 82DF2FDC: 4B4CD8B5  bl 0x822c0890
	ctx.lr = 0x82DF2FE0;
	sub_822C0890(ctx, base);
	// 82DF2FE0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF2FE4: 483B51D8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF2FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF2FE8 size=164
    let mut pc: u32 = 0x82DF2FE8;
    'dispatch: loop {
        match pc {
            0x82DF2FE8 => {
    //   block [0x82DF2FE8..0x82DF308C)
	// 82DF2FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF2FEC: 483B517D  bl 0x831a8168
	ctx.lr = 0x82DF2FF0;
	sub_831A8130(ctx, base);
	// 82DF2FF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF2FF4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF2FF8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DF2FFC: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82DF3000: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82DF3004: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82DF3008: 482D0311  bl 0x830c3318
	ctx.lr = 0x82DF300C;
	sub_830C3318(ctx, base);
	// 82DF300C: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 82DF3010: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 82DF3014: 387F0444  addi r3, r31, 0x444
	ctx.r[3].s64 = ctx.r[31].s64 + 1092;
	// 82DF3018: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 82DF301C: 48007B75  bl 0x82dfab90
	ctx.lr = 0x82DF3020;
	sub_82DFAB90(ctx, base);
	// 82DF3020: 93DF0850  stw r30, 0x850(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2128 as u32), ctx.r[30].u32 ) };
	// 82DF3024: 93DF0854  stw r30, 0x854(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2132 as u32), ctx.r[30].u32 ) };
	// 82DF3028: 387F085C  addi r3, r31, 0x85c
	ctx.r[3].s64 = ctx.r[31].s64 + 2140;
	// 82DF302C: 93DF0858  stw r30, 0x858(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2136 as u32), ctx.r[30].u32 ) };
	// 82DF3030: 480000C1  bl 0x82df30f0
	ctx.lr = 0x82DF3034;
	sub_82DF30F0(ctx, base);
	// 82DF3034: 3B9F0864  addi r28, r31, 0x864
	ctx.r[28].s64 = ctx.r[31].s64 + 2148;
	// 82DF3038: 93DF0860  stw r30, 0x860(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2144 as u32), ctx.r[30].u32 ) };
	// 82DF303C: 3BA00007  li r29, 7
	ctx.r[29].s64 = 7;
	// 82DF3040: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DF3044: 480000AD  bl 0x82df30f0
	ctx.lr = 0x82DF3048;
	sub_82DF30F0(ctx, base);
	// 82DF3048: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82DF304C: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 82DF3050: 4080FFF0  bge 0x82df3040
	if !ctx.cr[0].lt {
	pc = 0x82DF3040; continue 'dispatch;
	}
	// 82DF3054: 397F0424  addi r11, r31, 0x424
	ctx.r[11].s64 = ctx.r[31].s64 + 1060;
	// 82DF3058: 93DF0884  stw r30, 0x884(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2180 as u32), ctx.r[30].u32 ) };
	// 82DF305C: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 82DF3060: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 82DF3064: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82DF3068: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DF306C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82DF3070: 4200FFF8  bdnz 0x82df3068
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82DF3068; continue 'dispatch;
	}
	// 82DF3074: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82DF3078: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DF307C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF3080: 996A56F4  stb r11, 0x56f4(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(22260 as u32), ctx.r[11].u8 ) };
	// 82DF3084: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DF3088: 483B5130  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF3090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF3090 size=92
    let mut pc: u32 = 0x82DF3090;
    'dispatch: loop {
        match pc {
            0x82DF3090 => {
    //   block [0x82DF3090..0x82DF30EC)
	// 82DF3090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF3094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF3098: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF309C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF30A0: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82DF30A4: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82DF30A8: 3BEB5700  addi r31, r11, 0x5700
	ctx.r[31].s64 = ctx.r[11].s64 + 22272;
	// 82DF30AC: 816A5F88  lwz r11, 0x5f88(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(24456 as u32) ) } as u64;
	// 82DF30B0: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82DF30B4: 40820020  bne 0x82df30d4
	if !ctx.cr[0].eq {
	pc = 0x82DF30D4; continue 'dispatch;
	}
	// 82DF30B8: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 82DF30BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF30C0: 916A5F88  stw r11, 0x5f88(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(24456 as u32), ctx.r[11].u32 ) };
	// 82DF30C4: 4BFFFF25  bl 0x82df2fe8
	ctx.lr = 0x82DF30C8;
	sub_82DF2FE8(ctx, base);
	// 82DF30C8: 3D608324  lis r11, -0x7cdc
	ctx.r[11].s64 = -2094792704;
	// 82DF30CC: 386B1BF8  addi r3, r11, 0x1bf8
	ctx.r[3].s64 = ctx.r[11].s64 + 7160;
	// 82DF30D0: 483B5409  bl 0x831a84d8
	ctx.lr = 0x82DF30D4;
	sub_831A84D8(ctx, base);
	// 82DF30D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF30D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF30DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF30E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF30E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF30E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF30F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF30F0 size=16
    let mut pc: u32 = 0x82DF30F0;
    'dispatch: loop {
        match pc {
            0x82DF30F0 => {
    //   block [0x82DF30F0..0x82DF3100)
	// 82DF30F0: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DF30F4: 396BB22C  addi r11, r11, -0x4dd4
	ctx.r[11].s64 = ctx.r[11].s64 + -19924;
	// 82DF30F8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF30FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF3100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF3100 size=172
    let mut pc: u32 = 0x82DF3100;
    'dispatch: loop {
        match pc {
            0x82DF3100 => {
    //   block [0x82DF3100..0x82DF31AC)
	// 82DF3100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF3104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF3108: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF310C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF3110: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF3114: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF3118: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DF311C: 4082000C  bne 0x82df3128
	if !ctx.cr[0].eq {
	pc = 0x82DF3128; continue 'dispatch;
	}
	// 82DF3120: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DF3124: 48000028  b 0x82df314c
	pc = 0x82DF314C; continue 'dispatch;
	// 82DF3128: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82DF312C: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF3130: 40990014  ble cr6, 0x82df3144
	if !ctx.cr[6].gt {
	pc = 0x82DF3144; continue 'dispatch;
	}
	// 82DF3134: 3D60A000  lis r11, -0x6000
	ctx.r[11].s64 = -1610612736;
	// 82DF3138: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF313C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DF3140: 41980008  blt cr6, 0x82df3148
	if ctx.cr[6].lt {
	pc = 0x82DF3148; continue 'dispatch;
	}
	// 82DF3144: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF3148: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82DF314C: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF3150: 4082003C  bne 0x82df318c
	if !ctx.cr[0].eq {
	pc = 0x82DF318C; continue 'dispatch;
	}
	// 82DF3154: 396AFFFC  addi r11, r10, -4
	ctx.r[11].s64 = ctx.r[10].s64 + -4;
	// 82DF3158: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82DF315C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82DF3160: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82DF3164: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82DF3168: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82DF316C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82DF3170: 4082FFE8  bne 0x82df3158
	if !ctx.cr[0].eq {
	pc = 0x82DF3158; continue 'dispatch;
	}
	// 82DF3174: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82DF3178: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF317C: 409A0010  bne cr6, 0x82df318c
	if !ctx.cr[6].eq {
	pc = 0x82DF318C; continue 'dispatch;
	}
	// 82DF3180: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF3184: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 82DF3188: 483B5A81  bl 0x831a8c08
	ctx.lr = 0x82DF318C;
	sub_831A8C08(ctx, base);
	// 82DF318C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DF3190: 396BB22C  addi r11, r11, -0x4dd4
	ctx.r[11].s64 = ctx.r[11].s64 + -19924;
	// 82DF3194: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF3198: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF319C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF31A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF31A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF31A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF31B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF31B0 size=20
    let mut pc: u32 = 0x82DF31B0;
    'dispatch: loop {
        match pc {
            0x82DF31B0 => {
    //   block [0x82DF31B0..0x82DF31C4)
	// 82DF31B0: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF31B4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DF31B8: 4082000C  bne 0x82df31c4
	if !ctx.cr[0].eq {
		sub_82DF31C4(ctx, base);
		return;
	}
	// 82DF31BC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DF31C0: 48000028  b 0x82df31e8
	sub_82DF31C4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF31C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF31C4 size=44
    let mut pc: u32 = 0x82DF31C4;
    'dispatch: loop {
        match pc {
            0x82DF31C4 => {
    //   block [0x82DF31C4..0x82DF31F0)
	// 82DF31C4: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82DF31C8: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF31CC: 40990014  ble cr6, 0x82df31e0
	if !ctx.cr[6].gt {
	pc = 0x82DF31E0; continue 'dispatch;
	}
	// 82DF31D0: 3D60A000  lis r11, -0x6000
	ctx.r[11].s64 = -1610612736;
	// 82DF31D4: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF31D8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DF31DC: 41980008  blt cr6, 0x82df31e4
	if ctx.cr[6].lt {
	pc = 0x82DF31E4; continue 'dispatch;
	}
	// 82DF31E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF31E4: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82DF31E8: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF31EC: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF31F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF31F0 size=8
    let mut pc: u32 = 0x82DF31F0;
    'dispatch: loop {
        match pc {
            0x82DF31F0 => {
    //   block [0x82DF31F0..0x82DF31F8)
	// 82DF31F0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DF31F4: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF31F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF31F8 size=12
    let mut pc: u32 = 0x82DF31F8;
    'dispatch: loop {
        match pc {
            0x82DF31F8 => {
    //   block [0x82DF31F8..0x82DF3204)
	// 82DF31F8: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DF31FC: 386BB22C  addi r3, r11, -0x4dd4
	ctx.r[3].s64 = ctx.r[11].s64 + -19924;
	// 82DF3200: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF3208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF3208 size=4
    let mut pc: u32 = 0x82DF3208;
    'dispatch: loop {
        match pc {
            0x82DF3208 => {
    //   block [0x82DF3208..0x82DF320C)
	// 82DF3208: 4BFFFFA8  b 0x82df31b0
	sub_82DF31B0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF3210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF3210 size=36
    let mut pc: u32 = 0x82DF3210;
    'dispatch: loop {
        match pc {
            0x82DF3210 => {
    //   block [0x82DF3210..0x82DF3234)
	// 82DF3210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF3214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF3218: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF321C: 4BFFFF95  bl 0x82df31b0
	ctx.lr = 0x82DF3220;
	sub_82DF31B0(ctx, base);
	// 82DF3220: 7C632214  add r3, r3, r4
	ctx.r[3].u64 = ctx.r[3].u64 + ctx.r[4].u64;
	// 82DF3224: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF3228: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF322C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF3230: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF3238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF3238 size=100
    let mut pc: u32 = 0x82DF3238;
    'dispatch: loop {
        match pc {
            0x82DF3238 => {
    //   block [0x82DF3238..0x82DF329C)
	// 82DF3238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF323C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF3240: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF3244: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82DF3248: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DF324C: 4BFFFF65  bl 0x82df31b0
	ctx.lr = 0x82DF3250;
	sub_82DF31B0(ctx, base);
	// 82DF3250: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82DF3254: 7D234B78  mr r3, r9
	ctx.r[3].u64 = ctx.r[9].u64;
	// 82DF3258: 4BFFFF59  bl 0x82df31b0
	ctx.lr = 0x82DF325C;
	sub_82DF31B0(ctx, base);
	// 82DF325C: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF3260: 892A0000  lbz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF3264: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82DF3268: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 82DF326C: 41820014  beq 0x82df3280
	if ctx.cr[0].eq {
	pc = 0x82DF3280; continue 'dispatch;
	}
	// 82DF3270: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 82DF3274: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DF3278: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF327C: 419AFFE0  beq cr6, 0x82df325c
	if ctx.cr[6].eq {
	pc = 0x82DF325C; continue 'dispatch;
	}
	// 82DF3280: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82DF3284: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82DF3288: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82DF328C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF3290: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF3294: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF3298: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF32A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF32A0 size=100
    let mut pc: u32 = 0x82DF32A0;
    'dispatch: loop {
        match pc {
            0x82DF32A0 => {
    //   block [0x82DF32A0..0x82DF3304)
	// 82DF32A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF32A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF32A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF32AC: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82DF32B0: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DF32B4: 4BFFFEFD  bl 0x82df31b0
	ctx.lr = 0x82DF32B8;
	sub_82DF31B0(ctx, base);
	// 82DF32B8: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82DF32BC: 7D234B78  mr r3, r9
	ctx.r[3].u64 = ctx.r[9].u64;
	// 82DF32C0: 4BFFFEF1  bl 0x82df31b0
	ctx.lr = 0x82DF32C4;
	sub_82DF31B0(ctx, base);
	// 82DF32C4: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF32C8: 892A0000  lbz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF32CC: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82DF32D0: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 82DF32D4: 41820014  beq 0x82df32e8
	if ctx.cr[0].eq {
	pc = 0x82DF32E8; continue 'dispatch;
	}
	// 82DF32D8: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 82DF32DC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DF32E0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF32E4: 419AFFE0  beq cr6, 0x82df32c4
	if ctx.cr[6].eq {
	pc = 0x82DF32C4; continue 'dispatch;
	}
	// 82DF32E8: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82DF32EC: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82DF32F0: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 82DF32F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF32F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF32FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF3300: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF3308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF3308 size=96
    let mut pc: u32 = 0x82DF3308;
    'dispatch: loop {
        match pc {
            0x82DF3308 => {
    //   block [0x82DF3308..0x82DF3368)
	// 82DF3308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF330C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF3310: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF3314: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82DF3318: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DF331C: 4BFFFE95  bl 0x82df31b0
	ctx.lr = 0x82DF3320;
	sub_82DF31B0(ctx, base);
	// 82DF3320: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82DF3324: 7D234B78  mr r3, r9
	ctx.r[3].u64 = ctx.r[9].u64;
	// 82DF3328: 4BFFFE89  bl 0x82df31b0
	ctx.lr = 0x82DF332C;
	sub_82DF31B0(ctx, base);
	// 82DF332C: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF3330: 892A0000  lbz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF3334: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82DF3338: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 82DF333C: 41820014  beq 0x82df3350
	if ctx.cr[0].eq {
	pc = 0x82DF3350; continue 'dispatch;
	}
	// 82DF3340: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 82DF3344: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DF3348: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF334C: 419AFFE0  beq cr6, 0x82df332c
	if ctx.cr[6].eq {
	pc = 0x82DF332C; continue 'dispatch;
	}
	// 82DF3350: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82DF3354: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82DF3358: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF335C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF3360: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF3364: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF3368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF3368 size=188
    let mut pc: u32 = 0x82DF3368;
    'dispatch: loop {
        match pc {
            0x82DF3368 => {
    //   block [0x82DF3368..0x82DF3424)
	// 82DF3368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF336C: 483B4DF5  bl 0x831a8160
	ctx.lr = 0x82DF3370;
	sub_831A8130(ctx, base);
	// 82DF3370: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF3374: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82DF3378: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82DF337C: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82DF3380: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82DF3384: 419A0074  beq cr6, 0x82df33f8
	if ctx.cr[6].eq {
	pc = 0x82DF33F8; continue 'dispatch;
	}
	// 82DF3388: 813C0014  lwz r9, 0x14(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DF338C: 7F054840  cmplw cr6, r5, r9
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DF3390: 40980068  bge cr6, 0x82df33f8
	if !ctx.cr[6].lt {
	pc = 0x82DF33F8; continue 'dispatch;
	}
	// 82DF3394: 815C0018  lwz r10, 0x18(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DF3398: 3BDC0004  addi r30, r28, 4
	ctx.r[30].s64 = ctx.r[28].s64 + 4;
	// 82DF339C: 2B0A0010  cmplwi cr6, r10, 0x10
	ctx.cr[6].compare_u32(ctx.r[10].u32, 16 as u32, &mut ctx.xer);
	// 82DF33A0: 4198000C  blt cr6, 0x82df33ac
	if ctx.cr[6].lt {
	pc = 0x82DF33AC; continue 'dispatch;
	}
	// 82DF33A4: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF33A8: 48000008  b 0x82df33b0
	pc = 0x82DF33B0; continue 'dispatch;
	// 82DF33AC: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82DF33B0: 7FA95A14  add r29, r9, r11
	ctx.r[29].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82DF33B4: 2B0A0010  cmplwi cr6, r10, 0x10
	ctx.cr[6].compare_u32(ctx.r[10].u32, 16 as u32, &mut ctx.xer);
	// 82DF33B8: 4198000C  blt cr6, 0x82df33c4
	if ctx.cr[6].lt {
	pc = 0x82DF33C4; continue 'dispatch;
	}
	// 82DF33BC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF33C0: 48000008  b 0x82df33c8
	pc = 0x82DF33C8; continue 'dispatch;
	// 82DF33C4: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82DF33C8: 7FEB2A14  add r31, r11, r5
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 82DF33CC: 48000024  b 0x82df33f0
	pc = 0x82DF33F0; continue 'dispatch;
	// 82DF33D0: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF33D4: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82DF33D8: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82DF33DC: 7D640774  extsb r4, r11
	ctx.r[4].s64 = ctx.r[11].s8 as i64;
	// 82DF33E0: 483B6D41  bl 0x831aa120
	ctx.lr = 0x82DF33E4;
	sub_831AA120(ctx, base);
	// 82DF33E4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DF33E8: 4082001C  bne 0x82df3404
	if !ctx.cr[0].eq {
	pc = 0x82DF3404; continue 'dispatch;
	}
	// 82DF33EC: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82DF33F0: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82DF33F4: 4198FFDC  blt cr6, 0x82df33d0
	if ctx.cr[6].lt {
	pc = 0x82DF33D0; continue 'dispatch;
	}
	// 82DF33F8: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82DF33FC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DF3400: 483B4DB0  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 82DF3404: 817C0018  lwz r11, 0x18(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DF3408: 2B0B0010  cmplwi cr6, r11, 0x10
	ctx.cr[6].compare_u32(ctx.r[11].u32, 16 as u32, &mut ctx.xer);
	// 82DF340C: 4198000C  blt cr6, 0x82df3418
	if ctx.cr[6].lt {
	pc = 0x82DF3418; continue 'dispatch;
	}
	// 82DF3410: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF3414: 48000008  b 0x82df341c
	pc = 0x82DF341C; continue 'dispatch;
	// 82DF3418: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82DF341C: 7C6BF850  subf r3, r11, r31
	ctx.r[3].s64 = ctx.r[31].s64 - ctx.r[11].s64;
	// 82DF3420: 4BFFFFDC  b 0x82df33fc
	pc = 0x82DF33FC; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF3428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF3428 size=4
    let mut pc: u32 = 0x82DF3428;
    'dispatch: loop {
        match pc {
            0x82DF3428 => {
    //   block [0x82DF3428..0x82DF342C)
	// 82DF3428: 4BFFFCD8  b 0x82df3100
	sub_82DF3100(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF3430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF3430 size=84
    let mut pc: u32 = 0x82DF3430;
    'dispatch: loop {
        match pc {
            0x82DF3430 => {
    //   block [0x82DF3430..0x82DF3484)
	// 82DF3430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF3434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF3438: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF343C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF3440: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF3444: 419A002C  beq cr6, 0x82df3470
	if ctx.cr[6].eq {
	pc = 0x82DF3470; continue 'dispatch;
	}
	// 82DF3448: 4BFFFD69  bl 0x82df31b0
	ctx.lr = 0x82DF344C;
	sub_82DF31B0(ctx, base);
	// 82DF344C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DF3450: 89430000  lbz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF3454: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 82DF3458: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF345C: 409AFFF4  bne cr6, 0x82df3450
	if !ctx.cr[6].eq {
	pc = 0x82DF3450; continue 'dispatch;
	}
	// 82DF3460: 7D6B1850  subf r11, r11, r3
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	// 82DF3464: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DF3468: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82DF346C: 48000008  b 0x82df3474
	pc = 0x82DF3474; continue 'dispatch;
	// 82DF3470: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DF3474: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF3478: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF347C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF3480: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF3488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF3488 size=76
    let mut pc: u32 = 0x82DF3488;
    'dispatch: loop {
        match pc {
            0x82DF3488 => {
    //   block [0x82DF3488..0x82DF34D4)
	// 82DF3488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF348C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF3490: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF3494: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 82DF3498: 4BFFFD19  bl 0x82df31b0
	ctx.lr = 0x82DF349C;
	sub_82DF31B0(ctx, base);
	// 82DF349C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DF34A0: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF34A4: 890A0000  lbz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF34A8: 2C090000  cmpwi r9, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82DF34AC: 7C684850  subf r3, r8, r9
	ctx.r[3].s64 = ctx.r[9].s64 - ctx.r[8].s64;
	// 82DF34B0: 41820014  beq 0x82df34c4
	if ctx.cr[0].eq {
	pc = 0x82DF34C4; continue 'dispatch;
	}
	// 82DF34B4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DF34B8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DF34BC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82DF34C0: 419AFFE0  beq cr6, 0x82df34a0
	if ctx.cr[6].eq {
	pc = 0x82DF34A0; continue 'dispatch;
	}
	// 82DF34C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF34C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF34CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF34D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF34D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF34D8 size=160
    let mut pc: u32 = 0x82DF34D8;
    'dispatch: loop {
        match pc {
            0x82DF34D8 => {
    //   block [0x82DF34D8..0x82DF3578)
	// 82DF34D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF34DC: 483B4C8D  bl 0x831a8168
	ctx.lr = 0x82DF34E0;
	sub_831A8130(ctx, base);
	// 82DF34E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF34E4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82DF34E8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DF34EC: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82DF34F0: 4198000C  blt cr6, 0x82df34fc
	if ctx.cr[6].lt {
	pc = 0x82DF34FC; continue 'dispatch;
	}
	// 82DF34F4: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82DF34F8: 48000024  b 0x82df351c
	pc = 0x82DF351C; continue 'dispatch;
	// 82DF34FC: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82DF3500: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF3504: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DF3508: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF350C: 409AFFF4  bne cr6, 0x82df3500
	if !ctx.cr[6].eq {
	pc = 0x82DF3500; continue 'dispatch;
	}
	// 82DF3510: 7D7D5850  subf r11, r29, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 82DF3514: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DF3518: 557F003E  slwi r31, r11, 0
	ctx.r[31].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82DF351C: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82DF3520: 409A001C  bne cr6, 0x82df353c
	if !ctx.cr[6].eq {
	pc = 0x82DF353C; continue 'dispatch;
	}
	// 82DF3524: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DF3528: 4BFFFBD9  bl 0x82df3100
	ctx.lr = 0x82DF352C;
	sub_82DF3100(ctx, base);
	// 82DF352C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DF3530: 396BB22C  addi r11, r11, -0x4dd4
	ctx.r[11].s64 = ctx.r[11].s64 + -19924;
	// 82DF3534: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF3538: 48000038  b 0x82df3570
	pc = 0x82DF3570; continue 'dispatch;
	// 82DF353C: 387F0005  addi r3, r31, 5
	ctx.r[3].s64 = ctx.r[31].s64 + 5;
	// 82DF3540: 4BFFD041  bl 0x82df0580
	ctx.lr = 0x82DF3544;
	sub_82DF0580(ctx, base);
	// 82DF3544: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DF3548: 38BF0001  addi r5, r31, 1
	ctx.r[5].s64 = ctx.r[31].s64 + 1;
	// 82DF354C: 3BFE0004  addi r31, r30, 4
	ctx.r[31].s64 = ctx.r[30].s64 + 4;
	// 82DF3550: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DF3554: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF3558: 483B4FB9  bl 0x831a8510
	ctx.lr = 0x82DF355C;
	sub_831A8510(ctx, base);
	// 82DF355C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DF3560: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DF3564: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF3568: 4BFFFB99  bl 0x82df3100
	ctx.lr = 0x82DF356C;
	sub_82DF3100(ctx, base);
	// 82DF356C: 93FC0000  stw r31, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82DF3570: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DF3574: 483B4C44  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF3578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF3578 size=180
    let mut pc: u32 = 0x82DF3578;
    'dispatch: loop {
        match pc {
            0x82DF3578 => {
    //   block [0x82DF3578..0x82DF362C)
	// 82DF3578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF357C: 483B4BE5  bl 0x831a8160
	ctx.lr = 0x82DF3580;
	sub_831A8130(ctx, base);
	// 82DF3580: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF3584: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82DF3588: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82DF358C: 4BFFFC25  bl 0x82df31b0
	ctx.lr = 0x82DF3590;
	sub_82DF31B0(ctx, base);
	// 82DF3590: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DF3594: 89430000  lbz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF3598: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 82DF359C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF35A0: 409AFFF4  bne cr6, 0x82df3594
	if !ctx.cr[6].eq {
	pc = 0x82DF3594; continue 'dispatch;
	}
	// 82DF35A4: 7D4B1850  subf r10, r11, r3
	ctx.r[10].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	// 82DF35A8: 7F4BD378  mr r11, r26
	ctx.r[11].u64 = ctx.r[26].u64;
	// 82DF35AC: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82DF35B0: 555E003E  slwi r30, r10, 0
	ctx.r[30].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82DF35B4: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF35B8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DF35BC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF35C0: 409AFFF4  bne cr6, 0x82df35b4
	if !ctx.cr[6].eq {
	pc = 0x82DF35B4; continue 'dispatch;
	}
	// 82DF35C4: 7D7A5850  subf r11, r26, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[26].s64;
	// 82DF35C8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DF35CC: 557B003E  slwi r27, r11, 0
	ctx.r[27].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[27].u64 = ctx.r[27].u32 as u64;
	// 82DF35D0: 7D7BF214  add r11, r27, r30
	ctx.r[11].u64 = ctx.r[27].u64 + ctx.r[30].u64;
	// 82DF35D4: 386B0005  addi r3, r11, 5
	ctx.r[3].s64 = ctx.r[11].s64 + 5;
	// 82DF35D8: 4BFFCFA9  bl 0x82df0580
	ctx.lr = 0x82DF35DC;
	sub_82DF0580(ctx, base);
	// 82DF35DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF35E0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DF35E4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DF35E8: 3BBF0004  addi r29, r31, 4
	ctx.r[29].s64 = ctx.r[31].s64 + 4;
	// 82DF35EC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DF35F0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF35F4: 4BFFFBBD  bl 0x82df31b0
	ctx.lr = 0x82DF35F8;
	sub_82DF31B0(ctx, base);
	// 82DF35F8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DF35FC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DF3600: 483B4F11  bl 0x831a8510
	ctx.lr = 0x82DF3604;
	sub_831A8510(ctx, base);
	// 82DF3604: 7D7FF214  add r11, r31, r30
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[30].u64;
	// 82DF3608: 38BB0001  addi r5, r27, 1
	ctx.r[5].s64 = ctx.r[27].s64 + 1;
	// 82DF360C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82DF3610: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82DF3614: 483B4EFD  bl 0x831a8510
	ctx.lr = 0x82DF3618;
	sub_831A8510(ctx, base);
	// 82DF3618: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DF361C: 4BFFFAE5  bl 0x82df3100
	ctx.lr = 0x82DF3620;
	sub_82DF3100(ctx, base);
	// 82DF3620: 93BC0000  stw r29, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82DF3624: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DF3628: 483B4B88  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF3630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF3630 size=164
    let mut pc: u32 = 0x82DF3630;
    'dispatch: loop {
        match pc {
            0x82DF3630 => {
    //   block [0x82DF3630..0x82DF36D4)
	// 82DF3630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF3634: 483B4B2D  bl 0x831a8160
	ctx.lr = 0x82DF3638;
	sub_831A8130(ctx, base);
	// 82DF3638: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF363C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DF3640: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82DF3644: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82DF3648: 4BFFFB69  bl 0x82df31b0
	ctx.lr = 0x82DF364C;
	sub_82DF31B0(ctx, base);
	// 82DF364C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DF3650: 89430000  lbz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF3654: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 82DF3658: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF365C: 409AFFF4  bne cr6, 0x82df3650
	if !ctx.cr[6].eq {
	pc = 0x82DF3650; continue 'dispatch;
	}
	// 82DF3660: 7D6B1850  subf r11, r11, r3
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	// 82DF3664: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DF3668: 557E003E  slwi r30, r11, 0
	ctx.r[30].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82DF366C: 7D7EE214  add r11, r30, r28
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[28].u64;
	// 82DF3670: 386B0005  addi r3, r11, 5
	ctx.r[3].s64 = ctx.r[11].s64 + 5;
	// 82DF3674: 4BFFCF0D  bl 0x82df0580
	ctx.lr = 0x82DF3678;
	sub_82DF0580(ctx, base);
	// 82DF3678: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF367C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DF3680: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DF3684: 3B7F0004  addi r27, r31, 4
	ctx.r[27].s64 = ctx.r[31].s64 + 4;
	// 82DF3688: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DF368C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF3690: 4BFFFB21  bl 0x82df31b0
	ctx.lr = 0x82DF3694;
	sub_82DF31B0(ctx, base);
	// 82DF3694: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DF3698: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82DF369C: 483B4E75  bl 0x831a8510
	ctx.lr = 0x82DF36A0;
	sub_831A8510(ctx, base);
	// 82DF36A0: 7FFFF214  add r31, r31, r30
	ctx.r[31].u64 = ctx.r[31].u64 + ctx.r[30].u64;
	// 82DF36A4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82DF36A8: 7F440774  extsb r4, r26
	ctx.r[4].s64 = ctx.r[26].s8 as i64;
	// 82DF36AC: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82DF36B0: 483B4B31  bl 0x831a81e0
	ctx.lr = 0x82DF36B4;
	sub_831A81E0(ctx, base);
	// 82DF36B4: 7D7FE214  add r11, r31, r28
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[28].u64;
	// 82DF36B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DF36BC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DF36C0: 994B0004  stb r10, 4(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u8 ) };
	// 82DF36C4: 4BFFFA3D  bl 0x82df3100
	ctx.lr = 0x82DF36C8;
	sub_82DF3100(ctx, base);
	// 82DF36C8: 937D0000  stw r27, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82DF36CC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DF36D0: 483B4AE0  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF36D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF36D8 size=52
    let mut pc: u32 = 0x82DF36D8;
    'dispatch: loop {
        match pc {
            0x82DF36D8 => {
    //   block [0x82DF36D8..0x82DF370C)
	// 82DF36D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF36DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF36E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF36E4: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82DF36E8: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DF36EC: 4BFFFAC5  bl 0x82df31b0
	ctx.lr = 0x82DF36F0;
	sub_82DF31B0(ctx, base);
	// 82DF36F0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DF36F4: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 82DF36F8: 4BFFFE81  bl 0x82df3578
	ctx.lr = 0x82DF36FC;
	sub_82DF3578(ctx, base);
	// 82DF36FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF3700: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF3704: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF3708: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF3710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF3710 size=180
    let mut pc: u32 = 0x82DF3710;
    'dispatch: loop {
        match pc {
            0x82DF3710 => {
    //   block [0x82DF3710..0x82DF37C4)
	// 82DF3710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF3714: 483B4A49  bl 0x831a815c
	ctx.lr = 0x82DF3718;
	sub_831A8130(ctx, base);
	// 82DF3718: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF371C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DF3720: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82DF3724: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 82DF3728: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82DF372C: 4BFFFA85  bl 0x82df31b0
	ctx.lr = 0x82DF3730;
	sub_82DF31B0(ctx, base);
	// 82DF3730: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DF3734: 89430000  lbz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF3738: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 82DF373C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF3740: 409AFFF4  bne cr6, 0x82df3734
	if !ctx.cr[6].eq {
	pc = 0x82DF3734; continue 'dispatch;
	}
	// 82DF3744: 7D6B1850  subf r11, r11, r3
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	// 82DF3748: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DF374C: 557E003E  slwi r30, r11, 0
	ctx.r[30].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82DF3750: 7D7EE214  add r11, r30, r28
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[28].u64;
	// 82DF3754: 386B0005  addi r3, r11, 5
	ctx.r[3].s64 = ctx.r[11].s64 + 5;
	// 82DF3758: 4BFFCE29  bl 0x82df0580
	ctx.lr = 0x82DF375C;
	sub_82DF0580(ctx, base);
	// 82DF375C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF3760: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DF3764: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DF3768: 3B7F0004  addi r27, r31, 4
	ctx.r[27].s64 = ctx.r[31].s64 + 4;
	// 82DF376C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DF3770: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF3774: 4BFFFA3D  bl 0x82df31b0
	ctx.lr = 0x82DF3778;
	sub_82DF31B0(ctx, base);
	// 82DF3778: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DF377C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82DF3780: 483B4D91  bl 0x831a8510
	ctx.lr = 0x82DF3784;
	sub_831A8510(ctx, base);
	// 82DF3784: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82DF3788: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82DF378C: 4BFFFA25  bl 0x82df31b0
	ctx.lr = 0x82DF3790;
	sub_82DF31B0(ctx, base);
	// 82DF3790: 7D7FF214  add r11, r31, r30
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[30].u64;
	// 82DF3794: 7C83CA14  add r4, r3, r25
	ctx.r[4].u64 = ctx.r[3].u64 + ctx.r[25].u64;
	// 82DF3798: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82DF379C: 483B4D75  bl 0x831a8510
	ctx.lr = 0x82DF37A0;
	sub_831A8510(ctx, base);
	// 82DF37A0: 7D7FF214  add r11, r31, r30
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[30].u64;
	// 82DF37A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DF37A8: 7D6BE214  add r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82DF37AC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DF37B0: 994B0004  stb r10, 4(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u8 ) };
	// 82DF37B4: 4BFFF94D  bl 0x82df3100
	ctx.lr = 0x82DF37B8;
	sub_82DF3100(ctx, base);
	// 82DF37B8: 937D0000  stw r27, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82DF37BC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DF37C0: 483B49EC  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF37C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF37C8 size=84
    let mut pc: u32 = 0x82DF37C8;
    'dispatch: loop {
        match pc {
            0x82DF37C8 => {
    //   block [0x82DF37C8..0x82DF381C)
	// 82DF37C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF37CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF37D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF37D4: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82DF37D8: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DF37DC: 4BFFF9D5  bl 0x82df31b0
	ctx.lr = 0x82DF37E0;
	sub_82DF31B0(ctx, base);
	// 82DF37E0: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF37E4: 892A0000  lbz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF37E8: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82DF37EC: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 82DF37F0: 41820014  beq 0x82df3804
	if ctx.cr[0].eq {
	pc = 0x82DF3804; continue 'dispatch;
	}
	// 82DF37F4: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 82DF37F8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DF37FC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF3800: 419AFFE0  beq cr6, 0x82df37e0
	if ctx.cr[6].eq {
	pc = 0x82DF37E0; continue 'dispatch;
	}
	// 82DF3804: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82DF3808: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82DF380C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF3810: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF3814: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF3818: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF3820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF3820 size=88
    let mut pc: u32 = 0x82DF3820;
    'dispatch: loop {
        match pc {
            0x82DF3820 => {
    //   block [0x82DF3820..0x82DF3878)
	// 82DF3820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF3824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF3828: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF382C: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82DF3830: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DF3834: 4BFFF97D  bl 0x82df31b0
	ctx.lr = 0x82DF3838;
	sub_82DF31B0(ctx, base);
	// 82DF3838: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF383C: 892A0000  lbz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF3840: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82DF3844: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 82DF3848: 41820014  beq 0x82df385c
	if ctx.cr[0].eq {
	pc = 0x82DF385C; continue 'dispatch;
	}
	// 82DF384C: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 82DF3850: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DF3854: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF3858: 419AFFE0  beq cr6, 0x82df3838
	if ctx.cr[6].eq {
	pc = 0x82DF3838; continue 'dispatch;
	}
	// 82DF385C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82DF3860: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82DF3864: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 82DF3868: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF386C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF3870: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF3874: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF3878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF3878 size=52
    let mut pc: u32 = 0x82DF3878;
    'dispatch: loop {
        match pc {
            0x82DF3878 => {
    //   block [0x82DF3878..0x82DF38AC)
	// 82DF3878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF387C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF3880: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF3884: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF3888: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82DF388C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF3890: 4BFFFC49  bl 0x82df34d8
	ctx.lr = 0x82DF3894;
	sub_82DF34D8(ctx, base);
	// 82DF3894: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF3898: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF389C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF38A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF38A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF38A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF38B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF38B0 size=64
    let mut pc: u32 = 0x82DF38B0;
    'dispatch: loop {
        match pc {
            0x82DF38B0 => {
    //   block [0x82DF38B0..0x82DF38F0)
	// 82DF38B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF38B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF38B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF38BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF38C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF38C4: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DF38C8: 4BFFF8E9  bl 0x82df31b0
	ctx.lr = 0x82DF38CC;
	sub_82DF31B0(ctx, base);
	// 82DF38CC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DF38D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF38D4: 4BFFFCA5  bl 0x82df3578
	ctx.lr = 0x82DF38D8;
	sub_82DF3578(ctx, base);
	// 82DF38D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF38DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF38E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF38E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF38E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF38EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF38F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF38F0 size=132
    let mut pc: u32 = 0x82DF38F0;
    'dispatch: loop {
        match pc {
            0x82DF38F0 => {
    //   block [0x82DF38F0..0x82DF3974)
	// 82DF38F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF38F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF38F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF38FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF3900: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF3904: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF3908: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DF390C: 4082000C  bne 0x82df3918
	if !ctx.cr[0].eq {
	pc = 0x82DF3918; continue 'dispatch;
	}
	// 82DF3910: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DF3914: 48000028  b 0x82df393c
	pc = 0x82DF393C; continue 'dispatch;
	// 82DF3918: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82DF391C: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF3920: 40990014  ble cr6, 0x82df3934
	if !ctx.cr[6].gt {
	pc = 0x82DF3934; continue 'dispatch;
	}
	// 82DF3924: 3D60A000  lis r11, -0x6000
	ctx.r[11].s64 = -1610612736;
	// 82DF3928: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF392C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DF3930: 41980008  blt cr6, 0x82df3938
	if ctx.cr[6].lt {
	pc = 0x82DF3938; continue 'dispatch;
	}
	// 82DF3934: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF3938: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82DF393C: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF3940: 40820010  bne 0x82df3950
	if !ctx.cr[0].eq {
	pc = 0x82DF3950; continue 'dispatch;
	}
	// 82DF3944: 8164FFFC  lwz r11, -4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82DF3948: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82DF394C: 419A0010  beq cr6, 0x82df395c
	if ctx.cr[6].eq {
	pc = 0x82DF395C; continue 'dispatch;
	}
	// 82DF3950: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82DF3954: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF3958: 4BFFFB81  bl 0x82df34d8
	ctx.lr = 0x82DF395C;
	sub_82DF34D8(ctx, base);
	// 82DF395C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF3960: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF3964: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF3968: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF396C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF3970: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF3978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF3978 size=144
    let mut pc: u32 = 0x82DF3978;
    'dispatch: loop {
        match pc {
            0x82DF3978 => {
    //   block [0x82DF3978..0x82DF3A08)
	// 82DF3978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF397C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF3980: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF3984: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF3988: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF398C: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF3990: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DF3994: 4082000C  bne 0x82df39a0
	if !ctx.cr[0].eq {
	pc = 0x82DF39A0; continue 'dispatch;
	}
	// 82DF3998: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DF399C: 48000028  b 0x82df39c4
	pc = 0x82DF39C4; continue 'dispatch;
	// 82DF39A0: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82DF39A4: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF39A8: 40990014  ble cr6, 0x82df39bc
	if !ctx.cr[6].gt {
	pc = 0x82DF39BC; continue 'dispatch;
	}
	// 82DF39AC: 3D60A000  lis r11, -0x6000
	ctx.r[11].s64 = -1610612736;
	// 82DF39B0: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF39B4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DF39B8: 41980008  blt cr6, 0x82df39c0
	if ctx.cr[6].lt {
	pc = 0x82DF39C0; continue 'dispatch;
	}
	// 82DF39BC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF39C0: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82DF39C4: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF39C8: 40820010  bne 0x82df39d8
	if !ctx.cr[0].eq {
	pc = 0x82DF39D8; continue 'dispatch;
	}
	// 82DF39CC: 8164FFFC  lwz r11, -4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82DF39D0: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82DF39D4: 419A0010  beq cr6, 0x82df39e4
	if ctx.cr[6].eq {
	pc = 0x82DF39E4; continue 'dispatch;
	}
	// 82DF39D8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82DF39DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF39E0: 4BFFFAF9  bl 0x82df34d8
	ctx.lr = 0x82DF39E4;
	sub_82DF34D8(ctx, base);
	// 82DF39E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF39E8: 4BFFFA49  bl 0x82df3430
	ctx.lr = 0x82DF39EC;
	sub_82DF3430(ctx, base);
	// 82DF39EC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF39F0: 7C635A14  add r3, r3, r11
	ctx.r[3].u64 = ctx.r[3].u64 + ctx.r[11].u64;
	// 82DF39F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF39F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF39FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF3A00: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF3A04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF3A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF3A08 size=136
    let mut pc: u32 = 0x82DF3A08;
    'dispatch: loop {
        match pc {
            0x82DF3A08 => {
    //   block [0x82DF3A08..0x82DF3A90)
	// 82DF3A08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF3A0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF3A10: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF3A14: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF3A18: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF3A1C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DF3A20: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82DF3A24: 396BB22C  addi r11, r11, -0x4dd4
	ctx.r[11].s64 = ctx.r[11].s64 + -19924;
	// 82DF3A28: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF3A2C: 409A000C  bne cr6, 0x82df3a38
	if !ctx.cr[6].eq {
	pc = 0x82DF3A38; continue 'dispatch;
	}
	// 82DF3A30: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DF3A34: 48000028  b 0x82df3a5c
	pc = 0x82DF3A5C; continue 'dispatch;
	// 82DF3A38: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82DF3A3C: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF3A40: 40990014  ble cr6, 0x82df3a54
	if !ctx.cr[6].gt {
	pc = 0x82DF3A54; continue 'dispatch;
	}
	// 82DF3A44: 3D60A000  lis r11, -0x6000
	ctx.r[11].s64 = -1610612736;
	// 82DF3A48: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF3A4C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DF3A50: 41980008  blt cr6, 0x82df3a58
	if ctx.cr[6].lt {
	pc = 0x82DF3A58; continue 'dispatch;
	}
	// 82DF3A54: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF3A58: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82DF3A5C: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF3A60: 4182000C  beq 0x82df3a6c
	if ctx.cr[0].eq {
	pc = 0x82DF3A6C; continue 'dispatch;
	}
	// 82DF3A64: 909F0000  stw r4, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82DF3A68: 48000010  b 0x82df3a78
	pc = 0x82DF3A78; continue 'dispatch;
	// 82DF3A6C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82DF3A70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF3A74: 4BFFFA65  bl 0x82df34d8
	ctx.lr = 0x82DF3A78;
	sub_82DF34D8(ctx, base);
	// 82DF3A78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF3A7C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF3A80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF3A84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF3A88: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF3A8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF3A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF3A90 size=4
    let mut pc: u32 = 0x82DF3A90;
    'dispatch: loop {
        match pc {
            0x82DF3A90 => {
    //   block [0x82DF3A90..0x82DF3A94)
	// 82DF3A90: 4BFFF9A0  b 0x82df3430
	sub_82DF3430(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF3A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF3A98 size=64
    let mut pc: u32 = 0x82DF3A98;
    'dispatch: loop {
        match pc {
            0x82DF3A98 => {
    //   block [0x82DF3A98..0x82DF3AD8)
	// 82DF3A98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF3A9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF3AA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF3AA4: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82DF3AA8: 4BFFF989  bl 0x82df3430
	ctx.lr = 0x82DF3AAC;
	sub_82DF3430(ctx, base);
	// 82DF3AAC: 7F041800  cmpw cr6, r4, r3
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[3].s32, &mut ctx.xer);
	// 82DF3AB0: 40980014  bge cr6, 0x82df3ac4
	if !ctx.cr[6].lt {
	pc = 0x82DF3AC4; continue 'dispatch;
	}
	// 82DF3AB4: 7D234B78  mr r3, r9
	ctx.r[3].u64 = ctx.r[9].u64;
	// 82DF3AB8: 4BFFF6F9  bl 0x82df31b0
	ctx.lr = 0x82DF3ABC;
	sub_82DF31B0(ctx, base);
	// 82DF3ABC: 7C6320AE  lbzx r3, r3, r4
	ctx.r[3].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[3].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 82DF3AC0: 48000008  b 0x82df3ac8
	pc = 0x82DF3AC8; continue 'dispatch;
	// 82DF3AC4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DF3AC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF3ACC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF3AD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF3AD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF3AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF3AD8 size=208
    let mut pc: u32 = 0x82DF3AD8;
    'dispatch: loop {
        match pc {
            0x82DF3AD8 => {
    //   block [0x82DF3AD8..0x82DF3BA8)
	// 82DF3AD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF3ADC: 483B4691  bl 0x831a816c
	ctx.lr = 0x82DF3AE0;
	sub_831A8130(ctx, base);
	// 82DF3AE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF3AE4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DF3AE8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DF3AEC: 7F1EE840  cmplw cr6, r30, r29
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82DF3AF0: 419A00B0  beq cr6, 0x82df3ba0
	if ctx.cr[6].eq {
	pc = 0x82DF3BA0; continue 'dispatch;
	}
	// 82DF3AF4: 83FD0000  lwz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF3AF8: 4BFFF609  bl 0x82df3100
	ctx.lr = 0x82DF3AFC;
	sub_82DF3100(ctx, base);
	// 82DF3AFC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82DF3B00: 409A000C  bne cr6, 0x82df3b0c
	if !ctx.cr[6].eq {
	pc = 0x82DF3B0C; continue 'dispatch;
	}
	// 82DF3B04: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DF3B08: 48000028  b 0x82df3b30
	pc = 0x82DF3B30; continue 'dispatch;
	// 82DF3B0C: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82DF3B10: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF3B14: 40990014  ble cr6, 0x82df3b28
	if !ctx.cr[6].gt {
	pc = 0x82DF3B28; continue 'dispatch;
	}
	// 82DF3B18: 3D60A000  lis r11, -0x6000
	ctx.r[11].s64 = -1610612736;
	// 82DF3B1C: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF3B20: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DF3B24: 41980008  blt cr6, 0x82df3b2c
	if ctx.cr[6].lt {
	pc = 0x82DF3B2C; continue 'dispatch;
	}
	// 82DF3B28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF3B2C: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82DF3B30: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF3B34: 4182000C  beq 0x82df3b40
	if ctx.cr[0].eq {
	pc = 0x82DF3B40; continue 'dispatch;
	}
	// 82DF3B38: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82DF3B3C: 48000064  b 0x82df3ba0
	pc = 0x82DF3BA0; continue 'dispatch;
	// 82DF3B40: 397FFFFC  addi r11, r31, -4
	ctx.r[11].s64 = ctx.r[31].s64 + -4;
	// 82DF3B44: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82DF3B48: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82DF3B4C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82DF3B50: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DF3B54: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82DF3B58: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82DF3B5C: 4082FFE8  bne 0x82df3b44
	if !ctx.cr[0].eq {
	pc = 0x82DF3B44; continue 'dispatch;
	}
	// 82DF3B60: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DF3B64: 419A0024  beq cr6, 0x82df3b88
	if ctx.cr[6].eq {
	pc = 0x82DF3B88; continue 'dispatch;
	}
	// 82DF3B68: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82DF3B6C: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82DF3B70: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF3B74: 812A5F8C  lwz r9, 0x5f8c(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(24460 as u32) ) } as u64;
	// 82DF3B78: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF3B7C: 40980024  bge cr6, 0x82df3ba0
	if !ctx.cr[6].lt {
	pc = 0x82DF3BA0; continue 'dispatch;
	}
	// 82DF3B80: 916A5F8C  stw r11, 0x5f8c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(24460 as u32), ctx.r[11].u32 ) };
	// 82DF3B84: 4800001C  b 0x82df3ba0
	pc = 0x82DF3BA0; continue 'dispatch;
	// 82DF3B88: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DF3B8C: 4BFFF625  bl 0x82df31b0
	ctx.lr = 0x82DF3B90;
	sub_82DF31B0(ctx, base);
	// 82DF3B90: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DF3B94: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF3B98: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82DF3B9C: 4BFFF93D  bl 0x82df34d8
	ctx.lr = 0x82DF3BA0;
	sub_82DF34D8(ctx, base);
	// 82DF3BA0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF3BA4: 483B4618  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF3BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF3BA8 size=40
    let mut pc: u32 = 0x82DF3BA8;
    'dispatch: loop {
        match pc {
            0x82DF3BA8 => {
    //   block [0x82DF3BA8..0x82DF3BD0)
	// 82DF3BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF3BAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF3BB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF3BB4: 4BFFF87D  bl 0x82df3430
	ctx.lr = 0x82DF3BB8;
	sub_82DF3430(ctx, base);
	// 82DF3BB8: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 82DF3BBC: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82DF3BC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF3BC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF3BC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF3BCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF3BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF3BD0 size=48
    let mut pc: u32 = 0x82DF3BD0;
    'dispatch: loop {
        match pc {
            0x82DF3BD0 => {
    //   block [0x82DF3BD0..0x82DF3C00)
	// 82DF3BD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF3BD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF3BD8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF3BDC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF3BE0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF3BE4: 4BFFFEF5  bl 0x82df3ad8
	ctx.lr = 0x82DF3BE8;
	sub_82DF3AD8(ctx, base);
	// 82DF3BE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF3BEC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF3BF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF3BF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF3BF8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF3BFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF3C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF3C00 size=60
    let mut pc: u32 = 0x82DF3C00;
    'dispatch: loop {
        match pc {
            0x82DF3C00 => {
    //   block [0x82DF3C00..0x82DF3C3C)
	// 82DF3C00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF3C04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF3C08: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF3C0C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF3C10: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF3C14: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DF3C18: 396BB22C  addi r11, r11, -0x4dd4
	ctx.r[11].s64 = ctx.r[11].s64 + -19924;
	// 82DF3C1C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF3C20: 4BFFFEB9  bl 0x82df3ad8
	ctx.lr = 0x82DF3C24;
	sub_82DF3AD8(ctx, base);
	// 82DF3C24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF3C28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF3C2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF3C30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF3C34: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF3C38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF3C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF3C40 size=96
    let mut pc: u32 = 0x82DF3C40;
    'dispatch: loop {
        match pc {
            0x82DF3C40 => {
    //   block [0x82DF3C40..0x82DF3CA0)
	// 82DF3C40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF3C44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF3C48: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DF3C4C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF3C50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF3C54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF3C58: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DF3C5C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DF3C60: 4BFFF551  bl 0x82df31b0
	ctx.lr = 0x82DF3C64;
	sub_82DF31B0(ctx, base);
	// 82DF3C64: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DF3C68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF3C6C: 4BFFFD9D  bl 0x82df3a08
	ctx.lr = 0x82DF3C70;
	sub_82DF3A08(ctx, base);
	// 82DF3C70: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF3C74: 4BFFF53D  bl 0x82df31b0
	ctx.lr = 0x82DF3C78;
	sub_82DF31B0(ctx, base);
	// 82DF3C78: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DF3C7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF3C80: 4BFFF8F9  bl 0x82df3578
	ctx.lr = 0x82DF3C84;
	sub_82DF3578(ctx, base);
	// 82DF3C84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF3C88: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF3C8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF3C90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF3C94: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DF3C98: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF3C9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF3CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF3CA0 size=88
    let mut pc: u32 = 0x82DF3CA0;
    'dispatch: loop {
        match pc {
            0x82DF3CA0 => {
    //   block [0x82DF3CA0..0x82DF3CF8)
	// 82DF3CA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF3CA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF3CA8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DF3CAC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF3CB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF3CB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF3CB8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF3CBC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DF3CC0: 4BFFFD49  bl 0x82df3a08
	ctx.lr = 0x82DF3CC4;
	sub_82DF3A08(ctx, base);
	// 82DF3CC4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DF3CC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF3CCC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DF3CD0: 4BFFFF71  bl 0x82df3c40
	ctx.lr = 0x82DF3CD4;
	sub_82DF3C40(ctx, base);
	// 82DF3CD4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF3CD8: 4BFFF429  bl 0x82df3100
	ctx.lr = 0x82DF3CDC;
	sub_82DF3100(ctx, base);
	// 82DF3CDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF3CE0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF3CE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF3CE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF3CEC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DF3CF0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF3CF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF3CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF3CF8 size=92
    let mut pc: u32 = 0x82DF3CF8;
    'dispatch: loop {
        match pc {
            0x82DF3CF8 => {
    //   block [0x82DF3CF8..0x82DF3D54)
	// 82DF3CF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF3CFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF3D00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DF3D04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF3D08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF3D0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF3D10: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DF3D14: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DF3D18: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF3D1C: 4BFFFCED  bl 0x82df3a08
	ctx.lr = 0x82DF3D20;
	sub_82DF3A08(ctx, base);
	// 82DF3D20: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82DF3D24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF3D28: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DF3D2C: 4BFFFF15  bl 0x82df3c40
	ctx.lr = 0x82DF3D30;
	sub_82DF3C40(ctx, base);
	// 82DF3D30: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF3D34: 4BFFF3CD  bl 0x82df3100
	ctx.lr = 0x82DF3D38;
	sub_82DF3100(ctx, base);
	// 82DF3D38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF3D3C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF3D40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF3D44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF3D48: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DF3D4C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF3D50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF3D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF3D58 size=136
    let mut pc: u32 = 0x82DF3D58;
    'dispatch: loop {
        match pc {
            0x82DF3D58 => {
    //   block [0x82DF3D58..0x82DF3DE0)
	// 82DF3D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF3D5C: 483B4411  bl 0x831a816c
	ctx.lr = 0x82DF3D60;
	sub_831A8130(ctx, base);
	// 82DF3D60: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF3D64: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DF3D68: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DF3D6C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DF3D70: 396BB22C  addi r11, r11, -0x4dd4
	ctx.r[11].s64 = ctx.r[11].s64 + -19924;
	// 82DF3D74: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DF3D78: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF3D7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF3D80: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DF3D84: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF3D88: 4B4D0F59  bl 0x822c4ce0
	ctx.lr = 0x82DF3D8C;
	sub_822C4CE0(ctx, base);
	// 82DF3D8C: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82DF3D90: 419A0014  beq cr6, 0x82df3da4
	if ctx.cr[6].eq {
	pc = 0x82DF3DA4; continue 'dispatch;
	}
	// 82DF3D94: 7CBFF050  subf r5, r31, r30
	ctx.r[5].s64 = ctx.r[30].s64 - ctx.r[31].s64;
	// 82DF3D98: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF3D9C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF3DA0: 4B4D18C9  bl 0x822c5668
	ctx.lr = 0x82DF3DA4;
	sub_822C5668(ctx, base);
	// 82DF3DA4: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82DF3DA8: 80810054  lwz r4, 0x54(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DF3DAC: 2B0B0010  cmplwi cr6, r11, 0x10
	ctx.cr[6].compare_u32(ctx.r[11].u32, 16 as u32, &mut ctx.xer);
	// 82DF3DB0: 40980008  bge cr6, 0x82df3db8
	if !ctx.cr[6].lt {
	pc = 0x82DF3DB8; continue 'dispatch;
	}
	// 82DF3DB4: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82DF3DB8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82DF3DBC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DF3DC0: 4BFFF719  bl 0x82df34d8
	ctx.lr = 0x82DF3DC4;
	sub_82DF34D8(ctx, base);
	// 82DF3DC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF3DC8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DF3DCC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF3DD0: 4B4D0F11  bl 0x822c4ce0
	ctx.lr = 0x82DF3DD4;
	sub_822C4CE0(ctx, base);
	// 82DF3DD4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DF3DD8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DF3DDC: 483B43E0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF3DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF3DE0 size=128
    let mut pc: u32 = 0x82DF3DE0;
    'dispatch: loop {
        match pc {
            0x82DF3DE0 => {
    //   block [0x82DF3DE0..0x82DF3E60)
	// 82DF3DE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF3DE4: 483B4389  bl 0x831a816c
	ctx.lr = 0x82DF3DE8;
	sub_831A8130(ctx, base);
	// 82DF3DE8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF3DEC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF3DF0: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DF3DF4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DF3DF8: 396BB22C  addi r11, r11, -0x4dd4
	ctx.r[11].s64 = ctx.r[11].s64 + -19924;
	// 82DF3DFC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DF3E00: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF3E04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF3E08: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DF3E0C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF3E10: 4B4D0ED1  bl 0x822c4ce0
	ctx.lr = 0x82DF3E14;
	sub_822C4CE0(ctx, base);
	// 82DF3E14: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DF3E18: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DF3E1C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF3E20: 4B4D1849  bl 0x822c5668
	ctx.lr = 0x82DF3E24;
	sub_822C5668(ctx, base);
	// 82DF3E24: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82DF3E28: 80810054  lwz r4, 0x54(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DF3E2C: 2B0B0010  cmplwi cr6, r11, 0x10
	ctx.cr[6].compare_u32(ctx.r[11].u32, 16 as u32, &mut ctx.xer);
	// 82DF3E30: 40980008  bge cr6, 0x82df3e38
	if !ctx.cr[6].lt {
	pc = 0x82DF3E38; continue 'dispatch;
	}
	// 82DF3E34: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82DF3E38: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82DF3E3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF3E40: 4BFFF699  bl 0x82df34d8
	ctx.lr = 0x82DF3E44;
	sub_82DF34D8(ctx, base);
	// 82DF3E44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF3E48: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DF3E4C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF3E50: 4B4D0E91  bl 0x822c4ce0
	ctx.lr = 0x82DF3E54;
	sub_822C4CE0(ctx, base);
	// 82DF3E54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF3E58: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DF3E5C: 483B4360  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF3E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF3E60 size=188
    let mut pc: u32 = 0x82DF3E60;
    'dispatch: loop {
        match pc {
            0x82DF3E60 => {
    //   block [0x82DF3E60..0x82DF3F1C)
	// 82DF3E60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF3E64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF3E68: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DF3E6C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF3E70: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF3E74: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF3E78: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DF3E7C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DF3E80: 396BB22C  addi r11, r11, -0x4dd4
	ctx.r[11].s64 = ctx.r[11].s64 + -19924;
	// 82DF3E84: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DF3E88: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF3E8C: 4BFFF325  bl 0x82df31b0
	ctx.lr = 0x82DF3E90;
	sub_82DF31B0(ctx, base);
	// 82DF3E90: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DF3E94: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82DF3E98: 4B4D1A31  bl 0x822c58c8
	ctx.lr = 0x82DF3E9C;
	sub_822C58C8(ctx, base);
	// 82DF3E9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF3EA0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DF3EA4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF3EA8: 4B4D0E39  bl 0x822c4ce0
	ctx.lr = 0x82DF3EAC;
	sub_822C4CE0(ctx, base);
	// 82DF3EAC: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 82DF3EB0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DF3EB4: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82DF3EB8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF3EBC: 4B4D16C5  bl 0x822c5580
	ctx.lr = 0x82DF3EC0;
	sub_822C5580(ctx, base);
	// 82DF3EC0: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82DF3EC4: 80810054  lwz r4, 0x54(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DF3EC8: 2B0B0010  cmplwi cr6, r11, 0x10
	ctx.cr[6].compare_u32(ctx.r[11].u32, 16 as u32, &mut ctx.xer);
	// 82DF3ECC: 40980008  bge cr6, 0x82df3ed4
	if !ctx.cr[6].lt {
	pc = 0x82DF3ED4; continue 'dispatch;
	}
	// 82DF3ED0: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82DF3ED4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82DF3ED8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF3EDC: 4BFFF5FD  bl 0x82df34d8
	ctx.lr = 0x82DF3EE0;
	sub_82DF34D8(ctx, base);
	// 82DF3EE0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF3EE4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DF3EE8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF3EEC: 4B4D0DF5  bl 0x822c4ce0
	ctx.lr = 0x82DF3EF0;
	sub_822C4CE0(ctx, base);
	// 82DF3EF0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF3EF4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DF3EF8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82DF3EFC: 4B4D0DE5  bl 0x822c4ce0
	ctx.lr = 0x82DF3F00;
	sub_822C4CE0(ctx, base);
	// 82DF3F00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF3F04: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82DF3F08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF3F0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF3F10: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DF3F14: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF3F18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF3F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF3F20 size=168
    let mut pc: u32 = 0x82DF3F20;
    'dispatch: loop {
        match pc {
            0x82DF3F20 => {
    //   block [0x82DF3F20..0x82DF3FC8)
	// 82DF3F20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF3F24: 483B4249  bl 0x831a816c
	ctx.lr = 0x82DF3F28;
	sub_831A8130(ctx, base);
	// 82DF3F28: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF3F2C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF3F30: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DF3F34: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DF3F38: 396BB22C  addi r11, r11, -0x4dd4
	ctx.r[11].s64 = ctx.r[11].s64 + -19924;
	// 82DF3F3C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DF3F40: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF3F44: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82DF3F48: 4BFFF269  bl 0x82df31b0
	ctx.lr = 0x82DF3F4C;
	sub_82DF31B0(ctx, base);
	// 82DF3F4C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DF3F50: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82DF3F54: 4B4D1975  bl 0x822c58c8
	ctx.lr = 0x82DF3F58;
	sub_822C58C8(ctx, base);
	// 82DF3F58: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF3F5C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DF3F60: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF3F64: 4B4D0D7D  bl 0x822c4ce0
	ctx.lr = 0x82DF3F68;
	sub_822C4CE0(ctx, base);
	// 82DF3F68: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82DF3F6C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DF3F70: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82DF3F74: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF3F78: 4B4D1609  bl 0x822c5580
	ctx.lr = 0x82DF3F7C;
	sub_822C5580(ctx, base);
	// 82DF3F7C: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82DF3F80: 80810054  lwz r4, 0x54(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DF3F84: 2B0B0010  cmplwi cr6, r11, 0x10
	ctx.cr[6].compare_u32(ctx.r[11].u32, 16 as u32, &mut ctx.xer);
	// 82DF3F88: 40980008  bge cr6, 0x82df3f90
	if !ctx.cr[6].lt {
	pc = 0x82DF3F90; continue 'dispatch;
	}
	// 82DF3F8C: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82DF3F90: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82DF3F94: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF3F98: 4BFFF541  bl 0x82df34d8
	ctx.lr = 0x82DF3F9C;
	sub_82DF34D8(ctx, base);
	// 82DF3F9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF3FA0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DF3FA4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF3FA8: 4B4D0D39  bl 0x822c4ce0
	ctx.lr = 0x82DF3FAC;
	sub_822C4CE0(ctx, base);
	// 82DF3FAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF3FB0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DF3FB4: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82DF3FB8: 4B4D0D29  bl 0x822c4ce0
	ctx.lr = 0x82DF3FBC;
	sub_822C4CE0(ctx, base);
	// 82DF3FBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF3FC0: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82DF3FC4: 483B41F8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF3FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF3FC8 size=128
    let mut pc: u32 = 0x82DF3FC8;
    'dispatch: loop {
        match pc {
            0x82DF3FC8 => {
    //   block [0x82DF3FC8..0x82DF4048)
	// 82DF3FC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF3FCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF3FD0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF3FD4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF3FD8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DF3FDC: 4BFFF1D5  bl 0x82df31b0
	ctx.lr = 0x82DF3FE0;
	sub_82DF31B0(ctx, base);
	// 82DF3FE0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DF3FE4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF3FE8: 4B4D18E1  bl 0x822c58c8
	ctx.lr = 0x82DF3FEC;
	sub_822C58C8(ctx, base);
	// 82DF3FEC: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82DF3FF0: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF3FF4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DF3FF8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF3FFC: 409AFFF4  bne cr6, 0x82df3ff0
	if !ctx.cr[6].eq {
	pc = 0x82DF3FF0; continue 'dispatch;
	}
	// 82DF4000: 7D7F5850  subf r11, r31, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[31].s64;
	// 82DF4004: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF4008: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DF400C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF4010: 5566003E  slwi r6, r11, 0
	ctx.r[6].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82DF4014: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF4018: 4B4DE4C9  bl 0x822d24e0
	ctx.lr = 0x82DF401C;
	sub_822D24E0(ctx, base);
	// 82DF401C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF4020: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF4024: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DF4028: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF402C: 4B4D0CB5  bl 0x822c4ce0
	ctx.lr = 0x82DF4030;
	sub_822C4CE0(ctx, base);
	// 82DF4030: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF4034: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DF4038: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF403C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF4040: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF4044: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF4048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF4048 size=136
    let mut pc: u32 = 0x82DF4048;
    'dispatch: loop {
        match pc {
            0x82DF4048 => {
    //   block [0x82DF4048..0x82DF40D0)
	// 82DF4048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF404C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF4050: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF4054: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF4058: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DF405C: 4BFFF155  bl 0x82df31b0
	ctx.lr = 0x82DF4060;
	sub_82DF31B0(ctx, base);
	// 82DF4060: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DF4064: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF4068: 4B4D1861  bl 0x822c58c8
	ctx.lr = 0x82DF406C;
	sub_822C58C8(ctx, base);
	// 82DF406C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF4070: 4BFFF141  bl 0x82df31b0
	ctx.lr = 0x82DF4074;
	sub_82DF31B0(ctx, base);
	// 82DF4074: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DF4078: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DF407C: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF4080: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DF4084: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF4088: 409AFFF4  bne cr6, 0x82df407c
	if !ctx.cr[6].eq {
	pc = 0x82DF407C; continue 'dispatch;
	}
	// 82DF408C: 7D645850  subf r11, r4, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[4].s64;
	// 82DF4090: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF4094: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DF4098: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF409C: 5566003E  slwi r6, r11, 0
	ctx.r[6].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82DF40A0: 4B4DE441  bl 0x822d24e0
	ctx.lr = 0x82DF40A4;
	sub_822D24E0(ctx, base);
	// 82DF40A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF40A8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF40AC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DF40B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF40B4: 4B4D0C2D  bl 0x822c4ce0
	ctx.lr = 0x82DF40B8;
	sub_822C4CE0(ctx, base);
	// 82DF40B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF40BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DF40C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF40C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF40C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF40CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF40D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF40D0 size=104
    let mut pc: u32 = 0x82DF40D0;
    'dispatch: loop {
        match pc {
            0x82DF40D0 => {
    //   block [0x82DF40D0..0x82DF4138)
	// 82DF40D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF40D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF40D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF40DC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF40E0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DF40E4: 4BFFF0CD  bl 0x82df31b0
	ctx.lr = 0x82DF40E8;
	sub_82DF31B0(ctx, base);
	// 82DF40E8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DF40EC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DF40F0: 4B4D17D9  bl 0x822c58c8
	ctx.lr = 0x82DF40F4;
	sub_822C58C8(ctx, base);
	// 82DF40F4: 9BE10050  stb r31, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u8 ) };
	// 82DF40F8: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82DF40FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF4100: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DF4104: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DF4108: 4B4DE3D9  bl 0x822d24e0
	ctx.lr = 0x82DF410C;
	sub_822D24E0(ctx, base);
	// 82DF410C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF4110: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF4114: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DF4118: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DF411C: 4B4D0BC5  bl 0x822c4ce0
	ctx.lr = 0x82DF4120;
	sub_822C4CE0(ctx, base);
	// 82DF4120: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF4124: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DF4128: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF412C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF4130: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF4134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF4138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF4138 size=136
    let mut pc: u32 = 0x82DF4138;
    'dispatch: loop {
        match pc {
            0x82DF4138 => {
    //   block [0x82DF4138..0x82DF41C0)
	// 82DF4138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF413C: 483B4029  bl 0x831a8164
	ctx.lr = 0x82DF4140;
	sub_831A8130(ctx, base);
	// 82DF4140: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF4144: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF4148: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DF414C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DF4150: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82DF4154: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82DF4158: 4BFFF059  bl 0x82df31b0
	ctx.lr = 0x82DF415C;
	sub_82DF31B0(ctx, base);
	// 82DF415C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DF4160: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF4164: 4B4D1765  bl 0x822c58c8
	ctx.lr = 0x82DF4168;
	sub_822C58C8(ctx, base);
	// 82DF4168: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82DF416C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DF4170: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DF4174: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DF4178: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF417C: 4B4DEB55  bl 0x822d2cd0
	ctx.lr = 0x82DF4180;
	sub_822D2CD0(ctx, base);
	// 82DF4180: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DF4184: 2B0B0010  cmplwi cr6, r11, 0x10
	ctx.cr[6].compare_u32(ctx.r[11].u32, 16 as u32, &mut ctx.xer);
	// 82DF4188: 4198000C  blt cr6, 0x82df4194
	if ctx.cr[6].lt {
	pc = 0x82DF4194; continue 'dispatch;
	}
	// 82DF418C: 80830004  lwz r4, 4(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF4190: 48000008  b 0x82df4198
	pc = 0x82DF4198; continue 'dispatch;
	// 82DF4194: 38830004  addi r4, r3, 4
	ctx.r[4].s64 = ctx.r[3].s64 + 4;
	// 82DF4198: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82DF419C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF41A0: 4BFFF339  bl 0x82df34d8
	ctx.lr = 0x82DF41A4;
	sub_82DF34D8(ctx, base);
	// 82DF41A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF41A8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DF41AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF41B0: 4B4D0B31  bl 0x822c4ce0
	ctx.lr = 0x82DF41B4;
	sub_822C4CE0(ctx, base);
	// 82DF41B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF41B8: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82DF41BC: 483B3FF8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF41C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF41C0 size=160
    let mut pc: u32 = 0x82DF41C0;
    'dispatch: loop {
        match pc {
            0x82DF41C0 => {
    //   block [0x82DF41C0..0x82DF4260)
	// 82DF41C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF41C4: 483B3FA5  bl 0x831a8168
	ctx.lr = 0x82DF41C8;
	sub_831A8130(ctx, base);
	// 82DF41C8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF41CC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82DF41D0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DF41D4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DF41D8: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82DF41DC: 4BFFEFD5  bl 0x82df31b0
	ctx.lr = 0x82DF41E0;
	sub_82DF31B0(ctx, base);
	// 82DF41E0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DF41E4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF41E8: 4B4D16E1  bl 0x822c58c8
	ctx.lr = 0x82DF41EC;
	sub_822C58C8(ctx, base);
	// 82DF41EC: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82DF41F0: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF41F4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DF41F8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF41FC: 409AFFF4  bne cr6, 0x82df41f0
	if !ctx.cr[6].eq {
	pc = 0x82DF41F0; continue 'dispatch;
	}
	// 82DF4200: 7D7F5850  subf r11, r31, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[31].s64;
	// 82DF4204: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82DF4208: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DF420C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DF4210: 5567003E  slwi r7, r11, 0
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82DF4214: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DF4218: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF421C: 4B4DEAB5  bl 0x822d2cd0
	ctx.lr = 0x82DF4220;
	sub_822D2CD0(ctx, base);
	// 82DF4220: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DF4224: 2B0B0010  cmplwi cr6, r11, 0x10
	ctx.cr[6].compare_u32(ctx.r[11].u32, 16 as u32, &mut ctx.xer);
	// 82DF4228: 4198000C  blt cr6, 0x82df4234
	if ctx.cr[6].lt {
	pc = 0x82DF4234; continue 'dispatch;
	}
	// 82DF422C: 80830004  lwz r4, 4(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF4230: 48000008  b 0x82df4238
	pc = 0x82DF4238; continue 'dispatch;
	// 82DF4234: 38830004  addi r4, r3, 4
	ctx.r[4].s64 = ctx.r[3].s64 + 4;
	// 82DF4238: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82DF423C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DF4240: 4BFFF299  bl 0x82df34d8
	ctx.lr = 0x82DF4244;
	sub_82DF34D8(ctx, base);
	// 82DF4244: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF4248: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DF424C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF4250: 4B4D0A91  bl 0x822c4ce0
	ctx.lr = 0x82DF4254;
	sub_822C4CE0(ctx, base);
	// 82DF4254: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DF4258: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82DF425C: 483B3F5C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF4260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF4260 size=168
    let mut pc: u32 = 0x82DF4260;
    'dispatch: loop {
        match pc {
            0x82DF4260 => {
    //   block [0x82DF4260..0x82DF4308)
	// 82DF4260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF4264: 483B3F05  bl 0x831a8168
	ctx.lr = 0x82DF4268;
	sub_831A8130(ctx, base);
	// 82DF4268: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF426C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DF4270: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DF4274: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DF4278: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82DF427C: 4BFFEF35  bl 0x82df31b0
	ctx.lr = 0x82DF4280;
	sub_82DF31B0(ctx, base);
	// 82DF4280: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DF4284: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF4288: 4B4D1641  bl 0x822c58c8
	ctx.lr = 0x82DF428C;
	sub_822C58C8(ctx, base);
	// 82DF428C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DF4290: 4BFFEF21  bl 0x82df31b0
	ctx.lr = 0x82DF4294;
	sub_82DF31B0(ctx, base);
	// 82DF4294: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82DF4298: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 82DF429C: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF42A0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DF42A4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF42A8: 409AFFF4  bne cr6, 0x82df429c
	if !ctx.cr[6].eq {
	pc = 0x82DF429C; continue 'dispatch;
	}
	// 82DF42AC: 7D665850  subf r11, r6, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[6].s64;
	// 82DF42B0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DF42B4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DF42B8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF42BC: 5567003E  slwi r7, r11, 0
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82DF42C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF42C4: 4B4DEA0D  bl 0x822d2cd0
	ctx.lr = 0x82DF42C8;
	sub_822D2CD0(ctx, base);
	// 82DF42C8: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DF42CC: 2B0B0010  cmplwi cr6, r11, 0x10
	ctx.cr[6].compare_u32(ctx.r[11].u32, 16 as u32, &mut ctx.xer);
	// 82DF42D0: 4198000C  blt cr6, 0x82df42dc
	if ctx.cr[6].lt {
	pc = 0x82DF42DC; continue 'dispatch;
	}
	// 82DF42D4: 80830004  lwz r4, 4(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF42D8: 48000008  b 0x82df42e0
	pc = 0x82DF42E0; continue 'dispatch;
	// 82DF42DC: 38830004  addi r4, r3, 4
	ctx.r[4].s64 = ctx.r[3].s64 + 4;
	// 82DF42E0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82DF42E4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DF42E8: 4BFFF1F1  bl 0x82df34d8
	ctx.lr = 0x82DF42EC;
	sub_82DF34D8(ctx, base);
	// 82DF42EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF42F0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DF42F4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF42F8: 4B4D09E9  bl 0x822c4ce0
	ctx.lr = 0x82DF42FC;
	sub_822C4CE0(ctx, base);
	// 82DF42FC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DF4300: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82DF4304: 483B3EB4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF4308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF4308 size=112
    let mut pc: u32 = 0x82DF4308;
    'dispatch: loop {
        match pc {
            0x82DF4308 => {
    //   block [0x82DF4308..0x82DF4378)
	// 82DF4308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF430C: 483B3E61  bl 0x831a816c
	ctx.lr = 0x82DF4310;
	sub_831A8130(ctx, base);
	// 82DF4310: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF4314: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF4318: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DF431C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DF4320: 4BFFEE91  bl 0x82df31b0
	ctx.lr = 0x82DF4324;
	sub_82DF31B0(ctx, base);
	// 82DF4324: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DF4328: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF432C: 4B4D159D  bl 0x822c58c8
	ctx.lr = 0x82DF4330;
	sub_822C58C8(ctx, base);
	// 82DF4330: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DF4334: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DF4338: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF433C: 4B70C375  bl 0x825006b0
	ctx.lr = 0x82DF4340;
	sub_825006B0(ctx, base);
	// 82DF4340: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82DF4344: 80810054  lwz r4, 0x54(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DF4348: 2B0B0010  cmplwi cr6, r11, 0x10
	ctx.cr[6].compare_u32(ctx.r[11].u32, 16 as u32, &mut ctx.xer);
	// 82DF434C: 40980008  bge cr6, 0x82df4354
	if !ctx.cr[6].lt {
	pc = 0x82DF4354; continue 'dispatch;
	}
	// 82DF4350: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82DF4354: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82DF4358: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF435C: 4BFFF17D  bl 0x82df34d8
	ctx.lr = 0x82DF4360;
	sub_82DF34D8(ctx, base);
	// 82DF4360: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF4364: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DF4368: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF436C: 4B4D0975  bl 0x822c4ce0
	ctx.lr = 0x82DF4370;
	sub_822C4CE0(ctx, base);
	// 82DF4370: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DF4374: 483B3E48  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF4378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF4378 size=128
    let mut pc: u32 = 0x82DF4378;
    'dispatch: loop {
        match pc {
            0x82DF4378 => {
    //   block [0x82DF4378..0x82DF43F8)
	// 82DF4378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF437C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF4380: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF4384: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF4388: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DF438C: 4BFFEE25  bl 0x82df31b0
	ctx.lr = 0x82DF4390;
	sub_82DF31B0(ctx, base);
	// 82DF4390: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DF4394: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF4398: 4B4D1531  bl 0x822c58c8
	ctx.lr = 0x82DF439C;
	sub_822C58C8(ctx, base);
	// 82DF439C: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82DF43A0: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF43A4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DF43A8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF43AC: 409AFFF4  bne cr6, 0x82df43a0
	if !ctx.cr[6].eq {
	pc = 0x82DF43A0; continue 'dispatch;
	}
	// 82DF43B0: 7D7F5850  subf r11, r31, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[31].s64;
	// 82DF43B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82DF43B8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DF43BC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF43C0: 5566003E  slwi r6, r11, 0
	ctx.r[6].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82DF43C4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF43C8: 4B4DFE11  bl 0x822d41d8
	ctx.lr = 0x82DF43CC;
	sub_822D41D8(ctx, base);
	// 82DF43CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF43D0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF43D4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DF43D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF43DC: 4B4D0905  bl 0x822c4ce0
	ctx.lr = 0x82DF43E0;
	sub_822C4CE0(ctx, base);
	// 82DF43E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF43E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DF43E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF43EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF43F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF43F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF43F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF43F8 size=104
    let mut pc: u32 = 0x82DF43F8;
    'dispatch: loop {
        match pc {
            0x82DF43F8 => {
    //   block [0x82DF43F8..0x82DF4460)
	// 82DF43F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF43FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF4400: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF4404: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF4408: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DF440C: 4BFFEDA5  bl 0x82df31b0
	ctx.lr = 0x82DF4410;
	sub_82DF31B0(ctx, base);
	// 82DF4410: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DF4414: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DF4418: 4B4D14B1  bl 0x822c58c8
	ctx.lr = 0x82DF441C;
	sub_822C58C8(ctx, base);
	// 82DF441C: 9BE10050  stb r31, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u8 ) };
	// 82DF4420: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82DF4424: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82DF4428: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DF442C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DF4430: 4B4DFDA9  bl 0x822d41d8
	ctx.lr = 0x82DF4434;
	sub_822D41D8(ctx, base);
	// 82DF4434: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF4438: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF443C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DF4440: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DF4444: 4B4D089D  bl 0x822c4ce0
	ctx.lr = 0x82DF4448;
	sub_822C4CE0(ctx, base);
	// 82DF4448: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF444C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DF4450: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF4454: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF4458: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF445C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF4460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF4460 size=116
    let mut pc: u32 = 0x82DF4460;
    'dispatch: loop {
        match pc {
            0x82DF4460 => {
    //   block [0x82DF4460..0x82DF44D4)
	// 82DF4460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF4464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF4468: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DF446C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF4470: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF4474: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DF4478: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DF447C: 4BFFED35  bl 0x82df31b0
	ctx.lr = 0x82DF4480;
	sub_82DF31B0(ctx, base);
	// 82DF4480: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DF4484: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DF4488: 4B4D1441  bl 0x822c58c8
	ctx.lr = 0x82DF448C;
	sub_822C58C8(ctx, base);
	// 82DF448C: 9BE10050  stb r31, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u8 ) };
	// 82DF4490: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82DF4494: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DF4498: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DF449C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DF44A0: 4B4DE041  bl 0x822d24e0
	ctx.lr = 0x82DF44A4;
	sub_822D24E0(ctx, base);
	// 82DF44A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF44A8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF44AC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DF44B0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DF44B4: 4B4D082D  bl 0x822c4ce0
	ctx.lr = 0x82DF44B8;
	sub_822C4CE0(ctx, base);
	// 82DF44B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF44BC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82DF44C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF44C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF44C8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DF44CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF44D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF44D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF44D8 size=140
    let mut pc: u32 = 0x82DF44D8;
    'dispatch: loop {
        match pc {
            0x82DF44D8 => {
    //   block [0x82DF44D8..0x82DF4564)
	// 82DF44D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF44DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF44E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DF44E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF44E8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF44EC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DF44F0: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DF44F4: 4BFFECBD  bl 0x82df31b0
	ctx.lr = 0x82DF44F8;
	sub_82DF31B0(ctx, base);
	// 82DF44F8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DF44FC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF4500: 4B4D13C9  bl 0x822c58c8
	ctx.lr = 0x82DF4504;
	sub_822C58C8(ctx, base);
	// 82DF4504: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82DF4508: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF450C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DF4510: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF4514: 409AFFF4  bne cr6, 0x82df4508
	if !ctx.cr[6].eq {
	pc = 0x82DF4508; continue 'dispatch;
	}
	// 82DF4518: 7D7F5850  subf r11, r31, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[31].s64;
	// 82DF451C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DF4520: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DF4524: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF4528: 5566003E  slwi r6, r11, 0
	ctx.r[6].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82DF452C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF4530: 4BFFEE39  bl 0x82df3368
	ctx.lr = 0x82DF4534;
	sub_82DF3368(ctx, base);
	// 82DF4534: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF4538: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF453C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DF4540: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF4544: 4B4D079D  bl 0x822c4ce0
	ctx.lr = 0x82DF4548;
	sub_822C4CE0(ctx, base);
	// 82DF4548: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF454C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DF4550: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF4554: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF4558: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DF455C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF4560: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF4568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF4568 size=152
    let mut pc: u32 = 0x82DF4568;
    'dispatch: loop {
        match pc {
            0x82DF4568 => {
    //   block [0x82DF4568..0x82DF4600)
	// 82DF4568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF456C: 483B3C01  bl 0x831a816c
	ctx.lr = 0x82DF4570;
	sub_831A8130(ctx, base);
	// 82DF4570: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF4574: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF4578: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DF457C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DF4580: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82DF4584: 4BFFEC2D  bl 0x82df31b0
	ctx.lr = 0x82DF4588;
	sub_82DF31B0(ctx, base);
	// 82DF4588: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DF458C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82DF4590: 4B4D1339  bl 0x822c58c8
	ctx.lr = 0x82DF4594;
	sub_822C58C8(ctx, base);
	// 82DF4594: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF4598: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DF459C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF45A0: 4B4D0741  bl 0x822c4ce0
	ctx.lr = 0x82DF45A4;
	sub_822C4CE0(ctx, base);
	// 82DF45A4: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82DF45A8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DF45AC: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82DF45B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF45B4: 4B4D0FCD  bl 0x822c5580
	ctx.lr = 0x82DF45B8;
	sub_822C5580(ctx, base);
	// 82DF45B8: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82DF45BC: 80810054  lwz r4, 0x54(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DF45C0: 2B0B0010  cmplwi cr6, r11, 0x10
	ctx.cr[6].compare_u32(ctx.r[11].u32, 16 as u32, &mut ctx.xer);
	// 82DF45C4: 40980008  bge cr6, 0x82df45cc
	if !ctx.cr[6].lt {
	pc = 0x82DF45CC; continue 'dispatch;
	}
	// 82DF45C8: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82DF45CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF45D0: 4BFFF439  bl 0x82df3a08
	ctx.lr = 0x82DF45D4;
	sub_82DF3A08(ctx, base);
	// 82DF45D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF45D8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DF45DC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF45E0: 4B4D0701  bl 0x822c4ce0
	ctx.lr = 0x82DF45E4;
	sub_822C4CE0(ctx, base);
	// 82DF45E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF45E8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DF45EC: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82DF45F0: 4B4D06F1  bl 0x822c4ce0
	ctx.lr = 0x82DF45F4;
	sub_822C4CE0(ctx, base);
	// 82DF45F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF45F8: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82DF45FC: 483B3BC0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF4600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF4600 size=172
    let mut pc: u32 = 0x82DF4600;
    'dispatch: loop {
        match pc {
            0x82DF4600 => {
    //   block [0x82DF4600..0x82DF46AC)
	// 82DF4600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF4604: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF4608: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DF460C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF4610: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF4614: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF4618: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DF461C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DF4620: 4BFFEB91  bl 0x82df31b0
	ctx.lr = 0x82DF4624;
	sub_82DF31B0(ctx, base);
	// 82DF4624: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DF4628: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82DF462C: 4B4D129D  bl 0x822c58c8
	ctx.lr = 0x82DF4630;
	sub_822C58C8(ctx, base);
	// 82DF4630: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF4634: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DF4638: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF463C: 4B4D06A5  bl 0x822c4ce0
	ctx.lr = 0x82DF4640;
	sub_822C4CE0(ctx, base);
	// 82DF4640: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 82DF4644: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DF4648: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82DF464C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF4650: 4B4D0F31  bl 0x822c5580
	ctx.lr = 0x82DF4654;
	sub_822C5580(ctx, base);
	// 82DF4654: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82DF4658: 80810054  lwz r4, 0x54(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DF465C: 2B0B0010  cmplwi cr6, r11, 0x10
	ctx.cr[6].compare_u32(ctx.r[11].u32, 16 as u32, &mut ctx.xer);
	// 82DF4660: 40980008  bge cr6, 0x82df4668
	if !ctx.cr[6].lt {
	pc = 0x82DF4668; continue 'dispatch;
	}
	// 82DF4664: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82DF4668: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF466C: 4BFFF39D  bl 0x82df3a08
	ctx.lr = 0x82DF4670;
	sub_82DF3A08(ctx, base);
	// 82DF4670: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF4674: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DF4678: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF467C: 4B4D0665  bl 0x822c4ce0
	ctx.lr = 0x82DF4680;
	sub_822C4CE0(ctx, base);
	// 82DF4680: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF4684: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DF4688: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82DF468C: 4B4D0655  bl 0x822c4ce0
	ctx.lr = 0x82DF4690;
	sub_822C4CE0(ctx, base);
	// 82DF4690: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF4694: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82DF4698: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF469C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF46A0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DF46A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF46A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF46B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF46B0 size=128
    let mut pc: u32 = 0x82DF46B0;
    'dispatch: loop {
        match pc {
            0x82DF46B0 => {
    //   block [0x82DF46B0..0x82DF4730)
	// 82DF46B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF46B4: 483B3AB5  bl 0x831a8168
	ctx.lr = 0x82DF46B8;
	sub_831A8130(ctx, base);
	// 82DF46B8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF46BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF46C0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DF46C4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DF46C8: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82DF46CC: 4BFFEAE5  bl 0x82df31b0
	ctx.lr = 0x82DF46D0;
	sub_82DF31B0(ctx, base);
	// 82DF46D0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DF46D4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF46D8: 4B4D11F1  bl 0x822c58c8
	ctx.lr = 0x82DF46DC;
	sub_822C58C8(ctx, base);
	// 82DF46DC: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DF46E0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DF46E4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DF46E8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF46EC: 4B9F31BD  bl 0x827e78a8
	ctx.lr = 0x82DF46F0;
	sub_827E78A8(ctx, base);
	// 82DF46F0: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DF46F4: 2B0B0010  cmplwi cr6, r11, 0x10
	ctx.cr[6].compare_u32(ctx.r[11].u32, 16 as u32, &mut ctx.xer);
	// 82DF46F8: 4198000C  blt cr6, 0x82df4704
	if ctx.cr[6].lt {
	pc = 0x82DF4704; continue 'dispatch;
	}
	// 82DF46FC: 80830004  lwz r4, 4(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF4700: 48000008  b 0x82df4708
	pc = 0x82DF4708; continue 'dispatch;
	// 82DF4704: 38830004  addi r4, r3, 4
	ctx.r[4].s64 = ctx.r[3].s64 + 4;
	// 82DF4708: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82DF470C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF4710: 4BFFEDC9  bl 0x82df34d8
	ctx.lr = 0x82DF4714;
	sub_82DF34D8(ctx, base);
	// 82DF4714: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF4718: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DF471C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF4720: 4B4D05C1  bl 0x822c4ce0
	ctx.lr = 0x82DF4724;
	sub_822C4CE0(ctx, base);
	// 82DF4724: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF4728: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82DF472C: 483B3A8C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF4730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF4730 size=132
    let mut pc: u32 = 0x82DF4730;
    'dispatch: loop {
        match pc {
            0x82DF4730 => {
    //   block [0x82DF4730..0x82DF47B4)
	// 82DF4730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF4734: 483B3A39  bl 0x831a816c
	ctx.lr = 0x82DF4738;
	sub_831A8130(ctx, base);
	// 82DF4738: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF473C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DF4740: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DF4744: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82DF4748: 4BFFEA69  bl 0x82df31b0
	ctx.lr = 0x82DF474C;
	sub_82DF31B0(ctx, base);
	// 82DF474C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DF4750: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF4754: 4B4D1175  bl 0x822c58c8
	ctx.lr = 0x82DF4758;
	sub_822C58C8(ctx, base);
	// 82DF4758: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DF475C: 4BFFEA55  bl 0x82df31b0
	ctx.lr = 0x82DF4760;
	sub_82DF31B0(ctx, base);
	// 82DF4760: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82DF4764: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 82DF4768: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF476C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DF4770: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF4774: 409AFFF4  bne cr6, 0x82df4768
	if !ctx.cr[6].eq {
	pc = 0x82DF4768; continue 'dispatch;
	}
	// 82DF4778: 7D665850  subf r11, r6, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[6].s64;
	// 82DF477C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DF4780: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DF4784: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF4788: 5567003E  slwi r7, r11, 0
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82DF478C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF4790: 4B4DDE61  bl 0x822d25f0
	ctx.lr = 0x82DF4794;
	sub_822D25F0(ctx, base);
	// 82DF4794: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF4798: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF479C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DF47A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF47A4: 4B4D053D  bl 0x822c4ce0
	ctx.lr = 0x82DF47A8;
	sub_822C4CE0(ctx, base);
	// 82DF47A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF47AC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DF47B0: 483B3A0C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF47B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF47B8 size=64
    let mut pc: u32 = 0x82DF47B8;
    'dispatch: loop {
        match pc {
            0x82DF47B8 => {
    //   block [0x82DF47B8..0x82DF47F8)
	// 82DF47B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF47BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF47C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF47C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF47C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF47CC: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DF47D0: 4BFFE9E1  bl 0x82df31b0
	ctx.lr = 0x82DF47D4;
	sub_82DF31B0(ctx, base);
	// 82DF47D4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DF47D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF47DC: 4B4D10ED  bl 0x822c58c8
	ctx.lr = 0x82DF47E0;
	sub_822C58C8(ctx, base);
	// 82DF47E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF47E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF47E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF47EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF47F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF47F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF47F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF47F8 size=116
    let mut pc: u32 = 0x82DF47F8;
    'dispatch: loop {
        match pc {
            0x82DF47F8 => {
    //   block [0x82DF47F8..0x82DF486C)
	// 82DF47F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF47FC: 483B3971  bl 0x831a816c
	ctx.lr = 0x82DF4800;
	sub_831A8130(ctx, base);
	// 82DF4800: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF4804: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF4808: 4BFFF3F9  bl 0x82df3c00
	ctx.lr = 0x82DF480C;
	sub_82DF3C00(ctx, base);
	// 82DF480C: 3880002F  li r4, 0x2f
	ctx.r[4].s64 = 47;
	// 82DF4810: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF4814: 4BFFF8BD  bl 0x82df40d0
	ctx.lr = 0x82DF4818;
	sub_82DF40D0(ctx, base);
	// 82DF4818: 3FC08212  lis r30, -0x7dee
	ctx.r[30].s64 = -2112749568;
	// 82DF481C: 817EB230  lwz r11, -0x4dd0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-19920 as u32) ) } as u64;
	// 82DF4820: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DF4824: 419A003C  beq cr6, 0x82df4860
	if ctx.cr[6].eq {
	pc = 0x82DF4860; continue 'dispatch;
	}
	// 82DF4828: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DF482C: 3BABB234  addi r29, r11, -0x4dcc
	ctx.r[29].s64 = ctx.r[11].s64 + -19916;
	// 82DF4830: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DF4834: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82DF4838: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82DF483C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DF4840: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF4844: 4BFFF8F5  bl 0x82df4138
	ctx.lr = 0x82DF4848;
	sub_82DF4138(ctx, base);
	// 82DF4848: 3880002F  li r4, 0x2f
	ctx.r[4].s64 = 47;
	// 82DF484C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF4850: 4BFFF881  bl 0x82df40d0
	ctx.lr = 0x82DF4854;
	sub_82DF40D0(ctx, base);
	// 82DF4854: 817EB230  lwz r11, -0x4dd0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-19920 as u32) ) } as u64;
	// 82DF4858: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DF485C: 409AFFD4  bne cr6, 0x82df4830
	if !ctx.cr[6].eq {
	pc = 0x82DF4830; continue 'dispatch;
	}
	// 82DF4860: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF4864: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF4868: 483B3954  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF4870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF4870 size=372
    let mut pc: u32 = 0x82DF4870;
    'dispatch: loop {
        match pc {
            0x82DF4870 => {
    //   block [0x82DF4870..0x82DF49E4)
	// 82DF4870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF4874: 483B38F5  bl 0x831a8168
	ctx.lr = 0x82DF4878;
	sub_831A8130(ctx, base);
	// 82DF4878: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF487C: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82DF4880: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DF4884: 3BEB5F90  addi r31, r11, 0x5f90
	ctx.r[31].s64 = ctx.r[11].s64 + 24464;
	// 82DF4888: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DF488C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF4890: 4BFFF341  bl 0x82df3bd0
	ctx.lr = 0x82DF4894;
	sub_82DF3BD0(ctx, base);
	// 82DF4894: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF4898: 4BFFF311  bl 0x82df3ba8
	ctx.lr = 0x82DF489C;
	sub_82DF3BA8(ctx, base);
	// 82DF489C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF48A0: 40820118  bne 0x82df49b8
	if !ctx.cr[0].eq {
	pc = 0x82DF49B8; continue 'dispatch;
	}
	// 82DF48A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF48A8: 4BFFF1E9  bl 0x82df3a90
	ctx.lr = 0x82DF48AC;
	sub_82DF3A90(ctx, base);
	// 82DF48AC: 38A3FFFF  addi r5, r3, -1
	ctx.r[5].s64 = ctx.r[3].s64 + -1;
	// 82DF48B0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF48B4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF48B8: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 82DF48BC: 4BFFFCAD  bl 0x82df4568
	ctx.lr = 0x82DF48C0;
	sub_82DF4568(ctx, base);
	// 82DF48C0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DF48C4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82DF48C8: 3B8B043C  addi r28, r11, 0x43c
	ctx.r[28].s64 = ctx.r[11].s64 + 1084;
	// 82DF48CC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DF48D0: 4BFFF139  bl 0x82df3a08
	ctx.lr = 0x82DF48D4;
	sub_82DF3A08(ctx, base);
	// 82DF48D4: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82DF48D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF48DC: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82DF48E0: 4BFFE9C1  bl 0x82df32a0
	ctx.lr = 0x82DF48E4;
	sub_82DF32A0(ctx, base);
	// 82DF48E4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF48E8: 41820030  beq 0x82df4918
	if ctx.cr[0].eq {
	pc = 0x82DF4918; continue 'dispatch;
	}
	// 82DF48EC: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DF48F0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82DF48F4: 388BB234  addi r4, r11, -0x4dcc
	ctx.r[4].s64 = ctx.r[11].s64 + -19916;
	// 82DF48F8: 4BFFF111  bl 0x82df3a08
	ctx.lr = 0x82DF48FC;
	sub_82DF3A08(ctx, base);
	// 82DF48FC: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82DF4900: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF4904: 3BC00003  li r30, 3
	ctx.r[30].s64 = 3;
	// 82DF4908: 4BFFE999  bl 0x82df32a0
	ctx.lr = 0x82DF490C;
	sub_82DF32A0(ctx, base);
	// 82DF490C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF4910: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DF4914: 40820008  bne 0x82df491c
	if !ctx.cr[0].eq {
	pc = 0x82DF491C; continue 'dispatch;
	}
	// 82DF4918: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF491C: 57CA07BD  rlwinm. r10, r30, 0, 0x1e, 0x1e
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DF4920: 557D063E  clrlwi r29, r11, 0x18
	ctx.r[29].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82DF4924: 41820010  beq 0x82df4934
	if ctx.cr[0].eq {
	pc = 0x82DF4934; continue 'dispatch;
	}
	// 82DF4928: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82DF492C: 57DE07FA  rlwinm r30, r30, 0, 0x1f, 0x1d
	ctx.r[30].u64 = ctx.r[30].u32 as u64 & 0xFFFFFFFFu64;
	// 82DF4930: 4BFFEAF9  bl 0x82df3428
	ctx.lr = 0x82DF4934;
	sub_82DF3428(ctx, base);
	// 82DF4934: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF4938: 4182000C  beq 0x82df4944
	if ctx.cr[0].eq {
	pc = 0x82DF4944; continue 'dispatch;
	}
	// 82DF493C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82DF4940: 4BFFEAE9  bl 0x82df3428
	ctx.lr = 0x82DF4944;
	sub_82DF3428(ctx, base);
	// 82DF4944: 57AB063F  clrlwi. r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF4948: 41820024  beq 0x82df496c
	if ctx.cr[0].eq {
	pc = 0x82DF496C; continue 'dispatch;
	}
	// 82DF494C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DF4950: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82DF4954: 4BFFF0B5  bl 0x82df3a08
	ctx.lr = 0x82DF4958;
	sub_82DF3A08(ctx, base);
	// 82DF4958: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF495C: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82DF4960: 4BFFEF51  bl 0x82df38b0
	ctx.lr = 0x82DF4964;
	sub_82DF38B0(ctx, base);
	// 82DF4964: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82DF4968: 4BFFEAC1  bl 0x82df3428
	ctx.lr = 0x82DF496C;
	sub_82DF3428(ctx, base);
	// 82DF496C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF4970: 3880005C  li r4, 0x5c
	ctx.r[4].s64 = 92;
	// 82DF4974: 4BFFF75D  bl 0x82df40d0
	ctx.lr = 0x82DF4978;
	sub_82DF40D0(ctx, base);
	// 82DF4978: 3FC08212  lis r30, -0x7dee
	ctx.r[30].s64 = -2112749568;
	// 82DF497C: 48000024  b 0x82df49a0
	pc = 0x82DF49A0; continue 'dispatch;
	// 82DF4980: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DF4984: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF4988: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82DF498C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DF4990: 4BFFF7A9  bl 0x82df4138
	ctx.lr = 0x82DF4994;
	sub_82DF4138(ctx, base);
	// 82DF4994: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF4998: 3880005C  li r4, 0x5c
	ctx.r[4].s64 = 92;
	// 82DF499C: 4BFFF735  bl 0x82df40d0
	ctx.lr = 0x82DF49A0;
	sub_82DF40D0(ctx, base);
	// 82DF49A0: 817EB230  lwz r11, -0x4dd0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-19920 as u32) ) } as u64;
	// 82DF49A4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DF49A8: 7F045800  cmpw cr6, r4, r11
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DF49AC: 409AFFD4  bne cr6, 0x82df4980
	if !ctx.cr[6].eq {
	pc = 0x82DF4980; continue 'dispatch;
	}
	// 82DF49B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF49B4: 4BFFEA75  bl 0x82df3428
	ctx.lr = 0x82DF49B8;
	sub_82DF3428(ctx, base);
	// 82DF49B8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF49BC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82DF49C0: 4BFFFE39  bl 0x82df47f8
	ctx.lr = 0x82DF49C4;
	sub_82DF47F8(ctx, base);
	// 82DF49C4: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82DF49C8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DF49CC: 386B5F94  addi r3, r11, 0x5f94
	ctx.r[3].s64 = ctx.r[11].s64 + 24468;
	// 82DF49D0: 4BFFF201  bl 0x82df3bd0
	ctx.lr = 0x82DF49D4;
	sub_82DF3BD0(ctx, base);
	// 82DF49D4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82DF49D8: 4BFFEA51  bl 0x82df3428
	ctx.lr = 0x82DF49DC;
	sub_82DF3428(ctx, base);
	// 82DF49DC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DF49E0: 483B37D8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF49E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF49E8 size=128
    let mut pc: u32 = 0x82DF49E8;
    'dispatch: loop {
        match pc {
            0x82DF49E8 => {
    //   block [0x82DF49E8..0x82DF4A68)
	// 82DF49E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF49EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF49F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF49F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF49F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF49FC: 4BFFF1AD  bl 0x82df3ba8
	ctx.lr = 0x82DF4A00;
	sub_82DF3BA8(ctx, base);
	// 82DF4A00: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF4A04: 4182000C  beq 0x82df4a10
	if ctx.cr[0].eq {
	pc = 0x82DF4A10; continue 'dispatch;
	}
	// 82DF4A08: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DF4A0C: 48000048  b 0x82df4a54
	pc = 0x82DF4A54; continue 'dispatch;
	// 82DF4A10: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DF4A14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF4A18: 4BFFF081  bl 0x82df3a98
	ctx.lr = 0x82DF4A1C;
	sub_82DF3A98(ctx, base);
	// 82DF4A1C: 7C6B0774  extsb r11, r3
	ctx.r[11].s64 = ctx.r[3].s8 as i64;
	// 82DF4A20: 2F0B002F  cmpwi cr6, r11, 0x2f
	ctx.cr[6].compare_i32(ctx.r[11].s32, 47, &mut ctx.xer);
	// 82DF4A24: 409A000C  bne cr6, 0x82df4a30
	if !ctx.cr[6].eq {
	pc = 0x82DF4A30; continue 'dispatch;
	}
	// 82DF4A28: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DF4A2C: 48000028  b 0x82df4a54
	pc = 0x82DF4A54; continue 'dispatch;
	// 82DF4A30: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82DF4A34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF4A38: 388B2298  addi r4, r11, 0x2298
	ctx.r[4].s64 = ctx.r[11].s64 + 8856;
	// 82DF4A3C: 4BFFF58D  bl 0x82df3fc8
	ctx.lr = 0x82DF4A40;
	sub_82DF3FC8(ctx, base);
	// 82DF4A40: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DF4A44: 816BB230  lwz r11, -0x4dd0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19920 as u32) ) } as u64;
	// 82DF4A48: 7D635850  subf r11, r3, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 82DF4A4C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82DF4A50: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82DF4A54: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF4A58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF4A5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF4A60: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF4A64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF4A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF4A68 size=152
    let mut pc: u32 = 0x82DF4A68;
    'dispatch: loop {
        match pc {
            0x82DF4A68 => {
    //   block [0x82DF4A68..0x82DF4B00)
	// 82DF4A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF4A6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF4A70: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DF4A74: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF4A78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF4A7C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DF4A80: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF4A84: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF4A88: 4BFFFF61  bl 0x82df49e8
	ctx.lr = 0x82DF4A8C;
	sub_82DF49E8(ctx, base);
	// 82DF4A8C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF4A90: 40820014  bne 0x82df4aa4
	if !ctx.cr[0].eq {
	pc = 0x82DF4AA4; continue 'dispatch;
	}
	// 82DF4A94: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DF4A98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF4A9C: 4BFFF165  bl 0x82df3c00
	ctx.lr = 0x82DF4AA0;
	sub_82DF3C00(ctx, base);
	// 82DF4AA0: 48000044  b 0x82df4ae4
	pc = 0x82DF4AE4; continue 'dispatch;
	// 82DF4AA4: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82DF4AA8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF4AAC: 388B5F94  addi r4, r11, 0x5f94
	ctx.r[4].s64 = ctx.r[11].s64 + 24468;
	// 82DF4AB0: 4BFFF599  bl 0x82df4048
	ctx.lr = 0x82DF4AB4;
	sub_82DF4048(ctx, base);
	// 82DF4AB4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82DF4AB8: 4182FFDC  beq 0x82df4a94
	if ctx.cr[0].eq {
	pc = 0x82DF4A94; continue 'dispatch;
	}
	// 82DF4ABC: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82DF4AC0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DF4AC4: 388B5F90  addi r4, r11, 0x5f90
	ctx.r[4].s64 = ctx.r[11].s64 + 24464;
	// 82DF4AC8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF4ACC: 4BFFF175  bl 0x82df3c40
	ctx.lr = 0x82DF4AD0;
	sub_82DF3C40(ctx, base);
	// 82DF4AD0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DF4AD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF4AD8: 4BFFFD21  bl 0x82df47f8
	ctx.lr = 0x82DF4ADC;
	sub_82DF47F8(ctx, base);
	// 82DF4ADC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF4AE0: 4BFFE949  bl 0x82df3428
	ctx.lr = 0x82DF4AE4;
	sub_82DF3428(ctx, base);
	// 82DF4AE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF4AE8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF4AEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF4AF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF4AF4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DF4AF8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF4AFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF4B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF4B00 size=72
    let mut pc: u32 = 0x82DF4B00;
    'dispatch: loop {
        match pc {
            0x82DF4B00 => {
    //   block [0x82DF4B00..0x82DF4B48)
	// 82DF4B00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF4B04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF4B08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF4B0C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82DF4B10: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82DF4B14: 388BBE28  addi r4, r11, -0x41d8
	ctx.r[4].s64 = ctx.r[11].s64 + -16856;
	// 82DF4B18: 38A0001E  li r5, 0x1e
	ctx.r[5].s64 = 30;
	// 82DF4B1C: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 82DF4B20: 4BFFD8C9  bl 0x82df23e8
	ctx.lr = 0x82DF4B24;
	sub_82DF23E8(ctx, base);
	// 82DF4B24: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DF4B28: 4182000C  beq 0x82df4b34
	if ctx.cr[0].eq {
	pc = 0x82DF4B34; continue 'dispatch;
	}
	// 82DF4B2C: 4800686D  bl 0x82dfb398
	ctx.lr = 0x82DF4B30;
	sub_82DFB398(ctx, base);
	// 82DF4B30: 48000008  b 0x82df4b38
	pc = 0x82DF4B38; continue 'dispatch;
	// 82DF4B34: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DF4B38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF4B3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF4B40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF4B44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF4B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF4B48 size=72
    let mut pc: u32 = 0x82DF4B48;
    'dispatch: loop {
        match pc {
            0x82DF4B48 => {
    //   block [0x82DF4B48..0x82DF4B90)
	// 82DF4B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF4B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF4B50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF4B54: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82DF4B58: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82DF4B5C: 388BBE28  addi r4, r11, -0x41d8
	ctx.r[4].s64 = ctx.r[11].s64 + -16856;
	// 82DF4B60: 38A0001E  li r5, 0x1e
	ctx.r[5].s64 = 30;
	// 82DF4B64: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82DF4B68: 4BFFD881  bl 0x82df23e8
	ctx.lr = 0x82DF4B6C;
	sub_82DF23E8(ctx, base);
	// 82DF4B6C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DF4B70: 4182000C  beq 0x82df4b7c
	if ctx.cr[0].eq {
	pc = 0x82DF4B7C; continue 'dispatch;
	}
	// 82DF4B74: 480069DD  bl 0x82dfb550
	ctx.lr = 0x82DF4B78;
	sub_82DFB550(ctx, base);
	// 82DF4B78: 48000008  b 0x82df4b80
	pc = 0x82DF4B80; continue 'dispatch;
	// 82DF4B7C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DF4B80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF4B84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF4B88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF4B8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF4B90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF4B90 size=88
    let mut pc: u32 = 0x82DF4B90;
    'dispatch: loop {
        match pc {
            0x82DF4B90 => {
    //   block [0x82DF4B90..0x82DF4BE8)
	// 82DF4B90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF4B94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF4B98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF4B9C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF4BA0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82DF4BA4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DF4BA8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82DF4BAC: 388BBE28  addi r4, r11, -0x41d8
	ctx.r[4].s64 = ctx.r[11].s64 + -16856;
	// 82DF4BB0: 38A00073  li r5, 0x73
	ctx.r[5].s64 = 115;
	// 82DF4BB4: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82DF4BB8: 4BFFD831  bl 0x82df23e8
	ctx.lr = 0x82DF4BBC;
	sub_82DF23E8(ctx, base);
	// 82DF4BBC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DF4BC0: 41820010  beq 0x82df4bd0
	if ctx.cr[0].eq {
	pc = 0x82DF4BD0; continue 'dispatch;
	}
	// 82DF4BC4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF4BC8: 48006A69  bl 0x82dfb630
	ctx.lr = 0x82DF4BCC;
	sub_82DFB630(ctx, base);
	// 82DF4BCC: 48000008  b 0x82df4bd4
	pc = 0x82DF4BD4; continue 'dispatch;
	// 82DF4BD0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DF4BD4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF4BD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF4BDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF4BE0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF4BE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF4BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF4BE8 size=104
    let mut pc: u32 = 0x82DF4BE8;
    'dispatch: loop {
        match pc {
            0x82DF4BE8 => {
    //   block [0x82DF4BE8..0x82DF4C50)
	// 82DF4BE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF4BEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF4BF0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DF4BF4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF4BF8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF4BFC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82DF4C00: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DF4C04: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DF4C08: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82DF4C0C: 388BBE28  addi r4, r11, -0x41d8
	ctx.r[4].s64 = ctx.r[11].s64 + -16856;
	// 82DF4C10: 38A00073  li r5, 0x73
	ctx.r[5].s64 = 115;
	// 82DF4C14: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 82DF4C18: 4BFFD7D1  bl 0x82df23e8
	ctx.lr = 0x82DF4C1C;
	sub_82DF23E8(ctx, base);
	// 82DF4C1C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DF4C20: 41820014  beq 0x82df4c34
	if ctx.cr[0].eq {
	pc = 0x82DF4C34; continue 'dispatch;
	}
	// 82DF4C24: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DF4C28: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF4C2C: 4BFFDC1D  bl 0x82df2848
	ctx.lr = 0x82DF4C30;
	sub_82DF2848(ctx, base);
	// 82DF4C30: 48000008  b 0x82df4c38
	pc = 0x82DF4C38; continue 'dispatch;
	// 82DF4C34: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DF4C38: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF4C3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF4C40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF4C44: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DF4C48: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF4C4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF4C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF4C50 size=72
    let mut pc: u32 = 0x82DF4C50;
    'dispatch: loop {
        match pc {
            0x82DF4C50 => {
    //   block [0x82DF4C50..0x82DF4C98)
	// 82DF4C50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF4C54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF4C58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF4C5C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82DF4C60: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82DF4C64: 388BBE28  addi r4, r11, -0x41d8
	ctx.r[4].s64 = ctx.r[11].s64 + -16856;
	// 82DF4C68: 38A0001E  li r5, 0x1e
	ctx.r[5].s64 = 30;
	// 82DF4C6C: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 82DF4C70: 4BFFD779  bl 0x82df23e8
	ctx.lr = 0x82DF4C74;
	sub_82DF23E8(ctx, base);
	// 82DF4C74: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DF4C78: 4182000C  beq 0x82df4c84
	if ctx.cr[0].eq {
	pc = 0x82DF4C84; continue 'dispatch;
	}
	// 82DF4C7C: 48006CD5  bl 0x82dfb950
	ctx.lr = 0x82DF4C80;
	sub_82DFB950(ctx, base);
	// 82DF4C80: 48000008  b 0x82df4c88
	pc = 0x82DF4C88; continue 'dispatch;
	// 82DF4C84: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DF4C88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF4C8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF4C90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF4C94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF4C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF4C98 size=80
    let mut pc: u32 = 0x82DF4C98;
    'dispatch: loop {
        match pc {
            0x82DF4C98 => {
    //   block [0x82DF4C98..0x82DF4CE8)
	// 82DF4C98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF4C9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF4CA0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF4CA4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF4CA8: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82DF4CAC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF4CB0: 816B6C9C  lwz r11, 0x6c9c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27804 as u32) ) } as u64;
	// 82DF4CB4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF4CB8: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 82DF4CBC: 409A0008  bne cr6, 0x82df4cc4
	if !ctx.cr[6].eq {
	pc = 0x82DF4CC4; continue 'dispatch;
	}
	// 82DF4CC0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DF4CC4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DF4CC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF4CCC: 4BFFCF2D  bl 0x82df1bf8
	ctx.lr = 0x82DF4CD0;
	sub_82DF1BF8(ctx, base);
	// 82DF4CD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF4CD4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF4CD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF4CDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF4CE0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF4CE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF4CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF4CE8 size=80
    let mut pc: u32 = 0x82DF4CE8;
    'dispatch: loop {
        match pc {
            0x82DF4CE8 => {
    //   block [0x82DF4CE8..0x82DF4D38)
	// 82DF4CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF4CEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF4CF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF4CF4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82DF4CF8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82DF4CFC: 388BBE28  addi r4, r11, -0x41d8
	ctx.r[4].s64 = ctx.r[11].s64 + -16856;
	// 82DF4D00: 38A0001E  li r5, 0x1e
	ctx.r[5].s64 = 30;
	// 82DF4D04: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 82DF4D08: 4BFFD6E1  bl 0x82df23e8
	ctx.lr = 0x82DF4D0C;
	sub_82DF23E8(ctx, base);
	// 82DF4D0C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DF4D10: 41820014  beq 0x82df4d24
	if ctx.cr[0].eq {
	pc = 0x82DF4D24; continue 'dispatch;
	}
	// 82DF4D14: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DF4D18: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 82DF4D1C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF4D20: 48000008  b 0x82df4d28
	pc = 0x82DF4D28; continue 'dispatch;
	// 82DF4D24: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DF4D28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF4D2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF4D30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF4D34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF4D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF4D38 size=84
    let mut pc: u32 = 0x82DF4D38;
    'dispatch: loop {
        match pc {
            0x82DF4D38 => {
    //   block [0x82DF4D38..0x82DF4D8C)
	// 82DF4D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF4D3C: 483B3431  bl 0x831a816c
	ctx.lr = 0x82DF4D40;
	sub_831A8130(ctx, base);
	// 82DF4D40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF4D44: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DF4D48: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DF4D4C: 897F0019  lbz r11, 0x19(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(25 as u32) ) } as u64;
	// 82DF4D50: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF4D54: 409A0030  bne cr6, 0x82df4d84
	if !ctx.cr[6].eq {
	pc = 0x82DF4D84; continue 'dispatch;
	}
	// 82DF4D58: 3FA08335  lis r29, -0x7ccb
	ctx.r[29].s64 = -2093678592;
	// 82DF4D5C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF4D60: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF4D64: 4BFFFFD5  bl 0x82df4d38
	ctx.lr = 0x82DF4D68;
	sub_82DF4D38(ctx, base);
	// 82DF4D68: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF4D6C: 807D110C  lwz r3, 0x110c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82DF4D70: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF4D74: 4BFFD415  bl 0x82df2188
	ctx.lr = 0x82DF4D78;
	sub_82DF2188(ctx, base);
	// 82DF4D78: 897F0019  lbz r11, 0x19(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(25 as u32) ) } as u64;
	// 82DF4D7C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF4D80: 419AFFDC  beq cr6, 0x82df4d5c
	if ctx.cr[6].eq {
	pc = 0x82DF4D5C; continue 'dispatch;
	}
	// 82DF4D84: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF4D88: 483B3434  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF4D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF4D90 size=84
    let mut pc: u32 = 0x82DF4D90;
    'dispatch: loop {
        match pc {
            0x82DF4D90 => {
    //   block [0x82DF4D90..0x82DF4DE4)
	// 82DF4D90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF4D94: 483B33D9  bl 0x831a816c
	ctx.lr = 0x82DF4D98;
	sub_831A8130(ctx, base);
	// 82DF4D98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF4D9C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DF4DA0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DF4DA4: 897F0015  lbz r11, 0x15(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(21 as u32) ) } as u64;
	// 82DF4DA8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF4DAC: 409A0030  bne cr6, 0x82df4ddc
	if !ctx.cr[6].eq {
	pc = 0x82DF4DDC; continue 'dispatch;
	}
	// 82DF4DB0: 3FA08335  lis r29, -0x7ccb
	ctx.r[29].s64 = -2093678592;
	// 82DF4DB4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF4DB8: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF4DBC: 4BFFFFD5  bl 0x82df4d90
	ctx.lr = 0x82DF4DC0;
	sub_82DF4D90(ctx, base);
	// 82DF4DC0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF4DC4: 807D110C  lwz r3, 0x110c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82DF4DC8: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF4DCC: 4BFFD3BD  bl 0x82df2188
	ctx.lr = 0x82DF4DD0;
	sub_82DF2188(ctx, base);
	// 82DF4DD0: 897F0015  lbz r11, 0x15(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(21 as u32) ) } as u64;
	// 82DF4DD4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF4DD8: 419AFFDC  beq cr6, 0x82df4db4
	if ctx.cr[6].eq {
	pc = 0x82DF4DB4; continue 'dispatch;
	}
	// 82DF4DDC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF4DE0: 483B33DC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF4DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF4DE8 size=84
    let mut pc: u32 = 0x82DF4DE8;
    'dispatch: loop {
        match pc {
            0x82DF4DE8 => {
    //   block [0x82DF4DE8..0x82DF4E3C)
	// 82DF4DE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF4DEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF4DF0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF4DF4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF4DF8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF4DFC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF4E00: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF4E04: 4BFFFF35  bl 0x82df4d38
	ctx.lr = 0x82DF4E08;
	sub_82DF4D38(ctx, base);
	// 82DF4E08: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF4E0C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF4E10: 914A0004  stw r10, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DF4E14: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DF4E18: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF4E1C: 914A0000  stw r10, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DF4E20: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF4E24: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DF4E28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF4E2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF4E30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF4E34: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF4E38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF4E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF4E40 size=84
    let mut pc: u32 = 0x82DF4E40;
    'dispatch: loop {
        match pc {
            0x82DF4E40 => {
    //   block [0x82DF4E40..0x82DF4E94)
	// 82DF4E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF4E44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF4E48: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF4E4C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF4E50: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF4E54: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF4E58: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF4E5C: 4BFFFF35  bl 0x82df4d90
	ctx.lr = 0x82DF4E60;
	sub_82DF4D90(ctx, base);
	// 82DF4E60: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF4E64: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF4E68: 914A0004  stw r10, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DF4E6C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DF4E70: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF4E74: 914A0000  stw r10, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DF4E78: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF4E7C: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DF4E80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF4E84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF4E88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF4E8C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF4E90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF4E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF4E98 size=640
    let mut pc: u32 = 0x82DF4E98;
    'dispatch: loop {
        match pc {
            0x82DF4E98 => {
    //   block [0x82DF4E98..0x82DF5118)
	// 82DF4E98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF4E9C: 483B32CD  bl 0x831a8168
	ctx.lr = 0x82DF4EA0;
	sub_831A8130(ctx, base);
	// 82DF4EA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF4EA4: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82DF4EA8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DF4EAC: 3BEB5FE4  addi r31, r11, 0x5fe4
	ctx.r[31].s64 = ctx.r[11].s64 + 24548;
	// 82DF4EB0: 549C063F  clrlwi. r28, r4, 0x18
	ctx.r[28].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82DF4EB4: 816B5FE4  lwz r11, 0x5fe4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24548 as u32) ) } as u64;
	// 82DF4EB8: 41820038  beq 0x82df4ef0
	if ctx.cr[0].eq {
	pc = 0x82DF4EF0; continue 'dispatch;
	}
	// 82DF4EBC: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82DF4EC0: 3BCA5FDC  addi r30, r10, 0x5fdc
	ctx.r[30].s64 = ctx.r[10].s64 + 24540;
	// 82DF4EC4: 556A07FF  clrlwi. r10, r11, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DF4EC8: 40820064  bne 0x82df4f2c
	if !ctx.cr[0].eq {
	pc = 0x82DF4F2C; continue 'dispatch;
	}
	// 82DF4ECC: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 82DF4ED0: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 82DF4ED4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF4ED8: 4BFFD601  bl 0x82df24d8
	ctx.lr = 0x82DF4EDC;
	sub_82DF24D8(ctx, base);
	// 82DF4EDC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DF4EE0: 3D408324  lis r10, -0x7cdc
	ctx.r[10].s64 = -2094792704;
	// 82DF4EE4: 396B03C4  addi r11, r11, 0x3c4
	ctx.r[11].s64 = ctx.r[11].s64 + 964;
	// 82DF4EE8: 386A1C58  addi r3, r10, 0x1c58
	ctx.r[3].s64 = ctx.r[10].s64 + 7256;
	// 82DF4EEC: 48000034  b 0x82df4f20
	pc = 0x82DF4F20; continue 'dispatch;
	// 82DF4EF0: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82DF4EF4: 3BCA5FD4  addi r30, r10, 0x5fd4
	ctx.r[30].s64 = ctx.r[10].s64 + 24532;
	// 82DF4EF8: 556A07BD  rlwinm. r10, r11, 0, 0x1e, 0x1e
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DF4EFC: 40820030  bne 0x82df4f2c
	if !ctx.cr[0].eq {
	pc = 0x82DF4F2C; continue 'dispatch;
	}
	// 82DF4F00: 616B0002  ori r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 2;
	// 82DF4F04: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 82DF4F08: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF4F0C: 4BFFD5CD  bl 0x82df24d8
	ctx.lr = 0x82DF4F10;
	sub_82DF24D8(ctx, base);
	// 82DF4F10: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 82DF4F14: 3D408324  lis r10, -0x7cdc
	ctx.r[10].s64 = -2094792704;
	// 82DF4F18: 396B7838  addi r11, r11, 0x7838
	ctx.r[11].s64 = ctx.r[11].s64 + 30776;
	// 82DF4F1C: 386A1C48  addi r3, r10, 0x1c48
	ctx.r[3].s64 = ctx.r[10].s64 + 7240;
	// 82DF4F20: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF4F24: 483B35B5  bl 0x831a84d8
	ctx.lr = 0x82DF4F28;
	sub_831A84D8(ctx, base);
	// 82DF4F28: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF4F2C: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 82DF4F30: 3D008335  lis r8, -0x7ccb
	ctx.r[8].s64 = -2093678592;
	// 82DF4F34: 5569077B  rlwinm. r9, r11, 0, 0x1d, 0x1d
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82DF4F38: 93CA6C9C  stw r30, 0x6c9c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(27804 as u32), ctx.r[30].u32 ) };
	// 82DF4F3C: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82DF4F40: 93A8110C  stw r29, 0x110c(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4364 as u32), ctx.r[29].u32 ) };
	// 82DF4F44: 392A5FD0  addi r9, r10, 0x5fd0
	ctx.r[9].s64 = ctx.r[10].s64 + 24528;
	// 82DF4F48: 40820018  bne 0x82df4f60
	if !ctx.cr[0].eq {
	pc = 0x82DF4F60; continue 'dispatch;
	}
	// 82DF4F4C: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82DF4F50: 616B0004  ori r11, r11, 4
	ctx.r[11].u64 = ctx.r[11].u64 | 4;
	// 82DF4F54: 394AB238  addi r10, r10, -0x4dc8
	ctx.r[10].s64 = ctx.r[10].s64 + -19912;
	// 82DF4F58: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF4F5C: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DF4F60: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82DF4F64: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF4F68: 912B16D4  stw r9, 0x16d4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(5844 as u32), ctx.r[9].u32 ) };
	// 82DF4F6C: 4BFFFD2D  bl 0x82df4c98
	ctx.lr = 0x82DF4F70;
	sub_82DF4C98(ctx, base);
	// 82DF4F70: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF4F74: 4BFFCD1D  bl 0x82df1c90
	ctx.lr = 0x82DF4F78;
	sub_82DF1C90(ctx, base);
	// 82DF4F78: 48004429  bl 0x82df93a0
	ctx.lr = 0x82DF4F7C;
	sub_82DF93A0(ctx, base);
	// 82DF4F7C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF4F80: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82DF4F84: 419A0038  beq cr6, 0x82df4fbc
	if ctx.cr[6].eq {
	pc = 0x82DF4FBC; continue 'dispatch;
	}
	// 82DF4F88: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82DF4F8C: 55690739  rlwinm. r9, r11, 0, 0x1c, 0x1c
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82DF4F90: 3BCA5FC8  addi r30, r10, 0x5fc8
	ctx.r[30].s64 = ctx.r[10].s64 + 24520;
	// 82DF4F94: 40820060  bne 0x82df4ff4
	if !ctx.cr[0].eq {
	pc = 0x82DF4FF4; continue 'dispatch;
	}
	// 82DF4F98: 616B0008  ori r11, r11, 8
	ctx.r[11].u64 = ctx.r[11].u64 | 8;
	// 82DF4F9C: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 82DF4FA0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF4FA4: 4BFFD535  bl 0x82df24d8
	ctx.lr = 0x82DF4FA8;
	sub_82DF24D8(ctx, base);
	// 82DF4FA8: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DF4FAC: 3D408324  lis r10, -0x7cdc
	ctx.r[10].s64 = -2094792704;
	// 82DF4FB0: 396BB25C  addi r11, r11, -0x4da4
	ctx.r[11].s64 = ctx.r[11].s64 + -19876;
	// 82DF4FB4: 386A1C38  addi r3, r10, 0x1c38
	ctx.r[3].s64 = ctx.r[10].s64 + 7224;
	// 82DF4FB8: 48000034  b 0x82df4fec
	pc = 0x82DF4FEC; continue 'dispatch;
	// 82DF4FBC: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82DF4FC0: 556906F7  rlwinm. r9, r11, 0, 0x1b, 0x1b
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82DF4FC4: 3BCA5FC0  addi r30, r10, 0x5fc0
	ctx.r[30].s64 = ctx.r[10].s64 + 24512;
	// 82DF4FC8: 4082002C  bne 0x82df4ff4
	if !ctx.cr[0].eq {
	pc = 0x82DF4FF4; continue 'dispatch;
	}
	// 82DF4FCC: 616B0010  ori r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u64 | 16;
	// 82DF4FD0: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 82DF4FD4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF4FD8: 4BFFD501  bl 0x82df24d8
	ctx.lr = 0x82DF4FDC;
	sub_82DF24D8(ctx, base);
	// 82DF4FDC: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DF4FE0: 3D408324  lis r10, -0x7cdc
	ctx.r[10].s64 = -2094792704;
	// 82DF4FE4: 396BB260  addi r11, r11, -0x4da0
	ctx.r[11].s64 = ctx.r[11].s64 + -19872;
	// 82DF4FE8: 386A1C28  addi r3, r10, 0x1c28
	ctx.r[3].s64 = ctx.r[10].s64 + 7208;
	// 82DF4FEC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF4FF0: 483B34E9  bl 0x831a84d8
	ctx.lr = 0x82DF4FF4;
	sub_831A84D8(ctx, base);
	// 82DF4FF4: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82DF4FF8: 93CB5FE8  stw r30, 0x5fe8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24552 as u32), ctx.r[30].u32 ) };
	// 82DF4FFC: 480063DD  bl 0x82dfb3d8
	ctx.lr = 0x82DF5000;
	sub_82DFB3D8(ctx, base);
	// 82DF5000: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82DF5004: 394B5FBC  addi r10, r11, 0x5fbc
	ctx.r[10].s64 = ctx.r[11].s64 + 24508;
	// 82DF5008: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF500C: 556906B5  rlwinm. r9, r11, 0, 0x1a, 0x1a
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82DF5010: 40820018  bne 0x82df5028
	if !ctx.cr[0].eq {
	pc = 0x82DF5028; continue 'dispatch;
	}
	// 82DF5014: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82DF5018: 616B0020  ori r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 | 32;
	// 82DF501C: 3929B23C  addi r9, r9, -0x4dc4
	ctx.r[9].s64 = ctx.r[9].s64 + -19908;
	// 82DF5020: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF5024: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DF5028: 3D008335  lis r8, -0x7ccb
	ctx.r[8].s64 = -2093678592;
	// 82DF502C: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 82DF5030: 55670673  rlwinm. r7, r11, 0, 0x19, 0x19
	ctx.r[7].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82DF5034: 91481108  stw r10, 0x1108(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4360 as u32), ctx.r[10].u32 ) };
	// 82DF5038: 39495FB8  addi r10, r9, 0x5fb8
	ctx.r[10].s64 = ctx.r[9].s64 + 24504;
	// 82DF503C: 40820018  bne 0x82df5054
	if !ctx.cr[0].eq {
	pc = 0x82DF5054; continue 'dispatch;
	}
	// 82DF5040: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82DF5044: 616B0040  ori r11, r11, 0x40
	ctx.r[11].u64 = ctx.r[11].u64 | 64;
	// 82DF5048: 3929B240  addi r9, r9, -0x4dc0
	ctx.r[9].s64 = ctx.r[9].s64 + -19904;
	// 82DF504C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF5050: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DF5054: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 82DF5058: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 82DF505C: 55670631  rlwinm. r7, r11, 0, 0x18, 0x18
	ctx.r[7].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82DF5060: 91485F9C  stw r10, 0x5f9c(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(24476 as u32), ctx.r[10].u32 ) };
	// 82DF5064: 39495FB4  addi r10, r9, 0x5fb4
	ctx.r[10].s64 = ctx.r[9].s64 + 24500;
	// 82DF5068: 40820018  bne 0x82df5080
	if !ctx.cr[0].eq {
	pc = 0x82DF5080; continue 'dispatch;
	}
	// 82DF506C: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82DF5070: 616B0080  ori r11, r11, 0x80
	ctx.r[11].u64 = ctx.r[11].u64 | 128;
	// 82DF5074: 3929B244  addi r9, r9, -0x4dbc
	ctx.r[9].s64 = ctx.r[9].s64 + -19900;
	// 82DF5078: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF507C: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DF5080: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 82DF5084: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 82DF5088: 556705EF  rlwinm. r7, r11, 0, 0x17, 0x17
	ctx.r[7].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82DF508C: 91485FA0  stw r10, 0x5fa0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(24480 as u32), ctx.r[10].u32 ) };
	// 82DF5090: 39495FB0  addi r10, r9, 0x5fb0
	ctx.r[10].s64 = ctx.r[9].s64 + 24496;
	// 82DF5094: 40820018  bne 0x82df50ac
	if !ctx.cr[0].eq {
	pc = 0x82DF50AC; continue 'dispatch;
	}
	// 82DF5098: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82DF509C: 616B0100  ori r11, r11, 0x100
	ctx.r[11].u64 = ctx.r[11].u64 | 256;
	// 82DF50A0: 3929B248  addi r9, r9, -0x4db8
	ctx.r[9].s64 = ctx.r[9].s64 + -19896;
	// 82DF50A4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF50A8: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DF50AC: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 82DF50B0: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 82DF50B4: 556705AD  rlwinm. r7, r11, 0, 0x16, 0x16
	ctx.r[7].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82DF50B8: 91485F98  stw r10, 0x5f98(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(24472 as u32), ctx.r[10].u32 ) };
	// 82DF50BC: 39495FAC  addi r10, r9, 0x5fac
	ctx.r[10].s64 = ctx.r[9].s64 + 24492;
	// 82DF50C0: 40820018  bne 0x82df50d8
	if !ctx.cr[0].eq {
	pc = 0x82DF50D8; continue 'dispatch;
	}
	// 82DF50C4: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82DF50C8: 616B0200  ori r11, r11, 0x200
	ctx.r[11].u64 = ctx.r[11].u64 | 512;
	// 82DF50CC: 3929B24C  addi r9, r9, -0x4db4
	ctx.r[9].s64 = ctx.r[9].s64 + -19892;
	// 82DF50D0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF50D4: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DF50D8: 3D008335  lis r8, -0x7ccb
	ctx.r[8].s64 = -2093678592;
	// 82DF50DC: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 82DF50E0: 5567056B  rlwinm. r7, r11, 0, 0x15, 0x15
	ctx.r[7].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82DF50E4: 39295FA8  addi r9, r9, 0x5fa8
	ctx.r[9].s64 = ctx.r[9].s64 + 24488;
	// 82DF50E8: 91486FF4  stw r10, 0x6ff4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(28660 as u32), ctx.r[10].u32 ) };
	// 82DF50EC: 40820018  bne 0x82df5104
	if !ctx.cr[0].eq {
	pc = 0x82DF5104; continue 'dispatch;
	}
	// 82DF50F0: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82DF50F4: 616B0400  ori r11, r11, 0x400
	ctx.r[11].u64 = ctx.r[11].u64 | 1024;
	// 82DF50F8: 394AB250  addi r10, r10, -0x4db0
	ctx.r[10].s64 = ctx.r[10].s64 + -19888;
	// 82DF50FC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF5100: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DF5104: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82DF5108: 912B5FA4  stw r9, 0x5fa4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24484 as u32), ctx.r[9].u32 ) };
	// 82DF510C: 48006C8D  bl 0x82dfbd98
	ctx.lr = 0x82DF5110;
	sub_82DFBD98(ctx, base);
	// 82DF5110: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DF5114: 483B30A4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF5118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF5118 size=1016
    let mut pc: u32 = 0x82DF5118;
    'dispatch: loop {
        match pc {
            0x82DF5118 => {
    //   block [0x82DF5118..0x82DF5510)
	// 82DF5118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF511C: 483B303D  bl 0x831a8158
	ctx.lr = 0x82DF5120;
	sub_831A8130(ctx, base);
	// 82DF5120: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF5124: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82DF5128: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82DF512C: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82DF5130: 93E10104  stw r31, 0x104(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(260 as u32), ctx.r[31].u32 ) };
	// 82DF5134: 897F0019  lbz r11, 0x19(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(25 as u32) ) } as u64;
	// 82DF5138: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF513C: 419A0048  beq cr6, 0x82df5184
	if ctx.cr[6].eq {
	pc = 0x82DF5184; continue 'dispatch;
	}
	// 82DF5140: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82DF5144: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF5148: 388B9620  addi r4, r11, -0x69e0
	ctx.r[4].s64 = ctx.r[11].s64 + -27104;
	// 82DF514C: 4B4D077D  bl 0x822c58c8
	ctx.lr = 0x82DF5150;
	sub_822C58C8(ctx, base);
	// 82DF5150: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DF5154: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82DF5158: 4B4D4D59  bl 0x822c9eb0
	ctx.lr = 0x82DF515C;
	sub_822C9EB0(ctx, base);
	// 82DF515C: 4B4CF155  bl 0x822c42b0
	ctx.lr = 0x82DF5160;
	sub_822C42B0(ctx, base);
	// 82DF5160: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82DF5164: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82DF5168: 396B9600  addi r11, r11, -0x6a00
	ctx.r[11].s64 = ctx.r[11].s64 + -27136;
	// 82DF516C: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82DF5170: 4B4D0301  bl 0x822c5470
	ctx.lr = 0x82DF5174;
	sub_822C5470(ctx, base);
	// 82DF5174: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF5178: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DF517C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF5180: 4B4CFB61  bl 0x822c4ce0
	ctx.lr = 0x82DF5184;
	sub_822C4CE0(ctx, base);
	// 82DF5184: 38610104  addi r3, r1, 0x104
	ctx.r[3].s64 = ctx.r[1].s64 + 260;
	// 82DF5188: 7FFAFB78  mr r26, r31
	ctx.r[26].u64 = ctx.r[31].u64;
	// 82DF518C: 482CA69D  bl 0x830bf828
	ctx.lr = 0x82DF5190;
	sub_830BF828(ctx, base);
	// 82DF5190: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF5194: 894B0019  lbz r10, 0x19(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(25 as u32) ) } as u64;
	// 82DF5198: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF519C: 83210104  lwz r25, 0x104(r1)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(260 as u32) ) } as u64;
	// 82DF51A0: 419A000C  beq cr6, 0x82df51ac
	if ctx.cr[6].eq {
	pc = 0x82DF51AC; continue 'dispatch;
	}
	// 82DF51A4: 839A0008  lwz r28, 8(r26)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF51A8: 48000028  b 0x82df51d0
	pc = 0x82DF51D0; continue 'dispatch;
	// 82DF51AC: 815A0008  lwz r10, 8(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF51B0: 894A0019  lbz r10, 0x19(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(25 as u32) ) } as u64;
	// 82DF51B4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF51B8: 419A000C  beq cr6, 0x82df51c4
	if ctx.cr[6].eq {
	pc = 0x82DF51C4; continue 'dispatch;
	}
	// 82DF51BC: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 82DF51C0: 48000010  b 0x82df51d0
	pc = 0x82DF51D0; continue 'dispatch;
	// 82DF51C4: 83990008  lwz r28, 8(r25)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF51C8: 7F19D040  cmplw cr6, r25, r26
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82DF51CC: 409A00DC  bne cr6, 0x82df52a8
	if !ctx.cr[6].eq {
	pc = 0x82DF52A8; continue 'dispatch;
	}
	// 82DF51D0: 897C0019  lbz r11, 0x19(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(25 as u32) ) } as u64;
	// 82DF51D4: 83FA0004  lwz r31, 4(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF51D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF51DC: 409A0008  bne cr6, 0x82df51e4
	if !ctx.cr[6].eq {
	pc = 0x82DF51E4; continue 'dispatch;
	}
	// 82DF51E0: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82DF51E4: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF51E8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF51EC: 7F0AD040  cmplw cr6, r10, r26
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82DF51F0: 409A000C  bne cr6, 0x82df51fc
	if !ctx.cr[6].eq {
	pc = 0x82DF51FC; continue 'dispatch;
	}
	// 82DF51F4: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82DF51F8: 4800001C  b 0x82df5214
	pc = 0x82DF5214; continue 'dispatch;
	// 82DF51FC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF5200: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82DF5204: 409A000C  bne cr6, 0x82df5210
	if !ctx.cr[6].eq {
	pc = 0x82DF5210; continue 'dispatch;
	}
	// 82DF5208: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82DF520C: 48000008  b 0x82df5214
	pc = 0x82DF5214; continue 'dispatch;
	// 82DF5210: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82DF5214: 813B0004  lwz r9, 4(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF5218: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF521C: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82DF5220: 409A003C  bne cr6, 0x82df525c
	if !ctx.cr[6].eq {
	pc = 0x82DF525C; continue 'dispatch;
	}
	// 82DF5224: 897C0019  lbz r11, 0x19(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(25 as u32) ) } as u64;
	// 82DF5228: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF522C: 419A000C  beq cr6, 0x82df5238
	if ctx.cr[6].eq {
	pc = 0x82DF5238; continue 'dispatch;
	}
	// 82DF5230: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 82DF5234: 48000024  b 0x82df5258
	pc = 0x82DF5258; continue 'dispatch;
	// 82DF5238: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF523C: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 82DF5240: 4800000C  b 0x82df524c
	pc = 0x82DF524C; continue 'dispatch;
	// 82DF5244: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82DF5248: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF524C: 890B0019  lbz r8, 0x19(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(25 as u32) ) } as u64;
	// 82DF5250: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82DF5254: 419AFFF0  beq cr6, 0x82df5244
	if ctx.cr[6].eq {
	pc = 0x82DF5244; continue 'dispatch;
	}
	// 82DF5258: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DF525C: 813B0004  lwz r9, 4(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF5260: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF5264: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82DF5268: 409A00D4  bne cr6, 0x82df533c
	if !ctx.cr[6].eq {
	pc = 0x82DF533C; continue 'dispatch;
	}
	// 82DF526C: 897C0019  lbz r11, 0x19(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(25 as u32) ) } as u64;
	// 82DF5270: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF5274: 419A000C  beq cr6, 0x82df5280
	if ctx.cr[6].eq {
	pc = 0x82DF5280; continue 'dispatch;
	}
	// 82DF5278: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 82DF527C: 48000024  b 0x82df52a0
	pc = 0x82DF52A0; continue 'dispatch;
	// 82DF5280: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF5284: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 82DF5288: 4800000C  b 0x82df5294
	pc = 0x82DF5294; continue 'dispatch;
	// 82DF528C: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82DF5290: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF5294: 890B0019  lbz r8, 0x19(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(25 as u32) ) } as u64;
	// 82DF5298: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82DF529C: 419AFFF0  beq cr6, 0x82df528c
	if ctx.cr[6].eq {
	pc = 0x82DF528C; continue 'dispatch;
	}
	// 82DF52A0: 91490008  stw r10, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82DF52A4: 48000098  b 0x82df533c
	pc = 0x82DF533C; continue 'dispatch;
	// 82DF52A8: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 82DF52AC: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF52B0: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF52B4: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF52B8: 7F195840  cmplw cr6, r25, r11
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF52BC: 409A000C  bne cr6, 0x82df52c8
	if !ctx.cr[6].eq {
	pc = 0x82DF52C8; continue 'dispatch;
	}
	// 82DF52C0: 7F3FCB78  mr r31, r25
	ctx.r[31].u64 = ctx.r[25].u64;
	// 82DF52C4: 4800002C  b 0x82df52f0
	pc = 0x82DF52F0; continue 'dispatch;
	// 82DF52C8: 897C0019  lbz r11, 0x19(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(25 as u32) ) } as u64;
	// 82DF52CC: 83F90004  lwz r31, 4(r25)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF52D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF52D4: 409A0008  bne cr6, 0x82df52dc
	if !ctx.cr[6].eq {
	pc = 0x82DF52DC; continue 'dispatch;
	}
	// 82DF52D8: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82DF52DC: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82DF52E0: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF52E4: 91790008  stw r11, 8(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DF52E8: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF52EC: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 82DF52F0: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF52F4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF52F8: 7F0AD040  cmplw cr6, r10, r26
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82DF52FC: 409A000C  bne cr6, 0x82df5308
	if !ctx.cr[6].eq {
	pc = 0x82DF5308; continue 'dispatch;
	}
	// 82DF5300: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 82DF5304: 48000020  b 0x82df5324
	pc = 0x82DF5324; continue 'dispatch;
	// 82DF5308: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF530C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF5310: 7F0AD040  cmplw cr6, r10, r26
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82DF5314: 409A000C  bne cr6, 0x82df5320
	if !ctx.cr[6].eq {
	pc = 0x82DF5320; continue 'dispatch;
	}
	// 82DF5318: 932B0000  stw r25, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 82DF531C: 48000008  b 0x82df5324
	pc = 0x82DF5324; continue 'dispatch;
	// 82DF5320: 932B0008  stw r25, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[25].u32 ) };
	// 82DF5324: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF5328: 91790004  stw r11, 4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DF532C: 897A0018  lbz r11, 0x18(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DF5330: 89590018  lbz r10, 0x18(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[25].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DF5334: 99790018  stb r11, 0x18(r25)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[25].u32.wrapping_add(24 as u32), ctx.r[11].u8 ) };
	// 82DF5338: 995A0018  stb r10, 0x18(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(24 as u32), ctx.r[10].u8 ) };
	// 82DF533C: 897A0018  lbz r11, 0x18(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DF5340: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82DF5344: 409A0198  bne cr6, 0x82df54dc
	if !ctx.cr[6].eq {
	pc = 0x82DF54DC; continue 'dispatch;
	}
	// 82DF5348: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF534C: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82DF5350: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF5354: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF5358: 419A0180  beq cr6, 0x82df54d8
	if ctx.cr[6].eq {
	pc = 0x82DF54D8; continue 'dispatch;
	}
	// 82DF535C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82DF5360: 897C0018  lbz r11, 0x18(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DF5364: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82DF5368: 409A0170  bne cr6, 0x82df54d8
	if !ctx.cr[6].eq {
	pc = 0x82DF54D8; continue 'dispatch;
	}
	// 82DF536C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF5370: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF5374: 409A00A8  bne cr6, 0x82df541c
	if !ctx.cr[6].eq {
	pc = 0x82DF541C; continue 'dispatch;
	}
	// 82DF5378: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF537C: 894B0018  lbz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DF5380: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF5384: 409A001C  bne cr6, 0x82df53a0
	if !ctx.cr[6].eq {
	pc = 0x82DF53A0; continue 'dispatch;
	}
	// 82DF5388: 9BCB0018  stb r30, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 82DF538C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF5390: 9BBF0018  stb r29, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 82DF5394: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82DF5398: 4801CEA1  bl 0x82e12238
	ctx.lr = 0x82DF539C;
	sub_82E12238(ctx, base);
	// 82DF539C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF53A0: 894B0019  lbz r10, 0x19(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(25 as u32) ) } as u64;
	// 82DF53A4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF53A8: 409A00C8  bne cr6, 0x82df5470
	if !ctx.cr[6].eq {
	pc = 0x82DF5470; continue 'dispatch;
	}
	// 82DF53AC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF53B0: 894A0018  lbz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DF53B4: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82DF53B8: 409A0014  bne cr6, 0x82df53cc
	if !ctx.cr[6].eq {
	pc = 0x82DF53CC; continue 'dispatch;
	}
	// 82DF53BC: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF53C0: 894A0018  lbz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DF53C4: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82DF53C8: 419A00A4  beq cr6, 0x82df546c
	if ctx.cr[6].eq {
	pc = 0x82DF546C; continue 'dispatch;
	}
	// 82DF53CC: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF53D0: 894A0018  lbz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DF53D4: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82DF53D8: 409A0020  bne cr6, 0x82df53f8
	if !ctx.cr[6].eq {
	pc = 0x82DF53F8; continue 'dispatch;
	}
	// 82DF53DC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF53E0: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DF53E4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82DF53E8: 9BCA0018  stb r30, 0x18(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 82DF53EC: 9BAB0018  stb r29, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 82DF53F0: 4801CEB1  bl 0x82e122a0
	ctx.lr = 0x82DF53F4;
	sub_82E122A0(ctx, base);
	// 82DF53F4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF53F8: 895F0018  lbz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DF53FC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF5400: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82DF5404: 994B0018  stb r10, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[10].u8 ) };
	// 82DF5408: 9BDF0018  stb r30, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 82DF540C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF5410: 9BCB0018  stb r30, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 82DF5414: 4801CE25  bl 0x82e12238
	ctx.lr = 0x82DF5418;
	sub_82E12238(ctx, base);
	// 82DF5418: 480000C0  b 0x82df54d8
	pc = 0x82DF54D8; continue 'dispatch;
	// 82DF541C: 894B0018  lbz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DF5420: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF5424: 409A001C  bne cr6, 0x82df5440
	if !ctx.cr[6].eq {
	pc = 0x82DF5440; continue 'dispatch;
	}
	// 82DF5428: 9BCB0018  stb r30, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 82DF542C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF5430: 9BBF0018  stb r29, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 82DF5434: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82DF5438: 4801CE69  bl 0x82e122a0
	ctx.lr = 0x82DF543C;
	sub_82E122A0(ctx, base);
	// 82DF543C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF5440: 894B0019  lbz r10, 0x19(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(25 as u32) ) } as u64;
	// 82DF5444: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF5448: 409A0028  bne cr6, 0x82df5470
	if !ctx.cr[6].eq {
	pc = 0x82DF5470; continue 'dispatch;
	}
	// 82DF544C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF5450: 894A0018  lbz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DF5454: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82DF5458: 409A0034  bne cr6, 0x82df548c
	if !ctx.cr[6].eq {
	pc = 0x82DF548C; continue 'dispatch;
	}
	// 82DF545C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF5460: 894A0018  lbz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DF5464: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82DF5468: 409A0024  bne cr6, 0x82df548c
	if !ctx.cr[6].eq {
	pc = 0x82DF548C; continue 'dispatch;
	}
	// 82DF546C: 9BAB0018  stb r29, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 82DF5470: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF5474: 7FFCFB78  mr r28, r31
	ctx.r[28].u64 = ctx.r[31].u64;
	// 82DF5478: 83FF0004  lwz r31, 4(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF547C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF5480: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF5484: 409AFEDC  bne cr6, 0x82df5360
	if !ctx.cr[6].eq {
	pc = 0x82DF5360; continue 'dispatch;
	}
	// 82DF5488: 48000050  b 0x82df54d8
	pc = 0x82DF54D8; continue 'dispatch;
	// 82DF548C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF5490: 894A0018  lbz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DF5494: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82DF5498: 409A0020  bne cr6, 0x82df54b8
	if !ctx.cr[6].eq {
	pc = 0x82DF54B8; continue 'dispatch;
	}
	// 82DF549C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF54A0: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DF54A4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82DF54A8: 9BCA0018  stb r30, 0x18(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 82DF54AC: 9BAB0018  stb r29, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 82DF54B0: 4801CD89  bl 0x82e12238
	ctx.lr = 0x82DF54B4;
	sub_82E12238(ctx, base);
	// 82DF54B4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF54B8: 895F0018  lbz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DF54BC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF54C0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82DF54C4: 994B0018  stb r10, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[10].u8 ) };
	// 82DF54C8: 9BDF0018  stb r30, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 82DF54CC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF54D0: 9BCB0018  stb r30, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 82DF54D4: 4801CDCD  bl 0x82e122a0
	ctx.lr = 0x82DF54D8;
	sub_82E122A0(ctx, base);
	// 82DF54D8: 9BDC0018  stb r30, 0x18(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 82DF54DC: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82DF54E0: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82DF54E4: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82DF54E8: 4BFFCCA1  bl 0x82df2188
	ctx.lr = 0x82DF54EC;
	sub_82DF2188(ctx, base);
	// 82DF54EC: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF54F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF54F4: 419A000C  beq cr6, 0x82df5500
	if ctx.cr[6].eq {
	pc = 0x82DF5500; continue 'dispatch;
	}
	// 82DF54F8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DF54FC: 917B0008  stw r11, 8(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DF5500: 93380000  stw r25, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 82DF5504: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82DF5508: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82DF550C: 483B2C9C  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF5510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF5510 size=132
    let mut pc: u32 = 0x82DF5510;
    'dispatch: loop {
        match pc {
            0x82DF5510 => {
    //   block [0x82DF5510..0x82DF5594)
	// 82DF5510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF5514: 483B2C55  bl 0x831a8168
	ctx.lr = 0x82DF5518;
	sub_831A8130(ctx, base);
	// 82DF5518: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF551C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DF5520: 90A100A4  stw r5, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[5].u32 ) };
	// 82DF5524: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DF5528: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82DF552C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF5530: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF5534: 7F055040  cmplw cr6, r5, r10
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DF5538: 409A0044  bne cr6, 0x82df557c
	if !ctx.cr[6].eq {
	pc = 0x82DF557C; continue 'dispatch;
	}
	// 82DF553C: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF5540: 409A003C  bne cr6, 0x82df557c
	if !ctx.cr[6].eq {
	pc = 0x82DF557C; continue 'dispatch;
	}
	// 82DF5544: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF5548: 4BFFF8A1  bl 0x82df4de8
	ctx.lr = 0x82DF554C;
	sub_82DF4DE8(ctx, base);
	// 82DF554C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF5550: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF5554: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF5558: 48000030  b 0x82df5588
	pc = 0x82DF5588; continue 'dispatch;
	// 82DF555C: 386100A4  addi r3, r1, 0xa4
	ctx.r[3].s64 = ctx.r[1].s64 + 164;
	// 82DF5560: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82DF5564: 482CA2C5  bl 0x830bf828
	ctx.lr = 0x82DF5568;
	sub_830BF828(ctx, base);
	// 82DF5568: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82DF556C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF5570: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF5574: 4BFFFBA5  bl 0x82df5118
	ctx.lr = 0x82DF5578;
	sub_82DF5118(ctx, base);
	// 82DF5578: 80A100A4  lwz r5, 0xa4(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82DF557C: 7F05F040  cmplw cr6, r5, r30
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82DF5580: 409AFFDC  bne cr6, 0x82df555c
	if !ctx.cr[6].eq {
	pc = 0x82DF555C; continue 'dispatch;
	}
	// 82DF5584: 90BD0000  stw r5, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82DF5588: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DF558C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DF5590: 483B2C28  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF5598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF5598 size=88
    let mut pc: u32 = 0x82DF5598;
    'dispatch: loop {
        match pc {
            0x82DF5598 => {
    //   block [0x82DF5598..0x82DF55F0)
	// 82DF5598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF559C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF55A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF55A4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF55A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF55AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF55B0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF55B4: 80DF0004  lwz r6, 4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF55B8: 80A60000  lwz r5, 0(r6)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF55BC: 4BFFFF55  bl 0x82df5510
	ctx.lr = 0x82DF55C0;
	sub_82DF5510(ctx, base);
	// 82DF55C0: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82DF55C4: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF55C8: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82DF55CC: 4BFFCBBD  bl 0x82df2188
	ctx.lr = 0x82DF55D0;
	sub_82DF2188(ctx, base);
	// 82DF55D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF55D4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DF55D8: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DF55DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF55E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF55E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF55E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF55EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF55F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF55F0 size=144
    let mut pc: u32 = 0x82DF55F0;
    'dispatch: loop {
        match pc {
            0x82DF55F0 => {
    //   block [0x82DF55F0..0x82DF5680)
	// 82DF55F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF55F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF55F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF55FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF5600: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF5604: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DF5608: 396BB264  addi r11, r11, -0x4d9c
	ctx.r[11].s64 = ctx.r[11].s64 + -19868;
	// 82DF560C: 807F0044  lwz r3, 0x44(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82DF5610: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF5614: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DF5618: 419A0018  beq cr6, 0x82df5630
	if ctx.cr[6].eq {
	pc = 0x82DF5630; continue 'dispatch;
	}
	// 82DF561C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF5620: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DF5624: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF5628: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DF562C: 4E800421  bctrl
	ctx.lr = 0x82DF5630;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DF5630: 807F003C  lwz r3, 0x3c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82DF5634: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DF5638: 419A0008  beq cr6, 0x82df5640
	if ctx.cr[6].eq {
	pc = 0x82DF5640; continue 'dispatch;
	}
	// 82DF563C: 4B4CB255  bl 0x822c0890
	ctx.lr = 0x82DF5640;
	sub_822C0890(ctx, base);
	// 82DF5640: 807F0034  lwz r3, 0x34(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DF5644: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DF5648: 419A0008  beq cr6, 0x82df5650
	if ctx.cr[6].eq {
	pc = 0x82DF5650; continue 'dispatch;
	}
	// 82DF564C: 4B4CB245  bl 0x822c0890
	ctx.lr = 0x82DF5650;
	sub_822C0890(ctx, base);
	// 82DF5650: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 82DF5654: 4806C5E5  bl 0x82e61c38
	ctx.lr = 0x82DF5658;
	sub_82E61C38(ctx, base);
	// 82DF5658: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 82DF565C: 4BFFFF3D  bl 0x82df5598
	ctx.lr = 0x82DF5660;
	sub_82DF5598(ctx, base);
	// 82DF5660: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DF5664: 396BB37C  addi r11, r11, -0x4c84
	ctx.r[11].s64 = ctx.r[11].s64 + -19588;
	// 82DF5668: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF566C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF5670: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF5674: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF5678: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF567C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF5680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF5680 size=76
    let mut pc: u32 = 0x82DF5680;
    'dispatch: loop {
        match pc {
            0x82DF5680 => {
    //   block [0x82DF5680..0x82DF56CC)
	// 82DF5680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF5684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF5688: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DF568C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF5690: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF5694: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF5698: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DF569C: 4BFFFF55  bl 0x82df55f0
	ctx.lr = 0x82DF56A0;
	sub_82DF55F0(ctx, base);
	// 82DF56A0: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF56A4: 4182000C  beq 0x82df56b0
	if ctx.cr[0].eq {
	pc = 0x82DF56B0; continue 'dispatch;
	}
	// 82DF56A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF56AC: 4BFFCD2D  bl 0x82df23d8
	ctx.lr = 0x82DF56B0;
	sub_82DF23D8(ctx, base);
	// 82DF56B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF56B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF56B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF56BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF56C0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DF56C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF56C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF56D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF56D0 size=100
    let mut pc: u32 = 0x82DF56D0;
    'dispatch: loop {
        match pc {
            0x82DF56D0 => {
    //   block [0x82DF56D0..0x82DF5734)
	// 82DF56D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF56D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF56D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF56DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF56E0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82DF56E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82DF56E8: 388BBE28  addi r4, r11, -0x41d8
	ctx.r[4].s64 = ctx.r[11].s64 + -16856;
	// 82DF56EC: 38A0001E  li r5, 0x1e
	ctx.r[5].s64 = 30;
	// 82DF56F0: 38600048  li r3, 0x48
	ctx.r[3].s64 = 72;
	// 82DF56F4: 4BFFCCF5  bl 0x82df23e8
	ctx.lr = 0x82DF56F8;
	sub_82DF23E8(ctx, base);
	// 82DF56F8: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82DF56FC: 41820020  beq 0x82df571c
	if ctx.cr[0].eq {
	pc = 0x82DF571C; continue 'dispatch;
	}
	// 82DF5700: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF5704: 48007E35  bl 0x82dfd538
	ctx.lr = 0x82DF5708;
	sub_82DFD538(ctx, base);
	// 82DF5708: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DF570C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF5710: 396BB29C  addi r11, r11, -0x4d64
	ctx.r[11].s64 = ctx.r[11].s64 + -19812;
	// 82DF5714: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF5718: 48000008  b 0x82df5720
	pc = 0x82DF5720; continue 'dispatch;
	// 82DF571C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DF5720: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF5724: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF5728: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF572C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF5730: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF5738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF5738 size=20
    let mut pc: u32 = 0x82DF5738;
    'dispatch: loop {
        match pc {
            0x82DF5738 => {
    //   block [0x82DF5738..0x82DF574C)
	// 82DF5738: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF573C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82DF5740: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82DF5744: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 82DF5748: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF5750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF5750 size=16
    let mut pc: u32 = 0x82DF5750;
    'dispatch: loop {
        match pc {
            0x82DF5750 => {
    //   block [0x82DF5750..0x82DF5760)
	// 82DF5750: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF5754: 894B002D  lbz r10, 0x2d(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(45 as u32) ) } as u64;
	// 82DF5758: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF575C: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF5760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF5760 size=24
    let mut pc: u32 = 0x82DF5760;
    'dispatch: loop {
        match pc {
            0x82DF5760 => {
    //   block [0x82DF5760..0x82DF5778)
	// 82DF5760: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF5764: 892A002D  lbz r9, 0x2d(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(45 as u32) ) } as u64;
	// 82DF5768: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DF576C: 409A0040  bne cr6, 0x82df57ac
	if !ctx.cr[6].eq {
		sub_82DF5794(ctx, base);
		return;
	}
	// 82DF5770: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF5774: 4800000C  b 0x82df5780
	sub_82DF5778(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF5778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF5778 size=28
    let mut pc: u32 = 0x82DF5778;
    'dispatch: loop {
        match pc {
            0x82DF5778 => {
    //   block [0x82DF5778..0x82DF5794)
	// 82DF5778: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82DF577C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF5780: 892B002D  lbz r9, 0x2d(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(45 as u32) ) } as u64;
	// 82DF5784: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DF5788: 419AFFF0  beq cr6, 0x82df5778
	if ctx.cr[6].eq {
	pc = 0x82DF5778; continue 'dispatch;
	}
	// 82DF578C: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DF5790: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF5794(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF5794 size=48
    let mut pc: u32 = 0x82DF5794;
    'dispatch: loop {
        match pc {
            0x82DF5794 => {
    //   block [0x82DF5794..0x82DF57C4)
	// 82DF5794: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF5798: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF579C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DF57A0: 409A001C  bne cr6, 0x82df57bc
	if !ctx.cr[6].eq {
	pc = 0x82DF57BC; continue 'dispatch;
	}
	// 82DF57A4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF57A8: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DF57AC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF57B0: 894B002D  lbz r10, 0x2d(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(45 as u32) ) } as u64;
	// 82DF57B4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF57B8: 419AFFDC  beq cr6, 0x82df5794
	if ctx.cr[6].eq {
	pc = 0x82DF5794; continue 'dispatch;
	}
	// 82DF57BC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF57C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF57C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF57C8 size=116
    let mut pc: u32 = 0x82DF57C8;
    'dispatch: loop {
        match pc {
            0x82DF57C8 => {
    //   block [0x82DF57C8..0x82DF583C)
	// 82DF57C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF57CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF57D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF57D4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF57D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF57DC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82DF57E0: 419A0040  beq cr6, 0x82df5820
	if ctx.cr[6].eq {
	pc = 0x82DF5820; continue 'dispatch;
	}
	// 82DF57E4: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82DF57E8: 0CDF0000  twi 6, r31, 0
	// 82DF57EC: 7D6BFB96  divwu r11, r11, r31
	ctx.r[11].u32 = ctx.r[11].u32 / ctx.r[31].u32;
	// 82DF57F0: 2B0B0030  cmplwi cr6, r11, 0x30
	ctx.cr[6].compare_u32(ctx.r[11].u32, 48 as u32, &mut ctx.xer);
	// 82DF57F4: 4098002C  bge cr6, 0x82df5820
	if !ctx.cr[6].lt {
	pc = 0x82DF5820; continue 'dispatch;
	}
	// 82DF57F8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DF57FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DF5800: 396B0828  addi r11, r11, 0x828
	ctx.r[11].s64 = ctx.r[11].s64 + 2088;
	// 82DF5804: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82DF5808: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF580C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82DF5810: 4B4CEAA1  bl 0x822c42b0
	ctx.lr = 0x82DF5814;
	sub_822C42B0(ctx, base);
	// 82DF5814: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DF5818: 396B0818  addi r11, r11, 0x818
	ctx.r[11].s64 = ctx.r[11].s64 + 2072;
	// 82DF581C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82DF5820: 1C7F0030  mulli r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 * 48;
	// 82DF5824: 4B4CB115  bl 0x822c0938
	ctx.lr = 0x82DF5828;
	sub_822C0938(ctx, base);
	// 82DF5828: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF582C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF5830: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF5834: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF5838: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF5840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF5840 size=68
    let mut pc: u32 = 0x82DF5840;
    'dispatch: loop {
        match pc {
            0x82DF5840 => {
    //   block [0x82DF5840..0x82DF5884)
	// 82DF5840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF5844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF5848: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF584C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF5850: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82DF5854: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF5858: 806B16D4  lwz r3, 0x16d4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5844 as u32) ) } as u64;
	// 82DF585C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF5860: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF5864: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DF5868: 4E800421  bctrl
	ctx.lr = 0x82DF586C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DF586C: 907F001C  stw r3, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[3].u32 ) };
	// 82DF5870: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF5874: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF5878: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF587C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF5880: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF5888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF5888 size=204
    let mut pc: u32 = 0x82DF5888;
    'dispatch: loop {
        match pc {
            0x82DF5888 => {
    //   block [0x82DF5888..0x82DF5954)
	// 82DF5888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF588C: 483B28E1  bl 0x831a816c
	ctx.lr = 0x82DF5890;
	sub_831A8130(ctx, base);
	// 82DF5890: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF5894: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DF5898: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DF589C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DF58A0: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DF58A4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF58A8: 419A0018  beq cr6, 0x82df58c0
	if ctx.cr[6].eq {
	pc = 0x82DF58C0; continue 'dispatch;
	}
	// 82DF58AC: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82DF58B0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF58B4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF58B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DF58BC: 4E800421  bctrl
	ctx.lr = 0x82DF58C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DF58C0: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DF58C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF58C8: 419A0058  beq cr6, 0x82df5920
	if ctx.cr[6].eq {
	pc = 0x82DF5920; continue 'dispatch;
	}
	// 82DF58CC: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DF58D0: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF58D4: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82DF58D8: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 82DF58DC: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF58E0: 40980040  bge cr6, 0x82df5920
	if !ctx.cr[6].lt {
	pc = 0x82DF5920; continue 'dispatch;
	}
	// 82DF58E4: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DF58E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF58EC: 419A0018  beq cr6, 0x82df5904
	if ctx.cr[6].eq {
	pc = 0x82DF5904; continue 'dispatch;
	}
	// 82DF58F0: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82DF58F4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF58F8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DF58FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DF5900: 4E800421  bctrl
	ctx.lr = 0x82DF5904;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DF5904: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF5908: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DF590C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DF5910: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DF5914: 7C8A5A14  add r4, r10, r11
	ctx.r[4].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DF5918: 4BFFE2E9  bl 0x82df3c00
	ctx.lr = 0x82DF591C;
	sub_82DF3C00(ctx, base);
	// 82DF591C: 4800002C  b 0x82df5948
	pc = 0x82DF5948; continue 'dispatch;
	// 82DF5920: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DF5924: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF5928: 419A0018  beq cr6, 0x82df5940
	if ctx.cr[6].eq {
	pc = 0x82DF5940; continue 'dispatch;
	}
	// 82DF592C: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82DF5930: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF5934: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DF5938: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DF593C: 4E800421  bctrl
	ctx.lr = 0x82DF5940;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DF5940: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DF5944: 4BFFD7AD  bl 0x82df30f0
	ctx.lr = 0x82DF5948;
	sub_82DF30F0(ctx, base);
	// 82DF5948: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DF594C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF5950: 483B286C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF5958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF5958 size=88
    let mut pc: u32 = 0x82DF5958;
    'dispatch: loop {
        match pc {
            0x82DF5958 => {
    //   block [0x82DF5958..0x82DF59B0)
	// 82DF5958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF595C: 483B2811  bl 0x831a816c
	ctx.lr = 0x82DF5960;
	sub_831A8130(ctx, base);
	// 82DF5960: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF5964: 83C30004  lwz r30, 4(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF5968: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DF596C: 83FE0004  lwz r31, 4(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF5970: 48000028  b 0x82df5998
	pc = 0x82DF5998; continue 'dispatch;
	// 82DF5974: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DF5978: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82DF597C: 4BFFD8BD  bl 0x82df3238
	ctx.lr = 0x82DF5980;
	sub_82DF3238(ctx, base);
	// 82DF5980: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF5984: 4182000C  beq 0x82df5990
	if ctx.cr[0].eq {
	pc = 0x82DF5990; continue 'dispatch;
	}
	// 82DF5988: 83FF0008  lwz r31, 8(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF598C: 4800000C  b 0x82df5998
	pc = 0x82DF5998; continue 'dispatch;
	// 82DF5990: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	// 82DF5994: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF5998: 897F0015  lbz r11, 0x15(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(21 as u32) ) } as u64;
	// 82DF599C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF59A0: 419AFFD4  beq cr6, 0x82df5974
	if ctx.cr[6].eq {
	pc = 0x82DF5974; continue 'dispatch;
	}
	// 82DF59A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF59A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF59AC: 483B2810  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF59B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF59B0 size=64
    let mut pc: u32 = 0x82DF59B0;
    'dispatch: loop {
        match pc {
            0x82DF59B0 => {
    //   block [0x82DF59B0..0x82DF59F0)
	// 82DF59B0: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF59B4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF59B8: 91440008  stw r10, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82DF59BC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF59C0: 892A0015  lbz r9, 0x15(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(21 as u32) ) } as u64;
	// 82DF59C4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DF59C8: 409A0008  bne cr6, 0x82df59d0
	if !ctx.cr[6].eq {
	pc = 0x82DF59D0; continue 'dispatch;
	}
	// 82DF59CC: 908A0004  stw r4, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82DF59D0: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF59D4: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DF59D8: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF59DC: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF59E0: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DF59E4: 409A000C  bne cr6, 0x82df59f0
	if !ctx.cr[6].eq {
		sub_82DF59F0(ctx, base);
		return;
	}
	// 82DF59E8: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DF59EC: 48000020  b 0x82df5a0c
	sub_82DF5A08(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF59F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF59F0 size=24
    let mut pc: u32 = 0x82DF59F0;
    'dispatch: loop {
        match pc {
            0x82DF59F0 => {
    //   block [0x82DF59F0..0x82DF5A08)
	// 82DF59F0: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF59F4: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF59F8: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DF59FC: 409A000C  bne cr6, 0x82df5a08
	if !ctx.cr[6].eq {
		sub_82DF5A08(ctx, base);
		return;
	}
	// 82DF5A00: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF5A04: 48000008  b 0x82df5a0c
	sub_82DF5A08(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF5A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF5A08 size=16
    let mut pc: u32 = 0x82DF5A08;
    'dispatch: loop {
        match pc {
            0x82DF5A08 => {
    //   block [0x82DF5A08..0x82DF5A18)
	// 82DF5A08: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DF5A0C: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82DF5A10: 91640004  stw r11, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DF5A14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF5A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF5A18 size=64
    let mut pc: u32 = 0x82DF5A18;
    'dispatch: loop {
        match pc {
            0x82DF5A18 => {
    //   block [0x82DF5A18..0x82DF5A58)
	// 82DF5A18: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF5A1C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF5A20: 91440008  stw r10, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82DF5A24: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF5A28: 892A002D  lbz r9, 0x2d(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(45 as u32) ) } as u64;
	// 82DF5A2C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DF5A30: 409A0008  bne cr6, 0x82df5a38
	if !ctx.cr[6].eq {
	pc = 0x82DF5A38; continue 'dispatch;
	}
	// 82DF5A34: 908A0004  stw r4, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82DF5A38: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF5A3C: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DF5A40: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF5A44: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF5A48: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DF5A4C: 409A000C  bne cr6, 0x82df5a58
	if !ctx.cr[6].eq {
		sub_82DF5A58(ctx, base);
		return;
	}
	// 82DF5A50: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DF5A54: 48000020  b 0x82df5a74
	sub_82DF5A70(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF5A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF5A58 size=24
    let mut pc: u32 = 0x82DF5A58;
    'dispatch: loop {
        match pc {
            0x82DF5A58 => {
    //   block [0x82DF5A58..0x82DF5A70)
	// 82DF5A58: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF5A5C: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF5A60: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DF5A64: 409A000C  bne cr6, 0x82df5a70
	if !ctx.cr[6].eq {
		sub_82DF5A70(ctx, base);
		return;
	}
	// 82DF5A68: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF5A6C: 48000008  b 0x82df5a74
	sub_82DF5A70(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF5A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF5A70 size=16
    let mut pc: u32 = 0x82DF5A70;
    'dispatch: loop {
        match pc {
            0x82DF5A70 => {
    //   block [0x82DF5A70..0x82DF5A80)
	// 82DF5A70: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DF5A74: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82DF5A78: 91640004  stw r11, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DF5A7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF5A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF5A80 size=108
    let mut pc: u32 = 0x82DF5A80;
    'dispatch: loop {
        match pc {
            0x82DF5A80 => {
    //   block [0x82DF5A80..0x82DF5AEC)
	// 82DF5A80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF5A84: 483B26DD  bl 0x831a8160
	ctx.lr = 0x82DF5A88;
	sub_831A8130(ctx, base);
	// 82DF5A88: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF5A8C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DF5A90: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DF5A94: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DF5A98: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82DF5A9C: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82DF5AA0: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82DF5AA4: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 82DF5AA8: 48027539  bl 0x82e1cfe0
	ctx.lr = 0x82DF5AAC;
	sub_82E1CFE0(ctx, base);
	// 82DF5AAC: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82DF5AB0: 41820030  beq 0x82df5ae0
	if ctx.cr[0].eq {
	pc = 0x82DF5AE0; continue 'dispatch;
	}
	// 82DF5AB4: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82DF5AB8: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82DF5ABC: 939F0004  stw r28, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82DF5AC0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DF5AC4: 937F0008  stw r27, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[27].u32 ) };
	// 82DF5AC8: 4BFFE139  bl 0x82df3c00
	ctx.lr = 0x82DF5ACC;
	sub_82DF3C00(ctx, base);
	// 82DF5ACC: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF5AD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF5AD4: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82DF5AD8: 9B5F0014  stb r26, 0x14(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[26].u8 ) };
	// 82DF5ADC: 997F0015  stb r11, 0x15(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(21 as u32), ctx.r[11].u8 ) };
	// 82DF5AE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF5AE4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DF5AE8: 483B26C8  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF5AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF5AF0 size=88
    let mut pc: u32 = 0x82DF5AF0;
    'dispatch: loop {
        match pc {
            0x82DF5AF0 => {
    //   block [0x82DF5AF0..0x82DF5B48)
	// 82DF5AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF5AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF5AF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF5AFC: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82DF5B00: 4BFFC3A9  bl 0x82df1ea8
	ctx.lr = 0x82DF5B04;
	sub_82DF1EA8(ctx, base);
	// 82DF5B04: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DF5B08: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DF5B0C: 41820008  beq 0x82df5b14
	if ctx.cr[0].eq {
	pc = 0x82DF5B14; continue 'dispatch;
	}
	// 82DF5B10: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DF5B14: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF5B18: 41820008  beq 0x82df5b20
	if ctx.cr[0].eq {
	pc = 0x82DF5B20; continue 'dispatch;
	}
	// 82DF5B1C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DF5B20: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF5B24: 41820008  beq 0x82df5b2c
	if ctx.cr[0].eq {
	pc = 0x82DF5B2C; continue 'dispatch;
	}
	// 82DF5B28: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DF5B2C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DF5B30: 99430015  stb r10, 0x15(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(21 as u32), ctx.r[10].u8 ) };
	// 82DF5B34: 99630014  stb r11, 0x14(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 82DF5B38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF5B3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF5B40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF5B44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF5B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF5B48 size=104
    let mut pc: u32 = 0x82DF5B48;
    'dispatch: loop {
        match pc {
            0x82DF5B48 => {
    //   block [0x82DF5B48..0x82DF5BB0)
	// 82DF5B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF5B4C: 483B2615  bl 0x831a8160
	ctx.lr = 0x82DF5B50;
	sub_831A8130(ctx, base);
	// 82DF5B50: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF5B54: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82DF5B58: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DF5B5C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82DF5B60: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82DF5B64: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82DF5B68: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 82DF5B6C: 4BFFC33D  bl 0x82df1ea8
	ctx.lr = 0x82DF5B70;
	sub_82DF1EA8(ctx, base);
	// 82DF5B70: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82DF5B74: 41820030  beq 0x82df5ba4
	if ctx.cr[0].eq {
	pc = 0x82DF5BA4; continue 'dispatch;
	}
	// 82DF5B78: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82DF5B7C: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82DF5B80: 939F0004  stw r28, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82DF5B84: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DF5B88: 937F0008  stw r27, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[27].u32 ) };
	// 82DF5B8C: 4BFFE075  bl 0x82df3c00
	ctx.lr = 0x82DF5B90;
	sub_82DF3C00(ctx, base);
	// 82DF5B90: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF5B94: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF5B98: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82DF5B9C: 9B5F0014  stb r26, 0x14(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[26].u8 ) };
	// 82DF5BA0: 997F0015  stb r11, 0x15(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(21 as u32), ctx.r[11].u8 ) };
	// 82DF5BA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF5BA8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DF5BAC: 483B2604  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF5BB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF5BB0 size=92
    let mut pc: u32 = 0x82DF5BB0;
    'dispatch: loop {
        match pc {
            0x82DF5BB0 => {
    //   block [0x82DF5BB0..0x82DF5C0C)
	// 82DF5BB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF5BB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF5BB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF5BBC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DF5BC0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DF5BC4: 4BFFFC05  bl 0x82df57c8
	ctx.lr = 0x82DF5BC8;
	sub_82DF57C8(ctx, base);
	// 82DF5BC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DF5BCC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DF5BD0: 41820008  beq 0x82df5bd8
	if ctx.cr[0].eq {
	pc = 0x82DF5BD8; continue 'dispatch;
	}
	// 82DF5BD4: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DF5BD8: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF5BDC: 41820008  beq 0x82df5be4
	if ctx.cr[0].eq {
	pc = 0x82DF5BE4; continue 'dispatch;
	}
	// 82DF5BE0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DF5BE4: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF5BE8: 41820008  beq 0x82df5bf0
	if ctx.cr[0].eq {
	pc = 0x82DF5BF0; continue 'dispatch;
	}
	// 82DF5BEC: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DF5BF0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DF5BF4: 9943002D  stb r10, 0x2d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(45 as u32), ctx.r[10].u8 ) };
	// 82DF5BF8: 9963002C  stb r11, 0x2c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u8 ) };
	// 82DF5BFC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF5C00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF5C04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF5C08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF5C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF5C10 size=80
    let mut pc: u32 = 0x82DF5C10;
    'dispatch: loop {
        match pc {
            0x82DF5C10 => {
    //   block [0x82DF5C10..0x82DF5C60)
	// 82DF5C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF5C14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF5C18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF5C1C: 81640018  lwz r11, 0x18(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DF5C20: 80E40014  lwz r7, 0x14(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DF5C24: 2B0B0008  cmplwi cr6, r11, 8
	ctx.cr[6].compare_u32(ctx.r[11].u32, 8 as u32, &mut ctx.xer);
	// 82DF5C28: 4198000C  blt cr6, 0x82df5c34
	if ctx.cr[6].lt {
	pc = 0x82DF5C34; continue 'dispatch;
	}
	// 82DF5C2C: 80C40004  lwz r6, 4(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF5C30: 48000008  b 0x82df5c38
	pc = 0x82DF5C38; continue 'dispatch;
	// 82DF5C34: 38C40004  addi r6, r4, 4
	ctx.r[6].s64 = ctx.r[4].s64 + 4;
	// 82DF5C38: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DF5C3C: 80A30014  lwz r5, 0x14(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DF5C40: 4B753CA1  bl 0x825498e0
	ctx.lr = 0x82DF5C44;
	sub_825498E0(ctx, base);
	// 82DF5C44: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 82DF5C48: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82DF5C4C: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82DF5C50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF5C54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF5C58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF5C5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF5C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF5C60 size=88
    let mut pc: u32 = 0x82DF5C60;
    'dispatch: loop {
        match pc {
            0x82DF5C60 => {
    //   block [0x82DF5C60..0x82DF5CB8)
	// 82DF5C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF5C64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF5C68: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF5C6C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF5C70: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF5C74: 4BFFFE7D  bl 0x82df5af0
	ctx.lr = 0x82DF5C78;
	sub_82DF5AF0(ctx, base);
	// 82DF5C78: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DF5C7C: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82DF5C80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DF5C84: 99630015  stb r11, 0x15(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(21 as u32), ctx.r[11].u8 ) };
	// 82DF5C88: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF5C8C: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DF5C90: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF5C94: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF5C98: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF5C9C: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DF5CA0: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82DF5CA4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF5CA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF5CAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF5CB0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF5CB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF5CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF5CB8 size=88
    let mut pc: u32 = 0x82DF5CB8;
    'dispatch: loop {
        match pc {
            0x82DF5CB8 => {
    //   block [0x82DF5CB8..0x82DF5D10)
	// 82DF5CB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF5CBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF5CC0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF5CC4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF5CC8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF5CCC: 4BFFFEE5  bl 0x82df5bb0
	ctx.lr = 0x82DF5CD0;
	sub_82DF5BB0(ctx, base);
	// 82DF5CD0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DF5CD4: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82DF5CD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DF5CDC: 9963002D  stb r11, 0x2d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(45 as u32), ctx.r[11].u8 ) };
	// 82DF5CE0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF5CE4: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DF5CE8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF5CEC: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF5CF0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF5CF4: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DF5CF8: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82DF5CFC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF5D00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF5D04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF5D08: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF5D0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF5D10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF5D10 size=88
    let mut pc: u32 = 0x82DF5D10;
    'dispatch: loop {
        match pc {
            0x82DF5D10 => {
    //   block [0x82DF5D10..0x82DF5D68)
	// 82DF5D10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF5D14: 483B2459  bl 0x831a816c
	ctx.lr = 0x82DF5D18;
	sub_831A8130(ctx, base);
	// 82DF5D18: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF5D1C: 83C30004  lwz r30, 4(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF5D20: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DF5D24: 83FE0004  lwz r31, 4(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF5D28: 48000028  b 0x82df5d50
	pc = 0x82DF5D50; continue 'dispatch;
	// 82DF5D2C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DF5D30: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82DF5D34: 4BFFFEDD  bl 0x82df5c10
	ctx.lr = 0x82DF5D38;
	sub_82DF5C10(ctx, base);
	// 82DF5D38: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF5D3C: 4182000C  beq 0x82df5d48
	if ctx.cr[0].eq {
	pc = 0x82DF5D48; continue 'dispatch;
	}
	// 82DF5D40: 83FF0008  lwz r31, 8(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF5D44: 4800000C  b 0x82df5d50
	pc = 0x82DF5D50; continue 'dispatch;
	// 82DF5D48: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	// 82DF5D4C: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF5D50: 897F002D  lbz r11, 0x2d(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(45 as u32) ) } as u64;
	// 82DF5D54: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF5D58: 419AFFD4  beq cr6, 0x82df5d2c
	if ctx.cr[6].eq {
	pc = 0x82DF5D2C; continue 'dispatch;
	}
	// 82DF5D5C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF5D60: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF5D64: 483B2458  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF5D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF5D68 size=88
    let mut pc: u32 = 0x82DF5D68;
    'dispatch: loop {
        match pc {
            0x82DF5D68 => {
    //   block [0x82DF5D68..0x82DF5DC0)
	// 82DF5D68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF5D6C: 483B2401  bl 0x831a816c
	ctx.lr = 0x82DF5D70;
	sub_831A8130(ctx, base);
	// 82DF5D70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF5D74: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DF5D78: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DF5D7C: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 82DF5D80: 897E0015  lbz r11, 0x15(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(21 as u32) ) } as u64;
	// 82DF5D84: 4800002C  b 0x82df5db0
	pc = 0x82DF5DB0; continue 'dispatch;
	// 82DF5D88: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DF5D8C: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF5D90: 4BFFFFD9  bl 0x82df5d68
	ctx.lr = 0x82DF5D94;
	sub_82DF5D68(ctx, base);
	// 82DF5D94: 387E000C  addi r3, r30, 0xc
	ctx.r[3].s64 = ctx.r[30].s64 + 12;
	// 82DF5D98: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF5D9C: 4BFFD68D  bl 0x82df3428
	ctx.lr = 0x82DF5DA0;
	sub_82DF3428(ctx, base);
	// 82DF5DA0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF5DA4: 4B4CA4C5  bl 0x822c0268
	ctx.lr = 0x82DF5DA8;
	sub_822C0268(ctx, base);
	// 82DF5DA8: 897F0015  lbz r11, 0x15(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(21 as u32) ) } as u64;
	// 82DF5DAC: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	// 82DF5DB0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF5DB4: 419AFFD4  beq cr6, 0x82df5d88
	if ctx.cr[6].eq {
	pc = 0x82DF5D88; continue 'dispatch;
	}
	// 82DF5DB8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF5DBC: 483B2400  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF5DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF5DC0 size=88
    let mut pc: u32 = 0x82DF5DC0;
    'dispatch: loop {
        match pc {
            0x82DF5DC0 => {
    //   block [0x82DF5DC0..0x82DF5E18)
	// 82DF5DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF5DC4: 483B23A9  bl 0x831a816c
	ctx.lr = 0x82DF5DC8;
	sub_831A8130(ctx, base);
	// 82DF5DC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF5DCC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DF5DD0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DF5DD4: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 82DF5DD8: 897E0015  lbz r11, 0x15(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(21 as u32) ) } as u64;
	// 82DF5DDC: 4800002C  b 0x82df5e08
	pc = 0x82DF5E08; continue 'dispatch;
	// 82DF5DE0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DF5DE4: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF5DE8: 4BFFFFD9  bl 0x82df5dc0
	ctx.lr = 0x82DF5DEC;
	sub_82DF5DC0(ctx, base);
	// 82DF5DEC: 387E000C  addi r3, r30, 0xc
	ctx.r[3].s64 = ctx.r[30].s64 + 12;
	// 82DF5DF0: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF5DF4: 4BFFD635  bl 0x82df3428
	ctx.lr = 0x82DF5DF8;
	sub_82DF3428(ctx, base);
	// 82DF5DF8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF5DFC: 4BFFC0B5  bl 0x82df1eb0
	ctx.lr = 0x82DF5E00;
	sub_82DF1EB0(ctx, base);
	// 82DF5E00: 897F0015  lbz r11, 0x15(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(21 as u32) ) } as u64;
	// 82DF5E04: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	// 82DF5E08: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF5E0C: 419AFFD4  beq cr6, 0x82df5de0
	if ctx.cr[6].eq {
	pc = 0x82DF5DE0; continue 'dispatch;
	}
	// 82DF5E10: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF5E14: 483B23A8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF5E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF5E18 size=216
    let mut pc: u32 = 0x82DF5E18;
    'dispatch: loop {
        match pc {
            0x82DF5E18 => {
    //   block [0x82DF5E18..0x82DF5EF0)
	// 82DF5E18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF5E1C: 483B2351  bl 0x831a816c
	ctx.lr = 0x82DF5E20;
	sub_831A8130(ctx, base);
	// 82DF5E20: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF5E24: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DF5E28: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DF5E2C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DF5E30: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DF5E34: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF5E38: 419A0018  beq cr6, 0x82df5e50
	if ctx.cr[6].eq {
	pc = 0x82DF5E50; continue 'dispatch;
	}
	// 82DF5E3C: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82DF5E40: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF5E44: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF5E48: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DF5E4C: 4E800421  bctrl
	ctx.lr = 0x82DF5E50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DF5E50: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DF5E54: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF5E58: 419A005C  beq cr6, 0x82df5eb4
	if ctx.cr[6].eq {
	pc = 0x82DF5EB4; continue 'dispatch;
	}
	// 82DF5E5C: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DF5E60: 3920001C  li r9, 0x1c
	ctx.r[9].s64 = 28;
	// 82DF5E64: 811E0000  lwz r8, 0(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF5E68: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82DF5E6C: 7D6B4BD6  divw r11, r11, r9
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[9].s32;
	// 82DF5E70: 7F085840  cmplw cr6, r8, r11
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF5E74: 40980040  bge cr6, 0x82df5eb4
	if !ctx.cr[6].lt {
	pc = 0x82DF5EB4; continue 'dispatch;
	}
	// 82DF5E78: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DF5E7C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF5E80: 419A0018  beq cr6, 0x82df5e98
	if ctx.cr[6].eq {
	pc = 0x82DF5E98; continue 'dispatch;
	}
	// 82DF5E84: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82DF5E88: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF5E8C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DF5E90: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DF5E94: 4E800421  bctrl
	ctx.lr = 0x82DF5E98;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DF5E98: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF5E9C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DF5EA0: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DF5EA4: 1D4A001C  mulli r10, r10, 0x1c
	ctx.r[10].s64 = ctx.r[10].s64 * 28;
	// 82DF5EA8: 7C8A5A14  add r4, r10, r11
	ctx.r[4].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DF5EAC: 4B4D3FAD  bl 0x822c9e58
	ctx.lr = 0x82DF5EB0;
	sub_822C9E58(ctx, base);
	// 82DF5EB0: 48000034  b 0x82df5ee4
	pc = 0x82DF5EE4; continue 'dispatch;
	// 82DF5EB4: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DF5EB8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF5EBC: 419A0018  beq cr6, 0x82df5ed4
	if ctx.cr[6].eq {
	pc = 0x82DF5ED4; continue 'dispatch;
	}
	// 82DF5EC0: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82DF5EC4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF5EC8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DF5ECC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DF5ED0: 4E800421  bctrl
	ctx.lr = 0x82DF5ED4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DF5ED4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF5ED8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DF5EDC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DF5EE0: 4B4D2F51  bl 0x822c8e30
	ctx.lr = 0x82DF5EE4;
	sub_822C8E30(ctx, base);
	// 82DF5EE4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DF5EE8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF5EEC: 483B22D0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF5EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF5EF0 size=112
    let mut pc: u32 = 0x82DF5EF0;
    'dispatch: loop {
        match pc {
            0x82DF5EF0 => {
    //   block [0x82DF5EF0..0x82DF5F60)
	// 82DF5EF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF5EF4: 483B2279  bl 0x831a816c
	ctx.lr = 0x82DF5EF8;
	sub_831A8130(ctx, base);
	// 82DF5EF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF5EFC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DF5F00: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DF5F04: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DF5F08: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DF5F0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF5F10: 4BFFFE01  bl 0x82df5d10
	ctx.lr = 0x82DF5F14;
	sub_82DF5D10(ctx, base);
	// 82DF5F14: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF5F18: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82DF5F1C: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF5F20: 419A0020  beq cr6, 0x82df5f40
	if ctx.cr[6].eq {
	pc = 0x82DF5F40; continue 'dispatch;
	}
	// 82DF5F24: 3883000C  addi r4, r3, 0xc
	ctx.r[4].s64 = ctx.r[3].s64 + 12;
	// 82DF5F28: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF5F2C: 4BFFFCE5  bl 0x82df5c10
	ctx.lr = 0x82DF5F30;
	sub_82DF5C10(ctx, base);
	// 82DF5F30: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF5F34: 4082000C  bne 0x82df5f40
	if !ctx.cr[0].eq {
	pc = 0x82DF5F40; continue 'dispatch;
	}
	// 82DF5F38: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82DF5F3C: 48000010  b 0x82df5f4c
	pc = 0x82DF5F4C; continue 'dispatch;
	// 82DF5F40: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF5F44: 39610054  addi r11, r1, 0x54
	ctx.r[11].s64 = ctx.r[1].s64 + 84;
	// 82DF5F48: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82DF5F4C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF5F50: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DF5F54: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF5F58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DF5F5C: 483B2260  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF5F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF5F60 size=88
    let mut pc: u32 = 0x82DF5F60;
    'dispatch: loop {
        match pc {
            0x82DF5F60 => {
    //   block [0x82DF5F60..0x82DF5FB8)
	// 82DF5F60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF5F64: 483B2201  bl 0x831a8164
	ctx.lr = 0x82DF5F68;
	sub_831A8130(ctx, base);
	// 82DF5F68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF5F6C: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82DF5F70: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82DF5F74: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82DF5F78: 7FFDFB78  mr r29, r31
	ctx.r[29].u64 = ctx.r[31].u64;
	// 82DF5F7C: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 82DF5F80: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82DF5F84: 419A0024  beq cr6, 0x82df5fa8
	if ctx.cr[6].eq {
	pc = 0x82DF5FA8; continue 'dispatch;
	}
	// 82DF5F88: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82DF5F8C: 419A0010  beq cr6, 0x82df5f9c
	if ctx.cr[6].eq {
	pc = 0x82DF5F9C; continue 'dispatch;
	}
	// 82DF5F90: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82DF5F94: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF5F98: 4BFFDC69  bl 0x82df3c00
	ctx.lr = 0x82DF5F9C;
	sub_82DF3C00(ctx, base);
	// 82DF5F9C: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82DF5FA0: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82DF5FA4: 4082FFE4  bne 0x82df5f88
	if !ctx.cr[0].eq {
	pc = 0x82DF5F88; continue 'dispatch;
	}
	// 82DF5FA8: 57EB103A  slwi r11, r31, 2
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DF5FAC: 7C6BE214  add r3, r11, r28
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82DF5FB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DF5FB4: 483B2200  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF5FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF5FB8 size=84
    let mut pc: u32 = 0x82DF5FB8;
    'dispatch: loop {
        match pc {
            0x82DF5FB8 => {
    //   block [0x82DF5FB8..0x82DF600C)
	// 82DF5FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF5FBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF5FC0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF5FC4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF5FC8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF5FCC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF5FD0: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF5FD4: 4BFFFD95  bl 0x82df5d68
	ctx.lr = 0x82DF5FD8;
	sub_82DF5D68(ctx, base);
	// 82DF5FD8: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF5FDC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF5FE0: 914A0004  stw r10, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DF5FE4: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DF5FE8: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF5FEC: 914A0000  stw r10, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DF5FF0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF5FF4: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DF5FF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF5FFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF6000: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF6004: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF6008: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF6010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF6010 size=84
    let mut pc: u32 = 0x82DF6010;
    'dispatch: loop {
        match pc {
            0x82DF6010 => {
    //   block [0x82DF6010..0x82DF6064)
	// 82DF6010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF6014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF6018: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF601C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF6020: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF6024: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF6028: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF602C: 4BFFFD95  bl 0x82df5dc0
	ctx.lr = 0x82DF6030;
	sub_82DF5DC0(ctx, base);
	// 82DF6030: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF6034: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF6038: 914A0004  stw r10, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DF603C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DF6040: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF6044: 914A0000  stw r10, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DF6048: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF604C: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DF6050: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF6054: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF6058: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF605C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF6060: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF6068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF6068 size=148
    let mut pc: u32 = 0x82DF6068;
    'dispatch: loop {
        match pc {
            0x82DF6068 => {
    //   block [0x82DF6068..0x82DF60FC)
	// 82DF6068: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF606C: 3921FFF0  addi r9, r1, -0x10
	ctx.r[9].s64 = ctx.r[1].s64 + -16;
	// 82DF6070: 81030008  lwz r8, 8(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF6074: 38E1FFF0  addi r7, r1, -0x10
	ctx.r[7].s64 = ctx.r[1].s64 + -16;
	// 82DF6078: 80C3000C  lwz r6, 0xc(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DF607C: 39630004  addi r11, r3, 4
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	// 82DF6080: 80A30010  lwz r5, 0x10(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DF6084: 39640004  addi r11, r4, 4
	ctx.r[11].s64 = ctx.r[4].s64 + 4;
	// 82DF6088: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF608C: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DF6090: 91090004  stw r8, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DF6094: 90C90008  stw r6, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 82DF6098: 90A9000C  stw r5, 0xc(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(12 as u32), ctx.r[5].u32 ) };
	// 82DF609C: 81470000  lwz r10, 0(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF60A0: 81270004  lwz r9, 4(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF60A4: 81070008  lwz r8, 8(r7)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF60A8: 80E7000C  lwz r7, 0xc(r7)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DF60AC: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DF60B0: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF60B4: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DF60B8: 8164000C  lwz r11, 0xc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DF60BC: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82DF60C0: 81640010  lwz r11, 0x10(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DF60C4: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82DF60C8: 91440004  stw r10, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DF60CC: 91240008  stw r9, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82DF60D0: 9104000C  stw r8, 0xc(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82DF60D4: 90E40010  stw r7, 0x10(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(16 as u32), ctx.r[7].u32 ) };
	// 82DF60D8: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DF60DC: 81440014  lwz r10, 0x14(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DF60E0: 91430014  stw r10, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82DF60E4: 91640014  stw r11, 0x14(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82DF60E8: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DF60EC: 81440018  lwz r10, 0x18(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DF60F0: 91430018  stw r10, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 82DF60F4: 91640018  stw r11, 0x18(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82DF60F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF6100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF6100 size=88
    let mut pc: u32 = 0x82DF6100;
    'dispatch: loop {
        match pc {
            0x82DF6100 => {
    //   block [0x82DF6100..0x82DF6158)
	// 82DF6100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF6104: 483B2069  bl 0x831a816c
	ctx.lr = 0x82DF6108;
	sub_831A8130(ctx, base);
	// 82DF6108: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF610C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF6110: 83DF0004  lwz r30, 4(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF6114: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82DF6118: 419A0028  beq cr6, 0x82df6140
	if ctx.cr[6].eq {
	pc = 0x82DF6140; continue 'dispatch;
	}
	// 82DF611C: 83BF0008  lwz r29, 8(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF6120: 48000010  b 0x82df6130
	pc = 0x82DF6130; continue 'dispatch;
	// 82DF6124: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF6128: 4BFFD301  bl 0x82df3428
	ctx.lr = 0x82DF612C;
	sub_82DF3428(ctx, base);
	// 82DF612C: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82DF6130: 7F1EE840  cmplw cr6, r30, r29
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82DF6134: 409AFFF0  bne cr6, 0x82df6124
	if !ctx.cr[6].eq {
	pc = 0x82DF6124; continue 'dispatch;
	}
	// 82DF6138: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF613C: 4BFFBD75  bl 0x82df1eb0
	ctx.lr = 0x82DF6140;
	sub_82DF1EB0(ctx, base);
	// 82DF6140: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF6144: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DF6148: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DF614C: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82DF6150: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF6154: 483B2068  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF6158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF6158 size=108
    let mut pc: u32 = 0x82DF6158;
    'dispatch: loop {
        match pc {
            0x82DF6158 => {
    //   block [0x82DF6158..0x82DF61C4)
	// 82DF6158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF615C: 483B2005  bl 0x831a8160
	ctx.lr = 0x82DF6160;
	sub_831A8130(ctx, base);
	// 82DF6160: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF6164: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DF6168: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DF616C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DF6170: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82DF6174: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82DF6178: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82DF617C: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 82DF6180: 4BFFF649  bl 0x82df57c8
	ctx.lr = 0x82DF6184;
	sub_82DF57C8(ctx, base);
	// 82DF6184: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82DF6188: 41820030  beq 0x82df61b8
	if ctx.cr[0].eq {
	pc = 0x82DF61B8; continue 'dispatch;
	}
	// 82DF618C: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82DF6190: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82DF6194: 939F0004  stw r28, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82DF6198: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DF619C: 937F0008  stw r27, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[27].u32 ) };
	// 82DF61A0: 4B4D3CB9  bl 0x822c9e58
	ctx.lr = 0x82DF61A4;
	sub_822C9E58(ctx, base);
	// 82DF61A4: 815E001C  lwz r10, 0x1c(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DF61A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF61AC: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 82DF61B0: 9B5F002C  stb r26, 0x2c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[26].u8 ) };
	// 82DF61B4: 997F002D  stb r11, 0x2d(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(45 as u32), ctx.r[11].u8 ) };
	// 82DF61B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF61BC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DF61C0: 483B1FF0  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF61C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF61C8 size=548
    let mut pc: u32 = 0x82DF61C8;
    'dispatch: loop {
        match pc {
            0x82DF61C8 => {
    //   block [0x82DF61C8..0x82DF63EC)
	// 82DF61C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF61CC: 483B1F95  bl 0x831a8160
	ctx.lr = 0x82DF61D0;
	sub_831A8130(ctx, base);
	// 82DF61D0: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF61D4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DF61D8: 3D601FFF  lis r11, 0x1fff
	ctx.r[11].s64 = 536805376;
	// 82DF61DC: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82DF61E0: 616BFFFE  ori r11, r11, 0xfffe
	ctx.r[11].u64 = ctx.r[11].u64 | 65534;
	// 82DF61E4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82DF61E8: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF61EC: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82DF61F0: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 82DF61F4: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF61F8: 41980048  blt cr6, 0x82df6240
	if ctx.cr[6].lt {
	pc = 0x82DF6240; continue 'dispatch;
	}
	// 82DF61FC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82DF6200: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF6204: 388B9BCC  addi r4, r11, -0x6434
	ctx.r[4].s64 = ctx.r[11].s64 + -25652;
	// 82DF6208: 4B4CF6C1  bl 0x822c58c8
	ctx.lr = 0x82DF620C;
	sub_822C58C8(ctx, base);
	// 82DF620C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DF6210: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82DF6214: 4B4CF605  bl 0x822c5818
	ctx.lr = 0x82DF6218;
	sub_822C5818(ctx, base);
	// 82DF6218: 4B4CE099  bl 0x822c42b0
	ctx.lr = 0x82DF621C;
	sub_822C42B0(ctx, base);
	// 82DF621C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82DF6220: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82DF6224: 396B94A0  addi r11, r11, -0x6b60
	ctx.r[11].s64 = ctx.r[11].s64 + -27488;
	// 82DF6228: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82DF622C: 4B4CF245  bl 0x822c5470
	ctx.lr = 0x82DF6230;
	sub_822C5470(ctx, base);
	// 82DF6230: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF6234: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DF6238: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF623C: 4B4CEAA5  bl 0x822c4ce0
	ctx.lr = 0x82DF6240;
	sub_822C4CE0(ctx, base);
	// 82DF6240: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF6244: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82DF6248: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82DF624C: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 82DF6250: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DF6254: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF6258: 4BFFF829  bl 0x82df5a80
	ctx.lr = 0x82DF625C;
	sub_82DF5A80(ctx, base);
	// 82DF625C: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF6260: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF6264: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82DF6268: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DF626C: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF6270: 915E0008  stw r10, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82DF6274: 409A0018  bne cr6, 0x82df628c
	if !ctx.cr[6].eq {
	pc = 0x82DF628C; continue 'dispatch;
	}
	// 82DF6278: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82DF627C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF6280: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82DF6284: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF6288: 4800003C  b 0x82df62c4
	pc = 0x82DF62C4; continue 'dispatch;
	// 82DF628C: 576B063F  clrlwi. r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF6290: 41820020  beq 0x82df62b0
	if ctx.cr[0].eq {
	pc = 0x82DF62B0; continue 'dispatch;
	}
	// 82DF6294: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82DF6298: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF629C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF62A0: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DF62A4: 409A0024  bne cr6, 0x82df62c8
	if !ctx.cr[6].eq {
	pc = 0x82DF62C8; continue 'dispatch;
	}
	// 82DF62A8: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82DF62AC: 4800001C  b 0x82df62c8
	pc = 0x82DF62C8; continue 'dispatch;
	// 82DF62B0: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82DF62B4: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF62B8: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF62BC: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DF62C0: 409A0008  bne cr6, 0x82df62c8
	if !ctx.cr[6].eq {
	pc = 0x82DF62C8; continue 'dispatch;
	}
	// 82DF62C4: 938B0008  stw r28, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82DF62C8: 815C0004  lwz r10, 4(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF62CC: 397C0004  addi r11, r28, 4
	ctx.r[11].s64 = ctx.r[28].s64 + 4;
	// 82DF62D0: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 82DF62D4: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	// 82DF62D8: 894A0014  lbz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DF62DC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF62E0: 409A00F0  bne cr6, 0x82df63d0
	if !ctx.cr[6].eq {
	pc = 0x82DF63D0; continue 'dispatch;
	}
	// 82DF62E4: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82DF62E8: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF62EC: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF62F0: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF62F4: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DF62F8: 409A0054  bne cr6, 0x82df634c
	if !ctx.cr[6].eq {
	pc = 0x82DF634C; continue 'dispatch;
	}
	// 82DF62FC: 814A0008  lwz r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF6300: 892A0014  lbz r9, 0x14(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DF6304: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DF6308: 419A0054  beq cr6, 0x82df635c
	if ctx.cr[6].eq {
	pc = 0x82DF635C; continue 'dispatch;
	}
	// 82DF630C: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF6310: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF6314: 409A0010  bne cr6, 0x82df6324
	if !ctx.cr[6].eq {
	pc = 0x82DF6324; continue 'dispatch;
	}
	// 82DF6318: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF631C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DF6320: 4BFFF691  bl 0x82df59b0
	ctx.lr = 0x82DF6324;
	sub_82DF59B0(ctx, base);
	// 82DF6324: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF6328: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF632C: 9BAB0014  stb r29, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[29].u8 ) };
	// 82DF6330: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF6334: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF6338: 9B6B0014  stb r27, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[27].u8 ) };
	// 82DF633C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF6340: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF6344: 4806A73D  bl 0x82e60a80
	ctx.lr = 0x82DF6348;
	sub_82E60A80(ctx, base);
	// 82DF6348: 48000074  b 0x82df63bc
	pc = 0x82DF63BC; continue 'dispatch;
	// 82DF634C: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF6350: 892A0014  lbz r9, 0x14(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DF6354: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DF6358: 409A0028  bne cr6, 0x82df6380
	if !ctx.cr[6].eq {
	pc = 0x82DF6380; continue 'dispatch;
	}
	// 82DF635C: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF6360: 9BA90014  stb r29, 0x14(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(20 as u32), ctx.r[29].u8 ) };
	// 82DF6364: 9BAA0014  stb r29, 0x14(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(20 as u32), ctx.r[29].u8 ) };
	// 82DF6368: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF636C: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF6370: 9B6A0014  stb r27, 0x14(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(20 as u32), ctx.r[27].u8 ) };
	// 82DF6374: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF6378: 83EB0004  lwz r31, 4(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF637C: 48000040  b 0x82df63bc
	pc = 0x82DF63BC; continue 'dispatch;
	// 82DF6380: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF6384: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF6388: 409A0010  bne cr6, 0x82df6398
	if !ctx.cr[6].eq {
	pc = 0x82DF6398; continue 'dispatch;
	}
	// 82DF638C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF6390: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DF6394: 4806A6ED  bl 0x82e60a80
	ctx.lr = 0x82DF6398;
	sub_82E60A80(ctx, base);
	// 82DF6398: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF639C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF63A0: 9BAB0014  stb r29, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[29].u8 ) };
	// 82DF63A4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF63A8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF63AC: 9B6B0014  stb r27, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[27].u8 ) };
	// 82DF63B0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF63B4: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF63B8: 4BFFF5F9  bl 0x82df59b0
	ctx.lr = 0x82DF63BC;
	sub_82DF59B0(ctx, base);
	// 82DF63BC: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF63C0: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 82DF63C4: 894A0014  lbz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DF63C8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF63CC: 419AFF1C  beq cr6, 0x82df62e8
	if ctx.cr[6].eq {
	pc = 0x82DF62E8; continue 'dispatch;
	}
	// 82DF63D0: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF63D4: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82DF63D8: 939A0000  stw r28, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82DF63DC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF63E0: 9BAB0014  stb r29, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[29].u8 ) };
	// 82DF63E4: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82DF63E8: 483B1DC8  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF63F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF63F0 size=548
    let mut pc: u32 = 0x82DF63F0;
    'dispatch: loop {
        match pc {
            0x82DF63F0 => {
    //   block [0x82DF63F0..0x82DF6614)
	// 82DF63F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF63F4: 483B1D6D  bl 0x831a8160
	ctx.lr = 0x82DF63F8;
	sub_831A8130(ctx, base);
	// 82DF63F8: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF63FC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DF6400: 3D601FFF  lis r11, 0x1fff
	ctx.r[11].s64 = 536805376;
	// 82DF6404: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82DF6408: 616BFFFE  ori r11, r11, 0xfffe
	ctx.r[11].u64 = ctx.r[11].u64 | 65534;
	// 82DF640C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82DF6410: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF6414: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82DF6418: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 82DF641C: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF6420: 41980048  blt cr6, 0x82df6468
	if ctx.cr[6].lt {
	pc = 0x82DF6468; continue 'dispatch;
	}
	// 82DF6424: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82DF6428: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF642C: 388B9BCC  addi r4, r11, -0x6434
	ctx.r[4].s64 = ctx.r[11].s64 + -25652;
	// 82DF6430: 4B4CF499  bl 0x822c58c8
	ctx.lr = 0x82DF6434;
	sub_822C58C8(ctx, base);
	// 82DF6434: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DF6438: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82DF643C: 4B4CF3DD  bl 0x822c5818
	ctx.lr = 0x82DF6440;
	sub_822C5818(ctx, base);
	// 82DF6440: 4B4CDE71  bl 0x822c42b0
	ctx.lr = 0x82DF6444;
	sub_822C42B0(ctx, base);
	// 82DF6444: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82DF6448: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82DF644C: 396B94A0  addi r11, r11, -0x6b60
	ctx.r[11].s64 = ctx.r[11].s64 + -27488;
	// 82DF6450: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82DF6454: 4B4CF01D  bl 0x822c5470
	ctx.lr = 0x82DF6458;
	sub_822C5470(ctx, base);
	// 82DF6458: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF645C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DF6460: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF6464: 4B4CE87D  bl 0x822c4ce0
	ctx.lr = 0x82DF6468;
	sub_822C4CE0(ctx, base);
	// 82DF6468: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF646C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82DF6470: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82DF6474: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 82DF6478: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DF647C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF6480: 4BFFF6C9  bl 0x82df5b48
	ctx.lr = 0x82DF6484;
	sub_82DF5B48(ctx, base);
	// 82DF6484: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF6488: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF648C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82DF6490: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DF6494: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF6498: 915E0008  stw r10, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82DF649C: 409A0018  bne cr6, 0x82df64b4
	if !ctx.cr[6].eq {
	pc = 0x82DF64B4; continue 'dispatch;
	}
	// 82DF64A0: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82DF64A4: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF64A8: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82DF64AC: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF64B0: 4800003C  b 0x82df64ec
	pc = 0x82DF64EC; continue 'dispatch;
	// 82DF64B4: 576B063F  clrlwi. r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF64B8: 41820020  beq 0x82df64d8
	if ctx.cr[0].eq {
	pc = 0x82DF64D8; continue 'dispatch;
	}
	// 82DF64BC: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82DF64C0: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF64C4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF64C8: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DF64CC: 409A0024  bne cr6, 0x82df64f0
	if !ctx.cr[6].eq {
	pc = 0x82DF64F0; continue 'dispatch;
	}
	// 82DF64D0: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82DF64D4: 4800001C  b 0x82df64f0
	pc = 0x82DF64F0; continue 'dispatch;
	// 82DF64D8: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82DF64DC: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF64E0: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF64E4: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DF64E8: 409A0008  bne cr6, 0x82df64f0
	if !ctx.cr[6].eq {
	pc = 0x82DF64F0; continue 'dispatch;
	}
	// 82DF64EC: 938B0008  stw r28, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82DF64F0: 815C0004  lwz r10, 4(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF64F4: 397C0004  addi r11, r28, 4
	ctx.r[11].s64 = ctx.r[28].s64 + 4;
	// 82DF64F8: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 82DF64FC: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	// 82DF6500: 894A0014  lbz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DF6504: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF6508: 409A00F0  bne cr6, 0x82df65f8
	if !ctx.cr[6].eq {
	pc = 0x82DF65F8; continue 'dispatch;
	}
	// 82DF650C: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82DF6510: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF6514: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF6518: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF651C: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DF6520: 409A0054  bne cr6, 0x82df6574
	if !ctx.cr[6].eq {
	pc = 0x82DF6574; continue 'dispatch;
	}
	// 82DF6524: 814A0008  lwz r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF6528: 892A0014  lbz r9, 0x14(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DF652C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DF6530: 419A0054  beq cr6, 0x82df6584
	if ctx.cr[6].eq {
	pc = 0x82DF6584; continue 'dispatch;
	}
	// 82DF6534: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF6538: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF653C: 409A0010  bne cr6, 0x82df654c
	if !ctx.cr[6].eq {
	pc = 0x82DF654C; continue 'dispatch;
	}
	// 82DF6540: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF6544: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DF6548: 4BFFF469  bl 0x82df59b0
	ctx.lr = 0x82DF654C;
	sub_82DF59B0(ctx, base);
	// 82DF654C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF6550: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF6554: 9BAB0014  stb r29, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[29].u8 ) };
	// 82DF6558: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF655C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF6560: 9B6B0014  stb r27, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[27].u8 ) };
	// 82DF6564: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF6568: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF656C: 4806A515  bl 0x82e60a80
	ctx.lr = 0x82DF6570;
	sub_82E60A80(ctx, base);
	// 82DF6570: 48000074  b 0x82df65e4
	pc = 0x82DF65E4; continue 'dispatch;
	// 82DF6574: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF6578: 892A0014  lbz r9, 0x14(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DF657C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DF6580: 409A0028  bne cr6, 0x82df65a8
	if !ctx.cr[6].eq {
	pc = 0x82DF65A8; continue 'dispatch;
	}
	// 82DF6584: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF6588: 9BA90014  stb r29, 0x14(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(20 as u32), ctx.r[29].u8 ) };
	// 82DF658C: 9BAA0014  stb r29, 0x14(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(20 as u32), ctx.r[29].u8 ) };
	// 82DF6590: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF6594: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF6598: 9B6A0014  stb r27, 0x14(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(20 as u32), ctx.r[27].u8 ) };
	// 82DF659C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF65A0: 83EB0004  lwz r31, 4(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF65A4: 48000040  b 0x82df65e4
	pc = 0x82DF65E4; continue 'dispatch;
	// 82DF65A8: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF65AC: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF65B0: 409A0010  bne cr6, 0x82df65c0
	if !ctx.cr[6].eq {
	pc = 0x82DF65C0; continue 'dispatch;
	}
	// 82DF65B4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF65B8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DF65BC: 4806A4C5  bl 0x82e60a80
	ctx.lr = 0x82DF65C0;
	sub_82E60A80(ctx, base);
	// 82DF65C0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF65C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF65C8: 9BAB0014  stb r29, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[29].u8 ) };
	// 82DF65CC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF65D0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF65D4: 9B6B0014  stb r27, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[27].u8 ) };
	// 82DF65D8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF65DC: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF65E0: 4BFFF3D1  bl 0x82df59b0
	ctx.lr = 0x82DF65E4;
	sub_82DF59B0(ctx, base);
	// 82DF65E4: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF65E8: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 82DF65EC: 894A0014  lbz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DF65F0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF65F4: 419AFF1C  beq cr6, 0x82df6510
	if ctx.cr[6].eq {
	pc = 0x82DF6510; continue 'dispatch;
	}
	// 82DF65F8: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF65FC: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82DF6600: 939A0000  stw r28, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82DF6604: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF6608: 9BAB0014  stb r29, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[29].u8 ) };
	// 82DF660C: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82DF6610: 483B1BA0  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF6618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF6618 size=548
    let mut pc: u32 = 0x82DF6618;
    'dispatch: loop {
        match pc {
            0x82DF6618 => {
    //   block [0x82DF6618..0x82DF683C)
	// 82DF6618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF661C: 483B1B45  bl 0x831a8160
	ctx.lr = 0x82DF6620;
	sub_831A8130(ctx, base);
	// 82DF6620: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF6624: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DF6628: 3D6007FF  lis r11, 0x7ff
	ctx.r[11].s64 = 134152192;
	// 82DF662C: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82DF6630: 616BFFFE  ori r11, r11, 0xfffe
	ctx.r[11].u64 = ctx.r[11].u64 | 65534;
	// 82DF6634: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82DF6638: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF663C: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82DF6640: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 82DF6644: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF6648: 41980048  blt cr6, 0x82df6690
	if ctx.cr[6].lt {
	pc = 0x82DF6690; continue 'dispatch;
	}
	// 82DF664C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82DF6650: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF6654: 388B9BCC  addi r4, r11, -0x6434
	ctx.r[4].s64 = ctx.r[11].s64 + -25652;
	// 82DF6658: 4B4CF271  bl 0x822c58c8
	ctx.lr = 0x82DF665C;
	sub_822C58C8(ctx, base);
	// 82DF665C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DF6660: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82DF6664: 4B4CF1B5  bl 0x822c5818
	ctx.lr = 0x82DF6668;
	sub_822C5818(ctx, base);
	// 82DF6668: 4B4CDC49  bl 0x822c42b0
	ctx.lr = 0x82DF666C;
	sub_822C42B0(ctx, base);
	// 82DF666C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82DF6670: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82DF6674: 396B94A0  addi r11, r11, -0x6b60
	ctx.r[11].s64 = ctx.r[11].s64 + -27488;
	// 82DF6678: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82DF667C: 4B4CEDF5  bl 0x822c5470
	ctx.lr = 0x82DF6680;
	sub_822C5470(ctx, base);
	// 82DF6680: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF6684: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DF6688: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF668C: 4B4CE655  bl 0x822c4ce0
	ctx.lr = 0x82DF6690;
	sub_822C4CE0(ctx, base);
	// 82DF6690: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF6694: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82DF6698: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82DF669C: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 82DF66A0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DF66A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF66A8: 4BFFFAB1  bl 0x82df6158
	ctx.lr = 0x82DF66AC;
	sub_82DF6158(ctx, base);
	// 82DF66AC: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF66B0: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF66B4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82DF66B8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DF66BC: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF66C0: 915E0008  stw r10, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82DF66C4: 409A0018  bne cr6, 0x82df66dc
	if !ctx.cr[6].eq {
	pc = 0x82DF66DC; continue 'dispatch;
	}
	// 82DF66C8: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82DF66CC: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF66D0: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82DF66D4: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF66D8: 4800003C  b 0x82df6714
	pc = 0x82DF6714; continue 'dispatch;
	// 82DF66DC: 576B063F  clrlwi. r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF66E0: 41820020  beq 0x82df6700
	if ctx.cr[0].eq {
	pc = 0x82DF6700; continue 'dispatch;
	}
	// 82DF66E4: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82DF66E8: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF66EC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF66F0: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DF66F4: 409A0024  bne cr6, 0x82df6718
	if !ctx.cr[6].eq {
	pc = 0x82DF6718; continue 'dispatch;
	}
	// 82DF66F8: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82DF66FC: 4800001C  b 0x82df6718
	pc = 0x82DF6718; continue 'dispatch;
	// 82DF6700: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82DF6704: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF6708: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF670C: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DF6710: 409A0008  bne cr6, 0x82df6718
	if !ctx.cr[6].eq {
	pc = 0x82DF6718; continue 'dispatch;
	}
	// 82DF6714: 938B0008  stw r28, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82DF6718: 815C0004  lwz r10, 4(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF671C: 397C0004  addi r11, r28, 4
	ctx.r[11].s64 = ctx.r[28].s64 + 4;
	// 82DF6720: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 82DF6724: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	// 82DF6728: 894A002C  lbz r10, 0x2c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DF672C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF6730: 409A00F0  bne cr6, 0x82df6820
	if !ctx.cr[6].eq {
	pc = 0x82DF6820; continue 'dispatch;
	}
	// 82DF6734: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82DF6738: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF673C: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF6740: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF6744: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DF6748: 409A0054  bne cr6, 0x82df679c
	if !ctx.cr[6].eq {
	pc = 0x82DF679C; continue 'dispatch;
	}
	// 82DF674C: 814A0008  lwz r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF6750: 892A002C  lbz r9, 0x2c(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DF6754: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DF6758: 419A0054  beq cr6, 0x82df67ac
	if ctx.cr[6].eq {
	pc = 0x82DF67AC; continue 'dispatch;
	}
	// 82DF675C: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF6760: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF6764: 409A0010  bne cr6, 0x82df6774
	if !ctx.cr[6].eq {
	pc = 0x82DF6774; continue 'dispatch;
	}
	// 82DF6768: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF676C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DF6770: 4BFFF2A9  bl 0x82df5a18
	ctx.lr = 0x82DF6774;
	sub_82DF5A18(ctx, base);
	// 82DF6774: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF6778: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF677C: 9BAB002C  stb r29, 0x2c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), ctx.r[29].u8 ) };
	// 82DF6780: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF6784: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF6788: 9B6B002C  stb r27, 0x2c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), ctx.r[27].u8 ) };
	// 82DF678C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF6790: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF6794: 480210FD  bl 0x82e17890
	ctx.lr = 0x82DF6798;
	sub_82E17890(ctx, base);
	// 82DF6798: 48000074  b 0x82df680c
	pc = 0x82DF680C; continue 'dispatch;
	// 82DF679C: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF67A0: 892A002C  lbz r9, 0x2c(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DF67A4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DF67A8: 409A0028  bne cr6, 0x82df67d0
	if !ctx.cr[6].eq {
	pc = 0x82DF67D0; continue 'dispatch;
	}
	// 82DF67AC: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF67B0: 9BA9002C  stb r29, 0x2c(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(44 as u32), ctx.r[29].u8 ) };
	// 82DF67B4: 9BAA002C  stb r29, 0x2c(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(44 as u32), ctx.r[29].u8 ) };
	// 82DF67B8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF67BC: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF67C0: 9B6A002C  stb r27, 0x2c(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(44 as u32), ctx.r[27].u8 ) };
	// 82DF67C4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF67C8: 83EB0004  lwz r31, 4(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF67CC: 48000040  b 0x82df680c
	pc = 0x82DF680C; continue 'dispatch;
	// 82DF67D0: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF67D4: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF67D8: 409A0010  bne cr6, 0x82df67e8
	if !ctx.cr[6].eq {
	pc = 0x82DF67E8; continue 'dispatch;
	}
	// 82DF67DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF67E0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DF67E4: 480210AD  bl 0x82e17890
	ctx.lr = 0x82DF67E8;
	sub_82E17890(ctx, base);
	// 82DF67E8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF67EC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF67F0: 9BAB002C  stb r29, 0x2c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), ctx.r[29].u8 ) };
	// 82DF67F4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF67F8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF67FC: 9B6B002C  stb r27, 0x2c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), ctx.r[27].u8 ) };
	// 82DF6800: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF6804: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF6808: 4BFFF211  bl 0x82df5a18
	ctx.lr = 0x82DF680C;
	sub_82DF5A18(ctx, base);
	// 82DF680C: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF6810: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 82DF6814: 894A002C  lbz r10, 0x2c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DF6818: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF681C: 419AFF1C  beq cr6, 0x82df6738
	if ctx.cr[6].eq {
	pc = 0x82DF6738; continue 'dispatch;
	}
	// 82DF6820: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF6824: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82DF6828: 939A0000  stw r28, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82DF682C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF6830: 9BAB002C  stb r29, 0x2c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), ctx.r[29].u8 ) };
	// 82DF6834: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82DF6838: 483B1978  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF6840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF6840 size=1016
    let mut pc: u32 = 0x82DF6840;
    'dispatch: loop {
        match pc {
            0x82DF6840 => {
    //   block [0x82DF6840..0x82DF6C38)
	// 82DF6840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF6844: 483B1915  bl 0x831a8158
	ctx.lr = 0x82DF6848;
	sub_831A8130(ctx, base);
	// 82DF6848: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF684C: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82DF6850: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82DF6854: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82DF6858: 93E10104  stw r31, 0x104(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(260 as u32), ctx.r[31].u32 ) };
	// 82DF685C: 897F0015  lbz r11, 0x15(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(21 as u32) ) } as u64;
	// 82DF6860: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF6864: 419A0048  beq cr6, 0x82df68ac
	if ctx.cr[6].eq {
	pc = 0x82DF68AC; continue 'dispatch;
	}
	// 82DF6868: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82DF686C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF6870: 388B9620  addi r4, r11, -0x69e0
	ctx.r[4].s64 = ctx.r[11].s64 + -27104;
	// 82DF6874: 4B4CF055  bl 0x822c58c8
	ctx.lr = 0x82DF6878;
	sub_822C58C8(ctx, base);
	// 82DF6878: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DF687C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82DF6880: 4B4D3631  bl 0x822c9eb0
	ctx.lr = 0x82DF6884;
	sub_822C9EB0(ctx, base);
	// 82DF6884: 4B4CDA2D  bl 0x822c42b0
	ctx.lr = 0x82DF6888;
	sub_822C42B0(ctx, base);
	// 82DF6888: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82DF688C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82DF6890: 396B9600  addi r11, r11, -0x6a00
	ctx.r[11].s64 = ctx.r[11].s64 + -27136;
	// 82DF6894: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82DF6898: 4B4CEBD9  bl 0x822c5470
	ctx.lr = 0x82DF689C;
	sub_822C5470(ctx, base);
	// 82DF689C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF68A0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DF68A4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF68A8: 4B4CE439  bl 0x822c4ce0
	ctx.lr = 0x82DF68AC;
	sub_822C4CE0(ctx, base);
	// 82DF68AC: 38610104  addi r3, r1, 0x104
	ctx.r[3].s64 = ctx.r[1].s64 + 260;
	// 82DF68B0: 7FFAFB78  mr r26, r31
	ctx.r[26].u64 = ctx.r[31].u64;
	// 82DF68B4: 480053A5  bl 0x82dfbc58
	ctx.lr = 0x82DF68B8;
	sub_82DFBC58(ctx, base);
	// 82DF68B8: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF68BC: 894B0015  lbz r10, 0x15(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 82DF68C0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF68C4: 83210104  lwz r25, 0x104(r1)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(260 as u32) ) } as u64;
	// 82DF68C8: 419A000C  beq cr6, 0x82df68d4
	if ctx.cr[6].eq {
	pc = 0x82DF68D4; continue 'dispatch;
	}
	// 82DF68CC: 839A0008  lwz r28, 8(r26)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF68D0: 48000028  b 0x82df68f8
	pc = 0x82DF68F8; continue 'dispatch;
	// 82DF68D4: 815A0008  lwz r10, 8(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF68D8: 894A0015  lbz r10, 0x15(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(21 as u32) ) } as u64;
	// 82DF68DC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF68E0: 419A000C  beq cr6, 0x82df68ec
	if ctx.cr[6].eq {
	pc = 0x82DF68EC; continue 'dispatch;
	}
	// 82DF68E4: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 82DF68E8: 48000010  b 0x82df68f8
	pc = 0x82DF68F8; continue 'dispatch;
	// 82DF68EC: 83990008  lwz r28, 8(r25)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF68F0: 7F19D040  cmplw cr6, r25, r26
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82DF68F4: 409A00DC  bne cr6, 0x82df69d0
	if !ctx.cr[6].eq {
	pc = 0x82DF69D0; continue 'dispatch;
	}
	// 82DF68F8: 897C0015  lbz r11, 0x15(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(21 as u32) ) } as u64;
	// 82DF68FC: 83FA0004  lwz r31, 4(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF6900: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF6904: 409A0008  bne cr6, 0x82df690c
	if !ctx.cr[6].eq {
	pc = 0x82DF690C; continue 'dispatch;
	}
	// 82DF6908: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82DF690C: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF6910: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF6914: 7F0AD040  cmplw cr6, r10, r26
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82DF6918: 409A000C  bne cr6, 0x82df6924
	if !ctx.cr[6].eq {
	pc = 0x82DF6924; continue 'dispatch;
	}
	// 82DF691C: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82DF6920: 4800001C  b 0x82df693c
	pc = 0x82DF693C; continue 'dispatch;
	// 82DF6924: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF6928: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82DF692C: 409A000C  bne cr6, 0x82df6938
	if !ctx.cr[6].eq {
	pc = 0x82DF6938; continue 'dispatch;
	}
	// 82DF6930: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82DF6934: 48000008  b 0x82df693c
	pc = 0x82DF693C; continue 'dispatch;
	// 82DF6938: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82DF693C: 813B0004  lwz r9, 4(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF6940: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF6944: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82DF6948: 409A003C  bne cr6, 0x82df6984
	if !ctx.cr[6].eq {
	pc = 0x82DF6984; continue 'dispatch;
	}
	// 82DF694C: 897C0015  lbz r11, 0x15(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(21 as u32) ) } as u64;
	// 82DF6950: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF6954: 419A000C  beq cr6, 0x82df6960
	if ctx.cr[6].eq {
	pc = 0x82DF6960; continue 'dispatch;
	}
	// 82DF6958: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 82DF695C: 48000024  b 0x82df6980
	pc = 0x82DF6980; continue 'dispatch;
	// 82DF6960: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF6964: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 82DF6968: 4800000C  b 0x82df6974
	pc = 0x82DF6974; continue 'dispatch;
	// 82DF696C: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82DF6970: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF6974: 890B0015  lbz r8, 0x15(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 82DF6978: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82DF697C: 419AFFF0  beq cr6, 0x82df696c
	if ctx.cr[6].eq {
	pc = 0x82DF696C; continue 'dispatch;
	}
	// 82DF6980: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DF6984: 813B0004  lwz r9, 4(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF6988: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF698C: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82DF6990: 409A00D4  bne cr6, 0x82df6a64
	if !ctx.cr[6].eq {
	pc = 0x82DF6A64; continue 'dispatch;
	}
	// 82DF6994: 897C0015  lbz r11, 0x15(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(21 as u32) ) } as u64;
	// 82DF6998: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF699C: 419A000C  beq cr6, 0x82df69a8
	if ctx.cr[6].eq {
	pc = 0x82DF69A8; continue 'dispatch;
	}
	// 82DF69A0: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 82DF69A4: 48000024  b 0x82df69c8
	pc = 0x82DF69C8; continue 'dispatch;
	// 82DF69A8: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF69AC: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 82DF69B0: 4800000C  b 0x82df69bc
	pc = 0x82DF69BC; continue 'dispatch;
	// 82DF69B4: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82DF69B8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF69BC: 890B0015  lbz r8, 0x15(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 82DF69C0: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82DF69C4: 419AFFF0  beq cr6, 0x82df69b4
	if ctx.cr[6].eq {
	pc = 0x82DF69B4; continue 'dispatch;
	}
	// 82DF69C8: 91490008  stw r10, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82DF69CC: 48000098  b 0x82df6a64
	pc = 0x82DF6A64; continue 'dispatch;
	// 82DF69D0: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 82DF69D4: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF69D8: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF69DC: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF69E0: 7F195840  cmplw cr6, r25, r11
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF69E4: 409A000C  bne cr6, 0x82df69f0
	if !ctx.cr[6].eq {
	pc = 0x82DF69F0; continue 'dispatch;
	}
	// 82DF69E8: 7F3FCB78  mr r31, r25
	ctx.r[31].u64 = ctx.r[25].u64;
	// 82DF69EC: 4800002C  b 0x82df6a18
	pc = 0x82DF6A18; continue 'dispatch;
	// 82DF69F0: 897C0015  lbz r11, 0x15(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(21 as u32) ) } as u64;
	// 82DF69F4: 83F90004  lwz r31, 4(r25)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF69F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF69FC: 409A0008  bne cr6, 0x82df6a04
	if !ctx.cr[6].eq {
	pc = 0x82DF6A04; continue 'dispatch;
	}
	// 82DF6A00: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82DF6A04: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82DF6A08: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF6A0C: 91790008  stw r11, 8(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DF6A10: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF6A14: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 82DF6A18: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF6A1C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF6A20: 7F0AD040  cmplw cr6, r10, r26
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82DF6A24: 409A000C  bne cr6, 0x82df6a30
	if !ctx.cr[6].eq {
	pc = 0x82DF6A30; continue 'dispatch;
	}
	// 82DF6A28: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 82DF6A2C: 48000020  b 0x82df6a4c
	pc = 0x82DF6A4C; continue 'dispatch;
	// 82DF6A30: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF6A34: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF6A38: 7F0AD040  cmplw cr6, r10, r26
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82DF6A3C: 409A000C  bne cr6, 0x82df6a48
	if !ctx.cr[6].eq {
	pc = 0x82DF6A48; continue 'dispatch;
	}
	// 82DF6A40: 932B0000  stw r25, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 82DF6A44: 48000008  b 0x82df6a4c
	pc = 0x82DF6A4C; continue 'dispatch;
	// 82DF6A48: 932B0008  stw r25, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[25].u32 ) };
	// 82DF6A4C: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF6A50: 91790004  stw r11, 4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DF6A54: 897A0014  lbz r11, 0x14(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DF6A58: 89590014  lbz r10, 0x14(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[25].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DF6A5C: 99790014  stb r11, 0x14(r25)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[25].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 82DF6A60: 995A0014  stb r10, 0x14(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(20 as u32), ctx.r[10].u8 ) };
	// 82DF6A64: 897A0014  lbz r11, 0x14(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DF6A68: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82DF6A6C: 409A0198  bne cr6, 0x82df6c04
	if !ctx.cr[6].eq {
	pc = 0x82DF6C04; continue 'dispatch;
	}
	// 82DF6A70: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF6A74: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82DF6A78: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF6A7C: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF6A80: 419A0180  beq cr6, 0x82df6c00
	if ctx.cr[6].eq {
	pc = 0x82DF6C00; continue 'dispatch;
	}
	// 82DF6A84: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82DF6A88: 897C0014  lbz r11, 0x14(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DF6A8C: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82DF6A90: 409A0170  bne cr6, 0x82df6c00
	if !ctx.cr[6].eq {
	pc = 0x82DF6C00; continue 'dispatch;
	}
	// 82DF6A94: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF6A98: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF6A9C: 409A00A8  bne cr6, 0x82df6b44
	if !ctx.cr[6].eq {
	pc = 0x82DF6B44; continue 'dispatch;
	}
	// 82DF6AA0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF6AA4: 894B0014  lbz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DF6AA8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF6AAC: 409A001C  bne cr6, 0x82df6ac8
	if !ctx.cr[6].eq {
	pc = 0x82DF6AC8; continue 'dispatch;
	}
	// 82DF6AB0: 9BCB0014  stb r30, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 82DF6AB4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF6AB8: 9BBF0014  stb r29, 0x14(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[29].u8 ) };
	// 82DF6ABC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82DF6AC0: 4BFFEEF1  bl 0x82df59b0
	ctx.lr = 0x82DF6AC4;
	sub_82DF59B0(ctx, base);
	// 82DF6AC4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF6AC8: 894B0015  lbz r10, 0x15(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 82DF6ACC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF6AD0: 409A00C8  bne cr6, 0x82df6b98
	if !ctx.cr[6].eq {
	pc = 0x82DF6B98; continue 'dispatch;
	}
	// 82DF6AD4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF6AD8: 894A0014  lbz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DF6ADC: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82DF6AE0: 409A0014  bne cr6, 0x82df6af4
	if !ctx.cr[6].eq {
	pc = 0x82DF6AF4; continue 'dispatch;
	}
	// 82DF6AE4: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF6AE8: 894A0014  lbz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DF6AEC: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82DF6AF0: 419A00A4  beq cr6, 0x82df6b94
	if ctx.cr[6].eq {
	pc = 0x82DF6B94; continue 'dispatch;
	}
	// 82DF6AF4: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF6AF8: 894A0014  lbz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DF6AFC: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82DF6B00: 409A0020  bne cr6, 0x82df6b20
	if !ctx.cr[6].eq {
	pc = 0x82DF6B20; continue 'dispatch;
	}
	// 82DF6B04: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF6B08: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DF6B0C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82DF6B10: 9BCA0014  stb r30, 0x14(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 82DF6B14: 9BAB0014  stb r29, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[29].u8 ) };
	// 82DF6B18: 48069F69  bl 0x82e60a80
	ctx.lr = 0x82DF6B1C;
	sub_82E60A80(ctx, base);
	// 82DF6B1C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF6B20: 895F0014  lbz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DF6B24: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF6B28: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82DF6B2C: 994B0014  stb r10, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u8 ) };
	// 82DF6B30: 9BDF0014  stb r30, 0x14(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 82DF6B34: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF6B38: 9BCB0014  stb r30, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 82DF6B3C: 4BFFEE75  bl 0x82df59b0
	ctx.lr = 0x82DF6B40;
	sub_82DF59B0(ctx, base);
	// 82DF6B40: 480000C0  b 0x82df6c00
	pc = 0x82DF6C00; continue 'dispatch;
	// 82DF6B44: 894B0014  lbz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DF6B48: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF6B4C: 409A001C  bne cr6, 0x82df6b68
	if !ctx.cr[6].eq {
	pc = 0x82DF6B68; continue 'dispatch;
	}
	// 82DF6B50: 9BCB0014  stb r30, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 82DF6B54: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF6B58: 9BBF0014  stb r29, 0x14(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[29].u8 ) };
	// 82DF6B5C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82DF6B60: 48069F21  bl 0x82e60a80
	ctx.lr = 0x82DF6B64;
	sub_82E60A80(ctx, base);
	// 82DF6B64: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF6B68: 894B0015  lbz r10, 0x15(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 82DF6B6C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF6B70: 409A0028  bne cr6, 0x82df6b98
	if !ctx.cr[6].eq {
	pc = 0x82DF6B98; continue 'dispatch;
	}
	// 82DF6B74: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF6B78: 894A0014  lbz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DF6B7C: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82DF6B80: 409A0034  bne cr6, 0x82df6bb4
	if !ctx.cr[6].eq {
	pc = 0x82DF6BB4; continue 'dispatch;
	}
	// 82DF6B84: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF6B88: 894A0014  lbz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DF6B8C: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82DF6B90: 409A0024  bne cr6, 0x82df6bb4
	if !ctx.cr[6].eq {
	pc = 0x82DF6BB4; continue 'dispatch;
	}
	// 82DF6B94: 9BAB0014  stb r29, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[29].u8 ) };
	// 82DF6B98: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF6B9C: 7FFCFB78  mr r28, r31
	ctx.r[28].u64 = ctx.r[31].u64;
	// 82DF6BA0: 83FF0004  lwz r31, 4(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF6BA4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF6BA8: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF6BAC: 409AFEDC  bne cr6, 0x82df6a88
	if !ctx.cr[6].eq {
	pc = 0x82DF6A88; continue 'dispatch;
	}
	// 82DF6BB0: 48000050  b 0x82df6c00
	pc = 0x82DF6C00; continue 'dispatch;
	// 82DF6BB4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF6BB8: 894A0014  lbz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DF6BBC: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82DF6BC0: 409A0020  bne cr6, 0x82df6be0
	if !ctx.cr[6].eq {
	pc = 0x82DF6BE0; continue 'dispatch;
	}
	// 82DF6BC4: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF6BC8: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DF6BCC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82DF6BD0: 9BCA0014  stb r30, 0x14(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 82DF6BD4: 9BAB0014  stb r29, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[29].u8 ) };
	// 82DF6BD8: 4BFFEDD9  bl 0x82df59b0
	ctx.lr = 0x82DF6BDC;
	sub_82DF59B0(ctx, base);
	// 82DF6BDC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF6BE0: 895F0014  lbz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DF6BE4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF6BE8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82DF6BEC: 994B0014  stb r10, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u8 ) };
	// 82DF6BF0: 9BDF0014  stb r30, 0x14(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 82DF6BF4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF6BF8: 9BCB0014  stb r30, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 82DF6BFC: 48069E85  bl 0x82e60a80
	ctx.lr = 0x82DF6C00;
	sub_82E60A80(ctx, base);
	// 82DF6C00: 9BDC0014  stb r30, 0x14(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 82DF6C04: 387A000C  addi r3, r26, 0xc
	ctx.r[3].s64 = ctx.r[26].s64 + 12;
	// 82DF6C08: 4BFFC821  bl 0x82df3428
	ctx.lr = 0x82DF6C0C;
	sub_82DF3428(ctx, base);
	// 82DF6C0C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82DF6C10: 4B4C9659  bl 0x822c0268
	ctx.lr = 0x82DF6C14;
	sub_822C0268(ctx, base);
	// 82DF6C14: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF6C18: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF6C1C: 419A000C  beq cr6, 0x82df6c28
	if ctx.cr[6].eq {
	pc = 0x82DF6C28; continue 'dispatch;
	}
	// 82DF6C20: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DF6C24: 917B0008  stw r11, 8(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DF6C28: 93380000  stw r25, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 82DF6C2C: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82DF6C30: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82DF6C34: 483B1574  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF6C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF6C38 size=1016
    let mut pc: u32 = 0x82DF6C38;
    'dispatch: loop {
        match pc {
            0x82DF6C38 => {
    //   block [0x82DF6C38..0x82DF7030)
	// 82DF6C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF6C3C: 483B151D  bl 0x831a8158
	ctx.lr = 0x82DF6C40;
	sub_831A8130(ctx, base);
	// 82DF6C40: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF6C44: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82DF6C48: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82DF6C4C: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82DF6C50: 93E10104  stw r31, 0x104(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(260 as u32), ctx.r[31].u32 ) };
	// 82DF6C54: 897F0015  lbz r11, 0x15(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(21 as u32) ) } as u64;
	// 82DF6C58: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF6C5C: 419A0048  beq cr6, 0x82df6ca4
	if ctx.cr[6].eq {
	pc = 0x82DF6CA4; continue 'dispatch;
	}
	// 82DF6C60: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82DF6C64: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF6C68: 388B9620  addi r4, r11, -0x69e0
	ctx.r[4].s64 = ctx.r[11].s64 + -27104;
	// 82DF6C6C: 4B4CEC5D  bl 0x822c58c8
	ctx.lr = 0x82DF6C70;
	sub_822C58C8(ctx, base);
	// 82DF6C70: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DF6C74: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82DF6C78: 4B4D3239  bl 0x822c9eb0
	ctx.lr = 0x82DF6C7C;
	sub_822C9EB0(ctx, base);
	// 82DF6C7C: 4B4CD635  bl 0x822c42b0
	ctx.lr = 0x82DF6C80;
	sub_822C42B0(ctx, base);
	// 82DF6C80: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82DF6C84: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82DF6C88: 396B9600  addi r11, r11, -0x6a00
	ctx.r[11].s64 = ctx.r[11].s64 + -27136;
	// 82DF6C8C: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82DF6C90: 4B4CE7E1  bl 0x822c5470
	ctx.lr = 0x82DF6C94;
	sub_822C5470(ctx, base);
	// 82DF6C94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF6C98: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DF6C9C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF6CA0: 4B4CE041  bl 0x822c4ce0
	ctx.lr = 0x82DF6CA4;
	sub_822C4CE0(ctx, base);
	// 82DF6CA4: 38610104  addi r3, r1, 0x104
	ctx.r[3].s64 = ctx.r[1].s64 + 260;
	// 82DF6CA8: 7FFAFB78  mr r26, r31
	ctx.r[26].u64 = ctx.r[31].u64;
	// 82DF6CAC: 48004FAD  bl 0x82dfbc58
	ctx.lr = 0x82DF6CB0;
	sub_82DFBC58(ctx, base);
	// 82DF6CB0: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF6CB4: 894B0015  lbz r10, 0x15(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 82DF6CB8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF6CBC: 83210104  lwz r25, 0x104(r1)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(260 as u32) ) } as u64;
	// 82DF6CC0: 419A000C  beq cr6, 0x82df6ccc
	if ctx.cr[6].eq {
	pc = 0x82DF6CCC; continue 'dispatch;
	}
	// 82DF6CC4: 839A0008  lwz r28, 8(r26)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF6CC8: 48000028  b 0x82df6cf0
	pc = 0x82DF6CF0; continue 'dispatch;
	// 82DF6CCC: 815A0008  lwz r10, 8(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF6CD0: 894A0015  lbz r10, 0x15(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(21 as u32) ) } as u64;
	// 82DF6CD4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF6CD8: 419A000C  beq cr6, 0x82df6ce4
	if ctx.cr[6].eq {
	pc = 0x82DF6CE4; continue 'dispatch;
	}
	// 82DF6CDC: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 82DF6CE0: 48000010  b 0x82df6cf0
	pc = 0x82DF6CF0; continue 'dispatch;
	// 82DF6CE4: 83990008  lwz r28, 8(r25)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF6CE8: 7F19D040  cmplw cr6, r25, r26
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82DF6CEC: 409A00DC  bne cr6, 0x82df6dc8
	if !ctx.cr[6].eq {
	pc = 0x82DF6DC8; continue 'dispatch;
	}
	// 82DF6CF0: 897C0015  lbz r11, 0x15(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(21 as u32) ) } as u64;
	// 82DF6CF4: 83FA0004  lwz r31, 4(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF6CF8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF6CFC: 409A0008  bne cr6, 0x82df6d04
	if !ctx.cr[6].eq {
	pc = 0x82DF6D04; continue 'dispatch;
	}
	// 82DF6D00: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82DF6D04: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF6D08: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF6D0C: 7F0AD040  cmplw cr6, r10, r26
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82DF6D10: 409A000C  bne cr6, 0x82df6d1c
	if !ctx.cr[6].eq {
	pc = 0x82DF6D1C; continue 'dispatch;
	}
	// 82DF6D14: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82DF6D18: 4800001C  b 0x82df6d34
	pc = 0x82DF6D34; continue 'dispatch;
	// 82DF6D1C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF6D20: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82DF6D24: 409A000C  bne cr6, 0x82df6d30
	if !ctx.cr[6].eq {
	pc = 0x82DF6D30; continue 'dispatch;
	}
	// 82DF6D28: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82DF6D2C: 48000008  b 0x82df6d34
	pc = 0x82DF6D34; continue 'dispatch;
	// 82DF6D30: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82DF6D34: 813B0004  lwz r9, 4(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF6D38: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF6D3C: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82DF6D40: 409A003C  bne cr6, 0x82df6d7c
	if !ctx.cr[6].eq {
	pc = 0x82DF6D7C; continue 'dispatch;
	}
	// 82DF6D44: 897C0015  lbz r11, 0x15(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(21 as u32) ) } as u64;
	// 82DF6D48: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF6D4C: 419A000C  beq cr6, 0x82df6d58
	if ctx.cr[6].eq {
	pc = 0x82DF6D58; continue 'dispatch;
	}
	// 82DF6D50: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 82DF6D54: 48000024  b 0x82df6d78
	pc = 0x82DF6D78; continue 'dispatch;
	// 82DF6D58: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF6D5C: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 82DF6D60: 4800000C  b 0x82df6d6c
	pc = 0x82DF6D6C; continue 'dispatch;
	// 82DF6D64: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82DF6D68: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF6D6C: 890B0015  lbz r8, 0x15(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 82DF6D70: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82DF6D74: 419AFFF0  beq cr6, 0x82df6d64
	if ctx.cr[6].eq {
	pc = 0x82DF6D64; continue 'dispatch;
	}
	// 82DF6D78: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DF6D7C: 813B0004  lwz r9, 4(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF6D80: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF6D84: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82DF6D88: 409A00D4  bne cr6, 0x82df6e5c
	if !ctx.cr[6].eq {
	pc = 0x82DF6E5C; continue 'dispatch;
	}
	// 82DF6D8C: 897C0015  lbz r11, 0x15(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(21 as u32) ) } as u64;
	// 82DF6D90: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF6D94: 419A000C  beq cr6, 0x82df6da0
	if ctx.cr[6].eq {
	pc = 0x82DF6DA0; continue 'dispatch;
	}
	// 82DF6D98: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 82DF6D9C: 48000024  b 0x82df6dc0
	pc = 0x82DF6DC0; continue 'dispatch;
	// 82DF6DA0: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF6DA4: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 82DF6DA8: 4800000C  b 0x82df6db4
	pc = 0x82DF6DB4; continue 'dispatch;
	// 82DF6DAC: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82DF6DB0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF6DB4: 890B0015  lbz r8, 0x15(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 82DF6DB8: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82DF6DBC: 419AFFF0  beq cr6, 0x82df6dac
	if ctx.cr[6].eq {
	pc = 0x82DF6DAC; continue 'dispatch;
	}
	// 82DF6DC0: 91490008  stw r10, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82DF6DC4: 48000098  b 0x82df6e5c
	pc = 0x82DF6E5C; continue 'dispatch;
	// 82DF6DC8: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 82DF6DCC: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF6DD0: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF6DD4: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF6DD8: 7F195840  cmplw cr6, r25, r11
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF6DDC: 409A000C  bne cr6, 0x82df6de8
	if !ctx.cr[6].eq {
	pc = 0x82DF6DE8; continue 'dispatch;
	}
	// 82DF6DE0: 7F3FCB78  mr r31, r25
	ctx.r[31].u64 = ctx.r[25].u64;
	// 82DF6DE4: 4800002C  b 0x82df6e10
	pc = 0x82DF6E10; continue 'dispatch;
	// 82DF6DE8: 897C0015  lbz r11, 0x15(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(21 as u32) ) } as u64;
	// 82DF6DEC: 83F90004  lwz r31, 4(r25)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF6DF0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF6DF4: 409A0008  bne cr6, 0x82df6dfc
	if !ctx.cr[6].eq {
	pc = 0x82DF6DFC; continue 'dispatch;
	}
	// 82DF6DF8: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82DF6DFC: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82DF6E00: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF6E04: 91790008  stw r11, 8(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DF6E08: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF6E0C: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 82DF6E10: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF6E14: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF6E18: 7F0AD040  cmplw cr6, r10, r26
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82DF6E1C: 409A000C  bne cr6, 0x82df6e28
	if !ctx.cr[6].eq {
	pc = 0x82DF6E28; continue 'dispatch;
	}
	// 82DF6E20: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 82DF6E24: 48000020  b 0x82df6e44
	pc = 0x82DF6E44; continue 'dispatch;
	// 82DF6E28: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF6E2C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF6E30: 7F0AD040  cmplw cr6, r10, r26
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82DF6E34: 409A000C  bne cr6, 0x82df6e40
	if !ctx.cr[6].eq {
	pc = 0x82DF6E40; continue 'dispatch;
	}
	// 82DF6E38: 932B0000  stw r25, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 82DF6E3C: 48000008  b 0x82df6e44
	pc = 0x82DF6E44; continue 'dispatch;
	// 82DF6E40: 932B0008  stw r25, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[25].u32 ) };
	// 82DF6E44: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF6E48: 91790004  stw r11, 4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DF6E4C: 897A0014  lbz r11, 0x14(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DF6E50: 89590014  lbz r10, 0x14(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[25].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DF6E54: 99790014  stb r11, 0x14(r25)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[25].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 82DF6E58: 995A0014  stb r10, 0x14(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(20 as u32), ctx.r[10].u8 ) };
	// 82DF6E5C: 897A0014  lbz r11, 0x14(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DF6E60: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82DF6E64: 409A0198  bne cr6, 0x82df6ffc
	if !ctx.cr[6].eq {
	pc = 0x82DF6FFC; continue 'dispatch;
	}
	// 82DF6E68: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF6E6C: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82DF6E70: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF6E74: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF6E78: 419A0180  beq cr6, 0x82df6ff8
	if ctx.cr[6].eq {
	pc = 0x82DF6FF8; continue 'dispatch;
	}
	// 82DF6E7C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82DF6E80: 897C0014  lbz r11, 0x14(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DF6E84: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82DF6E88: 409A0170  bne cr6, 0x82df6ff8
	if !ctx.cr[6].eq {
	pc = 0x82DF6FF8; continue 'dispatch;
	}
	// 82DF6E8C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF6E90: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF6E94: 409A00A8  bne cr6, 0x82df6f3c
	if !ctx.cr[6].eq {
	pc = 0x82DF6F3C; continue 'dispatch;
	}
	// 82DF6E98: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF6E9C: 894B0014  lbz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DF6EA0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF6EA4: 409A001C  bne cr6, 0x82df6ec0
	if !ctx.cr[6].eq {
	pc = 0x82DF6EC0; continue 'dispatch;
	}
	// 82DF6EA8: 9BCB0014  stb r30, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 82DF6EAC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF6EB0: 9BBF0014  stb r29, 0x14(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[29].u8 ) };
	// 82DF6EB4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82DF6EB8: 4BFFEAF9  bl 0x82df59b0
	ctx.lr = 0x82DF6EBC;
	sub_82DF59B0(ctx, base);
	// 82DF6EBC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF6EC0: 894B0015  lbz r10, 0x15(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 82DF6EC4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF6EC8: 409A00C8  bne cr6, 0x82df6f90
	if !ctx.cr[6].eq {
	pc = 0x82DF6F90; continue 'dispatch;
	}
	// 82DF6ECC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF6ED0: 894A0014  lbz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DF6ED4: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82DF6ED8: 409A0014  bne cr6, 0x82df6eec
	if !ctx.cr[6].eq {
	pc = 0x82DF6EEC; continue 'dispatch;
	}
	// 82DF6EDC: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF6EE0: 894A0014  lbz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DF6EE4: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82DF6EE8: 419A00A4  beq cr6, 0x82df6f8c
	if ctx.cr[6].eq {
	pc = 0x82DF6F8C; continue 'dispatch;
	}
	// 82DF6EEC: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF6EF0: 894A0014  lbz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DF6EF4: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82DF6EF8: 409A0020  bne cr6, 0x82df6f18
	if !ctx.cr[6].eq {
	pc = 0x82DF6F18; continue 'dispatch;
	}
	// 82DF6EFC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF6F00: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DF6F04: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82DF6F08: 9BCA0014  stb r30, 0x14(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 82DF6F0C: 9BAB0014  stb r29, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[29].u8 ) };
	// 82DF6F10: 48069B71  bl 0x82e60a80
	ctx.lr = 0x82DF6F14;
	sub_82E60A80(ctx, base);
	// 82DF6F14: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF6F18: 895F0014  lbz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DF6F1C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF6F20: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82DF6F24: 994B0014  stb r10, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u8 ) };
	// 82DF6F28: 9BDF0014  stb r30, 0x14(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 82DF6F2C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF6F30: 9BCB0014  stb r30, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 82DF6F34: 4BFFEA7D  bl 0x82df59b0
	ctx.lr = 0x82DF6F38;
	sub_82DF59B0(ctx, base);
	// 82DF6F38: 480000C0  b 0x82df6ff8
	pc = 0x82DF6FF8; continue 'dispatch;
	// 82DF6F3C: 894B0014  lbz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DF6F40: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF6F44: 409A001C  bne cr6, 0x82df6f60
	if !ctx.cr[6].eq {
	pc = 0x82DF6F60; continue 'dispatch;
	}
	// 82DF6F48: 9BCB0014  stb r30, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 82DF6F4C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF6F50: 9BBF0014  stb r29, 0x14(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[29].u8 ) };
	// 82DF6F54: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82DF6F58: 48069B29  bl 0x82e60a80
	ctx.lr = 0x82DF6F5C;
	sub_82E60A80(ctx, base);
	// 82DF6F5C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF6F60: 894B0015  lbz r10, 0x15(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 82DF6F64: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF6F68: 409A0028  bne cr6, 0x82df6f90
	if !ctx.cr[6].eq {
	pc = 0x82DF6F90; continue 'dispatch;
	}
	// 82DF6F6C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF6F70: 894A0014  lbz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DF6F74: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82DF6F78: 409A0034  bne cr6, 0x82df6fac
	if !ctx.cr[6].eq {
	pc = 0x82DF6FAC; continue 'dispatch;
	}
	// 82DF6F7C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF6F80: 894A0014  lbz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DF6F84: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82DF6F88: 409A0024  bne cr6, 0x82df6fac
	if !ctx.cr[6].eq {
	pc = 0x82DF6FAC; continue 'dispatch;
	}
	// 82DF6F8C: 9BAB0014  stb r29, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[29].u8 ) };
	// 82DF6F90: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF6F94: 7FFCFB78  mr r28, r31
	ctx.r[28].u64 = ctx.r[31].u64;
	// 82DF6F98: 83FF0004  lwz r31, 4(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF6F9C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF6FA0: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF6FA4: 409AFEDC  bne cr6, 0x82df6e80
	if !ctx.cr[6].eq {
	pc = 0x82DF6E80; continue 'dispatch;
	}
	// 82DF6FA8: 48000050  b 0x82df6ff8
	pc = 0x82DF6FF8; continue 'dispatch;
	// 82DF6FAC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF6FB0: 894A0014  lbz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DF6FB4: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82DF6FB8: 409A0020  bne cr6, 0x82df6fd8
	if !ctx.cr[6].eq {
	pc = 0x82DF6FD8; continue 'dispatch;
	}
	// 82DF6FBC: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF6FC0: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DF6FC4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82DF6FC8: 9BCA0014  stb r30, 0x14(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 82DF6FCC: 9BAB0014  stb r29, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[29].u8 ) };
	// 82DF6FD0: 4BFFE9E1  bl 0x82df59b0
	ctx.lr = 0x82DF6FD4;
	sub_82DF59B0(ctx, base);
	// 82DF6FD4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF6FD8: 895F0014  lbz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DF6FDC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF6FE0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82DF6FE4: 994B0014  stb r10, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u8 ) };
	// 82DF6FE8: 9BDF0014  stb r30, 0x14(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 82DF6FEC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF6FF0: 9BCB0014  stb r30, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 82DF6FF4: 48069A8D  bl 0x82e60a80
	ctx.lr = 0x82DF6FF8;
	sub_82E60A80(ctx, base);
	// 82DF6FF8: 9BDC0014  stb r30, 0x14(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 82DF6FFC: 387A000C  addi r3, r26, 0xc
	ctx.r[3].s64 = ctx.r[26].s64 + 12;
	// 82DF7000: 4BFFC429  bl 0x82df3428
	ctx.lr = 0x82DF7004;
	sub_82DF3428(ctx, base);
	// 82DF7004: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82DF7008: 4BFFAEA9  bl 0x82df1eb0
	ctx.lr = 0x82DF700C;
	sub_82DF1EB0(ctx, base);
	// 82DF700C: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF7010: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF7014: 419A000C  beq cr6, 0x82df7020
	if ctx.cr[6].eq {
	pc = 0x82DF7020; continue 'dispatch;
	}
	// 82DF7018: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DF701C: 917B0008  stw r11, 8(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DF7020: 93380000  stw r25, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 82DF7024: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82DF7028: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82DF702C: 483B117C  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF7030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF7030 size=1024
    let mut pc: u32 = 0x82DF7030;
    'dispatch: loop {
        match pc {
            0x82DF7030 => {
    //   block [0x82DF7030..0x82DF7430)
	// 82DF7030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF7034: 483B1125  bl 0x831a8158
	ctx.lr = 0x82DF7038;
	sub_831A8130(ctx, base);
	// 82DF7038: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF703C: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82DF7040: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82DF7044: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82DF7048: 93E10104  stw r31, 0x104(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(260 as u32), ctx.r[31].u32 ) };
	// 82DF704C: 897F002D  lbz r11, 0x2d(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(45 as u32) ) } as u64;
	// 82DF7050: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF7054: 419A0048  beq cr6, 0x82df709c
	if ctx.cr[6].eq {
	pc = 0x82DF709C; continue 'dispatch;
	}
	// 82DF7058: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82DF705C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF7060: 388B9620  addi r4, r11, -0x69e0
	ctx.r[4].s64 = ctx.r[11].s64 + -27104;
	// 82DF7064: 4B4CE865  bl 0x822c58c8
	ctx.lr = 0x82DF7068;
	sub_822C58C8(ctx, base);
	// 82DF7068: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DF706C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82DF7070: 4B4D2E41  bl 0x822c9eb0
	ctx.lr = 0x82DF7074;
	sub_822C9EB0(ctx, base);
	// 82DF7074: 4B4CD23D  bl 0x822c42b0
	ctx.lr = 0x82DF7078;
	sub_822C42B0(ctx, base);
	// 82DF7078: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82DF707C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82DF7080: 396B9600  addi r11, r11, -0x6a00
	ctx.r[11].s64 = ctx.r[11].s64 + -27136;
	// 82DF7084: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82DF7088: 4B4CE3E9  bl 0x822c5470
	ctx.lr = 0x82DF708C;
	sub_822C5470(ctx, base);
	// 82DF708C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF7090: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DF7094: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF7098: 4B4CDC49  bl 0x822c4ce0
	ctx.lr = 0x82DF709C;
	sub_822C4CE0(ctx, base);
	// 82DF709C: 38610104  addi r3, r1, 0x104
	ctx.r[3].s64 = ctx.r[1].s64 + 260;
	// 82DF70A0: 7FFAFB78  mr r26, r31
	ctx.r[26].u64 = ctx.r[31].u64;
	// 82DF70A4: 4BFFE6AD  bl 0x82df5750
	ctx.lr = 0x82DF70A8;
	sub_82DF5750(ctx, base);
	// 82DF70A8: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF70AC: 894B002D  lbz r10, 0x2d(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(45 as u32) ) } as u64;
	// 82DF70B0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF70B4: 83210104  lwz r25, 0x104(r1)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(260 as u32) ) } as u64;
	// 82DF70B8: 419A000C  beq cr6, 0x82df70c4
	if ctx.cr[6].eq {
	pc = 0x82DF70C4; continue 'dispatch;
	}
	// 82DF70BC: 839A0008  lwz r28, 8(r26)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF70C0: 48000028  b 0x82df70e8
	pc = 0x82DF70E8; continue 'dispatch;
	// 82DF70C4: 815A0008  lwz r10, 8(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF70C8: 894A002D  lbz r10, 0x2d(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(45 as u32) ) } as u64;
	// 82DF70CC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF70D0: 419A000C  beq cr6, 0x82df70dc
	if ctx.cr[6].eq {
	pc = 0x82DF70DC; continue 'dispatch;
	}
	// 82DF70D4: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 82DF70D8: 48000010  b 0x82df70e8
	pc = 0x82DF70E8; continue 'dispatch;
	// 82DF70DC: 83990008  lwz r28, 8(r25)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF70E0: 7F19D040  cmplw cr6, r25, r26
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82DF70E4: 409A00DC  bne cr6, 0x82df71c0
	if !ctx.cr[6].eq {
	pc = 0x82DF71C0; continue 'dispatch;
	}
	// 82DF70E8: 897C002D  lbz r11, 0x2d(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(45 as u32) ) } as u64;
	// 82DF70EC: 83FA0004  lwz r31, 4(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF70F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF70F4: 409A0008  bne cr6, 0x82df70fc
	if !ctx.cr[6].eq {
	pc = 0x82DF70FC; continue 'dispatch;
	}
	// 82DF70F8: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82DF70FC: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF7100: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF7104: 7F0AD040  cmplw cr6, r10, r26
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82DF7108: 409A000C  bne cr6, 0x82df7114
	if !ctx.cr[6].eq {
	pc = 0x82DF7114; continue 'dispatch;
	}
	// 82DF710C: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82DF7110: 4800001C  b 0x82df712c
	pc = 0x82DF712C; continue 'dispatch;
	// 82DF7114: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF7118: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82DF711C: 409A000C  bne cr6, 0x82df7128
	if !ctx.cr[6].eq {
	pc = 0x82DF7128; continue 'dispatch;
	}
	// 82DF7120: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82DF7124: 48000008  b 0x82df712c
	pc = 0x82DF712C; continue 'dispatch;
	// 82DF7128: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82DF712C: 813B0004  lwz r9, 4(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF7130: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF7134: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82DF7138: 409A003C  bne cr6, 0x82df7174
	if !ctx.cr[6].eq {
	pc = 0x82DF7174; continue 'dispatch;
	}
	// 82DF713C: 897C002D  lbz r11, 0x2d(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(45 as u32) ) } as u64;
	// 82DF7140: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF7144: 419A000C  beq cr6, 0x82df7150
	if ctx.cr[6].eq {
	pc = 0x82DF7150; continue 'dispatch;
	}
	// 82DF7148: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 82DF714C: 48000024  b 0x82df7170
	pc = 0x82DF7170; continue 'dispatch;
	// 82DF7150: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF7154: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 82DF7158: 4800000C  b 0x82df7164
	pc = 0x82DF7164; continue 'dispatch;
	// 82DF715C: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82DF7160: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF7164: 890B002D  lbz r8, 0x2d(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(45 as u32) ) } as u64;
	// 82DF7168: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82DF716C: 419AFFF0  beq cr6, 0x82df715c
	if ctx.cr[6].eq {
	pc = 0x82DF715C; continue 'dispatch;
	}
	// 82DF7170: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DF7174: 813B0004  lwz r9, 4(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF7178: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF717C: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82DF7180: 409A00D4  bne cr6, 0x82df7254
	if !ctx.cr[6].eq {
	pc = 0x82DF7254; continue 'dispatch;
	}
	// 82DF7184: 897C002D  lbz r11, 0x2d(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(45 as u32) ) } as u64;
	// 82DF7188: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF718C: 419A000C  beq cr6, 0x82df7198
	if ctx.cr[6].eq {
	pc = 0x82DF7198; continue 'dispatch;
	}
	// 82DF7190: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 82DF7194: 48000024  b 0x82df71b8
	pc = 0x82DF71B8; continue 'dispatch;
	// 82DF7198: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF719C: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 82DF71A0: 4800000C  b 0x82df71ac
	pc = 0x82DF71AC; continue 'dispatch;
	// 82DF71A4: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82DF71A8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF71AC: 890B002D  lbz r8, 0x2d(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(45 as u32) ) } as u64;
	// 82DF71B0: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82DF71B4: 419AFFF0  beq cr6, 0x82df71a4
	if ctx.cr[6].eq {
	pc = 0x82DF71A4; continue 'dispatch;
	}
	// 82DF71B8: 91490008  stw r10, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82DF71BC: 48000098  b 0x82df7254
	pc = 0x82DF7254; continue 'dispatch;
	// 82DF71C0: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 82DF71C4: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF71C8: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF71CC: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF71D0: 7F195840  cmplw cr6, r25, r11
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF71D4: 409A000C  bne cr6, 0x82df71e0
	if !ctx.cr[6].eq {
	pc = 0x82DF71E0; continue 'dispatch;
	}
	// 82DF71D8: 7F3FCB78  mr r31, r25
	ctx.r[31].u64 = ctx.r[25].u64;
	// 82DF71DC: 4800002C  b 0x82df7208
	pc = 0x82DF7208; continue 'dispatch;
	// 82DF71E0: 897C002D  lbz r11, 0x2d(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(45 as u32) ) } as u64;
	// 82DF71E4: 83F90004  lwz r31, 4(r25)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF71E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF71EC: 409A0008  bne cr6, 0x82df71f4
	if !ctx.cr[6].eq {
	pc = 0x82DF71F4; continue 'dispatch;
	}
	// 82DF71F0: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82DF71F4: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82DF71F8: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF71FC: 91790008  stw r11, 8(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DF7200: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF7204: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 82DF7208: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF720C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF7210: 7F0AD040  cmplw cr6, r10, r26
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82DF7214: 409A000C  bne cr6, 0x82df7220
	if !ctx.cr[6].eq {
	pc = 0x82DF7220; continue 'dispatch;
	}
	// 82DF7218: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 82DF721C: 48000020  b 0x82df723c
	pc = 0x82DF723C; continue 'dispatch;
	// 82DF7220: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF7224: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF7228: 7F0AD040  cmplw cr6, r10, r26
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82DF722C: 409A000C  bne cr6, 0x82df7238
	if !ctx.cr[6].eq {
	pc = 0x82DF7238; continue 'dispatch;
	}
	// 82DF7230: 932B0000  stw r25, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 82DF7234: 48000008  b 0x82df723c
	pc = 0x82DF723C; continue 'dispatch;
	// 82DF7238: 932B0008  stw r25, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[25].u32 ) };
	// 82DF723C: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF7240: 91790004  stw r11, 4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DF7244: 897A002C  lbz r11, 0x2c(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DF7248: 8959002C  lbz r10, 0x2c(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[25].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DF724C: 9979002C  stb r11, 0x2c(r25)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[25].u32.wrapping_add(44 as u32), ctx.r[11].u8 ) };
	// 82DF7250: 995A002C  stb r10, 0x2c(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(44 as u32), ctx.r[10].u8 ) };
	// 82DF7254: 897A002C  lbz r11, 0x2c(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DF7258: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82DF725C: 409A0198  bne cr6, 0x82df73f4
	if !ctx.cr[6].eq {
	pc = 0x82DF73F4; continue 'dispatch;
	}
	// 82DF7260: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF7264: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82DF7268: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF726C: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF7270: 419A0180  beq cr6, 0x82df73f0
	if ctx.cr[6].eq {
	pc = 0x82DF73F0; continue 'dispatch;
	}
	// 82DF7274: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82DF7278: 897C002C  lbz r11, 0x2c(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DF727C: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82DF7280: 409A0170  bne cr6, 0x82df73f0
	if !ctx.cr[6].eq {
	pc = 0x82DF73F0; continue 'dispatch;
	}
	// 82DF7284: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF7288: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF728C: 409A00A8  bne cr6, 0x82df7334
	if !ctx.cr[6].eq {
	pc = 0x82DF7334; continue 'dispatch;
	}
	// 82DF7290: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF7294: 894B002C  lbz r10, 0x2c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DF7298: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF729C: 409A001C  bne cr6, 0x82df72b8
	if !ctx.cr[6].eq {
	pc = 0x82DF72B8; continue 'dispatch;
	}
	// 82DF72A0: 9BCB002C  stb r30, 0x2c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), ctx.r[30].u8 ) };
	// 82DF72A4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF72A8: 9BBF002C  stb r29, 0x2c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[29].u8 ) };
	// 82DF72AC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82DF72B0: 4BFFE769  bl 0x82df5a18
	ctx.lr = 0x82DF72B4;
	sub_82DF5A18(ctx, base);
	// 82DF72B4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF72B8: 894B002D  lbz r10, 0x2d(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(45 as u32) ) } as u64;
	// 82DF72BC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF72C0: 409A00C8  bne cr6, 0x82df7388
	if !ctx.cr[6].eq {
	pc = 0x82DF7388; continue 'dispatch;
	}
	// 82DF72C4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF72C8: 894A002C  lbz r10, 0x2c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DF72CC: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82DF72D0: 409A0014  bne cr6, 0x82df72e4
	if !ctx.cr[6].eq {
	pc = 0x82DF72E4; continue 'dispatch;
	}
	// 82DF72D4: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF72D8: 894A002C  lbz r10, 0x2c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DF72DC: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82DF72E0: 419A00A4  beq cr6, 0x82df7384
	if ctx.cr[6].eq {
	pc = 0x82DF7384; continue 'dispatch;
	}
	// 82DF72E4: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF72E8: 894A002C  lbz r10, 0x2c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DF72EC: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82DF72F0: 409A0020  bne cr6, 0x82df7310
	if !ctx.cr[6].eq {
	pc = 0x82DF7310; continue 'dispatch;
	}
	// 82DF72F4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF72F8: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DF72FC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82DF7300: 9BCA002C  stb r30, 0x2c(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(44 as u32), ctx.r[30].u8 ) };
	// 82DF7304: 9BAB002C  stb r29, 0x2c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), ctx.r[29].u8 ) };
	// 82DF7308: 48020589  bl 0x82e17890
	ctx.lr = 0x82DF730C;
	sub_82E17890(ctx, base);
	// 82DF730C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF7310: 895F002C  lbz r10, 0x2c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DF7314: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF7318: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82DF731C: 994B002C  stb r10, 0x2c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), ctx.r[10].u8 ) };
	// 82DF7320: 9BDF002C  stb r30, 0x2c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[30].u8 ) };
	// 82DF7324: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF7328: 9BCB002C  stb r30, 0x2c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), ctx.r[30].u8 ) };
	// 82DF732C: 4BFFE6ED  bl 0x82df5a18
	ctx.lr = 0x82DF7330;
	sub_82DF5A18(ctx, base);
	// 82DF7330: 480000C0  b 0x82df73f0
	pc = 0x82DF73F0; continue 'dispatch;
	// 82DF7334: 894B002C  lbz r10, 0x2c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DF7338: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF733C: 409A001C  bne cr6, 0x82df7358
	if !ctx.cr[6].eq {
	pc = 0x82DF7358; continue 'dispatch;
	}
	// 82DF7340: 9BCB002C  stb r30, 0x2c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), ctx.r[30].u8 ) };
	// 82DF7344: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF7348: 9BBF002C  stb r29, 0x2c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[29].u8 ) };
	// 82DF734C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82DF7350: 48020541  bl 0x82e17890
	ctx.lr = 0x82DF7354;
	sub_82E17890(ctx, base);
	// 82DF7354: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF7358: 894B002D  lbz r10, 0x2d(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(45 as u32) ) } as u64;
	// 82DF735C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF7360: 409A0028  bne cr6, 0x82df7388
	if !ctx.cr[6].eq {
	pc = 0x82DF7388; continue 'dispatch;
	}
	// 82DF7364: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF7368: 894A002C  lbz r10, 0x2c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DF736C: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82DF7370: 409A0034  bne cr6, 0x82df73a4
	if !ctx.cr[6].eq {
	pc = 0x82DF73A4; continue 'dispatch;
	}
	// 82DF7374: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF7378: 894A002C  lbz r10, 0x2c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DF737C: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82DF7380: 409A0024  bne cr6, 0x82df73a4
	if !ctx.cr[6].eq {
	pc = 0x82DF73A4; continue 'dispatch;
	}
	// 82DF7384: 9BAB002C  stb r29, 0x2c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), ctx.r[29].u8 ) };
	// 82DF7388: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF738C: 7FFCFB78  mr r28, r31
	ctx.r[28].u64 = ctx.r[31].u64;
	// 82DF7390: 83FF0004  lwz r31, 4(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF7394: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF7398: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF739C: 409AFEDC  bne cr6, 0x82df7278
	if !ctx.cr[6].eq {
	pc = 0x82DF7278; continue 'dispatch;
	}
	// 82DF73A0: 48000050  b 0x82df73f0
	pc = 0x82DF73F0; continue 'dispatch;
	// 82DF73A4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF73A8: 894A002C  lbz r10, 0x2c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DF73AC: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82DF73B0: 409A0020  bne cr6, 0x82df73d0
	if !ctx.cr[6].eq {
	pc = 0x82DF73D0; continue 'dispatch;
	}
	// 82DF73B4: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF73B8: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DF73BC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82DF73C0: 9BCA002C  stb r30, 0x2c(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(44 as u32), ctx.r[30].u8 ) };
	// 82DF73C4: 9BAB002C  stb r29, 0x2c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), ctx.r[29].u8 ) };
	// 82DF73C8: 4BFFE651  bl 0x82df5a18
	ctx.lr = 0x82DF73CC;
	sub_82DF5A18(ctx, base);
	// 82DF73CC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF73D0: 895F002C  lbz r10, 0x2c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DF73D4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF73D8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82DF73DC: 994B002C  stb r10, 0x2c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), ctx.r[10].u8 ) };
	// 82DF73E0: 9BDF002C  stb r30, 0x2c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[30].u8 ) };
	// 82DF73E4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF73E8: 9BCB002C  stb r30, 0x2c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), ctx.r[30].u8 ) };
	// 82DF73EC: 480204A5  bl 0x82e17890
	ctx.lr = 0x82DF73F0;
	sub_82E17890(ctx, base);
	// 82DF73F0: 9BDC002C  stb r30, 0x2c(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(44 as u32), ctx.r[30].u8 ) };
	// 82DF73F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF73F8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DF73FC: 387A000C  addi r3, r26, 0xc
	ctx.r[3].s64 = ctx.r[26].s64 + 12;
	// 82DF7400: 4B4D1A31  bl 0x822c8e30
	ctx.lr = 0x82DF7404;
	sub_822C8E30(ctx, base);
	// 82DF7404: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82DF7408: 4B4C8E61  bl 0x822c0268
	ctx.lr = 0x82DF740C;
	sub_822C0268(ctx, base);
	// 82DF740C: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF7410: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF7414: 419A000C  beq cr6, 0x82df7420
	if ctx.cr[6].eq {
	pc = 0x82DF7420; continue 'dispatch;
	}
	// 82DF7418: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DF741C: 917B0008  stw r11, 8(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DF7420: 93380000  stw r25, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 82DF7424: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82DF7428: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82DF742C: 483B0D7C  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF7430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF7430 size=96
    let mut pc: u32 = 0x82DF7430;
    'dispatch: loop {
        match pc {
            0x82DF7430 => {
    //   block [0x82DF7430..0x82DF7490)
	// 82DF7430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF7434: 483B0D39  bl 0x831a816c
	ctx.lr = 0x82DF7438;
	sub_831A8130(ctx, base);
	// 82DF7438: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF743C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DF7440: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DF7444: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 82DF7448: 897E002D  lbz r11, 0x2d(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(45 as u32) ) } as u64;
	// 82DF744C: 48000034  b 0x82df7480
	pc = 0x82DF7480; continue 'dispatch;
	// 82DF7450: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DF7454: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF7458: 4BFFFFD9  bl 0x82df7430
	ctx.lr = 0x82DF745C;
	sub_82DF7430(ctx, base);
	// 82DF745C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF7460: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DF7464: 387E000C  addi r3, r30, 0xc
	ctx.r[3].s64 = ctx.r[30].s64 + 12;
	// 82DF7468: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF746C: 4B4D19C5  bl 0x822c8e30
	ctx.lr = 0x82DF7470;
	sub_822C8E30(ctx, base);
	// 82DF7470: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF7474: 4B4C8DF5  bl 0x822c0268
	ctx.lr = 0x82DF7478;
	sub_822C0268(ctx, base);
	// 82DF7478: 897F002D  lbz r11, 0x2d(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(45 as u32) ) } as u64;
	// 82DF747C: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	// 82DF7480: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF7484: 419AFFCC  beq cr6, 0x82df7450
	if ctx.cr[6].eq {
	pc = 0x82DF7450; continue 'dispatch;
	}
	// 82DF7488: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF748C: 483B0D30  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF7490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF7490 size=88
    let mut pc: u32 = 0x82DF7490;
    'dispatch: loop {
        match pc {
            0x82DF7490 => {
    //   block [0x82DF7490..0x82DF74E8)
	// 82DF7490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF7494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF7498: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DF749C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF74A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF74A4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DF74A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF74AC: 7F03F040  cmplw cr6, r3, r30
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82DF74B0: 419A0020  beq cr6, 0x82df74d0
	if ctx.cr[6].eq {
	pc = 0x82DF74D0; continue 'dispatch;
	}
	// 82DF74B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF74B8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DF74BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF74C0: 4B4D1971  bl 0x822c8e30
	ctx.lr = 0x82DF74C4;
	sub_822C8E30(ctx, base);
	// 82DF74C4: 3BFF001C  addi r31, r31, 0x1c
	ctx.r[31].s64 = ctx.r[31].s64 + 28;
	// 82DF74C8: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82DF74CC: 409AFFE8  bne cr6, 0x82df74b4
	if !ctx.cr[6].eq {
	pc = 0x82DF74B4; continue 'dispatch;
	}
	// 82DF74D0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF74D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF74D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF74DC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DF74E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF74E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF74E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF74E8 size=120
    let mut pc: u32 = 0x82DF74E8;
    'dispatch: loop {
        match pc {
            0x82DF74E8 => {
    //   block [0x82DF74E8..0x82DF7560)
	// 82DF74E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF74EC: 483B0C81  bl 0x831a816c
	ctx.lr = 0x82DF74F0;
	sub_831A8130(ctx, base);
	// 82DF74F0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF74F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF74F8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DF74FC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DF7500: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF7504: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DF7508: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF750C: 4B4D1925  bl 0x822c8e30
	ctx.lr = 0x82DF7510;
	sub_822C8E30(ctx, base);
	// 82DF7510: 4800002C  b 0x82df753c
	pc = 0x82DF753C; continue 'dispatch;
	// 82DF7514: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82DF7518: 419A0010  beq cr6, 0x82df7528
	if ctx.cr[6].eq {
	pc = 0x82DF7528; continue 'dispatch;
	}
	// 82DF751C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DF7520: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF7524: 4B4D2935  bl 0x822c9e58
	ctx.lr = 0x82DF7528;
	sub_822C9E58(ctx, base);
	// 82DF7528: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF752C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF7530: 4BFFEB39  bl 0x82df6068
	ctx.lr = 0x82DF7534;
	sub_82DF6068(ctx, base);
	// 82DF7534: 3BFF001C  addi r31, r31, 0x1c
	ctx.r[31].s64 = ctx.r[31].s64 + 28;
	// 82DF7538: 3BDE001C  addi r30, r30, 0x1c
	ctx.r[30].s64 = ctx.r[30].s64 + 28;
	// 82DF753C: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82DF7540: 409AFFD4  bne cr6, 0x82df7514
	if !ctx.cr[6].eq {
	pc = 0x82DF7514; continue 'dispatch;
	}
	// 82DF7544: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF7548: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DF754C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF7550: 4B4D18E1  bl 0x822c8e30
	ctx.lr = 0x82DF7554;
	sub_822C8E30(ctx, base);
	// 82DF7554: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF7558: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DF755C: 483B0C60  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF7560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF7560 size=88
    let mut pc: u32 = 0x82DF7560;
    'dispatch: loop {
        match pc {
            0x82DF7560 => {
    //   block [0x82DF7560..0x82DF75B8)
	// 82DF7560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF7564: 483B0C01  bl 0x831a8164
	ctx.lr = 0x82DF7568;
	sub_831A8130(ctx, base);
	// 82DF7568: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF756C: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82DF7570: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82DF7574: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82DF7578: 7FFDFB78  mr r29, r31
	ctx.r[29].u64 = ctx.r[31].u64;
	// 82DF757C: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 82DF7580: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82DF7584: 419A0024  beq cr6, 0x82df75a8
	if ctx.cr[6].eq {
	pc = 0x82DF75A8; continue 'dispatch;
	}
	// 82DF7588: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82DF758C: 419A0010  beq cr6, 0x82df759c
	if ctx.cr[6].eq {
	pc = 0x82DF759C; continue 'dispatch;
	}
	// 82DF7590: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82DF7594: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF7598: 4B4D28C1  bl 0x822c9e58
	ctx.lr = 0x82DF759C;
	sub_822C9E58(ctx, base);
	// 82DF759C: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82DF75A0: 3BDE001C  addi r30, r30, 0x1c
	ctx.r[30].s64 = ctx.r[30].s64 + 28;
	// 82DF75A4: 4082FFE4  bne 0x82df7588
	if !ctx.cr[0].eq {
	pc = 0x82DF7588; continue 'dispatch;
	}
	// 82DF75A8: 1D7F001C  mulli r11, r31, 0x1c
	ctx.r[11].s64 = ctx.r[31].s64 * 28;
	// 82DF75AC: 7C6BE214  add r3, r11, r28
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82DF75B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DF75B4: 483B0C00  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF75B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF75B8 size=236
    let mut pc: u32 = 0x82DF75B8;
    'dispatch: loop {
        match pc {
            0x82DF75B8 => {
    //   block [0x82DF75B8..0x82DF76A4)
	// 82DF75B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF75BC: 483B0BA1  bl 0x831a815c
	ctx.lr = 0x82DF75C0;
	sub_831A8130(ctx, base);
	// 82DF75C0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF75C4: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82DF75C8: 3B200001  li r25, 1
	ctx.r[25].s64 = 1;
	// 82DF75CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF75D0: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82DF75D4: 7F3CCB78  mr r28, r25
	ctx.r[28].u64 = ctx.r[25].u64;
	// 82DF75D8: 83DA0004  lwz r30, 4(r26)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF75DC: 83BE0004  lwz r29, 4(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF75E0: 4800002C  b 0x82df760c
	pc = 0x82DF760C; continue 'dispatch;
	// 82DF75E4: 389D000C  addi r4, r29, 0xc
	ctx.r[4].s64 = ctx.r[29].s64 + 12;
	// 82DF75E8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82DF75EC: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 82DF75F0: 4BFFBC49  bl 0x82df3238
	ctx.lr = 0x82DF75F4;
	sub_82DF3238(ctx, base);
	// 82DF75F4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82DF75F8: 578B063F  clrlwi. r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF75FC: 4182000C  beq 0x82df7608
	if ctx.cr[0].eq {
	pc = 0x82DF7608; continue 'dispatch;
	}
	// 82DF7600: 83BD0000  lwz r29, 0(r29)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF7604: 48000008  b 0x82df760c
	pc = 0x82DF760C; continue 'dispatch;
	// 82DF7608: 83BD0008  lwz r29, 8(r29)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF760C: 897D0015  lbz r11, 0x15(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(21 as u32) ) } as u64;
	// 82DF7610: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF7614: 419AFFD0  beq cr6, 0x82df75e4
	if ctx.cr[6].eq {
	pc = 0x82DF75E4; continue 'dispatch;
	}
	// 82DF7618: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 82DF761C: 578B063F  clrlwi. r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF7620: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 82DF7624: 41820048  beq 0x82df766c
	if ctx.cr[0].eq {
	pc = 0x82DF766C; continue 'dispatch;
	}
	// 82DF7628: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF762C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF7630: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF7634: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF7638: 409A002C  bne cr6, 0x82df7664
	if !ctx.cr[6].eq {
	pc = 0x82DF7664; continue 'dispatch;
	}
	// 82DF763C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DF7640: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82DF7644: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82DF7648: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82DF764C: 4BFFEB7D  bl 0x82df61c8
	ctx.lr = 0x82DF7650;
	sub_82DF61C8(ctx, base);
	// 82DF7650: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DF7654: 9B3F0004  stb r25, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[25].u8 ) };
	// 82DF7658: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF765C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF7660: 48000038  b 0x82df7698
	pc = 0x82DF7698; continue 'dispatch;
	// 82DF7664: 480257FD  bl 0x82e1ce60
	ctx.lr = 0x82DF7668;
	sub_82E1CE60(ctx, base);
	// 82DF7668: 83A10050  lwz r29, 0x50(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DF766C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82DF7670: 387D000C  addi r3, r29, 0xc
	ctx.r[3].s64 = ctx.r[29].s64 + 12;
	// 82DF7674: 4BFFBBC5  bl 0x82df3238
	ctx.lr = 0x82DF7678;
	sub_82DF3238(ctx, base);
	// 82DF7678: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF767C: 41820010  beq 0x82df768c
	if ctx.cr[0].eq {
	pc = 0x82DF768C; continue 'dispatch;
	}
	// 82DF7680: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82DF7684: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF7688: 4BFFFFB8  b 0x82df7640
	pc = 0x82DF7640; continue 'dispatch;
	// 82DF768C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF7690: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82DF7694: 997F0004  stb r11, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u8 ) };
	// 82DF7698: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF769C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82DF76A0: 483B0B0C  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF76A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF76A8 size=132
    let mut pc: u32 = 0x82DF76A8;
    'dispatch: loop {
        match pc {
            0x82DF76A8 => {
    //   block [0x82DF76A8..0x82DF772C)
	// 82DF76A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF76AC: 483B0ABD  bl 0x831a8168
	ctx.lr = 0x82DF76B0;
	sub_831A8130(ctx, base);
	// 82DF76B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF76B4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DF76B8: 90A100A4  stw r5, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[5].u32 ) };
	// 82DF76BC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DF76C0: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82DF76C4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF76C8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF76CC: 7F055040  cmplw cr6, r5, r10
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DF76D0: 409A0044  bne cr6, 0x82df7714
	if !ctx.cr[6].eq {
	pc = 0x82DF7714; continue 'dispatch;
	}
	// 82DF76D4: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF76D8: 409A003C  bne cr6, 0x82df7714
	if !ctx.cr[6].eq {
	pc = 0x82DF7714; continue 'dispatch;
	}
	// 82DF76DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF76E0: 4BFFE8D9  bl 0x82df5fb8
	ctx.lr = 0x82DF76E4;
	sub_82DF5FB8(ctx, base);
	// 82DF76E4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF76E8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF76EC: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF76F0: 48000030  b 0x82df7720
	pc = 0x82DF7720; continue 'dispatch;
	// 82DF76F4: 386100A4  addi r3, r1, 0xa4
	ctx.r[3].s64 = ctx.r[1].s64 + 164;
	// 82DF76F8: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82DF76FC: 4800455D  bl 0x82dfbc58
	ctx.lr = 0x82DF7700;
	sub_82DFBC58(ctx, base);
	// 82DF7700: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82DF7704: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF7708: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF770C: 4BFFF135  bl 0x82df6840
	ctx.lr = 0x82DF7710;
	sub_82DF6840(ctx, base);
	// 82DF7710: 80A100A4  lwz r5, 0xa4(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82DF7714: 7F05F040  cmplw cr6, r5, r30
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82DF7718: 409AFFDC  bne cr6, 0x82df76f4
	if !ctx.cr[6].eq {
	pc = 0x82DF76F4; continue 'dispatch;
	}
	// 82DF771C: 90BD0000  stw r5, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82DF7720: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DF7724: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DF7728: 483B0A90  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF7730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF7730 size=848
    let mut pc: u32 = 0x82DF7730;
    'dispatch: loop {
        match pc {
            0x82DF7730 => {
    //   block [0x82DF7730..0x82DF7A80)
	// 82DF7730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF7734: 483B0A25  bl 0x831a8158
	ctx.lr = 0x82DF7738;
	sub_831A8130(ctx, base);
	// 82DF7738: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF773C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF7740: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82DF7744: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 82DF7748: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF774C: 7CB82B78  mr r24, r5
	ctx.r[24].u64 = ctx.r[5].u64;
	// 82DF7750: 4BFFC4B1  bl 0x82df3c00
	ctx.lr = 0x82DF7754;
	sub_82DF3C00(ctx, base);
	// 82DF7754: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF7758: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF775C: 409A000C  bne cr6, 0x82df7768
	if !ctx.cr[6].eq {
	pc = 0x82DF7768; continue 'dispatch;
	}
	// 82DF7760: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82DF7764: 48000010  b 0x82df7774
	pc = 0x82DF7774; continue 'dispatch;
	// 82DF7768: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DF776C: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82DF7770: 7D681670  srawi r8, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 82DF7774: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 82DF7778: 419A02F8  beq cr6, 0x82df7a70
	if ctx.cr[6].eq {
	pc = 0x82DF7A70; continue 'dispatch;
	}
	// 82DF777C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF7780: 409A000C  bne cr6, 0x82df778c
	if !ctx.cr[6].eq {
	pc = 0x82DF778C; continue 'dispatch;
	}
	// 82DF7784: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF7788: 48000010  b 0x82df7798
	pc = 0x82DF7798; continue 'dispatch;
	// 82DF778C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF7790: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82DF7794: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 82DF7798: 3D203FFF  lis r9, 0x3fff
	ctx.r[9].s64 = 1073676288;
	// 82DF779C: 6129FFFF  ori r9, r9, 0xffff
	ctx.r[9].u64 = ctx.r[9].u64 | 65535;
	// 82DF77A0: 7D6B4850  subf r11, r11, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 82DF77A4: 7F0BC040  cmplw cr6, r11, r24
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[24].u32, &mut ctx.xer);
	// 82DF77A8: 4098000C  bge cr6, 0x82df77b4
	if !ctx.cr[6].lt {
	pc = 0x82DF77B4; continue 'dispatch;
	}
	// 82DF77AC: 482CD98D  bl 0x830c5138
	ctx.lr = 0x82DF77B0;
	sub_830C5138(ctx, base);
	// 82DF77B0: 480002C0  b 0x82df7a70
	pc = 0x82DF7A70; continue 'dispatch;
	// 82DF77B4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF77B8: 409A000C  bne cr6, 0x82df77c4
	if !ctx.cr[6].eq {
	pc = 0x82DF77C4; continue 'dispatch;
	}
	// 82DF77BC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF77C0: 48000010  b 0x82df77d0
	pc = 0x82DF77D0; continue 'dispatch;
	// 82DF77C4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF77C8: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82DF77CC: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 82DF77D0: 7D6BC214  add r11, r11, r24
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[24].u64;
	// 82DF77D4: 7F085840  cmplw cr6, r8, r11
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF77D8: 40980168  bge cr6, 0x82df7940
	if !ctx.cr[6].lt {
	pc = 0x82DF7940; continue 'dispatch;
	}
	// 82DF77DC: 550BF87E  srwi r11, r8, 1
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DF77E0: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82DF77E4: 7D2B4850  subf r9, r11, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 82DF77E8: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82DF77EC: 41980008  blt cr6, 0x82df77f4
	if ctx.cr[6].lt {
	pc = 0x82DF77F4; continue 'dispatch;
	}
	// 82DF77F0: 7F2B4214  add r25, r11, r8
	ctx.r[25].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82DF77F4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF77F8: 409A000C  bne cr6, 0x82df7804
	if !ctx.cr[6].eq {
	pc = 0x82DF7804; continue 'dispatch;
	}
	// 82DF77FC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF7800: 48000010  b 0x82df7810
	pc = 0x82DF7810; continue 'dispatch;
	// 82DF7804: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF7808: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82DF780C: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 82DF7810: 7D6BC214  add r11, r11, r24
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[24].u64;
	// 82DF7814: 7F195840  cmplw cr6, r25, r11
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF7818: 40980024  bge cr6, 0x82df783c
	if !ctx.cr[6].lt {
	pc = 0x82DF783C; continue 'dispatch;
	}
	// 82DF781C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF7820: 409A000C  bne cr6, 0x82df782c
	if !ctx.cr[6].eq {
	pc = 0x82DF782C; continue 'dispatch;
	}
	// 82DF7824: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF7828: 48000010  b 0x82df7838
	pc = 0x82DF7838; continue 'dispatch;
	// 82DF782C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF7830: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82DF7834: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 82DF7838: 7F2BC214  add r25, r11, r24
	ctx.r[25].u64 = ctx.r[11].u64 + ctx.r[24].u64;
	// 82DF783C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DF7840: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82DF7844: 4BBF6625  bl 0x829ede68
	ctx.lr = 0x82DF7848;
	sub_829EDE68(ctx, base);
	// 82DF7848: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82DF784C: 83DF0004  lwz r30, 4(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF7850: 7F7DDB78  mr r29, r27
	ctx.r[29].u64 = ctx.r[27].u64;
	// 82DF7854: 48000020  b 0x82df7874
	pc = 0x82DF7874; continue 'dispatch;
	// 82DF7858: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82DF785C: 419A0010  beq cr6, 0x82df786c
	if ctx.cr[6].eq {
	pc = 0x82DF786C; continue 'dispatch;
	}
	// 82DF7860: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DF7864: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DF7868: 4BFFC399  bl 0x82df3c00
	ctx.lr = 0x82DF786C;
	sub_82DF3C00(ctx, base);
	// 82DF786C: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82DF7870: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82DF7874: 7F1ED040  cmplw cr6, r30, r26
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82DF7878: 409AFFE0  bne cr6, 0x82df7858
	if !ctx.cr[6].eq {
	pc = 0x82DF7858; continue 'dispatch;
	}
	// 82DF787C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82DF7880: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 82DF7884: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DF7888: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF788C: 4BFFE6D5  bl 0x82df5f60
	ctx.lr = 0x82DF7890;
	sub_82DF5F60(ctx, base);
	// 82DF7890: 83BF0008  lwz r29, 8(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF7894: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DF7898: 7F1AE840  cmplw cr6, r26, r29
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82DF789C: 419A002C  beq cr6, 0x82df78c8
	if ctx.cr[6].eq {
	pc = 0x82DF78C8; continue 'dispatch;
	}
	// 82DF78A0: 7F9ED050  subf r28, r30, r26
	ctx.r[28].s64 = ctx.r[26].s64 - ctx.r[30].s64;
	// 82DF78A4: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82DF78A8: 419A0010  beq cr6, 0x82df78b8
	if ctx.cr[6].eq {
	pc = 0x82DF78B8; continue 'dispatch;
	}
	// 82DF78AC: 7C9CF214  add r4, r28, r30
	ctx.r[4].u64 = ctx.r[28].u64 + ctx.r[30].u64;
	// 82DF78B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF78B4: 4BFFC34D  bl 0x82df3c00
	ctx.lr = 0x82DF78B8;
	sub_82DF3C00(ctx, base);
	// 82DF78B8: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82DF78BC: 7D7CF214  add r11, r28, r30
	ctx.r[11].u64 = ctx.r[28].u64 + ctx.r[30].u64;
	// 82DF78C0: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82DF78C4: 409AFFE0  bne cr6, 0x82df78a4
	if !ctx.cr[6].eq {
	pc = 0x82DF78A4; continue 'dispatch;
	}
	// 82DF78C8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF78CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF78D0: 409A000C  bne cr6, 0x82df78dc
	if !ctx.cr[6].eq {
	pc = 0x82DF78DC; continue 'dispatch;
	}
	// 82DF78D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DF78D8: 48000010  b 0x82df78e8
	pc = 0x82DF78E8; continue 'dispatch;
	// 82DF78DC: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF78E0: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82DF78E4: 7D4A1670  srawi r10, r10, 2
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 2) as i64;
	// 82DF78E8: 7F8AC214  add r28, r10, r24
	ctx.r[28].u64 = ctx.r[10].u64 + ctx.r[24].u64;
	// 82DF78EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF78F0: 419A0030  beq cr6, 0x82df7920
	if ctx.cr[6].eq {
	pc = 0x82DF7920; continue 'dispatch;
	}
	// 82DF78F4: 83BF0008  lwz r29, 8(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF78F8: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82DF78FC: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82DF7900: 419A0018  beq cr6, 0x82df7918
	if ctx.cr[6].eq {
	pc = 0x82DF7918; continue 'dispatch;
	}
	// 82DF7904: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF7908: 4BFFBB21  bl 0x82df3428
	ctx.lr = 0x82DF790C;
	sub_82DF3428(ctx, base);
	// 82DF790C: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82DF7910: 7F1EE840  cmplw cr6, r30, r29
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82DF7914: 409AFFF0  bne cr6, 0x82df7904
	if !ctx.cr[6].eq {
	pc = 0x82DF7904; continue 'dispatch;
	}
	// 82DF7918: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF791C: 4B4C894D  bl 0x822c0268
	ctx.lr = 0x82DF7920;
	sub_822C0268(ctx, base);
	// 82DF7920: 572B103A  slwi r11, r25, 2
	ctx.r[11].u32 = ctx.r[25].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DF7924: 937F0004  stw r27, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 82DF7928: 578A103A  slwi r10, r28, 2
	ctx.r[10].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DF792C: 7D6BDA14  add r11, r11, r27
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 82DF7930: 7D4ADA14  add r10, r10, r27
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[27].u64;
	// 82DF7934: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82DF7938: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82DF793C: 48000134  b 0x82df7a70
	pc = 0x82DF7A70; continue 'dispatch;
	// 82DF7940: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF7944: 7D7AF050  subf r11, r26, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[26].s64;
	// 82DF7948: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 82DF794C: 7F0BC040  cmplw cr6, r11, r24
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[24].u32, &mut ctx.xer);
	// 82DF7950: 40980090  bge cr6, 0x82df79e0
	if !ctx.cr[6].lt {
	pc = 0x82DF79E0; continue 'dispatch;
	}
	// 82DF7954: 571D103A  slwi r29, r24, 2
	ctx.r[29].u32 = ctx.r[24].u32.wrapping_shl(2);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82DF7958: 7F1AF040  cmplw cr6, r26, r30
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82DF795C: 7F9DD214  add r28, r29, r26
	ctx.r[28].u64 = ctx.r[29].u64 + ctx.r[26].u64;
	// 82DF7960: 419A002C  beq cr6, 0x82df798c
	if ctx.cr[6].eq {
	pc = 0x82DF798C; continue 'dispatch;
	}
	// 82DF7964: 7F7DE050  subf r27, r29, r28
	ctx.r[27].s64 = ctx.r[28].s64 - ctx.r[29].s64;
	// 82DF7968: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82DF796C: 419A0010  beq cr6, 0x82df797c
	if ctx.cr[6].eq {
	pc = 0x82DF797C; continue 'dispatch;
	}
	// 82DF7970: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82DF7974: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DF7978: 4BFFC289  bl 0x82df3c00
	ctx.lr = 0x82DF797C;
	sub_82DF3C00(ctx, base);
	// 82DF797C: 3B7B0004  addi r27, r27, 4
	ctx.r[27].s64 = ctx.r[27].s64 + 4;
	// 82DF7980: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 82DF7984: 7F1BF040  cmplw cr6, r27, r30
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82DF7988: 409AFFE0  bne cr6, 0x82df7968
	if !ctx.cr[6].eq {
	pc = 0x82DF7968; continue 'dispatch;
	}
	// 82DF798C: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF7990: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82DF7994: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF7998: 7D7A2050  subf r11, r26, r4
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[26].s64;
	// 82DF799C: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 82DF79A0: 7CABC050  subf r5, r11, r24
	ctx.r[5].s64 = ctx.r[24].s64 - ctx.r[11].s64;
	// 82DF79A4: 4BFFE5BD  bl 0x82df5f60
	ctx.lr = 0x82DF79A8;
	sub_82DF5F60(ctx, base);
	// 82DF79A8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF79AC: 7F5CD378  mr r28, r26
	ctx.r[28].u64 = ctx.r[26].u64;
	// 82DF79B0: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82DF79B4: 7FDD5850  subf r30, r29, r11
	ctx.r[30].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 82DF79B8: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DF79BC: 7F1AF040  cmplw cr6, r26, r30
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82DF79C0: 419A00B0  beq cr6, 0x82df7a70
	if ctx.cr[6].eq {
	pc = 0x82DF7A70; continue 'dispatch;
	}
	// 82DF79C4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DF79C8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DF79CC: 4BFFC205  bl 0x82df3bd0
	ctx.lr = 0x82DF79D0;
	sub_82DF3BD0(ctx, base);
	// 82DF79D0: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 82DF79D4: 7F1CF040  cmplw cr6, r28, r30
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82DF79D8: 409AFFEC  bne cr6, 0x82df79c4
	if !ctx.cr[6].eq {
	pc = 0x82DF79C4; continue 'dispatch;
	}
	// 82DF79DC: 48000094  b 0x82df7a70
	pc = 0x82DF7A70; continue 'dispatch;
	// 82DF79E0: 5719103A  slwi r25, r24, 2
	ctx.r[25].u32 = ctx.r[24].u32.wrapping_shl(2);
	ctx.r[25].u64 = ctx.r[25].u32 as u64;
	// 82DF79E4: 7FDCF378  mr r28, r30
	ctx.r[28].u64 = ctx.r[30].u64;
	// 82DF79E8: 7FB9F050  subf r29, r25, r30
	ctx.r[29].s64 = ctx.r[30].s64 - ctx.r[25].s64;
	// 82DF79EC: 7FBBEB78  mr r27, r29
	ctx.r[27].u64 = ctx.r[29].u64;
	// 82DF79F0: 7F1DF040  cmplw cr6, r29, r30
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82DF79F4: 419A0028  beq cr6, 0x82df7a1c
	if ctx.cr[6].eq {
	pc = 0x82DF7A1C; continue 'dispatch;
	}
	// 82DF79F8: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82DF79FC: 419A0010  beq cr6, 0x82df7a0c
	if ctx.cr[6].eq {
	pc = 0x82DF7A0C; continue 'dispatch;
	}
	// 82DF7A00: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82DF7A04: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DF7A08: 4BFFC1F9  bl 0x82df3c00
	ctx.lr = 0x82DF7A0C;
	sub_82DF3C00(ctx, base);
	// 82DF7A0C: 3B7B0004  addi r27, r27, 4
	ctx.r[27].s64 = ctx.r[27].s64 + 4;
	// 82DF7A10: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 82DF7A14: 7F1BF040  cmplw cr6, r27, r30
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82DF7A18: 409AFFE0  bne cr6, 0x82df79f8
	if !ctx.cr[6].eq {
	pc = 0x82DF79F8; continue 'dispatch;
	}
	// 82DF7A1C: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82DF7A20: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 82DF7A24: 7F1AE840  cmplw cr6, r26, r29
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82DF7A28: 419A0020  beq cr6, 0x82df7a48
	if ctx.cr[6].eq {
	pc = 0x82DF7A48; continue 'dispatch;
	}
	// 82DF7A2C: 7FDDF050  subf r30, r29, r30
	ctx.r[30].s64 = ctx.r[30].s64 - ctx.r[29].s64;
	// 82DF7A30: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 82DF7A34: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF7A38: 7C7EFA14  add r3, r30, r31
	ctx.r[3].u64 = ctx.r[30].u64 + ctx.r[31].u64;
	// 82DF7A3C: 4BFFC195  bl 0x82df3bd0
	ctx.lr = 0x82DF7A40;
	sub_82DF3BD0(ctx, base);
	// 82DF7A40: 7F1FD040  cmplw cr6, r31, r26
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82DF7A44: 409AFFEC  bne cr6, 0x82df7a30
	if !ctx.cr[6].eq {
	pc = 0x82DF7A30; continue 'dispatch;
	}
	// 82DF7A48: 7FD9D214  add r30, r25, r26
	ctx.r[30].u64 = ctx.r[25].u64 + ctx.r[26].u64;
	// 82DF7A4C: 7F5FD378  mr r31, r26
	ctx.r[31].u64 = ctx.r[26].u64;
	// 82DF7A50: 7F1AF040  cmplw cr6, r26, r30
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82DF7A54: 419A001C  beq cr6, 0x82df7a70
	if ctx.cr[6].eq {
	pc = 0x82DF7A70; continue 'dispatch;
	}
	// 82DF7A58: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DF7A5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF7A60: 4BFFC171  bl 0x82df3bd0
	ctx.lr = 0x82DF7A64;
	sub_82DF3BD0(ctx, base);
	// 82DF7A64: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82DF7A68: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82DF7A6C: 409AFFEC  bne cr6, 0x82df7a58
	if !ctx.cr[6].eq {
	pc = 0x82DF7A58; continue 'dispatch;
	}
	// 82DF7A70: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF7A74: 4BFFB9B5  bl 0x82df3428
	ctx.lr = 0x82DF7A78;
	sub_82DF3428(ctx, base);
	// 82DF7A78: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82DF7A7C: 483B072C  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF7A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF7A80 size=236
    let mut pc: u32 = 0x82DF7A80;
    'dispatch: loop {
        match pc {
            0x82DF7A80 => {
    //   block [0x82DF7A80..0x82DF7B6C)
	// 82DF7A80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF7A84: 483B06D9  bl 0x831a815c
	ctx.lr = 0x82DF7A88;
	sub_831A8130(ctx, base);
	// 82DF7A88: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF7A8C: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82DF7A90: 3B200001  li r25, 1
	ctx.r[25].s64 = 1;
	// 82DF7A94: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF7A98: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82DF7A9C: 7F3CCB78  mr r28, r25
	ctx.r[28].u64 = ctx.r[25].u64;
	// 82DF7AA0: 83DA0004  lwz r30, 4(r26)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF7AA4: 83BE0004  lwz r29, 4(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF7AA8: 4800002C  b 0x82df7ad4
	pc = 0x82DF7AD4; continue 'dispatch;
	// 82DF7AAC: 389D000C  addi r4, r29, 0xc
	ctx.r[4].s64 = ctx.r[29].s64 + 12;
	// 82DF7AB0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82DF7AB4: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 82DF7AB8: 4BFFB781  bl 0x82df3238
	ctx.lr = 0x82DF7ABC;
	sub_82DF3238(ctx, base);
	// 82DF7ABC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82DF7AC0: 578B063F  clrlwi. r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF7AC4: 4182000C  beq 0x82df7ad0
	if ctx.cr[0].eq {
	pc = 0x82DF7AD0; continue 'dispatch;
	}
	// 82DF7AC8: 83BD0000  lwz r29, 0(r29)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF7ACC: 48000008  b 0x82df7ad4
	pc = 0x82DF7AD4; continue 'dispatch;
	// 82DF7AD0: 83BD0008  lwz r29, 8(r29)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF7AD4: 897D0015  lbz r11, 0x15(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(21 as u32) ) } as u64;
	// 82DF7AD8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF7ADC: 419AFFD0  beq cr6, 0x82df7aac
	if ctx.cr[6].eq {
	pc = 0x82DF7AAC; continue 'dispatch;
	}
	// 82DF7AE0: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 82DF7AE4: 578B063F  clrlwi. r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF7AE8: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 82DF7AEC: 41820048  beq 0x82df7b34
	if ctx.cr[0].eq {
	pc = 0x82DF7B34; continue 'dispatch;
	}
	// 82DF7AF0: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF7AF4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF7AF8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF7AFC: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF7B00: 409A002C  bne cr6, 0x82df7b2c
	if !ctx.cr[6].eq {
	pc = 0x82DF7B2C; continue 'dispatch;
	}
	// 82DF7B04: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DF7B08: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82DF7B0C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82DF7B10: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82DF7B14: 4BFFE8DD  bl 0x82df63f0
	ctx.lr = 0x82DF7B18;
	sub_82DF63F0(ctx, base);
	// 82DF7B18: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DF7B1C: 9B3F0004  stb r25, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[25].u8 ) };
	// 82DF7B20: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF7B24: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF7B28: 48000038  b 0x82df7b60
	pc = 0x82DF7B60; continue 'dispatch;
	// 82DF7B2C: 48025335  bl 0x82e1ce60
	ctx.lr = 0x82DF7B30;
	sub_82E1CE60(ctx, base);
	// 82DF7B30: 83A10050  lwz r29, 0x50(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DF7B34: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82DF7B38: 387D000C  addi r3, r29, 0xc
	ctx.r[3].s64 = ctx.r[29].s64 + 12;
	// 82DF7B3C: 4BFFB6FD  bl 0x82df3238
	ctx.lr = 0x82DF7B40;
	sub_82DF3238(ctx, base);
	// 82DF7B40: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF7B44: 41820010  beq 0x82df7b54
	if ctx.cr[0].eq {
	pc = 0x82DF7B54; continue 'dispatch;
	}
	// 82DF7B48: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82DF7B4C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF7B50: 4BFFFFB8  b 0x82df7b08
	pc = 0x82DF7B08; continue 'dispatch;
	// 82DF7B54: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF7B58: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82DF7B5C: 997F0004  stb r11, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u8 ) };
	// 82DF7B60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF7B64: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82DF7B68: 483B0644  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF7B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF7B70 size=132
    let mut pc: u32 = 0x82DF7B70;
    'dispatch: loop {
        match pc {
            0x82DF7B70 => {
    //   block [0x82DF7B70..0x82DF7BF4)
	// 82DF7B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF7B74: 483B05F5  bl 0x831a8168
	ctx.lr = 0x82DF7B78;
	sub_831A8130(ctx, base);
	// 82DF7B78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF7B7C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DF7B80: 90A100A4  stw r5, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[5].u32 ) };
	// 82DF7B84: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DF7B88: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82DF7B8C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF7B90: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF7B94: 7F055040  cmplw cr6, r5, r10
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DF7B98: 409A0044  bne cr6, 0x82df7bdc
	if !ctx.cr[6].eq {
	pc = 0x82DF7BDC; continue 'dispatch;
	}
	// 82DF7B9C: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF7BA0: 409A003C  bne cr6, 0x82df7bdc
	if !ctx.cr[6].eq {
	pc = 0x82DF7BDC; continue 'dispatch;
	}
	// 82DF7BA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF7BA8: 4BFFE469  bl 0x82df6010
	ctx.lr = 0x82DF7BAC;
	sub_82DF6010(ctx, base);
	// 82DF7BAC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF7BB0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF7BB4: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF7BB8: 48000030  b 0x82df7be8
	pc = 0x82DF7BE8; continue 'dispatch;
	// 82DF7BBC: 386100A4  addi r3, r1, 0xa4
	ctx.r[3].s64 = ctx.r[1].s64 + 164;
	// 82DF7BC0: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82DF7BC4: 48004095  bl 0x82dfbc58
	ctx.lr = 0x82DF7BC8;
	sub_82DFBC58(ctx, base);
	// 82DF7BC8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82DF7BCC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF7BD0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF7BD4: 4BFFF065  bl 0x82df6c38
	ctx.lr = 0x82DF7BD8;
	sub_82DF6C38(ctx, base);
	// 82DF7BD8: 80A100A4  lwz r5, 0xa4(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82DF7BDC: 7F05F040  cmplw cr6, r5, r30
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82DF7BE0: 409AFFDC  bne cr6, 0x82df7bbc
	if !ctx.cr[6].eq {
	pc = 0x82DF7BBC; continue 'dispatch;
	}
	// 82DF7BE4: 90BD0000  stw r5, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82DF7BE8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DF7BEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DF7BF0: 483B05C8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF7BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF7BF8 size=848
    let mut pc: u32 = 0x82DF7BF8;
    'dispatch: loop {
        match pc {
            0x82DF7BF8 => {
    //   block [0x82DF7BF8..0x82DF7F48)
	// 82DF7BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF7BFC: 483B055D  bl 0x831a8158
	ctx.lr = 0x82DF7C00;
	sub_831A8130(ctx, base);
	// 82DF7C00: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF7C04: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DF7C08: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82DF7C0C: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 82DF7C10: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF7C14: 7CB82B78  mr r24, r5
	ctx.r[24].u64 = ctx.r[5].u64;
	// 82DF7C18: 4BFFBFE9  bl 0x82df3c00
	ctx.lr = 0x82DF7C1C;
	sub_82DF3C00(ctx, base);
	// 82DF7C1C: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF7C20: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DF7C24: 409A000C  bne cr6, 0x82df7c30
	if !ctx.cr[6].eq {
	pc = 0x82DF7C30; continue 'dispatch;
	}
	// 82DF7C28: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82DF7C2C: 48000010  b 0x82df7c3c
	pc = 0x82DF7C3C; continue 'dispatch;
	// 82DF7C30: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DF7C34: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 82DF7C38: 7D681670  srawi r8, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 82DF7C3C: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 82DF7C40: 419A02F8  beq cr6, 0x82df7f38
	if ctx.cr[6].eq {
	pc = 0x82DF7F38; continue 'dispatch;
	}
	// 82DF7C44: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DF7C48: 409A000C  bne cr6, 0x82df7c54
	if !ctx.cr[6].eq {
	pc = 0x82DF7C54; continue 'dispatch;
	}
	// 82DF7C4C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF7C50: 48000010  b 0x82df7c60
	pc = 0x82DF7C60; continue 'dispatch;
	// 82DF7C54: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF7C58: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 82DF7C5C: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 82DF7C60: 3D403FFF  lis r10, 0x3fff
	ctx.r[10].s64 = 1073676288;
	// 82DF7C64: 614AFFFF  ori r10, r10, 0xffff
	ctx.r[10].u64 = ctx.r[10].u64 | 65535;
	// 82DF7C68: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82DF7C6C: 7F0BC040  cmplw cr6, r11, r24
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[24].u32, &mut ctx.xer);
	// 82DF7C70: 4098000C  bge cr6, 0x82df7c7c
	if !ctx.cr[6].lt {
	pc = 0x82DF7C7C; continue 'dispatch;
	}
	// 82DF7C74: 482CD4C5  bl 0x830c5138
	ctx.lr = 0x82DF7C78;
	sub_830C5138(ctx, base);
	// 82DF7C78: 480002C0  b 0x82df7f38
	pc = 0x82DF7F38; continue 'dispatch;
	// 82DF7C7C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DF7C80: 409A000C  bne cr6, 0x82df7c8c
	if !ctx.cr[6].eq {
	pc = 0x82DF7C8C; continue 'dispatch;
	}
	// 82DF7C84: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF7C88: 48000010  b 0x82df7c98
	pc = 0x82DF7C98; continue 'dispatch;
	// 82DF7C8C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF7C90: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 82DF7C94: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 82DF7C98: 7D6BC214  add r11, r11, r24
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[24].u64;
	// 82DF7C9C: 7F085840  cmplw cr6, r8, r11
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF7CA0: 40980168  bge cr6, 0x82df7e08
	if !ctx.cr[6].lt {
	pc = 0x82DF7E08; continue 'dispatch;
	}
	// 82DF7CA4: 550BF87E  srwi r11, r8, 1
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DF7CA8: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82DF7CAC: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82DF7CB0: 4098000C  bge cr6, 0x82df7cbc
	if !ctx.cr[6].lt {
	pc = 0x82DF7CBC; continue 'dispatch;
	}
	// 82DF7CB4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF7CB8: 48000008  b 0x82df7cc0
	pc = 0x82DF7CC0; continue 'dispatch;
	// 82DF7CBC: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82DF7CC0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DF7CC4: 409A000C  bne cr6, 0x82df7cd0
	if !ctx.cr[6].eq {
	pc = 0x82DF7CD0; continue 'dispatch;
	}
	// 82DF7CC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DF7CCC: 48000010  b 0x82df7cdc
	pc = 0x82DF7CDC; continue 'dispatch;
	// 82DF7CD0: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF7CD4: 7D495050  subf r10, r9, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 82DF7CD8: 7D4A1670  srawi r10, r10, 2
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 2) as i64;
	// 82DF7CDC: 7D4AC214  add r10, r10, r24
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[24].u64;
	// 82DF7CE0: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DF7CE4: 40980024  bge cr6, 0x82df7d08
	if !ctx.cr[6].lt {
	pc = 0x82DF7D08; continue 'dispatch;
	}
	// 82DF7CE8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DF7CEC: 409A000C  bne cr6, 0x82df7cf8
	if !ctx.cr[6].eq {
	pc = 0x82DF7CF8; continue 'dispatch;
	}
	// 82DF7CF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF7CF4: 48000010  b 0x82df7d04
	pc = 0x82DF7D04; continue 'dispatch;
	// 82DF7CF8: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF7CFC: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 82DF7D00: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 82DF7D04: 7D6BC214  add r11, r11, r24
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[24].u64;
	// 82DF7D08: 5579103A  slwi r25, r11, 2
	ctx.r[25].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[25].u64 = ctx.r[25].u32 as u64;
	// 82DF7D0C: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82DF7D10: 4BFFA199  bl 0x82df1ea8
	ctx.lr = 0x82DF7D14;
	sub_82DF1EA8(ctx, base);
	// 82DF7D14: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82DF7D18: 83FE0004  lwz r31, 4(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF7D1C: 7F7DDB78  mr r29, r27
	ctx.r[29].u64 = ctx.r[27].u64;
	// 82DF7D20: 48000020  b 0x82df7d40
	pc = 0x82DF7D40; continue 'dispatch;
	// 82DF7D24: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82DF7D28: 419A0010  beq cr6, 0x82df7d38
	if ctx.cr[6].eq {
	pc = 0x82DF7D38; continue 'dispatch;
	}
	// 82DF7D2C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF7D30: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DF7D34: 4BFFBECD  bl 0x82df3c00
	ctx.lr = 0x82DF7D38;
	sub_82DF3C00(ctx, base);
	// 82DF7D38: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82DF7D3C: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82DF7D40: 7F1FD040  cmplw cr6, r31, r26
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82DF7D44: 409AFFE0  bne cr6, 0x82df7d24
	if !ctx.cr[6].eq {
	pc = 0x82DF7D24; continue 'dispatch;
	}
	// 82DF7D48: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82DF7D4C: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 82DF7D50: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DF7D54: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF7D58: 4BFFE209  bl 0x82df5f60
	ctx.lr = 0x82DF7D5C;
	sub_82DF5F60(ctx, base);
	// 82DF7D5C: 83BE0008  lwz r29, 8(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF7D60: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF7D64: 7F1AE840  cmplw cr6, r26, r29
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82DF7D68: 419A002C  beq cr6, 0x82df7d94
	if ctx.cr[6].eq {
	pc = 0x82DF7D94; continue 'dispatch;
	}
	// 82DF7D6C: 7F9FD050  subf r28, r31, r26
	ctx.r[28].s64 = ctx.r[26].s64 - ctx.r[31].s64;
	// 82DF7D70: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82DF7D74: 419A0010  beq cr6, 0x82df7d84
	if ctx.cr[6].eq {
	pc = 0x82DF7D84; continue 'dispatch;
	}
	// 82DF7D78: 7C9CFA14  add r4, r28, r31
	ctx.r[4].u64 = ctx.r[28].u64 + ctx.r[31].u64;
	// 82DF7D7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF7D80: 4BFFBE81  bl 0x82df3c00
	ctx.lr = 0x82DF7D84;
	sub_82DF3C00(ctx, base);
	// 82DF7D84: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82DF7D88: 7D7CFA14  add r11, r28, r31
	ctx.r[11].u64 = ctx.r[28].u64 + ctx.r[31].u64;
	// 82DF7D8C: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82DF7D90: 409AFFE0  bne cr6, 0x82df7d70
	if !ctx.cr[6].eq {
	pc = 0x82DF7D70; continue 'dispatch;
	}
	// 82DF7D94: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF7D98: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF7D9C: 409A000C  bne cr6, 0x82df7da8
	if !ctx.cr[6].eq {
	pc = 0x82DF7DA8; continue 'dispatch;
	}
	// 82DF7DA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DF7DA4: 48000010  b 0x82df7db4
	pc = 0x82DF7DB4; continue 'dispatch;
	// 82DF7DA8: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF7DAC: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82DF7DB0: 7D4A1670  srawi r10, r10, 2
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 2) as i64;
	// 82DF7DB4: 7F8AC214  add r28, r10, r24
	ctx.r[28].u64 = ctx.r[10].u64 + ctx.r[24].u64;
	// 82DF7DB8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF7DBC: 419A0030  beq cr6, 0x82df7dec
	if ctx.cr[6].eq {
	pc = 0x82DF7DEC; continue 'dispatch;
	}
	// 82DF7DC0: 83BE0008  lwz r29, 8(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF7DC4: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 82DF7DC8: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82DF7DCC: 419A0018  beq cr6, 0x82df7de4
	if ctx.cr[6].eq {
	pc = 0x82DF7DE4; continue 'dispatch;
	}
	// 82DF7DD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF7DD4: 4BFFB655  bl 0x82df3428
	ctx.lr = 0x82DF7DD8;
	sub_82DF3428(ctx, base);
	// 82DF7DD8: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82DF7DDC: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82DF7DE0: 409AFFF0  bne cr6, 0x82df7dd0
	if !ctx.cr[6].eq {
	pc = 0x82DF7DD0; continue 'dispatch;
	}
	// 82DF7DE4: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF7DE8: 4BFFA0C9  bl 0x82df1eb0
	ctx.lr = 0x82DF7DEC;
	sub_82DF1EB0(ctx, base);
	// 82DF7DEC: 578B103A  slwi r11, r28, 2
	ctx.r[11].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DF7DF0: 937E0004  stw r27, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 82DF7DF4: 7D59DA14  add r10, r25, r27
	ctx.r[10].u64 = ctx.r[25].u64 + ctx.r[27].u64;
	// 82DF7DF8: 7D6BDA14  add r11, r11, r27
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 82DF7DFC: 915E000C  stw r10, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82DF7E00: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DF7E04: 48000134  b 0x82df7f38
	pc = 0x82DF7F38; continue 'dispatch;
	// 82DF7E08: 83FE0008  lwz r31, 8(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF7E0C: 7D7AF850  subf r11, r26, r31
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[26].s64;
	// 82DF7E10: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 82DF7E14: 7F0BC040  cmplw cr6, r11, r24
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[24].u32, &mut ctx.xer);
	// 82DF7E18: 40980090  bge cr6, 0x82df7ea8
	if !ctx.cr[6].lt {
	pc = 0x82DF7EA8; continue 'dispatch;
	}
	// 82DF7E1C: 571D103A  slwi r29, r24, 2
	ctx.r[29].u32 = ctx.r[24].u32.wrapping_shl(2);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82DF7E20: 7F1AF840  cmplw cr6, r26, r31
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82DF7E24: 7F9DD214  add r28, r29, r26
	ctx.r[28].u64 = ctx.r[29].u64 + ctx.r[26].u64;
	// 82DF7E28: 419A002C  beq cr6, 0x82df7e54
	if ctx.cr[6].eq {
	pc = 0x82DF7E54; continue 'dispatch;
	}
	// 82DF7E2C: 7F7DE050  subf r27, r29, r28
	ctx.r[27].s64 = ctx.r[28].s64 - ctx.r[29].s64;
	// 82DF7E30: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82DF7E34: 419A0010  beq cr6, 0x82df7e44
	if ctx.cr[6].eq {
	pc = 0x82DF7E44; continue 'dispatch;
	}
	// 82DF7E38: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82DF7E3C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DF7E40: 4BFFBDC1  bl 0x82df3c00
	ctx.lr = 0x82DF7E44;
	sub_82DF3C00(ctx, base);
	// 82DF7E44: 3B7B0004  addi r27, r27, 4
	ctx.r[27].s64 = ctx.r[27].s64 + 4;
	// 82DF7E48: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 82DF7E4C: 7F1BF840  cmplw cr6, r27, r31
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82DF7E50: 409AFFE0  bne cr6, 0x82df7e30
	if !ctx.cr[6].eq {
	pc = 0x82DF7E30; continue 'dispatch;
	}
	// 82DF7E54: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF7E58: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82DF7E5C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF7E60: 7D7A2050  subf r11, r26, r4
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[26].s64;
	// 82DF7E64: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 82DF7E68: 7CABC050  subf r5, r11, r24
	ctx.r[5].s64 = ctx.r[24].s64 - ctx.r[11].s64;
	// 82DF7E6C: 4BFFE0F5  bl 0x82df5f60
	ctx.lr = 0x82DF7E70;
	sub_82DF5F60(ctx, base);
	// 82DF7E70: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF7E74: 7F5CD378  mr r28, r26
	ctx.r[28].u64 = ctx.r[26].u64;
	// 82DF7E78: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82DF7E7C: 7FFD5850  subf r31, r29, r11
	ctx.r[31].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 82DF7E80: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DF7E84: 7F1AF840  cmplw cr6, r26, r31
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82DF7E88: 419A00B0  beq cr6, 0x82df7f38
	if ctx.cr[6].eq {
	pc = 0x82DF7F38; continue 'dispatch;
	}
	// 82DF7E8C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DF7E90: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DF7E94: 4BFFBD3D  bl 0x82df3bd0
	ctx.lr = 0x82DF7E98;
	sub_82DF3BD0(ctx, base);
	// 82DF7E98: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 82DF7E9C: 7F1CF840  cmplw cr6, r28, r31
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82DF7EA0: 409AFFEC  bne cr6, 0x82df7e8c
	if !ctx.cr[6].eq {
	pc = 0x82DF7E8C; continue 'dispatch;
	}
	// 82DF7EA4: 48000094  b 0x82df7f38
	pc = 0x82DF7F38; continue 'dispatch;
	// 82DF7EA8: 5719103A  slwi r25, r24, 2
	ctx.r[25].u32 = ctx.r[24].u32.wrapping_shl(2);
	ctx.r[25].u64 = ctx.r[25].u32 as u64;
	// 82DF7EAC: 7FFCFB78  mr r28, r31
	ctx.r[28].u64 = ctx.r[31].u64;
	// 82DF7EB0: 7FB9F850  subf r29, r25, r31
	ctx.r[29].s64 = ctx.r[31].s64 - ctx.r[25].s64;
	// 82DF7EB4: 7FBBEB78  mr r27, r29
	ctx.r[27].u64 = ctx.r[29].u64;
	// 82DF7EB8: 7F1DF840  cmplw cr6, r29, r31
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82DF7EBC: 419A0028  beq cr6, 0x82df7ee4
	if ctx.cr[6].eq {
	pc = 0x82DF7EE4; continue 'dispatch;
	}
	// 82DF7EC0: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82DF7EC4: 419A0010  beq cr6, 0x82df7ed4
	if ctx.cr[6].eq {
	pc = 0x82DF7ED4; continue 'dispatch;
	}
	// 82DF7EC8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82DF7ECC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DF7ED0: 4BFFBD31  bl 0x82df3c00
	ctx.lr = 0x82DF7ED4;
	sub_82DF3C00(ctx, base);
	// 82DF7ED4: 3B7B0004  addi r27, r27, 4
	ctx.r[27].s64 = ctx.r[27].s64 + 4;
	// 82DF7ED8: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 82DF7EDC: 7F1BF840  cmplw cr6, r27, r31
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82DF7EE0: 409AFFE0  bne cr6, 0x82df7ec0
	if !ctx.cr[6].eq {
	pc = 0x82DF7EC0; continue 'dispatch;
	}
	// 82DF7EE4: 939E0008  stw r28, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82DF7EE8: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 82DF7EEC: 7F1AE840  cmplw cr6, r26, r29
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82DF7EF0: 419A0020  beq cr6, 0x82df7f10
	if ctx.cr[6].eq {
	pc = 0x82DF7F10; continue 'dispatch;
	}
	// 82DF7EF4: 7FFDF850  subf r31, r29, r31
	ctx.r[31].s64 = ctx.r[31].s64 - ctx.r[29].s64;
	// 82DF7EF8: 3BDEFFFC  addi r30, r30, -4
	ctx.r[30].s64 = ctx.r[30].s64 + -4;
	// 82DF7EFC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DF7F00: 7C7FF214  add r3, r31, r30
	ctx.r[3].u64 = ctx.r[31].u64 + ctx.r[30].u64;
	// 82DF7F04: 4BFFBCCD  bl 0x82df3bd0
	ctx.lr = 0x82DF7F08;
	sub_82DF3BD0(ctx, base);
	// 82DF7F08: 7F1ED040  cmplw cr6, r30, r26
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82DF7F0C: 409AFFEC  bne cr6, 0x82df7ef8
	if !ctx.cr[6].eq {
	pc = 0x82DF7EF8; continue 'dispatch;
	}
	// 82DF7F10: 7FD9D214  add r30, r25, r26
	ctx.r[30].u64 = ctx.r[25].u64 + ctx.r[26].u64;
	// 82DF7F14: 7F5FD378  mr r31, r26
	ctx.r[31].u64 = ctx.r[26].u64;
	// 82DF7F18: 7F1AF040  cmplw cr6, r26, r30
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82DF7F1C: 419A001C  beq cr6, 0x82df7f38
	if ctx.cr[6].eq {
	pc = 0x82DF7F38; continue 'dispatch;
	}
	// 82DF7F20: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DF7F24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF7F28: 4BFFBCA9  bl 0x82df3bd0
	ctx.lr = 0x82DF7F2C;
	sub_82DF3BD0(ctx, base);
	// 82DF7F2C: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82DF7F30: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82DF7F34: 409AFFEC  bne cr6, 0x82df7f20
	if !ctx.cr[6].eq {
	pc = 0x82DF7F20; continue 'dispatch;
	}
	// 82DF7F38: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF7F3C: 4BFFB4ED  bl 0x82df3428
	ctx.lr = 0x82DF7F40;
	sub_82DF3428(ctx, base);
	// 82DF7F40: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82DF7F44: 483B0264  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF7F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF7F48 size=236
    let mut pc: u32 = 0x82DF7F48;
    'dispatch: loop {
        match pc {
            0x82DF7F48 => {
    //   block [0x82DF7F48..0x82DF8034)
	// 82DF7F48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF7F4C: 483B0211  bl 0x831a815c
	ctx.lr = 0x82DF7F50;
	sub_831A8130(ctx, base);
	// 82DF7F50: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF7F54: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82DF7F58: 3B200001  li r25, 1
	ctx.r[25].s64 = 1;
	// 82DF7F5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF7F60: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82DF7F64: 7F3CCB78  mr r28, r25
	ctx.r[28].u64 = ctx.r[25].u64;
	// 82DF7F68: 83DA0004  lwz r30, 4(r26)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF7F6C: 83BE0004  lwz r29, 4(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF7F70: 4800002C  b 0x82df7f9c
	pc = 0x82DF7F9C; continue 'dispatch;
	// 82DF7F74: 389D000C  addi r4, r29, 0xc
	ctx.r[4].s64 = ctx.r[29].s64 + 12;
	// 82DF7F78: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82DF7F7C: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 82DF7F80: 4BFFDC91  bl 0x82df5c10
	ctx.lr = 0x82DF7F84;
	sub_82DF5C10(ctx, base);
	// 82DF7F84: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82DF7F88: 578B063F  clrlwi. r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF7F8C: 4182000C  beq 0x82df7f98
	if ctx.cr[0].eq {
	pc = 0x82DF7F98; continue 'dispatch;
	}
	// 82DF7F90: 83BD0000  lwz r29, 0(r29)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF7F94: 48000008  b 0x82df7f9c
	pc = 0x82DF7F9C; continue 'dispatch;
	// 82DF7F98: 83BD0008  lwz r29, 8(r29)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF7F9C: 897D002D  lbz r11, 0x2d(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(45 as u32) ) } as u64;
	// 82DF7FA0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF7FA4: 419AFFD0  beq cr6, 0x82df7f74
	if ctx.cr[6].eq {
	pc = 0x82DF7F74; continue 'dispatch;
	}
	// 82DF7FA8: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 82DF7FAC: 578B063F  clrlwi. r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF7FB0: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 82DF7FB4: 41820048  beq 0x82df7ffc
	if ctx.cr[0].eq {
	pc = 0x82DF7FFC; continue 'dispatch;
	}
	// 82DF7FB8: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF7FBC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF7FC0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF7FC4: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF7FC8: 409A002C  bne cr6, 0x82df7ff4
	if !ctx.cr[6].eq {
	pc = 0x82DF7FF4; continue 'dispatch;
	}
	// 82DF7FCC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DF7FD0: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82DF7FD4: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82DF7FD8: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82DF7FDC: 4BFFE63D  bl 0x82df6618
	ctx.lr = 0x82DF7FE0;
	sub_82DF6618(ctx, base);
	// 82DF7FE0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DF7FE4: 9B3F0004  stb r25, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[25].u8 ) };
	// 82DF7FE8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF7FEC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF7FF0: 48000038  b 0x82df8028
	pc = 0x82DF8028; continue 'dispatch;
	// 82DF7FF4: 4801F905  bl 0x82e178f8
	ctx.lr = 0x82DF7FF8;
	sub_82E178F8(ctx, base);
	// 82DF7FF8: 83A10050  lwz r29, 0x50(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DF7FFC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82DF8000: 387D000C  addi r3, r29, 0xc
	ctx.r[3].s64 = ctx.r[29].s64 + 12;
	// 82DF8004: 4BFFDC0D  bl 0x82df5c10
	ctx.lr = 0x82DF8008;
	sub_82DF5C10(ctx, base);
	// 82DF8008: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF800C: 41820010  beq 0x82df801c
	if ctx.cr[0].eq {
	pc = 0x82DF801C; continue 'dispatch;
	}
	// 82DF8010: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82DF8014: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF8018: 4BFFFFB8  b 0x82df7fd0
	pc = 0x82DF7FD0; continue 'dispatch;
	// 82DF801C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF8020: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82DF8024: 997F0004  stb r11, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u8 ) };
	// 82DF8028: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF802C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82DF8030: 483B017C  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF8038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF8038 size=84
    let mut pc: u32 = 0x82DF8038;
    'dispatch: loop {
        match pc {
            0x82DF8038 => {
    //   block [0x82DF8038..0x82DF808C)
	// 82DF8038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF803C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF8040: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF8044: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF8048: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF804C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF8050: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF8054: 4BFFF3DD  bl 0x82df7430
	ctx.lr = 0x82DF8058;
	sub_82DF7430(ctx, base);
	// 82DF8058: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF805C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF8060: 914A0004  stw r10, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DF8064: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DF8068: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF806C: 914A0000  stw r10, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DF8070: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF8074: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DF8078: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF807C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF8080: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF8084: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF8088: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF8090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF8090 size=424
    let mut pc: u32 = 0x82DF8090;
    'dispatch: loop {
        match pc {
            0x82DF8090 => {
    //   block [0x82DF8090..0x82DF8238)
	// 82DF8090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF8094: 483B00CD  bl 0x831a8160
	ctx.lr = 0x82DF8098;
	sub_831A8130(ctx, base);
	// 82DF8098: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF809C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DF80A0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DF80A4: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82DF80A8: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82DF80AC: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF80B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF80B4: 409A0020  bne cr6, 0x82df80d4
	if !ctx.cr[6].eq {
	pc = 0x82DF80D4; continue 'dispatch;
	}
	// 82DF80B8: 80DD0004  lwz r6, 4(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF80BC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DF80C0: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 82DF80C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF80C8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DF80CC: 4BFFE0FD  bl 0x82df61c8
	ctx.lr = 0x82DF80D0;
	sub_82DF61C8(ctx, base);
	// 82DF80D0: 4800015C  b 0x82df822c
	pc = 0x82DF822C; continue 'dispatch;
	// 82DF80D4: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF80D8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF80DC: 7F1C5040  cmplw cr6, r28, r10
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DF80E0: 409A0020  bne cr6, 0x82df8100
	if !ctx.cr[6].eq {
	pc = 0x82DF8100; continue 'dispatch;
	}
	// 82DF80E4: 389C000C  addi r4, r28, 0xc
	ctx.r[4].s64 = ctx.r[28].s64 + 12;
	// 82DF80E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF80EC: 4BFFB14D  bl 0x82df3238
	ctx.lr = 0x82DF80F0;
	sub_82DF3238(ctx, base);
	// 82DF80F0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF80F4: 4182011C  beq 0x82df8210
	if ctx.cr[0].eq {
	pc = 0x82DF8210; continue 'dispatch;
	}
	// 82DF80F8: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DF80FC: 4BFFFFC0  b 0x82df80bc
	pc = 0x82DF80BC; continue 'dispatch;
	// 82DF8100: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF8104: 409A002C  bne cr6, 0x82df8130
	if !ctx.cr[6].eq {
	pc = 0x82DF8130; continue 'dispatch;
	}
	// 82DF8108: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF810C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF8110: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 82DF8114: 4BFFB125  bl 0x82df3238
	ctx.lr = 0x82DF8118;
	sub_82DF3238(ctx, base);
	// 82DF8118: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF811C: 418200F4  beq 0x82df8210
	if ctx.cr[0].eq {
	pc = 0x82DF8210; continue 'dispatch;
	}
	// 82DF8120: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF8124: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF8128: 80CB0008  lwz r6, 8(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF812C: 4BFFFF94  b 0x82df80c0
	pc = 0x82DF80C0; continue 'dispatch;
	// 82DF8130: 3B5C000C  addi r26, r28, 0xc
	ctx.r[26].s64 = ctx.r[28].s64 + 12;
	// 82DF8134: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF8138: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82DF813C: 4BFFB0FD  bl 0x82df3238
	ctx.lr = 0x82DF8140;
	sub_82DF3238(ctx, base);
	// 82DF8140: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF8144: 4182005C  beq 0x82df81a0
	if ctx.cr[0].eq {
	pc = 0x82DF81A0; continue 'dispatch;
	}
	// 82DF8148: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 82DF814C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF8150: 48024D11  bl 0x82e1ce60
	ctx.lr = 0x82DF8154;
	sub_82E1CE60(ctx, base);
	// 82DF8154: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF8158: 83610050  lwz r27, 0x50(r1)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DF815C: 387B000C  addi r3, r27, 0xc
	ctx.r[3].s64 = ctx.r[27].s64 + 12;
	// 82DF8160: 4BFFB0D9  bl 0x82df3238
	ctx.lr = 0x82DF8164;
	sub_82DF3238(ctx, base);
	// 82DF8164: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF8168: 41820038  beq 0x82df81a0
	if ctx.cr[0].eq {
	pc = 0x82DF81A0; continue 'dispatch;
	}
	// 82DF816C: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF8170: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 82DF8174: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DF8178: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF817C: 896B0015  lbz r11, 0x15(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 82DF8180: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF8184: 419A0010  beq cr6, 0x82df8194
	if ctx.cr[6].eq {
	pc = 0x82DF8194; continue 'dispatch;
	}
	// 82DF8188: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82DF818C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF8190: 4BFFFF3C  b 0x82df80cc
	pc = 0x82DF80CC; continue 'dispatch;
	// 82DF8194: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DF8198: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DF819C: 4BFFFF30  b 0x82df80cc
	pc = 0x82DF80CC; continue 'dispatch;
	// 82DF81A0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF81A4: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82DF81A8: 4BFFB091  bl 0x82df3238
	ctx.lr = 0x82DF81AC;
	sub_82DF3238(ctx, base);
	// 82DF81AC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF81B0: 41820060  beq 0x82df8210
	if ctx.cr[0].eq {
	pc = 0x82DF8210; continue 'dispatch;
	}
	// 82DF81B4: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 82DF81B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF81BC: 48003A9D  bl 0x82dfbc58
	ctx.lr = 0x82DF81C0;
	sub_82DFBC58(ctx, base);
	// 82DF81C0: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF81C4: 83610050  lwz r27, 0x50(r1)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DF81C8: 7F1B5840  cmplw cr6, r27, r11
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF81CC: 419A0018  beq cr6, 0x82df81e4
	if ctx.cr[6].eq {
	pc = 0x82DF81E4; continue 'dispatch;
	}
	// 82DF81D0: 389B000C  addi r4, r27, 0xc
	ctx.r[4].s64 = ctx.r[27].s64 + 12;
	// 82DF81D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF81D8: 4BFFB061  bl 0x82df3238
	ctx.lr = 0x82DF81DC;
	sub_82DF3238(ctx, base);
	// 82DF81DC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF81E0: 41820030  beq 0x82df8210
	if ctx.cr[0].eq {
	pc = 0x82DF8210; continue 'dispatch;
	}
	// 82DF81E4: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF81E8: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 82DF81EC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DF81F0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF81F4: 896B0015  lbz r11, 0x15(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 82DF81F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF81FC: 419A000C  beq cr6, 0x82df8208
	if ctx.cr[6].eq {
	pc = 0x82DF8208; continue 'dispatch;
	}
	// 82DF8200: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DF8204: 4BFFFF88  b 0x82df818c
	pc = 0x82DF818C; continue 'dispatch;
	// 82DF8208: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82DF820C: 4BFFFF8C  b 0x82df8198
	pc = 0x82DF8198; continue 'dispatch;
	// 82DF8210: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DF8214: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DF8218: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF821C: 4BFFF39D  bl 0x82df75b8
	ctx.lr = 0x82DF8220;
	sub_82DF75B8(ctx, base);
	// 82DF8220: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DF8224: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF8228: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF822C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF8230: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DF8234: 483AFF7C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF8238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF8238 size=80
    let mut pc: u32 = 0x82DF8238;
    'dispatch: loop {
        match pc {
            0x82DF8238 => {
    //   block [0x82DF8238..0x82DF8288)
	// 82DF8238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF823C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF8240: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF8244: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF8248: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF824C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF8250: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF8254: 80DF0004  lwz r6, 4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF8258: 80A60000  lwz r5, 0(r6)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF825C: 4BFFF44D  bl 0x82df76a8
	ctx.lr = 0x82DF8260;
	sub_82DF76A8(ctx, base);
	// 82DF8260: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF8264: 4B4C8005  bl 0x822c0268
	ctx.lr = 0x82DF8268;
	sub_822C0268(ctx, base);
	// 82DF8268: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF826C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DF8270: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DF8274: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF8278: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF827C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF8280: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF8284: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF8288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF8288 size=108
    let mut pc: u32 = 0x82DF8288;
    'dispatch: loop {
        match pc {
            0x82DF8288 => {
    //   block [0x82DF8288..0x82DF82F4)
	// 82DF8288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF828C: 483AFEE1  bl 0x831a816c
	ctx.lr = 0x82DF8290;
	sub_831A8130(ctx, base);
	// 82DF8290: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF8294: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DF8298: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DF829C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DF82A0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF82A4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF82A8: 419A0014  beq cr6, 0x82df82bc
	if ctx.cr[6].eq {
	pc = 0x82DF82BC; continue 'dispatch;
	}
	// 82DF82AC: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF82B0: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82DF82B4: 7D4A1671  srawi. r10, r10, 2
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 2) as i64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DF82B8: 4082000C  bne 0x82df82c4
	if !ctx.cr[0].eq {
	pc = 0x82DF82C4; continue 'dispatch;
	}
	// 82DF82BC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82DF82C0: 4800000C  b 0x82df82cc
	pc = 0x82DF82CC; continue 'dispatch;
	// 82DF82C4: 7D6B2050  subf r11, r11, r4
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[11].s64;
	// 82DF82C8: 7D7D1670  srawi r29, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[29].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 82DF82CC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DF82D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF82D4: 4BFFF45D  bl 0x82df7730
	ctx.lr = 0x82DF82D8;
	sub_82DF7730(ctx, base);
	// 82DF82D8: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF82DC: 57AB103A  slwi r11, r29, 2
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DF82E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF82E4: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DF82E8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF82EC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF82F0: 483AFECC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF82F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF82F8 size=424
    let mut pc: u32 = 0x82DF82F8;
    'dispatch: loop {
        match pc {
            0x82DF82F8 => {
    //   block [0x82DF82F8..0x82DF84A0)
	// 82DF82F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF82FC: 483AFE65  bl 0x831a8160
	ctx.lr = 0x82DF8300;
	sub_831A8130(ctx, base);
	// 82DF8300: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF8304: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DF8308: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DF830C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82DF8310: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82DF8314: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF8318: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF831C: 409A0020  bne cr6, 0x82df833c
	if !ctx.cr[6].eq {
	pc = 0x82DF833C; continue 'dispatch;
	}
	// 82DF8320: 80DD0004  lwz r6, 4(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF8324: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DF8328: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 82DF832C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF8330: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DF8334: 4BFFE0BD  bl 0x82df63f0
	ctx.lr = 0x82DF8338;
	sub_82DF63F0(ctx, base);
	// 82DF8338: 4800015C  b 0x82df8494
	pc = 0x82DF8494; continue 'dispatch;
	// 82DF833C: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF8340: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF8344: 7F1C5040  cmplw cr6, r28, r10
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DF8348: 409A0020  bne cr6, 0x82df8368
	if !ctx.cr[6].eq {
	pc = 0x82DF8368; continue 'dispatch;
	}
	// 82DF834C: 389C000C  addi r4, r28, 0xc
	ctx.r[4].s64 = ctx.r[28].s64 + 12;
	// 82DF8350: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF8354: 4BFFAEE5  bl 0x82df3238
	ctx.lr = 0x82DF8358;
	sub_82DF3238(ctx, base);
	// 82DF8358: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF835C: 4182011C  beq 0x82df8478
	if ctx.cr[0].eq {
	pc = 0x82DF8478; continue 'dispatch;
	}
	// 82DF8360: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DF8364: 4BFFFFC0  b 0x82df8324
	pc = 0x82DF8324; continue 'dispatch;
	// 82DF8368: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF836C: 409A002C  bne cr6, 0x82df8398
	if !ctx.cr[6].eq {
	pc = 0x82DF8398; continue 'dispatch;
	}
	// 82DF8370: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF8374: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF8378: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 82DF837C: 4BFFAEBD  bl 0x82df3238
	ctx.lr = 0x82DF8380;
	sub_82DF3238(ctx, base);
	// 82DF8380: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF8384: 418200F4  beq 0x82df8478
	if ctx.cr[0].eq {
	pc = 0x82DF8478; continue 'dispatch;
	}
	// 82DF8388: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF838C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF8390: 80CB0008  lwz r6, 8(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF8394: 4BFFFF94  b 0x82df8328
	pc = 0x82DF8328; continue 'dispatch;
	// 82DF8398: 3B5C000C  addi r26, r28, 0xc
	ctx.r[26].s64 = ctx.r[28].s64 + 12;
	// 82DF839C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF83A0: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82DF83A4: 4BFFAE95  bl 0x82df3238
	ctx.lr = 0x82DF83A8;
	sub_82DF3238(ctx, base);
	// 82DF83A8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF83AC: 4182005C  beq 0x82df8408
	if ctx.cr[0].eq {
	pc = 0x82DF8408; continue 'dispatch;
	}
	// 82DF83B0: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 82DF83B4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF83B8: 48024AA9  bl 0x82e1ce60
	ctx.lr = 0x82DF83BC;
	sub_82E1CE60(ctx, base);
	// 82DF83BC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF83C0: 83610050  lwz r27, 0x50(r1)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DF83C4: 387B000C  addi r3, r27, 0xc
	ctx.r[3].s64 = ctx.r[27].s64 + 12;
	// 82DF83C8: 4BFFAE71  bl 0x82df3238
	ctx.lr = 0x82DF83CC;
	sub_82DF3238(ctx, base);
	// 82DF83CC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF83D0: 41820038  beq 0x82df8408
	if ctx.cr[0].eq {
	pc = 0x82DF8408; continue 'dispatch;
	}
	// 82DF83D4: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF83D8: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 82DF83DC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DF83E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF83E4: 896B0015  lbz r11, 0x15(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 82DF83E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF83EC: 419A0010  beq cr6, 0x82df83fc
	if ctx.cr[6].eq {
	pc = 0x82DF83FC; continue 'dispatch;
	}
	// 82DF83F0: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82DF83F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF83F8: 4BFFFF3C  b 0x82df8334
	pc = 0x82DF8334; continue 'dispatch;
	// 82DF83FC: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DF8400: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DF8404: 4BFFFF30  b 0x82df8334
	pc = 0x82DF8334; continue 'dispatch;
	// 82DF8408: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF840C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82DF8410: 4BFFAE29  bl 0x82df3238
	ctx.lr = 0x82DF8414;
	sub_82DF3238(ctx, base);
	// 82DF8414: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF8418: 41820060  beq 0x82df8478
	if ctx.cr[0].eq {
	pc = 0x82DF8478; continue 'dispatch;
	}
	// 82DF841C: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 82DF8420: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF8424: 48003835  bl 0x82dfbc58
	ctx.lr = 0x82DF8428;
	sub_82DFBC58(ctx, base);
	// 82DF8428: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF842C: 83610050  lwz r27, 0x50(r1)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DF8430: 7F1B5840  cmplw cr6, r27, r11
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF8434: 419A0018  beq cr6, 0x82df844c
	if ctx.cr[6].eq {
	pc = 0x82DF844C; continue 'dispatch;
	}
	// 82DF8438: 389B000C  addi r4, r27, 0xc
	ctx.r[4].s64 = ctx.r[27].s64 + 12;
	// 82DF843C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF8440: 4BFFADF9  bl 0x82df3238
	ctx.lr = 0x82DF8444;
	sub_82DF3238(ctx, base);
	// 82DF8444: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF8448: 41820030  beq 0x82df8478
	if ctx.cr[0].eq {
	pc = 0x82DF8478; continue 'dispatch;
	}
	// 82DF844C: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF8450: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 82DF8454: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DF8458: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF845C: 896B0015  lbz r11, 0x15(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 82DF8460: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF8464: 419A000C  beq cr6, 0x82df8470
	if ctx.cr[6].eq {
	pc = 0x82DF8470; continue 'dispatch;
	}
	// 82DF8468: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DF846C: 4BFFFF88  b 0x82df83f4
	pc = 0x82DF83F4; continue 'dispatch;
	// 82DF8470: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82DF8474: 4BFFFF8C  b 0x82df8400
	pc = 0x82DF8400; continue 'dispatch;
	// 82DF8478: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DF847C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DF8480: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF8484: 4BFFF5FD  bl 0x82df7a80
	ctx.lr = 0x82DF8488;
	sub_82DF7A80(ctx, base);
	// 82DF8488: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DF848C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF8490: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF8494: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF8498: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DF849C: 483AFD14  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF84A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF84A0 size=80
    let mut pc: u32 = 0x82DF84A0;
    'dispatch: loop {
        match pc {
            0x82DF84A0 => {
    //   block [0x82DF84A0..0x82DF84F0)
	// 82DF84A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF84A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF84A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF84AC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF84B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF84B4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF84B8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF84BC: 80DF0004  lwz r6, 4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF84C0: 80A60000  lwz r5, 0(r6)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF84C4: 4BFFF6AD  bl 0x82df7b70
	ctx.lr = 0x82DF84C8;
	sub_82DF7B70(ctx, base);
	// 82DF84C8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF84CC: 4BFF99E5  bl 0x82df1eb0
	ctx.lr = 0x82DF84D0;
	sub_82DF1EB0(ctx, base);
	// 82DF84D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF84D4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DF84D8: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DF84DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF84E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF84E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF84E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF84EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF84F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF84F0 size=108
    let mut pc: u32 = 0x82DF84F0;
    'dispatch: loop {
        match pc {
            0x82DF84F0 => {
    //   block [0x82DF84F0..0x82DF855C)
	// 82DF84F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF84F4: 483AFC79  bl 0x831a816c
	ctx.lr = 0x82DF84F8;
	sub_831A8130(ctx, base);
	// 82DF84F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF84FC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DF8500: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DF8504: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DF8508: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF850C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF8510: 419A0014  beq cr6, 0x82df8524
	if ctx.cr[6].eq {
	pc = 0x82DF8524; continue 'dispatch;
	}
	// 82DF8514: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF8518: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82DF851C: 7D4A1671  srawi. r10, r10, 2
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 2) as i64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DF8520: 4082000C  bne 0x82df852c
	if !ctx.cr[0].eq {
	pc = 0x82DF852C; continue 'dispatch;
	}
	// 82DF8524: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82DF8528: 4800000C  b 0x82df8534
	pc = 0x82DF8534; continue 'dispatch;
	// 82DF852C: 7D6B2050  subf r11, r11, r4
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[11].s64;
	// 82DF8530: 7D7D1670  srawi r29, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[29].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 82DF8534: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DF8538: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF853C: 4BFFF6BD  bl 0x82df7bf8
	ctx.lr = 0x82DF8540;
	sub_82DF7BF8(ctx, base);
	// 82DF8540: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF8544: 57AB103A  slwi r11, r29, 2
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DF8548: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF854C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DF8550: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF8554: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF8558: 483AFC64  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF8560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF8560 size=424
    let mut pc: u32 = 0x82DF8560;
    'dispatch: loop {
        match pc {
            0x82DF8560 => {
    //   block [0x82DF8560..0x82DF8708)
	// 82DF8560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF8564: 483AFBFD  bl 0x831a8160
	ctx.lr = 0x82DF8568;
	sub_831A8130(ctx, base);
	// 82DF8568: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF856C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DF8570: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DF8574: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82DF8578: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82DF857C: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF8580: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF8584: 409A0020  bne cr6, 0x82df85a4
	if !ctx.cr[6].eq {
	pc = 0x82DF85A4; continue 'dispatch;
	}
	// 82DF8588: 80DD0004  lwz r6, 4(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF858C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DF8590: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 82DF8594: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF8598: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DF859C: 4BFFE07D  bl 0x82df6618
	ctx.lr = 0x82DF85A0;
	sub_82DF6618(ctx, base);
	// 82DF85A0: 4800015C  b 0x82df86fc
	pc = 0x82DF86FC; continue 'dispatch;
	// 82DF85A4: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF85A8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF85AC: 7F1C5040  cmplw cr6, r28, r10
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DF85B0: 409A0020  bne cr6, 0x82df85d0
	if !ctx.cr[6].eq {
	pc = 0x82DF85D0; continue 'dispatch;
	}
	// 82DF85B4: 389C000C  addi r4, r28, 0xc
	ctx.r[4].s64 = ctx.r[28].s64 + 12;
	// 82DF85B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF85BC: 4BFFD655  bl 0x82df5c10
	ctx.lr = 0x82DF85C0;
	sub_82DF5C10(ctx, base);
	// 82DF85C0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF85C4: 4182011C  beq 0x82df86e0
	if ctx.cr[0].eq {
	pc = 0x82DF86E0; continue 'dispatch;
	}
	// 82DF85C8: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DF85CC: 4BFFFFC0  b 0x82df858c
	pc = 0x82DF858C; continue 'dispatch;
	// 82DF85D0: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF85D4: 409A002C  bne cr6, 0x82df8600
	if !ctx.cr[6].eq {
	pc = 0x82DF8600; continue 'dispatch;
	}
	// 82DF85D8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF85DC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF85E0: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 82DF85E4: 4BFFD62D  bl 0x82df5c10
	ctx.lr = 0x82DF85E8;
	sub_82DF5C10(ctx, base);
	// 82DF85E8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF85EC: 418200F4  beq 0x82df86e0
	if ctx.cr[0].eq {
	pc = 0x82DF86E0; continue 'dispatch;
	}
	// 82DF85F0: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF85F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF85F8: 80CB0008  lwz r6, 8(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF85FC: 4BFFFF94  b 0x82df8590
	pc = 0x82DF8590; continue 'dispatch;
	// 82DF8600: 3B5C000C  addi r26, r28, 0xc
	ctx.r[26].s64 = ctx.r[28].s64 + 12;
	// 82DF8604: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF8608: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82DF860C: 4BFFD605  bl 0x82df5c10
	ctx.lr = 0x82DF8610;
	sub_82DF5C10(ctx, base);
	// 82DF8610: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF8614: 4182005C  beq 0x82df8670
	if ctx.cr[0].eq {
	pc = 0x82DF8670; continue 'dispatch;
	}
	// 82DF8618: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 82DF861C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF8620: 4801F2D9  bl 0x82e178f8
	ctx.lr = 0x82DF8624;
	sub_82E178F8(ctx, base);
	// 82DF8624: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF8628: 83610050  lwz r27, 0x50(r1)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DF862C: 387B000C  addi r3, r27, 0xc
	ctx.r[3].s64 = ctx.r[27].s64 + 12;
	// 82DF8630: 4BFFD5E1  bl 0x82df5c10
	ctx.lr = 0x82DF8634;
	sub_82DF5C10(ctx, base);
	// 82DF8634: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF8638: 41820038  beq 0x82df8670
	if ctx.cr[0].eq {
	pc = 0x82DF8670; continue 'dispatch;
	}
	// 82DF863C: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF8640: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 82DF8644: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DF8648: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF864C: 896B002D  lbz r11, 0x2d(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(45 as u32) ) } as u64;
	// 82DF8650: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF8654: 419A0010  beq cr6, 0x82df8664
	if ctx.cr[6].eq {
	pc = 0x82DF8664; continue 'dispatch;
	}
	// 82DF8658: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82DF865C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF8660: 4BFFFF3C  b 0x82df859c
	pc = 0x82DF859C; continue 'dispatch;
	// 82DF8664: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DF8668: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DF866C: 4BFFFF30  b 0x82df859c
	pc = 0x82DF859C; continue 'dispatch;
	// 82DF8670: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF8674: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82DF8678: 4BFFD599  bl 0x82df5c10
	ctx.lr = 0x82DF867C;
	sub_82DF5C10(ctx, base);
	// 82DF867C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF8680: 41820060  beq 0x82df86e0
	if ctx.cr[0].eq {
	pc = 0x82DF86E0; continue 'dispatch;
	}
	// 82DF8684: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 82DF8688: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF868C: 4BFFD0C5  bl 0x82df5750
	ctx.lr = 0x82DF8690;
	sub_82DF5750(ctx, base);
	// 82DF8690: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF8694: 83610050  lwz r27, 0x50(r1)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DF8698: 7F1B5840  cmplw cr6, r27, r11
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF869C: 419A0018  beq cr6, 0x82df86b4
	if ctx.cr[6].eq {
	pc = 0x82DF86B4; continue 'dispatch;
	}
	// 82DF86A0: 389B000C  addi r4, r27, 0xc
	ctx.r[4].s64 = ctx.r[27].s64 + 12;
	// 82DF86A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF86A8: 4BFFD569  bl 0x82df5c10
	ctx.lr = 0x82DF86AC;
	sub_82DF5C10(ctx, base);
	// 82DF86AC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF86B0: 41820030  beq 0x82df86e0
	if ctx.cr[0].eq {
	pc = 0x82DF86E0; continue 'dispatch;
	}
	// 82DF86B4: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF86B8: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 82DF86BC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DF86C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF86C4: 896B002D  lbz r11, 0x2d(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(45 as u32) ) } as u64;
	// 82DF86C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF86CC: 419A000C  beq cr6, 0x82df86d8
	if ctx.cr[6].eq {
	pc = 0x82DF86D8; continue 'dispatch;
	}
	// 82DF86D0: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DF86D4: 4BFFFF88  b 0x82df865c
	pc = 0x82DF865C; continue 'dispatch;
	// 82DF86D8: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82DF86DC: 4BFFFF8C  b 0x82df8668
	pc = 0x82DF8668; continue 'dispatch;
	// 82DF86E0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DF86E4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DF86E8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF86EC: 4BFFF85D  bl 0x82df7f48
	ctx.lr = 0x82DF86F0;
	sub_82DF7F48(ctx, base);
	// 82DF86F0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DF86F4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF86F8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF86FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF8700: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DF8704: 483AFAAC  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF8708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF8708 size=88
    let mut pc: u32 = 0x82DF8708;
    'dispatch: loop {
        match pc {
            0x82DF8708 => {
    //   block [0x82DF8708..0x82DF8760)
	// 82DF8708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF870C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF8710: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF8714: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF8718: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF871C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF8720: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DF8724: 419A0018  beq cr6, 0x82df873c
	if ctx.cr[6].eq {
	pc = 0x82DF873C; continue 'dispatch;
	}
	// 82DF8728: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DF872C: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF8730: 4BFFED61  bl 0x82df7490
	ctx.lr = 0x82DF8734;
	sub_82DF7490(ctx, base);
	// 82DF8734: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF8738: 4B4C7B31  bl 0x822c0268
	ctx.lr = 0x82DF873C;
	sub_822C0268(ctx, base);
	// 82DF873C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF8740: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DF8744: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DF8748: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82DF874C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF8750: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF8754: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF8758: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF875C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF8760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF8760 size=132
    let mut pc: u32 = 0x82DF8760;
    'dispatch: loop {
        match pc {
            0x82DF8760 => {
    //   block [0x82DF8760..0x82DF87E4)
	// 82DF8760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF8764: 483AFA05  bl 0x831a8168
	ctx.lr = 0x82DF8768;
	sub_831A8130(ctx, base);
	// 82DF8768: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF876C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DF8770: 90A100A4  stw r5, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[5].u32 ) };
	// 82DF8774: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DF8778: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82DF877C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF8780: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF8784: 7F055040  cmplw cr6, r5, r10
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DF8788: 409A0044  bne cr6, 0x82df87cc
	if !ctx.cr[6].eq {
	pc = 0x82DF87CC; continue 'dispatch;
	}
	// 82DF878C: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF8790: 409A003C  bne cr6, 0x82df87cc
	if !ctx.cr[6].eq {
	pc = 0x82DF87CC; continue 'dispatch;
	}
	// 82DF8794: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF8798: 4BFFF8A1  bl 0x82df8038
	ctx.lr = 0x82DF879C;
	sub_82DF8038(ctx, base);
	// 82DF879C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF87A0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF87A4: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF87A8: 48000030  b 0x82df87d8
	pc = 0x82DF87D8; continue 'dispatch;
	// 82DF87AC: 386100A4  addi r3, r1, 0xa4
	ctx.r[3].s64 = ctx.r[1].s64 + 164;
	// 82DF87B0: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82DF87B4: 4BFFCF9D  bl 0x82df5750
	ctx.lr = 0x82DF87B8;
	sub_82DF5750(ctx, base);
	// 82DF87B8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82DF87BC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF87C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF87C4: 4BFFE86D  bl 0x82df7030
	ctx.lr = 0x82DF87C8;
	sub_82DF7030(ctx, base);
	// 82DF87C8: 80A100A4  lwz r5, 0xa4(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82DF87CC: 7F05F040  cmplw cr6, r5, r30
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82DF87D0: 409AFFDC  bne cr6, 0x82df87ac
	if !ctx.cr[6].eq {
	pc = 0x82DF87AC; continue 'dispatch;
	}
	// 82DF87D4: 90BD0000  stw r5, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82DF87D8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DF87DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DF87E0: 483AF9D8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF87E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF87E8 size=128
    let mut pc: u32 = 0x82DF87E8;
    'dispatch: loop {
        match pc {
            0x82DF87E8 => {
    //   block [0x82DF87E8..0x82DF8868)
	// 82DF87E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF87EC: 483AF981  bl 0x831a816c
	ctx.lr = 0x82DF87F0;
	sub_831A8130(ctx, base);
	// 82DF87F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF87F4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DF87F8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DF87FC: 4BFFD15D  bl 0x82df5958
	ctx.lr = 0x82DF8800;
	sub_82DF5958(ctx, base);
	// 82DF8800: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF8804: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF8808: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF880C: 419A0018  beq cr6, 0x82df8824
	if ctx.cr[6].eq {
	pc = 0x82DF8824; continue 'dispatch;
	}
	// 82DF8810: 389F000C  addi r4, r31, 0xc
	ctx.r[4].s64 = ctx.r[31].s64 + 12;
	// 82DF8814: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF8818: 4BFFAA21  bl 0x82df3238
	ctx.lr = 0x82DF881C;
	sub_82DF3238(ctx, base);
	// 82DF881C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF8820: 4182003C  beq 0x82df885c
	if ctx.cr[0].eq {
	pc = 0x82DF885C; continue 'dispatch;
	}
	// 82DF8824: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DF8828: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82DF882C: 4BFFB3D5  bl 0x82df3c00
	ctx.lr = 0x82DF8830;
	sub_82DF3C00(ctx, base);
	// 82DF8830: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF8834: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 82DF8838: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82DF883C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DF8840: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DF8844: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF8848: 4BFFF849  bl 0x82df8090
	ctx.lr = 0x82DF884C;
	sub_82DF8090(ctx, base);
	// 82DF884C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DF8850: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82DF8854: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF8858: 4BFFABD1  bl 0x82df3428
	ctx.lr = 0x82DF885C;
	sub_82DF3428(ctx, base);
	// 82DF885C: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82DF8860: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DF8864: 483AF958  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF8868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF8868 size=160
    let mut pc: u32 = 0x82DF8868;
    'dispatch: loop {
        match pc {
            0x82DF8868 => {
    //   block [0x82DF8868..0x82DF8908)
	// 82DF8868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF886C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF8870: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DF8874: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF8878: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF887C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF8880: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF8884: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF8888: 409A000C  bne cr6, 0x82df8894
	if !ctx.cr[6].eq {
	pc = 0x82DF8894; continue 'dispatch;
	}
	// 82DF888C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DF8890: 48000010  b 0x82df88a0
	pc = 0x82DF88A0; continue 'dispatch;
	// 82DF8894: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF8898: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82DF889C: 7D4A1670  srawi r10, r10, 2
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 2) as i64;
	// 82DF88A0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF88A4: 419A0038  beq cr6, 0x82df88dc
	if ctx.cr[6].eq {
	pc = 0x82DF88DC; continue 'dispatch;
	}
	// 82DF88A8: 813F000C  lwz r9, 0xc(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DF88AC: 7D6B4850  subf r11, r11, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 82DF88B0: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 82DF88B4: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF88B8: 40980024  bge cr6, 0x82df88dc
	if !ctx.cr[6].lt {
	pc = 0x82DF88DC; continue 'dispatch;
	}
	// 82DF88BC: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF88C0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82DF88C4: 419A000C  beq cr6, 0x82df88d0
	if ctx.cr[6].eq {
	pc = 0x82DF88D0; continue 'dispatch;
	}
	// 82DF88C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF88CC: 4BFFB335  bl 0x82df3c00
	ctx.lr = 0x82DF88D0;
	sub_82DF3C00(ctx, base);
	// 82DF88D0: 397E0004  addi r11, r30, 4
	ctx.r[11].s64 = ctx.r[30].s64 + 4;
	// 82DF88D4: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DF88D8: 48000018  b 0x82df88f0
	pc = 0x82DF88F0; continue 'dispatch;
	// 82DF88DC: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 82DF88E0: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF88E4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF88E8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF88EC: 4BFFF99D  bl 0x82df8288
	ctx.lr = 0x82DF88F0;
	sub_82DF8288(ctx, base);
	// 82DF88F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF88F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF88F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF88FC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DF8900: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF8904: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF8908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF8908 size=128
    let mut pc: u32 = 0x82DF8908;
    'dispatch: loop {
        match pc {
            0x82DF8908 => {
    //   block [0x82DF8908..0x82DF8988)
	// 82DF8908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF890C: 483AF861  bl 0x831a816c
	ctx.lr = 0x82DF8910;
	sub_831A8130(ctx, base);
	// 82DF8910: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF8914: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DF8918: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DF891C: 4BFFD03D  bl 0x82df5958
	ctx.lr = 0x82DF8920;
	sub_82DF5958(ctx, base);
	// 82DF8920: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF8924: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF8928: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF892C: 419A0018  beq cr6, 0x82df8944
	if ctx.cr[6].eq {
	pc = 0x82DF8944; continue 'dispatch;
	}
	// 82DF8930: 389F000C  addi r4, r31, 0xc
	ctx.r[4].s64 = ctx.r[31].s64 + 12;
	// 82DF8934: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF8938: 4BFFA901  bl 0x82df3238
	ctx.lr = 0x82DF893C;
	sub_82DF3238(ctx, base);
	// 82DF893C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF8940: 4182003C  beq 0x82df897c
	if ctx.cr[0].eq {
	pc = 0x82DF897C; continue 'dispatch;
	}
	// 82DF8944: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DF8948: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82DF894C: 4BFFB2B5  bl 0x82df3c00
	ctx.lr = 0x82DF8950;
	sub_82DF3C00(ctx, base);
	// 82DF8950: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF8954: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 82DF8958: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82DF895C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DF8960: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DF8964: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF8968: 4BFFF991  bl 0x82df82f8
	ctx.lr = 0x82DF896C;
	sub_82DF82F8(ctx, base);
	// 82DF896C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DF8970: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82DF8974: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF8978: 4BFFAAB1  bl 0x82df3428
	ctx.lr = 0x82DF897C;
	sub_82DF3428(ctx, base);
	// 82DF897C: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82DF8980: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DF8984: 483AF838  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF8988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF8988 size=160
    let mut pc: u32 = 0x82DF8988;
    'dispatch: loop {
        match pc {
            0x82DF8988 => {
    //   block [0x82DF8988..0x82DF8A28)
	// 82DF8988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF898C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF8990: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DF8994: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF8998: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF899C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF89A0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF89A4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF89A8: 409A000C  bne cr6, 0x82df89b4
	if !ctx.cr[6].eq {
	pc = 0x82DF89B4; continue 'dispatch;
	}
	// 82DF89AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DF89B0: 48000010  b 0x82df89c0
	pc = 0x82DF89C0; continue 'dispatch;
	// 82DF89B4: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF89B8: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82DF89BC: 7D4A1670  srawi r10, r10, 2
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 2) as i64;
	// 82DF89C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF89C4: 419A0038  beq cr6, 0x82df89fc
	if ctx.cr[6].eq {
	pc = 0x82DF89FC; continue 'dispatch;
	}
	// 82DF89C8: 813F000C  lwz r9, 0xc(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DF89CC: 7D6B4850  subf r11, r11, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 82DF89D0: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 82DF89D4: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF89D8: 40980024  bge cr6, 0x82df89fc
	if !ctx.cr[6].lt {
	pc = 0x82DF89FC; continue 'dispatch;
	}
	// 82DF89DC: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF89E0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82DF89E4: 419A000C  beq cr6, 0x82df89f0
	if ctx.cr[6].eq {
	pc = 0x82DF89F0; continue 'dispatch;
	}
	// 82DF89E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF89EC: 4BFFB215  bl 0x82df3c00
	ctx.lr = 0x82DF89F0;
	sub_82DF3C00(ctx, base);
	// 82DF89F0: 397E0004  addi r11, r30, 4
	ctx.r[11].s64 = ctx.r[30].s64 + 4;
	// 82DF89F4: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DF89F8: 48000018  b 0x82df8a10
	pc = 0x82DF8A10; continue 'dispatch;
	// 82DF89FC: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 82DF8A00: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF8A04: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF8A08: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF8A0C: 4BFFFAE5  bl 0x82df84f0
	ctx.lr = 0x82DF8A10;
	sub_82DF84F0(ctx, base);
	// 82DF8A10: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF8A14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF8A18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF8A1C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DF8A20: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF8A24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF8A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF8A28 size=136
    let mut pc: u32 = 0x82DF8A28;
    'dispatch: loop {
        match pc {
            0x82DF8A28 => {
    //   block [0x82DF8A28..0x82DF8AB0)
	// 82DF8A28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF8A2C: 483AF741  bl 0x831a816c
	ctx.lr = 0x82DF8A30;
	sub_831A8130(ctx, base);
	// 82DF8A30: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF8A34: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DF8A38: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DF8A3C: 4BFFD2D5  bl 0x82df5d10
	ctx.lr = 0x82DF8A40;
	sub_82DF5D10(ctx, base);
	// 82DF8A40: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF8A44: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF8A48: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF8A4C: 419A0018  beq cr6, 0x82df8a64
	if ctx.cr[6].eq {
	pc = 0x82DF8A64; continue 'dispatch;
	}
	// 82DF8A50: 389F000C  addi r4, r31, 0xc
	ctx.r[4].s64 = ctx.r[31].s64 + 12;
	// 82DF8A54: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF8A58: 4BFFD1B9  bl 0x82df5c10
	ctx.lr = 0x82DF8A5C;
	sub_82DF5C10(ctx, base);
	// 82DF8A5C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF8A60: 41820044  beq 0x82df8aa4
	if ctx.cr[0].eq {
	pc = 0x82DF8AA4; continue 'dispatch;
	}
	// 82DF8A64: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DF8A68: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DF8A6C: 4B4D13ED  bl 0x822c9e58
	ctx.lr = 0x82DF8A70;
	sub_822C9E58(ctx, base);
	// 82DF8A70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF8A74: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82DF8A78: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 82DF8A7C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DF8A80: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DF8A84: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF8A88: 4BFFFAD9  bl 0x82df8560
	ctx.lr = 0x82DF8A8C;
	sub_82DF8560(ctx, base);
	// 82DF8A8C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DF8A90: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF8A94: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DF8A98: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DF8A9C: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF8AA0: 4B4D0391  bl 0x822c8e30
	ctx.lr = 0x82DF8AA4;
	sub_822C8E30(ctx, base);
	// 82DF8AA4: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 82DF8AA8: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82DF8AAC: 483AF710  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF8AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF8AB0 size=80
    let mut pc: u32 = 0x82DF8AB0;
    'dispatch: loop {
        match pc {
            0x82DF8AB0 => {
    //   block [0x82DF8AB0..0x82DF8B00)
	// 82DF8AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF8AB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF8AB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF8ABC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF8AC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF8AC4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF8AC8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF8ACC: 80DF0004  lwz r6, 4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF8AD0: 80A60000  lwz r5, 0(r6)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF8AD4: 4BFFFC8D  bl 0x82df8760
	ctx.lr = 0x82DF8AD8;
	sub_82DF8760(ctx, base);
	// 82DF8AD8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF8ADC: 4B4C778D  bl 0x822c0268
	ctx.lr = 0x82DF8AE0;
	sub_822C0268(ctx, base);
	// 82DF8AE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF8AE4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DF8AE8: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DF8AEC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF8AF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF8AF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF8AF8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF8AFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF8B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF8B00 size=768
    let mut pc: u32 = 0x82DF8B00;
    'dispatch: loop {
        match pc {
            0x82DF8B00 => {
    //   block [0x82DF8B00..0x82DF8E00)
	// 82DF8B00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF8B04: 483AF659  bl 0x831a815c
	ctx.lr = 0x82DF8B08;
	sub_831A8130(ctx, base);
	// 82DF8B08: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF8B0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF8B10: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DF8B14: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 82DF8B18: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DF8B1C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82DF8B20: 4B4D1339  bl 0x822c9e58
	ctx.lr = 0x82DF8B24;
	sub_822C9E58(ctx, base);
	// 82DF8B24: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF8B28: 3B40001C  li r26, 0x1c
	ctx.r[26].s64 = 28;
	// 82DF8B2C: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82DF8B30: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF8B34: 409A000C  bne cr6, 0x82df8b40
	if !ctx.cr[6].eq {
	pc = 0x82DF8B40; continue 'dispatch;
	}
	// 82DF8B38: 7F28CB78  mr r8, r25
	ctx.r[8].u64 = ctx.r[25].u64;
	// 82DF8B3C: 48000010  b 0x82df8b4c
	pc = 0x82DF8B4C; continue 'dispatch;
	// 82DF8B40: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DF8B44: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82DF8B48: 7D0AD3D6  divw r8, r10, r26
	ctx.r[8].s32 = ctx.r[10].s32 / ctx.r[26].s32;
	// 82DF8B4C: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82DF8B50: 419A0298  beq cr6, 0x82df8de8
	if ctx.cr[6].eq {
	pc = 0x82DF8DE8; continue 'dispatch;
	}
	// 82DF8B54: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF8B58: 409A000C  bne cr6, 0x82df8b64
	if !ctx.cr[6].eq {
	pc = 0x82DF8B64; continue 'dispatch;
	}
	// 82DF8B5C: 7F2ACB78  mr r10, r25
	ctx.r[10].u64 = ctx.r[25].u64;
	// 82DF8B60: 48000010  b 0x82df8b70
	pc = 0x82DF8B70; continue 'dispatch;
	// 82DF8B64: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF8B68: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82DF8B6C: 7D4AD3D6  divw r10, r10, r26
	ctx.r[10].s32 = ctx.r[10].s32 / ctx.r[26].s32;
	// 82DF8B70: 3D200924  lis r9, 0x924
	ctx.r[9].s64 = 153354240;
	// 82DF8B74: 61299249  ori r9, r9, 0x9249
	ctx.r[9].u64 = ctx.r[9].u64 | 37449;
	// 82DF8B78: 7D4A4850  subf r10, r10, r9
	ctx.r[10].s64 = ctx.r[9].s64 - ctx.r[10].s64;
	// 82DF8B7C: 7F0AD840  cmplw cr6, r10, r27
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82DF8B80: 4098000C  bge cr6, 0x82df8b8c
	if !ctx.cr[6].lt {
	pc = 0x82DF8B8C; continue 'dispatch;
	}
	// 82DF8B84: 482CC5B5  bl 0x830c5138
	ctx.lr = 0x82DF8B88;
	sub_830C5138(ctx, base);
	// 82DF8B88: 48000260  b 0x82df8de8
	pc = 0x82DF8DE8; continue 'dispatch;
	// 82DF8B8C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF8B90: 409A000C  bne cr6, 0x82df8b9c
	if !ctx.cr[6].eq {
	pc = 0x82DF8B9C; continue 'dispatch;
	}
	// 82DF8B94: 7F2ACB78  mr r10, r25
	ctx.r[10].u64 = ctx.r[25].u64;
	// 82DF8B98: 48000010  b 0x82df8ba8
	pc = 0x82DF8BA8; continue 'dispatch;
	// 82DF8B9C: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF8BA0: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82DF8BA4: 7D4AD3D6  divw r10, r10, r26
	ctx.r[10].s32 = ctx.r[10].s32 / ctx.r[26].s32;
	// 82DF8BA8: 7D4ADA14  add r10, r10, r27
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[27].u64;
	// 82DF8BAC: 7F085040  cmplw cr6, r8, r10
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DF8BB0: 40980134  bge cr6, 0x82df8ce4
	if !ctx.cr[6].lt {
	pc = 0x82DF8CE4; continue 'dispatch;
	}
	// 82DF8BB4: 550AF87E  srwi r10, r8, 1
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DF8BB8: 7F3CCB78  mr r28, r25
	ctx.r[28].u64 = ctx.r[25].u64;
	// 82DF8BBC: 7D2A4850  subf r9, r10, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[10].s64;
	// 82DF8BC0: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82DF8BC4: 41980008  blt cr6, 0x82df8bcc
	if ctx.cr[6].lt {
	pc = 0x82DF8BCC; continue 'dispatch;
	}
	// 82DF8BC8: 7F8A4214  add r28, r10, r8
	ctx.r[28].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 82DF8BCC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF8BD0: 409A000C  bne cr6, 0x82df8bdc
	if !ctx.cr[6].eq {
	pc = 0x82DF8BDC; continue 'dispatch;
	}
	// 82DF8BD4: 7F2ACB78  mr r10, r25
	ctx.r[10].u64 = ctx.r[25].u64;
	// 82DF8BD8: 48000010  b 0x82df8be8
	pc = 0x82DF8BE8; continue 'dispatch;
	// 82DF8BDC: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF8BE0: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82DF8BE4: 7D4AD3D6  divw r10, r10, r26
	ctx.r[10].s32 = ctx.r[10].s32 / ctx.r[26].s32;
	// 82DF8BE8: 7D4ADA14  add r10, r10, r27
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[27].u64;
	// 82DF8BEC: 7F1C5040  cmplw cr6, r28, r10
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DF8BF0: 40980024  bge cr6, 0x82df8c14
	if !ctx.cr[6].lt {
	pc = 0x82DF8C14; continue 'dispatch;
	}
	// 82DF8BF4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF8BF8: 409A000C  bne cr6, 0x82df8c04
	if !ctx.cr[6].eq {
	pc = 0x82DF8C04; continue 'dispatch;
	}
	// 82DF8BFC: 7F2BCB78  mr r11, r25
	ctx.r[11].u64 = ctx.r[25].u64;
	// 82DF8C00: 48000010  b 0x82df8c10
	pc = 0x82DF8C10; continue 'dispatch;
	// 82DF8C04: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF8C08: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82DF8C0C: 7D6BD3D6  divw r11, r11, r26
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[26].s32;
	// 82DF8C10: 7F8BDA14  add r28, r11, r27
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 82DF8C14: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DF8C18: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DF8C1C: 482C6C85  bl 0x830bf8a0
	ctx.lr = 0x82DF8C20;
	sub_830BF8A0(ctx, base);
	// 82DF8C20: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82DF8C24: 88E10050  lbz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DF8C28: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DF8C2C: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF8C30: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82DF8C34: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DF8C38: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DF8C3C: 9B2B0000  stb r25, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[25].u8 ) };
	// 82DF8C40: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 82DF8C44: 89010050  lbz r8, 0x50(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DF8C48: 4BFFE8A1  bl 0x82df74e8
	ctx.lr = 0x82DF8C4C;
	sub_82DF74E8(ctx, base);
	// 82DF8C4C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DF8C50: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82DF8C54: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82DF8C58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF8C5C: 4BFFE905  bl 0x82df7560
	ctx.lr = 0x82DF8C60;
	sub_82DF7560(ctx, base);
	// 82DF8C60: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82DF8C64: 88E10050  lbz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DF8C68: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82DF8C6C: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82DF8C70: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF8C74: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF8C78: 9B2B0000  stb r25, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[25].u8 ) };
	// 82DF8C7C: 89010050  lbz r8, 0x50(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DF8C80: 4BFFE869  bl 0x82df74e8
	ctx.lr = 0x82DF8C84;
	sub_82DF74E8(ctx, base);
	// 82DF8C84: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF8C88: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DF8C8C: 409A000C  bne cr6, 0x82df8c98
	if !ctx.cr[6].eq {
	pc = 0x82DF8C98; continue 'dispatch;
	}
	// 82DF8C90: 7F2BCB78  mr r11, r25
	ctx.r[11].u64 = ctx.r[25].u64;
	// 82DF8C94: 48000010  b 0x82df8ca4
	pc = 0x82DF8CA4; continue 'dispatch;
	// 82DF8C98: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF8C9C: 7D635850  subf r11, r3, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 82DF8CA0: 7D6BD3D6  divw r11, r11, r26
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[26].s32;
	// 82DF8CA4: 7FCBDA14  add r30, r11, r27
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 82DF8CA8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DF8CAC: 419A0018  beq cr6, 0x82df8cc4
	if ctx.cr[6].eq {
	pc = 0x82DF8CC4; continue 'dispatch;
	}
	// 82DF8CB0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DF8CB4: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF8CB8: 4BFFE7D9  bl 0x82df7490
	ctx.lr = 0x82DF8CBC;
	sub_82DF7490(ctx, base);
	// 82DF8CBC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF8CC0: 4B4C75A9  bl 0x822c0268
	ctx.lr = 0x82DF8CC4;
	sub_822C0268(ctx, base);
	// 82DF8CC4: 1D7C001C  mulli r11, r28, 0x1c
	ctx.r[11].s64 = ctx.r[28].s64 * 28;
	// 82DF8CC8: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82DF8CCC: 1D5E001C  mulli r10, r30, 0x1c
	ctx.r[10].s64 = ctx.r[30].s64 * 28;
	// 82DF8CD0: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82DF8CD4: 7D4AEA14  add r10, r10, r29
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[29].u64;
	// 82DF8CD8: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82DF8CDC: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82DF8CE0: 48000108  b 0x82df8de8
	pc = 0x82DF8DE8; continue 'dispatch;
	// 82DF8CE4: 83BF0008  lwz r29, 8(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF8CE8: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82DF8CEC: 88E10050  lbz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DF8CF0: 7D7EE850  subf r11, r30, r29
	ctx.r[11].s64 = ctx.r[29].s64 - ctx.r[30].s64;
	// 82DF8CF4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DF8CF8: 7D6BD3D6  divw r11, r11, r26
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[26].s32;
	// 82DF8CFC: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82DF8D00: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82DF8D04: 9B2B0000  stb r25, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[25].u8 ) };
	// 82DF8D08: 4098006C  bge cr6, 0x82df8d74
	if !ctx.cr[6].lt {
	pc = 0x82DF8D74; continue 'dispatch;
	}
	// 82DF8D0C: 1F9B001C  mulli r28, r27, 0x1c
	ctx.r[28].s64 = ctx.r[27].s64 * 28;
	// 82DF8D10: 7CBCF214  add r5, r28, r30
	ctx.r[5].u64 = ctx.r[28].u64 + ctx.r[30].u64;
	// 82DF8D14: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF8D18: 89010050  lbz r8, 0x50(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DF8D1C: 4BFFE7CD  bl 0x82df74e8
	ctx.lr = 0x82DF8D20;
	sub_82DF74E8(ctx, base);
	// 82DF8D20: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF8D24: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82DF8D28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF8D2C: 7D7E2050  subf r11, r30, r4
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[30].s64;
	// 82DF8D30: 7D6BD3D6  divw r11, r11, r26
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[26].s32;
	// 82DF8D34: 7CABD850  subf r5, r11, r27
	ctx.r[5].s64 = ctx.r[27].s64 - ctx.r[11].s64;
	// 82DF8D38: 4BFFE829  bl 0x82df7560
	ctx.lr = 0x82DF8D3C;
	sub_82DF7560(ctx, base);
	// 82DF8D3C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF8D40: 7D6BE214  add r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82DF8D44: 7FBC5850  subf r29, r28, r11
	ctx.r[29].s64 = ctx.r[11].s64 - ctx.r[28].s64;
	// 82DF8D48: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DF8D4C: 4800001C  b 0x82df8d68
	pc = 0x82DF8D68; continue 'dispatch;
	// 82DF8D50: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 82DF8D54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF8D58: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82DF8D5C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF8D60: 4B4D03D1  bl 0x822c9130
	ctx.lr = 0x82DF8D64;
	sub_822C9130(ctx, base);
	// 82DF8D64: 3BDE001C  addi r30, r30, 0x1c
	ctx.r[30].s64 = ctx.r[30].s64 + 28;
	// 82DF8D68: 7F1EE840  cmplw cr6, r30, r29
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82DF8D6C: 409AFFE4  bne cr6, 0x82df8d50
	if !ctx.cr[6].eq {
	pc = 0x82DF8D50; continue 'dispatch;
	}
	// 82DF8D70: 48000078  b 0x82df8de8
	pc = 0x82DF8DE8; continue 'dispatch;
	// 82DF8D74: 1F7B001C  mulli r27, r27, 0x1c
	ctx.r[27].s64 = ctx.r[27].s64 * 28;
	// 82DF8D78: 7F9BE850  subf r28, r27, r29
	ctx.r[28].s64 = ctx.r[29].s64 - ctx.r[27].s64;
	// 82DF8D7C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DF8D80: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DF8D84: 89010050  lbz r8, 0x50(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DF8D88: 4BFFE761  bl 0x82df74e8
	ctx.lr = 0x82DF8D8C;
	sub_82DF74E8(ctx, base);
	// 82DF8D8C: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82DF8D90: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	// 82DF8D94: 7F1EE040  cmplw cr6, r30, r28
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82DF8D98: 419A0020  beq cr6, 0x82df8db8
	if ctx.cr[6].eq {
	pc = 0x82DF8DB8; continue 'dispatch;
	}
	// 82DF8D9C: 7FBCE850  subf r29, r28, r29
	ctx.r[29].s64 = ctx.r[29].s64 - ctx.r[28].s64;
	// 82DF8DA0: 3BFFFFE4  addi r31, r31, -0x1c
	ctx.r[31].s64 = ctx.r[31].s64 + -28;
	// 82DF8DA4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF8DA8: 7C7DFA14  add r3, r29, r31
	ctx.r[3].u64 = ctx.r[29].u64 + ctx.r[31].u64;
	// 82DF8DAC: 4BFFD2BD  bl 0x82df6068
	ctx.lr = 0x82DF8DB0;
	sub_82DF6068(ctx, base);
	// 82DF8DB0: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82DF8DB4: 409AFFEC  bne cr6, 0x82df8da0
	if !ctx.cr[6].eq {
	pc = 0x82DF8DA0; continue 'dispatch;
	}
	// 82DF8DB8: 7FBBF214  add r29, r27, r30
	ctx.r[29].u64 = ctx.r[27].u64 + ctx.r[30].u64;
	// 82DF8DBC: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 82DF8DC0: 7F1EE840  cmplw cr6, r30, r29
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82DF8DC4: 419A0024  beq cr6, 0x82df8de8
	if ctx.cr[6].eq {
	pc = 0x82DF8DE8; continue 'dispatch;
	}
	// 82DF8DC8: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 82DF8DCC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF8DD0: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82DF8DD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF8DD8: 4B4D0359  bl 0x822c9130
	ctx.lr = 0x82DF8DDC;
	sub_822C9130(ctx, base);
	// 82DF8DDC: 3BFF001C  addi r31, r31, 0x1c
	ctx.r[31].s64 = ctx.r[31].s64 + 28;
	// 82DF8DE0: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82DF8DE4: 409AFFE4  bne cr6, 0x82df8dc8
	if !ctx.cr[6].eq {
	pc = 0x82DF8DC8; continue 'dispatch;
	}
	// 82DF8DE8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF8DEC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DF8DF0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DF8DF4: 4B4D003D  bl 0x822c8e30
	ctx.lr = 0x82DF8DF8;
	sub_822C8E30(ctx, base);
	// 82DF8DF8: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82DF8DFC: 483AF3B0  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF8E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF8E00 size=196
    let mut pc: u32 = 0x82DF8E00;
    'dispatch: loop {
        match pc {
            0x82DF8E00 => {
    //   block [0x82DF8E00..0x82DF8EC4)
	// 82DF8E00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF8E04: 483AF369  bl 0x831a816c
	ctx.lr = 0x82DF8E08;
	sub_831A8130(ctx, base);
	// 82DF8E08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF8E0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF8E10: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DF8E14: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DF8E18: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF8E1C: 419A0018  beq cr6, 0x82df8e34
	if ctx.cr[6].eq {
	pc = 0x82DF8E34; continue 'dispatch;
	}
	// 82DF8E20: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82DF8E24: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF8E28: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF8E2C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DF8E30: 4E800421  bctrl
	ctx.lr = 0x82DF8E34;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DF8E34: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DF8E38: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF8E3C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF8E40: 4800E261  bl 0x82e070a0
	ctx.lr = 0x82DF8E44;
	sub_82E070A0(ctx, base);
	// 82DF8E44: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DF8E48: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF8E4C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DF8E50: 409A0044  bne cr6, 0x82df8e94
	if !ctx.cr[6].eq {
	pc = 0x82DF8E94; continue 'dispatch;
	}
	// 82DF8E54: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DF8E58: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82DF8E5C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF8E60: 409A000C  bne cr6, 0x82df8e6c
	if !ctx.cr[6].eq {
	pc = 0x82DF8E6C; continue 'dispatch;
	}
	// 82DF8E64: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DF8E68: 48000010  b 0x82df8e78
	pc = 0x82DF8E78; continue 'dispatch;
	// 82DF8E6C: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF8E70: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82DF8E74: 7D7E1670  srawi r30, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[30].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 82DF8E78: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DF8E7C: 4BFFF9ED  bl 0x82df8868
	ctx.lr = 0x82DF8E80;
	sub_82DF8868(ctx, base);
	// 82DF8E80: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DF8E84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF8E88: 4BFFF961  bl 0x82df87e8
	ctx.lr = 0x82DF8E8C;
	sub_82DF87E8(ctx, base);
	// 82DF8E8C: 93C30000  stw r30, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82DF8E90: 48000008  b 0x82df8e98
	pc = 0x82DF8E98; continue 'dispatch;
	// 82DF8E94: 83CB0010  lwz r30, 0x10(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DF8E98: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DF8E9C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF8EA0: 419A0018  beq cr6, 0x82df8eb8
	if ctx.cr[6].eq {
	pc = 0x82DF8EB8; continue 'dispatch;
	}
	// 82DF8EA4: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82DF8EA8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF8EAC: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DF8EB0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DF8EB4: 4E800421  bctrl
	ctx.lr = 0x82DF8EB8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DF8EB8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF8EBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DF8EC0: 483AF2FC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF8EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF8EC8 size=196
    let mut pc: u32 = 0x82DF8EC8;
    'dispatch: loop {
        match pc {
            0x82DF8EC8 => {
    //   block [0x82DF8EC8..0x82DF8F8C)
	// 82DF8EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF8ECC: 483AF2A1  bl 0x831a816c
	ctx.lr = 0x82DF8ED0;
	sub_831A8130(ctx, base);
	// 82DF8ED0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF8ED4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF8ED8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DF8EDC: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DF8EE0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF8EE4: 419A0018  beq cr6, 0x82df8efc
	if ctx.cr[6].eq {
	pc = 0x82DF8EFC; continue 'dispatch;
	}
	// 82DF8EE8: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82DF8EEC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF8EF0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF8EF4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DF8EF8: 4E800421  bctrl
	ctx.lr = 0x82DF8EFC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DF8EFC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DF8F00: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF8F04: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF8F08: 4800E199  bl 0x82e070a0
	ctx.lr = 0x82DF8F0C;
	sub_82E070A0(ctx, base);
	// 82DF8F0C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DF8F10: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF8F14: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DF8F18: 409A0044  bne cr6, 0x82df8f5c
	if !ctx.cr[6].eq {
	pc = 0x82DF8F5C; continue 'dispatch;
	}
	// 82DF8F1C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DF8F20: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82DF8F24: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF8F28: 409A000C  bne cr6, 0x82df8f34
	if !ctx.cr[6].eq {
	pc = 0x82DF8F34; continue 'dispatch;
	}
	// 82DF8F2C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DF8F30: 48000010  b 0x82df8f40
	pc = 0x82DF8F40; continue 'dispatch;
	// 82DF8F34: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF8F38: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82DF8F3C: 7D7E1670  srawi r30, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[30].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 82DF8F40: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DF8F44: 4BFFFA45  bl 0x82df8988
	ctx.lr = 0x82DF8F48;
	sub_82DF8988(ctx, base);
	// 82DF8F48: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DF8F4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF8F50: 4BFFF9B9  bl 0x82df8908
	ctx.lr = 0x82DF8F54;
	sub_82DF8908(ctx, base);
	// 82DF8F54: 93C30000  stw r30, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82DF8F58: 48000008  b 0x82df8f60
	pc = 0x82DF8F60; continue 'dispatch;
	// 82DF8F5C: 83CB0010  lwz r30, 0x10(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DF8F60: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DF8F64: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF8F68: 419A0018  beq cr6, 0x82df8f80
	if ctx.cr[6].eq {
	pc = 0x82DF8F80; continue 'dispatch;
	}
	// 82DF8F6C: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82DF8F70: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF8F74: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DF8F78: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DF8F7C: 4E800421  bctrl
	ctx.lr = 0x82DF8F80;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DF8F80: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF8F84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DF8F88: 483AF234  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF8F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF8F90 size=112
    let mut pc: u32 = 0x82DF8F90;
    'dispatch: loop {
        match pc {
            0x82DF8F90 => {
    //   block [0x82DF8F90..0x82DF9000)
	// 82DF8F90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF8F94: 483AF1D9  bl 0x831a816c
	ctx.lr = 0x82DF8F98;
	sub_831A8130(ctx, base);
	// 82DF8F98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF8F9C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DF8FA0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DF8FA4: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DF8FA8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF8FAC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF8FB0: 419A0018  beq cr6, 0x82df8fc8
	if ctx.cr[6].eq {
	pc = 0x82DF8FC8; continue 'dispatch;
	}
	// 82DF8FB4: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF8FB8: 3940001C  li r10, 0x1c
	ctx.r[10].s64 = 28;
	// 82DF8FBC: 7D2B4850  subf r9, r11, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 82DF8FC0: 7D2953D7  divw. r9, r9, r10
	ctx.r[9].s32 = ctx.r[9].s32 / ctx.r[10].s32;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82DF8FC4: 4082000C  bne 0x82df8fd0
	if !ctx.cr[0].eq {
	pc = 0x82DF8FD0; continue 'dispatch;
	}
	// 82DF8FC8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DF8FCC: 4800000C  b 0x82df8fd8
	pc = 0x82DF8FD8; continue 'dispatch;
	// 82DF8FD0: 7D6B2050  subf r11, r11, r4
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[11].s64;
	// 82DF8FD4: 7FCB53D6  divw r30, r11, r10
	ctx.r[30].s32 = ctx.r[11].s32 / ctx.r[10].s32;
	// 82DF8FD8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DF8FDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF8FE0: 4BFFFB21  bl 0x82df8b00
	ctx.lr = 0x82DF8FE4;
	sub_82DF8B00(ctx, base);
	// 82DF8FE4: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF8FE8: 1D7E001C  mulli r11, r30, 0x1c
	ctx.r[11].s64 = ctx.r[30].s64 * 28;
	// 82DF8FEC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DF8FF0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DF8FF4: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF8FF8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF8FFC: 483AF1C0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF9000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF9000 size=68
    let mut pc: u32 = 0x82DF9000;
    'dispatch: loop {
        match pc {
            0x82DF9000 => {
    //   block [0x82DF9000..0x82DF9044)
	// 82DF9000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF9004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF9008: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF900C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF9010: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF9014: 48032995  bl 0x82e2b9a8
	ctx.lr = 0x82DF9018;
	sub_82E2B9A8(ctx, base);
	// 82DF9018: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF901C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF9020: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82DF9024: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82DF9028: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82DF902C: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82DF9030: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF9034: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF9038: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF903C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF9040: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF9048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF9048 size=68
    let mut pc: u32 = 0x82DF9048;
    'dispatch: loop {
        match pc {
            0x82DF9048 => {
    //   block [0x82DF9048..0x82DF908C)
	// 82DF9048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF904C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF9050: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF9054: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF9058: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF905C: 4BFFCC05  bl 0x82df5c60
	ctx.lr = 0x82DF9060;
	sub_82DF5C60(ctx, base);
	// 82DF9060: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF9064: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF9068: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82DF906C: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82DF9070: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82DF9074: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82DF9078: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF907C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF9080: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF9084: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF9088: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF9090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF9090 size=164
    let mut pc: u32 = 0x82DF9090;
    'dispatch: loop {
        match pc {
            0x82DF9090 => {
    //   block [0x82DF9090..0x82DF9134)
	// 82DF9090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF9094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF9098: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DF909C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF90A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF90A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF90A8: 3920001C  li r9, 0x1c
	ctx.r[9].s64 = 28;
	// 82DF90AC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF90B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF90B4: 409A000C  bne cr6, 0x82df90c0
	if !ctx.cr[6].eq {
	pc = 0x82DF90C0; continue 'dispatch;
	}
	// 82DF90B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DF90BC: 48000010  b 0x82df90cc
	pc = 0x82DF90CC; continue 'dispatch;
	// 82DF90C0: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF90C4: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82DF90C8: 7D4A4BD6  divw r10, r10, r9
	ctx.r[10].s32 = ctx.r[10].s32 / ctx.r[9].s32;
	// 82DF90CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF90D0: 419A0038  beq cr6, 0x82df9108
	if ctx.cr[6].eq {
	pc = 0x82DF9108; continue 'dispatch;
	}
	// 82DF90D4: 811F000C  lwz r8, 0xc(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DF90D8: 7D6B4050  subf r11, r11, r8
	ctx.r[11].s64 = ctx.r[8].s64 - ctx.r[11].s64;
	// 82DF90DC: 7D6B4BD6  divw r11, r11, r9
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[9].s32;
	// 82DF90E0: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF90E4: 40980024  bge cr6, 0x82df9108
	if !ctx.cr[6].lt {
	pc = 0x82DF9108; continue 'dispatch;
	}
	// 82DF90E8: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF90EC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82DF90F0: 419A000C  beq cr6, 0x82df90fc
	if ctx.cr[6].eq {
	pc = 0x82DF90FC; continue 'dispatch;
	}
	// 82DF90F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF90F8: 4B4D0D61  bl 0x822c9e58
	ctx.lr = 0x82DF90FC;
	sub_822C9E58(ctx, base);
	// 82DF90FC: 397E001C  addi r11, r30, 0x1c
	ctx.r[11].s64 = ctx.r[30].s64 + 28;
	// 82DF9100: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DF9104: 48000018  b 0x82df911c
	pc = 0x82DF911C; continue 'dispatch;
	// 82DF9108: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 82DF910C: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF9110: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF9114: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF9118: 4BFFFE79  bl 0x82df8f90
	ctx.lr = 0x82DF911C;
	sub_82DF8F90(ctx, base);
	// 82DF911C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF9120: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF9124: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF9128: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DF912C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF9130: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF9138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF9138 size=92
    let mut pc: u32 = 0x82DF9138;
    'dispatch: loop {
        match pc {
            0x82DF9138 => {
    //   block [0x82DF9138..0x82DF9194)
	// 82DF9138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF913C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF9140: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF9144: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF9148: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82DF914C: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82DF9150: 3BEB5FEC  addi r31, r11, 0x5fec
	ctx.r[31].s64 = ctx.r[11].s64 + 24556;
	// 82DF9154: 816A600C  lwz r11, 0x600c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(24588 as u32) ) } as u64;
	// 82DF9158: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82DF915C: 40820020  bne 0x82df917c
	if !ctx.cr[0].eq {
	pc = 0x82DF917C; continue 'dispatch;
	}
	// 82DF9160: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 82DF9164: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF9168: 916A600C  stw r11, 0x600c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(24588 as u32), ctx.r[11].u32 ) };
	// 82DF916C: 4BFFFEDD  bl 0x82df9048
	ctx.lr = 0x82DF9170;
	sub_82DF9048(ctx, base);
	// 82DF9170: 3D608324  lis r11, -0x7cdc
	ctx.r[11].s64 = -2094792704;
	// 82DF9174: 386B1C68  addi r3, r11, 0x1c68
	ctx.r[3].s64 = ctx.r[11].s64 + 7272;
	// 82DF9178: 483AF361  bl 0x831a84d8
	ctx.lr = 0x82DF917C;
	sub_831A84D8(ctx, base);
	// 82DF917C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF9180: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF9184: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF9188: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF918C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF9190: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF9198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF9198 size=68
    let mut pc: u32 = 0x82DF9198;
    'dispatch: loop {
        match pc {
            0x82DF9198 => {
    //   block [0x82DF9198..0x82DF91DC)
	// 82DF9198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF919C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF91A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF91A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF91A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF91AC: 4BFFCB0D  bl 0x82df5cb8
	ctx.lr = 0x82DF91B0;
	sub_82DF5CB8(ctx, base);
	// 82DF91B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF91B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF91B8: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82DF91BC: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82DF91C0: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82DF91C4: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82DF91C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF91CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF91D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF91D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF91D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF91E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF91E0 size=200
    let mut pc: u32 = 0x82DF91E0;
    'dispatch: loop {
        match pc {
            0x82DF91E0 => {
    //   block [0x82DF91E0..0x82DF92A8)
	// 82DF91E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF91E4: 483AEF89  bl 0x831a816c
	ctx.lr = 0x82DF91E8;
	sub_831A8130(ctx, base);
	// 82DF91E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF91EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF91F0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DF91F4: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DF91F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF91FC: 419A0018  beq cr6, 0x82df9214
	if ctx.cr[6].eq {
	pc = 0x82DF9214; continue 'dispatch;
	}
	// 82DF9200: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82DF9204: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF9208: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF920C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DF9210: 4E800421  bctrl
	ctx.lr = 0x82DF9214;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DF9214: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DF9218: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF921C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF9220: 4BFFCCD1  bl 0x82df5ef0
	ctx.lr = 0x82DF9224;
	sub_82DF5EF0(ctx, base);
	// 82DF9224: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DF9228: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF922C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DF9230: 409A0048  bne cr6, 0x82df9278
	if !ctx.cr[6].eq {
	pc = 0x82DF9278; continue 'dispatch;
	}
	// 82DF9234: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DF9238: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82DF923C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF9240: 409A000C  bne cr6, 0x82df924c
	if !ctx.cr[6].eq {
	pc = 0x82DF924C; continue 'dispatch;
	}
	// 82DF9244: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DF9248: 48000014  b 0x82df925c
	pc = 0x82DF925C; continue 'dispatch;
	// 82DF924C: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF9250: 3920001C  li r9, 0x1c
	ctx.r[9].s64 = 28;
	// 82DF9254: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82DF9258: 7FCB4BD6  divw r30, r11, r9
	ctx.r[30].s32 = ctx.r[11].s32 / ctx.r[9].s32;
	// 82DF925C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DF9260: 4BFFFE31  bl 0x82df9090
	ctx.lr = 0x82DF9264;
	sub_82DF9090(ctx, base);
	// 82DF9264: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DF9268: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF926C: 4BFFF7BD  bl 0x82df8a28
	ctx.lr = 0x82DF9270;
	sub_82DF8A28(ctx, base);
	// 82DF9270: 93C30000  stw r30, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82DF9274: 48000008  b 0x82df927c
	pc = 0x82DF927C; continue 'dispatch;
	// 82DF9278: 83CB0028  lwz r30, 0x28(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DF927C: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DF9280: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF9284: 419A0018  beq cr6, 0x82df929c
	if ctx.cr[6].eq {
	pc = 0x82DF929C; continue 'dispatch;
	}
	// 82DF9288: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82DF928C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF9290: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DF9294: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DF9298: 4E800421  bctrl
	ctx.lr = 0x82DF929C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DF929C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF92A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DF92A4: 483AEF18  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF92A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF92A8 size=92
    let mut pc: u32 = 0x82DF92A8;
    'dispatch: loop {
        match pc {
            0x82DF92A8 => {
    //   block [0x82DF92A8..0x82DF9304)
	// 82DF92A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF92AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF92B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF92B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF92B8: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82DF92BC: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82DF92C0: 3BEB6010  addi r31, r11, 0x6010
	ctx.r[31].s64 = ctx.r[11].s64 + 24592;
	// 82DF92C4: 816A6030  lwz r11, 0x6030(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(24624 as u32) ) } as u64;
	// 82DF92C8: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82DF92CC: 40820020  bne 0x82df92ec
	if !ctx.cr[0].eq {
	pc = 0x82DF92EC; continue 'dispatch;
	}
	// 82DF92D0: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 82DF92D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF92D8: 916A6030  stw r11, 0x6030(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(24624 as u32), ctx.r[11].u32 ) };
	// 82DF92DC: 4BFFFD25  bl 0x82df9000
	ctx.lr = 0x82DF92E0;
	sub_82DF9000(ctx, base);
	// 82DF92E0: 3D608324  lis r11, -0x7cdc
	ctx.r[11].s64 = -2094792704;
	// 82DF92E4: 386B1CA8  addi r3, r11, 0x1ca8
	ctx.r[3].s64 = ctx.r[11].s64 + 7336;
	// 82DF92E8: 483AF1F1  bl 0x831a84d8
	ctx.lr = 0x82DF92EC;
	sub_831A84D8(ctx, base);
	// 82DF92EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF92F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF92F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF92F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF92FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF9300: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF9308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF9308 size=72
    let mut pc: u32 = 0x82DF9308;
    'dispatch: loop {
        match pc {
            0x82DF9308 => {
    //   block [0x82DF9308..0x82DF9350)
	// 82DF9308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF930C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF9310: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DF9314: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF9318: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF931C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF9320: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DF9324: 4BFFFE15  bl 0x82df9138
	ctx.lr = 0x82DF9328;
	sub_82DF9138(ctx, base);
	// 82DF9328: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DF932C: 4BFFFB9D  bl 0x82df8ec8
	ctx.lr = 0x82DF9330;
	sub_82DF8EC8(ctx, base);
	// 82DF9330: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82DF9334: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF9338: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF933C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF9340: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF9344: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DF9348: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF934C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF9350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF9350 size=76
    let mut pc: u32 = 0x82DF9350;
    'dispatch: loop {
        match pc {
            0x82DF9350 => {
    //   block [0x82DF9350..0x82DF939C)
	// 82DF9350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF9354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF9358: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DF935C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF9360: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF9364: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF9368: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DF936C: 4BFFFDCD  bl 0x82df9138
	ctx.lr = 0x82DF9370;
	sub_82DF9138(ctx, base);
	// 82DF9370: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DF9374: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF9378: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DF937C: 4BFFC50D  bl 0x82df5888
	ctx.lr = 0x82DF9380;
	sub_82DF5888(ctx, base);
	// 82DF9380: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF9384: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF9388: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF938C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF9390: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DF9394: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF9398: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF93A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF93A0 size=36
    let mut pc: u32 = 0x82DF93A0;
    'dispatch: loop {
        match pc {
            0x82DF93A0 => {
    //   block [0x82DF93A0..0x82DF93C4)
	// 82DF93A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF93A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF93A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF93AC: 4BFFFEFD  bl 0x82df92a8
	ctx.lr = 0x82DF93B0;
	sub_82DF92A8(ctx, base);
	// 82DF93B0: 4BFFC491  bl 0x82df5840
	ctx.lr = 0x82DF93B4;
	sub_82DF5840(ctx, base);
	// 82DF93B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF93B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF93BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF93C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF93C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF93C8 size=64
    let mut pc: u32 = 0x82DF93C8;
    'dispatch: loop {
        match pc {
            0x82DF93C8 => {
    //   block [0x82DF93C8..0x82DF9408)
	// 82DF93C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF93CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF93D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF93D4: 4BFFFED5  bl 0x82df92a8
	ctx.lr = 0x82DF93D8;
	sub_82DF92A8(ctx, base);
	// 82DF93D8: 8063001C  lwz r3, 0x1c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DF93DC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DF93E0: 419A0018  beq cr6, 0x82df93f8
	if ctx.cr[6].eq {
	pc = 0x82DF93F8; continue 'dispatch;
	}
	// 82DF93E4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF93E8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DF93EC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF93F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DF93F4: 4E800421  bctrl
	ctx.lr = 0x82DF93F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DF93F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF93FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF9400: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF9404: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF9408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF9408 size=72
    let mut pc: u32 = 0x82DF9408;
    'dispatch: loop {
        match pc {
            0x82DF9408 => {
    //   block [0x82DF9408..0x82DF9450)
	// 82DF9408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF940C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF9410: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DF9414: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF9418: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF941C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF9420: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DF9424: 4BFFFE85  bl 0x82df92a8
	ctx.lr = 0x82DF9428;
	sub_82DF92A8(ctx, base);
	// 82DF9428: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DF942C: 4BFFF9D5  bl 0x82df8e00
	ctx.lr = 0x82DF9430;
	sub_82DF8E00(ctx, base);
	// 82DF9430: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82DF9434: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF9438: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF943C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF9440: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF9444: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DF9448: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF944C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF9450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF9450 size=76
    let mut pc: u32 = 0x82DF9450;
    'dispatch: loop {
        match pc {
            0x82DF9450 => {
    //   block [0x82DF9450..0x82DF949C)
	// 82DF9450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF9454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF9458: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DF945C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF9460: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF9464: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF9468: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DF946C: 4BFFFE3D  bl 0x82df92a8
	ctx.lr = 0x82DF9470;
	sub_82DF92A8(ctx, base);
	// 82DF9470: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DF9474: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF9478: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DF947C: 4BFFC40D  bl 0x82df5888
	ctx.lr = 0x82DF9480;
	sub_82DF5888(ctx, base);
	// 82DF9480: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF9484: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF9488: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF948C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF9490: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DF9494: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF9498: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF94A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF94A0 size=92
    let mut pc: u32 = 0x82DF94A0;
    'dispatch: loop {
        match pc {
            0x82DF94A0 => {
    //   block [0x82DF94A0..0x82DF94FC)
	// 82DF94A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF94A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF94A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF94AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF94B0: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82DF94B4: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82DF94B8: 3BEB6034  addi r31, r11, 0x6034
	ctx.r[31].s64 = ctx.r[11].s64 + 24628;
	// 82DF94BC: 816A6054  lwz r11, 0x6054(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(24660 as u32) ) } as u64;
	// 82DF94C0: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82DF94C4: 40820020  bne 0x82df94e4
	if !ctx.cr[0].eq {
	pc = 0x82DF94E4; continue 'dispatch;
	}
	// 82DF94C8: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 82DF94CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF94D0: 916A6054  stw r11, 0x6054(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(24660 as u32), ctx.r[11].u32 ) };
	// 82DF94D4: 4BFFFCC5  bl 0x82df9198
	ctx.lr = 0x82DF94D8;
	sub_82DF9198(ctx, base);
	// 82DF94D8: 3D608324  lis r11, -0x7cdc
	ctx.r[11].s64 = -2094792704;
	// 82DF94DC: 386B1CE8  addi r3, r11, 0x1ce8
	ctx.r[3].s64 = ctx.r[11].s64 + 7400;
	// 82DF94E0: 483AEFF9  bl 0x831a84d8
	ctx.lr = 0x82DF94E4;
	sub_831A84D8(ctx, base);
	// 82DF94E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF94E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF94EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF94F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF94F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF94F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF9500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF9500 size=48
    let mut pc: u32 = 0x82DF9500;
    'dispatch: loop {
        match pc {
            0x82DF9500 => {
    //   block [0x82DF9500..0x82DF9530)
	// 82DF9500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF9504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF9508: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF950C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF9510: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF9514: 4BFFFEF5  bl 0x82df9408
	ctx.lr = 0x82DF9518;
	sub_82DF9408(ctx, base);
	// 82DF9518: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF951C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF9520: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF9524: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF9528: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF952C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF9530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF9530 size=72
    let mut pc: u32 = 0x82DF9530;
    'dispatch: loop {
        match pc {
            0x82DF9530 => {
    //   block [0x82DF9530..0x82DF9578)
	// 82DF9530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF9534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF9538: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF953C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF9540: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF9544: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF9548: 4BFFA4C1  bl 0x82df3a08
	ctx.lr = 0x82DF954C;
	sub_82DF3A08(ctx, base);
	// 82DF954C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF9550: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DF9554: 4BFFFEB5  bl 0x82df9408
	ctx.lr = 0x82DF9558;
	sub_82DF9408(ctx, base);
	// 82DF9558: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF955C: 4BFF9ECD  bl 0x82df3428
	ctx.lr = 0x82DF9560;
	sub_82DF3428(ctx, base);
	// 82DF9560: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF9564: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF9568: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF956C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF9570: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF9574: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF9578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF9578 size=36
    let mut pc: u32 = 0x82DF9578;
    'dispatch: loop {
        match pc {
            0x82DF9578 => {
    //   block [0x82DF9578..0x82DF959C)
	// 82DF9578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF957C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF9580: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF9584: 4BFFFF1D  bl 0x82df94a0
	ctx.lr = 0x82DF9588;
	sub_82DF94A0(ctx, base);
	// 82DF9588: 4BFFC2B9  bl 0x82df5840
	ctx.lr = 0x82DF958C;
	sub_82DF5840(ctx, base);
	// 82DF958C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF9590: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF9594: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF9598: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF95A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF95A0 size=64
    let mut pc: u32 = 0x82DF95A0;
    'dispatch: loop {
        match pc {
            0x82DF95A0 => {
    //   block [0x82DF95A0..0x82DF95E0)
	// 82DF95A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF95A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF95A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF95AC: 4BFFFEF5  bl 0x82df94a0
	ctx.lr = 0x82DF95B0;
	sub_82DF94A0(ctx, base);
	// 82DF95B0: 8063001C  lwz r3, 0x1c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DF95B4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DF95B8: 419A0018  beq cr6, 0x82df95d0
	if ctx.cr[6].eq {
	pc = 0x82DF95D0; continue 'dispatch;
	}
	// 82DF95BC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF95C0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DF95C4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF95C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DF95CC: 4E800421  bctrl
	ctx.lr = 0x82DF95D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DF95D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF95D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF95D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF95DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF95E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF95E0 size=72
    let mut pc: u32 = 0x82DF95E0;
    'dispatch: loop {
        match pc {
            0x82DF95E0 => {
    //   block [0x82DF95E0..0x82DF9628)
	// 82DF95E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF95E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF95E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DF95EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF95F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF95F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF95F8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DF95FC: 4BFFFEA5  bl 0x82df94a0
	ctx.lr = 0x82DF9600;
	sub_82DF94A0(ctx, base);
	// 82DF9600: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DF9604: 4BFFFBDD  bl 0x82df91e0
	ctx.lr = 0x82DF9608;
	sub_82DF91E0(ctx, base);
	// 82DF9608: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82DF960C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF9610: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF9614: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF9618: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF961C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DF9620: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF9624: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF9628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF9628 size=76
    let mut pc: u32 = 0x82DF9628;
    'dispatch: loop {
        match pc {
            0x82DF9628 => {
    //   block [0x82DF9628..0x82DF9674)
	// 82DF9628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF962C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF9630: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DF9634: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF9638: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF963C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF9640: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DF9644: 4BFFFE5D  bl 0x82df94a0
	ctx.lr = 0x82DF9648;
	sub_82DF94A0(ctx, base);
	// 82DF9648: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DF964C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF9650: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DF9654: 4BFFC7C5  bl 0x82df5e18
	ctx.lr = 0x82DF9658;
	sub_82DF5E18(ctx, base);
	// 82DF9658: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF965C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF9660: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF9664: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF9668: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DF966C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF9670: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF9678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF9678 size=80
    let mut pc: u32 = 0x82DF9678;
    'dispatch: loop {
        match pc {
            0x82DF9678 => {
    //   block [0x82DF9678..0x82DF96C8)
	// 82DF9678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF967C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF9680: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF9684: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF9688: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF968C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF9690: 4B4CFC61  bl 0x822c92f0
	ctx.lr = 0x82DF9694;
	sub_822C92F0(ctx, base);
	// 82DF9694: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF9698: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DF969C: 4BFFFF45  bl 0x82df95e0
	ctx.lr = 0x82DF96A0;
	sub_82DF95E0(ctx, base);
	// 82DF96A0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF96A4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DF96A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF96AC: 4B4CF785  bl 0x822c8e30
	ctx.lr = 0x82DF96B0;
	sub_822C8E30(ctx, base);
	// 82DF96B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF96B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DF96B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF96BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF96C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF96C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF96C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF96C8 size=192
    let mut pc: u32 = 0x82DF96C8;
    'dispatch: loop {
        match pc {
            0x82DF96C8 => {
    //   block [0x82DF96C8..0x82DF9788)
	// 82DF96C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF96CC: 483AEA9D  bl 0x831a8168
	ctx.lr = 0x82DF96D0;
	sub_831A8130(ctx, base);
	// 82DF96D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF96D4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DF96D8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82DF96DC: 3BBF0004  addi r29, r31, 4
	ctx.r[29].s64 = ctx.r[31].s64 + 4;
	// 82DF96E0: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DF96E4: 2B0B0008  cmplwi cr6, r11, 8
	ctx.cr[6].compare_u32(ctx.r[11].u32, 8 as u32, &mut ctx.xer);
	// 82DF96E8: 4198000C  blt cr6, 0x82df96f4
	if ctx.cr[6].lt {
	pc = 0x82DF96F4; continue 'dispatch;
	}
	// 82DF96EC: 80BF0004  lwz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF96F0: 48000008  b 0x82df96f8
	pc = 0x82DF96F8; continue 'dispatch;
	// 82DF96F4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DF96F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DF96FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82DF9700: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82DF9704: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82DF9708: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 82DF970C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DF9710: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DF9714: 483CB4A5  bl 0x831c4bb8
	ctx.lr = 0x82DF9718;
	sub_831C4BB8(ctx, base);
	// 82DF9718: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DF971C: 387E0001  addi r3, r30, 1
	ctx.r[3].s64 = ctx.r[30].s64 + 1;
	// 82DF9720: 4BFF6E61  bl 0x82df0580
	ctx.lr = 0x82DF9724;
	sub_82DF0580(ctx, base);
	// 82DF9724: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DF9728: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF972C: 2B0B0008  cmplwi cr6, r11, 8
	ctx.cr[6].compare_u32(ctx.r[11].u32, 8 as u32, &mut ctx.xer);
	// 82DF9730: 4198000C  blt cr6, 0x82df973c
	if ctx.cr[6].lt {
	pc = 0x82DF973C; continue 'dispatch;
	}
	// 82DF9734: 80BD0000  lwz r5, 0(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF9738: 48000008  b 0x82df9740
	pc = 0x82DF9740; continue 'dispatch;
	// 82DF973C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DF9740: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DF9744: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82DF9748: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 82DF974C: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 82DF9750: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 82DF9754: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DF9758: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DF975C: 483CB45D  bl 0x831c4bb8
	ctx.lr = 0x82DF9760;
	sub_831C4BB8(ctx, base);
	// 82DF9760: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF9764: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF9768: 7D7FF1AE  stbx r11, r31, r30
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32), ctx.r[11].u8) };
	// 82DF976C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DF9770: 4BFFA299  bl 0x82df3a08
	ctx.lr = 0x82DF9774;
	sub_82DF3A08(ctx, base);
	// 82DF9774: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF9778: 483AF491  bl 0x831a8c08
	ctx.lr = 0x82DF977C;
	sub_831A8C08(ctx, base);
	// 82DF977C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DF9780: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DF9784: 483AEA34  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF9788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF9788 size=176
    let mut pc: u32 = 0x82DF9788;
    'dispatch: loop {
        match pc {
            0x82DF9788 => {
    //   block [0x82DF9788..0x82DF9838)
	// 82DF9788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF978C: 483AE9DD  bl 0x831a8168
	ctx.lr = 0x82DF9790;
	sub_831A8130(ctx, base);
	// 82DF9790: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF9794: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DF9798: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DF979C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DF97A0: 4BFF9A11  bl 0x82df31b0
	ctx.lr = 0x82DF97A4;
	sub_82DF31B0(ctx, base);
	// 82DF97A4: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF97A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF97AC: 409A0018  bne cr6, 0x82df97c4
	if !ctx.cr[6].eq {
	pc = 0x82DF97C4; continue 'dispatch;
	}
	// 82DF97B0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82DF97B4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF97B8: 388B9D20  addi r4, r11, -0x62e0
	ctx.r[4].s64 = ctx.r[11].s64 + -25312;
	// 82DF97BC: 4B4CFB35  bl 0x822c92f0
	ctx.lr = 0x82DF97C0;
	sub_822C92F0(ctx, base);
	// 82DF97C0: 4800006C  b 0x82df982c
	pc = 0x82DF982C; continue 'dispatch;
	// 82DF97C4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DF97C8: 4BFF99E9  bl 0x82df31b0
	ctx.lr = 0x82DF97CC;
	sub_82DF31B0(ctx, base);
	// 82DF97CC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DF97D0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DF97D4: 38A003E8  li r5, 0x3e8
	ctx.r[5].s64 = 1000;
	// 82DF97D8: 4B4CA941  bl 0x822c4118
	ctx.lr = 0x82DF97DC;
	sub_822C4118(ctx, base);
	// 82DF97DC: 3D607FFF  lis r11, 0x7fff
	ctx.r[11].s64 = 2147418112;
	// 82DF97E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF97E4: 616BFFFF  ori r11, r11, 0xffff
	ctx.r[11].u64 = ctx.r[11].u64 | 65535;
	// 82DF97E8: 57E3083C  slwi r3, r31, 1
	ctx.r[3].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82DF97EC: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF97F0: 40990008  ble cr6, 0x82df97f8
	if !ctx.cr[6].gt {
	pc = 0x82DF97F8; continue 'dispatch;
	}
	// 82DF97F4: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82DF97F8: 4BFF6D89  bl 0x82df0580
	ctx.lr = 0x82DF97FC;
	sub_82DF0580(ctx, base);
	// 82DF97FC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82DF9800: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DF9804: 4BFF99AD  bl 0x82df31b0
	ctx.lr = 0x82DF9808;
	sub_82DF31B0(ctx, base);
	// 82DF9808: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DF980C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DF9810: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DF9814: 4B4CA905  bl 0x822c4118
	ctx.lr = 0x82DF9818;
	sub_822C4118(ctx, base);
	// 82DF9818: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DF981C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF9820: 4B4CFAD1  bl 0x822c92f0
	ctx.lr = 0x82DF9824;
	sub_822C92F0(ctx, base);
	// 82DF9824: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DF9828: 483AF3E1  bl 0x831a8c08
	ctx.lr = 0x82DF982C;
	sub_831A8C08(ctx, base);
	// 82DF982C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF9830: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DF9834: 483AE984  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF9838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF9838 size=128
    let mut pc: u32 = 0x82DF9838;
    'dispatch: loop {
        match pc {
            0x82DF9838 => {
    //   block [0x82DF9838..0x82DF98B8)
	// 82DF9838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF983C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF9840: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF9844: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF9848: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF984C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF9850: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF9854: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF9858: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF985C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DF9860: 419A0024  beq cr6, 0x82df9884
	if ctx.cr[6].eq {
	pc = 0x82DF9884; continue 'dispatch;
	}
	// 82DF9864: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82DF9868: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82DF986C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82DF9870: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82DF9874: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DF9878: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82DF987C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82DF9880: 4082FFE8  bne 0x82df9868
	if !ctx.cr[0].eq {
	pc = 0x82DF9868; continue 'dispatch;
	}
	// 82DF9884: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF9888: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DF988C: 419A0014  beq cr6, 0x82df98a0
	if ctx.cr[6].eq {
	pc = 0x82DF98A0; continue 'dispatch;
	}
	// 82DF9890: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF9894: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF9898: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DF989C: 4E800421  bctrl
	ctx.lr = 0x82DF98A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DF98A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF98A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF98A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF98AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF98B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF98B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF98B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF98B8 size=12
    let mut pc: u32 = 0x82DF98B8;
    'dispatch: loop {
        match pc {
            0x82DF98B8 => {
    //   block [0x82DF98B8..0x82DF98C4)
	// 82DF98B8: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF98BC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DF98C0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF98C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF98C4 size=8
    let mut pc: u32 = 0x82DF98C4;
    'dispatch: loop {
        match pc {
            0x82DF98C4 => {
    //   block [0x82DF98C4..0x82DF98CC)
	// 82DF98C4: 48000824  b 0x82dfa0e8
	sub_82DFA0E8(ctx, base);
	return;
	// 82DF98C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF98D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF98D0 size=24
    let mut pc: u32 = 0x82DF98D0;
    'dispatch: loop {
        match pc {
            0x82DF98D0 => {
    //   block [0x82DF98D0..0x82DF98E8)
	// 82DF98D0: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DF98D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DF98D8: 396BB2E0  addi r11, r11, -0x4d20
	ctx.r[11].s64 = ctx.r[11].s64 + -19744;
	// 82DF98DC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DF98E0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF98E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF98E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF98E8 size=136
    let mut pc: u32 = 0x82DF98E8;
    'dispatch: loop {
        match pc {
            0x82DF98E8 => {
    //   block [0x82DF98E8..0x82DF9970)
	// 82DF98E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF98EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF98F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DF98F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF98F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF98FC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DF9900: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DF9904: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82DF9908: 409A0020  bne cr6, 0x82df9928
	if !ctx.cr[6].eq {
	pc = 0x82DF9928; continue 'dispatch;
	}
	// 82DF990C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82DF9910: 419A0048  beq cr6, 0x82df9958
	if ctx.cr[6].eq {
	pc = 0x82DF9958; continue 'dispatch;
	}
	// 82DF9914: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF9918: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF991C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF9920: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DF9924: 48000034  b 0x82df9958
	pc = 0x82DF9958; continue 'dispatch;
	// 82DF9928: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 82DF992C: 419A002C  beq cr6, 0x82df9958
	if ctx.cr[6].eq {
	pc = 0x82DF9958; continue 'dispatch;
	}
	// 82DF9930: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82DF9934: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF9938: 388B9FD0  addi r4, r11, -0x6030
	ctx.r[4].s64 = ctx.r[11].s64 + -24624;
	// 82DF993C: 483AE7BD  bl 0x831a80f8
	ctx.lr = 0x82DF9940;
	sub_831A80F8(ctx, base);
	// 82DF9940: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF9944: 4182000C  beq 0x82df9950
	if ctx.cr[0].eq {
	pc = 0x82DF9950; continue 'dispatch;
	}
	// 82DF9948: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82DF994C: 4800000C  b 0x82df9958
	pc = 0x82DF9958; continue 'dispatch;
	// 82DF9950: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF9954: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF9958: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF995C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF9960: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF9964: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DF9968: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF996C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF9970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF9970 size=80
    let mut pc: u32 = 0x82DF9970;
    'dispatch: loop {
        match pc {
            0x82DF9970 => {
    //   block [0x82DF9970..0x82DF99C0)
	// 82DF9970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF9974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF9978: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF997C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF9980: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF9984: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82DF9988: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF998C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF9990: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DF9994: 4E800421  bctrl
	ctx.lr = 0x82DF9998;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DF9998: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DF999C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF99A0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF99A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DF99A8: 4E800421  bctrl
	ctx.lr = 0x82DF99AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DF99AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF99B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF99B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF99B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF99BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF99C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF99C0 size=24
    let mut pc: u32 = 0x82DF99C0;
    'dispatch: loop {
        match pc {
            0x82DF99C0 => {
    //   block [0x82DF99C0..0x82DF99D8)
	// 82DF99C0: 80630030  lwz r3, 0x30(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82DF99C4: 38800064  li r4, 0x64
	ctx.r[4].s64 = 100;
	// 82DF99C8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF99CC: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DF99D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DF99D4: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF99D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF99D8 size=8
    let mut pc: u32 = 0x82DF99D8;
    'dispatch: loop {
        match pc {
            0x82DF99D8 => {
    //   block [0x82DF99D8..0x82DF99E0)
	// 82DF99D8: 80630020  lwz r3, 0x20(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DF99DC: 4800041C  b 0x82df9df8
	sub_82DF9DF8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF99E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF99E0 size=196
    let mut pc: u32 = 0x82DF99E0;
    'dispatch: loop {
        match pc {
            0x82DF99E0 => {
    //   block [0x82DF99E0..0x82DF9AA4)
	// 82DF99E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF99E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF99E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DF99EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF99F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF99F4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DF99F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF99FC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82DF9A00: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DF9A04: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF9A08: 4B4C6F31  bl 0x822c0938
	ctx.lr = 0x82DF9A0C;
	sub_822C0938(ctx, base);
	// 82DF9A0C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DF9A10: 41820028  beq 0x82df9a38
	if ctx.cr[0].eq {
	pc = 0x82DF9A38; continue 'dispatch;
	}
	// 82DF9A14: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DF9A18: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82DF9A1C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DF9A20: 392BB790  addi r9, r11, -0x4870
	ctx.r[9].s64 = ctx.r[11].s64 + -18544;
	// 82DF9A24: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DF9A28: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DF9A2C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DF9A30: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82DF9A34: 48000008  b 0x82df9a3c
	pc = 0x82DF9A3C; continue 'dispatch;
	// 82DF9A38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF9A3C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF9A40: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF9A44: 409A0044  bne cr6, 0x82df9a88
	if !ctx.cr[6].eq {
	pc = 0x82DF9A88; continue 'dispatch;
	}
	// 82DF9A48: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82DF9A4C: 419A001C  beq cr6, 0x82df9a68
	if ctx.cr[6].eq {
	pc = 0x82DF9A68; continue 'dispatch;
	}
	// 82DF9A50: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF9A54: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DF9A58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF9A5C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF9A60: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DF9A64: 4E800421  bctrl
	ctx.lr = 0x82DF9A68;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DF9A68: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82DF9A6C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DF9A70: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF9A74: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82DF9A78: 816B9FC8  lwz r11, -0x6038(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24632 as u32) ) } as u64;
	// 82DF9A7C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82DF9A80: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82DF9A84: 4B4C657D  bl 0x822c0000
	ctx.lr = 0x82DF9A88;
	sub_822C0000(ctx, base);
	// 82DF9A88: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF9A8C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF9A90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF9A94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF9A98: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DF9A9C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF9AA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF9AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF9AA8 size=72
    let mut pc: u32 = 0x82DF9AA8;
    'dispatch: loop {
        match pc {
            0x82DF9AA8 => {
    //   block [0x82DF9AA8..0x82DF9AF0)
	// 82DF9AA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF9AAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF9AB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF9AB4: 2F050003  cmpwi cr6, r5, 3
	ctx.cr[6].compare_i32(ctx.r[5].s32, 3, &mut ctx.xer);
	// 82DF9AB8: 419A001C  beq cr6, 0x82df9ad4
	if ctx.cr[6].eq {
	pc = 0x82DF9AD4; continue 'dispatch;
	}
	// 82DF9ABC: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82DF9AC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DF9AC4: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82DF9AC8: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DF9ACC: 4BFFFE1D  bl 0x82df98e8
	ctx.lr = 0x82DF9AD0;
	sub_82DF98E8(ctx, base);
	// 82DF9AD0: 48000010  b 0x82df9ae0
	pc = 0x82DF9AE0; continue 'dispatch;
	// 82DF9AD4: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82DF9AD8: 396B9FD0  addi r11, r11, -0x6030
	ctx.r[11].s64 = ctx.r[11].s64 + -24624;
	// 82DF9ADC: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF9AE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF9AE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF9AE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF9AEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF9AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF9AF0 size=144
    let mut pc: u32 = 0x82DF9AF0;
    'dispatch: loop {
        match pc {
            0x82DF9AF0 => {
    //   block [0x82DF9AF0..0x82DF9B80)
	// 82DF9AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF9AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF9AF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DF9AFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF9B00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF9B04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF9B08: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DF9B0C: 9BDF0038  stb r30, 0x38(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[30].u8 ) };
	// 82DF9B10: 4BFFFE61  bl 0x82df9970
	ctx.lr = 0x82DF9B14;
	sub_82DF9970(ctx, base);
	// 82DF9B14: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 82DF9B18: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DF9B1C: 397F0020  addi r11, r31, 0x20
	ctx.r[11].s64 = ctx.r[31].s64 + 32;
	// 82DF9B20: 93DF0024  stw r30, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u32 ) };
	// 82DF9B24: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DF9B28: 419A0008  beq cr6, 0x82df9b30
	if ctx.cr[6].eq {
	pc = 0x82DF9B30; continue 'dispatch;
	}
	// 82DF9B2C: 4B4C6D65  bl 0x822c0890
	ctx.lr = 0x82DF9B30;
	sub_822C0890(ctx, base);
	// 82DF9B30: 807F0034  lwz r3, 0x34(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DF9B34: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DF9B38: 419A0008  beq cr6, 0x82df9b40
	if ctx.cr[6].eq {
	pc = 0x82DF9B40; continue 'dispatch;
	}
	// 82DF9B3C: 4B4C6D55  bl 0x822c0890
	ctx.lr = 0x82DF9B40;
	sub_822C0890(ctx, base);
	// 82DF9B40: 807F002C  lwz r3, 0x2c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DF9B44: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DF9B48: 419A0008  beq cr6, 0x82df9b50
	if ctx.cr[6].eq {
	pc = 0x82DF9B50; continue 'dispatch;
	}
	// 82DF9B4C: 4B4C6D45  bl 0x822c0890
	ctx.lr = 0x82DF9B50;
	sub_822C0890(ctx, base);
	// 82DF9B50: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DF9B54: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DF9B58: 419A0008  beq cr6, 0x82df9b60
	if ctx.cr[6].eq {
	pc = 0x82DF9B60; continue 'dispatch;
	}
	// 82DF9B5C: 4B4C6D35  bl 0x822c0890
	ctx.lr = 0x82DF9B60;
	sub_822C0890(ctx, base);
	// 82DF9B60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF9B64: 4B4CF155  bl 0x822c8cb8
	ctx.lr = 0x82DF9B68;
	sub_822C8CB8(ctx, base);
	// 82DF9B68: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF9B6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF9B70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF9B74: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DF9B78: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF9B7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF9B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF9B80 size=128
    let mut pc: u32 = 0x82DF9B80;
    'dispatch: loop {
        match pc {
            0x82DF9B80 => {
    //   block [0x82DF9B80..0x82DF9C00)
	// 82DF9B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF9B84: 483AE5E9  bl 0x831a816c
	ctx.lr = 0x82DF9B88;
	sub_831A8130(ctx, base);
	// 82DF9B88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF9B8C: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82DF9B90: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82DF9B94: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DF9B98: 3BEB6058  addi r31, r11, 0x6058
	ctx.r[31].s64 = ctx.r[11].s64 + 24664;
	// 82DF9B9C: 816A6060  lwz r11, 0x6060(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(24672 as u32) ) } as u64;
	// 82DF9BA0: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82DF9BA4: 40820024  bne 0x82df9bc8
	if !ctx.cr[0].eq {
	pc = 0x82DF9BC8; continue 'dispatch;
	}
	// 82DF9BA8: 3D2082E1  lis r9, -0x7d1f
	ctx.r[9].s64 = -2099183616;
	// 82DF9BAC: 3D0082E0  lis r8, -0x7d20
	ctx.r[8].s64 = -2099249152;
	// 82DF9BB0: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 82DF9BB4: 3929BD78  addi r9, r9, -0x4288
	ctx.r[9].s64 = ctx.r[9].s64 + -17032;
	// 82DF9BB8: 39089AA8  addi r8, r8, -0x6558
	ctx.r[8].s64 = ctx.r[8].s64 + -25944;
	// 82DF9BBC: 916A6060  stw r11, 0x6060(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(24672 as u32), ctx.r[11].u32 ) };
	// 82DF9BC0: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DF9BC4: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82DF9BC8: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82DF9BCC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82DF9BD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF9BD4: 38BE0008  addi r5, r30, 8
	ctx.r[5].s64 = ctx.r[30].s64 + 8;
	// 82DF9BD8: 9BAB0000  stb r29, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u8 ) };
	// 82DF9BDC: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DF9BE0: 48011B11  bl 0x82e0b6f0
	ctx.lr = 0x82DF9BE4;
	sub_82E0B6F0(ctx, base);
	// 82DF9BE4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF9BE8: 4182000C  beq 0x82df9bf4
	if ctx.cr[0].eq {
	pc = 0x82DF9BF4; continue 'dispatch;
	}
	// 82DF9BEC: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82DF9BF0: 48000008  b 0x82df9bf8
	pc = 0x82DF9BF8; continue 'dispatch;
	// 82DF9BF4: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82DF9BF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DF9BFC: 483AE5C0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF9C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF9C00 size=116
    let mut pc: u32 = 0x82DF9C00;
    'dispatch: loop {
        match pc {
            0x82DF9C00 => {
    //   block [0x82DF9C00..0x82DF9C74)
	// 82DF9C00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF9C04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF9C08: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF9C0C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF9C10: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF9C14: 48000040  b 0x82df9c54
	pc = 0x82DF9C54; continue 'dispatch;
	// 82DF9C18: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DF9C1C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF9C20: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DF9C24: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DF9C28: 4E800421  bctrl
	ctx.lr = 0x82DF9C2C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DF9C2C: 897F0038  lbz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82DF9C30: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DF9C34: 4182000C  beq 0x82df9c40
	if ctx.cr[0].eq {
	pc = 0x82DF9C40; continue 'dispatch;
	}
	// 82DF9C38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF9C3C: 4B659A45  bl 0x82453680
	ctx.lr = 0x82DF9C40;
	sub_82453680(ctx, base);
	// 82DF9C40: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82DF9C44: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF9C48: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF9C4C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DF9C50: 4E800421  bctrl
	ctx.lr = 0x82DF9C54;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DF9C54: 897F0038  lbz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82DF9C58: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DF9C5C: 4082FFBC  bne 0x82df9c18
	if !ctx.cr[0].eq {
	pc = 0x82DF9C18; continue 'dispatch;
	}
	// 82DF9C60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DF9C64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF9C68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF9C6C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF9C70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF9C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF9C78 size=356
    let mut pc: u32 = 0x82DF9C78;
    'dispatch: loop {
        match pc {
            0x82DF9C78 => {
    //   block [0x82DF9C78..0x82DF9DDC)
	// 82DF9C78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF9C7C: 483AE4E5  bl 0x831a8160
	ctx.lr = 0x82DF9C80;
	sub_831A8130(ctx, base);
	// 82DF9C80: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF9C84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF9C88: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82DF9C8C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DF9C90: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82DF9C94: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 82DF9C98: 4B792279  bl 0x8258bf10
	ctx.lr = 0x82DF9C9C;
	sub_8258BF10(ctx, base);
	// 82DF9C9C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DF9CA0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82DF9CA4: 388BB2F8  addi r4, r11, -0x4d08
	ctx.r[4].s64 = ctx.r[11].s64 + -19720;
	// 82DF9CA8: 38A0001B  li r5, 0x1b
	ctx.r[5].s64 = 27;
	// 82DF9CAC: 38600038  li r3, 0x38
	ctx.r[3].s64 = 56;
	// 82DF9CB0: 4BFF8739  bl 0x82df23e8
	ctx.lr = 0x82DF9CB4;
	sub_82DF23E8(ctx, base);
	// 82DF9CB4: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82DF9CB8: 41820048  beq 0x82df9d00
	if ctx.cr[0].eq {
	pc = 0x82DF9D00; continue 'dispatch;
	}
	// 82DF9CBC: 3D6082E0  lis r11, -0x7d20
	ctx.r[11].s64 = -2099249152;
	// 82DF9CC0: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82DF9CC4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DF9CC8: 396B9C00  addi r11, r11, -0x6400
	ctx.r[11].s64 = ctx.r[11].s64 + -25600;
	// 82DF9CCC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DF9CD0: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82DF9CD4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82DF9CD8: E8810050  ld r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82DF9CDC: 4BFFFEA5  bl 0x82df9b80
	ctx.lr = 0x82DF9CE0;
	sub_82DF9B80(ctx, base);
	// 82DF9CE0: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 82DF9CE4: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82DF9CE8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DF9CEC: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82DF9CF0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF9CF4: 48000255  bl 0x82df9f48
	ctx.lr = 0x82DF9CF8;
	sub_82DF9F48(ctx, base);
	// 82DF9CF8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DF9CFC: 48000008  b 0x82df9d04
	pc = 0x82DF9D04; continue 'dispatch;
	// 82DF9D00: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DF9D04: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 82DF9D08: 397F0020  addi r11, r31, 0x20
	ctx.r[11].s64 = ctx.r[31].s64 + 32;
	// 82DF9D0C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DF9D10: 3BAB0004  addi r29, r11, 4
	ctx.r[29].s64 = ctx.r[11].s64 + 4;
	// 82DF9D14: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DF9D18: 4B6F3791  bl 0x824ed4a8
	ctx.lr = 0x82DF9D1C;
	sub_824ED4A8(ctx, base);
	// 82DF9D1C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DF9D20: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DF9D24: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DF9D28: 4B4C62D9  bl 0x822c0000
	ctx.lr = 0x82DF9D2C;
	sub_822C0000(ctx, base);
	// 82DF9D2C: 3FA08338  lis r29, -0x7cc8
	ctx.r[29].s64 = -2093481984;
	// 82DF9D30: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DF9D34: 807D5F9C  lwz r3, 0x5f9c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24476 as u32) ) } as u64;
	// 82DF9D38: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF9D3C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF9D40: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DF9D44: 4E800421  bctrl
	ctx.lr = 0x82DF9D48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DF9D48: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82DF9D4C: 397F0028  addi r11, r31, 0x28
	ctx.r[11].s64 = ctx.r[31].s64 + 40;
	// 82DF9D50: 937F0028  stw r27, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[27].u32 ) };
	// 82DF9D54: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82DF9D58: 3BCB0004  addi r30, r11, 4
	ctx.r[30].s64 = ctx.r[11].s64 + 4;
	// 82DF9D5C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF9D60: 4BFFFC81  bl 0x82df99e0
	ctx.lr = 0x82DF9D64;
	sub_82DF99E0(ctx, base);
	// 82DF9D64: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82DF9D68: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82DF9D6C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF9D70: 4B4C6291  bl 0x822c0000
	ctx.lr = 0x82DF9D74;
	sub_822C0000(ctx, base);
	// 82DF9D74: 807D5F9C  lwz r3, 0x5f9c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24476 as u32) ) } as u64;
	// 82DF9D78: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DF9D7C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF9D80: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF9D84: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DF9D88: 4E800421  bctrl
	ctx.lr = 0x82DF9D8C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DF9D8C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DF9D90: 397F0030  addi r11, r31, 0x30
	ctx.r[11].s64 = ctx.r[31].s64 + 48;
	// 82DF9D94: 93BF0030  stw r29, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[29].u32 ) };
	// 82DF9D98: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DF9D9C: 3BCB0004  addi r30, r11, 4
	ctx.r[30].s64 = ctx.r[11].s64 + 4;
	// 82DF9DA0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF9DA4: 4BFFFC3D  bl 0x82df99e0
	ctx.lr = 0x82DF9DA8;
	sub_82DF99E0(ctx, base);
	// 82DF9DA8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DF9DAC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DF9DB0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF9DB4: 4B4C624D  bl 0x822c0000
	ctx.lr = 0x82DF9DB8;
	sub_822C0000(ctx, base);
	// 82DF9DB8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DF9DBC: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DF9DC0: 997F0038  stb r11, 0x38(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u8 ) };
	// 82DF9DC4: 482A4675  bl 0x8309e438
	ctx.lr = 0x82DF9DC8;
	sub_8309E438(ctx, base);
	// 82DF9DC8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DF9DCC: 4B4CEEED  bl 0x822c8cb8
	ctx.lr = 0x82DF9DD0;
	sub_822C8CB8(ctx, base);
	// 82DF9DD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF9DD4: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82DF9DD8: 483AE3D8  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF9DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF9DE0 size=20
    let mut pc: u32 = 0x82DF9DE0;
    'dispatch: loop {
        match pc {
            0x82DF9DE0 => {
    //   block [0x82DF9DE0..0x82DF9DF4)
	// 82DF9DE0: 90830000  stw r4, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82DF9DE4: 90A30004  stw r5, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82DF9DE8: B0C30008  sth r6, 8(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[6].u16 ) };
	// 82DF9DEC: 90E3000C  stw r7, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 82DF9DF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF9DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF9DF8 size=20
    let mut pc: u32 = 0x82DF9DF8;
    'dispatch: loop {
        match pc {
            0x82DF9DF8 => {
    //   block [0x82DF9DF8..0x82DF9E0C)
	// 82DF9DF8: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF9DFC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF9E00: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DF9E04: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DF9E08: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF9E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF9E10 size=196
    let mut pc: u32 = 0x82DF9E10;
    'dispatch: loop {
        match pc {
            0x82DF9E10 => {
    //   block [0x82DF9E10..0x82DF9ED4)
	// 82DF9E10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF9E14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF9E18: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DF9E1C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF9E20: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF9E24: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DF9E28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF9E2C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82DF9E30: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DF9E34: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF9E38: 4B4C6B01  bl 0x822c0938
	ctx.lr = 0x82DF9E3C;
	sub_822C0938(ctx, base);
	// 82DF9E3C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DF9E40: 41820028  beq 0x82df9e68
	if ctx.cr[0].eq {
	pc = 0x82DF9E68; continue 'dispatch;
	}
	// 82DF9E44: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DF9E48: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82DF9E4C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DF9E50: 392BB790  addi r9, r11, -0x4870
	ctx.r[9].s64 = ctx.r[11].s64 + -18544;
	// 82DF9E54: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DF9E58: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DF9E5C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DF9E60: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82DF9E64: 48000008  b 0x82df9e6c
	pc = 0x82DF9E6C; continue 'dispatch;
	// 82DF9E68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF9E6C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF9E70: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF9E74: 409A0044  bne cr6, 0x82df9eb8
	if !ctx.cr[6].eq {
	pc = 0x82DF9EB8; continue 'dispatch;
	}
	// 82DF9E78: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82DF9E7C: 419A001C  beq cr6, 0x82df9e98
	if ctx.cr[6].eq {
	pc = 0x82DF9E98; continue 'dispatch;
	}
	// 82DF9E80: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF9E84: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DF9E88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF9E8C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF9E90: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DF9E94: 4E800421  bctrl
	ctx.lr = 0x82DF9E98;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DF9E98: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82DF9E9C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DF9EA0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF9EA4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82DF9EA8: 816BA070  lwz r11, -0x5f90(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24464 as u32) ) } as u64;
	// 82DF9EAC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82DF9EB0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82DF9EB4: 4B4C614D  bl 0x822c0000
	ctx.lr = 0x82DF9EB8;
	sub_822C0000(ctx, base);
	// 82DF9EB8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF9EBC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF9EC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF9EC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF9EC8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DF9ECC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF9ED0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF9ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF9ED8 size=112
    let mut pc: u32 = 0x82DF9ED8;
    'dispatch: loop {
        match pc {
            0x82DF9ED8 => {
    //   block [0x82DF9ED8..0x82DF9F48)
	// 82DF9ED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF9EDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF9EE0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DF9EE4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF9EE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF9EEC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DF9EF0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF9EF4: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82DF9EF8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82DF9EFC: 4BFFFF15  bl 0x82df9e10
	ctx.lr = 0x82DF9F00;
	sub_82DF9E10(ctx, base);
	// 82DF9F00: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DF9F04: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DF9F08: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82DF9F0C: 4B4C60F5  bl 0x822c0000
	ctx.lr = 0x82DF9F10;
	sub_822C0000(ctx, base);
	// 82DF9F10: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DF9F14: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DF9F18: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF9F1C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF9F20: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DF9F24: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DF9F28: 419A0008  beq cr6, 0x82df9f30
	if ctx.cr[6].eq {
	pc = 0x82DF9F30; continue 'dispatch;
	}
	// 82DF9F2C: 4B4C6965  bl 0x822c0890
	ctx.lr = 0x82DF9F30;
	sub_822C0890(ctx, base);
	// 82DF9F30: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF9F34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF9F38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF9F3C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DF9F40: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF9F44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF9F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF9F48 size=168
    let mut pc: u32 = 0x82DF9F48;
    'dispatch: loop {
        match pc {
            0x82DF9F48 => {
    //   block [0x82DF9F48..0x82DF9FF0)
	// 82DF9F48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF9F4C: 483AE211  bl 0x831a815c
	ctx.lr = 0x82DF9F50;
	sub_831A8130(ctx, base);
	// 82DF9F50: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF9F54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF9F58: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DF9F5C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82DF9F60: 396BB334  addi r11, r11, -0x4ccc
	ctx.r[11].s64 = ctx.r[11].s64 + -19660;
	// 82DF9F64: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82DF9F68: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF9F6C: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 82DF9F70: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82DF9F74: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82DF9F78: 939F000C  stw r28, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 82DF9F7C: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82DF9F80: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 82DF9F84: 3BDF0008  addi r30, r31, 8
	ctx.r[30].s64 = ctx.r[31].s64 + 8;
	// 82DF9F88: 4B791F89  bl 0x8258bf10
	ctx.lr = 0x82DF9F8C;
	sub_8258BF10(ctx, base);
	// 82DF9F8C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82DF9F90: B39F0030  sth r28, 0x30(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[28].u16 ) };
	// 82DF9F94: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82DF9F98: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82DF9F9C: 419A0010  beq cr6, 0x82df9fac
	if ctx.cr[6].eq {
	pc = 0x82DF9FAC; continue 'dispatch;
	}
	// 82DF9FA0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF9FA4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DF9FA8: 48000010  b 0x82df9fb8
	pc = 0x82DF9FB8; continue 'dispatch;
	// 82DF9FAC: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82DF9FB0: 806B5F98  lwz r3, 0x5f98(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24472 as u32) ) } as u64;
	// 82DF9FB4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF9FB8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF9FBC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DF9FC0: 4E800421  bctrl
	ctx.lr = 0x82DF9FC4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DF9FC4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DF9FC8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF9FCC: 4BFFFF0D  bl 0x82df9ed8
	ctx.lr = 0x82DF9FD0;
	sub_82DF9ED8(ctx, base);
	// 82DF9FD0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF9FD4: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF9FD8: 48390A91  bl 0x8318aa68
	ctx.lr = 0x82DF9FDC;
	sub_8318AA68(ctx, base);
	// 82DF9FDC: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82DF9FE0: 4B4CECD9  bl 0x822c8cb8
	ctx.lr = 0x82DF9FE4;
	sub_822C8CB8(ctx, base);
	// 82DF9FE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF9FE8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DF9FEC: 483AE1C0  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF9FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF9FF0 size=244
    let mut pc: u32 = 0x82DF9FF0;
    'dispatch: loop {
        match pc {
            0x82DF9FF0 => {
    //   block [0x82DF9FF0..0x82DFA0E4)
	// 82DF9FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF9FF4: 483AE169  bl 0x831a815c
	ctx.lr = 0x82DF9FF8;
	sub_831A8130(ctx, base);
	// 82DF9FF8: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF9FFC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82DFA000: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DFA004: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 82DFA008: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82DFA00C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DFA010: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82DFA014: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82DFA018: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82DFA01C: 388BB338  addi r4, r11, -0x4cc8
	ctx.r[4].s64 = ctx.r[11].s64 + -19656;
	// 82DFA020: 38A000AE  li r5, 0xae
	ctx.r[5].s64 = 174;
	// 82DFA024: 38600038  li r3, 0x38
	ctx.r[3].s64 = 56;
	// 82DFA028: 4BFF83C1  bl 0x82df23e8
	ctx.lr = 0x82DFA02C;
	sub_82DF23E8(ctx, base);
	// 82DFA02C: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82DFA030: 4182004C  beq 0x82dfa07c
	if ctx.cr[0].eq {
	pc = 0x82DFA07C; continue 'dispatch;
	}
	// 82DFA034: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82DFA038: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFA03C: 4BFF99CD  bl 0x82df3a08
	ctx.lr = 0x82DFA040;
	sub_82DF3A08(ctx, base);
	// 82DFA040: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82DFA044: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DFA048: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 82DFA04C: 3B610050  addi r27, r1, 0x50
	ctx.r[27].s64 = ctx.r[1].s64 + 80;
	// 82DFA050: 835E000C  lwz r26, 0xc(r30)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DFA054: 833E0000  lwz r25, 0(r30)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFA058: 4B791EB9  bl 0x8258bf10
	ctx.lr = 0x82DFA05C;
	sub_8258BF10(ctx, base);
	// 82DFA05C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DFA060: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFA064: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82DFA068: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 82DFA06C: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 82DFA070: 4BFFFED9  bl 0x82df9f48
	ctx.lr = 0x82DFA074;
	sub_82DF9F48(ctx, base);
	// 82DFA074: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFA078: 48000008  b 0x82dfa080
	pc = 0x82DFA080; continue 'dispatch;
	// 82DFA07C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82DFA080: 93FC0000  stw r31, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82DFA084: 3B7C0004  addi r27, r28, 4
	ctx.r[27].s64 = ctx.r[28].s64 + 4;
	// 82DFA088: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DFA08C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82DFA090: 4B6F3419  bl 0x824ed4a8
	ctx.lr = 0x82DFA094;
	sub_824ED4A8(ctx, base);
	// 82DFA094: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DFA098: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DFA09C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82DFA0A0: 4B4C5F61  bl 0x822c0000
	ctx.lr = 0x82DFA0A4;
	sub_822C0000(ctx, base);
	// 82DFA0A4: 57AB07FF  clrlwi. r11, r29, 0x1f
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DFA0A8: 4182000C  beq 0x82dfa0b4
	if ctx.cr[0].eq {
	pc = 0x82DFA0B4; continue 'dispatch;
	}
	// 82DFA0AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFA0B0: 4BFF9379  bl 0x82df3428
	ctx.lr = 0x82DFA0B4;
	sub_82DF3428(ctx, base);
	// 82DFA0B4: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFA0B8: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFA0BC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFA0C0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFA0C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DFA0C8: 4E800421  bctrl
	ctx.lr = 0x82DFA0CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DFA0CC: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFA0D0: A15E0008  lhz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFA0D4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DFA0D8: B14B0030  sth r10, 0x30(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(48 as u32), ctx.r[10].u16 ) };
	// 82DFA0DC: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82DFA0E0: 483AE0CC  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFA0E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFA0E8 size=84
    let mut pc: u32 = 0x82DFA0E8;
    'dispatch: loop {
        match pc {
            0x82DFA0E8 => {
    //   block [0x82DFA0E8..0x82DFA13C)
	// 82DFA0E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFA0EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFA0F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFA0F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFA0F8: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DFA0FC: 3BE30010  addi r31, r3, 0x10
	ctx.r[31].s64 = ctx.r[3].s64 + 16;
	// 82DFA100: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DFA104: 419A0024  beq cr6, 0x82dfa128
	if ctx.cr[6].eq {
	pc = 0x82DFA128; continue 'dispatch;
	}
	// 82DFA108: 3D60822C  lis r11, -0x7dd4
	ctx.r[11].s64 = -2111045632;
	// 82DFA10C: 396B0000  addi r11, r11, 0
	ctx.r[11].s64 = ctx.r[11].s64 + 0;
	// 82DFA110: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DFA114: 419A0014  beq cr6, 0x82dfa128
	if ctx.cr[6].eq {
	pc = 0x82DFA128; continue 'dispatch;
	}
	// 82DFA118: A0630030  lhz r3, 0x30(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82DFA11C: 4BFF7D9D  bl 0x82df1eb8
	ctx.lr = 0x82DFA120;
	sub_82DF1EB8(ctx, base);
	// 82DFA120: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFA124: 4B65955D  bl 0x82453680
	ctx.lr = 0x82DFA128;
	sub_82453680(ctx, base);
	// 82DFA128: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DFA12C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFA130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFA134: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFA138: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFA140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFA140 size=80
    let mut pc: u32 = 0x82DFA140;
    'dispatch: loop {
        match pc {
            0x82DFA140 => {
    //   block [0x82DFA140..0x82DFA190)
	// 82DFA140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFA144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFA148: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFA14C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFA150: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82DFA154: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFA158: 816B5FE8  lwz r11, 0x5fe8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24552 as u32) ) } as u64;
	// 82DFA15C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DFA160: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 82DFA164: 409A0008  bne cr6, 0x82dfa16c
	if !ctx.cr[6].eq {
	pc = 0x82DFA16C; continue 'dispatch;
	}
	// 82DFA168: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DFA16C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DFA170: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFA174: 4BFF7A85  bl 0x82df1bf8
	ctx.lr = 0x82DFA178;
	sub_82DF1BF8(ctx, base);
	// 82DFA178: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFA17C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DFA180: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFA184: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFA188: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFA18C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFA190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFA190 size=112
    let mut pc: u32 = 0x82DFA190;
    'dispatch: loop {
        match pc {
            0x82DFA190 => {
    //   block [0x82DFA190..0x82DFA200)
	// 82DFA190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFA194: 483ADFD9  bl 0x831a816c
	ctx.lr = 0x82DFA198;
	sub_831A8130(ctx, base);
	// 82DFA198: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFA19C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DFA1A0: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82DFA1A4: 548B063F  clrlwi. r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DFA1A8: 41820038  beq 0x82dfa1e0
	if ctx.cr[0].eq {
	pc = 0x82DFA1E0; continue 'dispatch;
	}
	// 82DFA1AC: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DFA1B0: 2B0B0010  cmplwi cr6, r11, 0x10
	ctx.cr[6].compare_u32(ctx.r[11].u32, 16 as u32, &mut ctx.xer);
	// 82DFA1B4: 4198002C  blt cr6, 0x82dfa1e0
	if ctx.cr[6].lt {
	pc = 0x82DFA1E0; continue 'dispatch;
	}
	// 82DFA1B8: 83BE0004  lwz r29, 4(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFA1BC: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 82DFA1C0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82DFA1C4: 419A0014  beq cr6, 0x82dfa1d8
	if ctx.cr[6].eq {
	pc = 0x82DFA1D8; continue 'dispatch;
	}
	// 82DFA1C8: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82DFA1CC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DFA1D0: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82DFA1D4: 483AEA5D  bl 0x831a8c30
	ctx.lr = 0x82DFA1D8;
	sub_831A8C30(ctx, base);
	// 82DFA1D8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DFA1DC: 4BFF7CD5  bl 0x82df1eb0
	ctx.lr = 0x82DFA1E0;
	sub_82DF1EB0(ctx, base);
	// 82DFA1E0: 397E0004  addi r11, r30, 4
	ctx.r[11].s64 = ctx.r[30].s64 + 4;
	// 82DFA1E4: 93FE0014  stw r31, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[31].u32 ) };
	// 82DFA1E8: 3940000F  li r10, 0xf
	ctx.r[10].s64 = 15;
	// 82DFA1EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82DFA1F0: 915E0018  stw r10, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 82DFA1F4: 7D2BF9AE  stbx r9, r11, r31
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[9].u8) };
	// 82DFA1F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DFA1FC: 483ADFC0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFA200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFA200 size=188
    let mut pc: u32 = 0x82DFA200;
    'dispatch: loop {
        match pc {
            0x82DFA200 => {
    //   block [0x82DFA200..0x82DFA2BC)
	// 82DFA200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFA204: 483ADF65  bl 0x831a8168
	ctx.lr = 0x82DFA208;
	sub_831A8130(ctx, base);
	// 82DFA208: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFA20C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFA210: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DFA214: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DFA218: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DFA21C: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82DFA220: 40980008  bge cr6, 0x82dfa228
	if !ctx.cr[6].lt {
	pc = 0x82DFA228; continue 'dispatch;
	}
	// 82DFA224: 4BFF6655  bl 0x82df0878
	ctx.lr = 0x82DFA228;
	sub_82DF0878(ctx, base);
	// 82DFA228: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DFA22C: 7D3D5850  subf r9, r29, r11
	ctx.r[9].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 82DFA230: 7F09F040  cmplw cr6, r9, r30
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82DFA234: 40980008  bge cr6, 0x82dfa23c
	if !ctx.cr[6].lt {
	pc = 0x82DFA23C; continue 'dispatch;
	}
	// 82DFA238: 7D3E4B78  mr r30, r9
	ctx.r[30].u64 = ctx.r[9].u64;
	// 82DFA23C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82DFA240: 419A0070  beq cr6, 0x82dfa2b0
	if ctx.cr[6].eq {
	pc = 0x82DFA2B0; continue 'dispatch;
	}
	// 82DFA244: 811F0018  lwz r8, 0x18(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DFA248: 3B9F0004  addi r28, r31, 4
	ctx.r[28].s64 = ctx.r[31].s64 + 4;
	// 82DFA24C: 2B080010  cmplwi cr6, r8, 0x10
	ctx.cr[6].compare_u32(ctx.r[8].u32, 16 as u32, &mut ctx.xer);
	// 82DFA250: 4198000C  blt cr6, 0x82dfa25c
	if ctx.cr[6].lt {
	pc = 0x82DFA25C; continue 'dispatch;
	}
	// 82DFA254: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFA258: 48000008  b 0x82dfa260
	pc = 0x82DFA260; continue 'dispatch;
	// 82DFA25C: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 82DFA260: 2B080010  cmplwi cr6, r8, 0x10
	ctx.cr[6].compare_u32(ctx.r[8].u32, 16 as u32, &mut ctx.xer);
	// 82DFA264: 4198000C  blt cr6, 0x82dfa270
	if ctx.cr[6].lt {
	pc = 0x82DFA270; continue 'dispatch;
	}
	// 82DFA268: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFA26C: 48000008  b 0x82dfa274
	pc = 0x82DFA274; continue 'dispatch;
	// 82DFA270: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 82DFA274: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82DFA278: 7CDE4850  subf r6, r30, r9
	ctx.r[6].s64 = ctx.r[9].s64 - ctx.r[30].s64;
	// 82DFA27C: 7CABF214  add r5, r11, r30
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82DFA280: 7C9D4050  subf r4, r29, r8
	ctx.r[4].s64 = ctx.r[8].s64 - ctx.r[29].s64;
	// 82DFA284: 7C6AEA14  add r3, r10, r29
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[29].u64;
	// 82DFA288: 483AEA81  bl 0x831a8d08
	ctx.lr = 0x82DFA28C;
	sub_831A8D08(ctx, base);
	// 82DFA28C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DFA290: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DFA294: 7D7E5850  subf r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 82DFA298: 2B0A0010  cmplwi cr6, r10, 0x10
	ctx.cr[6].compare_u32(ctx.r[10].u32, 16 as u32, &mut ctx.xer);
	// 82DFA29C: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82DFA2A0: 41980008  blt cr6, 0x82dfa2a8
	if ctx.cr[6].lt {
	pc = 0x82DFA2A8; continue 'dispatch;
	}
	// 82DFA2A4: 839C0000  lwz r28, 0(r28)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFA2A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DFA2AC: 7D5C59AE  stbx r10, r28, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[28].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u8) };
	// 82DFA2B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFA2B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DFA2B8: 483ADF00  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFA2C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFA2C0 size=208
    let mut pc: u32 = 0x82DFA2C0;
    'dispatch: loop {
        match pc {
            0x82DFA2C0 => {
    //   block [0x82DFA2C0..0x82DFA390)
	// 82DFA2C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFA2C4: 483ADEA1  bl 0x831a8164
	ctx.lr = 0x82DFA2C8;
	sub_831A8130(ctx, base);
	// 82DFA2C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFA2CC: 609F000F  ori r31, r4, 0xf
	ctx.r[31].u64 = ctx.r[4].u64 | 15;
	// 82DFA2D0: 3960FFFE  li r11, -2
	ctx.r[11].s64 = -2;
	// 82DFA2D4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DFA2D8: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82DFA2DC: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DFA2E0: 4099000C  ble cr6, 0x82dfa2ec
	if !ctx.cr[6].gt {
	pc = 0x82DFA2EC; continue 'dispatch;
	}
	// 82DFA2E4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DFA2E8: 4800002C  b 0x82dfa314
	pc = 0x82DFA314; continue 'dispatch;
	// 82DFA2EC: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DFA2F0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82DFA2F4: 556AF87E  srwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DFA2F8: 7D3F4B96  divwu r9, r31, r9
	ctx.r[9].u32 = ctx.r[31].u32 / ctx.r[9].u32;
	// 82DFA2FC: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DFA300: 40980014  bge cr6, 0x82dfa314
	if !ctx.cr[6].lt {
	pc = 0x82DFA314; continue 'dispatch;
	}
	// 82DFA304: 212AFFFE  subfic r9, r10, -2
	ctx.xer.ca = ctx.r[10].u32 <= -2 as u32;
	ctx.r[9].s64 = (-2 as i64) - ctx.r[10].s64;
	// 82DFA308: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DFA30C: 41990008  bgt cr6, 0x82dfa314
	if ctx.cr[6].gt {
	pc = 0x82DFA314; continue 'dispatch;
	}
	// 82DFA310: 7FEA5A14  add r31, r10, r11
	ctx.r[31].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DFA314: 3B9F0001  addi r28, r31, 1
	ctx.r[28].s64 = ctx.r[31].s64 + 1;
	// 82DFA318: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DFA31C: 4BFF7B8D  bl 0x82df1ea8
	ctx.lr = 0x82DFA320;
	sub_82DF1EA8(ctx, base);
	// 82DFA320: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DFA324: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82DFA328: 419A002C  beq cr6, 0x82dfa354
	if ctx.cr[6].eq {
	pc = 0x82DFA354; continue 'dispatch;
	}
	// 82DFA32C: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DFA330: 2B0B0010  cmplwi cr6, r11, 0x10
	ctx.cr[6].compare_u32(ctx.r[11].u32, 16 as u32, &mut ctx.xer);
	// 82DFA334: 4198000C  blt cr6, 0x82dfa340
	if ctx.cr[6].lt {
	pc = 0x82DFA340; continue 'dispatch;
	}
	// 82DFA338: 80BE0004  lwz r5, 4(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFA33C: 48000008  b 0x82dfa344
	pc = 0x82DFA344; continue 'dispatch;
	// 82DFA340: 38BE0004  addi r5, r30, 4
	ctx.r[5].s64 = ctx.r[30].s64 + 4;
	// 82DFA344: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82DFA348: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DFA34C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DFA350: 483AE8E1  bl 0x831a8c30
	ctx.lr = 0x82DFA354;
	sub_831A8C30(ctx, base);
	// 82DFA354: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DFA358: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DFA35C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DFA360: 4BFFFE31  bl 0x82dfa190
	ctx.lr = 0x82DFA364;
	sub_82DFA190(ctx, base);
	// 82DFA364: 397E0004  addi r11, r30, 4
	ctx.r[11].s64 = ctx.r[30].s64 + 4;
	// 82DFA368: 93FE0018  stw r31, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[31].u32 ) };
	// 82DFA36C: 2B1F0010  cmplwi cr6, r31, 0x10
	ctx.cr[6].compare_u32(ctx.r[31].u32, 16 as u32, &mut ctx.xer);
	// 82DFA370: 937E0014  stw r27, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[27].u32 ) };
	// 82DFA374: 93BE0004  stw r29, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82DFA378: 41980008  blt cr6, 0x82dfa380
	if ctx.cr[6].lt {
	pc = 0x82DFA380; continue 'dispatch;
	}
	// 82DFA37C: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82DFA380: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DFA384: 7D4BD9AE  stbx r10, r11, r27
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[27].u32), ctx.r[10].u8) };
	// 82DFA388: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DFA38C: 483ADE28  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFA390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DFA390 size=184
    let mut pc: u32 = 0x82DFA390;
    'dispatch: loop {
        match pc {
            0x82DFA390 => {
    //   block [0x82DFA390..0x82DFA448)
	// 82DFA390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFA394: 483ADDD5  bl 0x831a8168
	ctx.lr = 0x82DFA398;
	sub_831A8130(ctx, base);
	// 82DFA398: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFA39C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DFA3A0: 3960FFFE  li r11, -2
	ctx.r[11].s64 = -2;
	// 82DFA3A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFA3A8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DFA3AC: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DFA3B0: 40990008  ble cr6, 0x82dfa3b8
	if !ctx.cr[6].gt {
	pc = 0x82DFA3B8; continue 'dispatch;
	}
	// 82DFA3B4: 4BFF638D  bl 0x82df0740
	ctx.lr = 0x82DFA3B8;
	sub_82DF0740(ctx, base);
	// 82DFA3B8: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DFA3BC: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82DFA3C0: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82DFA3C4: 40980018  bge cr6, 0x82dfa3dc
	if !ctx.cr[6].lt {
	pc = 0x82DFA3DC; continue 'dispatch;
	}
	// 82DFA3C8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DFA3CC: 80BF0014  lwz r5, 0x14(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DFA3D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFA3D4: 4BFFFEED  bl 0x82dfa2c0
	ctx.lr = 0x82DFA3D8;
	sub_82DFA2C0(ctx, base);
	// 82DFA3D8: 4800005C  b 0x82dfa434
	pc = 0x82DFA434; continue 'dispatch;
	// 82DFA3DC: 57AA063F  clrlwi. r10, r29, 0x18
	ctx.r[10].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DFA3E0: 41820030  beq 0x82dfa410
	if ctx.cr[0].eq {
	pc = 0x82DFA410; continue 'dispatch;
	}
	// 82DFA3E4: 2B1E0010  cmplwi cr6, r30, 0x10
	ctx.cr[6].compare_u32(ctx.r[30].u32, 16 as u32, &mut ctx.xer);
	// 82DFA3E8: 40980028  bge cr6, 0x82dfa410
	if !ctx.cr[6].lt {
	pc = 0x82DFA410; continue 'dispatch;
	}
	// 82DFA3EC: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DFA3F0: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DFA3F4: 40980008  bge cr6, 0x82dfa3fc
	if !ctx.cr[6].lt {
	pc = 0x82DFA3FC; continue 'dispatch;
	}
	// 82DFA3F8: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82DFA3FC: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82DFA400: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DFA404: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFA408: 4BFFFD89  bl 0x82dfa190
	ctx.lr = 0x82DFA40C;
	sub_82DFA190(ctx, base);
	// 82DFA40C: 48000028  b 0x82dfa434
	pc = 0x82DFA434; continue 'dispatch;
	// 82DFA410: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82DFA414: 409A0020  bne cr6, 0x82dfa434
	if !ctx.cr[6].eq {
	pc = 0x82DFA434; continue 'dispatch;
	}
	// 82DFA418: 2B0B0010  cmplwi cr6, r11, 0x10
	ctx.cr[6].compare_u32(ctx.r[11].u32, 16 as u32, &mut ctx.xer);
	// 82DFA41C: 939F0014  stw r28, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[28].u32 ) };
	// 82DFA420: 4198000C  blt cr6, 0x82dfa42c
	if ctx.cr[6].lt {
	pc = 0x82DFA42C; continue 'dispatch;
	}
	// 82DFA424: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFA428: 48000008  b 0x82dfa430
	pc = 0x82DFA430; continue 'dispatch;
	// 82DFA42C: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 82DFA430: 9B8B0000  stb r28, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u8 ) };
	// 82DFA434: 7D7EE010  subfc r11, r30, r28
	ctx.xer.ca = ctx.r[28].u32 >= ctx.r[30].u32;
	ctx.r[11].s64 = ctx.r[28].s64 - ctx.r[30].s64;
	// 82DFA438: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82DFA43C: 556307FE  clrlwi r3, r11, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82DFA440: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DFA444: 483ADD74  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFA448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFA448 size=252
    let mut pc: u32 = 0x82DFA448;
    'dispatch: loop {
        match pc {
            0x82DFA448 => {
    //   block [0x82DFA448..0x82DFA544)
	// 82DFA448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFA44C: 483ADD19  bl 0x831a8164
	ctx.lr = 0x82DFA450;
	sub_831A8130(ctx, base);
	// 82DFA450: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFA454: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DFA458: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82DFA45C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFA460: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82DFA464: 817D0014  lwz r11, 0x14(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DFA468: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82DFA46C: 40980008  bge cr6, 0x82dfa474
	if !ctx.cr[6].lt {
	pc = 0x82DFA474; continue 'dispatch;
	}
	// 82DFA470: 4BFF6409  bl 0x82df0878
	ctx.lr = 0x82DFA474;
	sub_82DF0878(ctx, base);
	// 82DFA474: 817D0014  lwz r11, 0x14(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DFA478: 7D7B5850  subf r11, r27, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[27].s64;
	// 82DFA47C: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82DFA480: 40980008  bge cr6, 0x82dfa488
	if !ctx.cr[6].lt {
	pc = 0x82DFA488; continue 'dispatch;
	}
	// 82DFA484: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82DFA488: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DFA48C: 214BFFFF  subfic r10, r11, -1
	ctx.xer.ca = ctx.r[11].u32 <= -1 as u32;
	ctx.r[10].s64 = (-1 as i64) - ctx.r[11].s64;
	// 82DFA490: 7F0AF040  cmplw cr6, r10, r30
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82DFA494: 40990010  ble cr6, 0x82dfa4a4
	if !ctx.cr[6].gt {
	pc = 0x82DFA4A4; continue 'dispatch;
	}
	// 82DFA498: 7D4BF214  add r10, r11, r30
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82DFA49C: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DFA4A0: 40980008  bge cr6, 0x82dfa4a8
	if !ctx.cr[6].lt {
	pc = 0x82DFA4A8; continue 'dispatch;
	}
	// 82DFA4A4: 4BFF629D  bl 0x82df0740
	ctx.lr = 0x82DFA4A8;
	sub_82DF0740(ctx, base);
	// 82DFA4A8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82DFA4AC: 419A008C  beq cr6, 0x82dfa538
	if ctx.cr[6].eq {
	pc = 0x82DFA538; continue 'dispatch;
	}
	// 82DFA4B0: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DFA4B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DFA4B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFA4BC: 7F8BF214  add r28, r11, r30
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82DFA4C0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DFA4C4: 4BFFFECD  bl 0x82dfa390
	ctx.lr = 0x82DFA4C8;
	sub_82DFA390(ctx, base);
	// 82DFA4C8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DFA4CC: 4182006C  beq 0x82dfa538
	if ctx.cr[0].eq {
	pc = 0x82DFA538; continue 'dispatch;
	}
	// 82DFA4D0: 817D0018  lwz r11, 0x18(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DFA4D4: 2B0B0010  cmplwi cr6, r11, 0x10
	ctx.cr[6].compare_u32(ctx.r[11].u32, 16 as u32, &mut ctx.xer);
	// 82DFA4D8: 4198000C  blt cr6, 0x82dfa4e4
	if ctx.cr[6].lt {
	pc = 0x82DFA4E4; continue 'dispatch;
	}
	// 82DFA4DC: 811D0004  lwz r8, 4(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFA4E0: 48000008  b 0x82dfa4e8
	pc = 0x82DFA4E8; continue 'dispatch;
	// 82DFA4E4: 391D0004  addi r8, r29, 4
	ctx.r[8].s64 = ctx.r[29].s64 + 4;
	// 82DFA4E8: 813F0018  lwz r9, 0x18(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DFA4EC: 3BBF0004  addi r29, r31, 4
	ctx.r[29].s64 = ctx.r[31].s64 + 4;
	// 82DFA4F0: 2B090010  cmplwi cr6, r9, 0x10
	ctx.cr[6].compare_u32(ctx.r[9].u32, 16 as u32, &mut ctx.xer);
	// 82DFA4F4: 4198000C  blt cr6, 0x82dfa500
	if ctx.cr[6].lt {
	pc = 0x82DFA500; continue 'dispatch;
	}
	// 82DFA4F8: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFA4FC: 48000008  b 0x82dfa504
	pc = 0x82DFA504; continue 'dispatch;
	// 82DFA500: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 82DFA504: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DFA508: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82DFA50C: 7CA8DA14  add r5, r8, r27
	ctx.r[5].u64 = ctx.r[8].u64 + ctx.r[27].u64;
	// 82DFA510: 7C8B4850  subf r4, r11, r9
	ctx.r[4].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 82DFA514: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DFA518: 483AE719  bl 0x831a8c30
	ctx.lr = 0x82DFA51C;
	sub_831A8C30(ctx, base);
	// 82DFA51C: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DFA520: 939F0014  stw r28, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[28].u32 ) };
	// 82DFA524: 2B0B0010  cmplwi cr6, r11, 0x10
	ctx.cr[6].compare_u32(ctx.r[11].u32, 16 as u32, &mut ctx.xer);
	// 82DFA528: 41980008  blt cr6, 0x82dfa530
	if ctx.cr[6].lt {
	pc = 0x82DFA530; continue 'dispatch;
	}
	// 82DFA52C: 83BD0000  lwz r29, 0(r29)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFA530: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DFA534: 7D7DE1AE  stbx r11, r29, r28
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32), ctx.r[11].u8) };
	// 82DFA538: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFA53C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DFA540: 483ADC74  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFA548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFA548 size=232
    let mut pc: u32 = 0x82DFA548;
    'dispatch: loop {
        match pc {
            0x82DFA548 => {
    //   block [0x82DFA548..0x82DFA630)
	// 82DFA548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFA54C: 483ADC19  bl 0x831a8164
	ctx.lr = 0x82DFA550;
	sub_831A8130(ctx, base);
	// 82DFA550: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFA554: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DFA558: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82DFA55C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFA560: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82DFA564: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DFA568: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82DFA56C: 40980008  bge cr6, 0x82dfa574
	if !ctx.cr[6].lt {
	pc = 0x82DFA574; continue 'dispatch;
	}
	// 82DFA570: 4BFF6309  bl 0x82df0878
	ctx.lr = 0x82DFA574;
	sub_82DF0878(ctx, base);
	// 82DFA574: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DFA578: 7FBB5850  subf r29, r27, r11
	ctx.r[29].s64 = ctx.r[11].s64 - ctx.r[27].s64;
	// 82DFA57C: 7F1CE840  cmplw cr6, r28, r29
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82DFA580: 40980008  bge cr6, 0x82dfa588
	if !ctx.cr[6].lt {
	pc = 0x82DFA588; continue 'dispatch;
	}
	// 82DFA584: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 82DFA588: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82DFA58C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFA590: 409A0024  bne cr6, 0x82dfa5b4
	if !ctx.cr[6].eq {
	pc = 0x82DFA5B4; continue 'dispatch;
	}
	// 82DFA594: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82DFA598: 7C9DDA14  add r4, r29, r27
	ctx.r[4].u64 = ctx.r[29].u64 + ctx.r[27].u64;
	// 82DFA59C: 4BFFFC65  bl 0x82dfa200
	ctx.lr = 0x82DFA5A0;
	sub_82DFA200(ctx, base);
	// 82DFA5A0: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82DFA5A4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DFA5A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFA5AC: 4BFFFC55  bl 0x82dfa200
	ctx.lr = 0x82DFA5B0;
	sub_82DFA200(ctx, base);
	// 82DFA5B0: 48000074  b 0x82dfa624
	pc = 0x82DFA624; continue 'dispatch;
	// 82DFA5B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DFA5B8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DFA5BC: 4BFFFDD5  bl 0x82dfa390
	ctx.lr = 0x82DFA5C0;
	sub_82DFA390(ctx, base);
	// 82DFA5C0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DFA5C4: 41820060  beq 0x82dfa624
	if ctx.cr[0].eq {
	pc = 0x82DFA624; continue 'dispatch;
	}
	// 82DFA5C8: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DFA5CC: 2B0B0010  cmplwi cr6, r11, 0x10
	ctx.cr[6].compare_u32(ctx.r[11].u32, 16 as u32, &mut ctx.xer);
	// 82DFA5D0: 4198000C  blt cr6, 0x82dfa5dc
	if ctx.cr[6].lt {
	pc = 0x82DFA5DC; continue 'dispatch;
	}
	// 82DFA5D4: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFA5D8: 48000008  b 0x82dfa5e0
	pc = 0x82DFA5E0; continue 'dispatch;
	// 82DFA5DC: 397E0004  addi r11, r30, 4
	ctx.r[11].s64 = ctx.r[30].s64 + 4;
	// 82DFA5E0: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DFA5E4: 3BDF0004  addi r30, r31, 4
	ctx.r[30].s64 = ctx.r[31].s64 + 4;
	// 82DFA5E8: 2B040010  cmplwi cr6, r4, 0x10
	ctx.cr[6].compare_u32(ctx.r[4].u32, 16 as u32, &mut ctx.xer);
	// 82DFA5EC: 4198000C  blt cr6, 0x82dfa5f8
	if ctx.cr[6].lt {
	pc = 0x82DFA5F8; continue 'dispatch;
	}
	// 82DFA5F0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFA5F4: 48000008  b 0x82dfa5fc
	pc = 0x82DFA5FC; continue 'dispatch;
	// 82DFA5F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DFA5FC: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82DFA600: 7CABDA14  add r5, r11, r27
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 82DFA604: 483AE62D  bl 0x831a8c30
	ctx.lr = 0x82DFA608;
	sub_831A8C30(ctx, base);
	// 82DFA608: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DFA60C: 93BF0014  stw r29, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[29].u32 ) };
	// 82DFA610: 2B0B0010  cmplwi cr6, r11, 0x10
	ctx.cr[6].compare_u32(ctx.r[11].u32, 16 as u32, &mut ctx.xer);
	// 82DFA614: 41980008  blt cr6, 0x82dfa61c
	if ctx.cr[6].lt {
	pc = 0x82DFA61C; continue 'dispatch;
	}
	// 82DFA618: 83DE0000  lwz r30, 0(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFA61C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DFA620: 7D7EE9AE  stbx r11, r30, r29
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[29].u32), ctx.r[11].u8) };
	// 82DFA624: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFA628: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DFA62C: 483ADB88  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFA630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFA630 size=88
    let mut pc: u32 = 0x82DFA630;
    'dispatch: loop {
        match pc {
            0x82DFA630 => {
    //   block [0x82DFA630..0x82DFA688)
	// 82DFA630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFA634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFA638: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DFA63C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFA640: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFA644: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DFA648: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DFA64C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DFA650: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFA654: 4BFFFB3D  bl 0x82dfa190
	ctx.lr = 0x82DFA658;
	sub_82DFA190(ctx, base);
	// 82DFA658: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFA65C: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 82DFA660: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DFA664: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DFA668: 4BFFFEE1  bl 0x82dfa548
	ctx.lr = 0x82DFA66C;
	sub_82DFA548(ctx, base);
	// 82DFA66C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFA670: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DFA674: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFA678: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFA67C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DFA680: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFA684: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFA688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFA688 size=264
    let mut pc: u32 = 0x82DFA688;
    'dispatch: loop {
        match pc {
            0x82DFA688 => {
    //   block [0x82DFA688..0x82DFA790)
	// 82DFA688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFA68C: 483ADADD  bl 0x831a8168
	ctx.lr = 0x82DFA690;
	sub_831A8130(ctx, base);
	// 82DFA690: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFA694: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFA698: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DFA69C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82DFA6A0: 3BDF0004  addi r30, r31, 4
	ctx.r[30].s64 = ctx.r[31].s64 + 4;
	// 82DFA6A4: 813F0018  lwz r9, 0x18(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DFA6A8: 2B090010  cmplwi cr6, r9, 0x10
	ctx.cr[6].compare_u32(ctx.r[9].u32, 16 as u32, &mut ctx.xer);
	// 82DFA6AC: 4198000C  blt cr6, 0x82dfa6b8
	if ctx.cr[6].lt {
	pc = 0x82DFA6B8; continue 'dispatch;
	}
	// 82DFA6B0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFA6B4: 48000008  b 0x82dfa6bc
	pc = 0x82DFA6BC; continue 'dispatch;
	// 82DFA6B8: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82DFA6BC: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DFA6C0: 4198002C  blt cr6, 0x82dfa6ec
	if ctx.cr[6].lt {
	pc = 0x82DFA6EC; continue 'dispatch;
	}
	// 82DFA6C4: 2B090010  cmplwi cr6, r9, 0x10
	ctx.cr[6].compare_u32(ctx.r[9].u32, 16 as u32, &mut ctx.xer);
	// 82DFA6C8: 4198000C  blt cr6, 0x82dfa6d4
	if ctx.cr[6].lt {
	pc = 0x82DFA6D4; continue 'dispatch;
	}
	// 82DFA6CC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFA6D0: 48000008  b 0x82dfa6d8
	pc = 0x82DFA6D8; continue 'dispatch;
	// 82DFA6D4: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82DFA6D8: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DFA6DC: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DFA6E0: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82DFA6E4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DFA6E8: 41990008  bgt cr6, 0x82dfa6f0
	if ctx.cr[6].gt {
	pc = 0x82DFA6F0; continue 'dispatch;
	}
	// 82DFA6EC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DFA6F0: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DFA6F4: 41820030  beq 0x82dfa724
	if ctx.cr[0].eq {
	pc = 0x82DFA724; continue 'dispatch;
	}
	// 82DFA6F8: 2B090010  cmplwi cr6, r9, 0x10
	ctx.cr[6].compare_u32(ctx.r[9].u32, 16 as u32, &mut ctx.xer);
	// 82DFA6FC: 4198000C  blt cr6, 0x82dfa708
	if ctx.cr[6].lt {
	pc = 0x82DFA708; continue 'dispatch;
	}
	// 82DFA700: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFA704: 48000008  b 0x82dfa70c
	pc = 0x82DFA70C; continue 'dispatch;
	// 82DFA708: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82DFA70C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DFA710: 7CABE850  subf r5, r11, r29
	ctx.r[5].s64 = ctx.r[29].s64 - ctx.r[11].s64;
	// 82DFA714: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DFA718: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFA71C: 4BFFFE2D  bl 0x82dfa548
	ctx.lr = 0x82DFA720;
	sub_82DFA548(ctx, base);
	// 82DFA720: 48000068  b 0x82dfa788
	pc = 0x82DFA788; continue 'dispatch;
	// 82DFA724: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DFA728: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DFA72C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFA730: 4BFFFC61  bl 0x82dfa390
	ctx.lr = 0x82DFA734;
	sub_82DFA390(ctx, base);
	// 82DFA734: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DFA738: 4182004C  beq 0x82dfa784
	if ctx.cr[0].eq {
	pc = 0x82DFA784; continue 'dispatch;
	}
	// 82DFA73C: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DFA740: 2B040010  cmplwi cr6, r4, 0x10
	ctx.cr[6].compare_u32(ctx.r[4].u32, 16 as u32, &mut ctx.xer);
	// 82DFA744: 4198000C  blt cr6, 0x82dfa750
	if ctx.cr[6].lt {
	pc = 0x82DFA750; continue 'dispatch;
	}
	// 82DFA748: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFA74C: 48000008  b 0x82dfa754
	pc = 0x82DFA754; continue 'dispatch;
	// 82DFA750: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DFA754: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DFA758: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DFA75C: 483AE4D5  bl 0x831a8c30
	ctx.lr = 0x82DFA760;
	sub_831A8C30(ctx, base);
	// 82DFA760: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DFA764: 939F0014  stw r28, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[28].u32 ) };
	// 82DFA768: 2B0B0010  cmplwi cr6, r11, 0x10
	ctx.cr[6].compare_u32(ctx.r[11].u32, 16 as u32, &mut ctx.xer);
	// 82DFA76C: 4198000C  blt cr6, 0x82dfa778
	if ctx.cr[6].lt {
	pc = 0x82DFA778; continue 'dispatch;
	}
	// 82DFA770: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFA774: 48000008  b 0x82dfa77c
	pc = 0x82DFA77C; continue 'dispatch;
	// 82DFA778: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82DFA77C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DFA780: 7D4BE1AE  stbx r10, r11, r28
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32), ctx.r[10].u8) };
	// 82DFA784: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFA788: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DFA78C: 483ADA2C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFA790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFA790 size=108
    let mut pc: u32 = 0x82DFA790;
    'dispatch: loop {
        match pc {
            0x82DFA790 => {
    //   block [0x82DFA790..0x82DFA7FC)
	// 82DFA790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFA794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFA798: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DFA79C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFA7A0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFA7A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFA7A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFA7AC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DFA7B0: 4BFFFE81  bl 0x82dfa630
	ctx.lr = 0x82DFA7B4;
	sub_82DFA630(ctx, base);
	// 82DFA7B4: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 82DFA7B8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DFA7BC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DFA7C0: 4BFFFC89  bl 0x82dfa448
	ctx.lr = 0x82DFA7C4;
	sub_82DFA448(ctx, base);
	// 82DFA7C4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DFA7C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFA7CC: 4BFFFE65  bl 0x82dfa630
	ctx.lr = 0x82DFA7D0;
	sub_82DFA630(ctx, base);
	// 82DFA7D0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DFA7D4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DFA7D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFA7DC: 4BFFF9B5  bl 0x82dfa190
	ctx.lr = 0x82DFA7E0;
	sub_82DFA190(ctx, base);
	// 82DFA7E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFA7E4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DFA7E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFA7EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFA7F0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DFA7F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFA7F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFA800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFA800 size=112
    let mut pc: u32 = 0x82DFA800;
    'dispatch: loop {
        match pc {
            0x82DFA800 => {
    //   block [0x82DFA800..0x82DFA870)
	// 82DFA800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFA804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFA808: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DFA80C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFA810: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFA814: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DFA818: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DFA81C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DFA820: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFA824: 4BFFF96D  bl 0x82dfa190
	ctx.lr = 0x82DFA828;
	sub_82DFA190(ctx, base);
	// 82DFA828: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82DFA82C: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFA830: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DFA834: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DFA838: 409AFFF4  bne cr6, 0x82dfa82c
	if !ctx.cr[6].eq {
	pc = 0x82DFA82C; continue 'dispatch;
	}
	// 82DFA83C: 7D7E5850  subf r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 82DFA840: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DFA844: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DFA848: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFA84C: 5565003E  slwi r5, r11, 0
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82DFA850: 4BFFFE39  bl 0x82dfa688
	ctx.lr = 0x82DFA854;
	sub_82DFA688(ctx, base);
	// 82DFA854: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFA858: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DFA85C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFA860: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFA864: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DFA868: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFA86C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFA870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFA870 size=228
    let mut pc: u32 = 0x82DFA870;
    'dispatch: loop {
        match pc {
            0x82DFA870 => {
    //   block [0x82DFA870..0x82DFA954)
	// 82DFA870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFA874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFA878: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFA87C: F8A10020  std r5, 0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(32 as u32), ctx.r[5].u64 ) };
	// 82DFA880: F8C10028  std r6, 0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(40 as u32), ctx.r[6].u64 ) };
	// 82DFA884: F8E10030  std r7, 0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(48 as u32), ctx.r[7].u64 ) };
	// 82DFA888: F9010038  std r8, 0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(56 as u32), ctx.r[8].u64 ) };
	// 82DFA88C: F9210040  std r9, 0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(64 as u32), ctx.r[9].u64 ) };
	// 82DFA890: F9410048  std r10, 0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(72 as u32), ctx.r[10].u64 ) };
	// 82DFA894: 9421FE30  stwu r1, -0x1d0(r1)
	ea = ctx.r[1].u32.wrapping_add(-464 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFA898: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82DFA89C: 394101F0  addi r10, r1, 0x1f0
	ctx.r[10].s64 = ctx.r[1].s64 + 496;
	// 82DFA8A0: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82DFA8A4: 38800100  li r4, 0x100
	ctx.r[4].s64 = 256;
	// 82DFA8A8: 386100C0  addi r3, r1, 0xc0
	ctx.r[3].s64 = ctx.r[1].s64 + 192;
	// 82DFA8AC: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DFA8B0: 80C10050  lwz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DFA8B4: 483B3415  bl 0x831adcc8
	ctx.lr = 0x82DFA8B8;
	sub_831ADCC8(ctx, base);
	// 82DFA8B8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82DFA8BC: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 82DFA8C0: 388B7688  addi r4, r11, 0x7688
	ctx.r[4].s64 = ctx.r[11].s64 + 30344;
	// 82DFA8C4: 4BFFFF3D  bl 0x82dfa800
	ctx.lr = 0x82DFA8C8;
	sub_82DFA800(ctx, base);
	// 82DFA8C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFA8CC: 388100C0  addi r4, r1, 0xc0
	ctx.r[4].s64 = ctx.r[1].s64 + 192;
	// 82DFA8D0: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82DFA8D4: 4BFFFF2D  bl 0x82dfa800
	ctx.lr = 0x82DFA8D8;
	sub_82DFA800(ctx, base);
	// 82DFA8D8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DFA8DC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DFA8E0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DFA8E4: 4BFFFEAD  bl 0x82dfa790
	ctx.lr = 0x82DFA8E8;
	sub_82DFA790(ctx, base);
	// 82DFA8E8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DFA8EC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DFA8F0: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82DFA8F4: 4BFFF89D  bl 0x82dfa190
	ctx.lr = 0x82DFA8F8;
	sub_82DFA190(ctx, base);
	// 82DFA8F8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DFA8FC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DFA900: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 82DFA904: 4BFFF88D  bl 0x82dfa190
	ctx.lr = 0x82DFA908;
	sub_82DFA190(ctx, base);
	// 82DFA908: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFA90C: 4BFFF835  bl 0x82dfa140
	ctx.lr = 0x82DFA910;
	sub_82DFA140(ctx, base);
	// 82DFA910: 81610078  lwz r11, 0x78(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82DFA914: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82DFA918: 2B0B0010  cmplwi cr6, r11, 0x10
	ctx.cr[6].compare_u32(ctx.r[11].u32, 16 as u32, &mut ctx.xer);
	// 82DFA91C: 40980008  bge cr6, 0x82dfa924
	if !ctx.cr[6].lt {
	pc = 0x82DFA924; continue 'dispatch;
	}
	// 82DFA920: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 82DFA924: 4BDD2975  bl 0x82bcd298
	ctx.lr = 0x82DFA928;
	sub_82BCD298(ctx, base);
	// 82DFA928: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFA92C: 4BFF7365  bl 0x82df1c90
	ctx.lr = 0x82DFA930;
	sub_82DF1C90(ctx, base);
	// 82DFA930: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DFA934: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DFA938: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DFA93C: 4BFFF855  bl 0x82dfa190
	ctx.lr = 0x82DFA940;
	sub_82DFA190(ctx, base);
	// 82DFA940: 382101D0  addi r1, r1, 0x1d0
	ctx.r[1].s64 = ctx.r[1].s64 + 464;
	// 82DFA944: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFA948: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFA94C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFA950: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFA958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFA958 size=28
    let mut pc: u32 = 0x82DFA958;
    'dispatch: loop {
        match pc {
            0x82DFA958 => {
    //   block [0x82DFA958..0x82DFA974)
	// 82DFA958: 54AB063F  clrlwi. r11, r5, 0x18
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DFA95C: 396400F2  addi r11, r4, 0xf2
	ctx.r[11].s64 = ctx.r[4].s64 + 242;
	// 82DFA960: 40820008  bne 0x82dfa968
	if !ctx.cr[0].eq {
	pc = 0x82DFA968; continue 'dispatch;
	}
	// 82DFA964: 396400FA  addi r11, r4, 0xfa
	ctx.r[11].s64 = ctx.r[4].s64 + 250;
	// 82DFA968: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DFA96C: 7C6B182E  lwzx r3, r11, r3
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82DFA970: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFA978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFA978 size=32
    let mut pc: u32 = 0x82DFA978;
    'dispatch: loop {
        match pc {
            0x82DFA978 => {
    //   block [0x82DFA978..0x82DFA998)
	// 82DFA978: 54AB063F  clrlwi. r11, r5, 0x18
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DFA97C: 396400F2  addi r11, r4, 0xf2
	ctx.r[11].s64 = ctx.r[4].s64 + 242;
	// 82DFA980: 40820008  bne 0x82dfa988
	if !ctx.cr[0].eq {
	pc = 0x82DFA988; continue 'dispatch;
	}
	// 82DFA984: 396400FA  addi r11, r4, 0xfa
	ctx.r[11].s64 = ctx.r[4].s64 + 250;
	// 82DFA988: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DFA98C: 7D6B182E  lwzx r11, r11, r3
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82DFA990: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82DFA994: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFA998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFA998 size=32
    let mut pc: u32 = 0x82DFA998;
    'dispatch: loop {
        match pc {
            0x82DFA998 => {
    //   block [0x82DFA998..0x82DFA9B8)
	// 82DFA998: 54AB063F  clrlwi. r11, r5, 0x18
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DFA99C: 396400F2  addi r11, r4, 0xf2
	ctx.r[11].s64 = ctx.r[4].s64 + 242;
	// 82DFA9A0: 40820008  bne 0x82dfa9a8
	if !ctx.cr[0].eq {
	pc = 0x82DFA9A8; continue 'dispatch;
	}
	// 82DFA9A4: 396400FA  addi r11, r4, 0xfa
	ctx.r[11].s64 = ctx.r[4].s64 + 250;
	// 82DFA9A8: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DFA9AC: 7D6B182E  lwzx r11, r11, r3
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82DFA9B0: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 82DFA9B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFA9B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFA9B8 size=32
    let mut pc: u32 = 0x82DFA9B8;
    'dispatch: loop {
        match pc {
            0x82DFA9B8 => {
    //   block [0x82DFA9B8..0x82DFA9D8)
	// 82DFA9B8: 54AB063F  clrlwi. r11, r5, 0x18
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DFA9BC: 396400F2  addi r11, r4, 0xf2
	ctx.r[11].s64 = ctx.r[4].s64 + 242;
	// 82DFA9C0: 40820008  bne 0x82dfa9c8
	if !ctx.cr[0].eq {
	pc = 0x82DFA9C8; continue 'dispatch;
	}
	// 82DFA9C4: 396400FA  addi r11, r4, 0xfa
	ctx.r[11].s64 = ctx.r[4].s64 + 250;
	// 82DFA9C8: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DFA9CC: 7D6B182E  lwzx r11, r11, r3
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82DFA9D0: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 82DFA9D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFA9D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFA9D8 size=32
    let mut pc: u32 = 0x82DFA9D8;
    'dispatch: loop {
        match pc {
            0x82DFA9D8 => {
    //   block [0x82DFA9D8..0x82DFA9F8)
	// 82DFA9D8: 54AB063F  clrlwi. r11, r5, 0x18
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DFA9DC: 396400F2  addi r11, r4, 0xf2
	ctx.r[11].s64 = ctx.r[4].s64 + 242;
	// 82DFA9E0: 40820008  bne 0x82dfa9e8
	if !ctx.cr[0].eq {
	pc = 0x82DFA9E8; continue 'dispatch;
	}
	// 82DFA9E4: 396400FA  addi r11, r4, 0xfa
	ctx.r[11].s64 = ctx.r[4].s64 + 250;
	// 82DFA9E8: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DFA9EC: 7D6B182E  lwzx r11, r11, r3
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82DFA9F0: 386B002C  addi r3, r11, 0x2c
	ctx.r[3].s64 = ctx.r[11].s64 + 44;
	// 82DFA9F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFA9F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFA9F8 size=48
    let mut pc: u32 = 0x82DFA9F8;
    'dispatch: loop {
        match pc {
            0x82DFA9F8 => {
    //   block [0x82DFA9F8..0x82DFAA28)
	// 82DFA9F8: 54AB063F  clrlwi. r11, r5, 0x18
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DFA9FC: 396400F2  addi r11, r4, 0xf2
	ctx.r[11].s64 = ctx.r[4].s64 + 242;
	// 82DFAA00: 40820008  bne 0x82dfaa08
	if !ctx.cr[0].eq {
	pc = 0x82DFAA08; continue 'dispatch;
	}
	// 82DFAA04: 396400FA  addi r11, r4, 0xfa
	ctx.r[11].s64 = ctx.r[4].s64 + 250;
	// 82DFAA08: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DFAA0C: 7D6B182E  lwzx r11, r11, r3
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82DFAA10: 396B001C  addi r11, r11, 0x1c
	ctx.r[11].s64 = ctx.r[11].s64 + 28;
	// 82DFAA14: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFAA18: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DFAA1C: 409A000C  bne cr6, 0x82dfaa28
	if !ctx.cr[6].eq {
		sub_82DFAA28(ctx, base);
		return;
	}
	// 82DFAA20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DFAA24: 48000010  b 0x82dfaa34
	sub_82DFAA28(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFAA28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFAA28 size=28
    let mut pc: u32 = 0x82DFAA28;
    'dispatch: loop {
        match pc {
            0x82DFAA28 => {
    //   block [0x82DFAA28..0x82DFAA44)
	// 82DFAA28: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFAA2C: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82DFAA30: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 82DFAA34: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82DFAA38: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82DFAA3C: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 82DFAA40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFAA48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFAA48 size=80
    let mut pc: u32 = 0x82DFAA48;
    'dispatch: loop {
        match pc {
            0x82DFAA48 => {
    //   block [0x82DFAA48..0x82DFAA98)
	// 82DFAA48: 54AA063F  clrlwi. r10, r5, 0x18
	ctx.r[10].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DFAA4C: 396400F2  addi r11, r4, 0xf2
	ctx.r[11].s64 = ctx.r[4].s64 + 242;
	// 82DFAA50: 40820008  bne 0x82dfaa58
	if !ctx.cr[0].eq {
	pc = 0x82DFAA58; continue 'dispatch;
	}
	// 82DFAA54: 396400FA  addi r11, r4, 0xfa
	ctx.r[11].s64 = ctx.r[4].s64 + 250;
	// 82DFAA58: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DFAA5C: 3D20000F  lis r9, 0xf
	ctx.r[9].s64 = 983040;
	// 82DFAA60: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DFAA64: 612A4240  ori r10, r9, 0x4240
	ctx.r[10].u64 = ctx.r[9].u64 | 16960;
	// 82DFAA68: 7D6B182E  lwzx r11, r11, r3
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82DFAA6C: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DFAA70: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DFAA74: 396400F2  addi r11, r4, 0xf2
	ctx.r[11].s64 = ctx.r[4].s64 + 242;
	// 82DFAA78: 409A0008  bne cr6, 0x82dfaa80
	if !ctx.cr[6].eq {
	pc = 0x82DFAA80; continue 'dispatch;
	}
	// 82DFAA7C: 396400FA  addi r11, r4, 0xfa
	ctx.r[11].s64 = ctx.r[4].s64 + 250;
	// 82DFAA80: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DFAA84: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DFAA88: 7D6B182E  lwzx r11, r11, r3
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82DFAA8C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82DFAA90: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DFAA94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFAA98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFAA98 size=112
    let mut pc: u32 = 0x82DFAA98;
    'dispatch: loop {
        match pc {
            0x82DFAA98 => {
    //   block [0x82DFAA98..0x82DFAB08)
	// 82DFAA98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFAA9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFAAA0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DFAAA4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFAAA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFAAAC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DFAAB0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFAAB4: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82DFAAB8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82DFAABC: 4B4C603D  bl 0x822c0af8
	ctx.lr = 0x82DFAAC0;
	sub_822C0AF8(ctx, base);
	// 82DFAAC0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DFAAC4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DFAAC8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82DFAACC: 4B4C5535  bl 0x822c0000
	ctx.lr = 0x82DFAAD0;
	sub_822C0000(ctx, base);
	// 82DFAAD0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DFAAD4: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DFAAD8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DFAADC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFAAE0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DFAAE4: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DFAAE8: 419A0008  beq cr6, 0x82dfaaf0
	if ctx.cr[6].eq {
	pc = 0x82DFAAF0; continue 'dispatch;
	}
	// 82DFAAEC: 4B4C5DA5  bl 0x822c0890
	ctx.lr = 0x82DFAAF0;
	sub_822C0890(ctx, base);
	// 82DFAAF0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DFAAF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFAAF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFAAFC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DFAB00: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFAB04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFAB08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFAB08 size=132
    let mut pc: u32 = 0x82DFAB08;
    'dispatch: loop {
        match pc {
            0x82DFAB08 => {
    //   block [0x82DFAB08..0x82DFAB8C)
	// 82DFAB08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFAB0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFAB10: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFAB14: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFAB18: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFAB1C: 897F0038  lbz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82DFAB20: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DFAB24: 40820050  bne 0x82dfab74
	if !ctx.cr[0].eq {
	pc = 0x82DFAB74; continue 'dispatch;
	}
	// 82DFAB28: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DFAB2C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFAB30: 4BFFE921  bl 0x82df9450
	ctx.lr = 0x82DFAB34;
	sub_82DF9450(ctx, base);
	// 82DFAB34: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DFAB38: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DFAB3C: 4BFFEC4D  bl 0x82df9788
	ctx.lr = 0x82DFAB40;
	sub_82DF9788(ctx, base);
	// 82DFAB40: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DFAB44: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 82DFAB48: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DFAB4C: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 82DFAB50: 4B4CE5E1  bl 0x822c9130
	ctx.lr = 0x82DFAB54;
	sub_822C9130(ctx, base);
	// 82DFAB54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DFAB58: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DFAB5C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DFAB60: 4B4CE2D1  bl 0x822c8e30
	ctx.lr = 0x82DFAB64;
	sub_822C8E30(ctx, base);
	// 82DFAB64: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFAB68: 4BFF88C1  bl 0x82df3428
	ctx.lr = 0x82DFAB6C;
	sub_82DF3428(ctx, base);
	// 82DFAB6C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DFAB70: 997F0038  stb r11, 0x38(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u8 ) };
	// 82DFAB74: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 82DFAB78: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DFAB7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFAB80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFAB84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFAB88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFAB90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFAB90 size=288
    let mut pc: u32 = 0x82DFAB90;
    'dispatch: loop {
        match pc {
            0x82DFAB90 => {
    //   block [0x82DFAB90..0x82DFACB0)
	// 82DFAB90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFAB94: 483AD5C5  bl 0x831a8158
	ctx.lr = 0x82DFAB98;
	sub_831A8130(ctx, base);
	// 82DFAB98: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFAB9C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DFABA0: 483A0CE1  bl 0x8319b880
	ctx.lr = 0x82DFABA4;
	sub_8319B880(ctx, base);
	// 82DFABA4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82DFABA8: 38A00030  li r5, 0x30
	ctx.r[5].s64 = 48;
	// 82DFABAC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DFABB0: 93FE0004  stw r31, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82DFABB4: 387E0008  addi r3, r30, 8
	ctx.r[3].s64 = ctx.r[30].s64 + 8;
	// 82DFABB8: 483AD629  bl 0x831a81e0
	ctx.lr = 0x82DFABBC;
	sub_831A81E0(ctx, base);
	// 82DFABBC: 9BFE0038  stb r31, 0x38(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(56 as u32), ctx.r[31].u8 ) };
	// 82DFABC0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DFABC4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DFABC8: 387E003C  addi r3, r30, 0x3c
	ctx.r[3].s64 = ctx.r[30].s64 + 60;
	// 82DFABCC: 4B4CE265  bl 0x822c8e30
	ctx.lr = 0x82DFABD0;
	sub_822C8E30(ctx, base);
	// 82DFABD0: 3B5E0060  addi r26, r30, 0x60
	ctx.r[26].s64 = ctx.r[30].s64 + 96;
	// 82DFABD4: 93FE0058  stw r31, 0x58(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 82DFABD8: 39400007  li r10, 7
	ctx.r[10].s64 = 7;
	// 82DFABDC: 93FE005C  stw r31, 0x5c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 82DFABE0: 7F4BD378  mr r11, r26
	ctx.r[11].u64 = ctx.r[26].u64;
	// 82DFABE4: 93EB0000  stw r31, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82DFABE8: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DFABEC: 93EB0004  stw r31, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82DFABF0: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DFABF4: 4080FFF0  bge 0x82dfabe4
	if !ctx.cr[0].lt {
	pc = 0x82DFABE4; continue 'dispatch;
	}
	// 82DFABF8: 395E00C8  addi r10, r30, 0xc8
	ctx.r[10].s64 = ctx.r[30].s64 + 200;
	// 82DFABFC: 3920000F  li r9, 0xf
	ctx.r[9].s64 = 15;
	// 82DFAC00: 396A0014  addi r11, r10, 0x14
	ctx.r[11].s64 = ctx.r[10].s64 + 20;
	// 82DFAC04: 3D003B9A  lis r8, 0x3b9a
	ctx.r[8].s64 = 999948288;
	// 82DFAC08: 93EA0000  stw r31, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82DFAC0C: 93EBFFF0  stw r31, -0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-16 as u32), ctx.r[31].u32 ) };
	// 82DFAC10: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82DFAC14: 6108CA00  ori r8, r8, 0xca00
	ctx.r[8].u64 = ctx.r[8].u64 | 51712;
	// 82DFAC18: 394A0030  addi r10, r10, 0x30
	ctx.r[10].s64 = ctx.r[10].s64 + 48;
	// 82DFAC1C: 910BFFF4  stw r8, -0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-12 as u32), ctx.r[8].u32 ) };
	// 82DFAC20: 93EBFFFC  stw r31, -4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[31].u32 ) };
	// 82DFAC24: 93EB0000  stw r31, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82DFAC28: 93EB0004  stw r31, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82DFAC2C: 93EB000C  stw r31, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82DFAC30: 93EB0010  stw r31, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[31].u32 ) };
	// 82DFAC34: 93EB0014  stw r31, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[31].u32 ) };
	// 82DFAC38: 93EB0018  stw r31, 0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[31].u32 ) };
	// 82DFAC3C: 396B0030  addi r11, r11, 0x30
	ctx.r[11].s64 = ctx.r[11].s64 + 48;
	// 82DFAC40: 4080FFC4  bge 0x82dfac04
	if !ctx.cr[0].lt {
	pc = 0x82DFAC04; continue 'dispatch;
	}
	// 82DFAC44: 7FFCFB78  mr r28, r31
	ctx.r[28].u64 = ctx.r[31].u64;
	// 82DFAC48: 3BBE03E8  addi r29, r30, 0x3e8
	ctx.r[29].s64 = ctx.r[30].s64 + 1000;
	// 82DFAC4C: 3B7E00F8  addi r27, r30, 0xf8
	ctx.r[27].s64 = ctx.r[30].s64 + 248;
	// 82DFAC50: 3B3E00A0  addi r25, r30, 0xa0
	ctx.r[25].s64 = ctx.r[30].s64 + 160;
	// 82DFAC54: 3F008335  lis r24, -0x7ccb
	ctx.r[24].s64 = -2093678592;
	// 82DFAC58: 80781108  lwz r3, 0x1108(r24)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4360 as u32) ) } as u64;
	// 82DFAC5C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFAC60: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFAC64: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DFAC68: 4E800421  bctrl
	ctx.lr = 0x82DFAC6C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DFAC6C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DFAC70: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82DFAC74: 4BFFFE25  bl 0x82dfaa98
	ctx.lr = 0x82DFAC78;
	sub_82DFAA98(ctx, base);
	// 82DFAC78: 397BFFD0  addi r11, r27, -0x30
	ctx.r[11].s64 = ctx.r[27].s64 + -48;
	// 82DFAC7C: 937D0000  stw r27, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82DFAC80: 3B5A0008  addi r26, r26, 8
	ctx.r[26].s64 = ctx.r[26].s64 + 8;
	// 82DFAC84: 917DFFE0  stw r11, -0x20(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(-32 as u32), ctx.r[11].u32 ) };
	// 82DFAC88: 3B7B0060  addi r27, r27, 0x60
	ctx.r[27].s64 = ctx.r[27].s64 + 96;
	// 82DFAC8C: 7FF9E1AE  stbx r31, r25, r28
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[25].u32.wrapping_add(ctx.r[28].u32), ctx.r[31].u8) };
	// 82DFAC90: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82DFAC94: 93FDFCC0  stw r31, -0x340(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(-832 as u32), ctx.r[31].u32 ) };
	// 82DFAC98: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82DFAC9C: 2F1C0008  cmpwi cr6, r28, 8
	ctx.cr[6].compare_i32(ctx.r[28].s32, 8, &mut ctx.xer);
	// 82DFACA0: 4198FFB8  blt cr6, 0x82dfac58
	if ctx.cr[6].lt {
	pc = 0x82DFAC58; continue 'dispatch;
	}
	// 82DFACA4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DFACA8: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82DFACAC: 483AD4FC  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFACB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFACB0 size=76
    let mut pc: u32 = 0x82DFACB0;
    'dispatch: loop {
        match pc {
            0x82DFACB0 => {
    //   block [0x82DFACB0..0x82DFACFC)
	// 82DFACB0: 8123000C  lwz r9, 0xc(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DFACB4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DFACB8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DFACBC: 419A0028  beq cr6, 0x82dface4
	if ctx.cr[6].eq {
	pc = 0x82DFACE4; continue 'dispatch;
	}
	// 82DFACC0: 81030008  lwz r8, 8(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFACC4: 39480004  addi r10, r8, 4
	ctx.r[10].s64 = ctx.r[8].s64 + 4;
	// 82DFACC8: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFACCC: 7F072040  cmplw cr6, r7, r4
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82DFACD0: 419A002C  beq cr6, 0x82dfacfc
	if ctx.cr[6].eq {
		sub_82DFACFC(ctx, base);
		return;
	}
	// 82DFACD4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DFACD8: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82DFACDC: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DFACE0: 4198FFE8  blt cr6, 0x82dfacc8
	if ctx.cr[6].lt {
	pc = 0x82DFACC8; continue 'dispatch;
	}
	// 82DFACE4: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFACE8: 54AB103A  slwi r11, r5, 2
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DFACEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82DFACF0: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DFACF4: 912BFFFC  stw r9, -4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[9].u32 ) };
	// 82DFACF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFACFC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFACFC size=28
    let mut pc: u32 = 0x82DFACFC;
    'dispatch: loop {
        match pc {
            0x82DFACFC => {
    //   block [0x82DFACFC..0x82DFAD18)
	// 82DFACFC: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFAD00: 54AA103A  slwi r10, r5, 2
	ctx.r[10].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DFAD04: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DFAD08: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82DFAD0C: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82DFAD10: 916AFFFC  stw r11, -4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-4 as u32), ctx.r[11].u32 ) };
	// 82DFAD14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFAD18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFAD18 size=184
    let mut pc: u32 = 0x82DFAD18;
    'dispatch: loop {
        match pc {
            0x82DFAD18 => {
    //   block [0x82DFAD18..0x82DFADD0)
	// 82DFAD18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFAD1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFAD20: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DFAD24: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFAD28: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFAD2C: 89630024  lbz r11, 0x24(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DFAD30: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DFAD34: 40820020  bne 0x82dfad54
	if !ctx.cr[0].eq {
	pc = 0x82DFAD54; continue 'dispatch;
	}
	// 82DFAD38: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DFAD3C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DFAD40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFAD44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFAD48: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DFAD4C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFAD50: 4E800020  blr
	return;
	// 82DFAD54: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82DFAD58: 419AFFE0  beq cr6, 0x82dfad38
	if ctx.cr[6].eq {
	pc = 0x82DFAD38; continue 'dispatch;
	}
	// 82DFAD5C: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DFAD60: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DFAD64: 4199FFD4  bgt cr6, 0x82dfad38
	if ctx.cr[6].gt {
	pc = 0x82DFAD38; continue 'dispatch;
	}
	// 82DFAD68: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFAD6C: 548A103A  slwi r10, r4, 2
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DFAD70: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DFAD74: 83EBFFFC  lwz r31, -4(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82DFAD78: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82DFAD7C: 419AFFBC  beq cr6, 0x82dfad38
	if ctx.cr[6].eq {
	pc = 0x82DFAD38; continue 'dispatch;
	}
	// 82DFAD80: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DFAD84: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFAD88: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFAD8C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DFAD90: 4E800421  bctrl
	ctx.lr = 0x82DFAD94;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DFAD94: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFAD98: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DFAD9C: 409A0024  bne cr6, 0x82dfadc0
	if !ctx.cr[6].eq {
	pc = 0x82DFADC0; continue 'dispatch;
	}
	// 82DFADA0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DFADA4: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DFADA8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFADAC: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DFADB0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DFADB4: 4E800421  bctrl
	ctx.lr = 0x82DFADB8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DFADB8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DFADBC: 4BFFFF80  b 0x82dfad3c
	pc = 0x82DFAD3C; continue 'dispatch;
	// 82DFADC0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFADC4: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82DFADC8: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82DFADCC: 4BFFFFD8  b 0x82dfada4
	pc = 0x82DFADA4; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFADD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFADD0 size=24
    let mut pc: u32 = 0x82DFADD0;
    'dispatch: loop {
        match pc {
            0x82DFADD0 => {
    //   block [0x82DFADD0..0x82DFADE8)
	// 82DFADD0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DFADD4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFADD8: 7F045040  cmplw cr6, r4, r10
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DFADDC: 4098000C  bge cr6, 0x82dfade8
	if !ctx.cr[6].lt {
		sub_82DFADE8(ctx, base);
		return;
	}
	// 82DFADE0: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82DFADE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFADE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFADE8 size=52
    let mut pc: u32 = 0x82DFADE8;
    'dispatch: loop {
        match pc {
            0x82DFADE8 => {
    //   block [0x82DFADE8..0x82DFAE1C)
	// 82DFADE8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFADEC: 7F045040  cmplw cr6, r4, r10
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DFADF0: 4098FFF0  bge cr6, 0x82dfade0
	if !ctx.cr[6].lt {
		sub_82DFADD0(ctx, base);
		return;
	}
	// 82DFADF4: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DFADF8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DFADFC: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFAE00: 554BF87E  srwi r11, r10, 1
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DFAE04: 55682036  slwi r8, r11, 4
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82DFAE08: 7D08482E  lwzx r8, r8, r9
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82DFAE0C: 7F044040  cmplw cr6, r4, r8
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82DFAE10: 4098000C  bge cr6, 0x82dfae1c
	if !ctx.cr[6].lt {
		sub_82DFAE1C(ctx, base);
		return;
	}
	// 82DFAE14: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82DFAE18: 48000008  b 0x82dfae20
	sub_82DFAE1C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFAE1C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFAE1C size=16
    let mut pc: u32 = 0x82DFAE1C;
    'dispatch: loop {
        match pc {
            0x82DFAE1C => {
    //   block [0x82DFAE1C..0x82DFAE2C)
	// 82DFAE1C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82DFAE20: 39630001  addi r11, r3, 1
	ctx.r[11].s64 = ctx.r[3].s64 + 1;
	// 82DFAE24: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DFAE28: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFAE2C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFAE2C size=12
    let mut pc: u32 = 0x82DFAE2C;
    'dispatch: loop {
        match pc {
            0x82DFAE2C => {
    //   block [0x82DFAE2C..0x82DFAE38)
	// 82DFAE2C: 7D6A1A14  add r11, r10, r3
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 82DFAE30: 556BF87E  srwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DFAE34: 4BFFFFD0  b 0x82dfae04
	sub_82DFADE8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFAE38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFAE38 size=132
    let mut pc: u32 = 0x82DFAE38;
    'dispatch: loop {
        match pc {
            0x82DFAE38 => {
    //   block [0x82DFAE38..0x82DFAEBC)
	// 82DFAE38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFAE3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFAE40: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DFAE44: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFAE48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFAE4C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DFAE50: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82DFAE54: 4098000C  bge cr6, 0x82dfae60
	if !ctx.cr[6].lt {
	pc = 0x82DFAE60; continue 'dispatch;
	}
	// 82DFAE58: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DFAE5C: 48000048  b 0x82dfaea4
	pc = 0x82DFAEA4; continue 'dispatch;
	// 82DFAE60: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFAE64: 54AA2036  slwi r10, r5, 4
	ctx.r[10].u32 = ctx.r[5].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DFAE68: 7FEA5A14  add r31, r10, r11
	ctx.r[31].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DFAE6C: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DFAE70: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFAE74: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFAE78: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DFAE7C: 4E800421  bctrl
	ctx.lr = 0x82DFAE80;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DFAE80: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFAE84: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DFAE88: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DFAE8C: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82DFAE90: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFAE94: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DFAE98: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DFAE9C: 4E800421  bctrl
	ctx.lr = 0x82DFAEA0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DFAEA0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DFAEA4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DFAEA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFAEAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFAEB0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DFAEB4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFAEB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFAEC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFAEC0 size=188
    let mut pc: u32 = 0x82DFAEC0;
    'dispatch: loop {
        match pc {
            0x82DFAEC0 => {
    //   block [0x82DFAEC0..0x82DFAF7C)
	// 82DFAEC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFAEC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFAEC8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DFAECC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFAED0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFAED4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DFAED8: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82DFAEDC: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82DFAEE0: 90BE0000  stw r5, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82DFAEE4: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFAEE8: 915E0004  stw r10, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DFAEEC: 90BE0008  stw r5, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82DFAEF0: 806B16D4  lwz r3, 0x16d4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5844 as u32) ) } as u64;
	// 82DFAEF4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFAEF8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFAEFC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DFAF00: 4E800421  bctrl
	ctx.lr = 0x82DFAF04;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DFAF04: 907E000C  stw r3, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 82DFAF08: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFAF0C: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DFAF10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DFAF14: 41820034  beq 0x82dfaf48
	if ctx.cr[0].eq {
	pc = 0x82DFAF48; continue 'dispatch;
	}
	// 82DFAF18: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFAF1C: 396A0001  addi r11, r10, 1
	ctx.r[11].s64 = ctx.r[10].s64 + 1;
	// 82DFAF20: 813E0008  lwz r9, 8(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFAF24: 7CE851D6  mullw r7, r8, r10
	ctx.r[7].s64 = (ctx.r[8].s32 as i64) * (ctx.r[10].s32 as i64);
	// 82DFAF28: 7D4B41D6  mullw r10, r11, r8
	ctx.r[10].s64 = (ctx.r[11].s32 as i64) * (ctx.r[8].s32 as i64);
	// 82DFAF2C: 7D0A4A14  add r8, r10, r9
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82DFAF30: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82DFAF34: 7D07492E  stwx r8, r7, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[9].u32), ctx.r[8].u32) };
	// 82DFAF38: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFAF3C: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82DFAF40: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DFAF44: 4198FFD4  blt cr6, 0x82dfaf18
	if ctx.cr[6].lt {
	pc = 0x82DFAF18; continue 'dispatch;
	}
	// 82DFAF48: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFAF4C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DFAF50: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFAF54: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DFAF58: 811E0008  lwz r8, 8(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFAF5C: 7D6B49D6  mullw r11, r11, r9
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[9].s32 as i64);
	// 82DFAF60: 7D4B412E  stwx r10, r11, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32), ctx.r[10].u32) };
	// 82DFAF64: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DFAF68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFAF6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFAF70: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DFAF74: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFAF78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFAF80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFAF80 size=72
    let mut pc: u32 = 0x82DFAF80;
    'dispatch: loop {
        match pc {
            0x82DFAF80 => {
    //   block [0x82DFAF80..0x82DFAFC8)
	// 82DFAF80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFAF84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFAF88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFAF8C: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 82DFAF90: 89670024  lbz r11, 0x24(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DFAF94: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DFAF98: 4082000C  bne 0x82dfafa4
	if !ctx.cr[0].eq {
	pc = 0x82DFAFA4; continue 'dispatch;
	}
	// 82DFAF9C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DFAFA0: 48000018  b 0x82dfafb8
	pc = 0x82DFAFB8; continue 'dispatch;
	// 82DFAFA4: 7CE33B78  mr r3, r7
	ctx.r[3].u64 = ctx.r[7].u64;
	// 82DFAFA8: 4BFFFE29  bl 0x82dfadd0
	ctx.lr = 0x82DFAFAC;
	sub_82DFADD0(ctx, base);
	// 82DFAFAC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82DFAFB0: 7CE33B78  mr r3, r7
	ctx.r[3].u64 = ctx.r[7].u64;
	// 82DFAFB4: 4BFFFE85  bl 0x82dfae38
	ctx.lr = 0x82DFAFB8;
	sub_82DFAE38(ctx, base);
	// 82DFAFB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DFAFBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFAFC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFAFC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFAFC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFAFC8 size=352
    let mut pc: u32 = 0x82DFAFC8;
    'dispatch: loop {
        match pc {
            0x82DFAFC8 => {
    //   block [0x82DFAFC8..0x82DFB128)
	// 82DFAFC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFAFCC: 483AD199  bl 0x831a8164
	ctx.lr = 0x82DFAFD0;
	sub_831A8130(ctx, base);
	// 82DFAFD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFAFD4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFAFD8: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DFAFDC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DFAFE0: 419A0010  beq cr6, 0x82dfaff0
	if ctx.cr[6].eq {
	pc = 0x82DFAFF0; continue 'dispatch;
	}
	// 82DFAFE4: 815F001C  lwz r10, 0x1c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DFAFE8: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82DFAFEC: 7D6B1E70  srawi r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	// 82DFAFF0: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DFAFF4: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DFAFF8: 3D208335  lis r9, -0x7ccb
	ctx.r[9].s64 = -2093678592;
	// 82DFAFFC: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DFB000: 557E103A  slwi r30, r11, 2
	ctx.r[30].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82DFB004: 8069110C  lwz r3, 0x110c(r9)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82DFB008: 48257741  bl 0x83052748
	ctx.lr = 0x82DFB00C;
	sub_83052748(ctx, base);
	// 82DFB00C: 7D7E1B96  divwu r11, r30, r3
	ctx.r[11].u32 = ctx.r[30].u32 / ctx.r[3].u32;
	// 82DFB010: 0CC30000  twi 6, r3, 0
	// 82DFB014: 7D6B19D6  mullw r11, r11, r3
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[3].s32 as i64);
	// 82DFB018: 7D6BF051  subf. r11, r11, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DFB01C: 4182000C  beq 0x82dfb028
	if ctx.cr[0].eq {
	pc = 0x82DFB028; continue 'dispatch;
	}
	// 82DFB020: 7D6B1850  subf r11, r11, r3
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	// 82DFB024: 7FCBF214  add r30, r11, r30
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82DFB028: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DFB02C: 7FDBF378  mr r27, r30
	ctx.r[27].u64 = ctx.r[30].u64;
	// 82DFB030: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DFB034: 409A000C  bne cr6, 0x82dfb040
	if !ctx.cr[6].eq {
	pc = 0x82DFB040; continue 'dispatch;
	}
	// 82DFB038: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82DFB03C: 48000010  b 0x82dfb04c
	pc = 0x82DFB04C; continue 'dispatch;
	// 82DFB040: 815F001C  lwz r10, 0x1c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DFB044: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82DFB048: 7D7D1E70  srawi r29, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[29].s64 = (ctx.r[11].s32 >> 3) as i64;
	// 82DFB04C: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82DFB050: 40990028  ble cr6, 0x82dfb078
	if !ctx.cr[6].gt {
	pc = 0x82DFB078; continue 'dispatch;
	}
	// 82DFB054: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DFB058: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 82DFB05C: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFB060: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DFB064: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFB068: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DFB06C: 7D2941D6  mullw r9, r9, r8
	ctx.r[9].s64 = (ctx.r[9].s32 as i64) * (ctx.r[8].s32 as i64);
	// 82DFB070: 7FC9F214  add r30, r9, r30
	ctx.r[30].u64 = ctx.r[9].u64 + ctx.r[30].u64;
	// 82DFB074: 4082FFE8  bne 0x82dfb05c
	if !ctx.cr[0].eq {
	pc = 0x82DFB05C; continue 'dispatch;
	}
	// 82DFB078: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DFB07C: 4BFF6E2D  bl 0x82df1ea8
	ctx.lr = 0x82DFB080;
	sub_82DF1EA8(ctx, base);
	// 82DFB080: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DFB084: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DFB088: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82DFB08C: 483AD155  bl 0x831a81e0
	ctx.lr = 0x82DFB090;
	sub_831A81E0(ctx, base);
	// 82DFB090: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFB094: 7D4BF214  add r10, r11, r30
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82DFB098: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DFB09C: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DFB0A0: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DFB0A4: 7F8A5A14  add r28, r10, r11
	ctx.r[28].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DFB0A8: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82DFB0AC: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DFB0B0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DFB0B4: 419A0010  beq cr6, 0x82dfb0c4
	if ctx.cr[6].eq {
	pc = 0x82DFB0C4; continue 'dispatch;
	}
	// 82DFB0B8: 813F001C  lwz r9, 0x1c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DFB0BC: 7D4A4850  subf r10, r10, r9
	ctx.r[10].s64 = ctx.r[9].s64 - ctx.r[10].s64;
	// 82DFB0C0: 7D4A1E70  srawi r10, r10, 3
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 3) as i64;
	// 82DFB0C4: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82DFB0C8: 7F6BDA14  add r27, r11, r27
	ctx.r[27].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 82DFB0CC: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82DFB0D0: 40990048  ble cr6, 0x82dfb118
	if !ctx.cr[6].gt {
	pc = 0x82DFB118; continue 'dispatch;
	}
	// 82DFB0D4: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DFB0D8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DFB0DC: 7CDE5A14  add r6, r30, r11
	ctx.r[6].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82DFB0E0: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82DFB0E4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DFB0E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFB0EC: 4BFFFDD5  bl 0x82dfaec0
	ctx.lr = 0x82DFB0F0;
	sub_82DFAEC0(ctx, base);
	// 82DFB0F0: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DFB0F4: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82DFB0F8: 7D5E5A14  add r10, r30, r11
	ctx.r[10].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82DFB0FC: 3B9C0010  addi r28, r28, 0x10
	ctx.r[28].s64 = ctx.r[28].s64 + 16;
	// 82DFB100: 7D3E582E  lwzx r9, r30, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DFB104: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 82DFB108: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFB10C: 7D4A49D6  mullw r10, r10, r9
	ctx.r[10].s64 = (ctx.r[10].s32 as i64) * (ctx.r[9].s32 as i64);
	// 82DFB110: 7F6ADA14  add r27, r10, r27
	ctx.r[27].u64 = ctx.r[10].u64 + ctx.r[27].u64;
	// 82DFB114: 4082FFC8  bne 0x82dfb0dc
	if !ctx.cr[0].eq {
	pc = 0x82DFB0DC; continue 'dispatch;
	}
	// 82DFB118: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DFB11C: 997F0024  stb r11, 0x24(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u8 ) };
	// 82DFB120: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DFB124: 483AD090  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFB128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFB128 size=144
    let mut pc: u32 = 0x82DFB128;
    'dispatch: loop {
        match pc {
            0x82DFB128 => {
    //   block [0x82DFB128..0x82DFB1B8)
	// 82DFB128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFB12C: 483AD03D  bl 0x831a8168
	ctx.lr = 0x82DFB130;
	sub_831A8130(ctx, base);
	// 82DFB130: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFB134: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DFB138: 3B9D0014  addi r28, r29, 0x14
	ctx.r[28].s64 = ctx.r[29].s64 + 20;
	// 82DFB13C: 817D0018  lwz r11, 0x18(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DFB140: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DFB144: 419A0010  beq cr6, 0x82dfb154
	if ctx.cr[6].eq {
	pc = 0x82DFB154; continue 'dispatch;
	}
	// 82DFB148: 815C0008  lwz r10, 8(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFB14C: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82DFB150: 7D6B1E70  srawi r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	// 82DFB154: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DFB158: 40990040  ble cr6, 0x82dfb198
	if !ctx.cr[6].gt {
	pc = 0x82DFB198; continue 'dispatch;
	}
	// 82DFB15C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82DFB160: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82DFB164: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFB168: 7D7F5A14  add r11, r31, r11
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[11].u64;
	// 82DFB16C: 806B000C  lwz r3, 0xc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DFB170: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DFB174: 419A0018  beq cr6, 0x82dfb18c
	if ctx.cr[6].eq {
	pc = 0x82DFB18C; continue 'dispatch;
	}
	// 82DFB178: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFB17C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DFB180: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFB184: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DFB188: 4E800421  bctrl
	ctx.lr = 0x82DFB18C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DFB18C: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82DFB190: 3BFF0010  addi r31, r31, 0x10
	ctx.r[31].s64 = ctx.r[31].s64 + 16;
	// 82DFB194: 4082FFD0  bne 0x82dfb164
	if !ctx.cr[0].eq {
	pc = 0x82DFB164; continue 'dispatch;
	}
	// 82DFB198: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFB19C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DFB1A0: 419A0008  beq cr6, 0x82dfb1a8
	if ctx.cr[6].eq {
	pc = 0x82DFB1A8; continue 'dispatch;
	}
	// 82DFB1A4: 4BFF6D0D  bl 0x82df1eb0
	ctx.lr = 0x82DFB1A8;
	sub_82DF1EB0(ctx, base);
	// 82DFB1A8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DFB1AC: 48003F2D  bl 0x82dff0d8
	ctx.lr = 0x82DFB1B0;
	sub_82DFF0D8(ctx, base);
	// 82DFB1B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DFB1B4: 483AD004  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


